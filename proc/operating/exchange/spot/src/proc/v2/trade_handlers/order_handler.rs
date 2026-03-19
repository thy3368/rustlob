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
use base_types::exchange::spot::spot_types::{OrderSide, SpotOrder, SpotTrade};
use base_types::{AccountId, AssetId, Quantity, Timestamp};
use db_repo::MySqlDbRepo;
use diff::{ChangeLogEntry, Entity};
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{NewOrderCmd, SpotTradeResAny};
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
    ) -> Result<ChangeLogEntry, SpotCmdErrorAny> {
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
    ) -> Result<ChangeLogEntry, SpotCmdErrorAny> {
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
    fn publish_logs(&self, order_log: &ChangeLogEntry, balance_log: Option<&ChangeLogEntry>) {
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
        balance_log: ChangeLogEntry,
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

    fn persist_change_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), SpotCmdErrorAny> {
        // TODO: 实现持久化逻辑
        Ok(())
    }
}
