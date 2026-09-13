use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    MiStateMachineOwnedV2Diff, MiStateMachineV2, MiStateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};

use super::spot_order_v2_use_case_family_v3::{
    SpotOrderV2AfterChangesV3, SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3,
    SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3, SpotOrderV2UseCaseFamilyV3Error,
    TriggerSpotOrderV2AfterChangesV3, TriggerSpotOrderV2ChangesV3, TriggerSpotOrderV2CmdV3,
};
use crate::entity::account::balance_ledger_entry_v2::BalanceLedgerEntryV2;
use crate::entity::spot::spot_order_v2::SpotOrderV2;
use crate::entity::{Balance, SettlementTransferVoucher, SpotTrade};

pub type TriggerSpotOrderV2Error = SpotOrderV2UseCaseFamilyV3Error;
pub type TriggerSpotOrderV2AfterChanges = TriggerSpotOrderV2AfterChangesV3;
pub type TriggerSpotOrderV2Changes = TriggerSpotOrderV2ChangesV3;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TriggerSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggerSpotOrderV2State {
    pub order: SpotOrderV2,
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct TriggerSpotOrderV2UseCase;

fn legacy_cmd(cmd: &TriggerSpotOrderV2Cmd) -> TriggerSpotOrderV2CmdV3 {
    TriggerSpotOrderV2CmdV3 {
        party_id: cmd.party_id.clone(),
        asset: cmd.asset,
        order_id: cmd.order_id.clone(),
    }
}

fn legacy_state(state: &TriggerSpotOrderV2State) -> SpotOrderV2GivenStateV3 {
    SpotOrderV2GivenStateV3::Trigger {
        order: state.order.clone(),
        maker_orders: state.maker_orders.clone(),
        settlement_balances: state.settlement_balances.clone(),
        base_asset_id: state.base_asset_id.clone(),
        quote_asset_id: state.quote_asset_id.clone(),
        fee_account_id: state.fee_account_id.clone(),
        maker_fee_bps: state.maker_fee_bps,
        taker_fee_bps: state.taker_fee_bps,
    }
}

impl MiStateMachineV2Unchecked for TriggerSpotOrderV2UseCase {
    type Command = TriggerSpotOrderV2Cmd;
    type GivenState = TriggerSpotOrderV2State;
    type Error = TriggerSpotOrderV2Error;
    type AfterChanges = TriggerSpotOrderV2AfterChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        SpotOrderV2UseCaseFamilyV3
            .pre_check_command(&SpotOrderV2CommandV3::Trigger(legacy_cmd(cmd)))
    }
    fn validate_against_given_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        SpotOrderV2UseCaseFamilyV3.validate_against_given_state(
            &SpotOrderV2CommandV3::Trigger(legacy_cmd(cmd)),
            &legacy_state(state),
        )
    }
    fn compute_after_state_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        let SpotOrderV2AfterChangesV3::Trigger(after) = SpotOrderV2UseCaseFamilyV3
            .compute_after_state(
                &SpotOrderV2CommandV3::Trigger(legacy_cmd(cmd)),
                &legacy_state(state),
            )?
        else {
            return Err(TriggerSpotOrderV2Error::BranchMismatch);
        };
        Ok(*after)
    }
}

impl common_entity::MiStateMachineOwnedV2Diff for TriggerSpotOrderV2UseCase {
    type DiffChanges = TriggerSpotOrderV2Changes;
    fn merge_before_and_after(
        state: TriggerSpotOrderV2State,
        after: Self::AfterChanges,
    ) -> Result<Self::DiffChanges, Self::Error> {
        let changes = SpotOrderV2UseCaseFamilyV3::merge_before_and_after(
            legacy_state(&state),
            SpotOrderV2AfterChangesV3::Trigger(Box::new(after)),
        )?;
        let SpotOrderV2CaseChangesV3::Trigger(changes) = changes else {
            return Err(TriggerSpotOrderV2Error::BranchMismatch);
        };
        Ok(*changes)
    }
}
