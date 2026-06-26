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

/// 引用型实体变更：只暴露本次业务结果的 after 引用。
///
/// 适用于当前流程内继续读取 after 视图、但不想为大对象做 clone 的场景。
/// 它不承诺可直接持久化或投影 replay 事件。
#[derive(Debug, PartialEq, Eq)]
pub struct ChangedEntityRef<'a, E> {
    pub after: &'a E,
}

impl<'a, E> Copy for ChangedEntityRef<'a, E> {}

impl<'a, E> Clone for ChangedEntityRef<'a, E> {
    fn clone(&self) -> Self {
        *self
    }
}

/// 引用型实体变更对：同时暴露 before / after 的借用视图。
///
/// 仅用于流程内需要做即时对比、但仍不想持有 owned 快照的场景。
/// 如果需要稳定 diff、回放或持久化，应改为返回 owned changes。
#[derive(Debug, PartialEq, Eq)]
pub struct UpdatedEntityRefPair<'a, E> {
    pub before: &'a E,
    pub after: &'a E,
}

impl<'a, E> Copy for UpdatedEntityRefPair<'a, E> {}

impl<'a, E> Clone for UpdatedEntityRefPair<'a, E> {
    fn clone(&self) -> Self {
        *self
    }
}

/// 实体内部业务方法的显式状态机版契约。
///
/// 适合“实体有明确状态流转”的场景。`state()` 用于显式暴露当前状态视图，便于把
/// 状态校验和变化计算写成清晰的状态机逻辑。
///
/// 该 trait 走 owned changes 路径：`compute_changes()` 必须返回可稳定持有的 changes，
/// 供后续 replay、持久化、diff 或显式 before/after 快照比较使用。
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

/// 实体内部业务方法的借用型状态机契约。
///
/// 该 trait 面向“当前流程只想继续读取 after 状态、且不希望 clone 大对象”的场景。
/// 返回值允许同时借用 `self` 和 `cmd`，但不承诺 replay / 持久化语义。
///
/// ```rust
/// use entity::{ChangedEntityRef, MiStateMachineRef};
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct NonCloneBalance {
///     available: u64,
/// }
///
/// #[derive(Debug)]
/// struct FreezeCommand {
///     reason: &'static str,
/// }
///
/// #[derive(Debug)]
/// struct BalanceMachine {
///     after: NonCloneBalance,
/// }
///
/// #[derive(Debug, PartialEq, Eq)]
/// struct BorrowedChanges<'a> {
///     balance: ChangedEntityRef<'a, NonCloneBalance>,
///     reason: &'a str,
/// }
///
/// impl MiStateMachineRef for BalanceMachine {
///     type Command = FreezeCommand;
///     type State = NonCloneBalance;
///     type Error = ();
///     type Changes<'a>
///         = BorrowedChanges<'a>
///     where
///         Self: 'a,
///         Self::Command: 'a;
///
///     fn state(&self) -> &Self::State {
///         &self.after
///     }
///
///     fn compute_changes_ref<'a>(
///         &'a self,
///         cmd: &'a Self::Command,
///     ) -> Result<Self::Changes<'a>, Self::Error> {
///         Ok(BorrowedChanges {
///             balance: ChangedEntityRef { after: &self.after },
///             reason: cmd.reason,
///         })
///     }
/// }
///
/// let machine = BalanceMachine {
///     after: NonCloneBalance { available: 42 },
/// };
/// let cmd = FreezeCommand { reason: "risk_hold" };
/// let changes = machine.compute_changes_ref(&cmd).unwrap();
///
/// assert_eq!(changes.balance.after.available, 42);
/// assert_eq!(changes.reason, "risk_hold");
/// ```
///
/// ```rust,compile_fail
/// use entity::{ChangedEntityRef, ReplayableChanges};
///
/// fn project<C: ReplayableChanges>(_changes: &C) {}
///
/// fn borrowed_changes_are_not_replayable<'a, T>(changes: ChangedEntityRef<'a, T>) {
///     project(&changes);
/// }
/// ```
///
/// ```rust,compile_fail
/// use entity::{ChangedEntityRef, MiStateMachineRef};
///
/// #[derive(Debug)]
/// struct State {
///     value: u64,
/// }
///
/// #[derive(Debug)]
/// struct Command;
///
/// #[derive(Debug)]
/// struct Machine {
///     state: State,
/// }
///
/// #[derive(Debug)]
/// struct BorrowedChanges<'a> {
///     state: ChangedEntityRef<'a, State>,
/// }
///
/// impl MiStateMachineRef for Machine {
///     type Command = Command;
///     type State = State;
///     type Error = ();
///     type Changes<'a>
///         = BorrowedChanges<'a>
///     where
///         Self: 'a,
///         Self::Command: 'a;
///
///     fn state(&self) -> &Self::State {
///         &self.state
///     }
///
///     fn compute_changes_ref<'a>(
///         &'a self,
///         cmd: &'a Self::Command,
///     ) -> Result<Self::Changes<'a>, Self::Error> {
///         let _ = cmd;
///         Ok(BorrowedChanges {
///             state: ChangedEntityRef { after: &self.state },
///         })
///     }
/// }
///
/// fn leak(machine: &Machine, cmd: &Command) -> BorrowedChanges<'static> {
///     machine.compute_changes_ref(cmd).unwrap()
/// }
/// ```
pub trait MiStateMachineRef: Debug + Send + Sync {
    type Command;
    type State;
    type Error;
    type Changes<'a>
    where
        Self: 'a,
        Self::Command: 'a;

    fn state(&self) -> &Self::State;

    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_state_transition(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn compute_changes_ref<'a>(
        &'a self,
        _cmd: &'a Self::Command,
    ) -> Result<Self::Changes<'a>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use crate::{
        ChangedEntityRef, Entity, EntityError, EntityFieldChange, EntityReplayableEvent,
        MiStateMachine, MiStateMachineRef, ReplayableChanges, UpdatedEntityRefPair,
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

    #[derive(Debug, PartialEq, Eq)]
    struct NonCloneState {
        value: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct BorrowedChanges<'a> {
        state: ChangedEntityRef<'a, NonCloneState>,
        command_note: &'a str,
    }

    #[derive(Debug)]
    struct BorrowedMachine {
        state: NonCloneState,
    }

    impl MiStateMachineRef for BorrowedMachine {
        type Command = String;
        type State = NonCloneState;
        type Error = EntityError;
        type Changes<'a>
            = BorrowedChanges<'a>
        where
            Self: 'a,
            Self::Command: 'a;

        fn state(&self) -> &Self::State {
            &self.state
        }

        fn compute_changes_ref<'a>(
            &'a self,
            cmd: &'a Self::Command,
        ) -> Result<Self::Changes<'a>, Self::Error> {
            Ok(BorrowedChanges {
                state: ChangedEntityRef { after: &self.state },
                command_note: cmd.as_str(),
            })
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

    #[test]
    fn borrowed_state_machine_can_return_non_clone_state_refs() {
        let machine = BorrowedMachine { state: NonCloneState { value: "after".to_string() } };
        let cmd = "hold".to_string();

        let changes = machine.compute_changes_ref(&cmd).unwrap();

        assert_eq!(changes.state.after.value, "after");
        assert_eq!(changes.command_note, "hold");
    }

    #[test]
    fn updated_entity_ref_pair_exposes_before_and_after_views() {
        let before = NonCloneState { value: "before".to_string() };
        let after = NonCloneState { value: "after".to_string() };
        let pair = UpdatedEntityRefPair { before: &before, after: &after };

        assert_eq!(pair.before.value, "before");
        assert_eq!(pair.after.value, "after");
    }

    #[test]
    fn changed_entity_ref_is_copyable_because_it_only_borrows() {
        let state = NonCloneState { value: "after".to_string() };
        let changed = ChangedEntityRef { after: &state };
        let copied = changed;

        assert_eq!(copied.after.value, "after");
        assert_eq!(changed.after.value, "after");
    }
}
