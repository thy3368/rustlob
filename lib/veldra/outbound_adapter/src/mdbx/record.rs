use cmd_handler::EntityReplayableEvent;
use example_core::{Balance, SpotOrder};
use veldra_core::entity::{CommandEnvelope, NewBlock, ProductCommand};

use super::codec::{SCHEMA_VERSION_V1, decode_record, encode_record};
use super::error::VeldraMdbxStorageError;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BlockHeaderRecord {
    pub schema_version: u16,
    pub block_height: u64,
    pub parent_block_hash: String,
    pub commands_root: String,
    pub events_root: String,
    pub post_state_root: String,
    pub block_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredCommandEnvelope {
    pub schema_version: u16,
    pub block_height: u64,
    pub command_index: u64,
    pub command_id: String,
    pub account_id: String,
    pub nonce: u64,
    pub timestamp_ns: u64,
    pub command_kind: String,
    pub encoded_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredReplayableEvent {
    pub schema_version: u16,
    pub block_height: u64,
    pub event_sequence: u64,
    pub entity_id: i64,
    pub entity_type: u8,
    pub change_type: u8,
    pub encoded_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredSpotOrderSnapshot {
    pub schema_version: u16,
    pub block_height: u64,
    pub order_id: String,
    pub account_id: String,
    pub symbol: String,
    pub encoded_payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StoredBalanceSnapshot {
    pub schema_version: u16,
    pub block_height: u64,
    pub account_id: String,
    pub asset_id: String,
    pub encoded_payload: Vec<u8>,
}

impl From<&NewBlock> for BlockHeaderRecord {
    fn from(value: &NewBlock) -> Self {
        Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height: value.block_height,
            parent_block_hash: value.parent_block_hash.clone(),
            commands_root: value.commands_root.clone(),
            events_root: value.events_root.clone(),
            post_state_root: value.post_state_root.clone(),
            block_hash: value.block_hash.clone(),
        }
    }
}

impl From<BlockHeaderRecord> for NewBlock {
    fn from(value: BlockHeaderRecord) -> Self {
        Self {
            block_height: value.block_height,
            parent_block_hash: value.parent_block_hash,
            commands_root: value.commands_root,
            events_root: value.events_root,
            post_state_root: value.post_state_root,
            block_hash: value.block_hash,
        }
    }
}

impl StoredCommandEnvelope {
    pub fn from_command(
        block_height: u64,
        command_index: u64,
        command: &CommandEnvelope<ProductCommand>,
    ) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            command_index,
            command_id: command.command_id.clone(),
            account_id: command.account_id.clone(),
            nonce: command.nonce,
            timestamp_ns: command.timestamp_ns,
            command_kind: command.command.kind().to_string(),
            encoded_payload: encode_record(command)?,
        })
    }

    pub fn decode_command(
        &self,
    ) -> Result<CommandEnvelope<ProductCommand>, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

impl StoredReplayableEvent {
    pub fn from_event(
        block_height: u64,
        event: &EntityReplayableEvent,
    ) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            event_sequence: event.sequence,
            entity_id: event.entity_id,
            entity_type: event.entity_type,
            change_type: event.change_type,
            encoded_payload: encode_record(event)?,
        })
    }

    pub fn decode_event(&self) -> Result<EntityReplayableEvent, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

impl StoredSpotOrderSnapshot {
    pub fn from_order(
        block_height: u64,
        order: &SpotOrder,
    ) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            order_id: order.order_id.clone(),
            account_id: order.account_id.clone(),
            symbol: order.symbol.clone(),
            encoded_payload: encode_record(order)?,
        })
    }

    pub fn decode_order(&self) -> Result<SpotOrder, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}

impl StoredBalanceSnapshot {
    pub fn from_balance(
        block_height: u64,
        balance: &Balance,
    ) -> Result<Self, VeldraMdbxStorageError> {
        Ok(Self {
            schema_version: SCHEMA_VERSION_V1,
            block_height,
            account_id: balance.account_id.clone(),
            asset_id: balance.asset_id.clone(),
            encoded_payload: encode_record(balance)?,
        })
    }

    pub fn decode_balance(&self) -> Result<Balance, VeldraMdbxStorageError> {
        decode_record(&self.encoded_payload)
    }
}
