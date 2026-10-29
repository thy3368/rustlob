use example_core_entity::spot::spot_order_v3::SpotOrderV3;
use example_core_entity::{BalanceLedgerV2, BalanceV2, Open};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct FreezeSpotOrderV2Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderV2State {
    pub order: SpotOrderV3,
    pub balance: BalanceV2<Open>,
    // pub base_asset_id: String,
    // pub quote_asset_id: String,
    // pub maker_fee_bps: u64,
    // pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeSpotOrderV2AfterChanges {
    pub balance_after: BalanceV2<Open>,
    pub created_balance_ledger_entry: BalanceLedgerV2,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FreezeSpotOrderV2UseCase;

// impl StateMachineV2 for FreezeSpotOrderV2UseCase {
//     type Command = LogEntry<FreezeSpotOrderV2Cmd>;
// }
