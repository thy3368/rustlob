use bincode::Options;
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::error::VeldraMdbxStorageError;

pub const SCHEMA_VERSION_V1: u16 = 1;

pub fn encode_record<T: Serialize>(value: &T) -> Result<Vec<u8>, VeldraMdbxStorageError> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .serialize(value)
        .map_err(|error| VeldraMdbxStorageError::Encode(Box::new(error)))
}

pub fn decode_record<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, VeldraMdbxStorageError> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .deserialize(bytes)
        .map_err(|error| VeldraMdbxStorageError::Decode(Box::new(error)))
}
