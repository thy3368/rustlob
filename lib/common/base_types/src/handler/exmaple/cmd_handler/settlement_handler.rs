//! Settlement CommandHandler 示例实现。

use crate::handler::exmaple::cmd_handler::example_types::{
    AccountBalance, BalanceChange, HandlerError, Trade,
};
use crate::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};

pub struct SettlementCmd {
    pub settlement_id: String,
    pub trade_ids: Vec<String>,
}

pub struct SettlementState {
    pub trades: Vec<Trade>,
    pub accounts: Vec<AccountBalance>,
}

pub struct SettlementResult {
    pub balance_changes: Vec<BalanceChange>,
}

pub enum SettlementLog {
    BalanceSettled(BalanceSettled),
}

pub struct BalanceSettled {
    pub trade_id: String,
}

pub struct SettlementHandler;

impl SettlementHandler {
    pub fn new() -> Self {
        Self
    }
}

impl
    CmdHandlerForUpdate<
        SettlementCmd,
        SettlementState,
        SettlementResult,
        SettlementLog,
        SettlementResult,
        HandlerError,
    > for SettlementHandler
{
    fn pre_check_command(&self, _cmd: &SettlementCmd) -> Result<(), HandlerError> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &SettlementCmd,
    ) -> Result<SettlementState, HandlerError> {
        Ok(SettlementState { trades: vec![], accounts: vec![] })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &SettlementCmd,
        _state_set: &SettlementState,
    ) -> Result<(), HandlerError> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        _cmd: &SettlementCmd,
        _state_set: SettlementState,
    ) -> Result<ChangeSet<SettlementResult, SettlementLog>, HandlerError> {
        let result = SettlementResult { balance_changes: vec![] };
        Ok(ChangeSet { writes: result, changelogs: vec![] })
    }

    fn persist_changelogs(&self, _changelogs: &[SettlementLog]) -> Result<(), HandlerError> {
        Ok(())
    }

    fn replay_changelogs_to_state(
        &self,
        _changelogs: &[SettlementLog],
    ) -> Result<(), HandlerError> {
        Ok(())
    }

    fn publish_changelog(&self, _changelogs: &[SettlementLog]) -> Result<(), HandlerError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_handler() {
        let handler = SettlementHandler::new();
        let cmd = SettlementCmd { settlement_id: "s1".into(), trade_ids: vec!["trade_1".into()] };

        let result = handler.cmd_handle(cmd, |_, _| SettlementResult { balance_changes: vec![] });
        assert!(result.is_ok());
    }
}
