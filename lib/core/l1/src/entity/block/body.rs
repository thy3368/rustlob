#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockBody<Tx> {
    pub transactions: Vec<Tx>,
}
