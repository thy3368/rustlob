use common_entity::fsm_v2::{Fsm, SequencedCommand};
use common_entity::{
    AggregateRole, Entity, EntityError, EntityFieldChange, EntityMutationModel, FourColorArchetype,
    Shard, TrackDiff,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::OrderSide;
use crate::entity::common::common_trade::TradeBase;

/// 一笔现货成交在买卖双方的结算金额。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpotTradeSettlementAmounts {
    pub base_amount: Decimal,
    pub buyer_quote_payment: Decimal,
    pub seller_quote_receipt: Decimal,
}

/// 现货成交结算金额派生错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SpotTradeSettlementError {
    #[error("trade price and size must be greater than zero")]
    InvalidTradeAmount,
    #[error("trade fees must not be negative")]
    InvalidFee,
    #[error("trade amount arithmetic overflow")]
    ArithmeticOverflow,
    #[error("seller fee exceeds quote proceeds")]
    SellerFeeExceedsProceeds,
}

/// 现货成交的结算生命周期状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotTradeSettlementStatus {
    Unsettled,
    Settled,
}

/// 现货成交支持的生命周期命令。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpotTradeV3Command {
    Settle,
}

/// 现货成交状态迁移错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotTradeStateError {
    #[error("replay projection failed: {0}")]
    Replay(#[from] EntityError),
    #[error("spot trade settlement was already completed")]
    AlreadySettled,
    #[error("stale lsn {incoming_lsn}; current lsn is {current_lsn}")]
    StaleLsn { incoming_lsn: u64, current_lsn: u64 },
}

/// 现货成交；结算版本直接采用 command LSN，可从较早版本跳号。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeV3 {
    pub comon_trade: TradeBase,
    pub settlement_status: SpotTradeSettlementStatus,
    pub lsn: u64,
    pub command_index: u32,
}

impl TrackDiff for SpotTradeV3 {
    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        let trade = &self.comon_trade;
        vec![
            EntityFieldChange::new("trade_id", "", trade.trade_id.to_string()),
            EntityFieldChange::new("taker_order_id", "", trade.taker_order_id.to_string()),
            EntityFieldChange::new("maker_order_id", "", trade.maker_order_id.to_string()),
            EntityFieldChange::new("price", "", trade.price.to_string()),
            EntityFieldChange::new("size", "", trade.size.to_string()),
            EntityFieldChange::new("taker_fee", "", trade.taker_fee.to_string()),
            EntityFieldChange::new("maker_fee", "", trade.maker_fee.to_string()),
            EntityFieldChange::new(
                "settlement_status",
                "",
                format!("{:?}", self.settlement_status),
            ),
            EntityFieldChange::new("created_at", "", trade.created_at.to_string()),
            EntityFieldChange::new("updated_at", "", trade.updated_at.to_string()),
        ]
    }

    fn diff(&self, other: &Self) -> Vec<EntityFieldChange> {
        let mut changes = Vec::new();
        if self.settlement_status != other.settlement_status {
            changes.push(EntityFieldChange::new(
                "settlement_status",
                format!("{:?}", self.settlement_status),
                format!("{:?}", other.settlement_status),
            ));
        }
        if self.comon_trade.updated_at != other.comon_trade.updated_at {
            changes.push(EntityFieldChange::new(
                "updated_at",
                self.comon_trade.updated_at.to_string(),
                other.comon_trade.updated_at.to_string(),
            ));
        }
        changes
    }
}

impl Entity for SpotTradeV3 {
    fn entity_id(&self) -> u64 {
        self.comon_trade.trade_id
    }
    fn is_derived() -> bool {
        false
    }
    fn entity_type() -> u8 {
        13
    }
    fn lsn(&self) -> u64 {
        self.lsn
    }
    fn command_index(&self) -> u32 {
        self.command_index
    }
    fn four_color_archetype() -> FourColorArchetype {
        FourColorArchetype::MomentInterval
    }
    fn mutation_model() -> EntityMutationModel {
        EntityMutationModel::AppendOnlyRecord
    }
    fn aggregate_role() -> AggregateRole {
        AggregateRole::AggregateRoot
    }
    fn replay_field_type(name: &str) -> u8 {
        if matches!(name, "created_at" | "updated_at") {
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

impl Shard for SpotTradeV3 {
    fn partition(&self) -> u8 {
        self.comon_trade.symbol.base.index() as u8
    }
}

impl common_entity::PositionedEntity for SpotTradeV3 {
    fn entity_id(&self) -> u64 {
        <Self as Entity>::entity_id(self)
    }
    fn entity_type(&self) -> u8 {
        <Self as Entity>::entity_type()
    }
    fn lsn(&self) -> u64 {
        self.lsn
    }
    fn command_index(&self) -> u32 {
        self.command_index
    }
}

impl SpotTradeV3 {
    /// 派生本笔成交的买方支付、卖方所得和 base 交付金额。
    ///
    /// 费用均以 quote 计价；买方支付本金加买方费用，卖方所得为本金减卖方费用。
    /// 账户余额的实际变更由 settlement use case 编排。
    pub fn settlement_amounts(
        &self,
    ) -> Result<SpotTradeSettlementAmounts, SpotTradeSettlementError> {
        let trade = &self.comon_trade;
        if trade.price <= Decimal::ZERO || trade.size <= Decimal::ZERO {
            return Err(SpotTradeSettlementError::InvalidTradeAmount);
        }
        if trade.taker_fee < Decimal::ZERO || trade.maker_fee < Decimal::ZERO {
            return Err(SpotTradeSettlementError::InvalidFee);
        }

        let quote_notional = trade
            .price
            .checked_mul(trade.size)
            .ok_or(SpotTradeSettlementError::ArithmeticOverflow)?;
        let (buyer_fee, seller_fee) = match trade.taker_side {
            OrderSide::Buy => (trade.taker_fee, trade.maker_fee),
            OrderSide::Sell => (trade.maker_fee, trade.taker_fee),
        };
        if seller_fee > quote_notional {
            return Err(SpotTradeSettlementError::SellerFeeExceedsProceeds);
        }
        let buyer_quote_payment = quote_notional
            .checked_add(buyer_fee)
            .ok_or(SpotTradeSettlementError::ArithmeticOverflow)?;
        let seller_quote_receipt = quote_notional
            .checked_sub(seller_fee)
            .ok_or(SpotTradeSettlementError::SellerFeeExceedsProceeds)?;

        Ok(SpotTradeSettlementAmounts {
            base_amount: trade.size,
            buyer_quote_payment,
            seller_quote_receipt,
        })
    }

    /// Compatibility alias for callers expressing the operation as settlement.
    pub fn settle_amounts(&self) -> Result<SpotTradeSettlementAmounts, SpotTradeSettlementError> {
        self.settlement_amounts()
    }

    /// Retained as a side-effect-free settlement entry point for API compatibility.
    pub fn settle(&self) -> Result<SpotTradeSettlementAmounts, SpotTradeSettlementError> {
        self.settlement_amounts()
    }
}

impl Fsm for SpotTradeV3 {
    type Command = SpotTradeV3Command;
    type Error = SpotTradeStateError;

    fn event_type(_: &Self::Command) -> u8 {
        1
    }

    /// 使用 envelope 中的权威位置与逻辑时间纯计算结算后的成交事实。
    fn derive_transition(
        &self,
        command: SequencedCommand<Self::Command>,
    ) -> Result<Self, Self::Error> {
        let (metadata, command) = command.into_parts();
        match (self.settlement_status, command) {
            (SpotTradeSettlementStatus::Unsettled, SpotTradeV3Command::Settle) => {
                let position = (metadata.lsn, metadata.command_index);
                if position < (self.lsn, self.command_index) {
                    return Err(SpotTradeStateError::StaleLsn {
                        incoming_lsn: position.0,
                        current_lsn: self.lsn,
                    });
                }
                let mut after = self.clone();
                after.settlement_status = SpotTradeSettlementStatus::Settled;
                after.comon_trade.version = position.0;
                after.comon_trade.updated_at = metadata.logical_ts;
                after.lsn = position.0;
                after.command_index = position.1;
                Ok(after)
            }
            (SpotTradeSettlementStatus::Settled, SpotTradeV3Command::Settle) => {
                Err(SpotTradeStateError::AlreadySettled)
            }
        }
    }
}
