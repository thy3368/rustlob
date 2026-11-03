use serde::{Deserialize, Serialize};

/// Example entity context 的用户标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[repr(transparent)]
pub struct UserId(pub u64);

impl From<u64> for UserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}



/// 定位一个已经存在的订单。
///
/// 现货改单支持用 Hyperliquid numeric `oid` 或 `cloid` 查找原订单。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderId {
    Oid(u64),
    Cloid(String),
}

impl Default for OrderId {
    fn default() -> Self {
        Self::Oid(0)
    }
}
