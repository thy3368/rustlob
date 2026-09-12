use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    EntityReplayableEvent, MiStateMachineOwnedV2BeforeAfter, MiStateMachineV2,
    MiStateMachineV2Unchecked, ReplayableChanges,
};
use serde::{Deserialize, Serialize};

use super::spot_order_v2_use_case_family_v3::{
    PlaceSpotOrderV2AfterChangesV3, PlaceSpotOrderV2ChangesV3, PlaceSpotOrderV2CmdV3,
    PlaceSpotOrderV2TakerTemplateContextV3, SpotOrderV2AfterChangesV3, SpotOrderV2CaseChangesV3,
    SpotOrderV2CommandV3, SpotOrderV2GivenStateV3, SpotOrderV2UseCaseFamilyV3,
    SpotOrderV2UseCaseFamilyV3Error, build_place_spot_order_v2_taker_template_v3,
};
use crate::entity::account::balance_ledger_entry_v2::BalanceLedgerEntryV2;
use crate::entity::{Balance, SettlementTransferVoucher, SpotOrderV2, SpotTrade};

pub type PlaceSpotOrderV2Error = SpotOrderV2UseCaseFamilyV3Error;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlaceSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub tif: String,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2State {
    pub taker_order: SpotOrderV2,
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub fee_account_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2AfterChanges {
    pub taker_order_after: SpotOrderV2,
    pub maker_orders_after: Vec<SpotOrderV2>,
    pub balances_after: Vec<Balance>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV2Changes {
    pub updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_trades: Vec<SpotTrade>,
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceSpotOrderV2UseCase;

fn legacy_cmd(cmd: &PlaceSpotOrderV2Cmd) -> PlaceSpotOrderV2CmdV3 {
    PlaceSpotOrderV2CmdV3 {
        party_id: cmd.party_id.clone(),
        asset: cmd.asset,
        is_buy: cmd.is_buy,
        price: cmd.price.clone(),
        size: cmd.size.clone(),
        tif: cmd.tif.clone(),
        cloid: cmd.cloid.clone(),
    }
}

fn legacy_state(state: &PlaceSpotOrderV2State) -> SpotOrderV2GivenStateV3 {
    SpotOrderV2GivenStateV3::Place {
        taker_order: state.taker_order.clone(),
        maker_orders: state.maker_orders.clone(),
        settlement_balances: state.settlement_balances.clone(),
        base_asset_id: state.base_asset_id.clone(),
        quote_asset_id: state.quote_asset_id.clone(),
        fee_account_id: state.fee_account_id.clone(),
        maker_fee_bps: state.maker_fee_bps,
        taker_fee_bps: state.taker_fee_bps,
    }
}

impl ReplayableChanges for PlaceSpotOrderV2Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        PlaceSpotOrderV2ChangesV3 {
            updated_taker_order: self.updated_taker_order.clone(),
            updated_maker_orders: self.updated_maker_orders.clone(),
            updated_balances: self.updated_balances.clone(),
            created_trades: self.created_trades.clone(),
            created_vouchers: self.created_vouchers.clone(),
            created_balance_ledger_entries: self.created_balance_ledger_entries.clone(),
        }
        .to_replayable_events()
    }
}

impl MiStateMachineV2Unchecked for PlaceSpotOrderV2UseCase {
    type Command = PlaceSpotOrderV2Cmd;
    type GivenState = PlaceSpotOrderV2State;
    type Error = PlaceSpotOrderV2Error;
    type AfterChanges = PlaceSpotOrderV2AfterChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        SpotOrderV2UseCaseFamilyV3.pre_check_command(&SpotOrderV2CommandV3::Place(legacy_cmd(cmd)))
    }

    fn validate_against_given_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        SpotOrderV2UseCaseFamilyV3.validate_against_given_state(
            &SpotOrderV2CommandV3::Place(legacy_cmd(cmd)),
            &legacy_state(state),
        )
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        let SpotOrderV2AfterChangesV3::Place(after) = SpotOrderV2UseCaseFamilyV3
            .compute_after_changes(
                &SpotOrderV2CommandV3::Place(legacy_cmd(cmd)),
                &legacy_state(state),
            )?
        else {
            return Err(PlaceSpotOrderV2Error::BranchMismatch);
        };
        Ok(PlaceSpotOrderV2AfterChanges {
            taker_order_after: after.taker_order_after,
            maker_orders_after: after.maker_orders_after,
            balances_after: after.balances_after,
            created_trades: after.created_trades,
            created_vouchers: after.created_vouchers,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
}

impl MiStateMachineOwnedV2BeforeAfter for PlaceSpotOrderV2UseCase {
    type BeforeAfterChanges = PlaceSpotOrderV2Changes;

    fn merge_before_and_after(
        state: PlaceSpotOrderV2State,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
        let changes = SpotOrderV2UseCaseFamilyV3::merge_before_and_after(
            legacy_state(&state),
            SpotOrderV2AfterChangesV3::Place(PlaceSpotOrderV2AfterChangesV3 {
                taker_order_after: after.taker_order_after,
                maker_orders_after: after.maker_orders_after,
                balances_after: after.balances_after,
                created_trades: after.created_trades,
                created_vouchers: after.created_vouchers,
                created_balance_ledger_entries: after.created_balance_ledger_entries,
            }),
        )?;
        let SpotOrderV2CaseChangesV3::Place(changes) = changes else {
            return Err(PlaceSpotOrderV2Error::BranchMismatch);
        };
        Ok(PlaceSpotOrderV2Changes {
            updated_taker_order: changes.updated_taker_order,
            updated_maker_orders: changes.updated_maker_orders,
            updated_balances: changes.updated_balances,
            created_trades: changes.created_trades,
            created_vouchers: changes.created_vouchers,
            created_balance_ledger_entries: changes.created_balance_ledger_entries,
        })
    }
}

pub use super::spot_order_v2_use_case_family_v3::PlaceSpotOrderV2TakerTemplateContextV3 as PlaceSpotOrderV2TakerTemplateContext;

pub fn build_place_spot_order_v2_taker_template(
    cmd: &PlaceSpotOrderV2Cmd,
    context: PlaceSpotOrderV2TakerTemplateContext<'_>,
) -> Result<SpotOrderV2, PlaceSpotOrderV2Error> {
    build_place_spot_order_v2_taker_template_v3(&legacy_cmd(cmd), context)
}
