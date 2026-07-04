//! 多聚合 `core.use_case` family 编排骨架。
//!
//! 这组 trait 的公开语义只服务跨聚合或多业务对象协调的 `core.use_case` 编排。
//! 它表达的是：围绕一个主业务主题组织一组相关业务动作，在
//! `Command + GivenState -> Changes` 编排层分别驱动多个聚合，产出 case 级业务真相。
//!
//! 这里的“主业务主题”表示业务中心，而不是唯一参与对象。一个实现必须协调多个聚合
//! 或多个业务对象，但这些动作仍需共享：
//! - 可匹配的 authoritative `GivenState` 语义
//! - 一致的 `Changes` / case truth 模型
//! - 同一组对外公开的业务编排边界
//!
//! 它负责什么：
//! - 在 `core.use_case` 边界内协调多个聚合公开业务方法
//! - 为一组相关 use case 固定统一 hook 顺序
//! - 产出稳定的 after truth，或进一步产出 replay / persist / audit 所需的 case truth
//!
//! 它不负责什么：
//! - 状态加载、DB 访问、持久化执行
//! - 事件发布
//! - HTTP / WebSocket reply shaping
//! - 权限、鉴权、审计等基础设施实现
//!
//! 明确禁用：
//! - 不要把聚合内部业务演进建模到这里
//! - 不要把单个聚合的内部推导建模到这里
//! - 不要让任一聚合直接装载、导航、调用另一个聚合
//! - 不要把 adapter / infra 逻辑下沉进该实现
//!
//! 与 [`crate::CommandUseCase6`] 的关系是并列选择，而不是上下位包装关系：
//! - `MiStateMachineOwnedV2` 适合共享 `GivenState` 与 truth 模型的多聚合 `use-case family`
//! - `CommandUseCase6` 适合表达边界清晰的完整 use case 契约
//!
//! # Examples
//!
//! 下面的 `SpotTradingOrchestrator` 是一个多聚合 `use-case family` 的骨架示例。
//! `PlaceLimitOrder` 只是该 family 的一个动作分支；同一 family 还可以继续承载其它共享同一
//! 业务主题、`GivenState` 语义与 truth 模型的动作。
//!
//! ```rust
//! use entity::{
//!     CommandWithGivenState, Entity, EntityError, EntityFieldChange, EntityReplayableEvent,
//!     MiStateMachineOwnedV2, MiStateMachineOwnedV2BeforeAfter,
//!     MiStateMachineOwnedV2Unchecked, ReplayableChanges, UpdatedEntityPair,
//! };
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum TradingError {
//!     InvalidQuoteAmount,
//!     TradingDisabled,
//!     BranchMismatch,
//!     InsufficientAvailableCash,
//! }
//!
//! impl std::fmt::Display for TradingError {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "{self:?}")
//!     }
//! }
//!
//! impl std::error::Error for TradingError {}
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum OrderStatus {
//!     Draft,
//!     Open,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct Order {
//!     id: i64,
//!     status: OrderStatus,
//!     quote_locked: i64,
//!     version: u64,
//! }
//!
//! impl Order {
//!     fn open_limit(&self, quote_amount: i64) -> Self {
//!         Self {
//!             id: self.id,
//!             status: OrderStatus::Open,
//!             quote_locked: quote_amount,
//!             version: self.version + 1,
//!         }
//!     }
//! }
//!
//! impl Entity for Order {
//!     type Id = i64;
//!
//!     fn entity_id(&self) -> Self::Id {
//!         self.id
//!     }
//!
//!     fn entity_type() -> u8 {
//!         31
//!     }
//!
//!     fn entity_version(&self) -> u64 {
//!         self.version
//!     }
//!
//!     fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
//!         let mut changes = Vec::new();
//!         if self.status != other.status {
//!             changes.push(EntityFieldChange::new(
//!                 "status",
//!                 format!("{:?}", self.status),
//!                 format!("{:?}", other.status),
//!             ));
//!         }
//!         if self.quote_locked != other.quote_locked {
//!             changes.push(EntityFieldChange::new(
//!                 "quote_locked",
//!                 self.quote_locked.to_string(),
//!                 other.quote_locked.to_string(),
//!             ));
//!         }
//!         changes
//!     }
//!
//!     fn replay_field_type(field_name: &str) -> u8 {
//!         match field_name {
//!             "status" => 1,
//!             "quote_locked" => 2,
//!             _ => 0,
//!         }
//!     }
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct CashBalance {
//!     id: i64,
//!     available: i64,
//!     version: u64,
//! }
//!
//! impl CashBalance {
//!     fn reserve_quote(&self, quote_amount: i64) -> Result<Self, TradingError> {
//!         if self.available < quote_amount {
//!             return Err(TradingError::InsufficientAvailableCash);
//!         }
//!
//!         Ok(Self {
//!             id: self.id,
//!             available: self.available - quote_amount,
//!             version: self.version + 1,
//!         })
//!     }
//! }
//!
//! impl Entity for CashBalance {
//!     type Id = i64;
//!
//!     fn entity_id(&self) -> Self::Id {
//!         self.id
//!     }
//!
//!     fn entity_type() -> u8 {
//!         32
//!     }
//!
//!     fn entity_version(&self) -> u64 {
//!         self.version
//!     }
//!
//!     fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
//!         if self.available == other.available {
//!             Vec::new()
//!         } else {
//!             vec![EntityFieldChange::new(
//!                 "available",
//!                 self.available.to_string(),
//!                 other.available.to_string(),
//!             )]
//!         }
//!     }
//!
//!     fn replay_field_type(field_name: &str) -> u8 {
//!         match field_name {
//!             "available" => 1,
//!             _ => 0,
//!         }
//!     }
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct BalanceHold {
//!     id: i64,
//!     reserved: i64,
//!     active: bool,
//!     version: u64,
//! }
//!
//! impl BalanceHold {
//!     fn open(&self, quote_amount: i64) -> Self {
//!         Self {
//!             id: self.id,
//!             reserved: quote_amount,
//!             active: true,
//!             version: self.version + 1,
//!         }
//!     }
//! }
//!
//! impl Entity for BalanceHold {
//!     type Id = i64;
//!
//!     fn entity_id(&self) -> Self::Id {
//!         self.id
//!     }
//!
//!     fn entity_type() -> u8 {
//!         33
//!     }
//!
//!     fn entity_version(&self) -> u64 {
//!         self.version
//!     }
//!
//!     fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
//!         let mut changes = Vec::new();
//!         if self.reserved != other.reserved {
//!             changes.push(EntityFieldChange::new(
//!                 "reserved",
//!                 self.reserved.to_string(),
//!                 other.reserved.to_string(),
//!             ));
//!         }
//!         if self.active != other.active {
//!             changes.push(EntityFieldChange::new(
//!                 "active",
//!                 self.active.to_string(),
//!                 other.active.to_string(),
//!             ));
//!         }
//!         changes
//!     }
//!
//!     fn replay_field_type(field_name: &str) -> u8 {
//!         match field_name {
//!             "reserved" => 1,
//!             "active" => 2,
//!             _ => 0,
//!         }
//!     }
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct PlaceLimitOrder {
//!     quote_amount: i64,
//! }
//!
//! // 在生产实现中，同一 family 也可以改成 `enum SpotTradingCommand`，
//! // 把多个相关动作分支收敛进同一个编排入口。
//! impl CommandWithGivenState for PlaceLimitOrder {
//!     type GivenState = SpotTradingGivenState;
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum SpotTradingGivenState {
//!     Placeable {
//!         order: Order,
//!         cash: CashBalance,
//!         hold: BalanceHold,
//!         trading_enabled: bool,
//!     },
//!     Cancelable {
//!         hold: BalanceHold,
//!     },
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct SpotTradingAfterChanges {
//!     order_after: Order,
//!     cash_after: CashBalance,
//!     hold_after: BalanceHold,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct SpotTradingChanges {
//!     updated_order: UpdatedEntityPair<Order>,
//!     updated_cash: UpdatedEntityPair<CashBalance>,
//!     updated_hold: UpdatedEntityPair<BalanceHold>,
//! }
//!
//! impl ReplayableChanges for SpotTradingChanges {
//!     fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
//!         Ok(vec![
//!             self.updated_order.to_replayable_event()?,
//!             self.updated_cash.to_replayable_event()?,
//!             self.updated_hold.to_replayable_event()?,
//!         ])
//!     }
//! }
//!
//! #[derive(Debug, Clone)]
//! struct SpotTradingOrchestrator;
//!
//! impl MiStateMachineOwnedV2Unchecked for SpotTradingOrchestrator {
//!     type Command<'a> = PlaceLimitOrder
//!     where
//!         Self: 'a;
//!     type Error = TradingError;
//!     type AfterChanges = SpotTradingAfterChanges;
//!
//!     fn pre_check_command<'a>(&self, cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
//!         if cmd.quote_amount <= 0 {
//!             return Err(TradingError::InvalidQuoteAmount);
//!         }
//!         Ok(())
//!     }
//!
//!     fn validate_against_given_state<'a>(
//!         &self,
//!         _cmd: &Self::Command<'a>,
//!         given_state: &SpotTradingGivenState,
//!     ) -> Result<(), Self::Error> {
//!         match given_state {
//!             SpotTradingGivenState::Placeable { trading_enabled: true, .. } => Ok(()),
//!             SpotTradingGivenState::Placeable { trading_enabled: false, .. } => {
//!                 Err(TradingError::TradingDisabled)
//!             }
//!             SpotTradingGivenState::Cancelable { .. } => Err(TradingError::BranchMismatch),
//!         }
//!     }
//!
//!     fn compute_after_changes_unchecked<'a>(
//!         &self,
//!         cmd: &Self::Command<'a>,
//!         given_state: &SpotTradingGivenState,
//!     ) -> Result<Self::AfterChanges, Self::Error> {
//!         let SpotTradingGivenState::Placeable { order, cash, hold, .. } = given_state else {
//!             return Err(TradingError::BranchMismatch);
//!         };
//!
//!         let order_after = order.open_limit(cmd.quote_amount);
//!         let cash_after = cash.reserve_quote(cmd.quote_amount)?;
//!         let hold_after = hold.open(cmd.quote_amount);
//!
//!         Ok(SpotTradingAfterChanges {
//!             order_after,
//!             cash_after,
//!             hold_after,
//!         })
//!     }
//! }
//!
//! impl MiStateMachineOwnedV2BeforeAfter for SpotTradingOrchestrator {
//!     type BeforeAfterChanges = SpotTradingChanges;
//!
//!     fn merge_before_and_after(
//!         given_state: &SpotTradingGivenState,
//!         after: Self::AfterChanges,
//!     ) -> Result<Self::BeforeAfterChanges, Self::Error> {
//!         let SpotTradingGivenState::Placeable { order, cash, hold, .. } = given_state else {
//!             return Err(TradingError::BranchMismatch);
//!         };
//!
//!         Ok(SpotTradingChanges {
//!             updated_order: UpdatedEntityPair::new(order.clone(), after.order_after),
//!             updated_cash: UpdatedEntityPair::new(cash.clone(), after.cash_after),
//!             updated_hold: UpdatedEntityPair::new(hold.clone(), after.hold_after),
//!         })
//!     }
//! }
//!
//! let orchestrator = SpotTradingOrchestrator;
//! let command = PlaceLimitOrder { quote_amount: 30 };
//! let placeable_state = SpotTradingGivenState::Placeable {
//!     order: Order {
//!         id: 7001,
//!         status: OrderStatus::Draft,
//!         quote_locked: 0,
//!         version: 4,
//!     },
//!     cash: CashBalance {
//!         id: 8001,
//!         available: 100,
//!         version: 9,
//!     },
//!     hold: BalanceHold {
//!         id: 9001,
//!         reserved: 0,
//!         active: false,
//!         version: 1,
//!     },
//!     trading_enabled: true,
//! };
//!
//! let after = orchestrator
//!     .compute_after_changes(&command, &placeable_state)
//!     .unwrap();
//! assert_eq!(after.order_after.status, OrderStatus::Open);
//! assert_eq!(after.cash_after.available, 70);
//! assert_eq!(after.hold_after.reserved, 30);
//!
//! let replayable = orchestrator
//!     .compute_before_after_changes(&command, &placeable_state)
//!     .unwrap();
//! let events = replayable.to_replayable_events().unwrap();
//! assert_eq!(events.len(), 3);
//! assert!(events.iter().all(EntityReplayableEvent::is_updated));
//!
//! let disabled_state = match placeable_state.clone() {
//!     SpotTradingGivenState::Placeable {
//!         order,
//!         cash,
//!         hold,
//!         ..
//!     } => SpotTradingGivenState::Placeable {
//!         order,
//!         cash,
//!         hold,
//!         trading_enabled: false,
//!     },
//!     SpotTradingGivenState::Cancelable { .. } => unreachable!(),
//! };
//! assert_eq!(
//!     orchestrator.compute_after_changes(&command, &disabled_state),
//!     Err(TradingError::TradingDisabled)
//! );
//!
//! let cancelable_state = SpotTradingGivenState::Cancelable {
//!     hold: BalanceHold {
//!         id: 9100,
//!         reserved: 30,
//!         active: true,
//!         version: 3,
//!     },
//! };
//! assert_eq!(
//!     orchestrator.compute_after_changes(&command, &cancelable_state),
//!     Err(TradingError::BranchMismatch)
//! );
//! ```

use std::fmt::Debug;

use crate::{CommandWithGivenState, ReplayableChanges};

/// 多聚合 `use-case family` 编排的最低实现契约。
///
/// `Command<'a>` 可以是单个业务命令，也可以是一组相关动作的命令族；
/// 关键在于这些动作仍共享同一业务主题，以及可匹配的 `GivenState` / truth 模型。
/// 不要把聚合内部业务演进或单对象内部推导的契约放进这里。
pub trait MiStateMachineOwnedV2Unchecked: Clone + Debug + Send + Sync {
    type Command<'a>: CommandWithGivenState
    where
        Self: 'a;

    /// 当前 family 的业务错误类型。
    type Error;

    /// 当前 case 的 after truth。
    ///
    /// 这里不要求它长成单实体 after 快照；它可以是多聚合重组结果、业务结果 struct，
    /// 或命令族分支对应的 case 级 after truth。
    type AfterChanges;

    /// 对命令本身做不依赖 `GivenState` 的快速校验。
    fn pre_check_command<'a>(&self, _cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 基于已加载的 `GivenState` 做业务校验。
    ///
    /// `GivenState` 可以由多个聚合和上下文字段组成。实现者应在这里显式拒绝
    /// branch mismatch 或 state mismatch，而不是把这些不匹配静默吞掉。
    fn validate_against_given_state<'a>(
        &self,
        _cmd: &Self::Command<'a>,
        _given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// 在统一 hook 链路通过后，分别驱动多个聚合公开业务方法，计算 case 级 after truth。
    ///
    /// 该方法不是对外稳定入口。外部应始终走
    /// `pre_check_command() -> validate_against_given_state() -> compute_after_changes_unchecked()`
    /// 的统一链路。
    fn compute_after_changes_unchecked<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error>;
}

/// 多聚合 `use-case family` 的稳定对外入口。
///
/// 它是该 family 的公共业务执行壳，固定执行：
/// `pre_check_command() -> validate_against_given_state() -> compute_after_changes_unchecked()`
///
/// 这让多聚合编排 hook 顺序稳定下来，避免实现者绕过校验直接计算 after truth。
pub trait MiStateMachineOwnedV2: MiStateMachineOwnedV2Unchecked {
    fn compute_after_changes<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        self.pre_check_command(cmd)?;
        self.validate_against_given_state(cmd, given_state)?;
        self.compute_after_changes_unchecked(cmd, given_state)
    }
}

impl<T> MiStateMachineOwnedV2 for T where T: MiStateMachineOwnedV2Unchecked {}

/// 在同一多聚合 family 编排上补足 replay / persist / audit 所需 case truth 的扩展。
///
/// 只有当当前 family 需要稳定 replay、持久化、diff 或审计真相时，才需要实现该 trait。
/// 默认链路仍然保持单一真相路径：先复用 family 的 after 计算，再从 `GivenState`
/// 提取 case 级 before 并合并成 replayable changes。
pub trait MiStateMachineOwnedV2BeforeAfter: MiStateMachineOwnedV2 {
    /// 最终可 replay 的 before/after changes。
    type BeforeAfterChanges: ReplayableChanges;

    /// 把 `GivenState` 中的 before truth 与 after 结果合并成最终 replayable changes。
    ///
    /// 这里的 `GivenState` 仍然只是运输形状；实现者可以直接使用其全部内容，也可以只提取
    /// 必要子集，或重组为异构 before truth 后再与 after 稳定配对。
    fn merge_before_and_after(
        given_state: &<Self::Command<'_> as CommandWithGivenState>::GivenState,
        after: Self::AfterChanges,
    ) -> Result<Self::BeforeAfterChanges, Self::Error>;

    /// 基于 `Command + GivenState` 计算稳定的 replayable before/after changes。
    fn compute_before_after_changes<'a>(
        &self,
        cmd: &Self::Command<'a>,
        given_state: &<Self::Command<'a> as CommandWithGivenState>::GivenState,
    ) -> Result<Self::BeforeAfterChanges, Self::Error> {
        let after = <Self as MiStateMachineOwnedV2>::compute_after_changes(self, cmd, given_state)?;
        Self::merge_before_and_after(given_state, after)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use crate::{
        CommandWithGivenState, EntityError, EntityReplayableEvent, MiStateMachineOwnedV2,
        MiStateMachineOwnedV2BeforeAfter, MiStateMachineOwnedV2Unchecked,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum HookError {
        PreCheckRejected,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HookCommand {
        reject_in_pre_check: bool,
    }

    impl CommandWithGivenState for HookCommand {
        type GivenState = Arc<Mutex<Vec<&'static str>>>;
    }

    #[derive(Debug, Clone)]
    struct HookMachine;

    impl MiStateMachineOwnedV2Unchecked for HookMachine {
        type Command<'a>
            = HookCommand
        where
            Self: 'a;
        type Error = HookError;
        type AfterChanges = ();

        fn pre_check_command<'a>(&self, cmd: &Self::Command<'a>) -> Result<(), Self::Error> {
            if cmd.reject_in_pre_check {
                return Err(HookError::PreCheckRejected);
            }
            Ok(())
        }

        fn validate_against_given_state<'a>(
            &self,
            _cmd: &Self::Command<'a>,
            given_state: &Arc<Mutex<Vec<&'static str>>>,
        ) -> Result<(), Self::Error> {
            given_state.lock().unwrap().push("validate");
            Ok(())
        }

        fn compute_after_changes_unchecked<'a>(
            &self,
            _cmd: &Self::Command<'a>,
            given_state: &Arc<Mutex<Vec<&'static str>>>,
        ) -> Result<Self::AfterChanges, Self::Error> {
            given_state.lock().unwrap().push("unchecked");
            Ok(())
        }
    }

    #[derive(Debug, Clone)]
    struct ReplayableLog;

    impl crate::ReplayableChanges for ReplayableLog {
        fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
            Ok(Vec::new())
        }
    }

    impl MiStateMachineOwnedV2BeforeAfter for HookMachine {
        type BeforeAfterChanges = ReplayableLog;

        fn merge_before_and_after(
            given_state: &Arc<Mutex<Vec<&'static str>>>,
            _after: Self::AfterChanges,
        ) -> Result<Self::BeforeAfterChanges, Self::Error> {
            given_state.lock().unwrap().push("merge");
            Ok(ReplayableLog)
        }
    }

    #[test]
    fn v2_after_path_reuses_validate_then_unchecked_chain() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let machine = HookMachine;

        machine.compute_after_changes(&HookCommand { reject_in_pre_check: false }, &log).unwrap();

        assert_eq!(*log.lock().unwrap(), vec!["validate", "unchecked"]);
    }

    #[test]
    fn v2_before_after_path_reuses_after_pipeline_then_merges_with_given_state() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let machine = HookMachine;

        machine
            .compute_before_after_changes(&HookCommand { reject_in_pre_check: false }, &log)
            .unwrap();

        assert_eq!(*log.lock().unwrap(), vec!["validate", "unchecked", "merge"]);
    }

    #[test]
    fn v2_pre_check_stops_validate_and_unchecked() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let machine = HookMachine;

        assert_eq!(
            machine.compute_after_changes(&HookCommand { reject_in_pre_check: true }, &log,),
            Err(HookError::PreCheckRejected)
        );
        assert!(log.lock().unwrap().is_empty());
    }
}
