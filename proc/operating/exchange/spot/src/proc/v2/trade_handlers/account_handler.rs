//! 账户查询处理器
//!
//! 负责处理账户相关查询命令：
//! - Account: 查询账户信息
//! - MyTrades: 查询成交历史
//! - QueryUnfilledOrderCount: 查询未成交订单数

use std::sync::Arc;

use base_types::account::balance::Balance;
use base_types::cqrs::cqrs_types::{CmdResp, ResMetadata};
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::Timestamp;
use db_repo::MySqlDbRepo;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    AccountCmd, MyTradesCmd, QueryUnfilledOrderCountCmd, SpotTradeResAny,
};

use super::CommandHandler;

/// 账户查询处理器
pub struct AccountHandler {
    balance_repo: Arc<MySqlDbRepo<Balance>>,
    trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
    order_repo: Arc<MySqlDbRepo<SpotOrder>>,
}

impl AccountHandler {
    pub fn new(
        balance_repo: Arc<MySqlDbRepo<Balance>>,
        trade_repo: Arc<MySqlDbRepo<SpotTrade>>,
        order_repo: Arc<MySqlDbRepo<SpotOrder>>,
    ) -> Self {
        Self {
            balance_repo,
            trade_repo,
            order_repo,
        }
    }

    /// 处理账户信息查询
    pub fn handle_account(
        &self,
        cmd: AccountCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // TODO: 实现账户查询逻辑
        todo!("Implement account query")
    }

    /// 处理成交历史查询
    pub fn handle_my_trades(
        &self,
        cmd: MyTradesCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // TODO: 实现成交历史查询逻辑
        todo!("Implement my trades query")
    }

    /// 处理未成交订单数查询
    pub fn handle_unfilled_order_count(
        &self,
        cmd: QueryUnfilledOrderCountCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        // TODO: 实现未成交订单数查询逻辑
        todo!("Implement unfilled order count query")
    }
}

#[async_trait::async_trait]
impl CommandHandler<AccountCmd, SpotTradeResAny, SpotCmdErrorAny> for AccountHandler {
    async fn handle(
        &self,
        cmd: AccountCmd,
    ) -> Result<CmdResp<SpotTradeResAny>, SpotCmdErrorAny> {
        self.handle_account(cmd)
    }
}
