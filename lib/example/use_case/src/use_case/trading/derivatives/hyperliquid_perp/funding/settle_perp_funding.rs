use std::collections::HashSet;

use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, TrackDiff, UpdatedEntityPair,
    UseCase,
};
use thiserror::Error;

use crate::entity::perp::fund::user_funding::UserFunding;
use crate::entity::perp::fund::user_funding_domain_service::{
    SettleUserFundingBatchError, settle_user_funding_batch,
};
use crate::entity::{BalanceV2, MarginModeV3, PerpPositionV3, UserId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingCmd {
    pub operator: UserId,
    pub funding_batch_id: u64,
    pub funding_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingState {
    pub fundings: Vec<UserFunding>,
    pub positions: Vec<PerpPositionV3>,
    pub margin_balances: Vec<BalanceV2>,
    pub settled_funding_ids: Vec<u64>,
    pub margin_mode: MarginModeV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpFundingError {
    #[error("funding batch id must be non-zero")]
    InvalidFundingBatchId,
    #[error("funding ids must not be empty")]
    EmptyFundingIds,
    #[error("funding ids do not match loaded facts")]
    FundingIdsMismatch,
    #[error("funding fact was already settled")]
    FundingAlreadySettled,
    #[error(transparent)]
    Settlement(#[from] SettleUserFundingBatchError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpFundingUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingChanges {
    pub changed_fundings: Vec<UpdatedEntityPair<UserFunding>>,
    pub changed_positions: Vec<UpdatedEntityPair<PerpPositionV3>>,
    pub changed_margin_balances: Vec<UpdatedEntityPair<BalanceV2>>,
}

impl ReplayableChanges for SettleHyperliquidPerpFundingChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(
            self.changed_fundings.len()
                + self.changed_positions.len()
                + self.changed_margin_balances.len(),
        );
        for pair in &self.changed_fundings {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        for pair in &self.changed_positions {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        for pair in &self.changed_margin_balances {
            events.push(pair.after.track_update_event_from(&pair.before)?);
        }
        Ok(events)
    }
}

impl UseCase for SettleHyperliquidPerpFundingUseCase {
    type Command = LogEntry<SettleHyperliquidPerpFundingCmd>;
    type StateGiven = SettleHyperliquidPerpFundingState;
    type Error = SettleHyperliquidPerpFundingError;
    type StateChanged = SettleHyperliquidPerpFundingChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.funding_batch_id == 0 {
            return Err(SettleHyperliquidPerpFundingError::InvalidFundingBatchId);
        }
        if cmd.funding_ids.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::EmptyFundingIds);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if cmd.funding_ids.len() != state.fundings.len()
            || cmd
                .funding_ids
                .iter()
                .zip(&state.fundings)
                .any(|(id, funding)| *id != funding.funding_id())
        {
            return Err(SettleHyperliquidPerpFundingError::FundingIdsMismatch);
        }
        let settled: HashSet<u64> = state.settled_funding_ids.iter().copied().collect();
        if cmd.funding_ids.iter().any(|id| settled.contains(id)) {
            return Err(SettleHyperliquidPerpFundingError::FundingAlreadySettled);
        }
        settle_user_funding_batch(
            &state.fundings,
            &state.positions,
            &state.margin_balances,
            cmd.logical_ts(),
        )
        .map(|_| ())
        .map_err(Into::into)
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        //todo 增加 settle_hour
        let output = settle_user_funding_batch(
            &state.fundings,
            &state.positions,
            &state.margin_balances,
            cmd.logical_ts(),
        )?;
        Ok(SettleHyperliquidPerpFundingChanges {
            changed_fundings: output.changed_fundings,
            changed_positions: output.changed_positions,
            changed_margin_balances: output.changed_balances,
        })
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, LogEntrySequencer};
    use rust_decimal::Decimal;

    use super::*;
    use crate::entity::account::coin::Coin;
    use crate::entity::account::symbol::Symbol;
    use crate::entity::common::FundingRate;
    use crate::entity::perp::trade::perp_position_v3::FundingAccumulatorV3;

    const PARTY_ID: u64 = 7;
    const SETTLED_AT: u64 = 120;

    fn command(
        funding_batch_id: u64,
        funding_ids: Vec<u64>,
    ) -> LogEntry<SettleHyperliquidPerpFundingCmd> {
        LogEntrySequencer::from_metadata(
            1,
            SETTLED_AT,
            1,
            1,
            common_entity::SYSTEM_USER_ID.to_owned(),
            SettleHyperliquidPerpFundingCmd { operator: UserId(99), funding_batch_id, funding_ids },
        )
    }

    fn funding(id: u64, amount: Decimal) -> UserFunding {
        UserFunding::new(
            id,
            PARTY_ID,
            Symbol::new(1, Coin::BTC, Coin::USDC),
            amount,
            Decimal::ONE,
            FundingRate::new(Decimal::new(1, 4)).unwrap(),
            None,
            100,
        )
    }

    fn state(fundings: Vec<UserFunding>) -> SettleHyperliquidPerpFundingState {
        SettleHyperliquidPerpFundingState {
            fundings,
            positions: vec![
                PerpPositionV3::new(
                    2,
                    UserId(PARTY_ID),
                    Coin::BTC,
                    Decimal::ONE,
                    Decimal::from(100),
                    Decimal::from(5),
                    MarginModeV3::Cross,
                    None,
                    Decimal::ZERO,
                    FundingAccumulatorV3::default(),
                    Decimal::ZERO,
                    3,
                )
                .unwrap(),
            ],
            margin_balances: vec![BalanceV2::new(
                PARTY_ID,
                11,
                u64::from(Coin::USDC.index()),
                Decimal::TEN,
                Decimal::ZERO,
                4,
                100,
            )],
            settled_funding_ids: Vec::new(),
            margin_mode: MarginModeV3::Cross,
        }
    }

    #[test]
    fn validates_command_and_authoritative_funding_ids() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        assert_eq!(
            use_case.check_command(&command(0, vec![1])),
            Err(SettleHyperliquidPerpFundingError::InvalidFundingBatchId)
        );
        assert_eq!(
            use_case.check_command(&command(1, vec![])),
            Err(SettleHyperliquidPerpFundingError::EmptyFundingIds)
        );

        let loaded = state(vec![funding(1, Decimal::ONE), funding(2, Decimal::ONE)]);
        assert_eq!(
            use_case.validate_state_given(&command(1, vec![2, 1]), &loaded),
            Err(SettleHyperliquidPerpFundingError::FundingIdsMismatch)
        );
        let mut already_settled = loaded;
        already_settled.settled_funding_ids.push(2);
        assert_eq!(
            use_case.validate_state_given(&command(1, vec![1, 2]), &already_settled),
            Err(SettleHyperliquidPerpFundingError::FundingAlreadySettled)
        );
    }

    #[test]
    fn propagates_domain_error_transparently() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let loaded = state(vec![funding(1, Decimal::ZERO)]);

        assert_eq!(
            use_case.validate_state_given(&command(1, vec![1]), &loaded),
            Err(SettleHyperliquidPerpFundingError::Settlement(
                SettleUserFundingBatchError::ZeroFundingAmount { funding_id: 1 }
            ))
        );
    }

    #[test]
    fn maps_pairs_and_projects_stable_unique_single_step_events() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let loaded = state(vec![funding(2, Decimal::from(3)), funding(1, Decimal::from(-1))]);
        let changes = use_case.compute_state_changed(&command(1, vec![2, 1]), &loaded).unwrap();

        assert_eq!(changes.changed_fundings.len(), 2);
        assert_eq!(changes.changed_positions.len(), 1);
        assert_eq!(changes.changed_margin_balances.len(), 1);
        assert_eq!(
            changes.changed_fundings[0].after.version(),
            changes.changed_fundings[0].before.version() + 1
        );
        assert_eq!(
            changes.changed_positions[0].after.version(),
            changes.changed_positions[0].before.version() + 1
        );
        assert_eq!(
            changes.changed_margin_balances[0].after.version(),
            changes.changed_margin_balances[0].before.version() + 1
        );

        let events = changes.to_replayable_events().unwrap();
        let expected = vec![
            (UserFunding::entity_type(), 2),
            (UserFunding::entity_type(), 1),
            (PerpPositionV3::entity_type(), 2),
            (BalanceV2::entity_type(), changes.changed_margin_balances[0].after.entity_id()),
        ];
        assert_eq!(
            events.iter().map(|event| (event.entity_type, event.entity_id)).collect::<Vec<_>>(),
            expected
        );
        let unique =
            events.iter().map(|event| (event.entity_type, event.entity_id)).collect::<HashSet<_>>();
        assert_eq!(unique.len(), events.len());
    }
}
