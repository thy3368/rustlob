use common_entity::{Entity, EntityFieldChange, EntityUseCaseApiSurface, FieldDiff};

pub struct Voucher;

impl Voucher {
    pub fn new(legs: Vec<Leg>) -> Self {
        let _ = legs;
        Self
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

pub struct Leg;
