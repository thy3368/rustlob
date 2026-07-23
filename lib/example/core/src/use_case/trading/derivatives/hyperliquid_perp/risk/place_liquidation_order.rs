use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationStatus, HyperliquidPerpOrder,
    HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide, HyperliquidPerpOrderTimeInForce,
    HyperliquidPerpPosition,
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

/// 发出强平单后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpLiquidationOrderChanges {
    /// 本次新创建的强平单。
    pub created_order: HyperliquidPerpOrder,
    /// 强平会话的 before/after 对。
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
}

impl ReplayableChanges for PlaceHyperliquidPerpLiquidationOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.created_order.track_create_event()?,
            self.changed_liquidation
                .after
                .track_update_event_from(&self.changed_liquidation.before)?,
        ])
    }
}

impl CommandUseCase4 for PlaceHyperliquidPerpLiquidationOrderUseCase {
    type Command = PlaceHyperliquidPerpLiquidationOrderCmd;
    type GivenState = PlaceHyperliquidPerpLiquidationOrderState;
    type Error = PlaceHyperliquidPerpLiquidationOrderError;
    type Changes = PlaceHyperliquidPerpLiquidationOrderChanges;

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

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let order_qty = state
            .liquidation
            .next_order_qty(state.position.qty())
            .ok_or(PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;
        let order_id =
            format!("{}-{}-{}", state.account_id, state.position.coin, state.next_order_sequence);
        let side = liquidation_order_side(&state.position)?;
        let created_order = HyperliquidPerpOrder::new(
            order_id.clone(),
            None,
            state.position.perp_asset_id,
            state.account_id.clone(),
            state.position.coin.clone(),
            side,
            HyperliquidPerpOrderExecution::Market { aggressive_price: cmd.price_cap },
            HyperliquidPerpOrderTimeInForce::Ioc,
            order_qty,
            true,
            cmd.cloid.clone(),
            None,
        )
        .with_liquidation(cmd.liquidation_id.clone());

        let mut liquidation_after = state.liquidation.clone();
        let next_version = liquidation_after
            .version
            .checked_add(1)
            .ok_or(PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;
        liquidation_after
            .apply_order_placed(order_id, order_qty, next_version)
            .ok_or(PlaceHyperliquidPerpLiquidationOrderError::ArithmeticOverflow)?;

        Ok(PlaceHyperliquidPerpLiquidationOrderChanges {
            created_order,
            changed_liquidation: UpdatedEntityPair {
                before: state.liquidation,
                after: liquidation_after,
            },
        })
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
    if state.position.position_key != state.liquidation.position_id
        || state.position.perp_asset_id != state.liquidation.asset
        || state.position.coin != state.liquidation.symbol
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
    ) {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::LiquidationNotPlaceable);
    }
    if !state.liquidation.can_place_order() {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::LiquidationNotPlaceable);
    }
    if state.liquidation.remaining_qty == 0 {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::EmptyRemainingQty);
    }
    if !state.existing_open_liquidation_order_ids.is_empty() {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::OpenLiquidationOrderExists);
    }
    if state.position.qty() < state.liquidation.remaining_qty {
        return Err(PlaceHyperliquidPerpLiquidationOrderError::PositionQtyTooSmall);
    }
    let _ = liquidation_order_side(&state.position)?;
    Ok(())
}

fn liquidation_order_side(
    position: &HyperliquidPerpPosition,
) -> Result<HyperliquidPerpOrderSide, PlaceHyperliquidPerpLiquidationOrderError> {
    if position.is_long() {
        Ok(HyperliquidPerpOrderSide::Sell)
    } else if position.is_short() {
        Ok(HyperliquidPerpOrderSide::Buy)
    } else {
        Err(PlaceHyperliquidPerpLiquidationOrderError::FlatPosition)
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::{HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode};

    fn position(signed_size: i64) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::new(
            "position-1".to_string(),
            "trader-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            signed_size,
            60_000,
            5,
            HyperliquidPerpMarginMode::Cross,
            0,
            3,
        )
    }

    fn liquidation(signed_size: i64) -> HyperliquidPerpLiquidation {
        HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "liq-batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            signed_size,
            signed_size.unsigned_abs(),
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Started,
        )
    }

    fn state(
        signed_size: i64,
    ) -> PlaceHyperliquidPerpLiquidationOrderState {
        PlaceHyperliquidPerpLiquidationOrderState {
            liquidation: liquidation(signed_size),
            position: position(signed_size),
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
        let mut state = state(10);
        state.existing_open_liquidation_order_ids.push("order-1".to_string());

        assert_eq!(
            PlaceHyperliquidPerpLiquidationOrderUseCase.validate_against_state(&cmd(), &state),
            Err(PlaceHyperliquidPerpLiquidationOrderError::OpenLiquidationOrderExists)
        );
    }

    #[test]
    fn risk_engine_place_order_for_long_updates_session_pair_and_projects_two_events() {
        // Rule:
        // - 对 long 仓位发强平单时，风险引擎必须创建一张 sell liquidation order，
        //   并把强平会话推进到 `OrderPlaced`。
        //
        // Given:
        // - 一个 Started 状态、剩余数量为 10 的 long liquidation 会话。
        //
        // When:
        // - 调用 `compute_changes()` 并再投影 `to_replayable_events()`。
        //
        // Then:
        // - changes 先给出新订单和 liquidation before/after pair。
        // - after 状态为 `OrderPlaced`，数量、最近订单和版本推进正确。
        // - 最终按顺序投影出 order create event 和 liquidation update event。

        // arrange
        let use_case = PlaceHyperliquidPerpLiquidationOrderUseCase;

        // act
        let changes =
            use_case.compute_changes(&cmd(), state(10)).unwrap();
        let events = changes.to_replayable_events().unwrap();

        // assert
        assert_eq!(changes.created_order.side, HyperliquidPerpOrderSide::Sell);
        assert_eq!(changes.created_order.liquidation_id.as_deref(), Some("liq-1"));
        assert_eq!(changes.created_order.qty, 2);
        assert_eq!(
            changes.changed_liquidation.before.status,
            HyperliquidPerpLiquidationStatus::Started
        );
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::OrderPlaced
        );
        assert_eq!(changes.changed_liquidation.after.placed_order_count, 1);
        assert_eq!(changes.changed_liquidation.after.remaining_qty, 8);
        assert_eq!(
            changes.changed_liquidation.after.last_order_id.as_deref(),
            Some("trader-1-BTC-PERP-11")
        );
        assert_eq!(changes.changed_liquidation.before.version, 1);
        assert_eq!(changes.changed_liquidation.after.version, 2);

        assert_eq!(events.len(), 2);
        assert!(events[0].is_created());
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("side") && change.new_value_bytes() == b"sell"
        }));
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("qty") && change.new_value_bytes() == b"2"
        }));
        assert!(events[1].is_updated());
        assert!(events[1].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("status")
                && change.new_value_bytes() == b"order_placed"
        }));
        assert!(events[1].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("remaining_qty")
                && change.new_value_bytes() == b"8"
        }));
    }

    #[test]
    fn risk_engine_place_order_for_small_short_creates_buy_order_and_consumes_remaining_qty() {
        // Rule:
        // - 对小数量 short 仓位发强平单时，本轮直接下满量 buy order。
        //
        // Given:
        // - 一个 Started 状态、剩余数量为 3 的 short liquidation 会话。
        //
        // When:
        // - 调用 `compute_changes()` 并再投影 `to_replayable_events()`。
        //
        // Then:
        // - created order 为 buy，数量为 3。
        // - liquidation after 进入 `OrderPlaced` 且 `remaining_qty == 0`。

        // arrange
        let use_case = PlaceHyperliquidPerpLiquidationOrderUseCase;

        // act
        let changes =
            use_case.compute_changes(&cmd(), state(-3)).unwrap();
        let events = changes.to_replayable_events().unwrap();

        // assert
        assert_eq!(changes.created_order.side, HyperliquidPerpOrderSide::Buy);
        assert_eq!(changes.created_order.qty, 3);
        assert_eq!(
            changes.changed_liquidation.before.status,
            HyperliquidPerpLiquidationStatus::Started
        );
        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::OrderPlaced
        );
        assert_eq!(changes.changed_liquidation.after.remaining_qty, 0);
        assert_eq!(changes.changed_liquidation.after.placed_qty_total, 3);

        assert_eq!(events.len(), 2);
        assert!(events[0].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("side") && change.new_value_bytes() == b"buy"
        }));
        assert!(events[1].field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("remaining_qty")
                && change.new_value_bytes() == b"0"
        }));
    }
}
