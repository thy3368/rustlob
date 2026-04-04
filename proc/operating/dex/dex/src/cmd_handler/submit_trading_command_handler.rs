use std::{collections::VecDeque, sync::Mutex};

use base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};

use super::trading_command::TradingCommandEnvelope;

type SubmitTradingCommandError = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitCommandResult {
    pub accepted: bool,
    pub queue_len: usize,
}

impl SubmitCommandResult {
    #[inline]
    fn accepted(queue_len: usize) -> Self {
        Self {
            accepted: true,
            queue_len,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SubmitTradingCommandState {
    pub pending_len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmitTradingCommandLog {
    CommandQueued {
        command_id: u64,
        queue_len: usize,
    },
}

#[derive(Debug, Default)]
pub struct SubmitTradingCommandHandler {
    pending_commands: Mutex<VecDeque<TradingCommandEnvelope>>,
}

impl SubmitTradingCommandHandler {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn next_queue_len(pending_len: usize) -> usize {
        pending_len + 1
    }

    pub fn drain_pending_commands(&self, max_batch_size: usize) -> Vec<TradingCommandEnvelope> {
        let mut pending_commands = self.pending_commands.lock().unwrap();
        let batch_size = max_batch_size.min(pending_commands.len());
        pending_commands.drain(..batch_size).collect()
    }

    pub fn pending_len(&self) -> usize {
        self.pending_commands.lock().unwrap().len()
    }
}

impl CmdHandlerForUpdate<
    TradingCommandEnvelope,
    SubmitTradingCommandState,
    SubmitCommandResult,
    SubmitTradingCommandLog,
    SubmitTradingCommandError,
> for SubmitTradingCommandHandler
{
    fn pre_check_command(
        &self,
        _cmd: &TradingCommandEnvelope,
    ) -> Result<(), SubmitTradingCommandError> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &TradingCommandEnvelope,
    ) -> Result<SubmitTradingCommandState, SubmitTradingCommandError> {
        Ok(SubmitTradingCommandState {
            pending_len: self.pending_commands.lock().unwrap().len(),
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &TradingCommandEnvelope,
        _state_set: &SubmitTradingCommandState,
    ) -> Result<(), SubmitTradingCommandError> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &TradingCommandEnvelope,
        state_set: SubmitTradingCommandState,
    ) -> Result<ChangeSet<SubmitCommandResult, SubmitTradingCommandLog>, SubmitTradingCommandError>
    {
        let queue_len = Self::next_queue_len(state_set.pending_len);

        Ok(ChangeSet {
            writes: SubmitCommandResult::accepted(queue_len),
            changelogs: vec![SubmitTradingCommandLog::CommandQueued {
                command_id: cmd.command_id,
                queue_len,
            }],
        })
    }

    fn persist_changelogs(
        &self,
        _changelogs: &[SubmitTradingCommandLog],
    ) -> Result<(), SubmitTradingCommandError> {
        Ok(())
    }

    fn replay_changelogs_to_state(
        &self,
        _changelogs: &[SubmitTradingCommandLog],
    ) -> Result<(), SubmitTradingCommandError> {
        Ok(())
    }

    fn publish_changelog(
        &self,
        _changelogs: &[SubmitTradingCommandLog],
    ) -> Result<(), SubmitTradingCommandError> {
        Ok(())
    }

    fn cmd_handle<R, F>(
        &self,
        cmd: TradingCommandEnvelope,
        result_mapper: F,
    ) -> Result<R, SubmitTradingCommandError>
    where
        F: FnOnce(&SubmitCommandResult, &[SubmitTradingCommandLog]) -> R,
    {
        let result = self.pre_check_command(&cmd).and_then(|_| {
            let state_set = self.load_state_set_for_update(&cmd)?;
            self.validate_command_in_lock(&cmd, &state_set)?;
            let changes = self.apply_command_and_collect_changes(&cmd, state_set)?;
            self.persist_changelogs(&changes.changelogs)?;
            self.replay_changelogs_to_state(&changes.changelogs)?;
            self.publish_changelog(&changes.changelogs)?;
            Ok(result_mapper(&changes.writes, &changes.changelogs))
        })?;

        self.pending_commands.lock().unwrap().push_back(cmd);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd_handler::{
        AmendOrderCmd, CancelOrderCmd, OrderSide, PlaceOrderCmd, TradingCommand,
    };

    fn place_order_envelope(command_id: u64) -> TradingCommandEnvelope {
        TradingCommandEnvelope {
            command_id,
            trader_id: 1,
            nonce: command_id,
            timestamp_ns: 1_000 + command_id,
            command: TradingCommand::PlaceOrder(PlaceOrderCmd {
                trader_id: 1,
                market: "BTC-PERP".into(),
                side: OrderSide::Buy,
                price: 100_000,
                quantity: 2,
            }),
        }
    }

    #[test]
    fn cmd_handle_enqueues_command() {
        let handler = SubmitTradingCommandHandler::new();

        let result = handler.cmd_handle(place_order_envelope(1), |writes, _| writes.clone());

        assert!(result.is_ok());
        let writes = result.unwrap();
        assert!(writes.accepted);
        assert_eq!(writes.queue_len, 1);
        assert_eq!(handler.pending_len(), 1);
    }

    #[test]
    fn drain_pending_commands_limits_batch_size() {
        let handler = SubmitTradingCommandHandler::new();
        handler
            .cmd_handle(place_order_envelope(1), |writes, _| writes.clone())
            .unwrap();
        handler
            .cmd_handle(
                TradingCommandEnvelope {
                    command_id: 2,
                    trader_id: 1,
                    nonce: 2,
                    timestamp_ns: 1_002,
                    command: TradingCommand::CancelOrder(CancelOrderCmd {
                        trader_id: 1,
                        order_id: 42,
                    }),
                },
                |writes, _| writes.clone(),
            )
            .unwrap();
        handler
            .cmd_handle(
                TradingCommandEnvelope {
                    command_id: 3,
                    trader_id: 1,
                    nonce: 3,
                    timestamp_ns: 1_003,
                    command: TradingCommand::AmendOrder(AmendOrderCmd {
                        trader_id: 1,
                        order_id: 42,
                        new_price: Some(101_000),
                        new_quantity: Some(3),
                    }),
                },
                |writes, _| writes.clone(),
            )
            .unwrap();

        let batch = handler.drain_pending_commands(2);

        assert_eq!(batch.len(), 2);
        assert_eq!(handler.pending_len(), 1);
        assert!(matches!(batch[0].command, TradingCommand::PlaceOrder(_)));
        assert!(matches!(batch[1].command, TradingCommand::CancelOrder(_)));
    }

    #[test]
    fn cmd_handler_for_update_returns_accepted_result() {
        let handler = SubmitTradingCommandHandler::new();

        let result = handler.cmd_handle(place_order_envelope(1), |writes, changelogs| {
            (writes.clone(), changelogs.len())
        });

        assert!(result.is_ok());
        let (writes, changelog_count) = result.unwrap();
        assert!(writes.accepted);
        assert_eq!(writes.queue_len, 1);
        assert_eq!(changelog_count, 1);
        assert_eq!(handler.pending_len(), 1);
    }
}
