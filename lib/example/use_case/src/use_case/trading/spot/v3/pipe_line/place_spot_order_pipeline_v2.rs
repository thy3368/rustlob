use std::collections::HashMap;

use common_entity::{LogEntry, UseCaseV3};
use example_core_entity::{PlaceSpotOrderPipelineV2Cmd, SpotOrderV7};

use crate::use_case::trading::spot::v3::pipe::place_spot_order_pipeline_v2::{
    place_spot_order_pipeline_v2, validate_place_spot_order_pipeline_v2_command, validate_place_spot_order_pipeline_v2_state,
    PlaceSpotOrderPipelineV2Changes, PlaceSpotOrderPipelineV2Error,
};
use crate::use_case::trading::spot::v3::pipe::settle_matched_spot_trades::SpotOrderSettlementBalances;

/// 独立下单所需的 authoritative 撮合与结算快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderPipelineV2State {
    pub maker_orders: Vec<SpotOrderV7>,
    pub balances_by_order_id: HashMap<u64, SpotOrderSettlementBalances>,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceSpotOrderPipelineV2UseCase;

impl UseCaseV3 for PlaceSpotOrderPipelineV2UseCase {
    type Command = LogEntry<PlaceSpotOrderPipelineV2Cmd>;
    type StateGiven = PlaceSpotOrderPipelineV2State;
    type Error = PlaceSpotOrderPipelineV2Error;
    type StateChanged = PlaceSpotOrderPipelineV2Changes;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        validate_place_spot_order_pipeline_v2_command(&cmd.payload())?;
        if cmd.payload().party_id != common_entity::stable_string_entity_id(cmd.user_id()) {
            return Err(PlaceSpotOrderPipelineV2Error::InvalidInput(
                "command party id does not match log user identity".to_owned(),
            ));
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        validate_place_spot_order_pipeline_v2_state(
            &cmd.payload(),
            &state.maker_orders,
            &state.balances_by_order_id,
            state.trade_id_start,
            cmd.lsn(),
        )
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        place_spot_order_pipeline_v2(
            cmd.payload().clone(),
            &state.maker_orders,
            &state.balances_by_order_id,
            state.trade_id_start,
            state.maker_fee_bps,
            state.taker_fee_bps,
            cmd.lsn(),
            cmd.logical_ts(),
            cmd.topic(),
            cmd.partition(),
        )
    }
}
