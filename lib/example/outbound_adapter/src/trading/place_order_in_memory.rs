use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::MiFamilyOutbound;
use example_core::{
    Balance, MarketRules, PlaceSpotOrderV2TakerTemplateContextV3, Reservation,
    ReservationCloseReason, ReservationKind, ReservationMarketKind, ReservationStatus,
    SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderTimeInForce, SpotOrderV2,
    SpotOrderV2CommandV3, SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3, SpotTrade,
    build_place_spot_order_v2_taker_template_v3,
};

use crate::shared::{
    InMemorySpotPipelineBroker, InMemoryStore, PlaceOrderOutboundError, SpotOrderPlacedMessage,
    SpotPipelineBroker, SpotPipelineMessage, balance_key, event_string_field, event_u64_field,
};

#[derive(Debug, Clone, Default)]
pub struct InMemoryPlaceOrderOutbound {
    store: InMemoryStore,
    broker: Option<InMemorySpotPipelineBroker>,
}

impl InMemoryPlaceOrderOutbound {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_store(store: InMemoryStore) -> Self {
        Self { store, broker: None }
    }

    pub fn from_store_with_broker(
        store: InMemoryStore,
        broker: InMemorySpotPipelineBroker,
    ) -> Self {
        Self { store, broker: Some(broker) }
    }

    pub fn seeded(
        base_balance: Balance,
        quote_balance: Balance,
        market_rules: MarketRules,
    ) -> Result<Self, crate::StoreError> {
        Ok(Self::from_store(InMemoryStore::seed_balances(
            base_balance,
            quote_balance,
            market_rules,
        )?))
    }

    pub fn seed_balance(&self, balance: Balance) -> Result<(), crate::StoreError> {
        self.store.seed_balance(balance)
    }

    pub fn seed_market_rules(&self, market_rules: MarketRules) -> Result<(), crate::StoreError> {
        self.store.seed_market_rules(market_rules)
    }

    pub fn seed_order(&self, order: SpotOrderV2) -> Result<(), crate::StoreError> {
        self.store.seed_order(order)
    }

    pub fn snapshot(&self) -> Result<crate::StoreSnapshot, crate::StoreError> {
        self.store.snapshot()
    }
}

const DEFAULT_FEE_ACCOUNT_ID: &str = "fee";
const DEFAULT_MAKER_FEE_BPS: u64 = 5;
const DEFAULT_TAKER_FEE_BPS: u64 = 10;

impl MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3> for InMemoryPlaceOrderOutbound {
    type Error = PlaceOrderOutboundError;

    fn load_given_state(
        &self,
        cmd: &SpotOrderV2CommandV3,
    ) -> Result<SpotOrderV2GivenStateV3, Self::Error> {
        let SpotOrderV2CommandV3::Place(cmd) = cmd else {
            return Err(PlaceOrderOutboundError::UnsupportedCommandBranch);
        };
        let state = self.store.lock_state()?;
        let symbol = symbol_for_asset(cmd.asset);
        let market_rules = state
            .market_rules_by_symbol
            .get(symbol)
            .cloned()
            .ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;
        let base_asset_id = base_asset_id_for(market_rules.symbol.as_str()).to_string();
        let quote_asset_id = quote_asset_id_for(market_rules.symbol.as_str()).to_string();
        let mut settlement_balances = state.balances.values().cloned().collect::<Vec<_>>();
        if !state
            .balances
            .contains_key(&balance_key(DEFAULT_FEE_ACCOUNT_ID, quote_asset_id.as_str()))
        {
            settlement_balances.push(Balance::new(
                DEFAULT_FEE_ACCOUNT_ID.to_string(),
                quote_asset_id.clone(),
                0,
                0,
                1,
            ));
        }
        if !state.balances.contains_key(&balance_key(cmd.party_id.as_str(), base_asset_id.as_str()))
            || !state
                .balances
                .contains_key(&balance_key(cmd.party_id.as_str(), quote_asset_id.as_str()))
        {
            return Err(PlaceOrderOutboundError::BalanceNotFound);
        }

        let order_id =
            format!("{}-{}-{}", cmd.party_id, market_rules.symbol, state.next_order_sequence);
        let taker_order = build_place_spot_order_v2_taker_template_v3(
            cmd,
            PlaceSpotOrderV2TakerTemplateContextV3 {
                order_id,
                symbol: market_rules.symbol.clone(),
                settlement_balances: &settlement_balances,
                base_asset_id: base_asset_id.clone(),
                quote_asset_id: quote_asset_id.clone(),
                maker_fee_bps: DEFAULT_MAKER_FEE_BPS,
                taker_fee_bps: DEFAULT_TAKER_FEE_BPS,
            },
        )?;
        let maker_orders = state
            .orders
            .values()
            .filter(|order| {
                order.trades_asset(cmd.asset)
                    && order.trades_symbol(market_rules.symbol.as_str())
                    && order.side() != taker_order.side()
                    && matches!(
                        order.status(),
                        SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
                    )
            })
            .cloned()
            .collect();

        Ok(SpotOrderV2GivenStateV3::Place {
            taker_order,
            maker_orders,
            settlement_balances,
            base_asset_id,
            quote_asset_id,
            fee_account_id: DEFAULT_FEE_ACCOUNT_ID.to_string(),
            maker_fee_bps: DEFAULT_MAKER_FEE_BPS,
            taker_fee_bps: DEFAULT_TAKER_FEE_BPS,
        })
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.persisted_events.extend(events.iter().cloned());
        Ok(())
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;

        for event in events {
            if event.is_created()
                && event_string_field(event, "reservation_id").is_some()
                && event_string_field(event, "owner_account_id").is_some()
                && event_string_field(event, "caused_by_order_id").is_some()
            {
                let reservation = decode_created_reservation(event)?;
                state.reservations.insert(reservation.reservation_id.clone(), reservation);
                continue;
            }

            if event.is_created() && event_string_field(event, "order_id").is_some() {
                let order = SpotOrderV2::new(
                    {
                        event_string_field(event, "order_id")
                            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                    },
                    event_u64_field(event, "asset")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                        as u32,
                    event_u64_field(event, "exchange_oid"),
                    event_string_field(event, "account_id")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    event_string_field(event, "symbol")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    decode_side(event)?,
                    decode_execution(event)?,
                    decode_time_in_force(event)?,
                    event_u64_field(event, "qty")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    event_u64_field(event, "filled_qty")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    decode_status(event)?,
                    None,
                    decode_embedded_order_reservation(event)?,
                    event_string_field(event, "client_order_id").filter(|value| !value.is_empty()),
                    event_u64_field(event, "version").unwrap_or(1),
                );
                state.orders.insert(order.order_id.clone(), order);
                state.next_order_sequence = state
                    .next_order_sequence
                    .checked_add(1)
                    .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
                continue;
            }

            if event.is_created() && event_string_field(event, "trade_id").is_some() {
                let trade = decode_created_trade(event)?;
                state.trades.insert(trade.trade_id.clone(), trade);
                continue;
            }

            if event.is_updated() && event_string_field(event, "order_id").is_some() {
                apply_order_update_event(&mut state.orders, event)?;
                state.next_order_sequence = state
                    .next_order_sequence
                    .checked_add(1)
                    .ok_or(PlaceOrderOutboundError::SequenceOverflow)?;
                continue;
            }

            if event.is_updated() && event_string_field(event, "asset_id").is_some() {
                let account_id = event_string_field(event, "account_id")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let asset_id = event_string_field(event, "asset_id")
                    .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
                let key = balance_key(account_id.as_str(), asset_id.as_str());
                let balance = state
                    .balances
                    .entry(key)
                    .or_insert_with(|| Balance::new(account_id.clone(), asset_id.clone(), 0, 0, 0));
                if let Some(available) = event_u64_field(event, "available") {
                    balance.available = available;
                }
                if let Some(frozen) = event_u64_field(event, "frozen") {
                    balance.frozen = frozen;
                }
                balance.version = event.new_version;
            }
        }

        Ok(())
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        let mut state = self.store.lock_state()?;
        state.published_events.extend(events.iter().cloned());
        drop(state);

        let Some(broker) = &self.broker else {
            return Ok(());
        };

        for event in events {
            if !event.is_created() || event_string_field(event, "order_id").is_none() {
                continue;
            }

            let order_id = event_string_field(event, "order_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let party_id = event_string_field(event, "account_id")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let symbol = event_string_field(event, "symbol")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
            let asset = event_u64_field(event, "asset")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)? as u32;
            broker
                .publish(SpotPipelineMessage::SpotOrderPlaced(SpotOrderPlacedMessage {
                    match_id: order_id.clone(),
                    order_id,
                    party_id,
                    symbol,
                    asset,
                    trace_id: None,
                    command_id: None,
                }))
                .map_err(|_| PlaceOrderOutboundError::BrokerPublishFailed)?;
        }
        Ok(())
    }
}

fn decode_side(event: &EntityReplayableEvent) -> Result<SpotOrderSide, PlaceOrderOutboundError> {
    match event_string_field(event, "side").as_deref() {
        Some("buy") => Ok(SpotOrderSide::Buy),
        Some("sell") => Ok(SpotOrderSide::Sell),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_execution(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderExecution, PlaceOrderOutboundError> {
    let price =
        event_u64_field(event, "price").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    match event_string_field(event, "execution").as_deref() {
        Some("market") => Ok(SpotOrderExecution::Market { aggressive_price: price }),
        Some("limit") => Ok(SpotOrderExecution::Limit { price }),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_time_in_force(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderTimeInForce, PlaceOrderOutboundError> {
    match event_string_field(event, "time_in_force").as_deref() {
        Some("gtc") => Ok(SpotOrderTimeInForce::Gtc),
        Some("ioc") => Ok(SpotOrderTimeInForce::Ioc),
        Some("alo") => Ok(SpotOrderTimeInForce::Alo),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_status(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderStatus, PlaceOrderOutboundError> {
    match event_string_field(event, "status").as_deref() {
        Some("open") => Ok(SpotOrderStatus::Open),
        Some("partially_filled") => Ok(SpotOrderStatus::PartiallyFilled),
        Some("filled") => Ok(SpotOrderStatus::Filled),
        Some("canceled") => Ok(SpotOrderStatus::Canceled),
        Some("rejected") => Ok(SpotOrderStatus::Rejected),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_embedded_order_reservation(
    event: &EntityReplayableEvent,
) -> Result<Reservation, PlaceOrderOutboundError> {
    let order_id =
        event_string_field(event, "order_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let account_id = event_string_field(event, "account_id")
        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    let reservation_id = event_string_field(event, "reservation_id")
        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    Ok(Reservation {
        reservation_id,
        owner_account_id: account_id,
        caused_by_order_id: order_id,
        market_kind: ReservationMarketKind::Spot,
        reservation_kind: decode_reservation_kind(
            event_string_field(event, "reservation_kind")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                .as_str(),
        )?,
        asset_id: event_string_field(event, "reservation_asset_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        original_amount: event_u64_field(event, "reservation_original_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        consumed_amount: event_u64_field(event, "reservation_consumed_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        released_amount: event_u64_field(event, "reservation_released_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        remaining_amount: event_u64_field(event, "reservation_remaining_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        status: decode_reservation_status(
            event_string_field(event, "reservation_status")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                .as_str(),
        )?,
        close_reason: None,
        version: 1,
    })
}

fn decode_created_reservation(
    event: &EntityReplayableEvent,
) -> Result<Reservation, PlaceOrderOutboundError> {
    Ok(Reservation {
        reservation_id: event_string_field(event, "reservation_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        owner_account_id: event_string_field(event, "owner_account_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        caused_by_order_id: event_string_field(event, "caused_by_order_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        market_kind: decode_market_kind(
            event_string_field(event, "market_kind")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                .as_str(),
        )?,
        reservation_kind: decode_reservation_kind(
            event_string_field(event, "reservation_kind")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                .as_str(),
        )?,
        asset_id: event_string_field(event, "asset_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        original_amount: event_u64_field(event, "original_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        consumed_amount: event_u64_field(event, "consumed_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        released_amount: event_u64_field(event, "released_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        remaining_amount: event_u64_field(event, "remaining_amount")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        status: decode_reservation_status(
            event_string_field(event, "status")
                .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?
                .as_str(),
        )?,
        close_reason: decode_optional_close_reason(
            event_string_field(event, "close_reason").as_deref(),
        )?,
        version: event.new_version,
    })
}

fn decode_market_kind(value: &str) -> Result<ReservationMarketKind, PlaceOrderOutboundError> {
    match value {
        "spot" => Ok(ReservationMarketKind::Spot),
        "perp" => Ok(ReservationMarketKind::Perp),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_reservation_kind(value: &str) -> Result<ReservationKind, PlaceOrderOutboundError> {
    match value {
        "spot_buy_quote" => Ok(ReservationKind::SpotBuyQuote),
        "spot_sell_base" => Ok(ReservationKind::SpotSellBase),
        "spot_buy_fee_quote" => Ok(ReservationKind::SpotBuyFeeQuote),
        "spot_sell_fee_quote" => Ok(ReservationKind::SpotSellFeeQuote),
        "perp_open_margin" => Ok(ReservationKind::PerpOpenMargin),
        "perp_flip_net_new_margin" => Ok(ReservationKind::PerpFlipNetNewMargin),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_reservation_status(value: &str) -> Result<ReservationStatus, PlaceOrderOutboundError> {
    match value {
        "active" => Ok(ReservationStatus::Active),
        "exhausted_by_consume" => Ok(ReservationStatus::ExhaustedByConsume),
        "closed_by_release" => Ok(ReservationStatus::ClosedByRelease),
        "closed_mixed" => Ok(ReservationStatus::ClosedMixed),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn decode_optional_close_reason(
    value: Option<&str>,
) -> Result<Option<ReservationCloseReason>, PlaceOrderOutboundError> {
    match value {
        None | Some("") => Ok(None),
        Some("filled") => Ok(Some(ReservationCloseReason::Filled)),
        Some("canceled") => Ok(Some(ReservationCloseReason::Canceled)),
        Some("rejected") => Ok(Some(ReservationCloseReason::Rejected)),
        Some("ioc_remainder_canceled") => Ok(Some(ReservationCloseReason::IocRemainderCanceled)),
        Some("expired") => Ok(Some(ReservationCloseReason::Expired)),
        Some(_) => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

pub(crate) fn base_asset_id_for(symbol: &str) -> &str {
    match symbol {
        "BTCUSDT" => "BTC",
        _ => "BTC",
    }
}

pub(crate) fn quote_asset_id_for(symbol: &str) -> &str {
    match symbol {
        "BTCUSDT" => "USDT",
        _ => "USDT",
    }
}

pub(crate) fn symbol_for_asset(asset: u32) -> &'static str {
    match asset {
        10_000 | 10_001 => "BTCUSDT",
        _ => "BTCUSDT",
    }
}

fn decode_created_trade(
    event: &EntityReplayableEvent,
) -> Result<SpotTrade, PlaceOrderOutboundError> {
    Ok(SpotTrade::new(
        event_string_field(event, "trade_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "match_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "asset").ok_or(PlaceOrderOutboundError::EventDecodeFailed)? as u32,
        event_string_field(event, "symbol").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "taker_order_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "maker_order_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "taker_account_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "maker_account_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        decode_trade_side(event)?,
        event_u64_field(event, "price").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "qty").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "taker_fee").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "maker_fee").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
    ))
}

fn decode_trade_side(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderSide, PlaceOrderOutboundError> {
    match event_string_field(event, "taker_side").as_deref() {
        Some("buy") => Ok(SpotOrderSide::Buy),
        Some("sell") => Ok(SpotOrderSide::Sell),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}

fn apply_order_update_event(
    orders: &mut std::collections::HashMap<String, SpotOrderV2>,
    event: &EntityReplayableEvent,
) -> Result<(), PlaceOrderOutboundError> {
    let order_id =
        event_string_field(event, "order_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    if !orders.contains_key(&order_id) {
        orders.insert(order_id.clone(), decode_order_snapshot_from_event(event)?);
    }
    let order = orders.get_mut(&order_id).ok_or(PlaceOrderOutboundError::EventDecodeFailed)?;
    if let Some(status) = event_string_field(event, "status") {
        order.status = decode_status_value(status.as_str())?;
    }
    if let Some(filled_qty) = event_u64_field(event, "filled_qty") {
        order.filled_qty = filled_qty;
    }
    order.version = event.new_version;
    Ok(())
}

fn decode_order_snapshot_from_event(
    event: &EntityReplayableEvent,
) -> Result<SpotOrderV2, PlaceOrderOutboundError> {
    Ok(SpotOrderV2::new(
        event_string_field(event, "order_id").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "asset").ok_or(PlaceOrderOutboundError::EventDecodeFailed)? as u32,
        event_u64_field(event, "exchange_oid"),
        event_string_field(event, "account_id")
            .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_string_field(event, "symbol").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        decode_side(event)?,
        decode_execution(event)?,
        decode_time_in_force(event)?,
        event_u64_field(event, "qty").ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "filled_qty").unwrap_or(0),
        decode_status(event).unwrap_or(SpotOrderStatus::Open),
        None,
        decode_embedded_order_reservation(event)?,
        event_string_field(event, "client_order_id").filter(|value| !value.is_empty()),
        event.new_version,
    ))
}

fn decode_status_value(value: &str) -> Result<SpotOrderStatus, PlaceOrderOutboundError> {
    match value {
        "open" => Ok(SpotOrderStatus::Open),
        "partially_filled" => Ok(SpotOrderStatus::PartiallyFilled),
        "filled" => Ok(SpotOrderStatus::Filled),
        "canceled" => Ok(SpotOrderStatus::Canceled),
        "rejected" => Ok(SpotOrderStatus::Rejected),
        _ => Err(PlaceOrderOutboundError::EventDecodeFailed),
    }
}
