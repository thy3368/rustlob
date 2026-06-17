use super::error::VeldraMdbxStorageError;

pub fn encode_u64_be(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

pub fn encode_block_sequence_key(block_height: u64, sequence: u64) -> [u8; 16] {
    let mut key = [0u8; 16];
    key[..8].copy_from_slice(&block_height.to_be_bytes());
    key[8..].copy_from_slice(&sequence.to_be_bytes());
    key
}

pub fn encode_account_asset_key(account_id: &str, asset_id: &str) -> Vec<u8> {
    let mut key = Vec::with_capacity(account_id.len() + asset_id.len() + 1);
    key.extend_from_slice(account_id.as_bytes());
    key.push(0);
    key.extend_from_slice(asset_id.as_bytes());
    key
}

pub fn decode_u64_be(bytes: &[u8]) -> Result<u64, VeldraMdbxStorageError> {
    let array: [u8; 8] =
        bytes.try_into().map_err(|error| VeldraMdbxStorageError::Decode(Box::new(error)))?;
    Ok(u64::from_be_bytes(array))
}
