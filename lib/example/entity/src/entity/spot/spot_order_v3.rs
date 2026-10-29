use std::marker::PhantomData;

use common_entity::{
    Entity, EntityError, EntityFieldChange, EntityReplayableEvent, FieldDiff, ReplayFieldChange,
};
use rust_decimal::Decimal;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::{
    CommonOrderExecution, OrderBase, SpotAmountConversionError,
};
use crate::entity::common::common_trade::TradeBase;
use crate::entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
use crate::entity::spot::spot_order_v2::OrderGroupRelation;
use crate::spot::spot_trade_v3::SpotTradeV3;

/// 订单生命周期的 typestate 标记。
pub trait SpotOrderStage:
    Clone + Copy + PartialEq + Eq + std::fmt::Debug + Send + Sync + 'static
{
    const STATUS: SpotOrderStatus;
}

/// 已受理，等待进入执行流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pending;
impl SpotOrderStage for Pending {
    const STATUS: SpotOrderStatus = SpotOrderStatus::Pending;
}
/// 已进入执行流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Open;
impl SpotOrderStage for Open {
    const STATUS: SpotOrderStatus = SpotOrderStatus::Open;
}
/// 部分成交。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartiallyFilled;
impl SpotOrderStage for PartiallyFilled {
    const STATUS: SpotOrderStatus = SpotOrderStatus::PartiallyFilled;
}
/// 完全成交。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filled;
impl SpotOrderStage for Filled {
    const STATUS: SpotOrderStatus = SpotOrderStatus::Filled;
}
/// 已撤销。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Canceled;
impl SpotOrderStage for Canceled {
    const STATUS: SpotOrderStatus = SpotOrderStatus::Canceled;
}
/// 已拒绝。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rejected;
impl SpotOrderStage for Rejected {
    const STATUS: SpotOrderStatus = SpotOrderStatus::Rejected;
}

/// 从持久化记录动态分派出的订单状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderV3Any {
    Pending(SpotOrderV3<Pending>),
    Open(SpotOrderV3<Open>),
    PartiallyFilled(SpotOrderV3<PartiallyFilled>),
    Filled(SpotOrderV3<Filled>),
    Canceled(SpotOrderV3<Canceled>),
    Rejected(SpotOrderV3<Rejected>),
}

/// 撮合批次的时间、费用和稳定成交编号输入。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderV3Input {
    /// 本批第一笔成交的稳定 ID，后续成交按序递增。
    pub trade_id_start: u64,
    /// maker 费率，单位为万分比（bps）。
    pub maker_fee_bps: u64,
    /// taker 费率，单位为万分比（bps）。
    pub taker_fee_bps: u64,
    /// 成交及订单状态变更时间，单位为 Unix 纳秒。
    pub timestamp: u64,
}

/// 撮合后对 taker 的五种业务结论。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult {
    /// 没有成交，订单继续留在订单簿；本次没有其它业务副作用。
    Resting { taker_order_after: SpotOrderV3<Open> },
    /// 发生成交但仍有剩余数量，订单继续可撮合。
    PartiallyFilled {
        taker_order_after: SpotOrderV3<PartiallyFilled>,
        maker_orders_after: Vec<SpotOrderV3Any>,
        created_trades: Vec<SpotTradeV3>,
    },
    /// 成交数量达到订单总数量，订单终止为 Filled。
    Filled {
        filled_taker_order_after: SpotOrderV3<Filled>,
        maker_orders_after: Vec<SpotOrderV3Any>,
        created_trades: Vec<SpotTradeV3>,
    },
    /// IOC 部分成交后，剩余数量被取消并释放冻结。
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV3<Canceled>,
        maker_orders_after: Vec<SpotOrderV3Any>,
        created_trades: Vec<SpotTradeV3>,
    },
    /// ALO crossing maker 或 IOC 无流动性导致拒单。
    Rejected { rejected_taker_order_after: SpotOrderV3<Rejected> },
}

/// 现货委托这一 Moment-Interval 业务事实及其生命周期类型。
///
/// `OrderBase` 保存稳定事实；新创建入口返回 Pending，加载入口保留记录中的生命周期状态。
/// 撮合行为负责验证事实并返回相应状态的 after truth。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3<S> {
    comon_order: OrderBase,
    stage: PhantomData<S>,
}

impl SpotOrderV3<Pending> {
    /// 依 maker 顺序撮合 Pending taker，并返回更新后的订单和成交事实。
    ///
    /// 成交费按 quote 币精度向上取整。订单仅在全部检查成功后作为结果返回。
    pub fn match_with_input(
        &self,
        makers: Vec<SpotOrderV3Any>,
        input: MatchSpotOrderV3Input,
    ) -> Result<MatchResult, EntityError> {
        let mut taker = self.comon_order.clone();
        validate_order(&taker)?;
        let mut makers = makers;
        let tif = taker.effective_tif();

        if tif == OrderTif::Alo {
            if let Some(maker) = makers.first() {
                validate_pair(&taker, maker.order_base())?;
                if crosses(&taker, maker.order_base()) {
                    transition(&mut taker, SpotOrderStatus::Rejected, input.timestamp)?;
                    return Ok(MatchResult::Rejected {
                        rejected_taker_order_after: with_stage(taker)?,
                    });
                }
            }
            return resting_result(taker, makers, input.timestamp);
        }

        let mut trades = Vec::new();
        for maker in &mut makers {
            validate_pair(&taker, maker.order_base())?;
            if !crosses(&taker, maker.order_base()) {
                break;
            }

            let maker_base = maker.order_base();
            let size = taker.size.min(maker_base.size);
            let price = maker_base.limit_px;
            let notional =
                price.checked_mul(size).ok_or_else(|| entity_error("trade notional overflow"))?;
            let trade_offset =
                u64::try_from(trades.len()).map_err(|_| entity_error("trade id overflow"))?;
            let trade_id = input
                .trade_id_start
                .checked_add(trade_offset)
                .ok_or_else(|| entity_error("trade id overflow"))?;
            let taker_fee = fee_amount(notional, input.taker_fee_bps, taker.symbol.quote.scale())?;
            let maker_fee =
                fee_amount(notional, input.maker_fee_bps, maker.order_base().symbol.quote.scale())?;
            let next_taker = apply_fill(&taker, size, taker_fee, input.timestamp)?;
            let next_maker = apply_fill(maker.order_base(), size, maker_fee, input.timestamp)?;

            trades.push(SpotTradeV3 {
                comon_trade: TradeBase {
                    trade_id,
                    symbol: taker.symbol.clone(),
                    taker_user: taker.owner,
                    maker_user: maker.order_base().owner,
                    taker_order_id: taker.order_id,
                    maker_order_id: maker.order_base().order_id,
                    taker_side: taker.side,
                    price,
                    size,
                    taker_fee,
                    maker_fee,
                    created_at: input.timestamp,
                    updated_at: input.timestamp,
                    version: 0,
                },
            });
            taker = next_taker;
            *maker = SpotOrderV3Any::from_record(next_maker)?;
            if taker.size == Decimal::ZERO {
                break;
            }
        }

        finalize_match(taker, makers, trades, tif, input.timestamp)
    }

    /// 以零费率执行兼容撮合；正式调用应提供批次费率和时间。
    pub fn match_order(&self, makers: Vec<SpotOrderV3Any>) -> Result<MatchResult, EntityError> {
        self.match_with_input(
            makers,
            MatchSpotOrderV3Input {
                trade_id_start: self.comon_order.order_id,
                maker_fee_bps: 0,
                taker_fee_bps: 0,
                timestamp: self.comon_order.updated_at,
            },
        )
    }
}

impl SpotOrderV3<Pending> {
    /// 将新建委托投影为完整字段创建事件，使用独立的 V3 实体类型 16。
    ///
    /// 字段值使用 OrderBase 的 JSON 表示，field_type 为 0；父子事件顺序由用例维护。
    /// 任一字段超过回放协议的 64 字节容量时返回错误，避免静默截断。
    pub fn track_create_event(&self) -> Result<EntityReplayableEvent, EntityError> {
        let record = &self.comon_order;
        let fields =
            serde_json::to_value(record).map_err(|error| EntityError::Custom(error.to_string()))?;
        let fields = fields
            .as_object()
            .ok_or_else(|| EntityError::Custom("order record must be an object".to_owned()))?;
        let mut event = EntityReplayableEvent::new(
            record.created_at,
            0,
            record.version,
            record.order_id,
            16,
            0,
        );
        for (name, value) in fields {
            let value = value.to_string();
            if value.len() > 64 {
                return Err(EntityError::FieldParseError {
                    field: name.clone(),
                    reason: "created field exceeds 64-byte replay capacity".to_owned(),
                });
            }
            event.field_changes.push(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(name),
                &[],
                value.as_bytes(),
                0,
            ));
        }
        Ok(event)
    }

    /// 仅装配已校验事实，规范化为独立 Pending 委托；其余字段原样保留。
    pub fn single(mut order: OrderBase) -> Self {
        order.status = SpotOrderStatus::Pending;
        order.group_relation = OrderGroupRelation::Standalone;
        Self { comon_order: order, stage: PhantomData }
    }

    /// 仅装配已校验的 normalTpsl 父子委托，保持子单顺序。
    ///
    /// 调用方须先校验子单非空及父子业务约束。本方法不重复 admission 校验；
    /// 即使传入空子单，也只返回 Pending 父单与空集合，不 panic。
    /// 除状态和关系外，所有订单事实保持不变。
    pub fn normal_tpsl(parent: OrderBase, children: Vec<OrderBase>) -> (Self, Vec<Self>) {
        let mut parent = Self::single(parent);
        parent.comon_order.group_relation = OrderGroupRelation::NormalTpslParent;
        let parent_order_id = parent.comon_order.order_id;
        let children = children
            .into_iter()
            .map(|child| {
                let mut child = Self::single(child);
                child.comon_order.group_relation =
                    OrderGroupRelation::NormalTpslChild { parent_order_id };
                child
            })
            .collect();
        (parent, children)
    }
}

impl SpotOrderV3Any {
    /// 按记录状态校验并分派为对应的静态生命周期类型。
    pub fn from_record(order: OrderBase) -> Result<Self, EntityError> {
        Ok(match order.status {
            SpotOrderStatus::Pending => Self::Pending(SpotOrderV3::try_from_record(order)?),
            SpotOrderStatus::Open => Self::Open(SpotOrderV3::try_from_record(order)?),
            SpotOrderStatus::PartiallyFilled => {
                Self::PartiallyFilled(SpotOrderV3::try_from_record(order)?)
            }
            SpotOrderStatus::Filled => Self::Filled(SpotOrderV3::try_from_record(order)?),
            SpotOrderStatus::Canceled => Self::Canceled(SpotOrderV3::try_from_record(order)?),
            SpotOrderStatus::Rejected => Self::Rejected(SpotOrderV3::try_from_record(order)?),
        })
    }

    pub fn order_base(&self) -> &OrderBase {
        match self {
            Self::Pending(order) => order.order_base(),
            Self::Open(order) => order.order_base(),
            Self::PartiallyFilled(order) => order.order_base(),
            Self::Filled(order) => order.order_base(),
            Self::Canceled(order) => order.order_base(),
            Self::Rejected(order) => order.order_base(),
        }
    }
}

impl<S: SpotOrderStage> SpotOrderV3<S> {
    /// 仅装配状态与 typestate 一致的持久化订单记录。
    pub fn try_from_record(order: OrderBase) -> Result<Self, EntityError> {
        if order.status != S::STATUS {
            return Err(entity_error("order status does not match typestate"));
        }
        Ok(Self { comon_order: order, stage: PhantomData })
    }

    /// 委托的只读事实快照，供查询和持久化投影使用。
    pub fn order_base(&self) -> &OrderBase {
        &self.comon_order
    }

    /// 返回订单需要预冻结的单币种金额，不修改订单状态。
    ///
    /// 买单冻结 quote 币的原始数量限价金额及最大 taker 手续费；卖单冻结
    /// base 币原始数量，手续费从卖出所得中支付。
    pub fn freeze_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let order = &self.comon_order;
        let orig_size = order.orig_size;
        let fee = order.taker_max_fee;

        if orig_size <= Decimal::ZERO || fee < Decimal::ZERO {
            return Err(SpotAmountConversionError::InvalidAmount);
        }

        match order.side {
            OrderSide::Buy => {
                if order.limit_px <= Decimal::ZERO {
                    return Err(SpotAmountConversionError::InvalidAmount);
                }

                let amount = order
                    .limit_px
                    .checked_mul(orig_size)
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?
                    .checked_add(fee)
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;

                CoinAmount::new(order.symbol.quote, amount).map_err(map_coin_amount_error)
            }
            OrderSide::Sell => {
                CoinAmount::new(order.symbol.base, orig_size).map_err(map_coin_amount_error)
            }
        }
    }
}

impl SpotOrderV3<Canceled> {
    /// 取消尚未终止的 Pending 订单，返回 Canceled after truth。
    pub fn cancel_pending(&self, timestamp: u64) -> Result<Self, EntityError> {
        cancel_order(&self.comon_order, timestamp)
    }

    /// 返回撤单后仍可释放的单币种冻结金额，不修改余额。
    pub fn cancel_release_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let order = &self.comon_order;
        if order.size < Decimal::ZERO
            || order.orig_size <= Decimal::ZERO
            || order.size > order.orig_size
            || (order.side == OrderSide::Buy && order.limit_px <= Decimal::ZERO)
            || order.taker_max_fee < Decimal::ZERO
            || order.fee_consumed < Decimal::ZERO
            || order.fee_consumed > order.taker_max_fee
        {
            return Err(SpotAmountConversionError::InvalidAmount);
        }
        let amount = match order.side {
            OrderSide::Buy => order
                .limit_px
                .checked_mul(order.size)
                .and_then(|value| value.checked_add(order.taker_max_fee))
                .and_then(|value| value.checked_sub(order.fee_consumed))
                .ok_or(SpotAmountConversionError::ArithmeticOverflow)?,
            OrderSide::Sell => order.size,
        };
        let currency =
            if order.side == OrderSide::Buy { order.symbol.quote } else { order.symbol.base };
        CoinAmount::new(currency, amount).map_err(map_coin_amount_error)
    }
}

impl SpotOrderV3<Open> {
    pub fn cancel(&self, timestamp: u64) -> Result<SpotOrderV3<Canceled>, EntityError> {
        cancel_order(&self.comon_order, timestamp)
    }
}

impl SpotOrderV3<PartiallyFilled> {
    pub fn cancel(&self, timestamp: u64) -> Result<SpotOrderV3<Canceled>, EntityError> {
        cancel_order(&self.comon_order, timestamp)
    }
}

impl SpotOrderV3<Pending> {
    pub fn cancel(&self, timestamp: u64) -> Result<SpotOrderV3<Canceled>, EntityError> {
        cancel_order(&self.comon_order, timestamp)
    }
}

fn cancel_order(order: &OrderBase, timestamp: u64) -> Result<SpotOrderV3<Canceled>, EntityError> {
    let mut order = order.clone();
    transition(&mut order, SpotOrderStatus::Canceled, timestamp)?;
    with_stage(order)
}

fn map_coin_amount_error(_: CoinAmountError) -> SpotAmountConversionError {
    SpotAmountConversionError::InexactConversion
}

fn entity_error(message: &str) -> EntityError {
    EntityError::Custom(message.to_owned())
}

fn validate_order(order: &OrderBase) -> Result<(), EntityError> {
    if !order.has_consistent_execution_facts()
        || order.orig_size <= Decimal::ZERO
        || order.size <= Decimal::ZERO
        || order.size > order.orig_size
        || order.limit_px <= Decimal::ZERO
    {
        return Err(entity_error("order has invalid execution facts"));
    }
    if !matches!(
        order.status,
        SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
    ) {
        return Err(entity_error("order is not matchable"));
    }
    if order.status == SpotOrderStatus::PartiallyFilled && order.size >= order.orig_size {
        return Err(entity_error("partially filled order has inconsistent size"));
    }
    Ok(())
}

fn validate_pair(taker: &OrderBase, maker: &OrderBase) -> Result<(), EntityError> {
    validate_order(maker)?;
    if maker.order_id == taker.order_id || maker.side == taker.side || maker.symbol != taker.symbol
    {
        return Err(entity_error("maker is not compatible with taker"));
    }
    if maker.status == SpotOrderStatus::Pending || maker.execution != CommonOrderExecution::Limit {
        return Err(entity_error("maker must be an open limit order"));
    }
    Ok(())
}

fn crosses(taker: &OrderBase, maker: &OrderBase) -> bool {
    match taker.side {
        OrderSide::Buy => taker.limit_px >= maker.limit_px,
        OrderSide::Sell => taker.limit_px <= maker.limit_px,
    }
}

fn apply_fill(
    order: &OrderBase,
    size: Decimal,
    fee_consumed: Decimal,
    timestamp: u64,
) -> Result<OrderBase, EntityError> {
    let remaining = order
        .size
        .checked_sub(size)
        .filter(|remaining| *remaining >= Decimal::ZERO)
        .ok_or_else(|| entity_error("trade exceeds order remaining size"))?;
    let version = order
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: order.version })?;
    let mut updated = order.clone();
    updated.size = remaining;
    updated.fee_consumed = updated
        .fee_consumed
        .checked_add(fee_consumed)
        .ok_or_else(|| entity_error("order fee arithmetic overflow"))?;
    if updated.fee_consumed > updated.taker_max_fee {
        return Err(entity_error("order fee exceeds reserved maximum"));
    }
    updated.status = if remaining == Decimal::ZERO {
        SpotOrderStatus::Filled
    } else {
        SpotOrderStatus::PartiallyFilled
    };
    updated.updated_at = timestamp;
    updated.version = version;
    Ok(updated)
}

fn transition(
    order: &mut OrderBase,
    status: SpotOrderStatus,
    timestamp: u64,
) -> Result<(), EntityError> {
    order.version = order
        .version
        .checked_add(1)
        .ok_or(EntityError::VersionOverflow { version: order.version })?;
    order.status = status;
    order.updated_at = timestamp;
    Ok(())
}

fn fee_amount(notional: Decimal, fee_bps: u64, scale: u32) -> Result<Decimal, EntityError> {
    let fee = notional
        .checked_mul(Decimal::from(fee_bps))
        .and_then(|amount| amount.checked_div(Decimal::from(10_000u64)))
        .ok_or_else(|| entity_error("trade fee arithmetic overflow"))?;
    Ok(fee.round_dp_with_strategy(scale, rust_decimal::RoundingStrategy::ToPositiveInfinity))
}

fn with_stage<S: SpotOrderStage>(comon_order: OrderBase) -> Result<SpotOrderV3<S>, EntityError> {
    if comon_order.status != S::STATUS {
        return Err(entity_error("order status does not match typestate"));
    }
    Ok(SpotOrderV3 { comon_order, stage: PhantomData })
}

impl<S> FieldDiff for SpotOrderV3<S> {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("order_id", "", self.comon_order.order_id.to_string()),
            EntityFieldChange::new("status", "", format!("{:?}", self.comon_order.status)),
            EntityFieldChange::new("updated_at", "", self.comon_order.updated_at.to_string()),
            EntityFieldChange::new("fee_consumed", "", self.comon_order.fee_consumed.to_string()),
            EntityFieldChange::new("version", "", self.comon_order.version.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        if self.comon_order.status != other.comon_order.status {
            changes.push(EntityFieldChange::new(
                "status",
                format!("{:?}", self.comon_order.status),
                format!("{:?}", other.comon_order.status),
            ));
        }
        if self.comon_order.updated_at != other.comon_order.updated_at {
            changes.push(EntityFieldChange::new(
                "updated_at",
                self.comon_order.updated_at.to_string(),
                other.comon_order.updated_at.to_string(),
            ));
        }
        if self.comon_order.fee_consumed != other.comon_order.fee_consumed {
            changes.push(EntityFieldChange::new(
                "fee_consumed",
                self.comon_order.fee_consumed.to_string(),
                other.comon_order.fee_consumed.to_string(),
            ));
        }
        if self.comon_order.version != other.comon_order.version {
            changes.push(EntityFieldChange::new(
                "version",
                self.comon_order.version.to_string(),
                other.comon_order.version.to_string(),
            ));
        }
        changes
    }
}

impl<S> Entity for SpotOrderV3<S>
where
    S: Clone + std::fmt::Debug + Send + Sync + 'static,
{
    fn entity_id(&self) -> u64 {
        self.comon_order.order_id
    }

    fn entity_type() -> u8 {
        16
    }

    fn is_derived() -> bool {
        false
    }

    fn lsn(&self) -> u64 {
        self.comon_order.version
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "updated_at" | "fee_consumed" | "version" => 1,
            _ => 0,
        }
    }
}

impl<S> common_entity::Shard for SpotOrderV3<S> {
    fn partition(&self) -> u8 {
        self.comon_order.symbol.base.index() as u8
    }
}

fn finalize_match(
    mut taker: OrderBase,
    makers: Vec<SpotOrderV3Any>,
    trades: Vec<SpotTradeV3>,
    tif: OrderTif,
    timestamp: u64,
) -> Result<MatchResult, EntityError> {
    if taker.size == Decimal::ZERO {
        return Ok(MatchResult::Filled {
            filled_taker_order_after: with_stage(taker)?,
            maker_orders_after: makers,
            created_trades: trades,
        });
    }

    if trades.is_empty() {
        return match tif {
            OrderTif::Ioc => {
                transition(&mut taker, SpotOrderStatus::Rejected, timestamp)?;
                Ok(MatchResult::Rejected { rejected_taker_order_after: with_stage(taker)? })
            }
            _ => resting_result(taker, makers, timestamp),
        };
    }

    if tif == OrderTif::Ioc {
        transition(&mut taker, SpotOrderStatus::Canceled, timestamp)?;
        return Ok(MatchResult::CanceledAfterPartialFill {
            canceled_taker_order_after: with_stage(taker)?,
            maker_orders_after: makers,
            created_trades: trades,
        });
    }

    Ok(MatchResult::PartiallyFilled {
        taker_order_after: with_stage(taker)?,
        maker_orders_after: makers,
        created_trades: trades,
    })
}

fn resting_result(
    mut taker: OrderBase,
    makers: Vec<SpotOrderV3Any>,
    timestamp: u64,
) -> Result<MatchResult, EntityError> {
    if taker.status == SpotOrderStatus::PartiallyFilled {
        return Ok(MatchResult::PartiallyFilled {
            taker_order_after: with_stage(taker)?,
            maker_orders_after: makers,
            created_trades: Vec::new(),
        });
    }
    if taker.status == SpotOrderStatus::Pending {
        transition(&mut taker, SpotOrderStatus::Open, timestamp)?;
    }
    Ok(MatchResult::Resting { taker_order_after: with_stage(taker)? })
}

#[cfg(test)]
mod tests {
    use base_types::UserId;
    use rust_decimal::Decimal;

    use super::{MatchResult, MatchSpotOrderV3Input, SpotOrderV3, SpotOrderV3Any};
    use crate::account::coin::{Coin, CoinAmount};
    use crate::account::symbol::Symbol;
    use crate::entity::common::common_order::{
        CommonOrderExecution, CommonOrderTrigger, OrderBase, SpotAmountConversionError,
    };
    use crate::entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
    use crate::entity::spot::spot_order_v2::OrderGroupRelation;

    fn order(
        side: OrderSide,
        limit_px: Decimal,
        orig_size: Decimal,
        fee: Decimal,
    ) -> SpotOrderV3<super::Pending> {
        SpotOrderV3::single(OrderBase {
            owner: UserId(1),
            order_id: 1,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px,
            size: Decimal::ZERO,
            orig_size,
            execution: CommonOrderExecution::Limit,
            tif: OrderTif::Gtc,
            trigger: CommonOrderTrigger::None,
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status: SpotOrderStatus::Pending,
            taker_max_fee: fee,
            fee_consumed: Decimal::ZERO,
            cloid: None,
            created_at: 0,
            updated_at: 0,
            version: 0,
        })
    }

    fn active_order(
        id: u64,
        side: OrderSide,
        limit_px: Decimal,
        size: Decimal,
        tif: OrderTif,
    ) -> SpotOrderV3<super::Pending> {
        let mut order = order(side, limit_px, size, Decimal::ZERO);
        order.comon_order.order_id = id;
        order.comon_order.taker_max_fee = Decimal::new(100, 0);
        order.comon_order.size = size;
        order.comon_order.tif = tif;
        order
    }

    fn as_any<S: super::SpotOrderStage>(
        order: &SpotOrderV3<S>,
        status: SpotOrderStatus,
    ) -> SpotOrderV3Any {
        let mut record = order.order_base().clone();
        record.status = status;
        SpotOrderV3Any::from_record(record).unwrap()
    }

    fn match_input() -> MatchSpotOrderV3Input {
        MatchSpotOrderV3Input {
            trade_id_start: 500,
            maker_fee_bps: 100,
            taker_fee_bps: 200,
            timestamp: 123,
        }
    }

    #[test]
    fn matches_at_maker_price_and_updates_both_orders() {
        let taker = active_order(
            1,
            OrderSide::Buy,
            Decimal::new(100, 0),
            Decimal::new(2, 0),
            OrderTif::Gtc,
        );
        let maker = active_order(
            2,
            OrderSide::Sell,
            Decimal::new(99, 0),
            Decimal::new(3, 0),
            OrderTif::Gtc,
        );
        let maker_record = maker.order_base().clone();
        let mut maker_record = maker_record;
        maker_record.status = SpotOrderStatus::Open;
        let maker = SpotOrderV3Any::from_record(maker_record).unwrap();

        let result = taker.match_with_input(vec![maker], match_input()).unwrap();
        let MatchResult::Filled { filled_taker_order_after, maker_orders_after, created_trades } =
            result
        else {
            panic!("expected filled result");
        };

        assert_eq!(filled_taker_order_after.order_base().status, SpotOrderStatus::Filled);
        assert_eq!(filled_taker_order_after.order_base().size, Decimal::ZERO);
        assert_eq!(maker_orders_after[0].order_base().status, SpotOrderStatus::PartiallyFilled);
        assert_eq!(maker_orders_after[0].order_base().size, Decimal::ONE);
        assert_eq!(created_trades[0].comon_trade.trade_id, 500);
        assert_eq!(created_trades[0].comon_trade.price, Decimal::new(99, 0));
        assert_eq!(created_trades[0].comon_trade.size, Decimal::new(2, 0));
        assert_eq!(created_trades[0].comon_trade.taker_fee, Decimal::new(396, 2));
        assert_eq!(created_trades[0].comon_trade.maker_fee, Decimal::new(198, 2));
        assert_eq!(filled_taker_order_after.order_base().fee_consumed, Decimal::new(396, 2));
        assert_eq!(maker_orders_after[0].order_base().fee_consumed, Decimal::new(198, 2));
    }

    #[test]
    fn cancels_ioc_remainder_after_partial_fill() {
        let taker = active_order(
            1,
            OrderSide::Buy,
            Decimal::new(100, 0),
            Decimal::new(3, 0),
            OrderTif::Ioc,
        );
        let maker = active_order(
            2,
            OrderSide::Sell,
            Decimal::new(99, 0),
            Decimal::new(2, 0),
            OrderTif::Gtc,
        );

        let result = taker
            .match_with_input(vec![as_any(&maker, SpotOrderStatus::Open)], match_input())
            .unwrap();
        let MatchResult::CanceledAfterPartialFill {
            canceled_taker_order_after,
            created_trades,
            ..
        } = result
        else {
            panic!("expected canceled partial fill");
        };
        assert_eq!(canceled_taker_order_after.order_base().status, SpotOrderStatus::Canceled);
        assert_eq!(canceled_taker_order_after.order_base().size, Decimal::ONE);
        assert_eq!(created_trades.len(), 1);
    }

    #[test]
    fn fills_multiple_makers_in_input_order() {
        let taker = active_order(
            1,
            OrderSide::Buy,
            Decimal::new(100, 0),
            Decimal::new(3, 0),
            OrderTif::Gtc,
        );
        let first =
            active_order(2, OrderSide::Sell, Decimal::new(99, 0), Decimal::ONE, OrderTif::Gtc);
        let second = active_order(
            3,
            OrderSide::Sell,
            Decimal::new(100, 0),
            Decimal::new(2, 0),
            OrderTif::Gtc,
        );

        let result = taker
            .match_with_input(
                vec![as_any(&first, SpotOrderStatus::Open), as_any(&second, SpotOrderStatus::Open)],
                match_input(),
            )
            .unwrap();
        let MatchResult::Filled { created_trades, .. } = result else {
            panic!("expected filled result");
        };
        assert_eq!(created_trades.len(), 2);
        assert_eq!(created_trades[0].comon_trade.maker_order_id, 2);
        assert_eq!(created_trades[1].comon_trade.maker_order_id, 3);
        assert_eq!(created_trades[1].comon_trade.trade_id, 501);
    }

    #[test]
    fn pending_gtc_without_makers_becomes_open() {
        let mut taker = order(OrderSide::Buy, Decimal::ONE, Decimal::ONE, Decimal::ZERO);
        taker.comon_order.size = Decimal::ONE;
        let result = taker.match_with_input(vec![], match_input()).unwrap();
        let MatchResult::Resting { taker_order_after } = result else {
            panic!("expected resting result");
        };
        assert_eq!(taker_order_after.order_base().status, SpotOrderStatus::Open);
        assert_eq!(taker_order_after.order_base().version, 1);
        assert_eq!(taker_order_after.order_base().updated_at, 123);
    }

    #[test]
    fn rejects_alo_that_crosses_the_best_maker() {
        let taker =
            active_order(1, OrderSide::Buy, Decimal::new(100, 0), Decimal::ONE, OrderTif::Alo);
        let maker =
            active_order(2, OrderSide::Sell, Decimal::new(99, 0), Decimal::ONE, OrderTif::Gtc);
        let result = taker
            .match_with_input(vec![as_any(&maker, SpotOrderStatus::Open)], match_input())
            .unwrap();
        let MatchResult::Rejected { rejected_taker_order_after } = result else {
            panic!("expected rejection");
        };
        assert_eq!(rejected_taker_order_after.order_base().status, SpotOrderStatus::Rejected);
    }

    #[test]
    fn record_loading_dispatches_state_and_rejects_typestate_mismatch() {
        let mut record =
            order(OrderSide::Buy, Decimal::ONE, Decimal::ONE, Decimal::ZERO).order_base().clone();
        record.status = SpotOrderStatus::Open;
        assert!(SpotOrderV3::<super::Pending>::try_from_record(record.clone()).is_err());
        assert!(matches!(SpotOrderV3Any::from_record(record), Ok(SpotOrderV3Any::Open(_))));
    }

    #[test]
    fn factories_only_normalize_status_and_relationships() {
        let mut original =
            order(OrderSide::Buy, Decimal::ONE, Decimal::ONE, Decimal::ZERO).order_base().clone();
        original.status = SpotOrderStatus::Filled;
        original.group_relation = OrderGroupRelation::NormalTpslChild { parent_order_id: 99 };
        original.created_at = 123;
        original.updated_at = 456;
        original.version = 7;
        let loaded = SpotOrderV3Any::from_record(original.clone()).unwrap();
        assert_eq!(loaded.order_base(), &original);

        let single: SpotOrderV3<super::Pending> = SpotOrderV3::single(original.clone());
        let mut expected = original.clone();
        expected.status = SpotOrderStatus::Pending;
        expected.group_relation = OrderGroupRelation::Standalone;
        assert_eq!(single.order_base(), &expected);

        let mut first = original.clone();
        first.order_id = 2;
        let mut second = first.clone();
        second.order_id = 3;
        let (parent, children) =
            SpotOrderV3::normal_tpsl(original.clone(), vec![first.clone(), second.clone()]);
        expected.group_relation = OrderGroupRelation::NormalTpslParent;
        assert_eq!(parent.order_base(), &expected);
        for (child, mut expected) in children.iter().zip([first, second]) {
            expected.status = SpotOrderStatus::Pending;
            expected.group_relation = OrderGroupRelation::NormalTpslChild { parent_order_id: 1 };
            assert_eq!(child.order_base(), &expected);
        }
        let (parent, children) = SpotOrderV3::normal_tpsl(original, vec![]);
        assert_eq!(parent.order_base().status, SpotOrderStatus::Pending);
        assert!(children.is_empty());
    }

    #[test]
    fn lifecycle_types_share_read_only_queries() {
        fn query<S: super::SpotOrderStage>(order: &SpotOrderV3<S>) {
            let _ = order.freeze_requirement();
        }
        let _ = query::<super::Pending>;
        let _ = query::<super::Open>;
        let _ = query::<super::PartiallyFilled>;
        let _ = query::<super::Filled>;
        let _ = query::<super::Canceled>;
        let _ = query::<super::Rejected>;
    }

    #[test]
    fn buy_freezes_original_quote_notional_plus_fee() {
        let order =
            order(OrderSide::Buy, Decimal::new(250, 2), Decimal::new(3, 0), Decimal::new(25, 2));

        assert_eq!(
            order.freeze_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(775, 2)).unwrap())
        );
    }

    #[test]
    fn cancel_advances_open_order_to_canceled() {
        let original = as_any(
            &order(OrderSide::Sell, Decimal::ONE, Decimal::ONE, Decimal::ZERO),
            SpotOrderStatus::Open,
        );
        let SpotOrderV3Any::Open(original) = original else { unreachable!() };
        let canceled = original.cancel(42).unwrap();

        assert_eq!(canceled.order_base().status, SpotOrderStatus::Canceled);
        assert_eq!(canceled.order_base().updated_at, 42);
        assert_eq!(canceled.order_base().version, original.order_base().version + 1);
    }

    #[test]
    fn cancel_rejects_terminal_order() {
        let mut record =
            order(OrderSide::Sell, Decimal::ONE, Decimal::ONE, Decimal::ZERO).order_base().clone();
        record.status = SpotOrderStatus::Filled;
        let filled = SpotOrderV3Any::from_record(record).unwrap();
        assert!(matches!(filled, SpotOrderV3Any::Filled(_)));
    }

    #[test]
    fn cancel_preserves_order_when_version_overflows() {
        let mut record =
            order(OrderSide::Sell, Decimal::ONE, Decimal::ONE, Decimal::ZERO).order_base().clone();
        record.status = SpotOrderStatus::Open;
        record.version = u64::MAX;
        let SpotOrderV3Any::Open(original) = SpotOrderV3Any::from_record(record).unwrap() else {
            unreachable!()
        };

        assert!(original.cancel(42).is_err());
        assert_eq!(original.order_base().status, SpotOrderStatus::Open);
        assert_eq!(original.order_base().version, u64::MAX);
    }

    #[test]
    fn canceled_buy_releases_remaining_notional_and_unconsumed_fee() {
        let mut record =
            order(OrderSide::Buy, Decimal::new(100, 0), Decimal::new(2, 0), Decimal::new(50, 1))
                .order_base()
                .clone();
        record.size = Decimal::ONE;
        record.fee_consumed = Decimal::new(2, 1);
        record.status = SpotOrderStatus::Canceled;
        let SpotOrderV3Any::Canceled(order) = SpotOrderV3Any::from_record(record).unwrap() else {
            unreachable!()
        };

        assert_eq!(
            order.cancel_release_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(103, 0)).unwrap())
        );
    }

    #[test]
    fn canceled_sell_releases_remaining_base_size() {
        let mut record = order(OrderSide::Sell, Decimal::ONE, Decimal::new(3, 0), Decimal::ZERO)
            .order_base()
            .clone();
        record.size = Decimal::new(125, 2);
        record.status = SpotOrderStatus::Canceled;
        let SpotOrderV3Any::Canceled(order) = SpotOrderV3Any::from_record(record).unwrap() else {
            unreachable!()
        };

        assert_eq!(
            order.cancel_release_requirement(),
            Ok(CoinAmount::new(Coin::BTC, Decimal::new(125, 2)).unwrap())
        );
    }

    #[test]
    fn cancel_release_rejects_fee_consumption_above_reservation() {
        let mut record =
            order(OrderSide::Buy, Decimal::ONE, Decimal::ONE, Decimal::ONE).order_base().clone();
        record.status = SpotOrderStatus::Canceled;
        record.fee_consumed = Decimal::new(11, 1);
        let SpotOrderV3Any::Canceled(order) = SpotOrderV3Any::from_record(record).unwrap() else {
            unreachable!()
        };
        assert_eq!(
            order.cancel_release_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );
    }

    #[test]
    fn sell_freezes_original_base_size_independent_of_quote_fee() {
        let order =
            order(OrderSide::Sell, Decimal::new(250, 2), Decimal::new(3, 0), Decimal::new(900, 0));

        assert_eq!(
            order.freeze_requirement(),
            Ok(CoinAmount::new(Coin::BTC, Decimal::new(3, 0)).unwrap())
        );
    }

    #[test]
    fn rejects_non_positive_size_negative_fee_and_non_positive_buy_price() {
        let invalid_size = order(OrderSide::Sell, Decimal::ONE, Decimal::ZERO, Decimal::ZERO);
        assert_eq!(
            invalid_size.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let negative_size =
            order(OrderSide::Buy, Decimal::ONE, Decimal::NEGATIVE_ONE, Decimal::ZERO);
        assert_eq!(
            negative_size.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let negative_fee =
            order(OrderSide::Sell, Decimal::ONE, Decimal::ONE, Decimal::NEGATIVE_ONE);
        assert_eq!(
            negative_fee.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        for price in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            let invalid_price = order(OrderSide::Buy, price, Decimal::ONE, Decimal::ZERO);
            assert_eq!(
                invalid_price.freeze_requirement(),
                Err(SpotAmountConversionError::InvalidAmount)
            );
        }
    }

    #[test]
    fn enforces_coin_precision_for_freeze_amount() {
        let exact = order(OrderSide::Buy, Decimal::new(123, 2), Decimal::ONE, Decimal::ZERO);
        assert_eq!(
            exact.freeze_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(123, 2)).unwrap())
        );

        let excessive = order(OrderSide::Buy, Decimal::new(1, 7), Decimal::ONE, Decimal::ZERO);
        assert_eq!(
            excessive.freeze_requirement(),
            Err(SpotAmountConversionError::InexactConversion)
        );
    }

    #[test]
    fn reports_checked_decimal_multiplication_overflow() {
        let order = order(OrderSide::Buy, Decimal::MAX, Decimal::MAX, Decimal::ZERO);

        assert_eq!(order.freeze_requirement(), Err(SpotAmountConversionError::ArithmeticOverflow));
    }

    #[test]
    fn reports_checked_decimal_addition_overflow() {
        let order = order(OrderSide::Buy, Decimal::MAX, Decimal::ONE, Decimal::ONE);

        assert_eq!(order.freeze_requirement(), Err(SpotAmountConversionError::ArithmeticOverflow));
    }
}
