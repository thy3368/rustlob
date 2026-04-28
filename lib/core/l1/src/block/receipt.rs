use alloy_primitives::{Bloom, Log};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Receipt {
    pub success: bool,
    pub cumulative_gas_used: u64,
    pub logs: Vec<Log>,
    pub bloom: Bloom,
}
