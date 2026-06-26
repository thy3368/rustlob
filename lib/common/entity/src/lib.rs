use std::sync::LazyLock;
use std::sync::atomic::AtomicU64;

mod entity;
mod entity_field_change;

pub use entity::{
    Entity, EntityLifecycleModel, EntityMethodBias, EntityMutationModel, FourColorArchetype,
    MiBusinessMethod, MiCausalRelation, MiCausalSourceMetadata, MiCreationStateMachine, MiFactType,
    MiStateMachine, ReplayableChanges,
};
pub use entity_field_change::{
    EntityChangeType, EntityFieldChange, EntityReplayableEvent, ReplayFieldChange,
};

static EVENT_SEQUENCE: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));

/// Error raised while tracking entity state into replayable events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityError {
    EntityIdMismatch { expected: String, actual: String },
    EntityIdParseError { entity_id: String, reason: String },
    EntityVersionMismatch { expected_next: u64, actual_next: u64 },
    FieldParseError { field: String, reason: String },
    NoChangesDetected,
    VersionOverflow { version: u64 },
    ClockError(String),
    Custom(String),
}

impl std::fmt::Display for EntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EntityIdMismatch { expected, actual } => {
                write!(f, "Entity ID mismatch: expected {}, got {}", expected, actual)
            }
            Self::EntityIdParseError { entity_id, reason } => {
                write!(f, "Failed to parse entity ID '{}': {}", entity_id, reason)
            }
            Self::EntityVersionMismatch { expected_next, actual_next } => {
                write!(
                    f,
                    "Entity version mismatch: expected next version {}, got {}",
                    expected_next, actual_next
                )
            }
            Self::FieldParseError { field, reason } => {
                write!(f, "Failed to parse field '{}': {}", field, reason)
            }
            Self::NoChangesDetected => write!(f, "No changes detected"),
            Self::VersionOverflow { version } => {
                write!(f, "Entity version overflow at {}", version)
            }
            Self::ClockError(reason) => write!(f, "Clock error: {}", reason),
            Self::Custom(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for EntityError {}

impl From<String> for EntityError {
    fn from(value: String) -> Self {
        Self::Custom(value)
    }
}

impl From<&str> for EntityError {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}
