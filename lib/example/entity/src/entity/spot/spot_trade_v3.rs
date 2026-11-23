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
    pub position: common_entity::EventPosition,
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
        self.position.lsn
    }
    fn event_position(&self) -> common_entity::EventPosition {
        self.position
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
    fn event_position(&self) -> common_entity::EventPosition {
        self.position
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
                let position = metadata.position;
                if position < self.position {
                    return Err(SpotTradeStateError::StaleLsn {
                        incoming_lsn: position.lsn,
                        current_lsn: self.position.lsn,
                    });
                }
                let mut after = self.clone();
                after.settlement_status = SpotTradeSettlementStatus::Settled;
                after.comon_trade.version = position.lsn;
                after.comon_trade.updated_at = metadata.logical_ts;
                after.position = position;
                Ok(after)
            }
            (SpotTradeSettlementStatus::Settled, SpotTradeV3Command::Settle) => {
                Err(SpotTradeStateError::AlreadySettled)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common_entity::fsm_v2::{CommandMetadata, Fsm, SequencedCommand};
    use common_entity::{Entity, EventPosition, Shard};
    use rust_decimal::Decimal;

    use super::{
        SpotTradeSettlementError, SpotTradeSettlementStatus, SpotTradeStateError, SpotTradeV3,
        SpotTradeV3Command,
    };
    use crate::OrderSide;
    use crate::account::coin::Coin;
    use crate::account::symbol::Symbol;
    use crate::entity::common::UserId;
    use crate::entity::common::common_trade::TradeBase;

    fn command_at(
        trade: &SpotTradeV3,
        position: EventPosition,
        logical_ts: u64,
    ) -> SequencedCommand<SpotTradeV3Command> {
        SequencedCommand::new(
            CommandMetadata::new_at(
                position,
                logical_ts,
                trade.topic(),
                trade.partition(),
                trade.party_id(),
            ),
            SpotTradeV3Command::Settle,
        )
    }

    fn trade(taker_side: OrderSide, taker_fee: i64, maker_fee: i64) -> SpotTradeV3 {
        SpotTradeV3 {
            comon_trade: TradeBase {
                trade_id: 7,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                taker_user: UserId(10),
                maker_user: UserId(20),
                taker_order_id: 1,
                maker_order_id: 2,
                taker_side,
                price: Decimal::from(2),
                size: Decimal::from(3),
                taker_fee: Decimal::from(taker_fee),
                maker_fee: Decimal::from(maker_fee),
                created_at: 0,
                updated_at: 0,
                version: 0,
            },
            settlement_status: SpotTradeSettlementStatus::Unsettled,
            position: common_entity::EventPosition::ZERO,
        }
    }

    #[test]
    fn settlement_amounts_map_fees_for_taker_buy_and_sell() {
        let buy = trade(OrderSide::Buy, 1, 2).settlement_amounts().unwrap();
        assert_eq!(buy.base_amount, Decimal::from(3));
        assert_eq!(buy.buyer_quote_payment, Decimal::from(7));
        assert_eq!(buy.seller_quote_receipt, Decimal::from(4));

        let sell = trade(OrderSide::Sell, 1, 2).settlement_amounts().unwrap();
        assert_eq!(sell.buyer_quote_payment, Decimal::from(8));
        assert_eq!(sell.seller_quote_receipt, Decimal::from(5));
    }

    #[test]
    fn seller_fee_equal_to_or_greater_than_proceeds_is_handled() {
        assert_eq!(
            trade(OrderSide::Buy, 1, 6)
                .settlement_amounts()
                .unwrap()
                .seller_quote_receipt,
            Decimal::ZERO
        );
        assert_eq!(
            trade(OrderSide::Buy, 1, 7).settlement_amounts(),
            Err(SpotTradeSettlementError::SellerFeeExceedsProceeds)
        );
    }

    #[test]
    fn settle_returns_after_truth_and_replay_event_without_mutating_before() {
        let before = trade(OrderSide::Buy, 1, 2);
        let position = EventPosition::new(20, 5);
        let (after, event) = before.transition(command_at(&before, position, 7)).unwrap();

        assert_eq!(
            before.settlement_status,
            SpotTradeSettlementStatus::Unsettled
        );
        assert_eq!(before.position, EventPosition::ZERO);
        assert_eq!(after.settlement_status, SpotTradeSettlementStatus::Settled);
        assert_eq!(after.comon_trade.version, 20);
        assert_eq!(after.comon_trade.updated_at, 7);
        assert_eq!(after.position, position);
        assert_eq!(event.entity_id, Entity::entity_id(&before));
        assert_eq!(event.entity_type, <SpotTradeV3 as Entity>::entity_type());
        assert_eq!(event.previous_position, EventPosition::ZERO);
        assert_eq!(event.position, position);
        assert_eq!(event.domain_event_type, 1);
        assert_eq!(event.logical_ts, 7);
        assert_eq!(event.t, before.topic());
        assert_eq!(event.p, before.partition());
    }

    #[test]
    fn stale_lsn_is_rejected_atomically_and_max_is_accepted() {
        let mut stale = trade(OrderSide::Buy, 1, 2);
        stale.comon_trade.version = 20;
        stale.position = EventPosition::new(20, 1);
        let before = stale.clone();
        assert_eq!(
            stale.transition(command_at(&stale, EventPosition::new(20, 0), 7)),
            Err(SpotTradeStateError::StaleLsn {
                incoming_lsn: 20,
                current_lsn: 20
            })
        );
        assert_eq!(stale, before);

        let same_position = trade(OrderSide::Buy, 1, 2);
        let (after, _) = same_position
            .transition(command_at(&same_position, EventPosition::ZERO, 6))
            .unwrap();
        assert_eq!(after.position, EventPosition::ZERO);
        assert_eq!(after.settlement_status, SpotTradeSettlementStatus::Settled);

        let max = trade(OrderSide::Buy, 1, 2);
        let (after, _) = max
            .transition(command_at(
                &max,
                EventPosition::new(u64::MAX, u32::MAX),
                7,
            ))
            .unwrap();
        assert_eq!(after.comon_trade.version, u64::MAX);
    }

    #[test]
    fn duplicate_settlement_is_rejected_without_mutating_before() {
        let mut settled = trade(OrderSide::Buy, 1, 2);
        settled.settlement_status = SpotTradeSettlementStatus::Settled;
        let before = settled.clone();

        assert_eq!(
            settled.transition(command_at(&settled, EventPosition::new(21, 0), 8)),
            Err(SpotTradeStateError::AlreadySettled)
        );
        assert_eq!(settled, before);
    }
}
