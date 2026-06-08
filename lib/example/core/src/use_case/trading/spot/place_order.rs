mod conditional_order;
mod immediate_order;

pub use conditional_order::{
    PlaceConditionalOrderCmd, PlaceConditionalOrderState, PlaceConditionalOrderUseCase,
};
pub use immediate_order::{
    PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceImmediateOrderState,
    PlaceImmediateOrderUseCase,
};
use thiserror::Error;

pub use crate::entity::{
    StoredOrderExecution as PlaceOrderExecution,
    StoredOrderPegOffsetType as PlaceOrderPegOffsetType,
    StoredOrderPegPriceType as PlaceOrderPegPriceType, StoredOrderRespType as PlaceOrderRespType,
    StoredOrderSelfTradePreventionMode as PlaceOrderSelfTradePreventionMode,
    StoredOrderSide as PlaceOrderSide, StoredOrderTimeInForce as PlaceOrderTimeInForce,
    StoredOrderTriggerRole as PlaceOrderTriggerRole,
};
use crate::{MarketRules, TradingAccount};

/// Business errors that can reject a spot order placement.
///
/// These errors are stable enough for adapters to map into CLI, HTTP, or tracing output through
/// [`std::fmt::Display`].
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOrderError {
    /// Returned when this example use case receives an order side it does not handle.
    #[error("unsupported order side")]
    UnsupportedSide,
    /// Returned when spot immediate order asks for reduce-only behavior.
    #[error("reduce_only is not supported for spot immediate orders")]
    UnsupportedReduceOnly,
    /// Returned when `qty == 0`.
    #[error("qty must be greater than zero")]
    InvalidQty,
    /// Returned when `price == 0`.
    #[error("price must be greater than zero")]
    InvalidPrice,
    /// Returned when `trigger_price == 0`.
    #[error("trigger_price must be greater than zero")]
    InvalidTriggerPrice,
    /// Returned when a strategy type violates the external spot API range.
    #[error("strategy_type must be at least 1000000")]
    InvalidStrategyType,
    /// Returned when a peg offset exceeds the external spot API range.
    #[error("peg_offset_value must be between 0 and 100")]
    InvalidPegOffsetValue,
    /// Returned when the quantity is below current market minimum rules.
    #[error("qty is below market minimum")]
    QtyBelowMin,
    /// Returned when the market is temporarily not accepting orders.
    #[error("trading is disabled")]
    TradingDisabled,
    /// Returned when the symbol is not supported by the loaded market rules.
    #[error("symbol is not tradable in current market rules")]
    SymbolNotTradable,
    /// Returned when the command account and loaded account snapshot do not match.
    #[error("account snapshot does not belong to command party")]
    AccountMismatch,
    /// Returned when the account cannot reserve enough quote balance.
    #[error("insufficient quote balance")]
    InsufficientQuoteBalance,
    /// Returned when the account cannot reserve enough base balance.
    #[error("insufficient base balance")]
    InsufficientBaseBalance,
    /// Returned when numeric derivation overflows while computing business results.
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

fn check_common_command(
    side: PlaceOrderSide,
    quantity: u64,
    strategy_type: Option<i32>,
    peg_offset_value: Option<i32>,
) -> Result<(), PlaceOrderError> {
    checked_qty(quantity)?;

    if let Some(strategy_type) = strategy_type {
        if strategy_type < 1_000_000 {
            return Err(PlaceOrderError::InvalidStrategyType);
        }
    }

    if let Some(peg_offset_value) = peg_offset_value {
        if !(0..=100).contains(&peg_offset_value) {
            return Err(PlaceOrderError::InvalidPegOffsetValue);
        }
    }

    Ok(())
}

fn validate_market_state(
    party_id: &str,
    symbol: &str,
    qty: u64,
    trading_enabled: bool,
    account: &TradingAccount,
    market_rules: &MarketRules,
) -> Result<(), PlaceOrderError> {
    if !trading_enabled {
        return Err(PlaceOrderError::TradingDisabled);
    }

    if account.account_id != party_id {
        return Err(PlaceOrderError::AccountMismatch);
    }

    if !market_rules.supports_symbol(symbol) {
        return Err(PlaceOrderError::SymbolNotTradable);
    }

    if !market_rules.validate_qty(qty) {
        return Err(PlaceOrderError::QtyBelowMin);
    }

    Ok(())
}

fn checked_qty(quantity: u64) -> Result<u64, PlaceOrderError> {
    if quantity == 0 {
        return Err(PlaceOrderError::InvalidQty);
    }
    Ok(quantity)
}

fn checked_price(price: u64) -> Result<u64, PlaceOrderError> {
    if price == 0 {
        return Err(PlaceOrderError::InvalidPrice);
    }
    Ok(price)
}

fn limit_execution_price(execution: PlaceOrderExecution) -> Result<Option<u64>, PlaceOrderError> {
    match execution {
        PlaceOrderExecution::Market => Ok(None),
        PlaceOrderExecution::Limit { price } => checked_price(price).map(Some),
    }
}
