use crate::LobError;

pub trait LobRepo {
    type Lob;
    fn load_from_snapshot(&self, asset: u32) -> Result<Self::Lob, LobError>;
    fn save_to_snapshot(&self, lob: &Self::Lob) -> Result<(), LobError>;
}
