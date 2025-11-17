/// 订单匹配服务
///
/// 实现价格-时间优先的订单匹配算法
/// 遵循Clean Architecture的领域服务模式
use super::handler::{Command, CommandResult, OrderCommandHandler};
use super::repository::{OrderRepository, RepositoryAccessor};
use super::types::{EntityEvent, OrderId, Price, Quantity, Side, Trade, TraderId};
use crate::lob::repository::entity_repo::EntityRepo;
use crate::lob::repository::event_repo::EventRepo;
use crate::lob::OrderEntry;
use crate::{event, fields};


/// 匹配服务
///
/// 负责订单匹配逻辑，持有OrderRepository引用
pub struct MatchingService<R>
where
    R: OrderRepository + RepositoryAccessor,
{
    lob_repo: R,
    event_repo: EventRepo,
    entity_repo: EntityRepo,
    /// 事件序列号计数器
    event_id_counter: u64,
    /// 事务ID计数器
    transaction_id_counter: u64,
}

/// 创建交易事件
///
/// # 参数
/// - `matched_orders`: 匹配的订单列表
/// - `price`: 新订单的交易员ID
/// - `quantity`: 新订单的方向
///
/// # 返回
/// (订单更新事件列表, 交易事件列表, 未成交金额, 新订单ID)
fn create_trades(
    matched_orders: Option<Vec<&mut OrderEntry>>,
    price: Price,
    quantity: Quantity,
) -> (Option<Vec<EntityEvent>>, Option<Vec<EntityEvent>>, Quantity) {
    use super::types::{EventOperation, FieldChange, FieldValue};

    let mut order_events = Vec::new();
    let mut trade_events = Vec::new();
    let mut remaining = quantity;
    let transaction_id = 1; // TODO: 应该传入
    let mut event_id = 1; // TODO: 应该从全局分配

    if let Some(orders) = matched_orders {
        for matched_order in orders {
            if remaining == 0 {
                break;
            }

            // 1. 计算成交数量
            let fill_qty = remaining.min(matched_order.unfilled_quantity);

            // 2. 更新订单的未成交数量
            let old_unfilled = matched_order.unfilled_quantity;
            let new_unfilled = old_unfilled - fill_qty;
            matched_order.unfilled_quantity = new_unfilled;
            remaining -= fill_qty;

            // 3. 创建订单更新事件
            let order_update_event = event!(
                "Order", EventOperation::Update, event_id, transaction_id, matched_order.order_id => {
                    update: ("unfilled_quantity", FieldValue::U32(old_unfilled), FieldValue::U32(new_unfilled)),
                }
            );
            order_events.push(order_update_event);
            event_id += 1;

            // 4. 创建交易事件
            let trade_id = event_id;
            let trade_event = event!(
                "Trade", EventOperation::Create, event_id, transaction_id, trade_id => {
                    create:
                        "price" => FieldValue::U32(price),
                        "quantity" => FieldValue::U32(fill_qty),
                        "matched_order_id" => FieldValue::U64(matched_order.order_id),
                }
            );
            trade_events.push(trade_event);
            event_id += 1;
        }
    }

    let unfilled_amount = remaining;

    let order_events_opt = if order_events.is_empty() {
        None
    } else {
        Some(order_events)
    };

    let trade_events_opt = if trade_events.is_empty() {
        None
    } else {
        Some(trade_events)
    };

    (order_events_opt, trade_events_opt, unfilled_amount)
}

impl<R> MatchingService<R>
where
    R: OrderRepository + RepositoryAccessor,
{
    /// 创建新的匹配服务
    pub fn new(repository: R) -> Self {
        Self {
            lob_repo: repository,
            event_repo: EventRepo {},
            entity_repo: EntityRepo {},
            event_id_counter: 1,
            transaction_id_counter: 1,
        }
    }

    /// 分配新的事件ID
    fn allocate_event_id(&mut self) -> u64 {
        let id = self.event_id_counter;
        self.event_id_counter += 1;
        id
    }

    /// 分配新的事务ID
    fn allocate_transaction_id(&mut self) -> u64 {
        let id = self.transaction_id_counter;
        self.transaction_id_counter += 1;
        id
    }

    /// 获取repository的可变引用
    pub fn repository_mut(&mut self) -> &mut R {
        &mut self.lob_repo
    }

    /// 获取repository的不可变引用
    pub fn repository(&self) -> &R {
        &self.lob_repo
    }

    /// 执行限价订单匹配
    ///
    /// 返回 (成交列表, 剩余数量)
    pub fn match_limit_order(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
    ) -> (Vec<Trade>, Quantity) {
        let mut remaining = quantity;
        let mut trades = Vec::new();

        match side {
            Side::Buy => {
                // 从最佳（最低）卖价开始匹配
                self.match_buy_order(trader, price, &mut remaining, &mut trades);
            }
            Side::Sell => {
                // 从最佳（最高）买价开始匹配
                self.match_sell_order(trader, price, &mut remaining, &mut trades);
            }
        }

        (trades, remaining)
    }

    /// 执行市价订单匹配
    ///
    /// 市价单以任何价格成交，直到数量全部成交或对手方订单耗尽
    /// 返回 (成交列表, 剩余数量)
    pub fn match_market_order(
        &mut self,
        trader: TraderId,
        side: Side,
        quantity: Quantity,
    ) -> (Vec<Trade>, Quantity) {
        let mut remaining = quantity;
        let mut trades = Vec::new();

        match side {
            Side::Buy => {
                // 市价买单：以任何价格成交（使用 u32::MAX 作为价格上限）
                self.match_buy_order(trader, u32::MAX, &mut remaining, &mut trades);
            }
            Side::Sell => {
                // 市价卖单：以任何价格成交（使用 0 作为价格下限）
                self.match_sell_order(trader, 0, &mut remaining, &mut trades);
            }
        }

        (trades, remaining)
    }

    /// 取消订单
    ///
    /// # 参数
    /// - `order_id`: 要取消的订单ID
    ///
    /// # 返回
    /// - `bool`: 取消是否成功
    pub fn cancel_order(&mut self, order_id: OrderId) -> bool {
        self.lob_repo.cancel_order(order_id)
    }

    /// 匹配买单
    fn match_buy_order(
        &mut self,
        trader: TraderId,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
    ) {
        // 早期退出优化：如果买价低于最低卖价，不可能匹配
        if let Some(ask_min) = self.lob_repo.best_ask() {
            if price < ask_min {
                return; // 买价太低，直接返回
            }
        } else {
            return; // 没有卖单，直接返回
        }

        // 从最低卖价开始匹配（而不是从0开始）
        let mut current_price = self.lob_repo.best_ask().unwrap();

        while *remaining > 0 && current_price <= price {
            // 查找下一个非空卖价
            if let Some(ask_price) =
                self.find_next_non_empty_price(current_price, price, Side::Sell)
            {
                let fills = self.match_at_price(trader, Side::Buy, ask_price, remaining);
                trades.extend(fills);
                current_price = ask_price + 1;
            } else {
                break;
            }
        }
    }

    /// 匹配卖单
    fn match_sell_order(
        &mut self,
        trader: TraderId,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
    ) {
        // 早期退出优化：如果卖价高于最高买价，不可能匹配
        if let Some(bid_max) = self.lob_repo.best_bid() {
            if price > bid_max {
                return; // 卖价太高，直接返回
            }
        } else {
            return; // 没有买单，直接返回
        }

        // 从最高买价开始匹配（而不是从 u32::MAX 开始）
        let mut current_price = self.lob_repo.best_bid().unwrap();

        while *remaining > 0 && current_price >= price {
            // 查找上一个非空买价
            if let Some(bid_price) = self.find_prev_non_empty_price(current_price, price, Side::Buy)
            {
                let fills = self.match_at_price(trader, Side::Sell, bid_price, remaining);
                trades.extend(fills);

                if bid_price == 0 {
                    break;
                }
                current_price = bid_price.saturating_sub(1);
            } else {
                break;
            }
        }
    }

    /// 在特定价格级别执行匹配
    fn match_at_price(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        remaining: &mut Quantity,
    ) -> Vec<Trade> {
        let mut trades = Vec::new();

        // 获取对手方
        let opposite_side = side.opposite();

        let mut current_idx = self.lob_repo.get_first_order_at_price(price, opposite_side);
        let mut first_active_idx = None;

        while *remaining > 0 && current_idx.is_some() {
            let idx = current_idx.unwrap();

            if let Some(entry) = self.lob_repo.get_entry_mut(idx) {
                if entry.is_active() {
                    // 跟踪第一个活跃订单
                    if first_active_idx.is_none() {
                        first_active_idx = Some(idx);
                    }

                    // 计算成交数量
                    let fill_qty = (*remaining).min(entry.unfilled_quantity);

                    // 创建交易记录
                    let trade = match side {
                        Side::Buy => Trade::new(trader, entry.trader, price, fill_qty),
                        Side::Sell => Trade::new(entry.trader, trader, price, fill_qty),
                    };
                    trades.push(trade);

                    // 更新数量
                    *remaining -= fill_qty;
                    entry.unfilled_quantity -= fill_qty;

                    // 如果订单完全成交，标记为非活跃
                    if entry.unfilled_quantity == 0 {
                        let order_id = entry.order_id;
                        // 通过 cancel_order 移除索引
                        self.lob_repo.cancel_order(order_id);

                        // 如果这是第一个活跃订单，重置标记
                        if first_active_idx == Some(idx) {
                            first_active_idx = None;
                        }
                    }
                }

                // 获取下一个订单索引
                current_idx = self.lob_repo.get_next_order(idx);

                // 更新 first_active_idx
                if first_active_idx.is_none() && current_idx.is_some() {
                    if let Some(next_entry) = self.lob_repo.get_entry(current_idx.unwrap()) {
                        if next_entry.is_active() {
                            first_active_idx = current_idx;
                        }
                    }
                }
            } else {
                break;
            }
        }

        // 更新价格点以反映第一个活跃订单
        if first_active_idx.is_none() && current_idx.is_none() {
            // 所有订单都已消费，清空价格级别
            self.lob_repo
                .update_price_point(price, opposite_side, None, None);
        } else if first_active_idx.is_some() {
            // 更新为第一个活跃订单
            self.lob_repo
                .update_price_point(price, opposite_side, first_active_idx, None);
        }

        trades
    }

    /// 查找下一个非空的价格级别（用于买单匹配卖单）
    fn find_next_non_empty_price(
        &self,
        start_price: Price,
        max_price: Price,
        side: Side,
    ) -> Option<Price> {
        for price in start_price..=max_price {
            if !self.lob_repo.is_price_empty(price, side) {
                return Some(price);
            }
        }
        None
    }

    /// 查找上一个非空的价格级别（用于卖单匹配买单）
    fn find_prev_non_empty_price(
        &self,
        start_price: Price,
        min_price: Price,
        side: Side,
    ) -> Option<Price> {
        let start = start_price.min(100_000); // 限制搜索范围

        for price in (min_price..=start).rev() {
            if !self.lob_repo.is_price_empty(price, side) {
                return Some(price);
            }
        }
        None
    }
}

impl<R> OrderCommandHandler for MatchingService<R>
where
    R: OrderRepository + RepositoryAccessor + Send + Sync,
{
    fn handle(&mut self, command: Command) -> CommandResult {
        match command {
            // ========== 基础订单类型 ==========
            Command::LimitOrder {
                trader,
                side,
                price,
                quantity,
            } => {
                // cqrs模式：1) 生成EntityEvent 2) replay事件到仓储

                let trades = Vec::new(); // TODO: 从 trade_events 转换
                let new_order_id = self.lob_repo.allocate_order_id();

                let orders = self.lob_repo.match_Orders(side, price, quantity);

                if orders.is_some() && orders.as_ref().unwrap().len() > 0 {
                    let (order_change_events, trade_create_events, unfilled_amount) =
                        create_trades(orders, price, quantity);

                    // 创建新订单事件（如果有未成交部分）

                    let transaction_id = self.allocate_transaction_id();
                    let event_id = self.allocate_event_id();

                    // 创建订单创建事件
                    let order_create_event = event!(
                        "Order", super::types::EventOperation::Create, event_id, transaction_id, new_order_id => {
                            create:
                                "order_id" => super::types::FieldValue::OrderId(new_order_id),
                                "trader" => super::types::FieldValue::TraderId(trader),
                                "side" => super::types::FieldValue::Side(side),
                                "quantity" => super::types::FieldValue::Quantity(unfilled_amount),
                                "price" => super::types::FieldValue::U32(price),
                        }
                    );

                    //TODO: 事件落库 - 需要添加 event_repo 字段
                    self.event_repo.save(order_create_event);

                    //todo if create_event没有全成交，还要写lob

                    //TODO: 保存事件 - 需要添加 event_repo 字段
                    self.event_repo.save_batch(order_change_events.clone());
                    self.event_repo.save_batch(trade_create_events.clone());

                    //TODO: 回放事件到订单簿 - 需要实现 replay 方法
                    //可以通过回放实现，也可以通过上面函数直接变更； 部份成交怎么处理？
                    if let Some(events) = order_change_events.clone() {
                        let _ = self.lob_repo.replay(events);
                    }

                    //TODO: 订单实体落库 - 需要添加 entity_repo 字段
                    self.entity_repo.replay(order_change_events);
                    self.entity_repo.replay(trade_create_events);
                } else {
                    //TODO: 事件落库 - 需要添加 event_repo 字段
                    // let create_event = EntityEvent::single(...);
                    // self.event_repo.save(create_event);

                    //TODO: 在lob生成订单 - 需要实现逻辑
                    // self.lob_repo.replay(create_event);

                    //TODO: order实体落库 - 需要添加 entity_repo 字段
                    // self.entity_repo.replay(order_events);
                }
                //event中 包括旧order的update，新order的create;

                //1，订单生成事件；2，订单更改事件（买卖单）；3，交易生成事件

                CommandResult::LimitOrder {
                    order_id: new_order_id,
                    trades,
                }
            }

            Command::MarketOrder {
                trader,
                side,
                quantity,
            } => {
                let transaction_id = self.allocate_transaction_id();

                let (trades, _remaining) = self.match_market_order(trader, side, quantity);
                CommandResult::MarketOrder { trades }
            }

            Command::IcebergOrder {
                trader,
                side,
                price,
                total_quantity,
                display_quantity,
            } => CommandResult::ToDo {},

            Command::CancelOrder { order_id } => {
                let success = self.cancel_order(order_id);
                CommandResult::CancelOrder { success }
            }

            // ========== 未实现的订单类型 ==========
            _ => {
                // 对于未实现的命令类型，返回空结果或错误
                // 这里简单返回一个取消失败的结果作为占位符
                CommandResult::CancelOrder { success: false }
            }
        }
    }

    fn handler_name(&self) -> &'static str {
        "PriceTimeMatchingService"
    }
}
