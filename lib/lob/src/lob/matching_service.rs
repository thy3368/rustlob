use crate::lob::types::lob_types::EntityEvent;
/// 订单匹配服务
///
/// 实现价格-时间优先的订单匹配算法
/// 遵循Clean Architecture的领域服务模式
use super::handler::{Command, CommandResult, OrderCommandHandler};
use super::repository::{OrderRepository, RepositoryAccessor};
use crate::event;
use crate::lob::repository::entity_repo::EntityRepo;
use crate::lob::repository::event_repo::EventRepo;
use crate::lob::repository::id_repo::IdRepo;
use crate::lob::types::lob_types::{EventOperation, FieldValue, OrderEntry, OrderId, Price, Quantity};

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
    id_repo: IdRepo,
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
        matched_orders: Option<Vec<&mut OrderEntry>>,
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

    // /// 分配新的事件ID
    // fn allocate_event_id(&mut self) -> u64 {
    //     let id = self.event_id_counter;
    //     self.event_id_counter += 1;
    //     id
    // }

    // /// 分配新的事务ID
    // fn allocate_transaction_id(&mut self) -> u64 {
    //     let id = self.transaction_id_counter;
    //     self.transaction_id_counter += 1;
    //     id
    // }

    /// 获取repository的可变引用
    pub fn repository_mut(&mut self) -> &mut R {
        &mut self.lob_repo
    }

    /// 获取repository的不可变引用
    pub fn repository(&self) -> &R {
        &self.lob_repo
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

                CommandResult::LimitOrder {
                    order_id: new_order_id,
                    trades,
                }
            }

            Command::MarketOrder {
                trader,
                side,
                quantity,
            } => CommandResult::ToDo {},

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
