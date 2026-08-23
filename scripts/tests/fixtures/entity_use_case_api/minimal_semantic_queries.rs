use common_entity::{Entity, EntityFieldChange, EntityUseCaseApiSurface, FieldDiff};

pub struct Voucher;

pub struct SettlementTransferSummary<'a> {
    pub account_id: &'a str,
    pub amount: u64,
}

impl Voucher {
    pub fn fee_amount_paid_by(&self, _account_id: &str) -> Option<u64> {
        Some(7)
    }

    pub fn amount_received_by_for_purpose(&self, _account_id: &str, _purpose: u8) -> Option<u64> {
        Some(11)
    }

    pub fn transfers_for_purpose(&self, _purpose: u8) -> Vec<SettlementTransferSummary<'_>> {
        Vec::new()
    }
}

impl FieldDiff for Voucher {
    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }
}

impl Entity for Voucher {
    type Id = u64;

    fn entity_id(&self) -> Self::Id {
        1
    }

    fn entity_type() -> u8 {
        1
    }

    fn use_case_api_surface() -> EntityUseCaseApiSurface {
        EntityUseCaseApiSurface::MinimalBusinessApi
    }

    fn entity_version(&self) -> u64 {
        1
    }
}
