use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase3, IssuedByParty, UseCaseOutput};
use common_entity::Entity;
use thiserror::Error;

use crate::MarketRules;
use crate::entity::{
    Balance, HyperliquidPerpOrder, HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide,
    HyperliquidPerpOrderTimeInForce, HyperliquidPerpPosition, HyperliquidPerpPositionSide,
    required_position_margin,
};

/// Hyperliquid perp 下单可能返回的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceHyperliquidPerpOrderError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 合约展示名不能为空。
    #[error("symbol must not be empty")]
    InvalidSymbol,
    /// 数量必须大于零。
    #[error("size must be greater than zero")]
    InvalidSize,
    /// 下单或保证金估算价格必须大于零。
    #[error("price must be greater than zero")]
    InvalidPrice,
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
    #[error("size is below market minimum")]
    SizeBelowMin,
    /// 保证金余额不是当前账户的 Cross 保证金币种余额。
    #[error("margin balance must be current account cross margin balance")]
    InvalidMarginBalance,
    /// 仓位账户、asset 或 symbol 与命令不一致。
    #[error("position does not match command account or market")]
    PositionMismatch,
    /// 仓位状态、数量、均价或保证金不一致。
    #[error("position state is inconsistent")]
    InconsistentPositionState,
    /// 仓位杠杆必须至少为 1。
    #[error("position leverage must be greater than or equal to one")]
    InvalidLeverage,
    /// 可用 Cross 保证金不足以冻结开仓保证金。
    #[error("insufficient cross margin balance")]
    InsufficientMarginBalance,
    /// reduce-only 订单不能增加或反向超过当前净仓位。
    #[error("invalid reduce-only order")]
    InvalidReduceOnly,
    /// 推导订单或余额事件时发生算术溢出。
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

/// 创建 Hyperliquid perp 订单的执行意图。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceHyperliquidPerpOrderExecution {
    /// Hyperliquid 限价订单。
    Limit {
        /// 委托价格。
        price: u64,
        /// Hyperliquid `tif`。
        time_in_force: HyperliquidPerpOrderTimeInForce,
    },
    /// 市价意图，adapter 可映射为 IOC + 激进限价。
    Market {
        /// 用于计算保证金冻结上限和撮合价格比较的激进价格。
        aggressive_price: u64,
    },
}

impl PlaceHyperliquidPerpOrderExecution {
    fn margin_price(self) -> Result<u64, PlaceHyperliquidPerpOrderError> {
        let price = match self {
            Self::Limit { price, .. } => price,
            Self::Market { aggressive_price } => aggressive_price,
        };
        if price == 0 {
            return Err(PlaceHyperliquidPerpOrderError::InvalidPrice);
        }
        Ok(price)
    }

    const fn stored_execution(self) -> HyperliquidPerpOrderExecution {
        match self {
            Self::Limit { price, .. } => HyperliquidPerpOrderExecution::Limit { price },
            Self::Market { aggressive_price } => {
                HyperliquidPerpOrderExecution::Market { aggressive_price }
            }
        }
    }

    const fn stored_time_in_force(self) -> HyperliquidPerpOrderTimeInForce {
        match self {
            Self::Limit { time_in_force, .. } => time_in_force,
            Self::Market { .. } => HyperliquidPerpOrderTimeInForce::Ioc,
        }
    }
}

/// 创建 Hyperliquid perp 订单的命令。
///
/// 字段对齐 Hyperliquid order API 第一版核心语义：`asset`、`isBuy`、`sz`、
/// `reduceOnly`、`limitPx` / aggressive price、`tif` 和 `cloid`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderCmd {
    /// 发起下单的交易账户 ID。
    pub party_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// `true` 表示买入，`false` 表示卖出。
    pub is_buy: bool,
    /// 合约数量。
    pub size: u64,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// 执行意图。
    pub execution: PlaceHyperliquidPerpOrderExecution,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub cloid: Option<String>,
}

impl PlaceHyperliquidPerpOrderCmd {
    fn checked_size(&self) -> Result<u64, PlaceHyperliquidPerpOrderError> {
        if self.size == 0 {
            return Err(PlaceHyperliquidPerpOrderError::InvalidSize);
        }
        Ok(self.size)
    }

    const fn side(&self) -> HyperliquidPerpOrderSide {
        if self.is_buy { HyperliquidPerpOrderSide::Buy } else { HyperliquidPerpOrderSide::Sell }
    }
}

impl IssuedByParty for PlaceHyperliquidPerpOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 创建 Hyperliquid perp 订单需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderState {
    /// 当前市场是否接受下单。
    pub trading_enabled: bool,
    /// 用于生成稳定订单 ID 的下一个订单序号。
    pub next_order_sequence: u64,
    /// 下单账户 ID。
    pub account_id: String,
    /// Cross 保证金余额快照。
    pub margin_balance: Balance,
    /// Cross 保证金币种，例如 `USDC`。
    pub margin_asset_id: String,
    /// 当前合约市场规则快照。
    pub market_rules: MarketRules,
    /// 当前账户在该 Hyperliquid perp asset 上的净仓位快照。
    pub position: HyperliquidPerpPosition,
}

/// 接受 Hyperliquid perp 订单，并在非 reduce-only 时冻结 Cross 保证金。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceHyperliquidPerpOrderUseCase;

/// 创建 Hyperliquid perp 订单后的业务产出。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderOutput {
    /// 下单后订单快照。
    pub order_after: HyperliquidPerpOrder,
    /// 本次实际受影响的保证金余额 after 快照；reduce-only 路径为空。
    pub balances_after: Vec<Balance>,
}

impl CommandUseCase3 for PlaceHyperliquidPerpOrderUseCase {
    type Command = PlaceHyperliquidPerpOrderCmd;
    type GivenState = PlaceHyperliquidPerpOrderState;
    type Error = PlaceHyperliquidPerpOrderError;
    type Output = PlaceHyperliquidPerpOrderOutput;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(PlaceHyperliquidPerpOrderError::InvalidPartyId);
        }
        if cmd.symbol.is_empty() {
            return Err(PlaceHyperliquidPerpOrderError::InvalidSymbol);
        }
        let _ = cmd.checked_size()?;
        let _ = cmd.execution.margin_price()?;
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let size = cmd.checked_size()?;
        let price = cmd.execution.margin_price()?;

        if !state.trading_enabled {
            return Err(PlaceHyperliquidPerpOrderError::TradingDisabled);
        }
        if state.account_id != cmd.party_id {
            return Err(PlaceHyperliquidPerpOrderError::AccountMismatch);
        }
        if !state.market_rules.supports_symbol(cmd.symbol.as_str()) {
            return Err(PlaceHyperliquidPerpOrderError::SymbolNotTradable);
        }
        if !state.market_rules.validate_qty(size) {
            return Err(PlaceHyperliquidPerpOrderError::SizeBelowMin);
        }
        if !state.margin_balance.belongs_to_account(state.account_id.as_str())
            || !state.margin_balance.is_asset(state.margin_asset_id.as_str())
        {
            return Err(PlaceHyperliquidPerpOrderError::InvalidMarginBalance);
        }
        if !state.position.belongs_to_account(state.account_id.as_str())
            || !state.position.trades_asset(cmd.asset)
            || !state.position.trades_symbol(cmd.symbol.as_str())
        {
            return Err(PlaceHyperliquidPerpOrderError::PositionMismatch);
        }
        if !state.position.has_consistent_state() {
            return Err(PlaceHyperliquidPerpOrderError::InconsistentPositionState);
        }
        if state.position.leverage == 0 {
            return Err(PlaceHyperliquidPerpOrderError::InvalidLeverage);
        }
        if !state.position.is_flat()
            && state.position.required_margin() != Some(state.position.margin)
        {
            return Err(PlaceHyperliquidPerpOrderError::InconsistentPositionState);
        }
        if cmd.reduce_only {
            validate_reduce_only(cmd, &state.position)?;
            return Ok(());
        }

        let margin = required_new_order_margin(cmd.side(), size, price, &state.position)?;
        if !state.margin_balance.can_reserve(margin) {
            return Err(PlaceHyperliquidPerpOrderError::InsufficientMarginBalance);
        }

        Ok(())
    }

    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        let size = cmd.checked_size()?;
        let price = cmd.execution.margin_price()?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);
        let order_after = HyperliquidPerpOrder::new(
            order_id,
            None,
            cmd.asset,
            state.account_id.clone(),
            cmd.symbol.clone(),
            cmd.side(),
            cmd.execution.stored_execution(),
            cmd.execution.stored_time_in_force(),
            size,
            cmd.reduce_only,
            cmd.cloid.clone(),
        );
        let order_event = order_after
            .track_create_event()
            .map_err(|_| PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;

        if cmd.reduce_only {
            return Ok(UseCaseOutput {
                output: PlaceHyperliquidPerpOrderOutput { order_after, balances_after: Vec::new() },
                events: vec![order_event],
            });
        }

        let margin = required_new_order_margin(cmd.side(), size, price, &state.position)?;
        if margin == 0 {
            return Ok(UseCaseOutput {
                output: PlaceHyperliquidPerpOrderOutput { order_after, balances_after: Vec::new() },
                events: vec![order_event],
            });
        }
        let mut next_balance = state.margin_balance.clone();
        let previous_balance = next_balance.clone();
        let (next_available, next_frozen) = state
            .margin_balance
            .reserve_after(margin)
            .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;
        let next_version = state
            .margin_balance
            .version
            .checked_add(1)
            .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;
        next_balance.apply_after(next_available, next_frozen, next_version);
        let balance_event = next_balance
            .track_update_event_from(&previous_balance)
            .map_err(|_| PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;

        Ok(UseCaseOutput {
            output: PlaceHyperliquidPerpOrderOutput {
                order_after,
                balances_after: vec![next_balance],
            },
            events: vec![order_event, balance_event],
        })
    }
}

fn required_new_order_margin(
    order_side: HyperliquidPerpOrderSide,
    size: u64,
    price: u64,
    position: &HyperliquidPerpPosition,
) -> Result<u64, PlaceHyperliquidPerpOrderError> {
    let net_new_qty = match (order_side, position.side) {
        (_, HyperliquidPerpPositionSide::Flat) => size,
        (HyperliquidPerpOrderSide::Buy, HyperliquidPerpPositionSide::Long)
        | (HyperliquidPerpOrderSide::Sell, HyperliquidPerpPositionSide::Short) => size,
        (HyperliquidPerpOrderSide::Buy, HyperliquidPerpPositionSide::Short)
        | (HyperliquidPerpOrderSide::Sell, HyperliquidPerpPositionSide::Long) => {
            size.saturating_sub(position.qty)
        }
    };
    if net_new_qty == 0 {
        return Ok(0);
    }
    required_position_margin(net_new_qty, price, position.leverage)
        .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)
}

fn validate_reduce_only(
    cmd: &PlaceHyperliquidPerpOrderCmd,
    position: &HyperliquidPerpPosition,
) -> Result<(), PlaceHyperliquidPerpOrderError> {
    let reduces_position = matches!(
        (cmd.side(), position.side),
        (HyperliquidPerpOrderSide::Buy, HyperliquidPerpPositionSide::Short)
            | (HyperliquidPerpOrderSide::Sell, HyperliquidPerpPositionSide::Long)
    );
    if !reduces_position || cmd.size > position.qty {
        return Err(PlaceHyperliquidPerpOrderError::InvalidReduceOnly);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::CommandUseCase3;
    use common_entity::EntityChangeType;
    use proptest::prelude::*;

    use super::*;

    fn use_case() -> PlaceHyperliquidPerpOrderUseCase {
        PlaceHyperliquidPerpOrderUseCase
    }

    fn limit_cmd() -> PlaceHyperliquidPerpOrderCmd {
        PlaceHyperliquidPerpOrderCmd {
            party_id: "trader-1".to_string(),
            asset: 0,
            symbol: "BTC-PERP".to_string(),
            is_buy: true,
            size: 3,
            reduce_only: false,
            execution: PlaceHyperliquidPerpOrderExecution::Limit {
                price: 101,
                time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
            },
            cloid: Some("client-1".to_string()),
        }
    }

    fn market_cmd() -> PlaceHyperliquidPerpOrderCmd {
        PlaceHyperliquidPerpOrderCmd {
            execution: PlaceHyperliquidPerpOrderExecution::Market { aggressive_price: 111 },
            ..limit_cmd()
        }
    }

    fn position() -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::empty_slot(
            "trader-1:0:BTC-PERP".to_string(),
            "trader-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            5,
        )
    }

    fn state() -> PlaceHyperliquidPerpOrderState {
        PlaceHyperliquidPerpOrderState {
            trading_enabled: true,
            next_order_sequence: 7,
            account_id: "trader-1".to_string(),
            margin_balance: Balance::new(
                "trader-1".to_string(),
                "USDC".to_string(),
                10_000,
                500,
                4,
            ),
            margin_asset_id: "USDC".to_string(),
            market_rules: MarketRules { symbol: "BTC-PERP".to_string(), min_qty: 1 },
            position: position(),
        }
    }

    fn non_flat_position(side: HyperliquidPerpPositionSide, qty: u64) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            "trader-1:0:BTC-PERP".to_string(),
            "trader-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            side,
            qty,
            100,
            5,
            required_position_margin(qty, 100, 5).unwrap(),
            0,
            2,
        )
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

    fn sell_limit_cmd() -> PlaceHyperliquidPerpOrderCmd {
        let mut cmd = limit_cmd();
        cmd.is_buy = false;
        cmd
    }

    fn sell_market_cmd() -> PlaceHyperliquidPerpOrderCmd {
        let mut cmd = market_cmd();
        cmd.is_buy = false;
        cmd
    }

    fn assert_order_snapshot(
        order: &HyperliquidPerpOrder,
        expected_side: HyperliquidPerpOrderSide,
        expected_execution: HyperliquidPerpOrderExecution,
        expected_tif: HyperliquidPerpOrderTimeInForce,
        expected_qty: u64,
        expected_reduce_only: bool,
    ) {
        assert_eq!(order.order_id, "trader-1-BTC-PERP-7");
        assert_eq!(order.asset, 0);
        assert_eq!(order.account_id, "trader-1");
        assert_eq!(order.symbol, "BTC-PERP");
        assert_eq!(order.side, expected_side);
        assert_eq!(order.execution, expected_execution);
        assert_eq!(order.time_in_force, expected_tif);
        assert_eq!(order.qty, expected_qty);
        assert_eq!(order.filled_qty, 0);
        assert_eq!(order.status.as_str(), "open");
        assert_eq!(order.reduce_only, expected_reduce_only);
        assert_eq!(order.client_order_id.as_deref(), Some("client-1"));
        assert_eq!(order.version, 1);
    }

    fn assert_balance_snapshot(balance: &Balance, available: u64, frozen: u64, version: u64) {
        assert_eq!(balance.account_id, "trader-1");
        assert_eq!(balance.asset_id, "USDC");
        assert_eq!(balance.available, available);
        assert_eq!(balance.frozen, frozen);
        assert_eq!(balance.version, version);
    }

    fn assert_order_created_event(
        event: &EntityReplayableEvent,
        expected_side: &str,
        expected_execution: &str,
        expected_tif: &str,
        expected_price: u64,
        expected_reduce_only: bool,
    ) {
        assert_eq!(event.change_type, EntityChangeType::Created.as_tag());
        assert_eq!(event_field(event, "order_id"), Some("trader-1-BTC-PERP-7"));
        assert_eq!(event_field_u64(event, "asset"), Some(0));
        assert_eq!(event_field(event, "account_id"), Some("trader-1"));
        assert_eq!(event_field(event, "symbol"), Some("BTC-PERP"));
        assert_eq!(event_field(event, "side"), Some(expected_side));
        assert_eq!(event_field(event, "execution"), Some(expected_execution));
        assert_eq!(event_field(event, "time_in_force"), Some(expected_tif));
        assert_eq!(event_field_u64(event, "qty"), Some(3));
        assert_eq!(event_field_u64(event, "price"), Some(expected_price));
        assert_eq!(
            event_field(event, "reduce_only"),
            Some(if expected_reduce_only { "true" } else { "false" })
        );
        assert_eq!(event_field(event, "client_order_id"), Some("client-1"));
    }

    fn assert_balance_updated_event(
        event: &EntityReplayableEvent,
        expected_available: u64,
        expected_frozen: u64,
    ) {
        assert_eq!(event.change_type, EntityChangeType::Updated.as_tag());
        assert_eq!(event_field_u64(event, "available"), Some(expected_available));
        assert_eq!(event_field_u64(event, "frozen"), Some(expected_frozen));
        assert_eq!(event.old_version, 4);
        assert_eq!(event.new_version, 5);
    }

    #[test]
    fn role_is_trader() {
        assert_eq!(use_case().role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_malformed_command_fields() {
        let mut cmd = limit_cmd();
        cmd.party_id.clear();
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceHyperliquidPerpOrderError::InvalidPartyId)
        );

        let mut cmd = limit_cmd();
        cmd.symbol.clear();
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceHyperliquidPerpOrderError::InvalidSymbol)
        );

        let mut cmd = limit_cmd();
        cmd.size = 0;
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceHyperliquidPerpOrderError::InvalidSize)
        );

        let mut cmd = limit_cmd();
        cmd.execution = PlaceHyperliquidPerpOrderExecution::Limit {
            price: 0,
            time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
        };
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceHyperliquidPerpOrderError::InvalidPrice)
        );

        let mut cmd = market_cmd();
        cmd.execution = PlaceHyperliquidPerpOrderExecution::Market { aggressive_price: 0 };
        assert_eq!(
            use_case().pre_check_command(&cmd),
            Err(PlaceHyperliquidPerpOrderError::InvalidPrice)
        );
    }

    #[test]
    fn validate_rejects_invalid_loaded_state() {
        let cmd = limit_cmd();

        let mut disabled = state();
        disabled.trading_enabled = false;
        assert_eq!(
            use_case().validate_against_state(&cmd, &disabled),
            Err(PlaceHyperliquidPerpOrderError::TradingDisabled)
        );

        let mut wrong_account = state();
        wrong_account.account_id = "trader-2".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_account),
            Err(PlaceHyperliquidPerpOrderError::AccountMismatch)
        );

        let mut wrong_symbol = state();
        wrong_symbol.market_rules.symbol = "ETH-PERP".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_symbol),
            Err(PlaceHyperliquidPerpOrderError::SymbolNotTradable)
        );

        let mut below_min = state();
        below_min.market_rules.min_qty = 4;
        assert_eq!(
            use_case().validate_against_state(&cmd, &below_min),
            Err(PlaceHyperliquidPerpOrderError::SizeBelowMin)
        );

        let mut wrong_margin_asset = state();
        wrong_margin_asset.margin_balance.asset_id = "USDT".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_margin_asset),
            Err(PlaceHyperliquidPerpOrderError::InvalidMarginBalance)
        );

        let mut wrong_margin_account = state();
        wrong_margin_account.margin_balance.account_id = "trader-2".to_string();
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_margin_account),
            Err(PlaceHyperliquidPerpOrderError::InvalidMarginBalance)
        );

        let mut wrong_position = state();
        wrong_position.position.asset = 1;
        assert_eq!(
            use_case().validate_against_state(&cmd, &wrong_position),
            Err(PlaceHyperliquidPerpOrderError::PositionMismatch)
        );

        let mut inconsistent_position = state();
        inconsistent_position.position = non_flat_position(HyperliquidPerpPositionSide::Long, 0);
        assert_eq!(
            use_case().validate_against_state(&cmd, &inconsistent_position),
            Err(PlaceHyperliquidPerpOrderError::InconsistentPositionState)
        );

        let mut zero_leverage = state();
        zero_leverage.position.leverage = 0;
        assert_eq!(
            use_case().validate_against_state(&cmd, &zero_leverage),
            Err(PlaceHyperliquidPerpOrderError::InvalidLeverage)
        );

        let mut insufficient = state();
        insufficient.margin_balance.available = 60;
        assert_eq!(
            use_case().validate_against_state(&cmd, &insufficient),
            Err(PlaceHyperliquidPerpOrderError::InsufficientMarginBalance)
        );
    }

    // 规格矩阵:
    // - 仓位意图: flat open / same-side increase / opposite reduce or flip / reduce_only
    // - 执行意图: limit / market
    // - 业务结果: freeze full order margin / freeze net new margin only / freeze zero
    // - 事件期望: order create 一定存在；balance update 仅在余额真实变化时存在
    //
    // current coverage:
    // - flat + buy limit
    // - flat + sell limit
    // - flat + buy market
    // - flat + sell market
    // - long + buy increase
    // - short + sell increase
    // - long + sell reduce_only
    // - short + buy reduce_only
    // - long + sell opposite partial reduce
    // - long + sell opposite flip
    // - short + buy opposite partial reduce
    // - short + buy opposite flip

    #[test]
    fn flat_account_buys_limit_order_and_freezes_margin_for_gtc_ioc_and_alo() {
        for (time_in_force, expected_tif) in [
            (HyperliquidPerpOrderTimeInForce::Gtc, "gtc"),
            (HyperliquidPerpOrderTimeInForce::Ioc, "ioc"),
            (HyperliquidPerpOrderTimeInForce::Alo, "alo"),
        ] {
            // Rule:
            // - 空仓账户提交 buy limit 时，应创建 open 订单并冻结整单所需 Cross 保证金。
            //
            // Given:
            // - 账户当前是 flat position。
            // - 执行方式是 buy limit。
            // - tif 在 gtc / ioc / alo 三种 Hyperliquid 成功路径内切换。
            //
            // When:
            // - 调用 `compute_output_and_events(...)`。
            //
            // Then:
            // - 先产出 order create event，再产出 balance update event。
            // - output 中保留 order_after 和单条 balance after。

            // arrange
            let cmd = PlaceHyperliquidPerpOrderCmd {
                execution: PlaceHyperliquidPerpOrderExecution::Limit { price: 101, time_in_force },
                ..limit_cmd()
            };
            assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

            // act
            let result = use_case().compute_output_and_events(&cmd, state()).unwrap();
            let events = result.events;

            // assert
            assert_order_snapshot(
                &result.output.order_after,
                HyperliquidPerpOrderSide::Buy,
                HyperliquidPerpOrderExecution::Limit { price: 101 },
                time_in_force,
                3,
                false,
            );
            assert_eq!(result.output.balances_after.len(), 1);
            assert_balance_snapshot(&result.output.balances_after[0], 9_939, 561, 5);
            assert_eq!(events.len(), 2);
            assert_order_created_event(&events[0], "buy", "limit", expected_tif, 101, false);
            assert_balance_updated_event(&events[1], 9_939, 561);
        }
    }

    #[test]
    fn flat_account_sells_limit_order_and_freezes_margin() {
        // Rule:
        // - 空仓账户提交 sell limit 时，也按整单名义价值冻结 Cross 保证金。
        //
        // Given:
        // - 账户当前是 flat position。
        // - 命令是 sell limit。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 先创建 sell open order，再生成 balance update。

        // arrange
        let cmd = sell_limit_cmd();
        assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, state()).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Sell,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            false,
        );
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_939, 561, 5);
        assert_eq!(events.len(), 2);
        assert_order_created_event(&events[0], "sell", "limit", "gtc", 101, false);
        assert_balance_updated_event(&events[1], 9_939, 561);
    }

    #[test]
    fn flat_account_buys_market_order_and_stores_aggressive_price_as_ioc() {
        // Rule:
        // - 空仓账户提交 market buy 意图时，订单快照应固化为 market + ioc，并按 aggressive price 冻结保证金。
        //
        // Given:
        // - 账户当前是 flat position。
        // - 命令是 market buy，aggressive price 为 111。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 订单事件记录 execution=market、tif=ioc、price=111。
        // - 余额事件反映按 111 计算的冻结金额。

        // arrange
        let cmd = market_cmd();
        assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, state()).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Market { aggressive_price: 111 },
            HyperliquidPerpOrderTimeInForce::Ioc,
            3,
            false,
        );
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_933, 567, 5);
        assert_eq!(events.len(), 2);
        assert_order_created_event(&events[0], "buy", "market", "ioc", 111, false);
        assert_balance_updated_event(&events[1], 9_933, 567);
    }

    #[test]
    fn flat_account_sells_market_order_and_stores_aggressive_price_as_ioc() {
        // Rule:
        // - 空仓账户提交 market sell 意图时，也应固化为 market + ioc，并按 aggressive price 冻结保证金。
        //
        // Given:
        // - 账户当前是 flat position。
        // - 命令是 market sell，aggressive price 为 111。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 订单事件记录 sell / market / ioc。
        // - 余额事件反映按 111 计算的冻结金额。

        // arrange
        let cmd = sell_market_cmd();
        assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, state()).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Sell,
            HyperliquidPerpOrderExecution::Market { aggressive_price: 111 },
            HyperliquidPerpOrderTimeInForce::Ioc,
            3,
            false,
        );
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_933, 567, 5);
        assert_eq!(events.len(), 2);
        assert_order_created_event(&events[0], "sell", "market", "ioc", 111, false);
        assert_balance_updated_event(&events[1], 9_933, 567);
    }

    #[test]
    fn long_position_buys_and_increases_exposure_with_reserved_cross_margin() {
        // Rule:
        // - 同向加仓仍按整单新下单数量冻结新增保证金。
        //
        // Given:
        // - 当前已有 long position。
        // - 命令继续 buy，同向增加敞口。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 仍然创建订单并冻结整单保证金。

        // arrange
        let cmd = limit_cmd();
        let long_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
            ..state()
        };
        assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, long_state).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            false,
        );
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_939, 561, 5);
        assert_eq!(events.len(), 2);
        assert_order_created_event(&events[0], "buy", "limit", "gtc", 101, false);
        assert_balance_updated_event(&events[1], 9_939, 561);
    }

    #[test]
    fn short_position_sells_and_increases_exposure_with_reserved_cross_margin() {
        // Rule:
        // - 同向加空仓时也按整单新下单数量冻结新增保证金。
        //
        // Given:
        // - 当前已有 short position。
        // - 命令继续 sell，同向增加敞口。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 仍然创建订单并冻结整单保证金。

        // arrange
        let cmd = sell_limit_cmd();
        let short_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
            ..state()
        };
        assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, short_state).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Sell,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            false,
        );
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_939, 561, 5);
        assert_eq!(events.len(), 2);
        assert_order_created_event(&events[0], "sell", "limit", "gtc", 101, false);
        assert_balance_updated_event(&events[1], 9_939, 561);
    }

    #[test]
    fn long_position_places_sell_reduce_only_order_without_freezing_margin() {
        // Rule:
        // - long 仓位上的 sell reduce_only 只允许减仓，不应新增冻结保证金。
        //
        // Given:
        // - 当前已有 long position。
        // - 命令是 sell reduce_only。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 只产生一条 order create event。
        // - balances_after 为空。

        // arrange
        let mut cmd = sell_limit_cmd();
        cmd.reduce_only = true;
        let long_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
            ..state()
        };
        assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, long_state).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Sell,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            true,
        );
        assert!(result.output.balances_after.is_empty());
        assert_eq!(events.len(), 1);
        assert_order_created_event(&events[0], "sell", "limit", "gtc", 101, true);
    }

    #[test]
    fn short_position_places_buy_reduce_only_order_without_freezing_margin() {
        // Rule:
        // - short 仓位上的 buy reduce_only 只允许减仓，不应新增冻结保证金。
        //
        // Given:
        // - 当前已有 short position。
        // - 命令是 buy reduce_only。
        //
        // When:
        // - 调用 `compute_output_and_events(...)`。
        //
        // Then:
        // - 只产生一条 order create event。
        // - balances_after 为空。

        // arrange
        let mut cmd = limit_cmd();
        cmd.reduce_only = true;
        let short_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
            ..state()
        };
        assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

        // act
        let result = use_case().compute_output_and_events(&cmd, short_state).unwrap();
        let events = result.events;

        // assert
        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            true,
        );
        assert!(result.output.balances_after.is_empty());
        assert_eq!(events.len(), 1);
        assert_order_created_event(&events[0], "buy", "limit", "gtc", 101, true);
    }

    #[test]
    fn long_position_places_opposite_non_reduce_only_sell_order_that_only_reduces_and_does_not_reserve_new_margin()
     {
        let cmd = sell_limit_cmd();
        let long_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
            ..state()
        };

        assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

        let result = use_case().compute_output_and_events(&cmd, long_state).unwrap();
        let events = result.events;

        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Sell,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            false,
        );
        assert!(result.output.balances_after.is_empty());
        assert_eq!(events.len(), 1);
        assert_order_created_event(&events[0], "sell", "limit", "gtc", 101, false);
    }

    #[test]
    fn long_position_places_opposite_non_reduce_only_sell_order_and_only_reserves_margin_for_net_new_short_exposure()
     {
        let mut cmd = sell_limit_cmd();
        cmd.size = 8;
        let long_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
            ..state()
        };

        assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

        let result = use_case().compute_output_and_events(&cmd, long_state).unwrap();
        let events = result.events;

        assert_eq!(result.output.order_after.qty, 8);
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_939, 561, 5);
        assert_eq!(events.len(), 2);
        assert_eq!(event_field_u64(&events[0], "qty"), Some(8));
        assert_balance_updated_event(&events[1], 9_939, 561);
    }

    #[test]
    fn short_position_places_opposite_non_reduce_only_buy_order_that_only_reduces_and_does_not_reserve_new_margin()
     {
        let cmd = limit_cmd();
        let short_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
            ..state()
        };

        assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

        let result = use_case().compute_output_and_events(&cmd, short_state).unwrap();
        let events = result.events;

        assert_order_snapshot(
            &result.output.order_after,
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            3,
            false,
        );
        assert!(result.output.balances_after.is_empty());
        assert_eq!(events.len(), 1);
        assert_order_created_event(&events[0], "buy", "limit", "gtc", 101, false);
    }

    #[test]
    fn short_position_places_opposite_non_reduce_only_buy_order_and_only_reserves_margin_for_net_new_long_exposure()
     {
        let mut cmd = limit_cmd();
        cmd.size = 8;
        let short_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
            ..state()
        };

        assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

        let result = use_case().compute_output_and_events(&cmd, short_state).unwrap();
        let events = result.events;

        assert_eq!(result.output.order_after.qty, 8);
        assert_eq!(result.output.balances_after.len(), 1);
        assert_balance_snapshot(&result.output.balances_after[0], 9_939, 561, 5);
        assert_eq!(events.len(), 2);
        assert_eq!(event_field_u64(&events[0], "qty"), Some(8));
        assert_balance_updated_event(&events[1], 9_939, 561);
    }

    #[test]
    fn reduce_only_rejects_flat_same_direction_and_oversized_orders() {
        let mut buy_reduce = limit_cmd();
        buy_reduce.reduce_only = true;
        assert_eq!(
            use_case().validate_against_state(&buy_reduce, &state()),
            Err(PlaceHyperliquidPerpOrderError::InvalidReduceOnly)
        );

        let long_state = PlaceHyperliquidPerpOrderState {
            position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
            ..state()
        };
        assert_eq!(
            use_case().validate_against_state(&buy_reduce, &long_state),
            Err(PlaceHyperliquidPerpOrderError::InvalidReduceOnly)
        );

        let mut sell_reduce = limit_cmd();
        sell_reduce.is_buy = false;
        sell_reduce.size = 6;
        sell_reduce.reduce_only = true;
        assert_eq!(
            use_case().validate_against_state(&sell_reduce, &long_state),
            Err(PlaceHyperliquidPerpOrderError::InvalidReduceOnly)
        );
    }

    #[test]
    fn validate_rejects_non_flat_position_whose_stored_margin_does_not_match_required_margin() {
        let cmd = limit_cmd();
        let mut invalid_position = non_flat_position(HyperliquidPerpPositionSide::Long, 5);
        invalid_position.margin -= 1;
        let invalid_state =
            PlaceHyperliquidPerpOrderState { position: invalid_position, ..state() };

        assert_eq!(
            use_case().validate_against_state(&cmd, &invalid_state),
            Err(PlaceHyperliquidPerpOrderError::InconsistentPositionState)
        );
    }

    proptest! {
        #[test]
        fn required_margin_is_ceiled_notional_divided_by_leverage(
            size in 1_u64..1_000_000,
            price in 1_u64..1_000_000,
            leverage in 1_u64..125,
        ) {
            let margin = required_position_margin(size, price, leverage).unwrap();
            let notional = size * price;
            prop_assert_eq!(margin, notional.div_ceil(leverage));
        }

        #[test]
        fn freezing_margin_preserves_total_cross_margin_balance(
            size in 1_u64..100_000,
            price in 1_u64..100_000,
            leverage in 1_u64..125,
            existing_frozen in 0_u64..1_000_000,
        ) {
            let margin = required_position_margin(size, price, leverage).unwrap();
            let cmd = PlaceHyperliquidPerpOrderCmd {
                size,
                execution: PlaceHyperliquidPerpOrderExecution::Limit {
                    price,
                    time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
                },
                ..limit_cmd()
            };
            let state = PlaceHyperliquidPerpOrderState {
                margin_balance: Balance::new(
                    "trader-1".to_string(),
                    "USDC".to_string(),
                    margin,
                    existing_frozen,
                    1,
                ),
                position: HyperliquidPerpPosition::empty_slot(
                    "trader-1:0:BTC-PERP".to_string(),
                    "trader-1".to_string(),
                    0,
                    "BTC-PERP".to_string(),
                    leverage,
                ),
                ..state()
            };

            let result = use_case().compute_output_and_events(&cmd, state).unwrap();
            let events = result.events;
            let next_available = event_field_u64(&events[1], "available").unwrap();
            let next_frozen = event_field_u64(&events[1], "frozen").unwrap();

            prop_assert_eq!(next_available, 0);
            prop_assert_eq!(next_frozen, existing_frozen + margin);
            prop_assert_eq!(next_available + next_frozen, existing_frozen + margin);
        }

        #[test]
        fn created_order_event_matches_command_and_state(
            size in 1_u64..100_000,
            price in 1_u64..100_000,
            leverage in 1_u64..125,
            is_buy in any::<bool>(),
            asset in 0_u32..10_000,
        ) {
            let margin = required_position_margin(size, price, leverage).unwrap();
            let cmd = PlaceHyperliquidPerpOrderCmd {
                asset,
                is_buy,
                size,
                execution: PlaceHyperliquidPerpOrderExecution::Limit {
                    price,
                    time_in_force: HyperliquidPerpOrderTimeInForce::Alo,
                },
                ..limit_cmd()
            };
            let state = PlaceHyperliquidPerpOrderState {
                next_order_sequence: 99,
                margin_balance: Balance::new(
                    "trader-1".to_string(),
                    "USDC".to_string(),
                    margin,
                    0,
                    1,
                ),
                position: HyperliquidPerpPosition::empty_slot(
                    format!("trader-1:{asset}:BTC-PERP"),
                    "trader-1".to_string(),
                    asset,
                    "BTC-PERP".to_string(),
                    leverage,
                ),
                ..state()
            };

            let result = use_case().compute_output_and_events(&cmd, state).unwrap();
            let events = result.events;
            let expected_side = if is_buy { "buy" } else { "sell" };

            prop_assert_eq!(event_field(&events[0], "order_id"), Some("trader-1-BTC-PERP-99"));
            prop_assert_eq!(event_field_u64(&events[0], "asset"), Some(u64::from(asset)));
            prop_assert_eq!(event_field(&events[0], "account_id"), Some("trader-1"));
            prop_assert_eq!(event_field(&events[0], "symbol"), Some("BTC-PERP"));
            prop_assert_eq!(event_field(&events[0], "side"), Some(expected_side));
            prop_assert_eq!(event_field(&events[0], "execution"), Some("limit"));
            prop_assert_eq!(event_field(&events[0], "time_in_force"), Some("alo"));
            prop_assert_eq!(event_field_u64(&events[0], "qty"), Some(size));
            prop_assert_eq!(event_field_u64(&events[0], "price"), Some(price));
            prop_assert_eq!(event_field(&events[0], "reduce_only"), Some("false"));
            prop_assert_eq!(event_field(&events[0], "client_order_id"), Some("client-1"));
        }
    }
}
