use cmd_handler::EntityReplayableEvent;
use example_core::{
    Balance, CancelSpotOrderCmd, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd,
    WithdrawQuoteCmd,
};

use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::cancel_order_block_command_handler::{CancelOrderBlockCommandHandler, CANCEL_ORDER_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::deposit_quote_block_command_handler::{DepositQuoteBlockCommandHandler, DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::execute_immediate_order_pipeline_block_command_handler::{ExecuteImmediateOrderPipelineBlockCommandHandler, EXECUTE_IMMEDIATE_ORDER_PIPELINE_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::perp_unsupported_block_command_handler::{PerpUnsupportedBlockCommandHandler, PERP_UNSUPPORTED_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::withdraw_quote_block_command_handler::{WithdrawQuoteBlockCommandHandler, WITHDRAW_QUOTE_BLOCK_COMMAND_HANDLER};
use crate::entity::{
    AccountAssetKey, CommandEnvelope, CommandExecutionResult, ExchangeState, ProductCommand,
    SpotCommand, TreasuryBalanceUpdate, TreasuryCommand, TreasuryState,
};

pub(in crate::use_case::block_execution) trait BlockCommandHandler {
    type Command;
    type Execution;

    fn validate(
        &self,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<(), BuildBlockError>;

    fn execute(
        &self,
        envelope: &CommandEnvelope<ProductCommand>,
        command: &Self::Command,
        exchange_state: &ExchangeState,
    ) -> Result<CommandExecutionResult, BuildBlockError>;

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution);

    fn events<'a>(&self, execution: &'a Self::Execution) -> &'a [EntityReplayableEvent];

    fn rebase_events(&self, execution: &mut Self::Execution, base_sequence: u64);
}

pub(in crate::use_case::block_execution) enum ResolvedBlockCommandHandler<'a> {
    ExecuteImmediateOrderPipeline(
        &'static ExecuteImmediateOrderPipelineBlockCommandHandler,
        &'a ExecuteImmediateSpotOrderPipelineCmd,
    ),
    CancelOrder(
        &'static CancelOrderBlockCommandHandler,
        &'a CancelSpotOrderCmd,
    ),
    DepositQuote(
        &'static DepositQuoteBlockCommandHandler,
        &'a DepositQuoteCmd,
    ),
    WithdrawQuote(
        &'static WithdrawQuoteBlockCommandHandler,
        &'a WithdrawQuoteCmd,
    ),
    PerpUnsupported(
        &'static PerpUnsupportedBlockCommandHandler,
    ),
}

pub(in crate::use_case::block_execution) fn resolve_block_command_handler(
    command: &ProductCommand,
) -> ResolvedBlockCommandHandler<'_> {
    match command {
        ProductCommand::Spot(SpotCommand::ExecuteImmediateOrderPipeline(command)) => {
            ResolvedBlockCommandHandler::ExecuteImmediateOrderPipeline(
                &EXECUTE_IMMEDIATE_ORDER_PIPELINE_BLOCK_COMMAND_HANDLER,
                command,
            )
        }
        ProductCommand::Spot(SpotCommand::CancelOrder(command)) => {
            ResolvedBlockCommandHandler::CancelOrder(&CANCEL_ORDER_BLOCK_COMMAND_HANDLER, command)
        }
        ProductCommand::Treasury(TreasuryCommand::DepositQuote(command)) => {
            ResolvedBlockCommandHandler::DepositQuote(&DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER, command)
        }
        ProductCommand::Treasury(TreasuryCommand::WithdrawQuote(command)) => {
            ResolvedBlockCommandHandler::WithdrawQuote(
                &WITHDRAW_QUOTE_BLOCK_COMMAND_HANDLER,
                command,
            )
        }
        ProductCommand::Perp(_) => {
            ResolvedBlockCommandHandler::PerpUnsupported(&PERP_UNSUPPORTED_BLOCK_COMMAND_HANDLER)
        }
    }
}

pub(in crate::use_case::block_execution) fn normalize_local_events(
    events: Vec<EntityReplayableEvent>,
) -> Vec<EntityReplayableEvent> {
    events
        .into_iter()
        .enumerate()
        .map(|(index, mut event)| {
            event.timestamp = 0;
            event.sequence = index as u64;
            event
        })
        .collect()
}

pub(in crate::use_case::block_execution) fn rebase_events(
    events: &[EntityReplayableEvent],
    base_sequence: u64,
) -> Vec<EntityReplayableEvent> {
    events
        .iter()
        .enumerate()
        .map(|(index, event)| {
            let mut cloned = event.clone();
            cloned.timestamp = 0;
            cloned.sequence = base_sequence + index as u64;
            cloned
        })
        .collect()
}

pub(in crate::use_case::block_execution) fn treasury_quote_balance(
    treasury_state: &TreasuryState,
    account_id: &str,
) -> Result<Balance, BuildBlockError> {
    treasury_state.balances.get(&AccountAssetKey::new(account_id, "USDT")).cloned().ok_or_else(
        || BuildBlockError::MissingTreasuryBalance {
            account_id: account_id.to_string(),
            asset_id: "USDT".to_string(),
        },
    )
}

pub(in crate::use_case::block_execution) fn apply_treasury_execution(
    treasury_state: &mut TreasuryState,
    result: &TreasuryBalanceUpdate,
) {
    treasury_state.balances.insert(
        AccountAssetKey::new(
            result.balance_after.account_id.as_str(),
            result.balance_after.asset_id.as_str(),
        ),
        result.balance_after.clone(),
    );
}
