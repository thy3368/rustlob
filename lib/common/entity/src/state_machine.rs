use std::fmt::Debug;

use crate::{EntityError, EntityReplayableEvent};

/// 实体业务变更的最小回放契约。
///
/// `Changes` 是一次业务 case 产生的业务变化的唯一真相。实现方必须把同一个实体实例在本次
/// case 中的修改合并成至多一个 before/after pair；该 pair 表达“本业务 case 开始时的状态
/// -> 本业务 case 结束时的状态”，不能拆成多个中间步骤。
///
/// 对同一个实体实例，不允许在同一个 `Changes` 中重复创建多段 `UpdatedEntityPair` 来表达
/// 中间状态。中间步骤、资金腿、流水、审计过程、撮合明细等应建模为 append-only facts、
/// ledger records 或 created records，并由这些事实记录承载顺序和因果信息。
///
/// `to_replayable_events()` 可以基于 append-only records 投影出多条单步 replay events；
/// 但这种投影能力不能反向放宽 `Changes` 的约束，也不能让同一个实体实例在 `Changes`
/// 中被重复表达为多个 before/after pair。
///
/// 本 trait 只服务 owned changes：调用方拿到的是稳定快照，可继续做 replay、持久化、diff
/// 或 outbound publish。借用型 changes 不应默认实现本 trait；如果某条链路之后确实要落库，
/// 必须由上层显式 materialize 成 owned changes。
pub trait ReplayableChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError>;
}

/// 实体内部业务方法的显式状态机版契约。
///
/// 适合“实体有明确状态流转”的场景。`state()` 用于显式暴露当前状态视图，便于把
/// 状态校验和变化计算写成清晰的状态机逻辑。
///
/// 该 trait 走 owned changes 路径：`compute_changes()` 必须返回可稳定持有的 changes，
/// 供后续 replay、持久化、diff 或显式 before/after 快照比较使用。
///
/// 可以用copy 不能用clone
pub trait MiStateMachine: Clone + Debug + Send + Sync + 'static {
    type Command;
    type State;
    type Error;
    type Changes: ReplayableChanges;

    fn state(&self) -> &Self::State;

    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_state_transition(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_changes(&self, _cmd: &Self::Command) -> Result<Self::Changes, Self::Error>;
}
#[cfg(test)]
mod tests {
    use crate::{
        Entity, EntityError, EntityFieldChange, EntityReplayableEvent, MiStateMachine,
        ReplayableChanges,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestEntity {
        id: i64,
        value: String,
        version: u64,
    }

    impl Entity for TestEntity {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            7
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            let mut changes = Vec::new();
            if self.value != other.value {
                changes.push(EntityFieldChange::new("value", &self.value, &other.value));
            }
            changes
        }

        fn replay_field_type(field_name: &str) -> u8 {
            match field_name {
                "value" => 1,
                _ => 0,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestOwnedChanges {
        events: Vec<EntityReplayableEvent>,
    }

    impl ReplayableChanges for TestOwnedChanges {
        fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
            Ok(self.events.clone())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct OwnedMachine {
        state: TestEntity,
    }

    impl MiStateMachine for OwnedMachine {
        type Command = &'static str;
        type State = TestEntity;
        type Error = EntityError;
        type Changes = TestOwnedChanges;

        fn state(&self) -> &Self::State {
            &self.state
        }

        fn compute_changes(&self, _cmd: &Self::Command) -> Result<Self::Changes, Self::Error> {
            Ok(TestOwnedChanges { events: vec![self.state.track_create_event()?] })
        }
    }

    #[test]
    fn owned_state_machine_changes_remain_replayable() {
        let machine =
            OwnedMachine { state: TestEntity { id: 7, value: "created".to_string(), version: 1 } };

        let changes = machine.compute_changes(&"create").unwrap();
        let events = changes.to_replayable_events().unwrap();

        assert_eq!(events.len(), 1);
        assert!(events[0].is_created());
        assert_eq!(events[0].entity_id, 7);
    }
}
