use common_entity::{Entity, EntityError, EntityFieldChange, FieldDiff, Fsm, Shard};
use lob_repo_v2::lob::LobOrder;
use rust_decimal::Decimal;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::common::PriceType;
use crate::entity::common::common_order::{OrderBase, SpotAmountConversionError};
use crate::entity::common::order_primitives::{
    OrderGroupRelation, OrderSide, OrderTif, SpotOrderStatus,
};

#[derive(Debug, Clone, PartialEq, Eq)]

///todo 创建(冻结)，撮合(fill,balance)，修改(金额变化)，取消(金额变化)
///todo 可以在根聚合上加领域服务，因为不少动作确实涉及多个聚合的变化。
pub enum SpotOrderV7Event {
    Activate {
        timestamp: u64,
    },
    /// 将已规划的全部 taker 成交一次性提交到订单聚合。
    MatchedAsTaker {
        filled_size: Decimal,
        quote_fee: Decimal,
        timestamp: u64,
    },
    /// 将一笔已规划的 maker 成交提交到订单聚合。
    MatchedAsMaker {
        filled_size: Decimal,
        quote_fee: Decimal,
        timestamp: u64,
    },
    /// ALO 遇到可成交对手盘时拒绝 taker。
    RejectedAsTaker {
        timestamp: u64,
    },
    Modify {
        price: Decimal,
        size: Decimal,
        tif: Option<crate::entity::common::order_primitives::OrderTif>,
        cloid: Option<String>,
        timestamp: u64,
    },
    Cancel {
        timestamp: u64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderV7FreezeRequirement {
    Buy { quote_principal: CoinAmount, quote_fee_reserve: CoinAmount, total_quote: CoinAmount },
    Sell { base_quantity: CoinAmount },
}

impl SpotOrderV7FreezeRequirement {
    pub fn currency(self) -> crate::account::coin::Coin {
        match self {
            Self::Buy { total_quote, .. } => total_quote.currency(),
            Self::Sell { base_quantity } => base_quantity.currency(),
        }
    }

    pub fn amount(self) -> Decimal {
        match self {
            Self::Buy { total_quote, .. } => total_quote.amount(),
            Self::Sell { base_quantity } => base_quantity.amount(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivateSpotOrderV7Output {
    pub freeze_requirement: SpotOrderV7FreezeRequirement,
}

/// 撮合角色事件提交后的订单事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplySpotOrderV7MatchOutput {
    pub order_after: SpotOrderV7,
}

/// 改单后订单对持仓冻结额的调整要求。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotOrderV7HoldAdjustment {
    /// 冻结额不变；`currency` 仍明确指出订单占用的资产。
    Unchanged { currency: crate::account::coin::Coin },
    /// 需要额外冻结资产。
    Increase { amount: CoinAmount },
    /// 需要释放已冻结资产。
    Decrease { amount: CoinAmount },
}

impl SpotOrderV7HoldAdjustment {
    pub fn currency(self) -> crate::account::coin::Coin {
        match self {
            Self::Unchanged { currency } => currency,
            Self::Increase { amount } | Self::Decrease { amount } => amount.currency(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV7Output {
    pub order_after: SpotOrderV7,
    pub hold_adjustment: SpotOrderV7HoldAdjustment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV7Output {
    pub order_after: SpotOrderV7,
    pub release_requirement: CoinAmount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV7Output {
    Activated(ActivateSpotOrderV7Output),
    MatchedAsTaker(ApplySpotOrderV7MatchOutput),
    MatchedAsMaker(ApplySpotOrderV7MatchOutput),
    RejectedAsTaker(ApplySpotOrderV7MatchOutput),
    Modified(ModifySpotOrderV7Output),
    Canceled(CancelSpotOrderV7Output),
}

#[derive(Debug, Clone, PartialEq, Eq)]

///todo 增加累计 冻结金额，解冻金额，支付金额（fill,fee);增加增量资金变化 1，创建冻结，2，fill, 3,cancel, 4.modify 用以创建balance的event
pub struct SpotOrderV7 {
    order_base: OrderBase,
}

impl SpotOrderV7 {
    pub fn single(mut order: OrderBase) -> Self {
        order.status = SpotOrderStatus::Pending;
        order.group_relation = OrderGroupRelation::Standalone;
        Self { order_base: order }
    }

    pub fn normal_tpsl(parent: OrderBase, children: Vec<OrderBase>) -> (Self, Vec<Self>) {
        let mut parent = Self::single(parent);
        parent.order_base.group_relation = OrderGroupRelation::NormalTpslParent;
        let id = parent.order_base.order_id;
        let children = children
            .into_iter()
            .map(|mut child| {
                child.status = SpotOrderStatus::Pending;
                child.group_relation = OrderGroupRelation::NormalTpslChild { parent_order_id: id };
                Self { order_base: child }
            })
            .collect();
        (parent, children)
    }

    pub fn from_record(order: OrderBase) -> Result<Self, EntityError> {
        validate_record(&order)?;
        Ok(Self { order_base: order })
    }
    pub fn order_base(&self) -> &OrderBase {
        &self.order_base
    }
    pub fn status(&self) -> SpotOrderStatus {
        self.order_base.status
    }
    pub fn freeze_requirement(
        &self,
    ) -> Result<SpotOrderV7FreezeRequirement, SpotAmountConversionError> {
        freeze_requirement(&self.order_base)
    }
    pub fn release_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        if !matches!(
            self.status(),
            SpotOrderStatus::Canceled | SpotOrderStatus::Filled | SpotOrderStatus::Rejected
        ) {
            return Err(SpotAmountConversionError::OrderNotCanceled);
        }
        let original = self.status() == SpotOrderStatus::Rejected;
        release_requirement(&self.order_base, original)
    }

    pub fn track_create_event(&self) -> Result<common_entity::EntityReplayableEvent, EntityError> {
        Entity::track_create_event(self)
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

    ///todo 可以remove
    pub fn modify(
        &mut self,
        price: Decimal,
        size: Decimal,
        timestamp: u64,
    ) -> Result<(), EntityError> {
        self.apply(SpotOrderV7Event::Modify { price, size, tif: None, cloid: None, timestamp })
            .map(|_| ())
    }
}

impl Fsm for SpotOrderV7 {
    type State = SpotOrderStatus;
    type BizEvent = SpotOrderV7Event;
    type Output = SpotOrderV7Output;
    type Error = EntityError;

    fn transition(
        &mut self,
        event: SpotOrderV7Event,
    ) -> Result<(Self::State, Self::Output), EntityError> {
        if matches!(
            self.order_base.status,
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) {
            return Err(err("order is terminal"));
        }
        let (next, output) = match event {
            SpotOrderV7Event::Activate { timestamp } => {
                if self.status() != SpotOrderStatus::Pending {
                    return Err(err("order is not pending"));
                }
                let freeze_requirement = freeze_requirement(&self.order_base)
                    .map_err(|error| err(&error.to_string()))?;
                let mut next = self.order_base.clone();
                next.status = SpotOrderStatus::Open;
                next.updated_at = timestamp;
                next.version = next
                    .version
                    .checked_add(1)
                    .ok_or(EntityError::VersionOverflow { version: next.version })?;
                (
                    next,
                    SpotOrderV7Output::Activated(ActivateSpotOrderV7Output { freeze_requirement }),
                )
            }
            SpotOrderV7Event::MatchedAsTaker { filled_size, quote_fee, timestamp } => {
                self.validate_matchable_as_taker()?;
                let next = matched_order(
                    &self.order_base,
                    filled_size,
                    quote_fee,
                    timestamp,
                    MatchRole::Taker,
                )?;
                let order_after = SpotOrderV7 { order_base: next.clone() };
                (
                    next,
                    SpotOrderV7Output::MatchedAsTaker(ApplySpotOrderV7MatchOutput { order_after }),
                )
            }
            SpotOrderV7Event::MatchedAsMaker { filled_size, quote_fee, timestamp } => {
                self.validate_matchable_as_maker()?;
                let next = matched_order(
                    &self.order_base,
                    filled_size,
                    quote_fee,
                    timestamp,
                    MatchRole::Maker,
                )?;
                let order_after = SpotOrderV7 { order_base: next.clone() };
                (
                    next,
                    SpotOrderV7Output::MatchedAsMaker(ApplySpotOrderV7MatchOutput { order_after }),
                )
            }
            SpotOrderV7Event::RejectedAsTaker { timestamp } => {
                self.validate_matchable_as_taker()?;
                if self.order_base.effective_tif() != OrderTif::Alo {
                    return Err(err("order cannot be rejected as ALO taker"));
                }
                let mut next = self.order_base.clone();
                next.status = SpotOrderStatus::Rejected;
                next.updated_at = timestamp;
                next.version = next
                    .version
                    .checked_add(1)
                    .ok_or(EntityError::VersionOverflow { version: next.version })?;
                let order_after = SpotOrderV7 { order_base: next.clone() };
                (
                    next,
                    SpotOrderV7Output::RejectedAsTaker(ApplySpotOrderV7MatchOutput { order_after }),
                )
            }
            SpotOrderV7Event::Modify { price, size, tif, cloid, timestamp } => {
                let hold_before = active_hold_requirement(&self.order_base)?;
                let mut next = modified_order(&self.order_base, price, size, timestamp)?;
                if let Some(tif) = tif {
                    next.tif = tif;
                }
                if let Some(cloid) = cloid {
                    next.cloid = Some(cloid);
                }
                let hold_after = active_hold_requirement(&next)?;
                let hold_adjustment = hold_adjustment(&self.order_base, hold_before, hold_after)?;
                let order_after = SpotOrderV7 { order_base: next.clone() };
                (
                    next,
                    SpotOrderV7Output::Modified(ModifySpotOrderV7Output {
                        order_after,
                        hold_adjustment,
                    }),
                )
            }
            SpotOrderV7Event::Cancel { timestamp } => {
                let was_pending = self.status() == SpotOrderStatus::Pending;
                let next = canceled_order(&self.order_base, timestamp)?;
                let order_after = SpotOrderV7 { order_base: next.clone() };
                let release_requirement = if was_pending {
                    zero_release_requirement(&order_after)?
                } else {
                    entity_release_requirement(&order_after)?
                };
                (
                    next,
                    SpotOrderV7Output::Canceled(CancelSpotOrderV7Output {
                        order_after,
                        release_requirement,
                    }),
                )
            }
        };
        self.order_base = next;
        Ok((self.order_base.status, output))
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
    quote_fee: Decimal,
    timestamp: u64,
    role: MatchRole,
) -> Result<OrderBase, EntityError> {
    if filled_size < Decimal::ZERO || filled_size > current.size || quote_fee < Decimal::ZERO {
        return Err(err("invalid match facts"));
    }
    if matches!(role, MatchRole::Maker) && filled_size == Decimal::ZERO {
        return Err(err("maker fill must be greater than zero"));
    }

    let mut next = current.clone();
    next.size = next.size.checked_sub(filled_size).ok_or_else(|| err("size underflow"))?;
    if next.side == OrderSide::Buy {
        next.buy_quote_fee_reserve_consumed = next
            .buy_quote_fee_reserve_consumed
            .checked_add(quote_fee)
            .ok_or_else(|| err("fee overflow"))?;
        if next.buy_quote_fee_reserve_consumed > next.buy_quote_fee_reserve_limit {
            return Err(err("fee exceeds reserve"));
        }
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
    next.updated_at = timestamp;
    next.version = next
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: next.version })?;
    Ok(next)
}

fn modified_order(
    current: &OrderBase,
    price: Decimal,
    size: Decimal,
    timestamp: u64,
) -> Result<OrderBase, EntityError> {
    let filled =
        current.orig_size.checked_sub(current.size).ok_or_else(|| err("invalid filled size"))?;
    if price <= Decimal::ZERO || size <= Decimal::ZERO || size < filled {
        return Err(err("invalid modified order"));
    }
    let mut next = current.clone();
    next.limit_px = price;
    next.orig_size = size;
    next.size = size.checked_sub(filled).ok_or_else(|| err("size overflow"))?;
    next.updated_at = timestamp;
    next.version = current
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: current.version })?;
    Ok(next)
}

fn canceled_order(current: &OrderBase, timestamp: u64) -> Result<OrderBase, EntityError> {
    let mut next = current.clone();
    next.status = SpotOrderStatus::Canceled;
    next.updated_at = timestamp;
    next.version = current
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: current.version })?;
    Ok(next)
}

impl FieldDiff for SpotOrderV7 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.order_base.order_id.to_string()),
            EntityFieldChange::new("status", "", format!("{:?}", self.status())),
            EntityFieldChange::new("version", "", self.order_base.version.to_string()),
        ]
    }
    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let a = &self.order_base;
        let b = &other.order_base;
        let mut v = Vec::new();
        for (n, x, y) in [
            ("limit_px", a.limit_px.to_string(), b.limit_px.to_string()),
            ("size", a.size.to_string(), b.size.to_string()),
            ("orig_size", a.orig_size.to_string(), b.orig_size.to_string()),
            ("status", format!("{:?}", a.status), format!("{:?}", b.status)),
            ("updated_at", a.updated_at.to_string(), b.updated_at.to_string()),
            ("version", a.version.to_string(), b.version.to_string()),
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
        self.order_base.version
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
        if matches!(name, "updated_at" | "version") { 1 } else { 0 }
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
        self.order_base.version
    }
    fn is_active(&self) -> bool {
        matches!(
            self.status(),
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) && self.order_base.size > Decimal::ZERO
    }
    fn from_replay_event(
        _: &common_entity::EntityReplayableEvent,
        _: u64,
    ) -> Result<Self, lob_repo_v2::LobError> {
        Err(lob_repo_v2::LobError::Event("SpotOrderV7 replay construction is not wired".into()))
    }
    fn apply_replay_event(
        self,
        _: &common_entity::EntityReplayableEvent,
    ) -> Result<Self, lob_repo_v2::LobError> {
        Err(lob_repo_v2::LobError::Event("SpotOrderV7 replay update is not wired".into()))
    }
}

fn err(s: &str) -> EntityError {
    EntityError::Custom(s.to_owned())
}
fn validate_record(o: &OrderBase) -> Result<(), EntityError> {
    if !o.has_consistent_execution_facts()
        || o.orig_size <= Decimal::ZERO
        || o.size < Decimal::ZERO
        || o.size > o.orig_size
        || o.limit_px <= Decimal::ZERO
    {
        return Err(err("invalid order facts"));
    }
    Ok(())
}
fn validate_matchable(o: &OrderBase) -> Result<(), EntityError> {
    validate_record(o)?;
    if !matches!(o.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
        || o.size <= Decimal::ZERO
    {
        return Err(err("order is not matchable"));
    }
    Ok(())
}
fn freeze_requirement(
    o: &OrderBase,
) -> Result<SpotOrderV7FreezeRequirement, SpotAmountConversionError> {
    if o.orig_size <= Decimal::ZERO || o.buy_quote_fee_reserve_limit < Decimal::ZERO {
        return Err(SpotAmountConversionError::InvalidAmount);
    }
    match o.side {
        OrderSide::Buy => {
            let principal = o
                .limit_px
                .checked_mul(o.orig_size)
                .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
            let total = principal
                .checked_add(o.buy_quote_fee_reserve_limit)
                .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
            Ok(SpotOrderV7FreezeRequirement::Buy {
                quote_principal: coin_amount(o.symbol.quote, principal)?,
                quote_fee_reserve: coin_amount(o.symbol.quote, o.buy_quote_fee_reserve_limit)?,
                total_quote: coin_amount(o.symbol.quote, total)?,
            })
        }
        OrderSide::Sell => Ok(SpotOrderV7FreezeRequirement::Sell {
            base_quantity: coin_amount(o.symbol.base, o.orig_size)?,
        }),
    }
}
fn coin_amount(
    coin: crate::account::coin::Coin,
    amount: Decimal,
) -> Result<CoinAmount, SpotAmountConversionError> {
    CoinAmount::new(coin, amount)
        .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
}
fn zero_release_requirement(order: &SpotOrderV7) -> Result<CoinAmount, EntityError> {
    let coin = match order.order_base.side {
        OrderSide::Buy => order.order_base.symbol.quote,
        OrderSide::Sell => order.order_base.symbol.base,
    };
    coin_amount(coin, Decimal::ZERO).map_err(|error| err(&error.to_string()))
}
fn release_requirement(
    o: &OrderBase,
    original: bool,
) -> Result<CoinAmount, SpotAmountConversionError> {
    let size = if original { o.orig_size } else { o.size };
    let amount = match o.side {
        OrderSide::Buy => o
            .limit_px
            .checked_mul(size)
            .and_then(|x| x.checked_add(o.buy_quote_fee_reserve_limit))
            .and_then(|x| x.checked_sub(o.buy_quote_fee_reserve_consumed))
            .ok_or(SpotAmountConversionError::ArithmeticOverflow)?,
        OrderSide::Sell => size,
    };
    CoinAmount::new(if o.side == OrderSide::Buy { o.symbol.quote } else { o.symbol.base }, amount)
        .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
}
fn entity_release_requirement(order: &SpotOrderV7) -> Result<CoinAmount, EntityError> {
    order.release_requirement().map_err(|error| err(&error.to_string()))
}
fn held_currency(o: &OrderBase) -> crate::account::coin::Coin {
    match o.side {
        OrderSide::Buy => o.symbol.quote,
        OrderSide::Sell => o.symbol.base,
    }
}
fn active_hold_requirement(o: &OrderBase) -> Result<Option<Decimal>, EntityError> {
    if o.status == SpotOrderStatus::Pending {
        return Ok(None);
    }
    let amount = match o.side {
        OrderSide::Buy => o
            .limit_px
            .checked_mul(o.size)
            .and_then(|principal| principal.checked_add(o.buy_quote_fee_reserve_limit))
            .and_then(|reserved| reserved.checked_sub(o.buy_quote_fee_reserve_consumed))
            .filter(|amount| *amount >= Decimal::ZERO)
            .ok_or_else(|| err("invalid active buy freeze requirement"))?,
        OrderSide::Sell => o.size,
    };
    Ok(Some(amount))
}
fn hold_adjustment(
    o: &OrderBase,
    before: Option<Decimal>,
    after: Option<Decimal>,
) -> Result<SpotOrderV7HoldAdjustment, EntityError> {
    let currency = held_currency(o);
    let (Some(before), Some(after)) = (before, after) else {
        return Ok(SpotOrderV7HoldAdjustment::Unchanged { currency });
    };
    match after.cmp(&before) {
        std::cmp::Ordering::Greater => Ok(SpotOrderV7HoldAdjustment::Increase {
            amount: coin_amount(
                currency,
                after.checked_sub(before).ok_or_else(|| err("modify freeze delta overflow"))?,
            )
            .map_err(|error| err(&error.to_string()))?,
        }),
        std::cmp::Ordering::Less => Ok(SpotOrderV7HoldAdjustment::Decrease {
            amount: coin_amount(
                currency,
                before.checked_sub(after).ok_or_else(|| err("modify release delta overflow"))?,
            )
            .map_err(|error| err(&error.to_string()))?,
        }),
        std::cmp::Ordering::Equal => Ok(SpotOrderV7HoldAdjustment::Unchanged { currency }),
    }
}
#[cfg(test)]
#[path = "spot_order_v7_tests.rs"]
mod spot_order_v7_tests;
