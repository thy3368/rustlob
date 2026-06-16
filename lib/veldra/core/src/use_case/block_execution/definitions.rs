use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{EventProjectError, IssuedByParty, ReplayableChanges};
use thiserror::Error;

use crate::entity::{
    CommandEnvelope, CommandExecutionResult, ExchangeState, NewBlock, ProductCommand,
};

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
pub struct BuildBlockFromCommandsChanges {
    pub new_block: NewBlock,
    pub command_results: Vec<CommandExecutionResult>,
    pub exchange_state: ExchangeState,
    pub replayable_events: Vec<EntityReplayableEvent>,
}

impl ReplayableChanges for BuildBlockFromCommandsChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        Ok(self.replayable_events.clone())
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
