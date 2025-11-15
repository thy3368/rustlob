/// 订单匹配服务
///
/// 实现价格-时间优先的订单匹配算法
/// 遵循Clean Architecture的领域服务模式
use super::handler::{Command, CommandResult, OrderCommandHandler};
use super::repository::{OrderRepository, RepositoryAccessor};
use super::types::{
    EntityEvent, EventOperation, FieldChange, FieldValue, OrderEntry, OrderId, Price, Quantity,
    Side, Trade, TraderId,
};


/// 匹配服务
///
/// 负责订单匹配逻辑，持有OrderRepository引用
pub struct MatchingService<R>
where
    R: OrderRepository + RepositoryAccessor,
{
    repository: R,
    /// 事件序列号计数器
    event_id_counter: u64,
    /// 事务ID计数器
    transaction_id_counter: u64,
}

impl<R> MatchingService<R>
where
    R: OrderRepository + RepositoryAccessor,
{
    /// 创建新的匹配服务
    pub fn new(repository: R) -> Self {
        Self {
            repository,
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
        &mut self.repository
    }

    /// 获取repository的不可变引用
    pub fn repository(&self) -> &R {
        &self.repository
    }

    /// 执行限价订单匹配（事件溯源版本）
    ///
    /// 生成所有状态变更的EntityEvent，包括：
    /// - 对手方订单数量更新事件
    /// - 交易执行事件
    /// - 对手方订单删除事件（完全成交）
    /// - 价格点更新事件
    /// - 新订单添加事件（如果有剩余）
    ///
    /// # 返回
    /// - `Vec<EntityEvent>`: 所有事件列表
    /// - `Vec<Trade>`: 交易列表
    /// - `OrderId`: 新订单ID（0表示完全成交）
    pub fn match_limit_order_with_events(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        quantity: Quantity,
        transaction_id: u64,
    ) -> (Vec<EntityEvent>, Vec<Trade>, OrderId) {
        let mut events = Vec::new();
        let mut trades = Vec::new();
        let mut remaining = quantity;

        // 匹配逻辑
        match side {
            Side::Buy => {
                self.match_buy_order_with_events(
                    trader,
                    price,
                    &mut remaining,
                    &mut trades,
                    &mut events,
                    transaction_id,
                );
            }
            Side::Sell => {
                self.match_sell_order_with_events(
                    trader,
                    price,
                    &mut remaining,
                    &mut trades,
                    &mut events,
                    transaction_id,
                );
            }
        }

        // 如果有剩余数量，生成添加订单事件
        let order_id = if remaining > 0 {
            let order_id = self.repository.allocate_order_id();
            let entry = OrderEntry::new(order_id, trader, remaining);

            let event_id = self.allocate_event_id();
            let add_order_event = EntityEvent::single(
                event_id,
                transaction_id,
                "Order",
                EventOperation::Create,
                order_id,
                vec![
                    FieldChange::created("entry", FieldValue::OrderEntry(entry)),
                    FieldChange::created("side", FieldValue::Side(side)),
                    FieldChange::created("price", FieldValue::U32(price)),
                ],
            );
            events.push(add_order_event);
            order_id
        } else {
            0
        };

        (events, trades, order_id)
    }

    /// 匹配买单（生成事件）
    fn match_buy_order_with_events(
        &mut self,
        trader: TraderId,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
        events: &mut Vec<EntityEvent>,
        transaction_id: u64,
    ) {
        // 早期退出：如果买价低于最低卖价，不可能匹配
        if let Some(ask_min) = self.repository.best_ask() {
            if price < ask_min {
                return;
            }
        } else {
            return;
        }

        let mut current_price = self.repository.best_ask().unwrap();

        while *remaining > 0 && current_price <= price {
            if let Some(ask_price) =
                self.find_next_non_empty_price(current_price, price, Side::Sell)
            {
                self.match_at_price_with_events(
                    trader,
                    Side::Buy,
                    ask_price,
                    remaining,
                    trades,
                    events,
                    transaction_id,
                );
                current_price = ask_price + 1;
            } else {
                break;
            }
        }
    }

    /// 匹配卖单（生成事件）
    fn match_sell_order_with_events(
        &mut self,
        trader: TraderId,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
        events: &mut Vec<EntityEvent>,
        transaction_id: u64,
    ) {
        // 早期退出：如果卖价高于最高买价，不可能匹配
        if let Some(bid_max) = self.repository.best_bid() {
            if price > bid_max {
                return;
            }
        } else {
            return;
        }

        let mut current_price = self.repository.best_bid().unwrap();

        while *remaining > 0 && current_price >= price {
            if let Some(bid_price) = self.find_prev_non_empty_price(current_price, price, Side::Buy)
            {
                self.match_at_price_with_events(
                    trader,
                    Side::Sell,
                    bid_price,
                    remaining,
                    trades,
                    events,
                    transaction_id,
                );

                if bid_price == 0 {
                    break;
                }
                current_price = bid_price.saturating_sub(1);
            } else {
                break;
            }
        }
    }

    /// 在特定价格级别匹配（生成事件）
    fn match_at_price_with_events(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        remaining: &mut Quantity,
        trades: &mut Vec<Trade>,
        events: &mut Vec<EntityEvent>,
        transaction_id: u64,
    ) {
        let opposite_side = side.opposite();
        let mut current_idx = self
            .repository
            .get_first_order_at_price(price, opposite_side);
        let mut first_active_idx = None;

        while *remaining > 0 && current_idx.is_some() {
            let idx = current_idx.unwrap();

            if let Some(entry) = self.repository.get_entry(idx) {
                if entry.is_active() {
                    if first_active_idx.is_none() {
                        first_active_idx = Some(idx);
                    }

                    let fill_qty = (*remaining).min(entry.unfilled_quantity);
                    let old_qty = entry.unfilled_quantity;
                    let new_qty = old_qty - fill_qty;
                    let order_id = entry.order_id;

                    // 生成交易事件
                    let trade = match side {
                        Side::Buy => Trade::new(trader, entry.trader, price, fill_qty),
                        Side::Sell => Trade::new(entry.trader, trader, price, fill_qty),
                    };
                    trades.push(trade);

                    // 生成订单更新事件
                    let event_id = self.allocate_event_id();
                    let update_event = EntityEvent::single(
                        event_id,
                        transaction_id,
                        "Order",
                        EventOperation::Update,
                        order_id,
                        vec![FieldChange::updated(
                            "unfilled_quantity",
                            FieldValue::U32(old_qty),
                            FieldValue::U32(new_qty),
                        )],
                    );
                    events.push(update_event);

                    *remaining -= fill_qty;

                    // 如果订单完全成交，生成删除事件
                    if new_qty == 0 {
                        let event_id = self.allocate_event_id();
                        let delete_event = EntityEvent::single(
                            event_id,
                            transaction_id,
                            "Order",
                            EventOperation::Delete,
                            order_id,
                            vec![],
                        );
                        events.push(delete_event);

                        if first_active_idx == Some(idx) {
                            first_active_idx = None;
                        }
                    }
                }

                current_idx = self.repository.get_next_order(idx);

                if first_active_idx.is_none() && current_idx.is_some() {
                    if let Some(next_entry) = self.repository.get_entry(current_idx.unwrap()) {
                        if next_entry.is_active() {
                            first_active_idx = current_idx;
                        }
                    }
                }
            } else {
                break;
            }
        }

        // 生成价格点更新事件
        let event_id = self.allocate_event_id();
        let price_event = EntityEvent::single(
            event_id,
            transaction_id,
            "PricePoint",
            EventOperation::Update,
            price as u64,
            vec![
                FieldChange::updated(
                    "first_idx",
                    FieldValue::OptionUsize(None),
                    FieldValue::OptionUsize(first_active_idx),
                ),
                FieldChange::updated(
                    "last_idx",
                    FieldValue::OptionUsize(None),
                    FieldValue::OptionUsize(if first_active_idx.is_none() {
                        None
                    } else {
                        current_idx
                    }),
                ),
                FieldChange::created("side", FieldValue::Side(opposite_side)),
            ],
        );
        events.push(price_event);
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
        self.repository.cancel_order(order_id)
    }

    /// 执行冰山订单匹配
    ///
    /// 冰山订单仅显示部分数量（display_quantity），当显示部分成交后自动补充
    ///
    /// # 参数
    /// - `trader`: 交易者ID
    /// - `side`: 买卖方向
    /// - `price`: 限价
    /// - `total_quantity`: 总数量
    /// - `display_quantity`: 每次显示的数量
    ///
    /// # 返回
    /// - `order_id`: 订单ID（挂单中的订单ID，如果没有挂单则为0）
    /// - `trades`: 本次成交列表
    /// - `remaining_total`: 剩余总数量（未成交且未挂单的数量）
    /// - `current_display`: 当前显示数量（挂单中的数量）
    pub fn match_iceberg_order(
        &mut self,
        trader: TraderId,
        side: Side,
        price: Price,
        total_quantity: Quantity,
        display_quantity: Quantity,
    ) -> (OrderId, Vec<Trade>, Quantity, Quantity) {
        // 1. 先匹配第一批显示数量
        let (trades, remaining_display) =
            self.match_limit_order(trader, side, price, display_quantity);

        // 2. 计算已成交数量和剩余总数量
        let matched_qty = display_quantity - remaining_display;
        let mut remaining_total = total_quantity - matched_qty;

        // 3. 根据成交情况决定如何处理
        if remaining_display == 0 && remaining_total > 0 {
            // 情况A: 显示部分完全成交，还有剩余总量
            // 自动将下一批显示数量加入订单簿
            let next_display = std::cmp::min(remaining_total, display_quantity);
            let order_id = self.repository.allocate_order_id();
            let entry = OrderEntry::new(order_id, trader, next_display);
            let _ = self.repository.add_order(order_id, entry, side, price);

            // 更新剩余总量（减去已挂单的数量）
            remaining_total -= next_display;

            (order_id, trades, remaining_total, next_display)
        } else if remaining_display > 0 {
            // 情况B: 显示部分未完全成交
            // 将剩余的显示部分加入订单簿
            let order_id = self.repository.allocate_order_id();
            let entry = OrderEntry::new(order_id, trader, remaining_display);
            let _ = self.repository.add_order(order_id, entry, side, price);

            // 剩余总量需要减去挂单的数量
            remaining_total -= remaining_display;

            (order_id, trades, remaining_total, remaining_display)
        } else {
            // 情况C: 显示部分完全成交且无剩余总量
            // 冰山订单完全执行完毕
            (0, trades, 0, 0)
        }
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
        if let Some(ask_min) = self.repository.best_ask() {
            if price < ask_min {
                return; // 买价太低，直接返回
            }
        } else {
            return; // 没有卖单，直接返回
        }

        // 从最低卖价开始匹配（而不是从0开始）
        let mut current_price = self.repository.best_ask().unwrap();

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
        if let Some(bid_max) = self.repository.best_bid() {
            if price > bid_max {
                return; // 卖价太高，直接返回
            }
        } else {
            return; // 没有买单，直接返回
        }

        // 从最高买价开始匹配（而不是从 u32::MAX 开始）
        let mut current_price = self.repository.best_bid().unwrap();

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

        let mut current_idx = self
            .repository
            .get_first_order_at_price(price, opposite_side);
        let mut first_active_idx = None;

        while *remaining > 0 && current_idx.is_some() {
            let idx = current_idx.unwrap();

            if let Some(entry) = self.repository.get_entry_mut(idx) {
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
                        self.repository.cancel_order(order_id);

                        // 如果这是第一个活跃订单，重置标记
                        if first_active_idx == Some(idx) {
                            first_active_idx = None;
                        }
                    }
                }

                // 获取下一个订单索引
                current_idx = self.repository.get_next_order(idx);

                // 更新 first_active_idx
                if first_active_idx.is_none() && current_idx.is_some() {
                    if let Some(next_entry) = self.repository.get_entry(current_idx.unwrap()) {
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
            self.repository
                .update_price_point(price, opposite_side, None, None);
        } else if first_active_idx.is_some() {
            // 更新为第一个活跃订单
            self.repository
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
            if !self.repository.is_price_empty(price, side) {
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
            if !self.repository.is_price_empty(price, side) {
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
                // 事件溯源模式：1) 生成EntityEvent 2) replay事件到仓储

                let orders = self.repository.match_Order(price, quantity);
                //event中 包括旧order的update，新order的create;

                let (order_events, my_order_event, trades, order_id) =
                    create_trade(orders, trader, side);
                //存事件
                self.event_repo.save(order_events);

                //更新内存订单薄，对于已成交的就在内存中删掉，哈一区别
                self.repository.replay2(order_events);

                //更新数据库订单
                self.db_repository.replay2(order_events);

                CommandResult::LimitOrder { order_id, trades }
            }

            Command::MarketOrder {
                trader,
                side,
                quantity,
            } => {
                let transaction_id = self.allocate_transaction_id();
                let (events, trades, order_id) = self.match_limit_order_with_events(
                    trader,
                    side,
                    price,
                    quantity,
                    transaction_id,
                );

                // 应用所有事件到仓储
                let _ = self.repository.replay(events);

                let (trades, _remaining) = self.match_market_order(trader, side, quantity);
                CommandResult::MarketOrder { trades }
            }

            Command::IcebergOrder {
                trader,
                side,
                price,
                total_quantity,
                display_quantity,
            } => {
                let (order_id, trades, remaining_total, current_display) =
                    self.match_iceberg_order(trader, side, price, total_quantity, display_quantity);

                CommandResult::IcebergOrder {
                    order_id,
                    trades,
                    remaining_total,
                    current_display,
                }
            }

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
