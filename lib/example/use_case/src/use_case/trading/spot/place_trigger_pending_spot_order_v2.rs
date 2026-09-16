use common_entity::{
    MiStateMachineOwnedV2Diff, MiStateMachineV2, MiStateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};

use super::spot_order_v2_use_case_family_v3::{
    PlaceTriggerPendingSpotOrderV2AfterChangesV3, PlaceTriggerPendingSpotOrderV2CmdV3,
    PlaceTriggerPendingSpotOrderV2TemplateContextV3, SpotOrderV2AfterChangesV3,
    SpotOrderV2CaseChangesV3, SpotOrderV2CommandV3, SpotOrderV2GivenStateV3,
    SpotOrderV2UseCaseFamilyV3, SpotOrderV2UseCaseFamilyV3Error,
    build_place_trigger_pending_spot_order_v2_template_v3,
};
use crate::entity::spot::spot_order_v2::SpotOrderV2;

pub type PlaceTriggerPendingSpotOrderV2Error = SpotOrderV2UseCaseFamilyV3Error;
pub type PlaceTriggerPendingSpotOrderV2TemplateContext =
    PlaceTriggerPendingSpotOrderV2TemplateContextV3;
pub type PlaceTriggerPendingSpotOrderV2AfterChanges = PlaceTriggerPendingSpotOrderV2AfterChangesV3;
pub type PlaceTriggerPendingSpotOrderV2Changes =
    super::spot_order_v2_use_case_family_v3::PlaceTriggerPendingSpotOrderV2ChangesV3;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaceTriggerPendingSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub is_buy: bool,
    pub trigger_price: String,
    pub price: String,
    pub size: String,
    pub tif: String,
    pub trigger_role: String,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceTriggerPendingSpotOrderV2State {
    pub order_template: SpotOrderV2,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceTriggerPendingSpotOrderV2UseCase;

fn legacy_cmd(cmd: &PlaceTriggerPendingSpotOrderV2Cmd) -> PlaceTriggerPendingSpotOrderV2CmdV3 {
    PlaceTriggerPendingSpotOrderV2CmdV3 {
        party_id: cmd.party_id.clone(),
        asset: cmd.asset,
        is_buy: cmd.is_buy,
        trigger_price: cmd.trigger_price.clone(),
        price: cmd.price.clone(),
        size: cmd.size.clone(),
        tif: cmd.tif.clone(),
        trigger_role: cmd.trigger_role.clone(),
        cloid: cmd.cloid.clone(),
    }
}

impl MiStateMachineV2Unchecked for PlaceTriggerPendingSpotOrderV2UseCase {
    type Command = PlaceTriggerPendingSpotOrderV2Cmd;
    type StateGiven = PlaceTriggerPendingSpotOrderV2State;
    type Error = PlaceTriggerPendingSpotOrderV2Error;
    type StateChanged = PlaceTriggerPendingSpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        SpotOrderV2UseCaseFamilyV3
            .check_command(&SpotOrderV2CommandV3::PlaceTriggerPending(legacy_cmd(cmd)))
    }
    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        SpotOrderV2UseCaseFamilyV3.validate_state_given(
            &SpotOrderV2CommandV3::PlaceTriggerPending(legacy_cmd(cmd)),
            &SpotOrderV2GivenStateV3::PlaceTriggerPending {
                order_template: state.order_template.clone(),
            },
        )
    }
    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let SpotOrderV2AfterChangesV3::PlaceTriggerPending(after) = SpotOrderV2UseCaseFamilyV3
            .compute_state_changed(
                &SpotOrderV2CommandV3::PlaceTriggerPending(legacy_cmd(cmd)),
                &SpotOrderV2GivenStateV3::PlaceTriggerPending {
                    order_template: state.order_template.clone(),
                },
            )?
        else {
            return Err(PlaceTriggerPendingSpotOrderV2Error::BranchMismatch);
        };
        Ok(after)
    }
}

impl MiStateMachineOwnedV2Diff for PlaceTriggerPendingSpotOrderV2UseCase {
    type StateDiff = PlaceTriggerPendingSpotOrderV2Changes;
    fn do_compute_state_diff(
        state: PlaceTriggerPendingSpotOrderV2State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let SpotOrderV2CaseChangesV3::PlaceTriggerPending(changes) =
            SpotOrderV2UseCaseFamilyV3::do_compute_state_diff(
                SpotOrderV2GivenStateV3::PlaceTriggerPending {
                    order_template: state.order_template,
                },
                SpotOrderV2AfterChangesV3::PlaceTriggerPending(after),
            )?
        else {
            return Err(PlaceTriggerPendingSpotOrderV2Error::BranchMismatch);
        };
        Ok(changes)
    }
}

pub fn build_place_trigger_pending_spot_order_v2_template(
    cmd: &PlaceTriggerPendingSpotOrderV2Cmd,
    context: PlaceTriggerPendingSpotOrderV2TemplateContext,
) -> Result<SpotOrderV2, PlaceTriggerPendingSpotOrderV2Error> {
    build_place_trigger_pending_spot_order_v2_template_v3(&legacy_cmd(cmd), context)
}
