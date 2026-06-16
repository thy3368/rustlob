use example_core::{
    Balance, CancelSpotOrderCmd, DepositQuoteChanges, DepositQuoteCmd,
    ExecuteImmediateSpotOrderPipelineCmd, WithdrawQuoteChanges, WithdrawQuoteCmd,
};

use crate::use_case::BuildBlockError;
use crate::use_case::block_execution::handler::cancel_order_block_command_handler::{CancelOrderBlockCommandHandler, CANCEL_ORDER_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::deposit_quote_block_command_handler::{DepositQuoteBlockCommandHandler, DEPOSIT_QUOTE_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::execute_immediate_order_pipeline_block_command_handler::{ExecuteImmediateOrderPipelineBlockCommandHandler, EXECUTE_IMMEDIATE_ORDER_PIPELINE_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::perp_unsupported_block_command_handler::{PerpUnsupportedBlockCommandHandler, PERP_UNSUPPORTED_BLOCK_COMMAND_HANDLER};
use crate::use_case::block_execution::handler::withdraw_quote_block_command_handler::{WithdrawQuoteBlockCommandHandler, WITHDRAW_QUOTE_BLOCK_COMMAND_HANDLER};
use crate::entity::{
    AccountAssetKey, CommandEnvelope, ExchangeState, ProductCommand, SpotCommand,
    TreasuryCommand, TreasuryState,
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
    ) -> Result<Self::Execution, BuildBlockError>;

    fn apply(&self, exchange_state: &mut ExchangeState, execution: &Self::Execution);
}

pub(in crate::use_case::block_execution) enum ResolvedBlockCommandHandler<'a> {
    ExecuteImmediateOrderPipeline(
        &'static ExecuteImmediateOrderPipelineBlockCommandHandler,
        &'a ExecuteImmediateSpotOrderPipelineCmd,
    ),
    CancelOrder(&'static CancelOrderBlockCommandHandler, &'a CancelSpotOrderCmd),
    DepositQuote(&'static DepositQuoteBlockCommandHandler, &'a DepositQuoteCmd),
    WithdrawQuote(&'static WithdrawQuoteBlockCommandHandler, &'a WithdrawQuoteCmd),
    PerpUnsupported(&'static PerpUnsupportedBlockCommandHandler),
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

pub(in crate::use_case::block_execution) fn apply_deposit_quote_changes(
    treasury_state: &mut TreasuryState,
    result: &DepositQuoteChanges,
) {
    treasury_state.balances.insert(
        AccountAssetKey::new(
            result.updated_quote_balance.after.account_id.as_str(),
            result.updated_quote_balance.after.asset_id.as_str(),
        ),
        result.updated_quote_balance.after.clone(),
    );
}

pub(in crate::use_case::block_execution) fn apply_withdraw_quote_changes(
    treasury_state: &mut TreasuryState,
    result: &WithdrawQuoteChanges,
) {
    treasury_state.balances.insert(
        AccountAssetKey::new(
            result.updated_quote_balance.after.account_id.as_str(),
            result.updated_quote_balance.after.asset_id.as_str(),
        ),
        result.updated_quote_balance.after.clone(),
    );
}
