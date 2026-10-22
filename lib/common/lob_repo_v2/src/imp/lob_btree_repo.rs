use std::fs;
use std::path::PathBuf;

use super::lob_btree::LocalLobBtree;
use crate::LobError;
use crate::lob::LobOrder;
use crate::lob_repo::LobRepo;

pub struct LobBtreeRepository<O: LobOrder> {
    path: PathBuf,
    marker: std::marker::PhantomData<O>,
}

impl<O: LobOrder> LobBtreeRepository<O> {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), marker: std::marker::PhantomData }
    }
}

impl<O: LobOrder + serde::Serialize + for<'de> serde::Deserialize<'de>> LobRepo
    for LobBtreeRepository<O>
{
    type Lob = LocalLobBtree<O>;

    fn load_from_snapshot(&self, asset: u32) -> Result<Self::Lob, LobError> {
        match fs::read(&self.path) {
            Ok(bytes) => bincode::deserialize(&bytes).map_err(|e| LobError::Codec(e.to_string())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Ok(LocalLobBtree::new(asset))
            }
            Err(error) => Err(error.into()),
        }
    }

    fn save_to_snapshot(&self, lob: &Self::Lob) -> Result<(), LobError> {
        let bytes = bincode::serialize(lob).map_err(|e| LobError::Codec(e.to_string()))?;
        fs::write(&self.path, bytes)?;
        Ok(())
    }
}
