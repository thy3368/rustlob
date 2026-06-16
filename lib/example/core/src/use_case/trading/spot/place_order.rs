mod conditional_order;
mod immediate_order;

pub use conditional_order::{
    PlaceConditionalOrderCmd, PlaceConditionalOrderOutput, PlaceConditionalOrderState,
    PlaceConditionalOrderUseCase,
};
pub use immediate_order::{
    PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceImmediateOrderOutput,
    PlaceImmediateOrderState, PlaceImmediateOrderUseCase,
};
use thiserror::Error;

use crate::MarketRules;
pub use crate::entity::{
    SpotOrderSide as PlaceOrderSide, SpotOrderTimeInForce as PlaceOrderTimeInForce,
    SpotOrderTriggerRole as PlaceOrderTriggerRole,
};

/// 触发后或立即进入执行流程的现货执行意图。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceOrderExecution {
    /// 市价意图。Hyperliquid spot 仍需要价格字段，adapter 可映射为 IOC + 激进限价。
    Market {
        /// 用于冻结上限和 Hyperliquid `p` 字段的激进价格。
        aggressive_price: u64,
    },
    /// 限价意图。
    Limit {
        /// quote 计价价格。
        price: u64,
    },
}

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

fn check_common_command(_side: PlaceOrderSide, quantity: u64) -> Result<(), PlaceOrderError> {
    checked_qty(quantity)?;
    Ok(())
}

fn validate_market_state(
    party_id: &str,
    symbol: &str,
    qty: u64,
    trading_enabled: bool,
    account_id: &str,
    market_rules: &MarketRules,
) -> Result<(), PlaceOrderError> {
    if !trading_enabled {
        return Err(PlaceOrderError::TradingDisabled);
    }

    if account_id != party_id {
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
        PlaceOrderExecution::Market { aggressive_price } => {
            checked_price(aggressive_price).map(Some)
        }
        PlaceOrderExecution::Limit { price } => checked_price(price).map(Some),
    }
}
