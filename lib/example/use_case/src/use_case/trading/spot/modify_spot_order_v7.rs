use common_entity::{
    EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::spot::spot_order_v7::{SpotOrderV7, SpotOrderV7Event};
use example_core_entity::{OrderTif, SpotOrderStatus};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifySpotOrderV7OrderType {
    Limit { tif: String },
    Trigger { is_market: bool, trigger_price: String, trigger_role: String },
}

/// 现货订单完整替换命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifySpotOrderV7Cmd {
    pub party_id: String,
    pub asset: u32,
    pub is_buy: bool,
    pub order_type: ModifySpotOrderV7OrderType,
    /// 顶层 `oid`，用于定位原订单。
    pub oid: u64,
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
pub struct ModifySpotOrderV7State {
    pub order: SpotOrderV7,
}

/// 现货改单的纯业务计算结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV7AfterChanges {
    pub order_after: SpotOrderV7,
}

/// 现货改单的 before/after 变化。
///
/// 订单 pair 和余额 pairs 分别是各自变化的唯一 authoritative truth；after
/// 快照不在此类型中重复保存。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV7Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
}

/// 现货限价订单改单业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModifySpotOrderV7Error {
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
pub struct ModifySpotOrderV7UseCase;

impl ReplayableChanges for ModifySpotOrderV7Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        Ok(vec![self.updated_order.to_replayable_event()?])
    }
}

impl UseCase for ModifySpotOrderV7UseCase {
    type Command = LogEntry<ModifySpotOrderV7Cmd>;
    type StateGiven = ModifySpotOrderV7State;
    type Error = ModifySpotOrderV7Error;
    type StateChanged = ModifySpotOrderV7AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.payload().party_id.is_empty()
            || cmd.payload().oid == 0
            || cmd.payload().size <= Decimal::ZERO
        {
            return Err(ModifySpotOrderV7Error::InvalidSize);
        }
        if cmd.payload().price <= Decimal::ZERO {
            return Err(ModifySpotOrderV7Error::InvalidPrice);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if given_state.order.order_base().order_id != cmd.payload().oid {
            return Err(ModifySpotOrderV7Error::OrderLookupMismatch);
        }
        if !matches!(
            given_state.order.status(),
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) {
            return Err(ModifySpotOrderV7Error::OrderNotModifiable);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut modified = given_state.order.clone();
        let tif = match &cmd.payload().order_type {
            ModifySpotOrderV7OrderType::Limit { tif } => Some(parse_tif(tif)?),
            ModifySpotOrderV7OrderType::Trigger { .. } => {
                return Err(ModifySpotOrderV7Error::OnlyLimitOrderCanBeModified);
            }
        };
        modified.apply(SpotOrderV7Event::Modify {
            price: cmd.payload().price,
            size: cmd.payload().size,
            tif,
            cloid: cmd.payload().cloid.clone().filter(|id| !id.is_empty()),
            timestamp: cmd.logical_ts(),
        })?;
        Ok(ModifySpotOrderV7AfterChanges { order_after: modified })
    }
}

fn parse_tif(value: &str) -> Result<OrderTif, ModifySpotOrderV7Error> {
    match value.to_ascii_lowercase().as_str() {
        "gtc" => Ok(OrderTif::Gtc),
        "ioc" => Ok(OrderTif::Ioc),
        "alo" => Ok(OrderTif::Alo),
        _ => Err(ModifySpotOrderV7Error::InvalidTimeInForce),
    }
}

impl StateDiff for ModifySpotOrderV7UseCase {
    type StateDiff = ModifySpotOrderV7Changes;

    fn do_compute_state_diff(
        given_state: ModifySpotOrderV7State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(ModifySpotOrderV7Changes {
            updated_order: UpdatedEntityPair {
                before: given_state.order,
                after: after.order_after,
            },
        })
    }
}
