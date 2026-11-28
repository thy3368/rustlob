use serde::{Deserialize, Serialize};

/// Example entity context 的业务参与方标识。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[repr(transparent)]
pub struct PartyId(pub String);

impl From<String> for PartyId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PartyId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl AsRef<str> for PartyId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PartyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
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
