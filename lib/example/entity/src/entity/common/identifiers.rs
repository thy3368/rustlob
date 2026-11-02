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
