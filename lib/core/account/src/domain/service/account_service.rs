//! 账户服务接口定义

use crate::domain::entity::{AccountCommand, AccountCommandResult, BalanceError};

/// 账户服务接口
///
/// 统一处理来自 LOB 和 Settlement 的所有账户操作：
/// - LOB: CheckAndFreeze, Unfreeze（下单/撤单）
/// - Settlement: Credit, Debit, DebitFrozen, Transfer, SettlePnl（结算）
pub trait AccountService: Send + Sync {
    /// 执行账户命令
    fn execute(&mut self, cmd: AccountCommand) -> AccountCommandResult;

    /// 批量执行（原子操作，全部成功或全部回滚）
    fn execute_batch(
        &mut self,
        cmds: Vec<AccountCommand>,
    ) -> Result<Vec<AccountCommandResult>, BalanceError>;
}
