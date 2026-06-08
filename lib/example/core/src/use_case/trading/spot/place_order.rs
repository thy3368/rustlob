use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::StoredOrder;
pub use crate::entity::{
    StoredConditionalOrderSpec as PlaceConditionalOrderSpec,
    StoredImmediateOrderSpec as PlaceImmediateOrderSpec,
    StoredOrderExecution as PlaceOrderExecution, StoredOrderKind as PlaceOrderKind,
    StoredOrderPegOffsetType as PlaceOrderPegOffsetType,
    StoredOrderPegPriceType as PlaceOrderPegPriceType, StoredOrderRespType as PlaceOrderRespType,
    StoredOrderSelfTradePreventionMode as PlaceOrderSelfTradePreventionMode,
    StoredOrderSide as PlaceOrderSide, StoredOrderTimeInForce as PlaceOrderTimeInForce,
    StoredOrderTriggerRole as PlaceOrderTriggerRole,
};
use crate::{MarketRules, TradingAccount};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderState {
    pub trading_enabled: bool,
    pub next_order_sequence: u64,
    pub account: TradingAccount,
    pub market_rules: MarketRules,
}

/// 创建现货订单的命令。
///
/// 这个命令只承载调用方输入。命令自身的廉价校验放在
/// [`PlaceOrderUseCase::pre_check_command`]；需要市场规则、账户余额等已加载状态的校验放在
/// [`PlaceOrderUseCase::validate_against_state`]。
///
/// # Examples
///
/// ```
/// use example_core::{
///     PlaceImmediateOrderSpec, PlaceOrderCmd, PlaceOrderExecution, PlaceOrderKind,
///     PlaceOrderSide, PlaceOrderTimeInForce,
/// };
///
/// let cmd = PlaceOrderCmd {
///     party_id: "trader-1".to_string(),
///     symbol: "BTCUSDT".to_string(),
///     side: PlaceOrderSide::Buy,
///     quantity: 2,
///     kind: PlaceOrderKind::Immediate(PlaceImmediateOrderSpec {
///         execution: PlaceOrderExecution::Limit { price: 100 },
///         time_in_force: PlaceOrderTimeInForce::Gtc,
///     }),
///     client_order_id: None,
///     strategy_id: None,
///     strategy_type: None,
///     iceberg_qty: None,
///     new_order_resp_type: None,
///     self_trade_prevention_mode: None,
///     peg_price_type: None,
///     peg_offset_value: None,
///     peg_offset_type: None,
/// };
///
/// assert_eq!(cmd.symbol, "BTCUSDT");
/// assert_eq!(cmd.quantity, 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCmd {
    /// 发起下单的交易账户 ID，也是后续冻结 quote 余额的账户。
    pub party_id: String,
    /// 交易对，例如 `BTCUSDT`。
    pub symbol: String,
    /// 订单方向。当前示例用例只处理买单，并为买单冻结 quote 余额。
    pub side: PlaceOrderSide,
    /// 以 base asset 计价的下单数量，例如买入多少 BTC。
    pub quantity: u64,
    /// Hyperliquid 风格的执行时机和执行方式组合。
    pub kind: PlaceOrderKind,
    /// 客户端自定义订单 ID，可由 adapter 映射为 Hyperliquid `cloid`。
    pub client_order_id: Option<String>,
    /// 客户端附带的策略 ID。
    pub strategy_id: Option<i64>,
    /// 客户端附带的策略类型。参考 Binance 规则，小于 1_000_000 的值保留不用。
    pub strategy_type: Option<i32>,
    /// 冰山订单的可见数量。
    pub iceberg_qty: Option<u64>,
    /// 期望的下单响应类型。
    pub new_order_resp_type: Option<PlaceOrderRespType>,
    /// 自成交保护模式。
    pub self_trade_prevention_mode: Option<PlaceOrderSelfTradePreventionMode>,
    /// 价格钉住类型。
    pub peg_price_type: Option<PlaceOrderPegPriceType>,
    /// 价格钉住偏移值。
    pub peg_offset_value: Option<i32>,
    /// 价格钉住偏移单位。
    pub peg_offset_type: Option<PlaceOrderPegOffsetType>,
}

impl PlaceOrderCmd {
    fn limit_qty(&self) -> Result<u64, PlaceOrderError> {
        if self.quantity == 0 {
            return Err(PlaceOrderError::InvalidQty);
        }
        Ok(self.quantity)
    }

    fn limit_price(&self) -> Result<u64, PlaceOrderError> {
        let price = match self.kind {
            PlaceOrderKind::Immediate(PlaceImmediateOrderSpec {
                execution: PlaceOrderExecution::Limit { price },
                ..
            }) => price,
            _ => return Err(PlaceOrderError::UnsupportedOrderKind),
        };

        if price == 0 {
            return Err(PlaceOrderError::InvalidPrice);
        }
        Ok(price)
    }
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
    /// Returned when this example use case receives an order side it does not handle.
    #[error("only buy orders are supported by this place-order use case")]
    UnsupportedSide,
    /// Returned when this example use case receives an order kind it does not handle.
    #[error("only immediate limit orders are supported by this place-order use case")]
    UnsupportedOrderKind,
    /// Returned when `qty == 0`.
    #[error("qty must be greater than zero")]
    InvalidQty,
    /// Returned when `price == 0`.
    #[error("price must be greater than zero")]
    InvalidPrice,
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
        if cmd.side != PlaceOrderSide::Buy {
            return Err(PlaceOrderError::UnsupportedSide);
        }

        let _ = cmd.limit_price()?;

        if let Some(strategy_type) = cmd.strategy_type {
            if strategy_type < 1_000_000 {
                return Err(PlaceOrderError::InvalidStrategyType);
            }
        }

        if let Some(peg_offset_value) = cmd.peg_offset_value {
            if !(0..=100).contains(&peg_offset_value) {
                return Err(PlaceOrderError::InvalidPegOffsetValue);
            }
        }

        cmd.limit_qty()?;

        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let qty = cmd.limit_qty()?;
        let price = cmd.limit_price()?;

        if !state.trading_enabled {
            return Err(PlaceOrderError::TradingDisabled);
        }

        if !state.market_rules.supports_symbol(cmd.symbol.as_str()) {
            return Err(PlaceOrderError::SymbolNotTradable);
        }

        if !state.market_rules.validate_qty(qty) {
            return Err(PlaceOrderError::QtyBelowMin);
        }

        let reserved_quote = state
            .market_rules
            .required_quote(qty, price)
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
        let qty = cmd.limit_qty()?;
        let price = cmd.limit_price()?;
        let reserved_quote = state
            .market_rules
            .required_quote(qty, price)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let (next_available, next_frozen) = state
            .account
            .reserve_quote_after(reserved_quote)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let next_version =
            state.account.version.checked_add(1).ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);

        let order = StoredOrder::new(
            order_id,
            cmd.party_id.clone(),
            cmd.symbol.clone(),
            cmd.side,
            cmd.kind,
            qty,
            reserved_quote,
            cmd.client_order_id.clone(),
            cmd.strategy_id,
            cmd.strategy_type,
            cmd.iceberg_qty,
            cmd.new_order_resp_type,
            cmd.self_trade_prevention_mode,
            cmd.peg_price_type,
            cmd.peg_offset_value,
            cmd.peg_offset_type,
        );
        let order_event =
            order.track_create_event().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

        let mut next_account = state.account.clone();
        let tracked_account_event = next_account
            .track_update_event(|account| {
                account.apply_reserved_quote_after(next_available, next_frozen, next_version);
            })
            .map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

        Ok(vec![order_event, tracked_account_event])
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
            side: PlaceOrderSide::Buy,
            quantity: 2,
            kind: PlaceOrderKind::Immediate(PlaceImmediateOrderSpec {
                execution: PlaceOrderExecution::Limit { price: 100 },
                time_in_force: PlaceOrderTimeInForce::Gtc,
            }),
            client_order_id: None,
            strategy_id: None,
            strategy_type: None,
            iceberg_qty: None,
            new_order_resp_type: None,
            self_trade_prevention_mode: None,
            peg_price_type: None,
            peg_offset_value: None,
            peg_offset_type: None,
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
        cmd.quantity = 0;

        let result = use_case.pre_check_command(&cmd);
        assert_eq!(result, Err(PlaceOrderError::InvalidQty));
    }

    #[test]
    fn pre_check_rejects_conditional_order_until_trigger_flow_is_supported() {
        let use_case = PlaceOrderUseCase;
        let mut cmd = sample_cmd();
        cmd.kind = PlaceOrderKind::Conditional(PlaceConditionalOrderSpec {
            trigger_price: 90,
            trigger_role: PlaceOrderTriggerRole::StopLoss,
            execution: PlaceOrderExecution::Market,
        });

        let result = use_case.pre_check_command(&cmd);
        assert_eq!(result, Err(PlaceOrderError::UnsupportedOrderKind));
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
