use std::borrow::Cow;
use std::sync::atomic::Ordering;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::{EVENT_SEQUENCE, EntityError};

/// Logical entity change kind used before encoding into compact replay tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityChangeType {
    Created,
    Updated,
    Deleted,
}

impl EntityChangeType {
    #[inline]
    pub const fn as_tag(self) -> u8 {
        match self {
            Self::Created => 0,
            Self::Updated => 1,
            Self::Deleted => 2,
        }
    }
}

/// Compact field change carried by an [`EntityReplayableEvent`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReplayFieldChange {
    pub field_name: [u8; 32],
    #[serde(with = "BigArray")]
    pub old_value: [u8; 64],
    pub old_value_len: u16,
    #[serde(with = "BigArray")]
    pub new_value: [u8; 64],
    pub new_value_len: u16,
    pub field_type: u8,
}

impl ReplayFieldChange {
    #[inline]
    pub fn new(field_name: [u8; 32], old_value: &[u8], new_value: &[u8], field_type: u8) -> Self {
        let mut old_buffer = [0u8; 64];
        let mut new_buffer = [0u8; 64];

        let old_value_len = old_value.len().min(old_buffer.len());
        let new_value_len = new_value.len().min(new_buffer.len());

        old_buffer[..old_value_len].copy_from_slice(&old_value[..old_value_len]);
        new_buffer[..new_value_len].copy_from_slice(&new_value[..new_value_len]);

        Self {
            field_name,
            old_value: old_buffer,
            old_value_len: old_value_len as u16,
            new_value: new_buffer,
            new_value_len: new_value_len as u16,
            field_type,
        }
    }

    #[inline]
    pub fn from_entity_field_change(change: &EntityFieldChange, field_type: u8) -> Self {
        Self::new(
            Self::field_name_from_str(change.field_name.as_ref()),
            change.old_value.as_bytes(),
            change.new_value.as_bytes(),
            field_type,
        )
    }

    #[inline]
    pub fn field_name_from_str(value: &str) -> [u8; 32] {
        let mut field_name = [0u8; 32];
        let bytes = value.as_bytes();
        let len = bytes.len().min(field_name.len());
        field_name[..len].copy_from_slice(&bytes[..len]);
        field_name
    }

    #[inline]
    pub fn field_name_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let end =
            self.field_name.iter().position(|byte| *byte == 0).unwrap_or(self.field_name.len());
        std::str::from_utf8(&self.field_name[..end])
    }

    #[inline]
    pub fn old_value_bytes(&self) -> &[u8] {
        &self.old_value[..self.old_value_len as usize]
    }

    #[inline]
    pub fn new_value_bytes(&self) -> &[u8] {
        &self.new_value[..self.new_value_len as usize]
    }
}

/// Entity event optimized for deterministic replay and optimistic locking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityReplayableEvent {
    pub timestamp: u64,
    pub sequence: u64,
    pub old_version: u64,
    pub new_version: u64,
    pub entity_id: i64,
    pub entity_type: u8,
    pub change_type: u8,
    pub field_changes: Vec<ReplayFieldChange>,
}

impl EntityReplayableEvent {
    #[inline]
    pub fn new(
        timestamp: u64,
        sequence: u64,
        old_version: u64,
        new_version: u64,
        entity_id: i64,
        entity_type: u8,
        change_type: u8,
    ) -> Self {
        Self {
            timestamp,
            sequence,
            old_version,
            new_version,
            entity_id,
            entity_type,
            change_type,
            field_changes: Vec::new(),
        }
    }

    #[inline]
    pub fn new_created(timestamp: u64, sequence: u64, entity_id: i64, entity_type: u8) -> Self {
        Self::new(
            timestamp,
            sequence,
            0,
            1,
            entity_id,
            entity_type,
            EntityChangeType::Created.as_tag(),
        )
    }

    #[inline]
    pub fn new_updated(
        timestamp: u64,
        sequence: u64,
        old_version: u64,
        new_version: u64,
        entity_id: i64,
        entity_type: u8,
    ) -> Self {
        Self::new(
            timestamp,
            sequence,
            old_version,
            new_version,
            entity_id,
            entity_type,
            EntityChangeType::Updated.as_tag(),
        )
    }

    #[inline]
    pub fn new_deleted(
        timestamp: u64,
        sequence: u64,
        old_version: u64,
        new_version: u64,
        entity_id: i64,
        entity_type: u8,
    ) -> Self {
        Self::new(
            timestamp,
            sequence,
            old_version,
            new_version,
            entity_id,
            entity_type,
            EntityChangeType::Deleted.as_tag(),
        )
    }

    #[inline]
    pub fn add_field_change(&mut self, field_change: ReplayFieldChange) {
        self.field_changes.push(field_change);
    }

    #[inline]
    pub fn is_created(&self) -> bool {
        self.change_type == EntityChangeType::Created.as_tag()
    }

    #[inline]
    pub fn is_updated(&self) -> bool {
        self.change_type == EntityChangeType::Updated.as_tag()
    }

    #[inline]
    pub fn is_deleted(&self) -> bool {
        self.change_type == EntityChangeType::Deleted.as_tag()
    }

    #[inline]
    pub fn verify_version(&self, expected_version: u64) -> bool {
        self.old_version == expected_version
    }

    #[inline]
    pub fn field_change_count(&self) -> usize {
        self.field_changes.len()
    }
}

#[inline]
pub fn next_sequence() -> u64 {
    EVENT_SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

#[inline]
pub fn current_timestamp() -> Result<u64, EntityError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| EntityError::ClockError(error.to_string()))?;
    let nanos = duration.as_nanos();
    u64::try_from(nanos).map_err(|error| EntityError::ClockError(error.to_string()))
}

/// Field-level state change produced by an entity diff.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityFieldChange {
    pub field_name: Cow<'static, str>,
    pub old_value: String,
    pub new_value: String,
}

impl EntityFieldChange {
    #[inline]
    pub fn new(
        field_name: impl Into<Cow<'static, str>>,
        old_value: impl Into<String>,
        new_value: impl Into<String>,
    ) -> Self {
        Self {
            field_name: field_name.into(),
            old_value: old_value.into(),
            new_value: new_value.into(),
        }
    }
}
