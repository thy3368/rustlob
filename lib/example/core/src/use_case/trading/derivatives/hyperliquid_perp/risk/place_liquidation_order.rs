use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus, HyperliquidPerpOrder,
    HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide, HyperliquidPerpOrderTimeInForce,
    HyperliquidPerpPosition, HyperliquidPerpPositionSide,
};

/// 发出单张 Hyperliquid perp 强平单的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpLiquidationOrderCmd {
    /// 发起风险下单的业务主体。
    pub party_id: String,
    /// 当前强平会话 ID。
    pub liquidation_id: String,
    /// 风险引擎给出的激进价格上限。
    pub price_cap: u64,
    /// 可选客户端订单 ID。
    pub cloid: Option<String>,
}

impl IssuedByParty for PlaceHyperliquidPerpLiquidationOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 发出强平单时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpLiquidationOrderState {
    /// 当前强平会话。
    pub liquidation: HyperliquidPerpLiquidation,
    /// 被强平仓位快照。
    pub position: HyperliquidPerpPosition,
    /// 当前账户 ID。
    pub account_id: String,
    /// 用于稳定生成订单 ID 的下一个序号。
    pub next_order_sequence: u64,
    /// 当前已存在的未完成强平单 ID。
    pub existing_open_liquidation_order_ids: Vec<String>,
}

/// 发出强平单时可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceHyperliquidPerpLiquidationOrderError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("price_cap must be greater than zero")]
    InvalidPriceCap,
    #[error("liquidation does not match command liquidation id")]
    LiquidationMismatch,
    #[error("account does not match liquidation account")]
    AccountMismatch,
    #[error("position does not match liquidation")]
    PositionMismatch,
    #[error("position state is inconsistent")]
    InconsistentPositionState,
    #[error("position is flat and cannot place liquidation order")]
    FlatPosition,
    #[error("liquidation status does not allow placing an order")]
    LiquidationNotPlaceable,
    #[error("liquidation remaining qty must be greater than zero")]
    EmptyRemainingQty,
    #[error("open liquidation order already exists")]
    OpenLiquidationOrderExists,
    #[error("position qty is smaller than liquidation remaining qty")]
    PositionQtyTooSmall,
    #[error("arithmetic overflow while deriving liquidation order")]
    ArithmeticOverflow,
}

/// 为一个已经启动的强平会话发出单张订单簿强平单。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceHyperliquidPerpLiquidationOrderUseCase;

impl CommandUseCase2 for PlaceHyperliquidPerpLiquidationOrderUseCase {
    type Command = PlaceHyperliquidPerpLiquidationOrderCmd;
    type GivenState = PlaceHyperliquidPerpLiquidationOrderState;
    type Error = PlaceHyperliquidPerpLiquidationOrderError;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(PlaceHyperliquidPerpLiquidationOrderError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(PlaceHyperliquidPerpLiquidationOrderError::InvalidLiquidationId);
        }
        if cmd.price_cap == 0 {
            return Err(PlaceHyperliquidPerpLiquidationOrderError::InvalidPriceCap);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        validate_state(cmd, state)
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        validate_state(cmd, &state)?;

        let order_qty = state
            .liquidation
            .next_order_qty(state.position.qty)
            .ok_or(PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;
        let order_id =
            format!("{}-{}-{}", state.account_id, state.position.symbol, state.next_order_sequence);
        let side = liquidation_order_side(state.position.side)?;
        let order = HyperliquidPerpOrder::new(
            order_id.clone(),
            None,
            state.position.asset,
            state.account_id.clone(),
            state.position.symbol.clone(),
            side,
            HyperliquidPerpOrderExecution::Market { aggressive_price: cmd.price_cap },
            HyperliquidPerpOrderTimeInForce::Ioc,
            order_qty,
            true,
            cmd.cloid.clone(),
        )
        .with_liquidation(cmd.liquidation_id.clone());
        let order_event = order
            .track_create_event()
            .map_err(|_| PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;

        let mut liquidation = state.liquidation;
        let next_version = liquidation
            .version
            .checked_add(1)
            .ok_or(PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;
        let liquidation_event = liquidation
            .track_update_event(|liquidation| {
                let _ = liquidation.apply_order_placed(order_id, order_qty, next_version);
            })
            .map_err(|_| PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;

        Ok(vec![order_event, liquidation_event])
    }
}

fn validate_state(
    cmd: &PlaceHyperliquidPerpLiquidationOrderCmd,
    state: &PlaceHyperliquidPerpLiquidationOrderState,
) -> Result<(), PlaceHyperliquidPerpLiquidationOrderError> {
    if state.liquidation.liquidation_id != cmd.liquidation_id {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::LiquidationMismatch);
    }
    if state.account_id != state.liquidation.account_id
        || state.position.account_id != state.liquidation.account_id
    {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::AccountMismatch);
    }
    if state.position.position_id != state.liquidation.position_id
        || state.position.asset != state.liquidation.asset
        || state.position.symbol != state.liquidation.symbol
    {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::PositionMismatch);
    }
    if !state.position.has_consistent_state() {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::InconsistentPositionState);
    }
    if state.position.is_flat() {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::FlatPosition);
    }
    if !matches!(
        state.liquidation.status,
        HyperliquidPerpLiquidationStatus::Started | HyperliquidPerpLiquidationStatus::OrderPlaced
    ) || !state.liquidation.can_place_order()
    {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::LiquidationNotPlaceable);
    }
    if state.liquidation.remaining_qty == 0 {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::EmptyRemainingQty);
    }
    if !state.existing_open_liquidation_order_ids.is_empty() {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::OpenLiquidationOrderExists);
    }
    if state.position.qty < state.liquidation.remaining_qty {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::PositionQtyTooSmall);
    }
    let _ = liquidation_order_side(state.position.side)?;
    Ok(())
}

fn liquidation_order_side(
    position_side: HyperliquidPerpPositionSide,
) -> Result<HyperliquidPerpOrderSide, PlaceHyperliquidPerpLiquidationOrderError> {
    match position_side {
        HyperliquidPerpPositionSide::Long => Ok(HyperliquidPerpOrderSide::Sell),
        HyperliquidPerpPositionSide::Short => Ok(HyperliquidPerpOrderSide::Buy),
        HyperliquidPerpPositionSide::Flat => {
            Err(PlaceHyperliquidPerpLiquidationOrderError::FlatPosition)
        }
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::CommandUseCase2;

    use super::*;
    use crate::entity::{HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode};

    fn position(side: HyperliquidPerpPositionSide, qty: u64) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            side,
            qty,
            60_000,
            5,
            24_000,
            0,
            3,
        )
    }

    fn liquidation(side: HyperliquidPerpPositionSide, qty: u64) -> HyperliquidPerpLiquidation {
        HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "liq-batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            side,
            qty,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Started,
        )
    }

    fn state(
        side: HyperliquidPerpPositionSide,
        qty: u64,
    ) -> PlaceHyperliquidPerpLiquidationOrderState {
        PlaceHyperliquidPerpLiquidationOrderState {
            liquidation: liquidation(side, qty),
            position: position(side, qty),
            account_id: "trader-1".to_string(),
            next_order_sequence: 11,
            existing_open_liquidation_order_ids: Vec::new(),
        }
    }

    fn cmd() -> PlaceHyperliquidPerpLiquidationOrderCmd {
        PlaceHyperliquidPerpLiquidationOrderCmd {
            party_id: "risk-engine".to_string(),
            liquidation_id: "liq-1".to_string(),
            price_cap: 48_500,
            cloid: Some("liq-cloid-1".to_string()),
        }
    }

    #[test]
    fn role_returns_risk_engine() {
        assert_eq!(PlaceHyperliquidPerpLiquidationOrderUseCase.role(), "RiskEngine");
    }

    #[test]
    fn pre_check_rejects_zero_price_cap() {
        let mut cmd = cmd();
        cmd.price_cap = 0;

        assert_eq!(
            PlaceHyperliquidPerpLiquidationOrderUseCase.pre_check_command(&cmd),
            Err(PlaceHyperliquidPerpLiquidationOrderError::InvalidPriceCap)
        );
    }

    #[test]
    fn validate_rejects_when_open_liquidation_order_exists() {
        let mut state = state(HyperliquidPerpPositionSide::Long, 10);
        state.existing_open_liquidation_order_ids.push("order-1".to_string());

        assert_eq!(
            PlaceHyperliquidPerpLiquidationOrderUseCase.validate_against_state(&cmd(), &state),
            Err(PlaceHyperliquidPerpLiquidationOrderError::OpenLiquidationOrderExists)
        );
    }

    #[test]
    fn compute_creates_sell_partial_liquidation_order_for_long() {
        let events = PlaceHyperliquidPerpLiquidationOrderUseCase
            .compute_replayable_events(&cmd(), state(HyperliquidPerpPositionSide::Long, 10))
            .unwrap();

        assert_eq!(events.len(), 2);
        assert!(events[0].is_created());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("side") && change.new_value_bytes() == b"sell"
        }));
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("qty") && change.new_value_bytes() == b"2"
        }));
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("is_liquidation")
                && change.new_value_bytes() == b"true"
        }));
        assert!(events[1].is_updated());
        assert!(events[1].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"order_placed"
        }));
    }

    #[test]
    fn compute_creates_buy_full_liquidation_order_for_small_short() {
        let events = PlaceHyperliquidPerpLiquidationOrderUseCase
            .compute_replayable_events(&cmd(), state(HyperliquidPerpPositionSide::Short, 3))
            .unwrap();

        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("side") && change.new_value_bytes() == b"buy"
        }));
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("qty") && change.new_value_bytes() == b"3"
        }));
    }
}
