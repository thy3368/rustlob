use crate::entity::common::common_order::CommonOrder;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3 {
    pub comon_order: CommonOrder,
}
