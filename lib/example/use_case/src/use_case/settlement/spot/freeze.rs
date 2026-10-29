use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_order_v3::SpotOrderV3;
use example_core_entity::{BalanceV2, Open};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FreezeSpotOrderV2Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderV2State {
    pub order: SpotOrderV3,
    pub balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderV2AfterChanges {
    pub balance_after: BalanceV2<Open>,
    // pub created_balance_ledger_entry: BalanceLedgerV2,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Freeze4SpotOrderV2UseCase;

impl StateMachineV2 for Freeze4SpotOrderV2UseCase {
    type Command = LogEntry<FreezeSpotOrderV2Cmd>;
    type StateGiven = FreezeSpotOrderV2State;
    type Error = ();
    type StateChanged = FreezeSpotOrderV2AfterChanges;

    fn compute_state_changed_only(
        &self,
        _cmd: &Self::Command,
        _given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        todo!()
    }
}
