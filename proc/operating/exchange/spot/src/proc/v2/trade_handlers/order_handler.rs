//! 订单处理器
//!
//! 负责处理订单相关命令：
//! - NewOrder: 创建新订单
//! - CancelOrder: 取消订单
//! - QueryOrder: 查询订单
//! - CancelReplaceOrder: 撤销并替换订单

use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::account::error::BalanceError;
use base_types::cqrs::cqrs_types::{CmdResp, ResMetadata};
use base_types::exchange::spot::spot_types::{OrderSide, OrderType, SpotOrder, SpotTrade};
use base_types::{AccountId, AssetId, Quantity, Timestamp, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::{ChangeLog, ChangeType, Entity};
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    NewOrderAck, NewOrderCmd, SpotTradeResAny,
};
use crate::proc::v2::id_repo::order_next_id;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;

/// 订单处理器
pub struct OrderHandler {
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,
    lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
    /// 事件发布器（支持 Disruptor/Kafka 切换）
    event_publisher: Arc<dyn EventPublisher>,
}

impl OrderHandler {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self { balance_repo, trade_repo, order_repo, lob_repo, event_publisher }
    }

    /// 构造余额ID
    ///
    /// # 参数
    /// - `asset_id`: 资产ID
    ///
    /// # 返回
    /// 格式化的余额ID: "account_id:asset_id"
    ///
    /// # 性能优化
    /// - 使用 `format!` 而非字符串拼接
    /// - 考虑使用缓存（如果频繁调用）
    #[inline]
    fn query_balance_id(&self, asset_id: AssetId) -> String {
        // TODO: 从订单或上下文中获取真实的 account_id
        // 当前硬编码为 AccountId(1)
        format!("{}:{}", AccountId(1).0, u32::from(asset_id))
    }

    /// 查询并锁定余额
    ///
    /// # 参数
    /// - `asset_id`: 资产ID
    /// - `order_id`: 订单ID（用于错误日志）
    ///
    /// # 返回
    /// - `Ok(Balance)`: 查询到的余额（已加锁）
    /// - `Err(SpotCmdErrorAny)`: 查询失败或余额不存在
    ///
    /// # 说明
    /// 使用 `find_by_id_4_update` 获取行锁，防止并发修改
    fn find_and_lock_balance(
        &self,
        asset_id: AssetId,
        order_id: &str,
    ) -> Result<Balance, SpotCmdErrorAny> {
        let balance_id = self.query_balance_id(asset_id);

        self.balance_repo
            .find_by_id_4_update(&balance_id)
            .map_err(|e| {
                tracing::error!(
                    order_id = %order_id,
                    asset_id = ?asset_id,
                    balance_id = %balance_id,
                    error = ?e,
                    "Failed to query balance from database"
                );
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Database error: {}", e),
                })
            })?
            .ok_or_else(|| {
                tracing::warn!(
                    order_id = %order_id,
                    asset_id = ?asset_id,
                    balance_id = %balance_id,
                    "Balance not found"
                );
                SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "balance",
                    reason: "balance not found",
                })
            })
    }

    /// 冻结资金并生成变更日志
    ///
    /// # 流程
    /// 1. 确定需要冻结的资产（买单冻结报价资产，卖单冻结基础资产）
    /// 2. 查询并锁定余额
    /// 3. 执行冻结操作
    /// 4. 生成变更日志
    ///
    /// # 参数
    /// - `internal_order`: 订单引用
    ///
    /// # 返回
    /// - `Ok(ChangeLogEntry)`: 余额变更日志
    /// - `Err(SpotCmdErrorAny)`: 冻结失败错误
    ///
    /// # 错误场景
    /// - 余额不存在
    /// - 余额不足
    /// - 数据库错误
    fn frozen_balance(
        &self,
        internal_order: &SpotOrder,
    ) -> Result<ChangeLog, SpotCmdErrorAny> {
        // 1. 确定需要冻结的资产
        let frozen_asset_id = internal_order.frozen_asset_id();

        // 2. 查询并锁定余额
        let mut balance =
            self.find_and_lock_balance(frozen_asset_id, &internal_order.order_id.to_string())?;

        // 3. 执行冻结并生成变更日志
        self.freeze_and_track(&mut balance, internal_order)
    }

    /// 执行冻结操作并生成变更日志
    ///
    /// # 参数
    /// - `balance`: 可变余额引用
    /// - `order`: 订单引用
    ///
    /// # 返回
    /// - `Ok(ChangeLogEntry)`: 余额变更日志
    /// - `Err(SpotCmdErrorAny)`: 冻结失败错误
    ///
    /// # 说明
    /// 此方法只做计算，不做 I/O 操作
    #[inline]
    fn freeze_and_track(
        &self,
        balance: &mut Balance,
        order: &SpotOrder,
    ) -> Result<ChangeLog, SpotCmdErrorAny> {
        // 1. 计算需要冻结的金额
        let frozen_amount = self.calculate_frozen_amount(order);

        // 2. 保存旧状态（用于生成变更日志）
        let old_balance = balance.clone();

        // 3. 执行冻结
        balance
            .frozen(frozen_amount, Timestamp::now_as_nanos())
            .map_err(|e| self.map_balance_error(e, order, frozen_amount))?;

        // 4. 生成变更日志
        balance.track_update_from(&old_balance).map_err(|e| {
            tracing::error!(
                order_id = %order.order_id,
                error = ?e,
                "Failed to track balance update"
            );
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to track balance update: {}", e),
            })
        })
    }

    /// 计算需要冻结的金额
    ///
    /// # 参数
    /// - `order`: 订单引用
    ///
    /// # 返回
    /// 需要冻结的金额
    ///
    /// # 规则
    /// - 买单：冻结报价资产（quote）= price * quantity
    /// - 卖单：冻结基础资产（base）= quantity
    #[inline]
    fn calculate_frozen_amount(&self, order: &SpotOrder) -> Quantity {
        match order.side {
            OrderSide::Buy => order.total_quote_qty,
            OrderSide::Sell => order.total_base_qty,
        }
    }

    /// 映射余额错误到业务错误
    ///
    /// # 参数
    /// - `error`: 余额错误
    /// - `order`: 订单引用
    /// - `required_amount`: 需要的金额
    ///
    /// # 返回
    /// 业务错误
    #[inline]
    fn map_balance_error(
        &self,
        error: BalanceError,
        order: &SpotOrder,
        required_amount: Quantity,
    ) -> SpotCmdErrorAny {
        match error {
            BalanceError::InsufficientAvailable { required, available } => {
                tracing::warn!(
                    order_id = %order.order_id,
                    side = ?order.side,
                    required = %required,
                    available = %available,
                    "Insufficient balance"
                );
                SpotCmdErrorAny::Common(CommonError::InsufficientBalance {
                    required: required as u64,
                    available: available as u64,
                })
            }
            _ => {
                tracing::error!(
                    order_id = %order.order_id,
                    error = ?error,
                    required_amount = %required_amount,
                    "Balance operation failed"
                );
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Balance error: {}", error),
                })
            }
        }
    }

    /// 发送日志到事件队列
    ///
    /// # 参数
    /// - `order_log`: 订单变更日志
    /// - `balance_log`: 余额变更日志（可选）
    ///
    /// # 说明
    /// 发送失败只记录错误日志，不影响主流程
    #[inline]
    fn publish_logs(&self, order_log: &ChangeLog, balance_log: Option<&ChangeLog>) {
        // 发送余额日志
        if let Some(log) = balance_log {
            if let Err(e) = self.event_publisher.publish_balance_log(log) {
                tracing::error!(
                    order_id = %order_log.entity_id(),
                    error = ?e,
                    "Failed to publish balance log"
                );
            }
        }

        // 发送订单日志
        if let Err(e) = self.event_publisher.publish_order_log(order_log) {
            tracing::error!(
                order_id = %order_log.entity_id(),
                error = ?e,
                "Failed to publish order log"
            );
        }
    }

    /// 处理新订单
    ///
    /// # 流程
    /// 1. 验证命令并转换为内部订单
    /// 2. 冻结资金
    /// 3. 生成变更日志
    /// 4. 发送日志到队列
    /// 5. 返回响应
    ///
    /// # 参数
    /// - `cmd`: 新订单命令
    ///
    /// # 返回
    /// - `Ok(CmdResp)`: 订单创建成功
    /// - `Err(SpotCmdErrorAny)`: 订单创建失败（验证失败、余额不足等）
    ///
    /// # 注意
    /// - 分布式定序器应在此函数调用前完成
    /// - 日志发送失败不影响订单创建结果
    pub fn handle_new_order(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 1. 验证命令并转换为内部订单
        let mut internal_order = self.validate_cmd_cpu(cmd)?;

        // 2. 尝试冻结资金
        let balance_log_result = self.frozen_balance(&internal_order);

        // 3. 根据冻结结果处理订单
        match balance_log_result {
            Ok(balance_log) => {
                // 冻结成功：订单接受
                self.handle_order_accepted(internal_order, balance_log)
            }
            Err(err) => {
                // 冻结失败：订单拒绝
                self.handle_order_rejected(internal_order, err)
            }
        }
    }

    /// 处理订单接受场景
    ///
    /// # 参数
    /// - `order`: 内部订单
    /// - `balance_log`: 余额变更日志
    ///
    /// # 返回
    /// 订单创建成功响应
    #[inline]
    fn handle_order_accepted(
        &self,
        order: SpotOrder,
        balance_log: ChangeLog,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 1. 生成订单日志
        let order_log = order.track_create();

        // 2. 发送日志到队列
        self.publish_logs(&order_log.unwrap(), Some(&balance_log));

        // 3. 构造成功响应
        Ok(CmdResp::new(
            ResMetadata::new(0, false, Timestamp::default()),
            SpotTradeResAny::TestNewOrderEmpty, // TODO: 返回正确的响应类型
        ))
    }

    /// 处理订单拒绝场景
    ///
    /// # 参数
    /// - `mut order`: 内部订单（可变）
    /// - `error`: 拒绝原因
    ///
    /// # 返回
    /// 订单创建失败错误
    #[inline]
    fn handle_order_rejected(
        &self,
        mut order: SpotOrder,
        error: SpotCmdErrorAny,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 1. 标记订单为拒绝状态
        order.state.status = base_types::exchange::spot::spot_types::OrderStatus::Rejected;
        order.state.last_updated = Timestamp::now_as_nanos();

        // 2. 生成拒绝订单日志
        let order_log = order.track_create();

        // 3. 发送日志到队列
        self.publish_logs(&order_log.unwrap(), None);

        // 4. 返回错误
        Err(error)
    }

    // ========== 私有辅助方法 ==========

    /// 验证订单命令参数（优化D：参数验证前置）
    fn validate_order_cmd(&self, cmd: &NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
        // 验证数量必须大于0
        if let Some(qty) = cmd.quantity() {
            if qty.is_zero() {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "quantity",
                    reason: "must be greater than 0",
                }));
            }
        }

        // 验证价格（限价单必须提供价格）
        match cmd.order_type() {
            OrderType::Limit
            | OrderType::StopLossLimit
            | OrderType::TakeProfitLimit
            | OrderType::LimitMaker => {
                if cmd.price().is_none() {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "price",
                        reason: "required for limit orders",
                    }));
                }
            }
            _ => {} // 市价单不需要价格
        }

        // 验证止损/止盈价格（条件单必须提供）
        match cmd.order_type() {
            OrderType::StopLoss
            | OrderType::StopLossLimit
            | OrderType::TakeProfit
            | OrderType::TakeProfitLimit => {
                if cmd.stop_price().is_none() {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "stop_price",
                        reason: "required for conditional orders",
                    }));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// 验证交易对是否支持
    ///
    /// # 参数
    /// - `symbol`: 交易对
    ///
    /// # 返回
    /// - `true`: 支持该交易对
    /// - `false`: 不支持该交易对
    ///
    /// # 说明
    /// 当前实现检查 LOB 仓储中是否存在该交易对
    /// 生产环境应该从配置或数据库中加载支持的交易对列表
    #[inline]
    fn is_symbol_supported(&self, symbol: TradingPair) -> bool {
        self.lob_repo.contains_symbol(&symbol)
    }

    #[inline]
    pub(crate) fn handle_post(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        self.validate_order_cmd(&cmd)?;

        let symbol = *cmd.symbol();
        if !self.is_symbol_supported(symbol) {
            return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                field: "symbol",
                reason: "trading pair not supported",
            }));
        }

        let order_id = order_next_id() as u64;
        let timestamp = Timestamp::now_as_nanos();

        let cmd_log = ChangeLog::new(
            order_id.to_string(),
            "NewOrderCmd".to_string(),
            ChangeType::Created { fields: Vec::new() },
            timestamp.0,
            0,
        );

        self.event_publisher.publish_order_log(&cmd_log).map_err(|e| {
            tracing::error!(order_id, symbol = %symbol, error = %e, "Failed to route order");
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to route order: {}", e),
            })
        })?;

        let ack = NewOrderAck::new(
            symbol,
            order_id,
            -1,
            cmd.new_client_order_id().as_ref().map(|s| s.clone()),
            timestamp,
        );

        Ok(CmdResp::new(ResMetadata::new(0, false, timestamp), SpotTradeResAny::NewOrderAck(ack)))
    }

    /// 同步接受新订单
    ///
    /// # 流程
    /// 1. 参数校验
    /// 2. 交易对校验
    /// 3. 生成订单 ID 和时间戳
    /// 4. 构造初始订单（状态为 New）
    /// 5. 同步写入 MySQL
    /// 6. 发布 Kafka 事件
    /// 7. 返回 NewOrderAck
    ///
    /// # 参数
    /// - `cmd`: 新订单命令
    ///
    /// # 返回
    /// - `Ok(NewOrderAck)`: 订单接受成功
    /// - `Err(SpotCmdErrorAny)`: 订单接受失败（参数错误、交易对不支持、持久化失败等）
    ///
    /// # 注意
    /// - 冻结资金由异步线程处理
    /// - 撮合由异步线程处理
    pub fn accept_new_order(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<NewOrderAck, SpotCmdErrorAny> {
        // 1. 参数校验
        self.validate_order_cmd(&cmd)?;

        // 2. 交易对校验
        let symbol = *cmd.symbol();
        if !self.is_symbol_supported(symbol) {
            return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                field: "symbol",
                reason: "trading pair not supported",
            }));
        }

        // 3. 生成订单 ID 和时间戳
        let order_id = order_next_id() as u64;
        let timestamp = Timestamp::now_as_nanos();

        // 4. 构造初始订单
        let mut order = SpotOrder::from(cmd.clone());
        order.state.status = base_types::exchange::spot::spot_types::OrderStatus::New;
        order.state.last_updated = timestamp;

        // 5. 生成变更日志
        let order_log = order.track_create().map_err(|e| {
            tracing::error!(
                order_id = %order_id,
                error = ?e,
                "Failed to track order creation"
            );
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to track order creation: {}", e),
            })
        })?;

        // 6. 同步写入 MySQL
        self.order_repo.replay_event(&order_log).map_err(|e| {
            tracing::error!(
                order_id = %order_id,
                error = ?e,
                "Failed to persist order to MySQL"
            );
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to persist order: {}", e),
            })
        })?;

        // 7. 发布 Kafka 事件
        if let Err(e) = self.event_publisher.publish_order_log(&order_log) {
            tracing::error!(
                order_id = %order_id,
                error = ?e,
                "Failed to publish order log to Kafka, attempting rollback"
            );

            // 尝试回滚：删除刚写入的订单
            if let Err(rollback_err) = self.order_repo.replay_event(&ChangeLog::new(
                order_id.to_string(),
                "SpotOrder".to_string(),
                ChangeType::Deleted,
                timestamp.0,
                1,
            )) {
                tracing::error!(
                    order_id = %order_id,
                    error = ?rollback_err,
                    "Failed to rollback order after Kafka publish failure"
                );
            }

            return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to publish order log: {}", e),
            }));
        }

        // 8. 返回 ACK
        Ok(NewOrderAck::new(
            symbol,
            order_id,
            -1,
            cmd.new_client_order_id().clone(),
            timestamp,
        ))
    }

    /// 验证命令并转换为内部订单
    ///
    /// # 参数
    /// - `cmd`: 新订单命令
    ///
    /// # 返回
    /// - `Ok(SpotOrder)`: 验证通过的内部订单
    /// - `Err(SpotCmdErrorAny)`: 验证失败错误
    fn validate_cmd_cpu(&self, cmd: NewOrderCmd) -> Result<SpotOrder, SpotCmdErrorAny> {
        // TODO: 实现验证逻辑
        // - 价格范围检查
        // - 数量范围检查
        // - 交易对有效性检查
        // - 订单类型组合检查

        // 转换为内部订单
        let internal_order = SpotOrder::from(cmd);

        Ok(internal_order)
    }

    fn persist_change_logs(&self, logs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        // TODO: 实现持久化逻辑
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};

    use base_types::exchange::spot::spot_types::TimeInForce;
    use base_types::{Decimal, Price, TradingPair};
    use lob_repo::core::repo_snapshot_support::RepoError;

    use super::*;
    use crate::proc::behavior::spot_trade_behavior::CMetadata;
    use crate::proc::behavior::v2::spot_trade_behavior_v2::SpotTradeCmdOrQuery;
    use crate::proc::v2::processor::kafka::event_publisher::PublishError;

    struct MockEventPublisher {
        published: AtomicBool,
        error_on_publish: AtomicBool,
    }

    impl MockEventPublisher {
        fn new() -> Self {
            Self { published: AtomicBool::new(false), error_on_publish: AtomicBool::new(false) }
        }
        fn with_error() -> Self {
            Self { published: AtomicBool::new(false), error_on_publish: AtomicBool::new(true) }
        }
        fn was_published(&self) -> bool {
            self.published.load(Ordering::SeqCst)
        }
    }

    impl EventPublisher for MockEventPublisher {
        fn publish_command(&self, _cmd: &SpotTradeCmdOrQuery) -> Result<(), PublishError> {
            self.published.store(true, Ordering::SeqCst);
            if self.error_on_publish.load(Ordering::SeqCst) {
                Err(PublishError::BackendUnavailable("mock error".into()))
            } else {
                Ok(())
            }
        }

        fn publish_order_log(&self, _log: &ChangeLog) -> Result<(), PublishError> {
            self.published.store(true, Ordering::SeqCst);
            if self.error_on_publish.load(Ordering::SeqCst) {
                Err(PublishError::BackendUnavailable("mock error".into()))
            } else {
                Ok(())
            }
        }

        fn publish_balance_log(&self, _log: &ChangeLog) -> Result<(), PublishError> {
            Ok(())
        }

        fn publish_trade_log(&self, _log: &ChangeLog) -> Result<(), PublishError> {
            Ok(())
        }

        fn publish_order_logs(&self, _logs: &[ChangeLog]) -> Result<(), PublishError> {
            Ok(())
        }

        fn publish_balance_logs(&self, _logs: &[ChangeLog]) -> Result<(), PublishError> {
            Ok(())
        }

        fn publish_trade_logs(&self, _logs: &[ChangeLog]) -> Result<(), PublishError> {
            Ok(())
        }
    }

    struct MockLobRepo {
        supported_symbols: Mutex<Vec<TradingPair>>,
    }

    impl MockLobRepo {
        fn new(supported: Vec<TradingPair>) -> Self {
            Self { supported_symbols: Mutex::new(supported) }
        }
    }

    impl MultiSymbolLobRepo for MockLobRepo {
        type Order = SpotOrder;

        fn match_orders(
            &self,
            _symbol: TradingPair,
            _side: OrderSide,
            _price: Price,
            _quantity: Quantity,
        ) -> (Option<Vec<&Self::Order>>, Quantity) {
            (None, Decimal::default())
        }

        fn best_bid(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn best_ask(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn contains_symbol(&self, symbol: &TradingPair) -> bool {
            self.supported_symbols.lock().unwrap().contains(symbol)
        }

        fn add_order(&self, _symbol: TradingPair, _order: Self::Order) -> Result<(), RepoError> {
            Ok(())
        }

        fn remove_order(&self, _symbol: TradingPair, _order_id: base_types::OrderId) -> bool {
            true
        }

        fn find_order(&self, _p0: TradingPair, _p1: base_types::OrderId) -> Option<&Self::Order> {
            None
        }

        fn find_order_mut(
            &self,
            _p0: TradingPair,
            _order_id: base_types::OrderId,
        ) -> Option<&mut Self::Order> {
            None
        }

        fn last_price(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn update_last_price(&self, _symbol: TradingPair, _price: Price) {}
    }

    fn create_test_order_cmd(symbol: TradingPair) -> NewOrderCmd {
        NewOrderCmd::new(
            CMetadata::new(
                "test_order_123".to_string(),
                Timestamp::now_as_nanos(),
                None,
                None,
                Some("test_user".to_string()),
                Vec::new(),
                None,
            ),
            symbol,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(0.1)),
            None,
            Some(Price::from_f64(45000.0)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    fn create_handler(
        event_publisher: Arc<dyn EventPublisher>,
        lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
    ) -> OrderHandler {
        OrderHandler::new(
            Arc::new(MySqlDbRepo::default()),
            Arc::new(MySqlDbRepo::default()),
            Arc::new(MySqlDbRepo::default()),
            lob_repo,
            event_publisher,
        )
    }

    #[test]
    fn test_handle_post_success() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::BtcUsdt]));
        let handler = create_handler(publisher.clone(), lob_repo);

        let cmd = create_test_order_cmd(TradingPair::BtcUsdt);
        let result = handler.handle_post(cmd);

        assert!(result.is_ok());
        assert!(publisher.was_published());
    }

    #[test]
    fn test_handle_post_unsupported_symbol() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::EthUsdt]));
        let handler = create_handler(publisher, lob_repo);

        let cmd = create_test_order_cmd(TradingPair::BtcUsdt);
        let result = handler.handle_post(cmd);

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            SpotCmdErrorAny::Common(CommonError::InvalidParameter { field, .. }) => {
                assert_eq!(field, "symbol");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_handle_post_zero_quantity() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::BtcUsdt]));
        let handler = create_handler(publisher, lob_repo);

        let cmd = NewOrderCmd::new(
            CMetadata::new(
                "test_order_123".to_string(),
                Timestamp::now_as_nanos(),
                None,
                None,
                Some("test_user".to_string()),
                Vec::new(),
                None,
            ),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::default()),
            None,
            Some(Price::from_f64(45000.0)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let result = handler.handle_post(cmd);

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            SpotCmdErrorAny::Common(CommonError::InvalidParameter { field, .. }) => {
                assert_eq!(field, "quantity");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_handle_post_limit_order_without_price() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::BtcUsdt]));
        let handler = create_handler(publisher, lob_repo);

        let cmd = NewOrderCmd::new(
            CMetadata::new(
                "test_order_456".to_string(),
                Timestamp::now_as_nanos(),
                None,
                None,
                Some("test_user".to_string()),
                Vec::new(),
                None,
            ),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(0.1)),
            None,
            None, // price 缺失，应该触发错误
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let result = handler.handle_post(cmd);

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            SpotCmdErrorAny::Common(CommonError::InvalidParameter { field, .. }) => {
                assert_eq!(field, "price");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_handle_post_publish_error() {
        let publisher = Arc::new(MockEventPublisher::with_error());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::BtcUsdt]));
        let handler = create_handler(publisher, lob_repo);

        let cmd = create_test_order_cmd(TradingPair::BtcUsdt);
        let result = handler.handle_post(cmd);

        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            SpotCmdErrorAny::Common(CommonError::Internal { .. }) => {}
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_accept_new_order_success() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::BtcUsdt]));
        let handler = create_handler(publisher.clone(), lob_repo);

        let cmd = create_test_order_cmd(TradingPair::BtcUsdt);
        let result = handler.accept_new_order(cmd);

        assert!(result.is_ok());
        let ack = result.unwrap();
        assert_eq!(*ack.symbol(), TradingPair::BtcUsdt);
        assert!(publisher.was_published());
    }

    #[test]
    fn test_accept_new_order_invalid_params() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::BtcUsdt]));
        let handler = create_handler(publisher, lob_repo);

        let cmd = NewOrderCmd::new(
            CMetadata::new(
                "test_order_123".to_string(),
                Timestamp::now_as_nanos(),
                None,
                None,
                Some("test_user".to_string()),
                Vec::new(),
                None,
            ),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::default()), // 零数量
            None,
            Some(Price::from_f64(45000.0)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let result = handler.accept_new_order(cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_accept_new_order_unsupported_symbol() {
        let publisher = Arc::new(MockEventPublisher::new());
        let lob_repo = Arc::new(MockLobRepo::new(vec![TradingPair::EthUsdt]));
        let handler = create_handler(publisher, lob_repo);

        let cmd = create_test_order_cmd(TradingPair::BtcUsdt);
        let result = handler.accept_new_order(cmd);

        assert!(result.is_err());
        match result.unwrap_err() {
            SpotCmdErrorAny::Common(CommonError::InvalidParameter { field, .. }) => {
                assert_eq!(field, "symbol");
            }
            _ => panic!("Expected InvalidParameter error for symbol"),
        }
    }
}
