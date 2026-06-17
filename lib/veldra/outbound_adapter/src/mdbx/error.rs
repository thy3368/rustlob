use thiserror::Error;

#[derive(Debug, Error)]
pub enum VeldraMdbxStorageError {
    #[error("failed to open storage: {0}")]
    Open(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to read storage: {0}")]
    Read(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to write storage: {0}")]
    Write(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to encode record: {0}")]
    Encode(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("failed to decode record: {0}")]
    Decode(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("block height {0} already exists")]
    DuplicateBlockHeight(u64),
    #[error("block hash '{0}' already exists")]
    DuplicateBlockHash(String),
    #[error(
        "execution body does not match header: header(height={header_height}, hash={header_hash}) body(height={body_height}, hash={body_hash})"
    )]
    HeaderBodyMismatch {
        header_height: u64,
        header_hash: String,
        body_height: u64,
        body_hash: String,
    },
    #[error("spot order snapshot identity changed during projection")]
    SpotOrderProjectionMismatch,
    #[error("balance snapshot identity changed during projection")]
    BalanceProjectionMismatch,
}
