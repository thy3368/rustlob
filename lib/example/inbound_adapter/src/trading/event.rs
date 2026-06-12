use cmd_handler::command_use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseExecutionError,
};
use example_core::{
    MatchSpotOrderCmd, MatchSpotOrderError, MatchSpotOrderOutput, MatchSpotOrderState,
    SettleSpotTradeCmd, SettleSpotTradeError, SettleSpotTradeOutput, SettleSpotTradeState,
};
use serde::{Deserialize, Serialize};

use crate::common::{execute_match_spot_order, execute_settle_spot_trade};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchSpotOrderEventRequest {
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
    pub party_id: String,
    pub taker_order_id: String,
    pub match_id: String,
}

impl MatchSpotOrderEventRequest {
    fn into_envelope(self) -> CommandEnvelope<MatchSpotOrderCmd> {
        CommandEnvelope {
            meta: CommandMeta { trace_id: self.trace_id, command_id: self.command_id },
            command: MatchSpotOrderCmd {
                party_id: self.party_id,
                taker_order_id: self.taker_order_id,
                match_id: self.match_id,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettleSpotTradeEventRequest {
    pub trace_id: Option<String>,
    pub command_id: Option<String>,
    pub party_id: String,
    pub settlement_batch_id: String,
    pub trade_ids: Vec<String>,
}

impl SettleSpotTradeEventRequest {
    fn into_envelope(self) -> CommandEnvelope<SettleSpotTradeCmd> {
        CommandEnvelope {
            meta: CommandMeta { trace_id: self.trace_id, command_id: self.command_id },
            command: SettleSpotTradeCmd {
                party_id: self.party_id,
                settlement_batch_id: self.settlement_batch_id,
                trade_ids: self.trade_ids,
            },
        }
    }
}

pub fn handle_spot_order_placed_event<OB>(
    request: MatchSpotOrderEventRequest,
    outbound: &OB,
) -> Result<
    cmd_handler::command_use_case_def2::UseCaseOutput<MatchSpotOrderOutput>,
    CommandUseCaseExecutionError<MatchSpotOrderError, OB::Error>,
>
where
    OB: ?Sized
        + Send
        + Sync
        + cmd_handler::command_use_case_def2::CommandUseCaseOutbound<
            Command = MatchSpotOrderCmd,
            State = MatchSpotOrderState,
        >,
    OB::Error: 'static,
{
    execute_match_spot_order(request.into_envelope(), outbound)
}

pub fn handle_spot_trade_matched_event<OB>(
    request: SettleSpotTradeEventRequest,
    outbound: &OB,
) -> Result<
    cmd_handler::command_use_case_def2::UseCaseOutput<SettleSpotTradeOutput>,
    CommandUseCaseExecutionError<SettleSpotTradeError, OB::Error>,
>
where
    OB: ?Sized
        + Send
        + Sync
        + cmd_handler::command_use_case_def2::CommandUseCaseOutbound<
            Command = SettleSpotTradeCmd,
            State = SettleSpotTradeState,
        >,
    OB::Error: 'static,
{
    execute_settle_spot_trade(request.into_envelope(), outbound)
}
