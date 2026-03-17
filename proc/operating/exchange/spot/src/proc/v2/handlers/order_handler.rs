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
use base_types::{AccountId, AssetId, Timestamp};
use db_repo::MySqlDbRepo;
use diff::{ChangeLogEntry, Entity};
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use super::CommandHandler;
use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{NewOrderCmd, SpotTradeResAny};

/// 订单处理器
pub struct OrderHandler {
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,

    lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
}

impl OrderHandler {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
        lob_repo: Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
    ) -> Self {
        Self { balance_repo, trade_repo, order_repo, lob_repo }
    }

    fn query_balance_id(&self, asset_id: AssetId) -> String {
        // BalanceId format: "account_id:asset_id"
        format!("{:?}:{}", AccountId(1), u32::from(asset_id))
    }

    // _op 只做计算 不做io
    pub(crate) fn frozen_balance(
        &self,
        internal_order: &SpotOrder,
    ) -> Result<ChangeLogEntry, SpotCmdErrorAny> {
        // 步骤1: 冻结资金（所有订单类型，包括条件单）
        let frozen_asset_id = internal_order.frozen_asset_id();
        let frozen_asset_balance_id = self.query_balance_id(frozen_asset_id);

        // 优化A: 消除 unwrap() - 使用安全的错误处理
        let mut frozen_asset_balance =
            match self.balance_repo.find_by_id_4_update(&frozen_asset_balance_id) {
                Ok(Some(balance)) => balance,
                Ok(None) => {
                    return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                        field: "balance",
                        reason: "balance not found",
                    }));
                }
                Err(e) => {
                    return Err(SpotCmdErrorAny::Common(CommonError::Internal {
                        message: format!("Failed to query balance: {}", e),
                    }));
                }
            };

        match self.handle_data_cpu(&mut frozen_asset_balance, &internal_order) {
            Ok(balance_change_log) => {
                return Ok(balance_change_log.clone());
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    /// 处理新订单
    //todo 分布式定序器在哪步做？
    pub fn handle_new_order(
        &self,
        cmd: NewOrderCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // 转换订单，失败不落单
        let mut internal_order = match self.validate_cmd(cmd) {
            Ok(order) => order,
            Err(err) => {
                return Err(err);
            }
        };

        // 1. 资金冻结
        match self.frozen_balance(&internal_order) {
            Ok(balance_logs) => {
                //todo 发送balance&order消息 balance_logs
                // 4. 构造响应

                //反回 NewOrderAck
                return Ok(CmdResp::new(
                    ResMetadata::new(0, false, Timestamp::default()),
                    SpotTradeResAny::TestNewOrderEmpty, // TODO: 返回正确的响应类型
                ));
            }
            Err(err) => {
                // 余额不足：设置订单状态为 Rejected
                internal_order.state.status =
                    base_types::exchange::spot::spot_types::OrderStatus::Rejected;
                internal_order.state.last_updated = Timestamp::now_as_nanos();

                //todo 发送订单消息
                //todo 将log发送到队列 kafka/disruptor

                return Err(err);
            }
        }
    }

    // ========== 私有辅助方法 ==========

    fn match_order(&self, order: &mut SpotOrder) -> Result<(), SpotCmdErrorAny> {
        // TODO: 实现匹配逻辑
        Ok(())
    }

    /// 执行资金冻结逻辑

    // ========== 私有辅助方法 ==========

    fn validate_cmd(&self, cmd: NewOrderCmd) -> Result<SpotOrder, SpotCmdErrorAny> {
        // TODO: 实现验证逻辑

        // 根据 NewOrderCmd 创建 SpotOrder
        let mut internal_order = SpotOrder::from(cmd);

        Ok((internal_order))
    }

    /// 处理数据操作：冻结资金并生成变更日志
    ///
    /// 此方法只做计算，不做 I/O 操作
    ///
    /// # 参数
    /// - `balance`: 可变的余额引用
    /// - `order`: 可变的订单引用
    ///
    /// # 返回
    /// - `Ok((order_change_log, balance_change_log))`: 成功时返回订单和余额的变更日志
    /// - `Err(SpotCmdErrorAny)`: 失败时返回错误（包括余额不足）
    fn handle_data_cpu(
        &self,
        balance: &mut Balance,
        order: &SpotOrder,
    ) -> Result<(ChangeLogEntry), SpotCmdErrorAny> {
        // 计算需要冻结的金额
        let frozen_amount = match order.side {
            OrderSide::Buy => order.total_quote_qty,
            OrderSide::Sell => order.total_base_qty,
        };

        // 保存旧状态用于生成变更日志
        let old_balance = balance.clone();

        // 尝试冻结资金
        let freeze_result = balance.frozen(frozen_amount, Timestamp::now_as_nanos());

        match freeze_result {
            Ok(_) => {
                // 冻结成功，生成变更日志
                let balance_change_log = balance.track_update_from(&old_balance).map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal {
                        message: format!("Failed to track balance update: {}", e),
                    })
                })?;

                Ok((balance_change_log))
            }
            Err(BalanceError::InsufficientAvailable { required, available }) => {
                // 返回余额不足错误
                Err(SpotCmdErrorAny::Common(CommonError::InsufficientBalance {
                    required: required as u64,
                    available: available as u64,
                }))
            }
            Err(e) => {
                // 其他余额错误
                Err(SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Balance error: {}", e),
                }))
            }
        }
    }

    fn persist_change_logs(&self, logs: &[ChangeLogEntry]) -> Result<(), SpotCmdErrorAny> {
        // TODO: 实现持久化逻辑
        Ok(())
    }
}

#[async_trait::async_trait]
impl CommandHandler<NewOrderCmd, SpotTradeResAny, SpotCmdErrorAny> for OrderHandler {
    async fn handle(&self, cmd: NewOrderCmd) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        self.handle_new_order(cmd)
    }
}
