use crate::event;
use crate::lob::adaptor::outbound::entity_repo::EntityRepo;
use crate::lob::adaptor::outbound::event_repo::EventRepo;
use crate::lob::adaptor::outbound::id_repo::IdRepo;
use crate::lob::domain::entity::lob_types::Side::{Buy, Sell};
use crate::lob::domain::entity::lob_types::{EntityEvent, Trade};
use crate::lob::domain::entity::lob_types::{
    EventOperation, FieldValue, OrderEntry, OrderId, Price, Quantity,
};
use crate::lob::domain::repository::OrderRepository;
/// 订单匹配服务
///
/// 实现价格-时间优先的订单匹配算法
/// 遵循Clean Architecture的领域服务模式
use crate::lob::domain::service::handler::{SpotCommand, SpotCommandResult, SpotOrderHandler};
use account::{
    AccountCommand, AccountCommandResult, AccountId, AccountService, BalanceError, TradingPair,
};

/// 匹配服务
///
/// 负责订单匹配逻辑，持有OrderRepository引用
/// 遵循Clean Architecture：通过trait注入AccountService依赖
pub struct MatchingService<R, A>
where
    R: OrderRepository,
    A: AccountService,
{
    lob_repo: R,
    account_service: A,
    trading_pair: TradingPair,
    event_repo: EventRepo,
    entity_repo: EntityRepo,
    id_repo: IdRepo,
}

impl<R, A> MatchingService<R, A>
where
    R: OrderRepository,
    A: AccountService,
{
    /// 创建新的匹配服务
    ///
    /// # 参数
    /// - `repository`: 订单仓储
    /// - `account_service`: 账户服务（用于余额检查和冻结）
    /// - `trading_pair`: 交易对配置
    pub fn new(repository: R, account_service: A, trading_pair: TradingPair) -> Self {
        Self {
            lob_repo: repository,
            account_service,
            trading_pair,
            event_repo: EventRepo {},
            id_repo: IdRepo {
                event_id_counter: 1,
                transaction_id_counter: 1,
            },
            entity_repo: EntityRepo {},
        }
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
        &self,
        matched_orders: Option<Vec<&OrderEntry>>,
        price: Price,
        quantity: Quantity,
    ) -> (Option<Vec<EntityEvent>>, Option<Vec<EntityEvent>>, Quantity) {
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
                // matched_order.unfilled_quantity = new_unfilled;
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
}

impl<R, A> SpotOrderHandler for MatchingService<R, A>
where
    R: OrderRepository + Send + Sync,
    A: AccountService,
{
    fn handle(&mut self, cmd: SpotCommand) -> SpotCommandResult {
        match cmd {
            SpotCommand::LimitOrder { .. } => self.handle_limit_order(cmd),
            SpotCommand::MarketOrder { .. } => SpotCommandResult::ToDo {},
            SpotCommand::IcebergOrder { .. } => SpotCommandResult::ToDo {},
            SpotCommand::CancelOrder { order_id } => {
                SpotCommandResult::CancelOrder { success: self.cancel_order(order_id) }
            }
            _ => SpotCommandResult::ToDo {},
        }
    }
}

impl<R, A> MatchingService<R, A>
where
    R: OrderRepository + Send + Sync,
    A: AccountService,
{
    fn handle_limit_order(&mut self, command: SpotCommand) -> SpotCommandResult {
        let trades: Vec<Trade> = Vec::new(); // TODO: 从 trade_events 转换

        if let SpotCommand::LimitOrder {
            trader,
            side,
            price,
            quantity,
        } = command
        {
            // 生成订单ID（用于账户冻结关联）
            let new_order_id = self.lob_repo.allocate_order_id();

            // 将 TraderId 转换为 AccountId（取前8字节作为u64）
            let account_id = AccountId(u64::from_le_bytes(*trader.as_bytes()));

            // 将 LOB 的 Side 转换为 Account 的 Side
            let account_side = match side {
                Buy => account::Side::Buy,
                Sell => account::Side::Sell,
            };

            // 下单前检查并冻结账户余额（原子操作）
            // Buy: 冻结 quote_asset (如 USDT), 金额 = price * quantity
            // Sell: 冻结 base_asset (如 BTC), 金额 = quantity
            let freeze_cmd = AccountCommand::CheckAndFreeze {
                account_id,
                order_id: new_order_id,
                pair: self.trading_pair,
                side: account_side,
                price: price as u64,
                quantity: quantity as u64,
            };

            let freeze_result = self.account_service.execute(freeze_cmd);

            // 检查冻结结果
            match freeze_result {
                AccountCommandResult::Frozen { .. } => {
                    // 冻结成功，继续下单流程
                }
                AccountCommandResult::Error(err) => {
                    // 余额不足或其他错误，返回失败
                    return SpotCommandResult::AccountCheckFailed { error: err };
                }
                _ => {
                    // 不应发生的情况
                    return SpotCommandResult::AccountCheckFailed {
                        error: BalanceError::Overflow,
                    };
                }
            }

            // 账户冻结成功，继续撮合流程
            let orders = self.lob_repo.match_orders(side, price, quantity);

            if orders.is_some() && orders.as_ref().unwrap().len() > 0 {
                let (order_change_events, trade_create_events, unfilled_amount) =
                    self.create_trades(orders, price, quantity);

                // 创建新订单事件（如果有未成交部分）

                let transaction_id = self.id_repo.allocate_transaction_id();
                let event_id = self.id_repo.allocate_event_id();

                // 创建订单创建事件
                let order_create_event = event!(
                    "Order", EventOperation::Create, event_id, transaction_id, new_order_id => {
                        create:
                            "order_id" => FieldValue::OrderId(new_order_id),
                            "trader" => FieldValue::TraderId(trader),
                            "side" => FieldValue::Side(side),
                            "quantity" => FieldValue::Quantity(unfilled_amount),
                            "price" => FieldValue::U32(price),
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

            // CommandResult::LimitOrder {
            //     order_id: new_order_id,
            //     trades,
            // }
        }

        SpotCommandResult::LimitOrder {
            order_id: 23232,
            trades: Vec::new(),
        }
    }
}
