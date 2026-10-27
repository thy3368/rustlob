use crate::entity::common::common_order::OrderBase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3 {
    pub comon_order: OrderBase,
}
