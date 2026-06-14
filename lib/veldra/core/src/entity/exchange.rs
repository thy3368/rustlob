use std::collections::{BTreeMap, BTreeSet};

use cmd_handler::EntityReplayableEvent;
use example_core::{
    Balance, DepositQuoteCmd, ExecuteImmediateSpotOrderPipelineCmd,
    ExecuteImmediateSpotOrderPipelineOutput, HyperliquidPerpFundingSettlement,
    HyperliquidPerpOrder, HyperliquidPerpPosition, MarketRules, SpotOrder, SpotSettlement,
    SpotTrade, WithdrawQuoteCmd,
};

use super::stable_hash_hex;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandExecutionResult {
    pub command_id: String,
    pub command_kind: String,
    pub command_commitment: String,
    pub result: ProductCommandResult,
}

impl CommandExecutionResult {
    pub fn commitment(&self) -> String {
        stable_hash_hex(&[
            self.command_id.as_str(),
            self.command_kind.as_str(),
            self.command_commitment.as_str(),
            self.result.commitment().as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductCommandResult {
    Spot(SpotCommandResult),
    Perp(PerpCommandResult),
    Treasury(TreasuryCommandResult),
}

impl ProductCommandResult {
    pub fn commitment(&self) -> String {
        match self {
            Self::Spot(result) => result.commitment(),
            Self::Perp(result) => result.commitment(),
            Self::Treasury(result) => result.commitment(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotCommandResult {
    ExecuteImmediateOrderPipeline(SpotPipelineExecution),
}

impl SpotCommandResult {
    pub fn commitment(&self) -> String {
        match self {
            Self::ExecuteImmediateOrderPipeline(result) => result.commitment(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotPipelineExecution {
    pub pipeline_output: ExecuteImmediateSpotOrderPipelineOutput,
    pub balances_after: Vec<Balance>,
    pub orders_after: Vec<SpotOrder>,
    pub trades: Vec<SpotTrade>,
    pub settlements: Vec<SpotSettlement>,
    pub settled_trade_ids_appended: Vec<String>,
    pub next_order_sequence: u64,
    pub events: Vec<EntityReplayableEvent>,
}

impl SpotPipelineExecution {
    pub fn commitment(&self) -> String {
        let balances = self.balances_after.iter().map(balance_commitment).collect::<Vec<_>>();
        let orders = self.orders_after.iter().map(spot_order_commitment).collect::<Vec<_>>();
        let trades = self.trades.iter().map(spot_trade_commitment).collect::<Vec<_>>();
        let settlements =
            self.settlements.iter().map(spot_settlement_commitment).collect::<Vec<_>>();
        stable_hash_hex(&[
            pipeline_output_commitment(&self.pipeline_output).as_str(),
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&orders).as_str(),
            stable_hash_hex(&trades).as_str(),
            stable_hash_hex(&settlements).as_str(),
            stable_hash_hex(&self.settled_trade_ids_appended).as_str(),
            self.next_order_sequence.to_string().as_str(),
            stable_hash_hex(&self.events.iter().map(super::event_commitment).collect::<Vec<_>>())
                .as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PerpCommandResult {
    Unsupported,
}

impl PerpCommandResult {
    pub fn commitment(&self) -> String {
        stable_hash_hex(&["perp.unsupported"])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreasuryCommandResult {
    QuoteBalanceUpdated(TreasuryBalanceUpdate),
}

impl TreasuryCommandResult {
    pub fn commitment(&self) -> String {
        match self {
            Self::QuoteBalanceUpdated(result) => result.commitment(),
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpotState {
    pub market_rules_by_symbol: BTreeMap<String, MarketRules>,
    pub asset_pairs_by_symbol: BTreeMap<String, SpotAssetPair>,
    pub trading_enabled_by_symbol: BTreeMap<String, bool>,
    pub balances: BTreeMap<AccountAssetKey, Balance>,
    pub orders: BTreeMap<String, SpotOrder>,
    pub settled_trade_ids: BTreeSet<String>,
    pub next_order_sequence_by_account: BTreeMap<String, u64>,
}

impl SpotState {
    pub fn commitment(&self) -> String {
        let market_rules = self
            .market_rules_by_symbol
            .iter()
            .map(|(symbol, rules)| format!("{symbol}:{}", market_rules_commitment(rules)))
            .collect::<Vec<_>>();
        let asset_pairs = self
            .asset_pairs_by_symbol
            .iter()
            .map(|(symbol, pair)| format!("{symbol}:{}", pair.commitment()))
            .collect::<Vec<_>>();
        let runtime = self
            .trading_enabled_by_symbol
            .iter()
            .map(|(symbol, enabled)| format!("{symbol}:{enabled}"))
            .collect::<Vec<_>>();
        let balances = self.balances.values().map(balance_commitment).collect::<Vec<_>>();
        let orders = self.orders.values().map(spot_order_commitment).collect::<Vec<_>>();
        let settled = self.settled_trade_ids.iter().cloned().collect::<Vec<_>>();
        let sequences = self
            .next_order_sequence_by_account
            .iter()
            .map(|(account_id, sequence)| format!("{account_id}:{sequence}"))
            .collect::<Vec<_>>();

        stable_hash_hex(&[
            stable_hash_hex(&market_rules).as_str(),
            stable_hash_hex(&asset_pairs).as_str(),
            stable_hash_hex(&runtime).as_str(),
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&orders).as_str(),
            stable_hash_hex(&settled).as_str(),
            stable_hash_hex(&sequences).as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PerpState {
    pub orders: BTreeMap<String, HyperliquidPerpOrder>,
    pub positions: BTreeMap<AccountMarketKey, HyperliquidPerpPosition>,
    pub balances: BTreeMap<AccountAssetKey, Balance>,
    pub funding_settlements: BTreeMap<String, HyperliquidPerpFundingSettlement>,
    pub trading_enabled_by_market: BTreeMap<String, bool>,
    pub next_order_sequence_by_account: BTreeMap<String, u64>,
}

impl PerpState {
    pub fn commitment(&self) -> String {
        let orders = self.orders.values().map(perp_order_commitment).collect::<Vec<_>>();
        let positions = self.positions.values().map(perp_position_commitment).collect::<Vec<_>>();
        let balances = self.balances.values().map(balance_commitment).collect::<Vec<_>>();
        let funding = self
            .funding_settlements
            .values()
            .map(perp_funding_settlement_commitment)
            .collect::<Vec<_>>();
        let runtime = self
            .trading_enabled_by_market
            .iter()
            .map(|(market, enabled)| format!("{market}:{enabled}"))
            .collect::<Vec<_>>();
        let sequences = self
            .next_order_sequence_by_account
            .iter()
            .map(|(account_id, sequence)| format!("{account_id}:{sequence}"))
            .collect::<Vec<_>>();

        stable_hash_hex(&[
            stable_hash_hex(&orders).as_str(),
            stable_hash_hex(&positions).as_str(),
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&funding).as_str(),
            stable_hash_hex(&runtime).as_str(),
            stable_hash_hex(&sequences).as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreasuryState {
    pub balances: BTreeMap<AccountAssetKey, Balance>,
    pub processed_transfer_ids: BTreeSet<String>,
    pub processed_withdraw_ids: BTreeSet<String>,
    pub withdraw_locks: BTreeMap<AccountAssetKey, WithdrawLockState>,
}

impl TreasuryState {
    pub fn commitment(&self) -> String {
        let balances = self.balances.values().map(balance_commitment).collect::<Vec<_>>();
        let transfers = self.processed_transfer_ids.iter().cloned().collect::<Vec<_>>();
        let withdraws = self.processed_withdraw_ids.iter().cloned().collect::<Vec<_>>();
        let locks =
            self.withdraw_locks.values().map(WithdrawLockState::commitment).collect::<Vec<_>>();

        stable_hash_hex(&[
            stable_hash_hex(&balances).as_str(),
            stable_hash_hex(&transfers).as_str(),
            stable_hash_hex(&withdraws).as_str(),
            stable_hash_hex(&locks).as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountAssetKey {
    pub account_id: String,
    pub asset: String,
}

impl AccountAssetKey {
    pub fn new(account_id: impl Into<String>, asset: impl Into<String>) -> Self {
        Self { account_id: account_id.into(), asset: asset.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AccountMarketKey {
    pub account_id: String,
    pub market: String,
}

impl AccountMarketKey {
    pub fn new(account_id: impl Into<String>, market: impl Into<String>) -> Self {
        Self { account_id: account_id.into(), market: market.into() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotAssetPair {
    pub base_asset_id: String,
    pub quote_asset_id: String,
}

impl SpotAssetPair {
    pub fn new(base_asset_id: impl Into<String>, quote_asset_id: impl Into<String>) -> Self {
        Self { base_asset_id: base_asset_id.into(), quote_asset_id: quote_asset_id.into() }
    }

    fn commitment(&self) -> String {
        stable_hash_hex(&[self.base_asset_id.as_str(), self.quote_asset_id.as_str()])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreasuryBalanceUpdate {
    pub balance_after: Balance,
    pub events: Vec<EntityReplayableEvent>,
}

impl TreasuryBalanceUpdate {
    fn commitment(&self) -> String {
        stable_hash_hex(&[
            balance_commitment(&self.balance_after).as_str(),
            stable_hash_hex(&self.events.iter().map(super::event_commitment).collect::<Vec<_>>())
                .as_str(),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithdrawLockState {
    pub account_id: String,
    pub asset: String,
    pub locked_amount: u64,
    pub reason: String,
    pub version: u64,
}

impl WithdrawLockState {
    fn commitment(&self) -> String {
        stable_hash_hex(&[
            self.account_id.as_str(),
            self.asset.as_str(),
            self.locked_amount.to_string().as_str(),
            self.reason.as_str(),
            self.version.to_string().as_str(),
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

fn pipeline_output_commitment(output: &ExecuteImmediateSpotOrderPipelineOutput) -> String {
    let place = stable_hash_hex(&[
        spot_order_commitment(&output.place_output.order).as_str(),
        balance_commitment(&output.place_output.affected_balance_after).as_str(),
    ]);
    let matched = output.match_output.as_ref().map_or_else(
        || stable_hash_hex(&["none"]),
        |match_output| {
            let trades = match_output.trades.iter().map(spot_trade_commitment).collect::<Vec<_>>();
            let makers = match_output
                .maker_orders_after
                .iter()
                .map(spot_order_commitment)
                .collect::<Vec<_>>();
            stable_hash_hex(&[
                stable_hash_hex(&trades).as_str(),
                spot_order_commitment(&match_output.taker_order_after).as_str(),
                stable_hash_hex(&makers).as_str(),
            ])
        },
    );
    stable_hash_hex(&[place.as_str(), matched.as_str()])
}

pub fn market_rules_commitment(rules: &MarketRules) -> String {
    stable_hash_hex(&[rules.symbol.as_str(), rules.min_qty.to_string().as_str()])
}

pub fn balance_commitment(balance: &Balance) -> String {
    stable_hash_hex(&[
        balance.account_id.as_str(),
        balance.asset_id.as_str(),
        balance.available.to_string().as_str(),
        balance.frozen.to_string().as_str(),
        balance.version.to_string().as_str(),
    ])
}

pub fn spot_order_commitment(order: &SpotOrder) -> String {
    let execution = match order.execution {
        example_core::SpotOrderExecution::Market { aggressive_price } => {
            format!("market:{aggressive_price}")
        }
        example_core::SpotOrderExecution::Limit { price } => format!("limit:{price}"),
    };
    let status_reason = order.status_reason.map(|value| value.as_str()).unwrap_or_default();
    stable_hash_hex(&[
        order.order_id.as_str(),
        order.asset.to_string().as_str(),
        order.exchange_oid.map(|value| value.to_string()).unwrap_or_default().as_str(),
        order.account_id.as_str(),
        order.symbol.as_str(),
        order.side.as_str(),
        execution.as_str(),
        spot_order_tif(order.time_in_force),
        order.qty.to_string().as_str(),
        order.filled_qty.to_string().as_str(),
        order.status.as_str(),
        status_reason,
        order.reserved_base.to_string().as_str(),
        order.reserved_quote.to_string().as_str(),
        order.client_order_id.as_deref().unwrap_or_default(),
        order.version.to_string().as_str(),
    ])
}

pub fn spot_trade_commitment(trade: &SpotTrade) -> String {
    stable_hash_hex(&[
        trade.trade_id.as_str(),
        trade.match_id.as_str(),
        trade.asset.to_string().as_str(),
        trade.symbol.as_str(),
        trade.taker_order_id.as_str(),
        trade.maker_order_id.as_str(),
        trade.taker_account_id.as_str(),
        trade.maker_account_id.as_str(),
        trade.taker_side.as_str(),
        trade.price.to_string().as_str(),
        trade.qty.to_string().as_str(),
    ])
}

pub fn spot_settlement_commitment(settlement: &SpotSettlement) -> String {
    stable_hash_hex(&[
        settlement.settlement_id.as_str(),
        settlement.trade_id.as_str(),
        settlement.match_id.as_str(),
        settlement.buyer_account_id.as_str(),
        settlement.seller_account_id.as_str(),
        settlement.base_qty.to_string().as_str(),
        settlement.quote_qty.to_string().as_str(),
        settlement.price.to_string().as_str(),
    ])
}

fn perp_order_commitment(order: &HyperliquidPerpOrder) -> String {
    stable_hash_hex(&[format!("{order:?}")])
}

fn perp_position_commitment(position: &HyperliquidPerpPosition) -> String {
    stable_hash_hex(&[format!("{position:?}")])
}

fn perp_funding_settlement_commitment(settlement: &HyperliquidPerpFundingSettlement) -> String {
    stable_hash_hex(&[format!("{settlement:?}")])
}

fn spot_tif(value: example_core::PlaceOrderTimeInForce) -> &'static str {
    match value {
        example_core::PlaceOrderTimeInForce::Gtc => "gtc",
        example_core::PlaceOrderTimeInForce::Ioc => "ioc",
        example_core::PlaceOrderTimeInForce::Alo => "alo",
    }
}

fn spot_order_tif(value: example_core::SpotOrderTimeInForce) -> &'static str {
    match value {
        example_core::SpotOrderTimeInForce::Gtc => "gtc",
        example_core::SpotOrderTimeInForce::Ioc => "ioc",
        example_core::SpotOrderTimeInForce::Alo => "alo",
    }
}
