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

/// Example entity context 的资产标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u32)]
#[derive(Default)]
pub enum AssetId {
    #[default]
    Usdt = 1,
    Btc = 2,
    Eth = 3,
}

impl AssetId {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Usdt => "USDT",
            Self::Btc => "BTC",
            Self::Eth => "ETH",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_uppercase().as_str() {
            "USDT" => Some(Self::Usdt),
            "BTC" => Some(Self::Btc),
            "ETH" => Some(Self::Eth),
            _ => None,
        }
    }
}

impl std::fmt::Display for AssetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<AssetId> for u32 {
    fn from(value: AssetId) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for AssetId {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Usdt),
            2 => Ok(Self::Btc),
            3 => Ok(Self::Eth),
            _ => Err(()),
        }
    }
}
