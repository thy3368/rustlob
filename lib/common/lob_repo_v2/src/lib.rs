use thiserror::Error;

pub mod imp;
pub mod lob;
pub mod lob_repo;
pub use lob_repo::LobRepo;

#[derive(Debug, Error)]
pub enum LobError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid stored data: {0}")]
    Codec(String),
    #[error("invalid event: {0}")]
    Event(String),
    #[error("unsupported snapshot version {0}")]
    SnapshotVersion(u32),
}
