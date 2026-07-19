use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use example_core::{Balance, BalanceLedgerEntryV2, SpotOrderV2, SpotTrade};
use example_core::entity::SettlementTransferVoucher;
use thiserror::Error;

use crate::entity::{BlockExecutionBody, CommandEnvelope, ExchangeState, NewBlock, ProductCommand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockFromCommandsCommand {
    pub block_height: u64,
}

impl IssuedByParty for BuildBlockFromCommandsCommand {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildBlockFromCommandsState {
    pub parent_height: u64,
    pub parent_block_hash: String,
    pub exchange_state: ExchangeState,
    pub commands: Vec<CommandEnvelope<ProductCommand>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockEntityChange {
    SpotOrderCreated(SpotOrderV2),
    SpotOrderUpdated(UpdatedEntityPair<SpotOrderV2>),
    BalanceUpdated(UpdatedEntityPair<Balance>),
    SpotTradeCreated(SpotTrade),
    SettlementTransferVoucherCreated(SettlementTransferVoucher),
    BalanceLedgerEntryCreated(BalanceLedgerEntryV2),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BuildBlockFromCommandsChanges {
    pub new_block: Option<NewBlock>,
    pub execution_body: Option<BlockExecutionBody>,
    pub ordered_changes: Vec<BlockEntityChange>,
}

impl ReplayableChanges for BuildBlockFromCommandsChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(self.ordered_changes.len());

        for change in &self.ordered_changes {
            match change {
                BlockEntityChange::SpotOrderCreated(order) => {
                    events.push(order.track_create_event()?);
                }
                BlockEntityChange::SpotOrderUpdated(order) => {
                    events.push(order.after.track_update_event_from(&order.before)?);
                }
                BlockEntityChange::BalanceUpdated(balance) => {
                    events.push(balance.after.track_update_event_from(&balance.before)?);
                }
                BlockEntityChange::SpotTradeCreated(trade) => {
                    events.push(trade.track_create_event()?);
                }
                BlockEntityChange::SettlementTransferVoucherCreated(voucher) => {
                    events.push(voucher.track_create_event()?);
                }
                BlockEntityChange::BalanceLedgerEntryCreated(entry) => {
                    events.push(entry.track_create_event()?);
                }
            }
        }

        Ok(events
            .into_iter()
            .enumerate()
            .map(|(sequence, mut event)| {
                event.timestamp = 0;
                event.sequence = sequence as u64;
                event
            })
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum BuildBlockError {
    #[error("block height must be greater than zero")]
    BlockHeightMustBePositive,
    #[error("command batch is empty")]
    EmptyCommands,
    #[error("duplicate command id '{command_id}' in batch")]
    DuplicateCommandId { command_id: String },
    #[error("duplicate nonce {nonce} for account '{account_id}' in batch")]
    DuplicateAccountNonce { account_id: String, nonce: u64 },
    #[error("command '{command_id}' has zero timestamp")]
    ZeroCommandTimestamp { command_id: String },
    #[error(
        "command '{command_id}' envelope account '{envelope_account_id}' does not match command party '{command_party_id}'"
    )]
    EnvelopeAccountMismatch {
        command_id: String,
        envelope_account_id: String,
        command_party_id: String,
    },
    #[error("command batch is not in canonical order")]
    NonCanonicalCommandOrder,
    #[error("block height {actual} is not continuous after parent height {parent_height}")]
    NonContinuousBlockHeight { parent_height: u64, actual: u64 },
    #[error("missing spot market rules for '{symbol}'")]
    MissingSpotMarketRules { symbol: String },
    #[error("missing spot asset pair for '{symbol}'")]
    MissingSpotAssetPair { symbol: String },
    #[error("missing spot symbol for asset '{asset}'")]
    MissingSpotAssetSymbol { asset: u32 },
    #[error("missing spot trading runtime for '{symbol}'")]
    MissingSpotTradingRuntime { symbol: String },
    #[error("missing spot balance for account '{account_id}' asset '{asset_id}'")]
    MissingSpotBalance { account_id: String, asset_id: String },
    #[error("missing spot next order sequence for account '{account_id}'")]
    MissingSpotOrderSequence { account_id: String },
    #[error("spot command execution failed: {0}")]
    SpotExecution(String),
    #[error("missing treasury quote balance for account '{account_id}' asset '{asset_id}'")]
    MissingTreasuryBalance { account_id: String, asset_id: String },
    #[error("treasury command execution failed: {0}")]
    TreasuryExecution(String),
    #[error("perp commands are not implemented yet")]
    UnsupportedPerpCommand,
}
