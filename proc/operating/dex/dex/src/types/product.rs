#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductType {
    Spot,
    Perp,
    Option,
    Treasury,
}
