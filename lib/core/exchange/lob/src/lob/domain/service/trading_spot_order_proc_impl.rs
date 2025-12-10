use crate::event;
use crate::lob::adaptor::outbound::entity_repo::EntityRepo;
use crate::lob::adaptor::outbound::event_repo::EventRepo;
use crate::lob::adaptor::outbound::id_repo::IdRepo;
use crate::lob::domain::entity::lob_types::Side::{Buy, Sell};
use crate::lob::domain::entity::lob_types::{EntityEvent, Trade};
use crate::lob::domain::entity::lob_types::{
    EventOperation, FieldValue, OrderEntry, OrderId, Price, Quantity,
};
use crate::lob::domain::repo::OrderRepo;
/// 订单匹配服务
///
/// 实现价格-时间优先的订单匹配算法
/// 遵循Clean Architecture的领域服务模式
use crate::lob::domain::service::trading_spot_order_proc::{
    CommandResponse, CommonError, IdempotentSpotCommand, IdempotentSpotResult, OrderStatus,
    SpotCommand, SpotCommandError, SpotCommandResult, SpotOrderProc, TimeInForce,
};
use account::{
    AccountCommand, AccountCommandResult, AccountId, AccountService, BalanceError, TradingPair,
};

/// 匹配服务
///
/// 负责订单匹配逻辑，持有OrderRepository引用
/// 遵循Clean Architecture：通过trait注入AccountService依赖
pub struct SpotMatchingService<R, A>
where
    R: OrderRepo,
    A: AccountService,
{
    lob_repo: R,
    account_service: A,
    trading_pair: TradingPair,
    event_repo: EventRepo,
    entity_repo: EntityRepo,
    id_repo: IdRepo,
}

impl<R, A> SpotMatchingService<R, A>
where
    R: OrderRepo,
    A: AccountService,
{
    /// 创建新的匹配服务
    ///
    /// # 参数
    /// - `repo`: 订单仓储
    /// - `account_service`: 账户服务（用于余额检查和冻结）
    /// - `trading_pair`: 交易对配置
    pub fn new(repository: R, account_service: A, trading_pair: TradingPair) -> Self {
        Self {
            lob_repo: repository,
            account_service,
            trading_pair,
            event_repo: EventRepo {},
            id_repo: IdRepo {},
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
    fn create_events(
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

impl<R, A> SpotOrderProc for SpotMatchingService<R, A>
where
    R: OrderRepo + Send + Sync,
    A: AccountService,
{
    fn handle(&mut self, cmd: IdempotentSpotCommand) -> IdempotentSpotResult {
        let nonce = cmd.nonce;

        // TODO: 检查 nonce 是否已处理（幂等性检查）
        // if let Some(cached_result) = self.nonce_cache.get(&nonce) {
        //     return Ok(CommandResponse::duplicate(nonce, cached_result.clone()));
        // }

        // 执行命令，获取Result
        let result = match cmd.payload {
            SpotCommand::LimitOrder { .. } => self.handle_limit_order(cmd.payload)?,
            SpotCommand::MarketOrder { .. } => self.handle_market_order(cmd.payload)?,
            SpotCommand::CancelOrder { order_id } => {
                let success = self.cancel_order(order_id);
                if !success {
                    return Err(SpotCommandError::Common(CommonError::OrderNotFound {
                        order_id,
                    }));
                }
                SpotCommandResult::CancelOrder {
                    order_id,
                    status: OrderStatus::Cancelled,
                }
            }
            SpotCommand::CancelAllOrders { .. } => {
                return Err(SpotCommandError::Common(CommonError::InvalidParameter {
                    field: "command",
                    reason: "CancelAllOrders not implemented yet",
                }));
            }
        };

        // TODO: 缓存结果用于幂等性
        // self.nonce_cache.insert(nonce, result.clone());

        Ok(CommandResponse::new(nonce, result))
    }
}

impl<R, A> SpotMatchingService<R, A>
where
    R: OrderRepo + Send + Sync,
    A: AccountService,
{
    fn handle_limit_order(
        &mut self,
        command: SpotCommand,
    ) -> Result<SpotCommandResult, SpotCommandError> {
        let _trades: Vec<Trade> = Vec::new(); // TODO: 从 trade_events 转换

        if let SpotCommand::LimitOrder {
            trader,
            symbol: _symbol, // TODO: 使用 symbol 路由到正确的订单簿
            side,
            price,
            quantity,
            time_in_force,
            client_order_id: _client_order_id, // TODO: 存储 client_order_id
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
                AccountCommandResult::Error(BalanceError::InsufficientAvailable {
                    required,
                    available,
                }) => {
                    // 余额不足
                    return Err(SpotCommandError::Common(CommonError::InsufficientBalance {
                        required,
                        available,
                    }));
                }
                AccountCommandResult::Error(_) => {
                    // 其他错误
                    return Err(SpotCommandError::Common(CommonError::Internal {
                        message: "Account service error".to_string(),
                    }));
                }
                _ => {
                    // 不应发生的情况
                    return Err(SpotCommandError::Common(CommonError::Internal {
                        message: "Unexpected account command result".to_string(),
                    }));
                }
            }

            // 账户冻结成功，继续撮合流程
            let orders = self.lob_repo.match_orders(side, price, quantity);

            // ========== PostOnly 检查：如果会立即成交则拒绝 ==========
            if time_in_force == TimeInForce::PostOnly {
                if orders.is_some() && orders.as_ref().unwrap().len() > 0 {
                    // PostOnly 订单会立即成交，拒绝
                    return Ok(SpotCommandResult::LimitOrder {
                        order_id: new_order_id,
                        status: OrderStatus::Rejected,
                        filled_quantity: 0,
                        remaining_quantity: quantity,
                        trades: Vec::new(),
                    });
                }
                // 不会立即成交，继续处理为 GTC
            }

            if orders.is_some() && orders.as_ref().unwrap().len() > 0 {
                let (order_change_events, trade_create_events, unfilled_amount) =
                    self.create_events(orders, price, quantity);

                let filled_quantity = quantity - unfilled_amount;

                // ========== TimeInForce 执行逻辑 ==========
                match time_in_force {
                    // FOK: 全部成交或全部拒绝
                    TimeInForce::FillOrKill => {
                        if unfilled_amount > 0 {
                            // 未能全部成交，拒绝订单
                            return Ok(SpotCommandResult::LimitOrder {
                                order_id: new_order_id,
                                status: OrderStatus::Rejected,
                                filled_quantity: 0,
                                remaining_quantity: quantity,
                                trades: Vec::new(),
                            });
                        }
                        // 全部成交，继续处理
                    }

                    // IOC: 立即成交，未成交部分取消
                    TimeInForce::ImmediateOrCancel => {
                        // 保存已成交的事件
                        self.event_repo.save_batch(order_change_events.clone());
                        self.event_repo.save_batch(trade_create_events.clone());

                        if let Some(events) = order_change_events.clone() {
                            let _ = self.lob_repo.replay(events);
                        }
                        self.entity_repo.replay(order_change_events);
                        self.entity_repo.replay(trade_create_events);

                        // 返回结果，不挂单
                        return Ok(SpotCommandResult::LimitOrder {
                            order_id: new_order_id,
                            status: if unfilled_amount == 0 {
                                OrderStatus::Filled
                            } else {
                                OrderStatus::Cancelled
                            },
                            filled_quantity,
                            remaining_quantity: unfilled_amount,
                            trades: Vec::new(), // TODO: 从 trade_events 转换
                        });
                    }

                    // GTC/GTD/PostOnly: 标准流程，挂单等待
                    TimeInForce::GoodTillCancel
                    | TimeInForce::GoodTillDate(_)
                    | TimeInForce::PostOnly => {
                        // 继续标准流程
                    }
                }

                // ========== 标准订单创建流程 (GTC/GTD/FOK全部成交) ==========
                let transaction_id = self.id_repo.next_transaction_id_u64();
                let event_id = self.id_repo.next_event_id_u64();

                // 创建订单创建事件（如果有未成交部分）
                if unfilled_amount > 0 {
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
                    self.event_repo.save(order_create_event);
                }

                // 保存事件
                self.event_repo.save_batch(order_change_events.clone());
                self.event_repo.save_batch(trade_create_events.clone());

                // 回放事件到订单簿
                if let Some(events) = order_change_events.clone() {
                    let _ = self.lob_repo.replay(events);
                }

                // 订单实体落库
                self.entity_repo.replay(order_change_events);
                self.entity_repo.replay(trade_create_events);

                // 返回成交结果
                return Ok(SpotCommandResult::LimitOrder {
                    order_id: new_order_id,
                    status: if unfilled_amount == 0 {
                        OrderStatus::Filled
                    } else {
                        OrderStatus::PartiallyFilled
                    },
                    filled_quantity,
                    remaining_quantity: unfilled_amount,
                    trades: Vec::new(), // TODO: 从 trade_events 转换
                });
            } else {
                // ========== 无成交情况 ==========
                match time_in_force {
                    // FOK: 无成交则拒绝
                    TimeInForce::FillOrKill => {
                        return Ok(SpotCommandResult::LimitOrder {
                            order_id: new_order_id,
                            status: OrderStatus::Rejected,
                            filled_quantity: 0,
                            remaining_quantity: quantity,
                            trades: Vec::new(),
                        });
                    }

                    // IOC: 无成交则取消
                    TimeInForce::ImmediateOrCancel => {
                        return Ok(SpotCommandResult::LimitOrder {
                            order_id: new_order_id,
                            status: OrderStatus::Cancelled,
                            filled_quantity: 0,
                            remaining_quantity: quantity,
                            trades: Vec::new(),
                        });
                    }

                    // GTC/GTD/PostOnly: 挂单等待
                    TimeInForce::GoodTillCancel
                    | TimeInForce::GoodTillDate(_)
                    | TimeInForce::PostOnly => {
                        // 创建订单事件并挂单
                        let transaction_id = self.id_repo.next_transaction_id_u64();
                        let event_id = self.id_repo.next_event_id_u64();

                        let order_create_event = event!(
                            "Order", EventOperation::Create, event_id, transaction_id, new_order_id => {
                                create:
                                    "order_id" => FieldValue::OrderId(new_order_id),
                                    "trader" => FieldValue::TraderId(trader),
                                    "side" => FieldValue::Side(side),
                                    "quantity" => FieldValue::Quantity(quantity),
                                    "price" => FieldValue::U32(price),
                            }
                        );

                        self.event_repo.save(order_create_event);
                        // TODO: 在lob中挂单
                        // self.lob_repo.add_order(order);

                        return Ok(SpotCommandResult::LimitOrder {
                            order_id: new_order_id,
                            status: OrderStatus::Pending,
                            filled_quantity: 0,
                            remaining_quantity: quantity,
                            trades: Vec::new(),
                        });
                    }
                }
            }
        }

        // 如果不是 LimitOrder，返回错误
        Err(SpotCommandError::Common(CommonError::Internal {
            message: "Unexpected command type".to_string(),
        }))
    }

    /// 处理市价单
    ///
    /// 市价单不会挂在订单簿上，而是立即尝试撮合：
    /// - 默认行为 (None) 或 IOC: 尽量成交，剩余取消
    /// - FOK: 全部成交或全部拒绝
    fn handle_market_order(
        &mut self,
        command: SpotCommand,
    ) -> Result<SpotCommandResult, SpotCommandError> {
        if let SpotCommand::MarketOrder {
            trader,
            symbol: _symbol,
            side,
            quantity,
            price_limit,
            time_in_force,
            client_order_id: _client_order_id,
        } = command
        {
            // 1. 验证 TimeInForce（市价单只允许 None/FOK/IOC）
            if let Some(tif) = time_in_force {
                match tif {
                    TimeInForce::FillOrKill | TimeInForce::ImmediateOrCancel => {
                        // 允许
                    }
                    _ => {
                        return Err(SpotCommandError::InvalidTimeInForce {
                            reason: "MarketOrder only supports None(IOC), FOK, or IOC",
                        });
                    }
                }
            }

            // 2. FOK 模式需要预先检查深度
            if time_in_force == Some(TimeInForce::FillOrKill) {
                // 检查订单簿深度是否足够
                let available_quantity = self.lob_repo.get_available_quantity(side, price_limit);
                if available_quantity < quantity {
                    // 深度不足，直接拒绝
                    return Ok(SpotCommandResult::MarketOrder {
                        status: OrderStatus::Rejected,
                        filled_quantity: 0,
                        trades: Vec::new(),
                    });
                }
            }

            // 3. 生成临时订单ID（用于账户结算，但不会挂单）
            let temp_order_id = self.lob_repo.allocate_order_id();

            // 4. 将 TraderId 转换为 AccountId
            let account_id = AccountId(u64::from_le_bytes(*trader.as_bytes()));
            let account_side = match side {
                Buy => account::Side::Buy,
                Sell => account::Side::Sell,
            };

            // 5. 市价单撮合前也需要检查和冻结账户余额
            // 对于市价买单，需要估算最大花费（使用 price_limit 或市场最差价）
            // 对于市价卖单，冻结 base_asset 数量
            let (freeze_price, freeze_quantity) = match side {
                Buy => {
                    // 买单：使用 price_limit 作为最高价，如果没有则使用订单簿最高卖价 * 1.1（保护机制）
                    let max_price = price_limit.unwrap_or_else(|| {
                        self.lob_repo
                            .get_best_ask()
                            .map(|p| (p as f64 * 1.1) as u32)
                            .unwrap_or(u32::MAX)
                    });
                    (max_price as u64, quantity as u64)
                }
                Sell => {
                    // 卖单：直接冻结卖出数量
                    (price_limit.unwrap_or(0) as u64, quantity as u64)
                }
            };

            let freeze_cmd = AccountCommand::CheckAndFreeze {
                account_id,
                order_id: temp_order_id,
                pair: self.trading_pair,
                side: account_side,
                price: freeze_price,
                quantity: freeze_quantity,
            };

            let freeze_result = self.account_service.execute(freeze_cmd);

            // 6. 检查冻结结果
            match freeze_result {
                AccountCommandResult::Frozen { .. } => {
                    // 冻结成功，继续撮合
                }
                AccountCommandResult::Error(BalanceError::InsufficientAvailable {
                    required,
                    available,
                }) => {
                    return Err(SpotCommandError::Common(CommonError::InsufficientBalance {
                        required,
                        available,
                    }));
                }
                AccountCommandResult::Error(_) => {
                    return Err(SpotCommandError::Common(CommonError::Internal {
                        message: "Account service error".to_string(),
                    }));
                }
                _ => {
                    return Err(SpotCommandError::Common(CommonError::Internal {
                        message: "Unexpected account command result".to_string(),
                    }));
                }
            }

            // 7. 执行市价单撮合（使用特殊的 price）
            // 市价买单：使用 u32::MAX 确保吃掉所有卖单
            // 市价卖单：使用 0 确保吃掉所有买单
            let match_price = match side {
                Buy => price_limit.unwrap_or(u32::MAX),
                Sell => price_limit.unwrap_or(0),
            };

            let orders = self.lob_repo.match_orders(side, match_price, quantity);

            // 8. 处理撮合结果
            if orders.is_some() && orders.as_ref().unwrap().len() > 0 {
                let (order_change_events, trade_create_events, unfilled_amount) =
                    self.create_events(orders, match_price, quantity);

                let filled_quantity = quantity - unfilled_amount;

                // 9. 根据 TimeInForce 决定最终状态
                let status = match time_in_force {
                    Some(TimeInForce::FillOrKill) => {
                        if unfilled_amount > 0 {
                            // FOK: 未全部成交，理论上不应发生（因为已经预检查）
                            // 回滚事件，不保存任何成交
                            return Ok(SpotCommandResult::MarketOrder {
                                status: OrderStatus::Rejected,
                                filled_quantity: 0,
                                trades: Vec::new(),
                            });
                        }
                        OrderStatus::Filled
                    }
                    _ => {
                        // None 或 IOC: 部分成交也接受
                        if unfilled_amount == 0 {
                            OrderStatus::Filled
                        } else {
                            OrderStatus::Cancelled
                        }
                    }
                };

                // 10. 保存事件
                self.event_repo.save_batch(order_change_events.clone());
                self.event_repo.save_batch(trade_create_events.clone());

                if let Some(events) = order_change_events.clone() {
                    let _ = self.lob_repo.replay(events);
                }
                self.entity_repo.replay(order_change_events);
                self.entity_repo.replay(trade_create_events);

                // 11. 返回结果（不包含 order_id，因为市价单不挂单）
                return Ok(SpotCommandResult::MarketOrder {
                    status,
                    filled_quantity,
                    trades: Vec::new(), // TODO: 从 trade_create_events 转换
                });
            } else {
                // 没有匹配到任何订单
                return Ok(SpotCommandResult::MarketOrder {
                    status: if time_in_force == Some(TimeInForce::FillOrKill) {
                        OrderStatus::Rejected
                    } else {
                        OrderStatus::Cancelled
                    },
                    filled_quantity: 0,
                    trades: Vec::new(),
                });
            }
        }

        // 如果不是 MarketOrder，返回错误
        Err(SpotCommandError::Common(CommonError::Internal {
            message: "Unexpected command type in handle_market_order".to_string(),
        }))
    }
}
