use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpLiquidation, HyperliquidPerpLiquidationFill, HyperliquidPerpLiquidationStatus,
    HyperliquidPerpTrade,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyHyperliquidPerpLiquidationFillCmd {
    pub party_id: String,
    pub liquidation_id: String,
    pub order_id: String,
    pub trade_id: String,
}

impl IssuedByParty for ApplyHyperliquidPerpLiquidationFillCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyHyperliquidPerpLiquidationFillState {
    pub liquidation: HyperliquidPerpLiquidation,
    pub trade: HyperliquidPerpTrade,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ApplyHyperliquidPerpLiquidationFillError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("liquidation_id must not be empty")]
    InvalidLiquidationId,
    #[error("order_id must not be empty")]
    InvalidOrderId,
    #[error("trade_id must not be empty")]
    InvalidTradeId,
    #[error("liquidation does not match command liquidation id")]
    LiquidationMismatch,
    #[error("trade does not match command trade id")]
    TradeMismatch,
    #[error("trade is not for the liquidation order")]
    OrderMismatch,
    #[error("trade does not belong to liquidation account")]
    AccountMismatch,
    #[error("trade market does not match liquidation")]
    MarketMismatch,
    #[error("liquidation status does not allow fill application")]
    LiquidationNotExecutable,
    #[error("trade qty exceeds liquidation remaining qty")]
    FillQtyTooLarge,
    #[error("arithmetic overflow while applying liquidation fill")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ApplyHyperliquidPerpLiquidationFillUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyHyperliquidPerpLiquidationFillChanges {
    pub created_fill: HyperliquidPerpLiquidationFill,
    pub changed_liquidation: UpdatedEntityPair<HyperliquidPerpLiquidation>,
}

impl ReplayableChanges for ApplyHyperliquidPerpLiquidationFillChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        Ok(vec![
            self.created_fill.track_create_event()?,
            self.changed_liquidation
                .after
                .track_update_event_from(&self.changed_liquidation.before)?,
        ])
    }
}

impl CommandUseCase4 for ApplyHyperliquidPerpLiquidationFillUseCase {
    type Command = ApplyHyperliquidPerpLiquidationFillCmd;
    type GivenState = ApplyHyperliquidPerpLiquidationFillState;
    type Error = ApplyHyperliquidPerpLiquidationFillError;
    type Changes = ApplyHyperliquidPerpLiquidationFillChanges;

    fn role(&self) -> &'static str {
        "RiskEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ApplyHyperliquidPerpLiquidationFillError::InvalidPartyId);
        }
        if cmd.liquidation_id.is_empty() {
            return Err(ApplyHyperliquidPerpLiquidationFillError::InvalidLiquidationId);
        }
        if cmd.order_id.is_empty() {
            return Err(ApplyHyperliquidPerpLiquidationFillError::InvalidOrderId);
        }
        if cmd.trade_id.is_empty() {
            return Err(ApplyHyperliquidPerpLiquidationFillError::InvalidTradeId);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.liquidation.liquidation_id != cmd.liquidation_id {
            return Err(ApplyHyperliquidPerpLiquidationFillError::LiquidationMismatch);
        }
        if state.trade.trade_id != cmd.trade_id {
            return Err(ApplyHyperliquidPerpLiquidationFillError::TradeMismatch);
        }
        if cmd.order_id != state.trade.taker_order_id && cmd.order_id != state.trade.maker_order_id
        {
            return Err(ApplyHyperliquidPerpLiquidationFillError::OrderMismatch);
        }
        let account_matches = if cmd.order_id == state.trade.taker_order_id {
            state.trade.taker_account_id == state.liquidation.account_id
        } else {
            state.trade.maker_account_id == state.liquidation.account_id
        };
        if !account_matches {
            return Err(ApplyHyperliquidPerpLiquidationFillError::AccountMismatch);
        }
        if state.trade.asset != state.liquidation.asset
            || state.trade.symbol != state.liquidation.symbol
        {
            return Err(ApplyHyperliquidPerpLiquidationFillError::MarketMismatch);
        }
        if !matches!(
            state.liquidation.status,
            HyperliquidPerpLiquidationStatus::Started | HyperliquidPerpLiquidationStatus::Executing
        ) {
            return Err(ApplyHyperliquidPerpLiquidationFillError::LiquidationNotExecutable);
        }
        if state.trade.qty == 0 || state.trade.qty > state.liquidation.remaining_qty {
            return Err(ApplyHyperliquidPerpLiquidationFillError::FillQtyTooLarge);
        }
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let recovered_quote = state
            .trade
            .notional_quote()
            .ok_or(ApplyHyperliquidPerpLiquidationFillError::ArithmeticOverflow)?;
        let created_fill = HyperliquidPerpLiquidationFill::new(
            format!("{}-{}", cmd.liquidation_id, cmd.trade_id),
            cmd.liquidation_id.clone(),
            cmd.order_id.clone(),
            cmd.trade_id.clone(),
            state.liquidation.account_id.clone(),
            state.liquidation.position_id.clone(),
            state.liquidation.asset,
            state.liquidation.symbol.clone(),
            state.liquidation.signed_size,
            state.trade.qty,
            state.trade.price,
            state.liquidation.bankruptcy_price,
            recovered_quote,
        );

        let before = state.liquidation;
        let mut after = before.clone();
        let next_version = after
            .version
            .checked_add(1)
            .ok_or(ApplyHyperliquidPerpLiquidationFillError::ArithmeticOverflow)?;
        after
            .mark_executing(cmd.order_id.clone(), state.trade.qty, next_version)
            .ok_or(ApplyHyperliquidPerpLiquidationFillError::ArithmeticOverflow)?;

        Ok(ApplyHyperliquidPerpLiquidationFillChanges {
            created_fill,
            changed_liquidation: UpdatedEntityPair { before, after },
        })
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::HyperliquidPerpOrderSide;
    use crate::entity::{HyperliquidPerpLiquidationTriggerReason, HyperliquidPerpMarginMode};

    fn liquidation() -> HyperliquidPerpLiquidation {
        HyperliquidPerpLiquidation::new(
            "liq-1".to_string(),
            "batch-1".to_string(),
            "risk-engine".to_string(),
            "trader-1".to_string(),
            "position-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            3,
            3,
            HyperliquidPerpMarginMode::Cross,
            49_000,
            50_000,
            HyperliquidPerpLiquidationTriggerReason::BankruptcyRisk,
            HyperliquidPerpLiquidationStatus::Started,
        )
    }

    fn trade() -> HyperliquidPerpTrade {
        HyperliquidPerpTrade::new(
            "trade-1".to_string(),
            "match-1".to_string(),
            7,
            "BTC-PERP".to_string(),
            "order-1".to_string(),
            "maker-1".to_string(),
            "trader-1".to_string(),
            "maker-account".to_string(),
            HyperliquidPerpOrderSide::Sell,
            48_000,
            2,
            1_717_171_717_000,
        )
    }

    #[test]
    fn apply_fill_creates_fact_and_moves_liquidation_to_executing() {
        let changes = ApplyHyperliquidPerpLiquidationFillUseCase
            .compute_changes(
                &ApplyHyperliquidPerpLiquidationFillCmd {
                    party_id: "risk-engine".to_string(),
                    liquidation_id: "liq-1".to_string(),
                    order_id: "order-1".to_string(),
                    trade_id: "trade-1".to_string(),
                },
                ApplyHyperliquidPerpLiquidationFillState {
                    liquidation: liquidation(),
                    trade: trade(),
                },
            )
            .unwrap();

        assert_eq!(
            changes.changed_liquidation.after.status,
            HyperliquidPerpLiquidationStatus::Executing
        );
        assert_eq!(changes.created_fill.recovered_quote, 96_000);
        assert_eq!(changes.to_replayable_events().unwrap().len(), 2);
    }
}
