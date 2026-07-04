use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use example_core::{
    Balance, MarketRules, PlaceImmediateOrderCmd, PlaceImmediateOrderState, Reservation,
    ReservationCloseReason, ReservationKind, ReservationMarketKind, ReservationStatus, SpotOrder,
    SpotOrderExecution, SpotOrderSide, SpotOrderTimeInForce,
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

    pub fn seed_order(&self, order: SpotOrder) -> Result<(), crate::StoreError> {
        self.store.seed_order(order)
    }

    pub fn snapshot(&self) -> Result<crate::StoreSnapshot, crate::StoreError> {
        self.store.snapshot()
    }
}

impl CommandUseCaseOutbound for InMemoryPlaceOrderOutbound {
    type Command = PlaceImmediateOrderCmd;
    type State = PlaceImmediateOrderState;
    type Error = PlaceOrderOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let state = self.store.lock_state()?;
        let market_rules = state
            .market_rules_by_symbol
            .get(cmd.symbol.as_str())
            .cloned()
            .ok_or(PlaceOrderOutboundError::MarketRulesNotFound)?;
        let base_asset_id = base_asset_id_for(cmd.symbol.as_str());
        let quote_asset_id = quote_asset_id_for(cmd.symbol.as_str());
        let base_balance = state
            .balances
            .get(&balance_key(cmd.party_id.as_str(), base_asset_id))
            .cloned()
            .ok_or(PlaceOrderOutboundError::BalanceNotFound)?;
        let quote_balance = state
            .balances
            .get(&balance_key(cmd.party_id.as_str(), quote_asset_id))
            .cloned()
            .ok_or(PlaceOrderOutboundError::BalanceNotFound)?;

        Ok(PlaceImmediateOrderState {
            trading_enabled: true,
            next_order_sequence: state.next_order_sequence,
            account_id: cmd.party_id.clone(),
            base_balance,
            quote_balance,
            market_rules,
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
                && event_string_field(event, "status").is_some()
            {
                let reservation = decode_created_reservation(event)?;
                state.reservations.insert(reservation.reservation_id.clone(), reservation);
                continue;
            }

            if event.is_created() && event_string_field(event, "order_id").is_some() {
                let order = SpotOrder::new(
                    event_string_field(event, "order_id")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
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
                    event_u64_field(event, "reserved_base")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    event_u64_field(event, "reserved_quote")
                        .ok_or(PlaceOrderOutboundError::EventDecodeFailed)?,
                    event_string_field(event, "client_order_id").filter(|value| !value.is_empty()),
                );
                state.orders.insert(order.order_id.clone(), order);
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
