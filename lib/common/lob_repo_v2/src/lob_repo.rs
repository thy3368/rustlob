use base_types_v2::Symbol;

use crate::LobError;

pub trait LobRepo {
    type Lob;
    fn load_from_snapshot(&self, symbol: Symbol) -> Result<Self::Lob, LobError>;
    fn save_to_snapshot(&self, lob: &Self::Lob) -> Result<(), LobError>;
}
