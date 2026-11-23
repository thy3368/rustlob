use std::collections::HashSet;

use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use common_entity::{EntityError, Fsm, Shard};
use example_core_entity::entity::common::common_trade::TradeBase;
use example_core_entity::spot::spot_order_v7::{
    ActivateSpotOrderV7Output, ApplySpotOrderV7MatchOutput, CancelSpotOrderV7Output,
    ModifySpotOrderV7Output, SpotOrderV7Output,
};
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3};
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, OrderSide, OrderTif, SpotOrderStatus, SpotOrderV7,
    SpotOrderV7Command, SpotOrderV7HoldAdjustment,
};
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ActivateSpotOrderV7Error {
    #[error("order id must be greater than zero")]
    InvalidOrderId,
    #[error("command order id does not match loaded order")]
    OrderMismatch,
    #[error("only pending orders can be activated")]
    OrderNotPending,
    #[error("balance owner does not match order owner")]
    OwnerMismatch,
    #[error("balance coin does not match freeze requirement")]
    CoinMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV7Error {
    #[error("balance asset does not match the order release asset")]
    AssetMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModifySpotOrderV7Error {
    #[error("balance asset does not match the order held asset")]
    AssetMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ReconcileSpotOrderHoldError {
    #[error("balance owner does not match order owner")]
    OwnerMismatch,
    #[error("balance coin does not match order hold asset")]
    CoinMismatch,
    #[error("arithmetic overflow while reconciling order hold")]
    ArithmeticOverflow,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

pub struct SpotOrderDomainService;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderV3Input {
    pub lsn: u64,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV3Outcome {
    Resting {
        taker_order_after: SpotOrderV7,
    },
    PartiallyFilled {
        taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
    },
    Filled {
        filled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
        release_requirement: example_core_entity::account::coin::CoinAmount,
    },
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
        release_requirement: example_core_entity::account::coin::CoinAmount,
    },
    Rejected {
        rejected_taker_order_after: SpotOrderV7,
        release_requirement: example_core_entity::account::coin::CoinAmount,
    },
}

impl SpotOrderDomainService {
    /// 将 taker 持仓冻结额对账到 command 结束状态，并直接采用 command 的 LSN 与逻辑时间。
    pub fn reconcile_taker_hold(
        order_after: &SpotOrderV7,
        mut settled_balance: BalanceV2,
        frozen_before_command: Decimal,
        lsn: u64,
        timestamp: u64,
    ) -> Result<BalanceV2, ReconcileSpotOrderHoldError> {
        let required = order_after
            .hold_requirement()
            .map_err(|error| EntityError::Custom(error.to_string()))?;
        if settled_balance.party_id() != order_after.order_base().owner.0 {
            return Err(ReconcileSpotOrderHoldError::OwnerMismatch);
        }
        if settled_balance.coin_idx() != u64::from(required.currency().index()) {
            return Err(ReconcileSpotOrderHoldError::CoinMismatch);
        }
        let target = frozen_before_command
            .checked_add(required.amount())
            .ok_or(ReconcileSpotOrderHoldError::ArithmeticOverflow)?;
        let current = settled_balance.frozen();
        if current > target {
            settled_balance.apply(BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: current - target,
                payment_amount: Decimal::ZERO,
                timestamp,
                lsn,
            })?;
        } else if current < target {
            settled_balance.apply(BalanceV2Event::Freeze {
                amount: target - current,
                timestamp,
                lsn,
            })?;
        }
        Ok(settled_balance)
    }

    /// 只读规划全部成交，验证成功后分别推进 taker 与 maker 聚合。
    pub fn match_order(
        mut taker: SpotOrderV7,
        mut makers: Vec<SpotOrderV7>,
        input: MatchSpotOrderV3Input,
    ) -> Result<MatchSpotOrderV3Outcome, EntityError> {
        let t = taker.order_base().clone();
        let mut ids = HashSet::with_capacity(makers.len() + 1);
        ids.insert(t.order_id);
        for maker in &makers {
            let m = maker.order_base();
            if !ids.insert(m.order_id) {
                return Err(match_error("order ids must be unique"));
            }
            if m.symbol != t.symbol || m.side == t.side {
                return Err(match_error("maker market or side mismatch"));
            }
        }

        let crosses = |maker: &SpotOrderV7| match t.side {
            OrderSide::Buy => t.limit_px >= maker.order_base().limit_px,
            OrderSide::Sell => t.limit_px <= maker.order_base().limit_px,
        };
        if t.effective_tif() == OrderTif::Alo && makers.iter().any(crosses) {
            let metadata = command_metadata(input.lsn, input.timestamp, &taker);
            let output = FsmV2::transition(
                &mut taker,
                SequencedCommand::new(
                    metadata,
                    SpotOrderV7Command::RejectedAsTaker,
                ),
            )?
            .output;
            let SpotOrderV7Output::RejectedAsTaker(ApplySpotOrderV7MatchOutput {
                order_after,
                release_requirement: Some(release_requirement),
            }) = output
            else {
                return Err(match_error(
                    "rejected taker event produced mismatched output",
                ));
            };
            return Ok(MatchSpotOrderV3Outcome::Rejected {
                rejected_taker_order_after: order_after,
                release_requirement: release_requirement.amount(),
            });
        }

        #[derive(Clone, Copy)]
        struct PlannedFill {
            maker_index: usize,
            size: Decimal,
            taker_fee: Decimal,
            maker_fee: Decimal,
            trade_id: u64,
        }

        let mut remaining = t.size;
        let mut taker_fee_total = Decimal::ZERO;
        let mut fills = Vec::new();
        for (maker_index, maker) in makers.iter().enumerate() {
            if remaining == Decimal::ZERO {
                break;
            }
            if !crosses(maker) {
                continue;
            }
            let size = remaining.min(maker.order_base().size);
            let notional = maker
                .order_base()
                .limit_px
                .checked_mul(size)
                .ok_or_else(|| match_error("notional overflow"))?;
            let taker_fee = match_fee(notional, input.taker_fee_bps, t.symbol.quote.scale())?;
            let maker_fee = match_fee(notional, input.maker_fee_bps, t.symbol.quote.scale())?;
            taker_fee_total = taker_fee_total
                .checked_add(taker_fee)
                .ok_or_else(|| match_error("fee overflow"))?;
            let trade_id = input
                .trade_id_start
                .checked_add(fills.len() as u64)
                .ok_or_else(|| match_error("trade id overflow"))?;
            remaining = remaining
                .checked_sub(size)
                .ok_or_else(|| match_error("size underflow"))?;
            fills.push(PlannedFill {
                maker_index,
                size,
                taker_fee,
                maker_fee,
                trade_id,
            });
        }

        let filled_size = t
            .size
            .checked_sub(remaining)
            .ok_or_else(|| match_error("size underflow"))?;
        if filled_size == Decimal::ZERO && t.effective_tif() != OrderTif::Ioc {
            return Ok(MatchSpotOrderV3Outcome::Resting {
                taker_order_after: taker,
            });
        }

        let mut trades = Vec::with_capacity(fills.len());
        for fill in fills {
            let maker = makers
                .get_mut(fill.maker_index)
                .ok_or_else(|| match_error("planned maker index is missing"))?;
            let maker_before = maker.order_base().clone();
            let metadata = command_metadata(input.lsn, input.timestamp, maker);
            let _ = FsmV2::transition(
                maker,
                SequencedCommand::new(
                    metadata,
                    SpotOrderV7Command::MatchedAsMaker {
                        filled_size: fill.size,
                        quote_fee: fill.maker_fee,
                    },
                ),
            )?;
            trades.push(SpotTradeV3 {
                comon_trade: TradeBase {
                    trade_id: fill.trade_id,
                    symbol: t.symbol,
                    taker_user: t.owner,
                    maker_user: maker_before.owner,
                    taker_order_id: t.order_id,
                    maker_order_id: maker_before.order_id,
                    taker_side: t.side,
                    price: maker_before.limit_px,
                    size: fill.size,
                    taker_fee: fill.taker_fee,
                    maker_fee: fill.maker_fee,
                    created_at: input.timestamp,
                    updated_at: input.timestamp,
                    version: 0,
                },
                settlement_status: SpotTradeSettlementStatus::Unsettled,
            });
        }
        let metadata = command_metadata(input.lsn, input.timestamp, &taker);
        let output = FsmV2::transition(
            &mut taker,
            SequencedCommand::new(
                metadata,
                SpotOrderV7Command::MatchedAsTaker {
                    filled_size,
                    quote_fee: taker_fee_total,
                },
            ),
        )?
        .output;
        let SpotOrderV7Output::MatchedAsTaker(ApplySpotOrderV7MatchOutput {
            order_after: taker_order_after,
            release_requirement,
        }) = output
        else {
            return Err(match_error(
                "matched taker event produced mismatched output",
            ));
        };

        match (taker_order_after.status(), release_requirement) {
            (SpotOrderStatus::Filled, Some(release_requirement)) => {
                Ok(MatchSpotOrderV3Outcome::Filled {
                    filled_taker_order_after: taker_order_after,
                    maker_orders_after: makers,
                    created_trades: trades,
                    release_requirement: release_requirement.amount(),
                })
            }
            (SpotOrderStatus::Canceled, Some(release_requirement)) => {
                Ok(MatchSpotOrderV3Outcome::CanceledAfterPartialFill {
                    canceled_taker_order_after: taker_order_after,
                    maker_orders_after: makers,
                    created_trades: trades,
                    release_requirement: release_requirement.amount(),
                })
            }
            (SpotOrderStatus::Rejected, Some(release_requirement)) => {
                Ok(MatchSpotOrderV3Outcome::Rejected {
                    rejected_taker_order_after: taker_order_after,
                    release_requirement: release_requirement.amount(),
                })
            }
            (SpotOrderStatus::PartiallyFilled, None) => {
                Ok(MatchSpotOrderV3Outcome::PartiallyFilled {
                    taker_order_after,
                    maker_orders_after: makers,
                    created_trades: trades,
                })
            }
            _ => Err(match_error("unexpected taker state after matching")),
        }
    }

    pub fn modify(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        price: Decimal,
        size: Decimal,
        tif: Option<OrderTif>,
        cloid: Option<String>,
        lsn: u64,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2), ModifySpotOrderV7Error> {
        let metadata = command_metadata(lsn, timestamp, &order);
        let output = FsmV2::transition(
            &mut order,
            SequencedCommand::new(
                metadata,
                SpotOrderV7Command::Modify {
                    price,
                    size,
                    tif,
                    cloid,
                },
            ),
        )?
        .output;
        let SpotOrderV7Output::Modified(ModifySpotOrderV7Output {
            order_after,
            hold_adjustment,
        }) = output
        else {
            return Err(
                EntityError::Custom("modify event produced mismatched output".into()).into(),
            );
        };

        if balance.coin_idx() != u64::from(hold_adjustment.currency().index()) {
            return Err(ModifySpotOrderV7Error::AssetMismatch);
        }
        match hold_adjustment {
            SpotOrderV7HoldAdjustment::Increase { amount } => {
                balance.apply(BalanceV2Event::Freeze {
                    amount: amount.amount(),
                    timestamp,
                    lsn,
                })?;
            }
            SpotOrderV7HoldAdjustment::Decrease { amount } => {
                balance.apply(BalanceV2Event::UnfreezeAndPay {
                    unfreeze_amount: amount.amount(),
                    payment_amount: Decimal::ZERO,
                    timestamp,
                    lsn,
                })?;
            }
            SpotOrderV7HoldAdjustment::Unchanged { .. } => {}
        }

        Ok((order_after, balance))
    }

    pub fn freeze(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        lsn: u64,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2), ActivateSpotOrderV7Error> {
        let metadata = command_metadata(lsn, timestamp, &order);
        let output = FsmV2::transition(
            &mut order,
            SequencedCommand::new(metadata, SpotOrderV7Command::Activate),
        )?
        .output;
        let SpotOrderV7Output::Activated(ActivateSpotOrderV7Output { freeze_requirement }) = output
        else {
            return Err(
                EntityError::Custom("activate event produced mismatched output".into()).into(),
            );
        };

        if balance.coin_idx() != u64::from(freeze_requirement.currency().index()) {
            return Err(ActivateSpotOrderV7Error::CoinMismatch);
        }
        balance.apply(BalanceV2Event::Freeze {
            amount: freeze_requirement.amount(),
            timestamp,
            lsn,
        })?;

        Ok((order, balance))
    }

    pub fn cancel(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        lsn: u64,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2), CancelSpotOrderV7Error> {
        let metadata = command_metadata(lsn, timestamp, &order);
        let output = FsmV2::transition(
            &mut order,
            SequencedCommand::new(metadata, SpotOrderV7Command::Cancel),
        )?
        .output;
        let SpotOrderV7Output::Canceled(CancelSpotOrderV7Output {
            release_requirement,
            ..
        }) = output
        else {
            return Err(
                EntityError::Custom("cancel event produced mismatched output".into()).into(),
            );
        };

        if balance.coin_idx() != u64::from(release_requirement.currency().index()) {
            return Err(CancelSpotOrderV7Error::AssetMismatch);
        }
        if release_requirement.amount() > Decimal::ZERO {
            balance.apply(BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: release_requirement.amount(),
                payment_amount: Decimal::ZERO,
                timestamp,
                lsn,
            })?;
        }

        Ok((order, balance))
    }
}

fn command_metadata(lsn: u64, logical_ts: u64, order: &SpotOrderV7) -> CommandMetadata {
    CommandMetadata::new(
        lsn,
        logical_ts,
        order.topic(),
        order.partition(),
        order.party_id(),
    )
}

fn match_fee(notional: Decimal, bps: u64, scale: u32) -> Result<Decimal, EntityError> {
    notional
        .checked_mul(Decimal::from(bps))
        .and_then(|value| value.checked_div(Decimal::from(10_000u64)))
        .map(|value| {
            value.round_dp_with_strategy(scale, rust_decimal::RoundingStrategy::ToPositiveInfinity)
        })
        .ok_or_else(|| match_error("fee overflow"))
}

fn match_error(message: &str) -> EntityError {
    EntityError::Custom(message.to_owned())
}
