use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;
use thiserror::Error;

use crate::MarketRules;
use crate::entity::{
    Balance, UsdsMFuturesOrder, UsdsMFuturesOrderExecution, UsdsMFuturesOrderSide,
    UsdsMFuturesOrderTimeInForce, UsdsMFuturesPositionSide, required_margin,
};

/// USDS-M 合约普通下单可能返回的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceUsdsMFuturesOrderError {
    /// 数量必须大于零。
    #[error("qty must be greater than zero")]
    InvalidQty,
    /// 下单或保证金估算价格必须大于零。
    #[error("price must be greater than zero")]
    InvalidPrice,
    /// 第一版不支持 reduce-only 订单。
    #[error("reduce_only is not supported for USDS-M futures orders")]
    UnsupportedReduceOnly,
    /// 市场当前不接受下单。
    #[error("trading is disabled")]
    TradingDisabled,
    /// command 账户和已加载账户状态不一致。
    #[error("account snapshot does not belong to command party")]
    AccountMismatch,
    /// 当前市场规则不支持该合约。
    #[error("symbol is not tradable in current market rules")]
    SymbolNotTradable,
    /// 下单数量低于市场最小数量。
    #[error("qty is below market minimum")]
    QtyBelowMin,
    /// 保证金余额不是当前账户的 USDT 余额。
    #[error("margin balance must be current account USDT balance")]
    InvalidMarginBalance,
    /// 杠杆必须至少为 1。
    #[error("leverage must be greater than or equal to one")]
    InvalidLeverage,
    /// 可用 USDT 不足以冻结开仓保证金。
    #[error("insufficient USDT margin balance")]
    InsufficientMarginBalance,
    /// 推导订单或余额事件时发生算术溢出。
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

/// 创建 USDS-M 合约普通订单需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceUsdsMFuturesOrderState {
    /// 当前市场是否接受下单。
    pub trading_enabled: bool,
    /// 用于生成稳定订单 ID 的下一个订单序号。
    pub next_order_sequence: u64,
    /// 下单账户 ID。
    pub account_id: String,
    /// USDT 保证金余额快照。
    pub margin_balance: Balance,
    /// 当前合约市场规则快照。
    pub market_rules: MarketRules,
    /// 当前账户在该合约上的杠杆；来自独立杠杆设置 endpoint。
    pub leverage: u64,
}

/// USDS-M 合约普通订单的执行意图。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceUsdsMFuturesOrderExecution {
    /// Binance `LIMIT` 订单。
    Limit {
        /// 委托价格。
        price: u64,
        /// Binance `timeInForce`。
        time_in_force: UsdsMFuturesOrderTimeInForce,
    },
    /// Binance `MARKET` 订单。
    Market {
        /// 用于计算保证金冻结上限的激进价格。
        aggressive_price: u64,
    },
}

impl PlaceUsdsMFuturesOrderExecution {
    fn margin_price(self) -> Result<u64, PlaceUsdsMFuturesOrderError> {
        let price = match self {
            Self::Limit { price, .. } => price,
            Self::Market { aggressive_price } => aggressive_price,
        };
        if price == 0 {
            return Err(PlaceUsdsMFuturesOrderError::InvalidPrice);
        }
        Ok(price)
    }

    const fn stored_execution(self) -> UsdsMFuturesOrderExecution {
        match self {
            Self::Limit { price, .. } => UsdsMFuturesOrderExecution::Limit { price },
            Self::Market { aggressive_price } => {
                UsdsMFuturesOrderExecution::Market { aggressive_price }
            }
        }
    }

    const fn stored_time_in_force(self) -> UsdsMFuturesOrderTimeInForce {
        match self {
            Self::Limit { time_in_force, .. } => time_in_force,
            Self::Market { .. } => UsdsMFuturesOrderTimeInForce::Ioc,
        }
    }
}

/// 创建 USDS-M 合约普通订单的命令。
///
/// 字段对齐 Binance `POST /fapi/v1/order` 第一版核心字段：`symbol`、`side`、
/// `type`、`timeInForce`、`quantity`、`price`、`reduceOnly`、`newClientOrderId`。
/// `party_id` 是 core 的业务发起方；杠杆由状态快照提供，不属于下单命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceUsdsMFuturesOrderCmd {
    /// 发起下单的交易账户 ID。
    pub party_id: String,
    /// 合约交易对，例如 `BTCUSDT`。
    pub symbol: String,
    /// 买卖方向。
    pub side: UsdsMFuturesOrderSide,
    /// 合约数量。
    pub qty: u64,
    /// 是否只减仓；第一版必须为 `false`。
    pub reduce_only: bool,
    /// 执行意图。
    pub execution: PlaceUsdsMFuturesOrderExecution,
    /// 客户端自定义订单 ID。
    pub client_order_id: Option<String>,
}

impl PlaceUsdsMFuturesOrderCmd {
    fn checked_qty(&self) -> Result<u64, PlaceUsdsMFuturesOrderError> {
        if self.qty == 0 {
            return Err(PlaceUsdsMFuturesOrderError::InvalidQty);
        }
        Ok(self.qty)
    }
}

impl IssuedByParty for PlaceUsdsMFuturesOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 接受 USDS-M 合约普通订单并冻结开仓保证金的 use case。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceUsdsMFuturesOrderUseCase;

impl CommandUseCase2 for PlaceUsdsMFuturesOrderUseCase {
    type Command = PlaceUsdsMFuturesOrderCmd;
    type GivenState = PlaceUsdsMFuturesOrderState;
    type Error = PlaceUsdsMFuturesOrderError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        let _ = cmd.checked_qty()?;
        if cmd.reduce_only {
            return Err(PlaceUsdsMFuturesOrderError::UnsupportedReduceOnly);
        }
        let _ = cmd.execution.margin_price()?;
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let qty = cmd.checked_qty()?;
        let price = cmd.execution.margin_price()?;

        if !state.trading_enabled {
            return Err(PlaceUsdsMFuturesOrderError::TradingDisabled);
        }
        if state.account_id != cmd.party_id {
            return Err(PlaceUsdsMFuturesOrderError::AccountMismatch);
        }
        if !state.market_rules.supports_symbol(cmd.symbol.as_str()) {
            return Err(PlaceUsdsMFuturesOrderError::SymbolNotTradable);
        }
        if !state.market_rules.validate_qty(qty) {
            return Err(PlaceUsdsMFuturesOrderError::QtyBelowMin);
        }
        if state.leverage == 0 {
            return Err(PlaceUsdsMFuturesOrderError::InvalidLeverage);
        }
        if !state.margin_balance.belongs_to_account(state.account_id.as_str())
            || !state.margin_balance.is_asset("USDT")
        {
            return Err(PlaceUsdsMFuturesOrderError::InvalidMarginBalance);
        }

        let margin = required_margin(qty, price, state.leverage)
            .ok_or(PlaceUsdsMFuturesOrderError::ArithmeticOverflow)?;
        if !state.margin_balance.can_reserve(margin) {
            return Err(PlaceUsdsMFuturesOrderError::InsufficientMarginBalance);
        }

        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let qty = cmd.checked_qty()?;
        let price = cmd.execution.margin_price()?;
        let margin = required_margin(qty, price, state.leverage)
            .ok_or(PlaceUsdsMFuturesOrderError::ArithmeticOverflow)?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);
        let order = UsdsMFuturesOrder::new(
            order_id,
            state.account_id.clone(),
            cmd.symbol.clone(),
            cmd.side,
            UsdsMFuturesPositionSide::Both,
            cmd.execution.stored_execution(),
            cmd.execution.stored_time_in_force(),
            qty,
            price,
            margin,
            cmd.reduce_only,
            cmd.client_order_id.clone(),
        );
        let order_event = order
            .track_create_event()
            .map_err(|_| PlaceUsdsMFuturesOrderError::ArithmeticOverflow)?;

        let mut next_balance = state.margin_balance.clone();
        let (next_available, next_frozen) = state
            .margin_balance
            .reserve_after(margin)
            .ok_or(PlaceUsdsMFuturesOrderError::ArithmeticOverflow)?;
        let next_version = state
            .margin_balance
            .version
            .checked_add(1)
            .ok_or(PlaceUsdsMFuturesOrderError::ArithmeticOverflow)?;
        let balance_event = next_balance
            .track_update_event(|balance| {
                balance.apply_after(next_available, next_frozen, next_version);
            })
            .map_err(|_| PlaceUsdsMFuturesOrderError::ArithmeticOverflow)?;

        Ok(vec![order_event, balance_event])
    }
}

#[cfg(test)]
mod tests {
    use common_entity::EntityChangeType;
    use proptest::prelude::*;

    use super::*;

    fn use_case() -> PlaceUsdsMFuturesOrderUseCase {
        PlaceUsdsMFuturesOrderUseCase
    }

    fn limit_cmd() -> PlaceUsdsMFuturesOrderCmd {
        PlaceUsdsMFuturesOrderCmd {
            party_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: UsdsMFuturesOrderSide::Buy,
            qty: 3,
            reduce_only: false,
            execution: PlaceUsdsMFuturesOrderExecution::Limit {
                price: 101,
                time_in_force: UsdsMFuturesOrderTimeInForce::Gtc,
            },
            client_order_id: Some("client-1".to_string()),
        }
    }

    fn market_cmd() -> PlaceUsdsMFuturesOrderCmd {
        PlaceUsdsMFuturesOrderCmd {
            execution: PlaceUsdsMFuturesOrderExecution::Market { aggressive_price: 111 },
            ..limit_cmd()
        }
    }

    fn state() -> PlaceUsdsMFuturesOrderState {
        PlaceUsdsMFuturesOrderState {
            trading_enabled: true,
            next_order_sequence: 7,
            account_id: "trader-1".to_string(),
            margin_balance: Balance::new(
                "trader-1".to_string(),
                "USDT".to_string(),
                10_000,
                500,
                4,
            ),
            market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
            leverage: 5,
        }
    }

    fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() == Some(field_name) {
                std::str::from_utf8(change.new_value_bytes()).ok()
            } else {
                None
            }
        })
    }

    fn event_field_u64(event: &EntityReplayableEvent, field_name: &str) -> Option<u64> {
        event_field(event, field_name)?.parse().ok()
    }

    #[test]
    fn role_is_trader() {
        assert_eq!(use_case().role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_malformed_command_fields() {
        let mut cmd = limit_cmd();
        cmd.qty = 0;
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceUsdsMFuturesOrderError::InvalidQty)
        );

        let mut cmd = limit_cmd();
        cmd.execution = PlaceUsdsMFuturesOrderExecution::Limit {
            price: 0,
            time_in_force: UsdsMFuturesOrderTimeInForce::Gtc,
        };
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceUsdsMFuturesOrderError::InvalidPrice)
        );

        let mut cmd = market_cmd();
        cmd.execution = PlaceUsdsMFuturesOrderExecution::Market { aggressive_price: 0 };
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceUsdsMFuturesOrderError::InvalidPrice)
        );

        let mut cmd = limit_cmd();
        cmd.reduce_only = true;
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceUsdsMFuturesOrderError::UnsupportedReduceOnly)
        );
    }

    #[test]
    fn validate_rejects_invalid_loaded_state() {
        let cmd = limit_cmd();

        let mut disabled = state();
        disabled.trading_enabled = false;
        assert_eq!(
            use_case().validate_against_state(&cmd, &disabled),
            Err(PlaceUsdsMFuturesOrderError::TradingDisabled)
        );

        let mut wrong_account = state();
        wrong_account.account_id = "trader-2".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_account),
            Err(PlaceUsdsMFuturesOrderError::AccountMismatch)
        );

        let mut wrong_symbol = state();
        wrong_symbol.market_rules.symbol = "ETHUSDT".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_symbol),
            Err(PlaceUsdsMFuturesOrderError::SymbolNotTradable)
        );

        let mut below_min = state();
        below_min.market_rules.min_qty = 4;
        assert_eq!(
            use_case().validate_against_state(&cmd, &below_min),
            Err(PlaceUsdsMFuturesOrderError::QtyBelowMin)
        );

        let mut zero_leverage = state();
        zero_leverage.leverage = 0;
        assert_eq!(
            use_case().validate_against_state(&cmd, &zero_leverage),
            Err(PlaceUsdsMFuturesOrderError::InvalidLeverage)
        );

        let mut wrong_asset = state();
        wrong_asset.margin_balance.asset_id = "USDC".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_asset),
            Err(PlaceUsdsMFuturesOrderError::InvalidMarginBalance)
        );

        let mut insufficient = state();
        insufficient.margin_balance.available = 60;
        assert_eq!(
            use_case().validate_against_state(&cmd, &insufficient),
            Err(PlaceUsdsMFuturesOrderError::InsufficientMarginBalance)
        );
    }

    #[test]
    fn compute_limit_order_creates_order_and_freezes_margin() {
        let events = use_case().compute_replayable_events(&limit_cmd(), state()).unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].change_type, EntityChangeType::Created.as_tag());
        assert_eq!(event_field(&events[0], "order_id"), Some("trader-1-BTCUSDT-7"));
        assert_eq!(event_field(&events[0], "position_side"), Some("both"));
        assert_eq!(event_field(&events[0], "execution"), Some("limit"));
        assert_eq!(event_field(&events[0], "time_in_force"), Some("gtc"));
        assert_eq!(event_field_u64(&events[0], "qty"), Some(3));
        assert_eq!(event_field_u64(&events[0], "price"), Some(101));
        assert_eq!(event_field_u64(&events[0], "required_margin"), Some(61));

        assert_eq!(events[1].change_type, EntityChangeType::Updated.as_tag());
        assert_eq!(event_field_u64(&events[1], "available"), Some(9_939));
        assert_eq!(event_field_u64(&events[1], "frozen"), Some(561));
        assert_eq!(events[1].old_version, 4);
        assert_eq!(events[1].new_version, 5);
    }

    #[test]
    fn compute_market_order_uses_aggressive_price_for_margin() {
        let events = use_case().compute_replayable_events(&market_cmd(), state()).unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(event_field(&events[0], "execution"), Some("market"));
        assert_eq!(event_field(&events[0], "time_in_force"), Some("ioc"));
        assert_eq!(event_field_u64(&events[0], "price"), Some(111));
        assert_eq!(event_field_u64(&events[0], "required_margin"), Some(67));
        assert_eq!(event_field_u64(&events[1], "available"), Some(9_933));
        assert_eq!(event_field_u64(&events[1], "frozen"), Some(567));
    }

    proptest! {
        #[test]
        fn required_margin_is_ceiled_notional_divided_by_leverage(
            qty in 1_u64..1_000_000,
            price in 1_u64..1_000_000,
            leverage in 1_u64..125,
        ) {
            let margin = required_margin(qty, price, leverage).unwrap();
            let notional = qty * price;
            prop_assert_eq!(margin, notional.div_ceil(leverage));
        }

        #[test]
        fn freezing_margin_preserves_total_usdt_balance(
            qty in 1_u64..100_000,
            price in 1_u64..100_000,
            leverage in 1_u64..125,
            existing_frozen in 0_u64..1_000_000,
        ) {
            let margin = required_margin(qty, price, leverage).unwrap();
            let cmd = PlaceUsdsMFuturesOrderCmd {
                qty,
                execution: PlaceUsdsMFuturesOrderExecution::Limit {
                    price,
                    time_in_force: UsdsMFuturesOrderTimeInForce::Gtc,
                },
                ..limit_cmd()
            };
            let state = PlaceUsdsMFuturesOrderState {
                margin_balance: Balance::new(
                    "trader-1".to_string(),
                    "USDT".to_string(),
                    margin,
                    existing_frozen,
                    1,
                ),
                leverage,
                ..state()
            };

            let events = use_case().compute_replayable_events(&cmd, state).unwrap();
            let next_available = event_field_u64(&events[1], "available").unwrap();
            let next_frozen = event_field_u64(&events[1], "frozen").unwrap();

            prop_assert_eq!(next_available, 0);
            prop_assert_eq!(next_frozen, existing_frozen + margin);
            prop_assert_eq!(next_available + next_frozen, existing_frozen + margin);
        }

        #[test]
        fn created_order_event_matches_command_and_state(
            qty in 1_u64..100_000,
            price in 1_u64..100_000,
            leverage in 1_u64..125,
            is_buy in any::<bool>(),
        ) {
            let side = if is_buy {
                UsdsMFuturesOrderSide::Buy
            } else {
                UsdsMFuturesOrderSide::Sell
            };
            let margin = required_margin(qty, price, leverage).unwrap();
            let cmd = PlaceUsdsMFuturesOrderCmd {
                side,
                qty,
                execution: PlaceUsdsMFuturesOrderExecution::Limit {
                    price,
                    time_in_force: UsdsMFuturesOrderTimeInForce::Fok,
                },
                ..limit_cmd()
            };
            let state = PlaceUsdsMFuturesOrderState {
                next_order_sequence: 99,
                margin_balance: Balance::new(
                    "trader-1".to_string(),
                    "USDT".to_string(),
                    margin,
                    0,
                    1,
                ),
                leverage,
                ..state()
            };

            let events = use_case().compute_replayable_events(&cmd, state).unwrap();

            prop_assert_eq!(event_field(&events[0], "order_id"), Some("trader-1-BTCUSDT-99"));
            prop_assert_eq!(event_field(&events[0], "account_id"), Some("trader-1"));
            prop_assert_eq!(event_field(&events[0], "symbol"), Some("BTCUSDT"));
            prop_assert_eq!(event_field(&events[0], "side"), Some(side.as_str()));
            prop_assert_eq!(event_field(&events[0], "position_side"), Some("both"));
            prop_assert_eq!(event_field(&events[0], "execution"), Some("limit"));
            prop_assert_eq!(event_field(&events[0], "time_in_force"), Some("fok"));
            prop_assert_eq!(event_field_u64(&events[0], "qty"), Some(qty));
            prop_assert_eq!(event_field_u64(&events[0], "price"), Some(price));
            prop_assert_eq!(event_field_u64(&events[0], "required_margin"), Some(margin));
            prop_assert_eq!(event_field(&events[0], "reduce_only"), Some("false"));
        }
    }
}
