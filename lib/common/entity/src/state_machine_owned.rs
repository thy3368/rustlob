//! 单聚合根 / 单状态机业务推导工具。
//!
//! 这一组 trait 更适合表达“某一个实体或某一个聚合根内部”的状态迁移与 before/after 推导。
//! 如果一次业务目标需要在 `Command + GivenState -> Changes` 里分别驱动多个聚合根，
//! 应优先使用 [`crate::MiStateMachineOwnedV2`] 及其扩展 trait，把多聚合编排职责明确放在
//! `core.use_case` 语义层，而不是继续把它表述成单聚合状态机。

use std::fmt::Debug;

use crate::{Entity, EntityError, EntityReplayableEvent};

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

/// owned 实体变更：只暴露本次业务结果的 after 快照。
///
/// 适用于“只关心最终状态、但需要 stable owned 持有”的场景。
/// 它不要求实现 `ReplayableChanges`，因此不默认承诺 replay、持久化或 diff 语义。
///
/// 如果同一个状态机还要实现 `MiStateMachineOwnedBeforeAfter`，这里的 after 结果也仍然只需要
/// 表达 after 视图；before 真相由扩展 trait 的 `BeforeSnapshot` 单独捕获与合并。
///
/// ```rust
/// use entity::{
///     ChangedEntity, CommandWithGivenState, MiStateMachineOwned, MiStateMachineOwnedUnchecked,
/// };
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// enum WalletStatus {
///     Ready,
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// enum WalletCommand {
///     Deposit { amount: u64 },
///     Withdraw { amount: u64 },
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct WalletAccount {
///     available: u64,
/// }
///
/// impl CommandWithGivenState for WalletCommand {
///     type GivenState = WalletAccount;
/// }
///
/// #[derive(Debug, Clone)]
/// struct WalletMachine {
///     state: WalletStatus,
/// }
///
/// impl MiStateMachineOwnedUnchecked for WalletMachine {
///     type Command<'a> = WalletCommand
///     where
///         Self: 'a;
///     type State = WalletStatus;
///     type Error = ();
///     type AfterChanges = ChangedEntity<WalletAccount>;
///
///     fn state(&self) -> &Self::State {
///         &self.state
///     }
///
///     fn validate_state_transition<'a>(
///         &self,
///         cmd: &Self::Command<'a>,
///         given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
///     ) -> Result<(), Self::Error> {
///         if given_state.available == 0 {
///             return Err(());
///         }
///         match cmd {
///             WalletCommand::Deposit { .. } | WalletCommand::Withdraw { .. } => Ok(()),
///         }
///     }
///
///     fn compute_after_changes_unchecked<'a>(
///         &self,
///         cmd: &Self::Command<'a>,
///         given_state: &WalletAccount,
///     ) -> Result<Self::AfterChanges, Self::Error> {
///         let available = match cmd {
///             WalletCommand::Deposit { amount } => given_state.available + amount,
///             WalletCommand::Withdraw { amount } => given_state.available - amount,
///         };
///         Ok(ChangedEntity {
///             after: WalletAccount { available },
///         })
///     }
/// }
///
/// let machine = WalletMachine {
///     state: WalletStatus::Ready,
/// };
///
/// let given_state = WalletAccount { available: 42 };
/// let changed = machine
///     .compute_after_changes(
///         &WalletCommand::Deposit { amount: 10 },
///         &given_state,
///     )
///     .unwrap();
/// assert_eq!(machine.state(), &WalletStatus::Ready);
/// assert_eq!(changed.after.available, 52);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedEntity<E> {
    pub after: E,
}

/// 单个实体实例的一次最终 before/after 更新真相。
///
/// 语义等价于 `cmd_handler` 中的 `UpdatedEntityPair<E>`，但定义在 `entity` crate 内部，
/// 供 owned 状态机本地使用，避免反向依赖 use case 框架层。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntityPair<E> {
    pub before: E,
    pub after: E,
}

impl<E> UpdatedEntityPair<E> {
    #[inline]
    pub fn new(before: E, after: E) -> Self {
        Self { before, after }
    }
}

impl<E> UpdatedEntityPair<E>
where
    E: Entity,
{
    #[inline]
    pub fn to_replayable_event(&self) -> Result<EntityReplayableEvent, EntityError> {
        self.after.track_update_event_from(&self.before)
    }
}

/// 同一种实体类型在一次业务 case 中被更新的 1-n 个最终 before/after 真相。
///
/// 该类型保证“至少一个 pair”，并要求每个 pair 都表达某个实体实例从 case 开始到 case 结束
/// 的最终 before/after，不允许拆成多个中间态 pair。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdatedEntities<E> {
    pub first: UpdatedEntityPair<E>,
    pub rest: Vec<UpdatedEntityPair<E>>,
}

impl<E> UpdatedEntities<E> {
    #[inline]
    pub fn new(first: UpdatedEntityPair<E>, rest: Vec<UpdatedEntityPair<E>>) -> Self {
        Self { first, rest }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &UpdatedEntityPair<E>> {
        std::iter::once(&self.first).chain(self.rest.iter())
    }

    #[inline]
    pub fn len(&self) -> usize {
        1 + self.rest.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl<E> UpdatedEntities<E>
where
    E: Entity,
{
    #[inline]
    pub fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        self.iter().map(UpdatedEntityPair::to_replayable_event).collect()
    }
}

/// 命令自己声明执行该命令时，外部必须额外提供的输入类型。
///
/// 这是 `Command -> GivenState` 的配对契约，尤其用于 enum / command family 场景。
/// 它保留命令族需要的外部 authoritative state 绑定，避免把多个动作压成一个松散公共超集。
/// `GivenState` 不是状态机内部 `self.state()` 的别名，它表示其它 entity、多个 entity 组合，
/// 或业务上下文输入。默认入口只会只读借用它；真正需要可变工作态时，应由实现者在内部显式
/// clone / move 到局部 working state。
pub trait CommandWithGivenState {
    type GivenState;
}

/// owned 状态机实现者的最小契约：读取当前状态值，并结合外部 `GivenState` 推导 after-only
/// owned changes。
///
/// 它只要求实现 after 结果计算，不默认要求 replay、持久化、diff 或审计语义。
///
/// 这里有三层语义需要显式区分：
/// - `self`：状态机实例本身
/// - `self.state()`：状态机当前持有的状态值，例如 `OrderStatus`
/// - `Command::GivenState`：本次执行命令时外部额外传入的 entity / 上下文输入
///
/// owned 状态机的校验与计算，都是基于“当前状态值 + `Command::GivenState`”来推导结果。
/// `state()` 不是 `GivenState` 的别名，也不应被暗当成外部 authoritative entity。
///
/// 如果该状态机还实现 `MiStateMachineOwnedBeforeAfter`，这里也不要求 `AfterChanges`
/// 自带 before 信息。默认 before/after 路径会先从 `GivenState` 捕获 `BeforeSnapshot`，
/// 再复用 `MiStateMachineOwned::compute_after_changes()` 计算 after，并在扩展 trait 中完成合并。
///
/// 实现者只能提供 `compute_after_changes_unchecked()` 里的纯业务 after 计算。
/// `pre_check_command()` 与 `validate_state_transition()` 的调用链由
/// `MiStateMachineOwned` 的默认入口统一保证，避免实现者自行决定是否复用 hook。
pub trait MiStateMachineOwnedUnchecked: Clone + Debug + Send + Sync {
    /// 业务命令类型。
    type Command<'a>: CommandWithGivenState
    where
        Self: 'a;
    /// 状态机当前持有的状态值类型，例如 `OrderStatus`。
    ///
    /// 它表示 `self.state()` 返回的内容，而不是外部输入 entity 本身。
    type State;
    /// 业务错误类型。
    type Error;
    /// 本次业务 changes 的 stable owned 结果。
    ///
    /// 对纯 after-only 状态机，它可以是 `ChangedEntity<E>` 一类的简化 after 视图；
    /// 对同时实现 before/after 扩展的状态机，它也只需要承载 after 结果本身，
    /// 不要求默认自带 before 真相。
    type AfterChanges;

    /// 返回状态机当前持有的状态值。
    fn state(&self) -> &Self::State;

    fn pre_check_command<'a>(&self, _cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 基于外部 `GivenState` 校验本次状态迁移是否合法。
    fn validate_state_transition<'a>(
        &self,
        _cmd: &Self::Command<'a>,
        _given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 在已完成 command pre-check 与状态迁移校验后，基于借用的 `GivenState` 计算 stable
    /// owned after changes。
    ///
    /// 该方法不是上层应直接依赖的业务入口。
    /// 上层应始终通过 `MiStateMachineOwned::compute_after_changes()` 进入统一链路：
    /// `pre_check_command() -> validate_state_transition() -> compute_after_changes_unchecked()`
    fn compute_after_changes_unchecked<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error>;
}

/// owned 状态机的稳定对外入口：统一执行 hook 链路后，再进入纯业务 after 计算。
///
/// 该 trait 由 blanket impl 自动提供，外部实现者不需要也不能单独实现它。
/// 这让 `compute_after_changes()` 的编排顺序固定下来，避免绕过 hook。
/// 对外应只走 `compute_after_changes()` / `compute_before_after_changes()`，不要直接调用
/// `compute_after_changes_unchecked()`。
pub trait MiStateMachineOwned: MiStateMachineOwnedUnchecked {
    /// 基于“当前状态值 + 外部 `GivenState`”计算 stable owned after changes。
    ///
    /// 这是 after 结果计算的唯一稳定入口。
    /// 如果还需要 replayable before/after changes，应由
    /// `MiStateMachineOwnedBeforeAfter::compute_before_after_changes()` 默认复用本方法。
    fn compute_after_changes<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        self.pre_check_command(cmd)?;
        self.validate_state_transition(cmd, given_state)?;
        self.compute_after_changes_unchecked(cmd, given_state)
    }
}

impl<T> MiStateMachineOwned for T where T: MiStateMachineOwnedUnchecked {}

/// owned 状态机的可选扩展契约：在 after-only 基础上，额外提供 replayable before/after changes。
///
/// 只有需要 replay、持久化、diff、审计等稳定 before/after 事实的状态机，才需要实现这个 trait。
/// 不需要这些能力的 owned 状态机，只实现 `MiStateMachineOwnedUnchecked` 即可；
/// `MiStateMachineOwned` 的稳定入口会自动可用。
///
/// `UpdatedEntityPair<E>` 与 `UpdatedEntities<E>` 是本 crate 内推荐的单实体 / 多实体
/// before/after 默认结果形状，适合直接承载 case 级最终真相。
///
/// `BeforeAfterChanges` 表示本次业务 case 内 1-n 个被更新 entity 的最终 before/after 真相。
/// 同一个实体实例在一次 `BeforeAfterChanges` 里仍然只能表达为单个最终 before/after pair，
/// 不允许拆分中间态。中间步骤应由 append-only facts、ledger records 或审计记录表达。
///
/// 默认实现会先从借用的 `GivenState` 捕获 before 快照，再复用
/// `MiStateMachineOwned::compute_after_changes()` 计算 after，最后把两者合并成
/// `BeforeAfterChanges`。
///
/// 因此实现者通常只需要：
/// - 在 `compute_after_changes_unchecked()` 里实现一次业务规则，产出 after 结果
/// - 用 `BeforeSnapshot` 表达 case 级 before 真相；它可以是 `GivenState` 的子集、重组结果、
///   或异构多实体快照，不要求与 `GivenState` 同型
/// - 在 `merge_before_and_after()` 里把 before 快照与 after 结果稳定配对成最终 1-n pair 集合
///
/// ```rust
/// use entity::{
///     CommandWithGivenState, Entity, EntityError, EntityFieldChange, MiStateMachineOwned,
///     MiStateMachineOwnedBeforeAfter, MiStateMachineOwnedUnchecked, ReplayableChanges,
///     UpdatedEntities, UpdatedEntityPair,
/// };
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// enum OrderStatus {
///     Open,
///     Cancelled,
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct Order {
///     id: i64,
///     status: OrderStatus,
///     version: u64,
/// }
///
/// impl Entity for Order {
///     type Id = i64;
///
///     fn entity_id(&self) -> Self::Id {
///         self.id
///     }
///
///     fn entity_type() -> u8 {
///         11
///     }
///
///     fn entity_version(&self) -> u64 {
///         self.version
///     }
///
///     fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
///         if self.status == other.status {
///             Vec::new()
///         } else {
///             vec![EntityFieldChange::new(
///                 "status",
///                 format!("{:?}", self.status),
///                 format!("{:?}", other.status),
///             )]
///         }
///     }
///
///     fn replay_field_type(field_name: &str) -> u8 {
///         match field_name {
///             "status" => 1,
///             _ => 0,
///         }
///     }
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct Balance {
///     id: i64,
///     available: i64,
///     version: u64,
/// }
///
/// impl Entity for Balance {
///     type Id = i64;
///
///     fn entity_id(&self) -> Self::Id {
///         self.id
///     }
///
///     fn entity_type() -> u8 {
///         12
///     }
///
///     fn entity_version(&self) -> u64 {
///         self.version
///     }
///
///     fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
///         if self.available == other.available {
///             Vec::new()
///         } else {
///             vec![EntityFieldChange::new(
///                 "available",
///                 self.available.to_string(),
///                 other.available.to_string(),
///             )]
///         }
///     }
///
///     fn replay_field_type(field_name: &str) -> u8 {
///         match field_name {
///             "available" => 1,
///             _ => 0,
///         }
///     }
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct AccountContext {
///     can_cancel: bool,
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct CancelOrder;
///
/// impl CommandWithGivenState for CancelOrder {
///     type GivenState = (Order, [Balance; 2], AccountContext);
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct AfterChanges {
///     order_after: Order,
///     balance_afters: [Balance; 2],
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct BeforeSnapshot {
///     order_before: Order,
///     balance_befores: [Balance; 2],
/// }
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct BeforeAfterChanges {
///     updated_order: UpdatedEntityPair<Order>,
///     updated_balances: UpdatedEntities<Balance>,
/// }
///
/// impl ReplayableChanges for BeforeAfterChanges {
///     fn to_replayable_events(
///         &self,
///     ) -> Result<Vec<entity::EntityReplayableEvent>, EntityError> {
///         let mut events = vec![self.updated_order.to_replayable_event()?];
///         events.extend(self.updated_balances.to_replayable_events()?);
///         Ok(events)
///     }
/// }
///
/// #[derive(Debug, Clone)]
/// struct Machine {
///     state: OrderStatus,
/// }
///
/// impl MiStateMachineOwnedUnchecked for Machine {
///     type Command<'a> = CancelOrder
///     where
///         Self: 'a;
///     type State = OrderStatus;
///     type Error = EntityError;
///     type AfterChanges = AfterChanges;
///
///     fn state(&self) -> &Self::State {
///         &self.state
///     }
///
///     fn compute_after_changes_unchecked<'a>(
///         &self,
///         cmd: &Self::Command<'a>,
///         given_state: &(Order, [Balance; 2], AccountContext),
///     ) -> Result<Self::AfterChanges, Self::Error> {
///         let _ = cmd;
///         let (order_before, [cash_before, fee_before], account) = given_state;
///         if !account.can_cancel {
///             return Err(EntityError::Custom("account cannot cancel order".to_string()));
///         }
///
///         let order_after = Order {
///             id: order_before.id,
///             status: OrderStatus::Cancelled,
///             version: order_before.version + 1,
///         };
///         let cash_after = Balance {
///             id: cash_before.id,
///             available: cash_before.available + 100,
///             version: cash_before.version + 1,
///         };
///         let fee_after = Balance {
///             id: fee_before.id,
///             available: fee_before.available - 2,
///             version: fee_before.version + 1,
///         };
///
///         Ok(AfterChanges {
///             order_after,
///             balance_afters: [cash_after, fee_after],
///         })
///     }
/// }
///
/// impl MiStateMachineOwnedBeforeAfter for Machine {
///     type BeforeAfterChanges = BeforeAfterChanges;
///     type BeforeSnapshot = BeforeSnapshot;
///
///     fn capture_before(
///         &self,
///         given_state: &(Order, [Balance; 2], AccountContext),
///     ) -> Self::BeforeSnapshot {
///         let (order_before, balance_befores, _account) = given_state;
///         BeforeSnapshot {
///             order_before: order_before.clone(),
///             balance_befores: balance_befores.clone(),
///         }
///     }
///
///     fn merge_before_and_after(
///         before: Self::BeforeSnapshot,
///         after: Self::AfterChanges,
///     ) -> Result<Self::BeforeAfterChanges, Self::Error> {
///         let BeforeSnapshot {
///             order_before,
///             balance_befores: [cash_before, fee_before],
///         } = before;
///         let AfterChanges {
///             order_after,
///             balance_afters: [cash_after, fee_after],
///         } = after;
///
///         Ok(BeforeAfterChanges {
///             updated_order: UpdatedEntityPair::new(order_before, order_after),
///             updated_balances: UpdatedEntities::new(
///                 UpdatedEntityPair::new(cash_before, cash_after),
///                 vec![UpdatedEntityPair::new(fee_before, fee_after)],
///             ),
///         })
///     }
/// }
///
/// let machine = Machine {
///     state: OrderStatus::Open,
/// };
///
/// let given_state = (
///     Order {
///         id: 41,
///         status: OrderStatus::Open,
///         version: 7,
///     },
///     [
///         Balance {
///             id: 1001,
///             available: 42,
///             version: 9,
///         },
///         Balance {
///             id: 1002,
///             available: 5,
///             version: 3,
///         },
///     ],
///     AccountContext { can_cancel: true },
/// );
///
/// let changes = machine
///     .compute_before_after_changes(
///         &CancelOrder,
///         &given_state,
///     )
///     .unwrap();
///
/// assert_eq!(machine.state(), &OrderStatus::Open);
/// assert_eq!(changes.updated_order.after.status, OrderStatus::Cancelled);
/// assert_eq!(changes.updated_balances.len(), 2);
/// assert_eq!(changes.to_replayable_events().unwrap().len(), 3);
/// ```
///
/// ```rust,compile_fail
/// use entity::{
///     CommandWithGivenState, EntityError, MiStateMachineOwnedBeforeAfter,
///     MiStateMachineOwnedUnchecked, ReplayableChanges,
/// };
///
/// #[derive(Debug, Clone)]
/// struct CancelOrder;
///
/// impl CommandWithGivenState for CancelOrder {
///     type GivenState = ();
/// }
///
/// #[derive(Debug, Clone)]
/// struct AfterChanges;
///
/// #[derive(Debug, Clone)]
/// struct Machine;
///
/// impl MiStateMachineOwnedUnchecked for Machine {
///     type Command = CancelOrder;
///     type State = ();
///     type Error = EntityError;
///     type AfterChanges = AfterChanges;
///
///     fn state(&self) -> &Self::State {
///         &()
///     }
///
///     fn compute_after_changes_unchecked(
///         &self,
///         _cmd: &Self::Command,
///         _given_state: &(),
///     ) -> Result<Self::AfterChanges, Self::Error> {
///         Ok(AfterChanges)
///     }
/// }
///
/// impl MiStateMachineOwnedBeforeAfter for Machine {
///     type BeforeAfterChanges = AfterChanges;
/// }
/// ```
///
/// ```rust,compile_fail
/// use entity::{
///     CommandWithGivenState, EntityError, MiStateMachineOwnedBeforeAfter,
///     MiStateMachineOwnedUnchecked,
/// };
///
/// #[derive(Debug, Clone)]
/// struct CancelOrder;
///
/// impl CommandWithGivenState for CancelOrder {
///     type GivenState = ();
/// }
///
/// #[derive(Debug, Clone)]
/// struct BeforeSnapshot;
///
/// #[derive(Debug, Clone)]
/// struct NotReplayableChanges;
///
/// #[derive(Debug, Clone)]
/// struct Machine;
///
/// impl MiStateMachineOwnedUnchecked for Machine {
///     type Command = CancelOrder;
///     type State = ();
///     type Error = EntityError;
///     type AfterChanges = ();
///
///     fn state(&self) -> &Self::State {
///         &()
///     }
///
///     fn compute_after_changes_unchecked(
///         &self,
///         _cmd: &Self::Command,
///         _given_state: &(),
///     ) -> Result<Self::AfterChanges, Self::Error> {
///         Ok(())
///     }
/// }
///
/// impl MiStateMachineOwnedBeforeAfter for Machine {
///     type BeforeAfterChanges = NotReplayableChanges;
///     type BeforeSnapshot = BeforeSnapshot;
///
///     fn capture_before(&self, _given_state: &()) -> Self::BeforeSnapshot {
///         BeforeSnapshot
///     }
///
///     fn merge_before_and_after(
///         _before: Self::BeforeSnapshot,
///         _after: Self::AfterChanges,
///     ) -> Result<Self::BeforeAfterChanges, Self::Error> {
///         Ok(NotReplayableChanges)
///     }
/// }
/// ```
///
/// ```rust,compile_fail
/// use entity::{
///     ChangedEntity, CommandWithGivenState, MiStateMachineOwned, MiStateMachineOwnedUnchecked,
/// };
///
/// #[derive(Debug, Clone)]
/// struct Deposit;
///
/// impl CommandWithGivenState for Deposit {
///     type GivenState = ();
/// }
///
/// #[derive(Debug, Clone)]
/// struct Machine;
///
/// impl MiStateMachineOwnedUnchecked for Machine {
///     type Command = Deposit;
///     type State = ();
///     type Error = ();
///     type AfterChanges = ChangedEntity<()>;
///
///     fn state(&self) -> &Self::State {
///         &()
///     }
///
///     fn compute_after_changes_unchecked(
///         &self,
///         _cmd: &Self::Command,
///         _given_state: &(),
///     ) -> Result<Self::AfterChanges, Self::Error> {
///         Ok(ChangedEntity { after: () })
///     }
/// }
///
/// impl MiStateMachineOwned for Machine {}
/// ```
pub trait MiStateMachineOwnedBeforeAfter: MiStateMachineOwned {
    /// 最终可 replay 的 before/after changes。
    type BeforeAfterChanges: ReplayableChanges;

    /// 从借用的 `GivenState` 捕获本次 case 需要保留的 before 真相。
    ///
    /// 它不要求等于 `GivenState`，实现者可以按业务只保留必要子集，或重组为异构快照。
    type BeforeSnapshot;

    /// 在 `compute_after_changes()` 借用 `GivenState` 期间，先捕获 before 快照。
    fn capture_before(
        &self,
        given_state: &<Self::Command<'_> as CommandWithGivenState>::GivenState,
    ) -> Self::BeforeSnapshot;

    /// 把 case 级 before 快照与 after 结果合并成最终 replayable before/after changes。
    fn merge_before_and_after(
        before: Self::BeforeSnapshot,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error>;

    /// 基于“当前状态值 + 外部 `GivenState`”计算可 replay 的 before/after changes。
    ///
    /// 默认实现会先捕获 before，再调用 `MiStateMachineOwned::compute_after_changes()`
    /// 复用 after 计算，最后把两者合并成完整的 `BeforeAfterChanges`。
    /// 这确保 hook 只执行一条固定链路，业务规则也只实现一遍，避免两套 API 语义漂移。
    fn compute_before_after_changes<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
        let before = self.capture_before(given_state);
        let after = <Self as MiStateMachineOwned>::compute_after_changes(self, cmd, given_state)?;
        Self::merge_before_and_after(before, after)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        CommandWithGivenState, Entity, EntityError, EntityFieldChange, EntityReplayableEvent,
        MiStateMachineOwned, MiStateMachineOwnedBeforeAfter, MiStateMachineOwnedUnchecked,
        ReplayableChanges, UpdatedEntities, UpdatedEntityPair,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestStatus {
        Open,
        Applied,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestOrder {
        id: i64,
        value: String,
        version: u64,
        status: TestStatus,
    }

    impl Entity for TestOrder {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            9
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
    struct TestBalance {
        id: i64,
        available: i64,
        version: u64,
    }

    impl Entity for TestBalance {
        type Id = i64;

        fn entity_id(&self) -> Self::Id {
            self.id
        }

        fn entity_type() -> u8 {
            10
        }

        fn entity_version(&self) -> u64 {
            self.version
        }

        fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
            let mut changes = Vec::new();
            if self.available != other.available {
                changes.push(EntityFieldChange::new(
                    "available",
                    self.available.to_string(),
                    other.available.to_string(),
                ));
            }
            changes
        }

        fn replay_field_type(field_name: &str) -> u8 {
            match field_name {
                "available" => 1,
                _ => 0,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum HookError {
        PreCheckRejected,
        InvalidTransition,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum FamilyState {
        Draft,
        Live,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum FamilyCommand {
        Open { opening_value: &'static str },
        Close { closing_value: &'static str },
        RejectInPreCheck,
    }

    impl CommandWithGivenState for FamilyCommand {
        type GivenState = FamilyGivenState;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FamilyGivenState {
        state: FamilyState,
        value: &'static str,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct FamilyAfterChanges {
        next_state: FamilyState,
        next_value: String,
    }

    #[derive(Debug, Clone, Default)]
    struct FamilyMachineLog(std::sync::Arc<std::sync::Mutex<Vec<&'static str>>>);

    impl FamilyMachineLog {
        fn push(&self, marker: &'static str) {
            self.0.lock().unwrap().push(marker);
        }

        fn snapshot(&self) -> Vec<&'static str> {
            self.0.lock().unwrap().clone()
        }
    }

    #[derive(Debug, Clone)]
    struct FamilyMachine {
        state: FamilyState,
        log: FamilyMachineLog,
    }

    impl FamilyMachine {
        fn new(state: FamilyState) -> Self {
            Self { state, log: FamilyMachineLog::default() }
        }
    }

    impl MiStateMachineOwnedUnchecked for FamilyMachine {
        type Command<'a>
            = FamilyCommand
        where
            Self: 'a;
        type State = FamilyState;
        type Error = HookError;
        type AfterChanges = FamilyAfterChanges;

        fn state(&self) -> &Self::State {
            &self.state
        }

        fn pre_check_command<'a>(&self, cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
            self.log.push("pre_check");
            match cmd {
                FamilyCommand::RejectInPreCheck => Err(HookError::PreCheckRejected),
                FamilyCommand::Open { opening_value } => {
                    if opening_value.is_empty() {
                        return Err(HookError::InvalidTransition);
                    }
                    Ok(())
                }
                FamilyCommand::Close { closing_value } => {
                    if closing_value.is_empty() {
                        return Err(HookError::InvalidTransition);
                    }
                    Ok(())
                }
            }
        }

        fn validate_state_transition<'a>(
            &self,
            cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<(), Self::Error> {
            self.log.push("validate");
            match (cmd, &given_state.state) {
                (FamilyCommand::Open { .. }, FamilyState::Draft) => Ok(()),
                (FamilyCommand::Close { .. }, FamilyState::Live) => Ok(()),
                _ => Err(HookError::InvalidTransition),
            }
        }

        fn compute_after_changes_unchecked<'a>(
            &self,
            cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<Self::AfterChanges, Self::Error> {
            self.log.push("unchecked");
            let (next_state, next_value) = match cmd {
                FamilyCommand::Open { opening_value } => {
                    (FamilyState::Live, format!("{}:{}", given_state.value, opening_value))
                }
                FamilyCommand::Close { closing_value } => {
                    (FamilyState::Draft, format!("{}:{}", given_state.value, closing_value))
                }
                FamilyCommand::RejectInPreCheck => unreachable!("rejected by pre_check_command"),
            };
            Ok(FamilyAfterChanges { next_state, next_value })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestCommand {
        Apply { next_value: &'static str, trading_balance_delta: i64, fee_balance_delta: i64 },
        RejectInPreCheck,
    }

    impl CommandWithGivenState for TestCommand {
        type GivenState = TestGivenState;
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestAccountContext {
        can_apply: bool,
        reject_in_transition: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestGivenState {
        order: TestOrder,
        balances: [TestBalance; 2],
        account: TestAccountContext,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestAfterChanges {
        updated_order_after: TestOrder,
        updated_balance_afters: [TestBalance; 2],
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestBeforeSnapshot {
        order_before: TestOrder,
        balance_befores: [TestBalance; 2],
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestBeforeAfterChanges {
        updated_order: UpdatedEntityPair<TestOrder>,
        updated_balances: UpdatedEntities<TestBalance>,
    }

    impl ReplayableChanges for TestBeforeAfterChanges {
        fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
            let mut events = vec![self.updated_order.to_replayable_event()?];
            events.extend(self.updated_balances.to_replayable_events()?);
            Ok(events)
        }
    }

    impl TestOrder {
        fn machine(status: TestStatus) -> Self {
            Self { id: 0, value: "machine".to_string(), version: 0, status }
        }
    }

    impl MiStateMachineOwnedUnchecked for TestOrder {
        type Command<'a>
            = TestCommand
        where
            Self: 'a;
        type State = TestStatus;
        type Error = HookError;
        type AfterChanges = TestAfterChanges;

        fn state(&self) -> &Self::State {
            &self.status
        }

        fn pre_check_command<'a>(&self, cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
            match cmd {
                TestCommand::RejectInPreCheck => Err(HookError::PreCheckRejected),
                _ => Ok(()),
            }
        }

        fn validate_state_transition<'a>(
            &self,
            _cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<(), Self::Error> {
            if !given_state.account.can_apply {
                return Err(HookError::InvalidTransition);
            }
            match given_state.account.reject_in_transition {
                true => Err(HookError::InvalidTransition),
                false => Ok(()),
            }
        }

        fn compute_after_changes_unchecked<'a>(
            &self,
            cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<Self::AfterChanges, Self::Error> {
            let TestGivenState { order, balances: [trading_balance, fee_balance], account: _ } =
                given_state;
            let (next_value, trading_balance_delta, fee_balance_delta) = match cmd {
                TestCommand::Apply { next_value, trading_balance_delta, fee_balance_delta } => {
                    (*next_value, *trading_balance_delta, *fee_balance_delta)
                }
                TestCommand::RejectInPreCheck => unreachable!("rejected by pre_check_command"),
            };

            let next_order = TestOrder {
                id: order.id,
                value: next_value.to_string(),
                version: order.version + 1,
                status: order.status.clone(),
            };
            let next_trading_balance = TestBalance {
                id: trading_balance.id,
                available: trading_balance.available + trading_balance_delta,
                version: trading_balance.version + 1,
            };
            let next_fee_balance = TestBalance {
                id: fee_balance.id,
                available: fee_balance.available + fee_balance_delta,
                version: fee_balance.version + 1,
            };

            Ok(TestAfterChanges {
                updated_order_after: next_order,
                updated_balance_afters: [next_trading_balance, next_fee_balance],
            })
        }
    }

    impl MiStateMachineOwnedBeforeAfter for TestOrder {
        type BeforeAfterChanges = TestBeforeAfterChanges;
        type BeforeSnapshot = TestBeforeSnapshot;

        fn capture_before(
            &self,
            given_state: &<Self::Command<'_> as CommandWithGivenState>::GivenState,
        ) -> Self::BeforeSnapshot {
            TestBeforeSnapshot {
                order_before: given_state.order.clone(),
                balance_befores: given_state.balances.clone(),
            }
        }

        fn merge_before_and_after(
            before: Self::BeforeSnapshot,
            after: Self::AfterChanges,
        ) -> Result<Self::BeforeAfterChanges, Self::Error> {
            let TestBeforeSnapshot {
                order_before,
                balance_befores: [trading_balance_before, fee_balance_before],
            } = before;
            let TestAfterChanges {
                updated_order_after,
                updated_balance_afters: [trading_balance_after, fee_balance_after],
            } = after;

            Ok(TestBeforeAfterChanges {
                updated_order: UpdatedEntityPair::new(order_before, updated_order_after),
                updated_balances: UpdatedEntities::new(
                    UpdatedEntityPair::new(trading_balance_before, trading_balance_after),
                    vec![UpdatedEntityPair::new(fee_balance_before, fee_balance_after)],
                ),
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct AfterOnlyMachine {
        state: TestStatus,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestAfterOnlyChanges {
        updated_order_after: TestOrder,
        created_bonus_balance_after: TestBalance,
    }

    impl MiStateMachineOwnedUnchecked for AfterOnlyMachine {
        type Command<'a>
            = TestCommand
        where
            Self: 'a;
        type State = TestStatus;
        type Error = HookError;
        type AfterChanges = TestAfterOnlyChanges;

        fn state(&self) -> &Self::State {
            &self.state
        }

        fn compute_after_changes_unchecked<'a>(
            &self,
            cmd: &Self::Command<'a>,
            given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
        ) -> Result<Self::AfterChanges, Self::Error> {
            let next_value = match cmd {
                TestCommand::Apply { next_value, .. } => *next_value,
                TestCommand::RejectInPreCheck => return Err(HookError::PreCheckRejected),
            };
            Ok(TestAfterOnlyChanges {
                updated_order_after: TestOrder {
                    id: given_state.order.id,
                    value: next_value.to_string(),
                    version: given_state.order.version + 1,
                    status: given_state.order.status.clone(),
                },
                created_bonus_balance_after: TestBalance { id: 303, available: 1, version: 1 },
            })
        }
    }

    fn sample_given_state() -> TestGivenState {
        TestGivenState {
            order: TestOrder {
                id: 9,
                value: "before".to_string(),
                version: 3,
                status: TestStatus::Open,
            },
            balances: [
                TestBalance { id: 101, available: 100, version: 5 },
                TestBalance { id: 202, available: 7, version: 8 },
            ],
            account: TestAccountContext { can_apply: true, reject_in_transition: false },
        }
    }

    #[test]
    fn owned_after_only_changes_can_return_update_and_create_afters() {
        let machine = AfterOnlyMachine { state: TestStatus::Open };

        let changes = machine
            .compute_after_changes(
                &TestCommand::Apply {
                    next_value: "after",
                    trading_balance_delta: 10,
                    fee_balance_delta: -1,
                },
                &sample_given_state(),
            )
            .unwrap();

        assert_eq!(machine.state(), &TestStatus::Open);
        assert_ne!(machine.state(), &TestStatus::Applied);
        assert_eq!(changes.updated_order_after.id, 9);
        assert_eq!(changes.updated_order_after.value, "after");
        assert_eq!(changes.updated_order_after.version, 4);
        assert_eq!(changes.created_bonus_balance_after.id, 303);
        assert_eq!(changes.created_bonus_balance_after.available, 1);
        assert_eq!(changes.created_bonus_balance_after.version, 1);
    }

    #[test]
    fn machine_state_is_owned_status_not_given_state_entity() {
        let machine = AfterOnlyMachine { state: TestStatus::Applied };

        let changes = machine
            .compute_after_changes(
                &TestCommand::Apply {
                    next_value: "after",
                    trading_balance_delta: 10,
                    fee_balance_delta: -1,
                },
                &sample_given_state(),
            )
            .unwrap();

        assert_eq!(machine.state(), &TestStatus::Applied);
        assert_eq!(changes.updated_order_after.id, 9);
        assert_eq!(changes.updated_order_after.value, "after");
        assert_eq!(changes.created_bonus_balance_after.id, 303);
    }

    #[test]
    fn default_before_after_changes_capture_before_then_merge_with_after() {
        let machine = TestOrder::machine(TestStatus::Applied);
        let cmd = TestCommand::Apply {
            next_value: "after",
            trading_balance_delta: 10,
            fee_balance_delta: -2,
        };

        let given_state = sample_given_state();
        let before_after_changes =
            machine.compute_before_after_changes(&cmd, &given_state).unwrap();

        assert_eq!(before_after_changes.updated_order.before.value, "before");
        assert_eq!(before_after_changes.updated_order.after.value, "after");
        assert_eq!(before_after_changes.updated_balances.len(), 2);
        assert_eq!(
            before_after_changes
                .updated_balances
                .iter()
                .map(|pair| pair.before.available)
                .collect::<Vec<_>>(),
            vec![100, 7]
        );
        assert_eq!(
            before_after_changes
                .updated_balances
                .iter()
                .map(|pair| pair.after.id)
                .collect::<Vec<_>>(),
            vec![101, 202]
        );
    }

    #[test]
    fn default_before_after_changes_capture_before_before_running_after_logic() {
        use std::sync::{Arc, Mutex};

        #[derive(Debug, Clone)]
        struct SequenceCommand;

        impl CommandWithGivenState for SequenceCommand {
            type GivenState = Arc<Mutex<Vec<&'static str>>>;
        }

        #[derive(Debug, Clone)]
        struct SequenceChanges;

        impl ReplayableChanges for SequenceChanges {
            fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
                Ok(vec![])
            }
        }

        #[derive(Debug, Clone)]
        struct SequenceMachine;

        impl MiStateMachineOwnedUnchecked for SequenceMachine {
            type Command<'a>
                = SequenceCommand
            where
                Self: 'a;
            type State = ();
            type Error = EntityError;
            type AfterChanges = ();

            fn state(&self) -> &Self::State {
                &()
            }

            fn compute_after_changes_unchecked<'a>(
                &self,
                _cmd: &Self::Command<'a>,
                given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
            ) -> Result<Self::AfterChanges, Self::Error> {
                given_state.lock().unwrap().push("after");
                Ok(())
            }
        }

        impl MiStateMachineOwnedBeforeAfter for SequenceMachine {
            type BeforeAfterChanges = SequenceChanges;
            type BeforeSnapshot = ();

            fn capture_before(
                &self,
                given_state: &<Self::Command<'_> as CommandWithGivenState>::GivenState,
            ) -> Self::BeforeSnapshot {
                given_state.lock().unwrap().push("before");
            }

            fn merge_before_and_after(
                _before: Self::BeforeSnapshot,
                _after: Self::AfterChanges,
            ) -> Result<Self::BeforeAfterChanges, Self::Error> {
                Ok(SequenceChanges)
            }
        }

        let log = Arc::new(Mutex::new(Vec::new()));
        SequenceMachine.compute_before_after_changes(&SequenceCommand, &log).unwrap();

        assert_eq!(*log.lock().unwrap(), vec!["before", "after"]);
    }

    #[test]
    fn multi_entity_before_after_changes_project_replayable_update_events() {
        let machine = TestOrder::machine(TestStatus::Applied);
        let given_state = sample_given_state();
        let changes = machine
            .compute_before_after_changes(
                &TestCommand::Apply {
                    next_value: "after",
                    trading_balance_delta: 10,
                    fee_balance_delta: -2,
                },
                &given_state,
            )
            .unwrap();
        let events = changes.to_replayable_events().unwrap();

        assert_eq!(events.len(), 3);
        assert!(events.iter().all(EntityReplayableEvent::is_updated));
        assert_eq!(
            events.iter().map(|event| event.entity_id).collect::<Vec<_>>(),
            vec![9, 101, 202]
        );
        assert_eq!(events[0].field_change_count(), 1);
        assert_eq!(events[1].field_change_count(), 1);
        assert_eq!(events[2].field_change_count(), 1);
    }

    #[test]
    fn before_after_extension_reuses_same_pre_check_hook() {
        let machine = TestOrder::machine(TestStatus::Open);

        assert_eq!(
            machine.compute_before_after_changes(
                &TestCommand::RejectInPreCheck,
                &sample_given_state(),
            ),
            Err(HookError::PreCheckRejected)
        );
    }

    #[test]
    fn before_after_extension_reuses_same_transition_validation_hook() {
        let machine = TestOrder::machine(TestStatus::Open);
        let mut given_state = sample_given_state();
        given_state.account.reject_in_transition = true;

        assert_eq!(
            machine.compute_before_after_changes(
                &TestCommand::Apply {
                    next_value: "after",
                    trading_balance_delta: 10,
                    fee_balance_delta: -2,
                },
                &given_state
            ),
            Err(HookError::InvalidTransition)
        );
    }

    #[test]
    fn enum_command_family_reuses_same_shell_for_multiple_variants() {
        let machine = FamilyMachine::new(FamilyState::Draft);

        let opened = machine
            .compute_after_changes(
                &FamilyCommand::Open { opening_value: "opened" },
                &FamilyGivenState { state: FamilyState::Draft, value: "base" },
            )
            .unwrap();
        assert_eq!(opened.next_state, FamilyState::Live);
        assert_eq!(opened.next_value, "base:opened");

        let closed = machine
            .compute_after_changes(
                &FamilyCommand::Close { closing_value: "closed" },
                &FamilyGivenState { state: FamilyState::Live, value: "base" },
            )
            .unwrap();
        assert_eq!(closed.next_state, FamilyState::Draft);
        assert_eq!(closed.next_value, "base:closed");

        assert_eq!(
            machine.log.snapshot(),
            vec!["pre_check", "validate", "unchecked", "pre_check", "validate", "unchecked"]
        );
    }

    #[test]
    fn enum_command_family_rejects_illegal_state_pair_in_validate() {
        let machine = FamilyMachine::new(FamilyState::Draft);

        assert_eq!(
            machine.compute_after_changes(
                &FamilyCommand::Close { closing_value: "closed" },
                &FamilyGivenState { state: FamilyState::Draft, value: "base" },
            ),
            Err(HookError::InvalidTransition)
        );

        assert_eq!(machine.log.snapshot(), vec!["pre_check", "validate"]);
    }

    #[test]
    fn enum_command_family_pre_check_stops_validate_and_unchecked() {
        let machine = FamilyMachine::new(FamilyState::Draft);

        assert_eq!(
            machine.compute_after_changes(
                &FamilyCommand::RejectInPreCheck,
                &FamilyGivenState { state: FamilyState::Draft, value: "base" },
            ),
            Err(HookError::PreCheckRejected)
        );

        assert_eq!(machine.log.snapshot(), vec!["pre_check"]);
    }

    #[test]
    fn base_trait_reuses_hooks_for_after_truth_changes() {
        let machine = TestOrder::machine(TestStatus::Open);

        assert_eq!(
            machine.compute_after_changes(&TestCommand::RejectInPreCheck, &sample_given_state()),
            Err(HookError::PreCheckRejected)
        );
    }
}
