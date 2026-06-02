use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use thiserror::Error;

use crate::entity::PlaceOrderState;
use crate::use_case::support::{
    ACCOUNT_ENTITY_TYPE, ORDER_ENTITY_TYPE, int_field, stable_entity_id, string_field,
    updated_int_field,
};

/// Command for placing a spot order that reserves quote balance from a trader account.
///
/// This command only carries caller input. Cheap input validation happens in
/// [`PlaceOrderUseCase::pre_check_command`], while market-rule and balance checks happen in
/// [`PlaceOrderUseCase::validate_against_state`].
///
/// # Examples
///
/// ```
/// use example_core::PlaceOrderCmd;
///
/// let cmd = PlaceOrderCmd {
///     party_id: "trader-1".to_string(),
///     symbol: "BTCUSDT".to_string(),
///     qty: 2,
///     price: 100,
/// };
///
/// assert_eq!(cmd.symbol, "BTCUSDT");
/// assert_eq!(cmd.qty, 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCmd {
    /// Trader account id that issues the command and owns the reserved quote balance.
    pub party_id: String,
    /// Market symbol to trade, for example `BTCUSDT`.
    pub symbol: String,
    /// Base-asset quantity to place. Must be greater than zero and satisfy market minimums.
    pub qty: u64,
    /// Limit price in quote units. Must be greater than zero.
    pub price: u64,
}

impl IssuedByParty for PlaceOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// Business errors that can reject a spot order placement.
///
/// These errors are stable enough for adapters to map into CLI, HTTP, or tracing output through
/// [`std::fmt::Display`].
///
/// # Examples
///
/// ```
/// use example_core::PlaceOrderError;
///
/// assert_eq!(
///     PlaceOrderError::InvalidQty.to_string(),
///     "qty must be greater than zero"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOrderError {
    /// Returned when `qty == 0`.
    #[error("qty must be greater than zero")]
    InvalidQty,
    /// Returned when `price == 0`.
    #[error("price must be greater than zero")]
    InvalidPrice,
    /// Returned when the quantity is below current market minimum rules.
    #[error("qty is below market minimum")]
    QtyBelowMin,
    /// Returned when the market is temporarily not accepting orders.
    #[error("trading is disabled")]
    TradingDisabled,
    /// Returned when the symbol is not supported by the loaded market rules.
    #[error("symbol is not tradable in current market rules")]
    SymbolNotTradable,
    /// Returned when the account cannot reserve enough quote balance.
    #[error("insufficient quote balance")]
    InsufficientQuoteBalance,
    /// Returned when numeric derivation overflows while computing business results.
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

/// Use case that validates a spot order command and derives replayable order/account events.
///
/// The use case itself is deterministic for the same command and loaded state. It does not talk to
/// storage, publish events, or shape HTTP replies.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceOrderUseCase;

impl CommandUseCase2 for PlaceOrderUseCase {
    type Command = PlaceOrderCmd;
    type GivenState = PlaceOrderState;
    type Error = PlaceOrderError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.qty == 0 {
            return Err(PlaceOrderError::InvalidQty);
        }

        if cmd.price == 0 {
            return Err(PlaceOrderError::InvalidPrice);
        }

        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.trading_enabled {
            return Err(PlaceOrderError::TradingDisabled);
        }

        if !state.market_rules.supports_symbol(cmd.symbol.as_str()) {
            return Err(PlaceOrderError::SymbolNotTradable);
        }

        if !state.market_rules.validate_qty(cmd.qty) {
            return Err(PlaceOrderError::QtyBelowMin);
        }

        let reserved_quote = state
            .market_rules
            .required_quote(cmd.qty, cmd.price)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;

        if !state.account.can_reserve_quote(reserved_quote) {
            return Err(PlaceOrderError::InsufficientQuoteBalance);
        }

        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let reserved_quote = state
            .market_rules
            .required_quote(cmd.qty, cmd.price)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let (next_available, next_frozen) = state
            .account
            .reserve_quote_after(reserved_quote)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let next_version =
            state.account.version.checked_add(1).ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);

        let mut order_event = EntityReplayableEvent::new_created(
            0,
            0,
            stable_entity_id(&order_id),
            ORDER_ENTITY_TYPE,
        );
        order_event.add_field_change(string_field("order_id", &order_id));
        order_event.add_field_change(string_field("account_id", &cmd.party_id));
        order_event.add_field_change(string_field("symbol", &cmd.symbol));
        order_event.add_field_change(int_field("order_sequence", state.next_order_sequence));
        order_event.add_field_change(int_field("qty", cmd.qty));
        order_event.add_field_change(int_field("price", cmd.price));
        order_event.add_field_change(int_field("reserved_quote", reserved_quote));

        let mut account_event = EntityReplayableEvent::new_updated(
            0,
            1,
            state.account.version,
            next_version,
            stable_entity_id(&state.account.account_id),
            ACCOUNT_ENTITY_TYPE,
        );
        account_event.add_field_change(string_field("account_id", &state.account.account_id));
        account_event.add_field_change(updated_int_field(
            "available_quote",
            state.account.available_quote,
            next_available,
        ));
        account_event.add_field_change(updated_int_field(
            "frozen_quote",
            state.account.frozen_quote,
            next_frozen,
        ));

        Ok(vec![order_event, account_event])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{MarketRules, TradingAccount};
    use crate::use_case::support::field_as_u64;

    fn sample_state() -> PlaceOrderState {
        PlaceOrderState {
            trading_enabled: true,
            next_order_sequence: 7,
            account: TradingAccount {
                account_id: "trader-1".to_string(),
                available_quote: 1_000,
                frozen_quote: 0,
                version: 3,
            },
            market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
        }
    }

    fn sample_cmd() -> PlaceOrderCmd {
        PlaceOrderCmd {
            party_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 2,
            price: 100,
        }
    }

    #[test]
    fn role_is_trader() {
        let use_case = PlaceOrderUseCase;
        assert_eq!(use_case.role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_zero_qty() {
        let use_case = PlaceOrderUseCase;
        let mut cmd = sample_cmd();
        cmd.qty = 0;

        let result = use_case.pre_check_command(&cmd);
        assert_eq!(result, Err(PlaceOrderError::InvalidQty));
    }

    #[test]
    fn validate_against_state_rejects_insufficient_balance() {
        let use_case = PlaceOrderUseCase;
        let mut state = sample_state();
        state.account.available_quote = 10;

        let result = use_case.validate_against_state(&sample_cmd(), &state);
        assert_eq!(result, Err(PlaceOrderError::InsufficientQuoteBalance));
    }

    #[test]
    fn compute_replayable_events_produces_order_and_account_events() -> Result<(), PlaceOrderError>
    {
        let use_case = PlaceOrderUseCase;
        let events = use_case.compute_replayable_events(&sample_cmd(), sample_state())?;

        assert_eq!(events.len(), 2);
        assert!(events[0].is_created());
        assert!(events[1].is_updated());
        assert_eq!(field_as_u64(&events[0], "order_sequence"), Some(7));
        assert_eq!(field_as_u64(&events[0], "reserved_quote"), Some(200));
        assert_eq!(field_as_u64(&events[1], "available_quote"), Some(800));
        assert_eq!(field_as_u64(&events[1], "frozen_quote"), Some(200));

        Ok(())
    }
}
