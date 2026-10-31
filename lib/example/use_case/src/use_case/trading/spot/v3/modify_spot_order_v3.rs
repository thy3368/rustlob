use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff, UpdatedEntityPair,
};
use example_core_entity::spot::spot_order_v3::SpotOrderV3Any;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 现货订单完整替换命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifySpotOrderV3Cmd {
    /// 顶层 `oid`，用于定位原订单。
    pub order_id: u64,
    /// Hyperliquid `order.c`，表示修改后订单的 client order id。
    ///
    /// 为空时由后续业务逻辑保留原订单的 client order id。
    pub cloid: Option<String>,

    /// 修改后的完整价格，而不是价格增量。
    pub price: Decimal,
    /// 修改后的完整数量，而不是数量增量。
    pub size: Decimal,
}

/// 现货改单所需的 authoritative 状态快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV3State {
    pub order: SpotOrderV3Any,
}

/// 现货改单的纯业务计算结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV3AfterChanges {
    pub order_after: SpotOrderV3Any,
}

/// 现货改单的 before/after 变化。
///
/// 订单 pair 和余额 pairs 分别是各自变化的唯一 authoritative truth；after
/// 快照不在此类型中重复保存。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV3Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV3Any>,
}

/// 现货限价订单改单业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModifySpotOrderV2Error {
    #[error("party id must not be empty")]
    EmptyPartyId,
    #[error("order lookup is missing or invalid")]
    InvalidLookup,
    #[error("order lookup does not match the loaded order")]
    OrderLookupMismatch,
    #[error("on_command party does not match the loaded order party")]
    PartyMismatch,
    #[error("on_command asset does not match the loaded order asset")]
    AssetMismatch,
    #[error("only limit orders can be modified")]
    OnlyLimitOrderCanBeModified,
    #[error("order is not an open, unfilled order")]
    OrderNotModifiable,
    #[error("changing order side is not supported")]
    SideChangeUnsupported,
    #[error("price must be a positive integer string")]
    InvalidPrice,
    #[error("size must be a positive integer string")]
    InvalidSize,
    #[error("time in force must be gtc, ioc, or alo")]
    InvalidTimeInForce,
    #[error("balance not found")]
    BalanceNotFound,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("arithmetic overflow while computing spot order v2 modification")]
    ArithmeticOverflow,
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModifySpotOrderV3UseCase;

impl ReplayableChanges for ModifySpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![self.updated_order.to_replayable_event()?])
    }
}

impl StateMachineV2 for ModifySpotOrderV3UseCase {
    type Command = LogEntry<ModifySpotOrderV3Cmd>;
    type StateGiven = ModifySpotOrderV3State;
    type Error = ModifySpotOrderV2Error;
    type StateChanged = ModifySpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.payload().order_id == 0 || cmd.payload().size <= Decimal::ZERO {
            return Err(ModifySpotOrderV2Error::InvalidSize);
        }
        if cmd.payload().price <= Decimal::ZERO {
            return Err(ModifySpotOrderV2Error::InvalidPrice);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if given_state.order.order_base().order_id != cmd.payload().order_id {
            return Err(ModifySpotOrderV2Error::OrderLookupMismatch);
        }
        if !matches!(
            given_state.order,
            SpotOrderV3Any::Pending(_)
                | SpotOrderV3Any::Open(_)
                | SpotOrderV3Any::PartiallyFilled(_)
        ) {
            return Err(ModifySpotOrderV2Error::OrderNotModifiable);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let modified = match &given_state.order {
            SpotOrderV3Any::Pending(order) => SpotOrderV3Any::Pending(order.modify(
                cmd.payload().price,
                cmd.payload().size,
                cmd.logical_ts(),
            )?),
            SpotOrderV3Any::Open(order) => SpotOrderV3Any::Open(order.modify(
                cmd.payload().price,
                cmd.payload().size,
                cmd.logical_ts(),
            )?),
            SpotOrderV3Any::PartiallyFilled(order) => SpotOrderV3Any::PartiallyFilled(
                order.modify(cmd.payload().price, cmd.payload().size, cmd.logical_ts())?,
            ),
            SpotOrderV3Any::Filled(_)
            | SpotOrderV3Any::Canceled(_)
            | SpotOrderV3Any::Rejected(_) => {
                return Err(ModifySpotOrderV2Error::OrderNotModifiable);
            }
        };
        let mut base = modified.order_base().clone();
        if let Some(cloid) = &cmd.payload().cloid {
            base.cloid = Some(cloid.clone());
        }
        base.version = cmd.lsn();
        Ok(ModifySpotOrderV3AfterChanges { order_after: SpotOrderV3Any::from_record(base)? })
    }
}

impl StateMachineV2Diff for ModifySpotOrderV3UseCase {
    type StateDiff = ModifySpotOrderV3Changes;

    fn do_compute_state_diff(
        given_state: ModifySpotOrderV3State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(ModifySpotOrderV3Changes {
            updated_order: UpdatedEntityPair {
                before: given_state.order,
                after: after.order_after,
            },
        })
    }
}
