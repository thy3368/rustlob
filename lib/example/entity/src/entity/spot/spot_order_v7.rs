use common_entity::fsm_v2::{Fsm, SequencedCommand};
use common_entity::{Entity, EntityError, EntityFieldChange, Shard, TrackDiff};
use lob_repo_v2::lob::LobOrder;
use rust_decimal::Decimal;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::common::PriceType;
use crate::entity::common::common_order::{OrderBase, OrderTrigger, SpotAmountConversionError};
use crate::entity::common::order_primitives::{
    OrderGroupRelation, OrderSide, OrderTif, OrderTriggerRole, OrderType, SpotOrderStatus,
};

/// 创建现货订单所需的业务事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSpotOrderV7Command {
    pub party_id: u64,
    pub order_id: u64,
    pub symbol: crate::account::symbol::Symbol,
    pub side: OrderSide,
    pub price: Decimal,
    pub size: Decimal,
    pub order_type: OrderType,
    pub relation: OrderGroupRelation,
    pub cloid: Option<String>,
    pub fee_bps: u64,
}

/// 不含定序元数据的现货订单业务命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV7Command {
    Create(CreateSpotOrderV7Command),
    Freeze,
    MatchedAsTaker {
        filled_size: Decimal,
        quote_notional: Decimal,
        quote_fee: Decimal,
    },
    MatchedAsMaker {
        filled_size: Decimal,
        quote_notional: Decimal,
        quote_fee: Decimal,
    },
    RejectedAsTaker,
    Modify {
        price: Decimal,
        size: Decimal,
        tif: Option<OrderTif>,
        cloid: Option<String>,
    },
    Cancel,
}

impl SpotOrderV7Command {
    const fn event_type(&self) -> u8 {
        match self {
            SpotOrderV7Command::Create(_) => 1,
            SpotOrderV7Command::Freeze => 2,
            SpotOrderV7Command::MatchedAsTaker { .. } => 3,
            SpotOrderV7Command::MatchedAsMaker { .. } => 4,
            SpotOrderV7Command::RejectedAsTaker => 5,
            SpotOrderV7Command::Modify { .. } => 6,
            SpotOrderV7Command::Cancel => 7,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]

///todo 增加累计 冻结金额，解冻金额，支付金额（fill,fee);增加增量资金变化 1，创建冻结，2，fill,
/// 3,cancel, 4.modify 用以创建balance的event
pub struct SpotOrderV7 {
    order_base: OrderBase,
    command_index: u32,
}

impl SpotOrderV7 {
    /// 由创建命令建立一个 `Pending` 订单。
    pub fn create(command: SequencedCommand<SpotOrderV7Command>) -> Result<Self, EntityError> {
        let (metadata, command) = command.into_parts();
        let SpotOrderV7Command::Create(command) = command else {
            return Err(err("spot order creation requires a create command"));
        };
        let (execution, tif, trigger, trigger_px) = match command.order_type {
            OrderType::Limit { tif } => (PriceType::Limit, tif, OrderTrigger::None, None),
            OrderType::Trigger {
                is_market,
                trigger_price,
                tpsl,
            } => (
                if is_market {
                    PriceType::Market
                } else {
                    PriceType::Limit
                },
                if is_market {
                    OrderTif::Ioc
                } else {
                    OrderTif::Gtc
                },
                match tpsl {
                    OrderTriggerRole::TakeProfit => OrderTrigger::TakeProfit,
                    OrderTriggerRole::StopLoss => OrderTrigger::StopLoss,
                },
                Some(Decimal::from(trigger_price)),
            ),
        };
        let lsn = metadata.lsn;
        let command_index = metadata.command_index;
        let mut order = OrderBase {
            owner: crate::entity::common::UserId(command.party_id),
            order_id: command.order_id,
            symbol: command.symbol,
            side: command.side,
            limit_px: command.price,
            size: command.size,
            orig_size: command.size,
            execution,
            tif,
            trigger,
            trigger_px,
            group_relation: command.relation,
            status: SpotOrderStatus::Pending,
            cloid: command.cloid,
            created_at: metadata.logical_ts,
            updated_at: metadata.logical_ts,
            lsn,
            ..OrderBase::default()
        };
        order.buy_quote_fee_reserve_bps = if order.side == OrderSide::Buy {
            command.fee_bps
        } else {
            0
        };
        order.buy_quote_fee_reserve_frozen = Decimal::ZERO;
        order.buy_quote_fee_reserve_paid = Decimal::ZERO;
        order.buy_quote_principal_frozen = Decimal::ZERO;
        order.buy_quote_principal_paid = Decimal::ZERO;
        order.buy_quote_principal_unfrozen = Decimal::ZERO;
        order.buy_quote_fee_reserve_unfrozen = Decimal::ZERO;
        order.sell_quote_fee_paid = Decimal::ZERO;
        order.sell_base_principal_frozen = Decimal::ZERO;
        order.sell_base_principal_unfrozen = Decimal::ZERO;
        Self::from_record_at(order, command_index)
    }

    pub fn single(mut order: OrderBase) -> Self {
        order.status = SpotOrderStatus::Pending;
        order.group_relation = OrderGroupRelation::Standalone;
        Self {
            order_base: order,
            command_index: 0,
        }
    }

    pub fn normal_tpsl(parent: OrderBase, children: Vec<OrderBase>) -> (Self, Vec<Self>) {
        let mut parent = Self::single(parent);
        parent.order_base.group_relation = OrderGroupRelation::NormalTpslParent;
        let id = parent.order_base.order_id;
        let children = children
            .into_iter()
            .map(|mut child| {
                child.status = SpotOrderStatus::Pending;
                child.group_relation = OrderGroupRelation::NormalTpslChild {
                    parent_order_id: id,
                };
                Self {
                    order_base: child,
                    command_index: 0,
                }
            })
            .collect();
        (parent, children)
    }

    pub fn from_record(order: OrderBase) -> Result<Self, EntityError> {
        Self::from_record_at(order, 0)
    }

    pub fn from_record_at(order: OrderBase, command_index: u32) -> Result<Self, EntityError> {
        validate_record(&order)?;
        Ok(Self {
            order_base: order,
            command_index,
        })
    }
    pub fn order_base(&self) -> &OrderBase {
        &self.order_base
    }
    pub fn status(&self) -> SpotOrderStatus {
        self.order_base.status
    }
    pub fn hold_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let o = self.order_base();
        if !matches!(
            o.status,
            SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) {
            return coin_amount(
                if o.side == OrderSide::Buy {
                    o.symbol.quote
                } else {
                    o.symbol.base
                },
                Decimal::ZERO,
            );
        }
        let amount = match o.side {
            OrderSide::Buy => buy_total_held(o)?,
            OrderSide::Sell => o.size,
        };
        coin_amount(
            if o.side == OrderSide::Buy {
                o.symbol.quote
            } else {
                o.symbol.base
            },
            amount,
        )
    }

    /// 校验订单作为 taker 参与撮合所需的单聚合事实。
    pub fn validate_matchable_as_taker(&self) -> Result<(), EntityError> {
        validate_matchable(&self.order_base)
    }

    /// 校验订单作为 maker 参与撮合所需的单聚合事实。
    pub fn validate_matchable_as_maker(&self) -> Result<(), EntityError> {
        validate_matchable(&self.order_base)?;
        if self.order_base.execution != PriceType::Limit {
            return Err(err("maker order must use limit execution"));
        }
        Ok(())
    }

    fn compute_transition(
        &self,
        command: SequencedCommand<SpotOrderV7Command>,
    ) -> Result<OrderBase, EntityError> {
        let (metadata, payload) = command.into_parts();
        let position = (metadata.lsn, metadata.command_index);
        let timestamp = metadata.logical_ts;
        if position < (self.order_base.lsn, self.command_index) {
            return Err(err("order command LSN is older than the current LSN"));
        }
        if matches!(
            self.order_base.status,
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) {
            return Err(err("order is terminal"));
        }
        match &payload {
            SpotOrderV7Command::Create(_) => {
                return Err(err("create cannot be applied to an existing order"));
            }
            SpotOrderV7Command::Freeze => {
                if self.status() != SpotOrderStatus::Pending {
                    return Err(err("order is not pending"));
                }
                let freeze_amount = self
                    .taker_freeze_amount()
                    .map_err(|error| err(&error.to_string()))?;
                let mut next = self.order_base.clone();
                if next.side == OrderSide::Buy {
                    next.buy_quote_principal_frozen = next
                        .limit_px
                        .checked_mul(next.orig_size)
                        .ok_or_else(|| err("freeze principal overflow"))?;
                    next.buy_quote_fee_reserve_frozen = fee_reserve_target(&next)?;
                } else {
                    next.sell_base_principal_frozen = next.orig_size;
                    next.sell_base_principal_unfrozen = Decimal::ZERO;
                }
                next.status = SpotOrderStatus::Open;
                next.updated_at = timestamp;
                next.lsn = position.0;
                let _ = freeze_amount;
                Ok(next)
            }
            SpotOrderV7Command::MatchedAsTaker {
                filled_size,
                quote_notional,
                quote_fee,
            } => {
                self.validate_matchable_as_taker()?;
                let next = matched_order(
                    &self.order_base,
                    *filled_size,
                    *quote_notional,
                    *quote_fee,
                    position.0,
                    timestamp,
                    MatchRole::Taker,
                )?;
                Ok(next)
            }
            SpotOrderV7Command::MatchedAsMaker {
                filled_size,
                quote_notional,
                quote_fee,
            } => {
                self.validate_matchable_as_maker()?;
                let next = matched_order(
                    &self.order_base,
                    *filled_size,
                    *quote_notional,
                    *quote_fee,
                    position.0,
                    timestamp,
                    MatchRole::Maker,
                )?;
                Ok(next)
            }
            SpotOrderV7Command::RejectedAsTaker => {
                self.validate_matchable_as_taker()?;
                if self.order_base.effective_tif() != OrderTif::Alo {
                    return Err(err("order cannot be rejected as ALO taker"));
                }
                let mut next = self.order_base.clone();
                next.status = SpotOrderStatus::Rejected;
                settle_terminal_unfreeze(&mut next)?;
                next.updated_at = timestamp;
                next.lsn = position.0;
                Ok(next)
            }
            SpotOrderV7Command::Modify {
                price,
                size,
                tif,
                cloid,
            } => {
                if !matches!(
                    self.status(),
                    SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
                ) {
                    return Err(err("order is not modifiable"));
                }
                let mut next =
                    modified_order(&self.order_base, *price, *size, position.0, timestamp)?;
                if let Some(tif) = tif {
                    next.tif = *tif;
                }
                if let Some(cloid) = cloid {
                    next.cloid = Some(cloid.clone());
                }
                apply_buy_modify_counters(&self.order_base, &mut next)?;
                apply_sell_modify_counters(&self.order_base, &mut next)?;
                Ok(next)
            }
            SpotOrderV7Command::Cancel => {
                let was_pending = self.status() == SpotOrderStatus::Pending;
                let mut next = canceled_order(&self.order_base, position.0, timestamp)?;
                if was_pending {
                    next.size = Decimal::ZERO;
                } else {
                    settle_terminal_unfreeze(&mut next)?;
                }
                Ok(next)
            }
        }
    }
}

impl Fsm for SpotOrderV7 {
    type Command = SpotOrderV7Command;
    type Error = EntityError;

    fn event_type(command: &SpotOrderV7Command) -> u8 {
        command.event_type()
    }

    fn derive_transition(
        &self,
        command: SequencedCommand<SpotOrderV7Command>,
    ) -> Result<Self, EntityError> {
        let command_index = command.metadata.command_index;
        let next = self.compute_transition(command)?;
        Ok(Self {
            order_base: next,
            command_index,
        })
    }
}

#[derive(Clone, Copy)]
enum MatchRole {
    Taker,
    Maker,
}

fn matched_order(
    current: &OrderBase,
    filled_size: Decimal,
    quote_notional: Decimal,
    quote_fee: Decimal,
    lsn: u64,
    timestamp: u64,
    role: MatchRole,
) -> Result<OrderBase, EntityError> {
    if filled_size < Decimal::ZERO
        || filled_size > current.size
        || (filled_size > Decimal::ZERO && quote_notional <= Decimal::ZERO)
        || quote_fee < Decimal::ZERO
    {
        return Err(err("invalid match facts"));
    }
    if matches!(role, MatchRole::Maker) && filled_size == Decimal::ZERO {
        return Err(err("maker fill must be greater than zero"));
    }

    let mut next = current.clone();
    let limit_notional = current
        .limit_px
        .checked_mul(filled_size)
        .ok_or_else(|| err("match notional overflow"))?;
    if (next.side == OrderSide::Buy && quote_notional > limit_notional)
        || (filled_size == Decimal::ZERO
            && (quote_notional != Decimal::ZERO || quote_fee != Decimal::ZERO))
    {
        return Err(err("match notional exceeds buy limit principal"));
    }
    next.size = next
        .size
        .checked_sub(filled_size)
        .ok_or_else(|| err("size underflow"))?;
    if next.side == OrderSide::Buy {
        next.buy_quote_principal_paid = next
            .buy_quote_principal_paid
            .checked_add(quote_notional)
            .ok_or_else(|| err("principal paid overflow"))?;
        next.buy_quote_fee_reserve_paid = next
            .buy_quote_fee_reserve_paid
            .checked_add(quote_fee)
            .ok_or_else(|| err("fee overflow"))?;
        if next.buy_quote_fee_reserve_paid > next.buy_quote_fee_reserve_frozen {
            return Err(err("fee exceeds reserve"));
        }
    } else {
        next.sell_quote_fee_paid = next
            .sell_quote_fee_paid
            .checked_add(quote_fee)
            .ok_or_else(|| err("sell fee overflow"))?;
    }
    next.status = if next.size == Decimal::ZERO {
        SpotOrderStatus::Filled
    } else {
        match role {
            MatchRole::Maker => SpotOrderStatus::PartiallyFilled,
            MatchRole::Taker if filled_size == Decimal::ZERO => {
                if current.effective_tif() == OrderTif::Ioc {
                    SpotOrderStatus::Rejected
                } else {
                    current.status
                }
            }
            MatchRole::Taker if current.effective_tif() == OrderTif::Ioc => {
                SpotOrderStatus::Canceled
            }
            MatchRole::Taker => SpotOrderStatus::PartiallyFilled,
        }
    };
    if matches!(
        next.status,
        SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
    ) {
        settle_terminal_unfreeze(&mut next)?;
    }
    next.updated_at = timestamp;
    next.lsn = lsn;
    Ok(next)
}

fn modified_order(
    current: &OrderBase,
    price: Decimal,
    size: Decimal,
    lsn: u64,
    timestamp: u64,
) -> Result<OrderBase, EntityError> {
    let filled = current
        .orig_size
        .checked_sub(current.size)
        .ok_or_else(|| err("invalid filled size"))?;
    if price <= Decimal::ZERO || size <= Decimal::ZERO || size < filled {
        return Err(err("invalid modified order"));
    }
    let mut next = current.clone();
    next.limit_px = price;
    next.orig_size = size;
    next.size = size
        .checked_sub(filled)
        .ok_or_else(|| err("size overflow"))?;
    next.updated_at = timestamp;
    next.lsn = lsn;
    Ok(next)
}

fn canceled_order(current: &OrderBase, lsn: u64, timestamp: u64) -> Result<OrderBase, EntityError> {
    let mut next = current.clone();
    next.status = SpotOrderStatus::Canceled;
    next.updated_at = timestamp;
    next.lsn = lsn;
    Ok(next)
}

impl TrackDiff for SpotOrderV7 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_base.order_id.to_string()),
            EntityFieldChange::new("status", "", format!("{:?}", self.status())),
        ]
    }
    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let a = &self.order_base;
        let b = &other.order_base;
        let mut v = Vec::new();
        for (n, x, y) in [
            ("limit_px", a.limit_px.to_string(), b.limit_px.to_string()),
            ("size", a.size.to_string(), b.size.to_string()),
            (
                "orig_size",
                a.orig_size.to_string(),
                b.orig_size.to_string(),
            ),
            (
                "status",
                format!("{:?}", a.status),
                format!("{:?}", b.status),
            ),
            ("tif", format!("{:?}", a.tif), format!("{:?}", b.tif)),
            ("cloid", format!("{:?}", a.cloid), format!("{:?}", b.cloid)),
            (
                "buy_quote_fee_reserve_paid",
                a.buy_quote_fee_reserve_paid.to_string(),
                b.buy_quote_fee_reserve_paid.to_string(),
            ),
            (
                "buy_quote_principal_frozen",
                a.buy_quote_principal_frozen.to_string(),
                b.buy_quote_principal_frozen.to_string(),
            ),
            (
                "buy_quote_principal_paid",
                a.buy_quote_principal_paid.to_string(),
                b.buy_quote_principal_paid.to_string(),
            ),
            (
                "buy_quote_principal_unfrozen",
                a.buy_quote_principal_unfrozen.to_string(),
                b.buy_quote_principal_unfrozen.to_string(),
            ),
            (
                "buy_quote_fee_reserve_unfrozen",
                a.buy_quote_fee_reserve_unfrozen.to_string(),
                b.buy_quote_fee_reserve_unfrozen.to_string(),
            ),
            (
                "sell_quote_fee_paid",
                a.sell_quote_fee_paid.to_string(),
                b.sell_quote_fee_paid.to_string(),
            ),
            (
                "sell_base_principal_frozen",
                a.sell_base_principal_frozen.to_string(),
                b.sell_base_principal_frozen.to_string(),
            ),
            (
                "sell_base_principal_unfrozen",
                a.sell_base_principal_unfrozen.to_string(),
                b.sell_base_principal_unfrozen.to_string(),
            ),
            (
                "updated_at",
                a.updated_at.to_string(),
                b.updated_at.to_string(),
            ),
        ] {
            if x != y {
                v.push(EntityFieldChange::new(n, x, y));
            }
        }
        v
    }
}

impl Entity for SpotOrderV7 {
    fn entity_id(&self) -> u64 {
        self.order_base.order_id
    }
    fn is_derived() -> bool {
        false
    }
    fn entity_type() -> u8 {
        16
    }
    fn lsn(&self) -> u64 {
        self.order_base.lsn
    }
    fn command_index(&self) -> u32 {
        self.command_index
    }
    fn four_color_archetype() -> common_entity::FourColorArchetype {
        common_entity::FourColorArchetype::MomentInterval
    }
    fn mutation_model() -> common_entity::EntityMutationModel {
        common_entity::EntityMutationModel::VersionedMutable
    }
    fn aggregate_role() -> common_entity::AggregateRole {
        common_entity::AggregateRole::AggregateRoot
    }
    fn replay_field_type(name: &str) -> u8 {
        if matches!(name, "updated_at" | "version") {
            1
        } else {
            0
        }
    }

    fn owner_id(&self) -> String {
        todo!()
    }

    fn op_id(&self) -> String {
        todo!()
    }

    fn created_at(&self) -> u64 {
        todo!()
    }

    fn updated_at(&self) -> u64 {
        todo!()
    }
}

impl common_entity::PositionedEntity for SpotOrderV7 {
    fn entity_id(&self) -> u64 {
        <Self as Entity>::entity_id(self)
    }

    fn entity_type(&self) -> u8 {
        <Self as Entity>::entity_type()
    }

    fn lsn(&self) -> u64 {
        self.order_base.lsn
    }

    fn command_index(&self) -> u32 {
        self.command_index
    }
}
impl Shard for SpotOrderV7 {
    fn partition(&self) -> u8 {
        self.order_base.symbol.base.index() as u8
    }
}

impl LobOrder for SpotOrderV7 {
    type Side = OrderSide;
    fn entity_type() -> u8 {
        <Self as Entity>::entity_type()
    }
    fn entity_id(&self) -> u64 {
        self.order_base.order_id
    }
    fn symbol(&self) -> crate::account::symbol::Symbol {
        self.order_base.symbol
    }
    fn side(&self) -> Self::Side {
        self.order_base.side
    }
    fn is_buy_side(side: Self::Side) -> bool {
        side == OrderSide::Buy
    }
    fn px(&self) -> Decimal {
        self.order_base.limit_px
    }
    fn orig_size(&self) -> Decimal {
        self.order_base.orig_size
    }
    fn size(&self) -> Decimal {
        self.order_base.size
    }
    fn created_at(&self) -> u64 {
        self.order_base.created_at
    }
    fn entity_ver(&self) -> u64 {
        self.order_base.lsn
    }
    fn is_active(&self) -> bool {
        matches!(
            self.status(),
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) && self.order_base.size > Decimal::ZERO
    }
    fn from_replay_event(
        _: &common_entity::ReplayableDomainEvent,
        _: u64,
    ) -> Result<Self, lob_repo_v2::LobError> {
        Err(lob_repo_v2::LobError::Event(
            "SpotOrderV7 replay construction is not wired".into(),
        ))
    }
    fn apply_replay_event(
        self,
        _: &common_entity::ReplayableDomainEvent,
    ) -> Result<Self, lob_repo_v2::LobError> {
        Err(lob_repo_v2::LobError::Event(
            "SpotOrderV7 replay update is not wired".into(),
        ))
    }
}

fn err(s: &str) -> EntityError {
    EntityError::Custom(s.to_owned())
}
fn validate_record(o: &OrderBase) -> Result<(), EntityError> {
    if o.order_id == 0
        || !o.has_consistent_execution_facts()
        || o.orig_size <= Decimal::ZERO
        || o.size < Decimal::ZERO
        || o.size > o.orig_size
        || o.limit_px <= Decimal::ZERO
        || [
            o.buy_quote_fee_reserve_frozen,
            o.buy_quote_fee_reserve_paid,
            o.buy_quote_principal_frozen,
            o.buy_quote_principal_paid,
            o.buy_quote_principal_unfrozen,
            o.buy_quote_fee_reserve_unfrozen,
            o.sell_quote_fee_paid,
            o.sell_base_principal_frozen,
            o.sell_base_principal_unfrozen,
        ]
        .into_iter()
        .any(|amount| amount < Decimal::ZERO)
    {
        return Err(err("invalid order facts"));
    }
    let principal_accounted = o
        .buy_quote_principal_paid
        .checked_add(o.buy_quote_principal_unfrozen)
        .ok_or_else(|| err("principal accounting overflow"))?;
    let fee_accounted = o
        .buy_quote_fee_reserve_paid
        .checked_add(o.buy_quote_fee_reserve_unfrozen)
        .ok_or_else(|| err("fee accounting overflow"))?;
    if principal_accounted > o.buy_quote_principal_frozen
        || fee_accounted > o.buy_quote_fee_reserve_frozen
    {
        return Err(err("buy fund classification exceeds frozen amount"));
    }
    if o.side == OrderSide::Sell
        && (o.buy_quote_fee_reserve_bps != 0
            || o.buy_quote_principal_frozen != Decimal::ZERO
            || o.buy_quote_fee_reserve_frozen != Decimal::ZERO
            || o.buy_quote_principal_paid != Decimal::ZERO
            || o.buy_quote_fee_reserve_paid != Decimal::ZERO
            || o.buy_quote_principal_unfrozen != Decimal::ZERO
            || o.buy_quote_fee_reserve_unfrozen != Decimal::ZERO)
    {
        return Err(err("sell order contains buy fund facts"));
    }
    let sell_filled = o
        .orig_size
        .checked_sub(o.size)
        .ok_or_else(|| err("sell fill underflow"))?;
    if o.side == OrderSide::Buy
        && (o.sell_base_principal_frozen != Decimal::ZERO
            || o.sell_base_principal_unfrozen != Decimal::ZERO)
    {
        return Err(err("buy order contains sell fund facts"));
    }
    if o.side == OrderSide::Sell {
        let accounted = sell_filled
            .checked_add(o.sell_base_principal_unfrozen)
            .and_then(|v| {
                v.checked_add(
                    o.sell_base_principal_frozen
                        .checked_sub(sell_filled)
                        .and_then(|v| v.checked_sub(o.sell_base_principal_unfrozen))?,
                )
            })
            .ok_or_else(|| err("sell funds conservation overflow"))?;
        if accounted != o.sell_base_principal_frozen {
            return Err(err("sell funds are not conserved"));
        }
        if matches!(
            o.status,
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) && o.sell_base_principal_frozen != sell_filled + o.sell_base_principal_unfrozen
        {
            return Err(err("terminal sell funds are not conserved"));
        }
    }
    if matches!(
        o.status,
        SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
    ) && o.side == OrderSide::Buy
        && (principal_accounted != o.buy_quote_principal_frozen
            || fee_accounted != o.buy_quote_fee_reserve_frozen)
    {
        return Err(err("terminal buy funds are not conserved"));
    }
    Ok(())
}
fn validate_matchable(o: &OrderBase) -> Result<(), EntityError> {
    validate_record(o)?;
    if !matches!(
        o.status,
        SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
    ) || o.size <= Decimal::ZERO
    {
        return Err(err("order is not matchable"));
    }
    Ok(())
}
fn coin_amount(
    coin: crate::account::coin::Coin,
    amount: Decimal,
) -> Result<CoinAmount, SpotAmountConversionError> {
    CoinAmount::new(coin, amount)
        .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
}

fn apply_buy_modify_counters(before: &OrderBase, next: &mut OrderBase) -> Result<(), EntityError> {
    if next.side != OrderSide::Buy {
        return Ok(());
    }
    let principal_target = next
        .limit_px
        .checked_mul(next.size)
        .ok_or_else(|| err("principal target overflow"))?;
    let principal_held = before
        .buy_quote_principal_frozen
        .checked_sub(before.buy_quote_principal_paid)
        .and_then(|v| v.checked_sub(before.buy_quote_principal_unfrozen))
        .filter(|v| *v >= Decimal::ZERO)
        .ok_or_else(|| err("invalid principal classification"))?;
    adjust_counter(
        principal_held,
        principal_target,
        &mut next.buy_quote_principal_frozen,
        &mut next.buy_quote_principal_unfrozen,
        "principal",
    )?;
    let fee_target = fee_reserve_target(next)?;
    let fee_held = before
        .buy_quote_fee_reserve_frozen
        .checked_sub(before.buy_quote_fee_reserve_paid)
        .and_then(|v| v.checked_sub(before.buy_quote_fee_reserve_unfrozen))
        .filter(|v| *v >= Decimal::ZERO)
        .ok_or_else(|| err("invalid fee classification"))?;
    adjust_counter(
        fee_held,
        fee_target,
        &mut next.buy_quote_fee_reserve_frozen,
        &mut next.buy_quote_fee_reserve_unfrozen,
        "fee",
    )?;
    Ok(())
}

fn apply_sell_modify_counters(before: &OrderBase, next: &mut OrderBase) -> Result<(), EntityError> {
    if next.side != OrderSide::Sell {
        return Ok(());
    }
    let filled = before
        .orig_size
        .checked_sub(before.size)
        .ok_or_else(|| err("invalid filled size"))?;
    let current = before
        .sell_base_principal_frozen
        .checked_sub(filled)
        .and_then(|v| v.checked_sub(before.sell_base_principal_unfrozen))
        .filter(|v| *v >= Decimal::ZERO)
        .ok_or_else(|| err("invalid sell principal classification"))?;
    adjust_counter(
        current,
        next.size,
        &mut next.sell_base_principal_frozen,
        &mut next.sell_base_principal_unfrozen,
        "sell principal",
    )
}

fn adjust_counter(
    current: Decimal,
    target: Decimal,
    frozen: &mut Decimal,
    unfrozen: &mut Decimal,
    label: &str,
) -> Result<(), EntityError> {
    if target > current {
        *frozen = frozen
            .checked_add(target - current)
            .ok_or_else(|| err(&format!("{label} freeze overflow")))?;
    } else if target < current {
        *unfrozen = unfrozen
            .checked_add(current - target)
            .ok_or_else(|| err(&format!("{label} unfreeze overflow")))?;
    }
    Ok(())
}

fn fee_reserve_target(order: &OrderBase) -> Result<Decimal, EntityError> {
    order
        .limit_px
        .checked_mul(order.size)
        .and_then(|v| v.checked_mul(Decimal::from(order.buy_quote_fee_reserve_bps)))
        .and_then(|v| v.checked_div(Decimal::from(10_000u64)))
        .map(|v| {
            v.round_dp_with_strategy(
                order.symbol.quote.scale(),
                rust_decimal::RoundingStrategy::ToPositiveInfinity,
            )
        })
        .ok_or_else(|| err("fee reserve target overflow"))
}

pub(super) fn buy_total_held(order: &OrderBase) -> Result<Decimal, SpotAmountConversionError> {
    let principal_held = order
        .buy_quote_principal_frozen
        .checked_sub(order.buy_quote_principal_paid)
        .and_then(|amount| amount.checked_sub(order.buy_quote_principal_unfrozen))
        .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
    let fee_held = order
        .buy_quote_fee_reserve_frozen
        .checked_sub(order.buy_quote_fee_reserve_paid)
        .and_then(|amount| amount.checked_sub(order.buy_quote_fee_reserve_unfrozen))
        .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
    principal_held
        .checked_add(fee_held)
        .ok_or(SpotAmountConversionError::ArithmeticOverflow)
}

fn settle_terminal_unfreeze(next: &mut OrderBase) -> Result<(), EntityError> {
    if next.side == OrderSide::Sell {
        if next.status != SpotOrderStatus::Filled {
            let filled = next
                .orig_size
                .checked_sub(next.size)
                .ok_or_else(|| err("sell fill underflow"))?;
            let release = next
                .sell_base_principal_frozen
                .checked_sub(filled)
                .and_then(|v| v.checked_sub(next.sell_base_principal_unfrozen))
                .filter(|v| *v >= Decimal::ZERO)
                .ok_or_else(|| err("sell principal classification exceeds frozen principal"))?;
            next.sell_base_principal_unfrozen = next
                .sell_base_principal_unfrozen
                .checked_add(release)
                .ok_or_else(|| err("sell unfreeze overflow"))?;
        }
        return Ok(());
    }
    if next.side != OrderSide::Buy {
        return Ok(());
    }
    let principal_accounted = next
        .buy_quote_principal_paid
        .checked_add(next.buy_quote_principal_unfrozen)
        .ok_or_else(|| err("principal accounting overflow"))?;
    let principal_release = next
        .buy_quote_principal_frozen
        .checked_sub(principal_accounted)
        .filter(|amount| *amount >= Decimal::ZERO)
        .ok_or_else(|| err("buy principal classification exceeds frozen principal"))?;
    next.buy_quote_principal_unfrozen = next
        .buy_quote_principal_unfrozen
        .checked_add(principal_release)
        .ok_or_else(|| err("principal unfreeze overflow"))?;

    let fee_accounted = next
        .buy_quote_fee_reserve_paid
        .checked_add(next.buy_quote_fee_reserve_unfrozen)
        .ok_or_else(|| err("fee accounting overflow"))?;
    let fee_unfreeze = next
        .buy_quote_fee_reserve_frozen
        .checked_sub(fee_accounted)
        .filter(|amount| *amount >= Decimal::ZERO)
        .ok_or_else(|| err("fee classification exceeds reserve"))?;
    next.buy_quote_fee_reserve_unfrozen = next
        .buy_quote_fee_reserve_unfrozen
        .checked_add(fee_unfreeze)
        .ok_or_else(|| err("fee unfreeze overflow"))?;
    validate_terminal_buy_conservation(next)
}

fn validate_terminal_buy_conservation(order: &OrderBase) -> Result<(), EntityError> {
    let remainder = order
        .buy_quote_principal_frozen
        .checked_add(order.buy_quote_fee_reserve_frozen)
        .and_then(|amount| amount.checked_sub(order.buy_quote_fee_reserve_paid))
        .and_then(|amount| amount.checked_sub(order.buy_quote_principal_paid))
        .and_then(|amount| amount.checked_sub(order.buy_quote_principal_unfrozen))
        .and_then(|amount| amount.checked_sub(order.buy_quote_fee_reserve_unfrozen))
        .ok_or_else(|| err("buy funds conservation overflow"))?;
    if remainder != Decimal::ZERO {
        return Err(err("terminal buy funds are not conserved"));
    }
    Ok(())
}
#[cfg(test)]
#[path = "spot_order_v7_tests.rs"]
mod spot_order_v7_tests;
