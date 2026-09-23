use cmd_handler::EntityReplayableEvent;
use example_core_use_case::{
    DepositQuoteCmd, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd,
    PlaceOnlySpotOrderV2OrderType, WithdrawQuoteCmd,
};
use serde::{Deserialize, Serialize};

use super::{PerpState, SpotState, TreasuryState, stable_hash_hex};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotCommand {
    PlaceSpotOrderV2(PlaceOnlySpotOrderV2Cmd),
}

impl SpotCommand {
    pub fn commitment(&self) -> String {
        match self {
            Self::PlaceSpotOrderV2(command) => spot_order_command_commitment(command),
        }
    }
}

fn spot_order_command_commitment(command: &PlaceOnlySpotOrderV2Cmd) -> String {
    match command {
        PlaceOnlySpotOrderV2Cmd::Single(order) => {
            let fields = order_commitment_fields(order);
            stable_hash_hex(&["spot.place_spot_order_v2", fields.as_str()])
        }
        PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent, children } => {
            let mut fields = vec!["spot.place_spot_order_v2.normal_tpsl".to_string()];
            fields.push(order_commitment_fields(parent).to_string());
            for child in children {
                fields.push(order_commitment_fields(child).to_string());
            }
            let refs = fields.iter().map(String::as_str).collect::<Vec<_>>();
            stable_hash_hex(&refs)
        }
    }
}

fn order_commitment_fields(order: &PlaceOnlySpotOrderV2OrderCmd) -> String {
    let order_type = match &order.order_type {
        PlaceOnlySpotOrderV2OrderType::Limit { tif } => format!("limit:{tif}"),
        PlaceOnlySpotOrderV2OrderType::Trigger { is_market, trigger_price, trigger_role } => {
            format!("trigger:{is_market}:{trigger_price}:{trigger_role}")
        }
    };
    format!(
        "{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        order.party_id,
        order.asset,
        order.order_id,
        order.symbol,
        order.is_buy,
        order.price,
        order.size,
        order_type,
        order.reduce_only,
        order.cloid.as_deref().unwrap_or_default(),
        order.base_asset_id,
        order.quote_asset_id,
        order.maker_fee_bps,
        order.taker_fee_bps,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerpCommand {
    Unsupported,
}

impl PerpCommand {
    pub fn commitment(&self) -> String {
        stable_hash_hex(&["perp.unsupported"])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
