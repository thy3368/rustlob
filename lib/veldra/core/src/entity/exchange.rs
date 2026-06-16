use cmd_handler::EntityReplayableEvent;
use example_core::{
    CancelSpotOrderCmd, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd,
    PlaceOrderTimeInForce, WithdrawQuoteCmd,
};

use super::{PerpState, SpotState, TreasuryState, stable_hash_hex};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandEnvelope<T> {
    pub command_id: String,
    pub account_id: String,
    pub nonce: u64,
    pub timestamp_ns: u64,
    pub command: T,
}

impl<T> CommandEnvelope<T> {
    pub fn map<U>(self, command: U) -> CommandEnvelope<U> {
        CommandEnvelope {
            command_id: self.command_id,
            account_id: self.account_id,
            nonce: self.nonce,
            timestamp_ns: self.timestamp_ns,
            command,
        }
    }
}

impl CommandEnvelope<ProductCommand> {
    pub fn commitment(&self) -> String {
        stable_hash_hex(&[
            self.command_id.as_str(),
            self.account_id.as_str(),
            self.nonce.to_string().as_str(),
            self.timestamp_ns.to_string().as_str(),
            self.command.commitment().as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductCommand {
    Spot(SpotCommand),
    Perp(PerpCommand),
    Treasury(TreasuryCommand),
}

impl ProductCommand {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Spot(_) => "spot",
            Self::Perp(_) => "perp",
            Self::Treasury(_) => "treasury",
        }
    }

    pub fn commitment(&self) -> String {
        match self {
            Self::Spot(command) => command.commitment(),
            Self::Perp(command) => command.commitment(),
            Self::Treasury(command) => command.commitment(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotCommand {
    ExecuteImmediateOrderPipeline(ExecuteImmediateSpotOrderPipelineCmd),
    CancelOrder(CancelSpotOrderCmd),
}

impl SpotCommand {
    pub fn commitment(&self) -> String {
        match self {
            Self::ExecuteImmediateOrderPipeline(command) => {
                let execution = match command.place.execution {
                    example_core::PlaceImmediateOrderExecution::Limit { price, time_in_force } => {
                        format!("limit:{price}:{}", spot_tif(time_in_force))
                    }
                    example_core::PlaceImmediateOrderExecution::Market { aggressive_price } => {
                        format!("market:{aggressive_price}")
                    }
                };
                stable_hash_hex(&[
                    "spot.execute_immediate_pipeline",
                    command.place.party_id.as_str(),
                    command.place.symbol.as_str(),
                    command.place.asset.to_string().as_str(),
                    if command.place.is_buy { "buy" } else { "sell" },
                    command.place.size.to_string().as_str(),
                    if command.place.reduce_only { "reduce_only" } else { "open" },
                    execution.as_str(),
                    command.place.cloid.as_deref().unwrap_or_default(),
                    command.match_id.as_str(),
                    command.settlement_batch_id.as_str(),
                ])
            }
            Self::CancelOrder(command) => stable_hash_hex(&[
                "spot.cancel_order",
                command.party_id.as_str(),
                command.asset.to_string().as_str(),
                command.order_id.to_string().as_str(),
            ]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpCommand {
    Unsupported,
}

impl PerpCommand {
    pub fn commitment(&self) -> String {
        stable_hash_hex(&["perp.unsupported"])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreasuryCommand {
    DepositQuote(DepositQuoteCmd),
    WithdrawQuote(WithdrawQuoteCmd),
}

impl TreasuryCommand {
    pub fn commitment(&self) -> String {
        match self {
            Self::DepositQuote(command) => stable_hash_hex(&[
                "treasury.deposit_quote",
                command.party_id.as_str(),
                command.amount.to_string().as_str(),
            ]),
            Self::WithdrawQuote(command) => stable_hash_hex(&[
                "treasury.withdraw_quote",
                command.party_id.as_str(),
                command.amount.to_string().as_str(),
            ]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExchangeState {
    pub spot: SpotState,
    pub perp: PerpState,
    pub treasury: TreasuryState,
}

impl ExchangeState {
    pub fn commitment(&self) -> String {
        stable_hash_hex(&[
            self.spot.commitment().as_str(),
            self.perp.commitment().as_str(),
            self.treasury.commitment().as_str(),
        ])
    }
}

pub fn build_new_block(
    block_height: u64,
    parent_block_hash: String,
    commands: &[CommandEnvelope<ProductCommand>],
    events: &[EntityReplayableEvent],
    exchange_state: &ExchangeState,
) -> super::NewBlock {
    let commands_root =
        stable_hash_hex(&commands.iter().map(CommandEnvelope::commitment).collect::<Vec<_>>());
    let events_root =
        stable_hash_hex(&events.iter().map(super::event_commitment).collect::<Vec<_>>());
    let post_state_root = exchange_state.commitment();
    super::NewBlock::new(
        block_height,
        parent_block_hash,
        commands_root,
        events_root,
        post_state_root,
    )
}

fn spot_tif(value: PlaceOrderTimeInForce) -> &'static str {
    match value {
        PlaceOrderTimeInForce::Gtc => "gtc",
        PlaceOrderTimeInForce::Ioc => "ioc",
        PlaceOrderTimeInForce::Alo => "alo",
    }
}
