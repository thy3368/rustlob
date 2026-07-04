use std::sync::LazyLock;
use std::sync::atomic::AtomicU64;

mod entity;
mod entity_field_change;

mod use_case;
pub use use_case::command_use_case_v6::{CommandUseCase6, IssuedByParty};
pub use use_case::command_use_case_v6_runtime::{
    CommandEnvelope, CommandMeta, CommandUseCaseExecutionError, CommandUseCaseExecutor6,
    CommandUseCaseOutbound, CommandUseCaseOutboundPhase, EventProjectError, HandlerLatencyMetrics,
    ObserveHandlerLatency, UseCaseChanges, UseCaseReplyMapper,
};
pub use entity::{
    AggregateRole, Entity, EntityMutationModel, EntityUseCaseApiPolicy, EntityUseCaseApiSurface,
    FinancialClassification, FourColorArchetype, MiCausalRelation, MiCausalSourceMetadata,
};
pub use entity_field_change::{
    EntityChangeType, EntityFieldChange, EntityReplayableEvent, ReplayFieldChange,
};
/// 文档首选称呼：围绕主业务主题组织多个相关 use case 的多聚合编排抽象。
///
/// 这是 `core.use_case` 层的多聚合 `use-case family` 公开称呼，只用于跨聚合或多业务对象协调。
pub use use_case::state_machine_owned_v2::MiStateMachineOwnedV2 as MultiAggregateUseCase;
/// 文档首选称呼：带 replay / persist / audit case truth 扩展的多聚合 use case。
///
/// 这是 `core.use_case` 层多聚合 `use-case family` 的 before/after case truth 扩展称呼。
pub use use_case::state_machine_owned_v2::MiStateMachineOwnedV2BeforeAfter as MultiAggregateUseCaseBeforeAfter;
/// 文档首选称呼：多聚合 `use-case family` 的最低实现契约。
///
/// 这是 `core.use_case` 层多聚合 `use-case family` 的最低实现契约称呼。
pub use use_case::state_machine_owned_v2::MiStateMachineOwnedV2Unchecked as MultiAggregateUseCaseUnchecked;
pub use use_case::state_machine_owned_v2::{
    MiStateMachineOwnedV2, MiStateMachineOwnedV2BeforeAfter, MiStateMachineOwnedV2Unchecked,
};
pub use use_case::use_case_support::{CommandWithGivenState, ReplayableChanges, UpdatedEntityPair};

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
