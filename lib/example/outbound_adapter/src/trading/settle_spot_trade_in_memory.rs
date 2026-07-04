use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCaseOutbound;
use example_core::{
    Balance, Reservation, ReservationCloseReason, ReservationStatus, SettleSpotTradeCmd,
    SettleSpotTradeState, SpotOrderSide, SpotSettlement,
};

use crate::shared::{
    InMemoryStore, SettleSpotTradeOutboundError, balance_key, event_string_field, event_u64_field,
};

#[derive(Debug, Clone)]
pub struct InMemorySettleSpotTradeOutbound {
    store: InMemoryStore,
}

impl InMemorySettleSpotTradeOutbound {
    pub fn new(store: InMemoryStore) -> Self {
        Self { store }
    }
}

impl CommandUseCaseOutbound for InMemorySettleSpotTradeOutbound {
    type Command = SettleSpotTradeCmd;
    type State = SettleSpotTradeState;
    type Error = SettleSpotTradeOutboundError;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        let state = self.store.lock_state()?;
        let trades = cmd
            .trade_ids
            .iter()
            .map(|trade_id| {
                state
                    .trades
                    .get(trade_id.as_str())
                    .cloned()
                    .ok_or(SettleSpotTradeOutboundError::TradeNotFound)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut balances = Vec::new();
        let mut reservations = Vec::new();
        for trade in &trades {
            let buyer_account_id = if matches!(trade.taker_side, SpotOrderSide::Buy) {
                trade.taker_account_id.as_str()
            } else {
                trade.maker_account_id.as_str()
            };
            let seller_account_id = if buyer_account_id == trade.taker_account_id {
                trade.maker_account_id.as_str()
            } else {
                trade.taker_account_id.as_str()
            };
            for key in [
                balance_key(buyer_account_id, "BTC"),
                balance_key(buyer_account_id, "USDT"),
                balance_key(seller_account_id, "BTC"),
                balance_key(seller_account_id, "USDT"),
            ] {
                let balance = state
                    .balances
                    .get(key.as_str())
                    .cloned()
                    .ok_or(SettleSpotTradeOutboundError::BalanceNotFound)?;
                if !balances.iter().any(|existing: &Balance| {
                    existing.account_id == balance.account_id
                        && existing.asset_id == balance.asset_id
                }) {
                    balances.push(balance);
                }
            }

            for (order_id, account_id, asset_id) in [
                (
                    buyer_order_id(trade),
                    buyer_account_id,
                    "USDT",
                ),
                (
                    seller_order_id(trade),
                    seller_account_id,
                    "BTC",
                ),
            ] {
                let reservation = state
                    .reservations
                    .values()
                    .find(|reservation| {
                        reservation.caused_by_order_id == order_id
                            && reservation.owner_account_id == account_id
                            && reservation.asset_id == asset_id
                    })
                    .cloned()
                    .ok_or(SettleSpotTradeOutboundError::ReservationNotFound)?;
                if !reservations.iter().any(|existing: &Reservation| {
                    existing.reservation_id == reservation.reservation_id
                }) {
                    reservations.push(reservation);
                }
            }
        }

        Ok(SettleSpotTradeState {
            trades,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            balances,
            reservations,
            settled_trade_ids: state.settlements.values().map(|it| it.trade_id.clone()).collect(),
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
            if event.is_created() && event_string_field(event, "settlement_id").is_some() {
                let settlement = decode_settlement(event)?;
                state.settlements.insert(settlement.settlement_id.clone(), settlement);
                continue;
            }

            if event.is_updated() && event_string_field(event, "reservation_id").is_some() {
                let reservation_id = event_string_field(event, "reservation_id")
                    .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?;
                let reservation = state
                    .reservations
                    .get_mut(reservation_id.as_str())
                    .ok_or(SettleSpotTradeOutboundError::ReservationNotFound)?;
                if let Some(consumed_amount) = event_u64_field(event, "consumed_amount") {
                    reservation.consumed_amount = consumed_amount;
                }
                if let Some(released_amount) = event_u64_field(event, "released_amount") {
                    reservation.released_amount = released_amount;
                }
                if let Some(remaining_amount) = event_u64_field(event, "remaining_amount") {
                    reservation.remaining_amount = remaining_amount;
                }
                if let Some(status) = event_string_field(event, "status") {
                    reservation.status = decode_reservation_status(status.as_str())?;
                }
                if let Some(close_reason) = event_string_field(event, "close_reason") {
                    reservation.close_reason = decode_optional_close_reason(Some(close_reason.as_str()))?;
                }
                reservation.version = event.new_version;
                continue;
            }

            if event.is_updated() && event_string_field(event, "asset_id").is_some() {
                let account_id = event_string_field(event, "account_id")
                    .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?;
                let asset_id = event_string_field(event, "asset_id")
                    .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?;
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
        Ok(())
    }
}

fn decode_settlement(
    event: &EntityReplayableEvent,
) -> Result<SpotSettlement, SettleSpotTradeOutboundError> {
    Ok(SpotSettlement::new(
        event_string_field(event, "settlement_id")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_string_field(event, "trade_id")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_string_field(event, "match_id")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_string_field(event, "buyer_account_id")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_string_field(event, "seller_account_id")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "base_qty")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "quote_qty")
            .ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
        event_u64_field(event, "price").ok_or(SettleSpotTradeOutboundError::EventDecodeFailed)?,
    ))
}

fn buyer_order_id(trade: &example_core::SpotTrade) -> &str {
    match trade.taker_side {
        SpotOrderSide::Buy => trade.taker_order_id.as_str(),
        SpotOrderSide::Sell => trade.maker_order_id.as_str(),
    }
}

fn seller_order_id(trade: &example_core::SpotTrade) -> &str {
    match trade.taker_side {
        SpotOrderSide::Buy => trade.maker_order_id.as_str(),
        SpotOrderSide::Sell => trade.taker_order_id.as_str(),
    }
}

fn decode_reservation_status(
    value: &str,
) -> Result<ReservationStatus, SettleSpotTradeOutboundError> {
    match value {
        "active" => Ok(ReservationStatus::Active),
        "exhausted_by_consume" => Ok(ReservationStatus::ExhaustedByConsume),
        "closed_by_release" => Ok(ReservationStatus::ClosedByRelease),
        "closed_mixed" => Ok(ReservationStatus::ClosedMixed),
        _ => Err(SettleSpotTradeOutboundError::EventDecodeFailed),
    }
}

fn decode_optional_close_reason(
    value: Option<&str>,
) -> Result<Option<ReservationCloseReason>, SettleSpotTradeOutboundError> {
    match value {
        None | Some("") => Ok(None),
        Some("filled") => Ok(Some(ReservationCloseReason::Filled)),
        Some("canceled") => Ok(Some(ReservationCloseReason::Canceled)),
        Some("rejected") => Ok(Some(ReservationCloseReason::Rejected)),
        Some("ioc_remainder_canceled") => Ok(Some(ReservationCloseReason::IocRemainderCanceled)),
        Some("expired") => Ok(Some(ReservationCloseReason::Expired)),
        Some(_) => Err(SettleSpotTradeOutboundError::EventDecodeFailed),
    }
}
