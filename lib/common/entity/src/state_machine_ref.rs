use std::fmt::Debug;

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
    use crate::{ChangedEntityRef, EntityError, MiStateMachineRef, UpdatedEntityRefPair};

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
