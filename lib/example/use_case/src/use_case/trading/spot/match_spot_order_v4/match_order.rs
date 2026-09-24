use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use spot_entity::spot_order_v2::{SpotOrderV2, SpotOrderV2MatchError};

use super::support::{MatchSpotOrderV4Error, validate_all_reservations_for_order, zip_pairs};
use crate::entity::{SpotOrderStatus, SpotOrderTif, spot as spot_entity};
use crate::support::concat2;
use crate::{MatchSpotOrderV2Input, SpotTrade};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchSpotOrderV4Cmd {
    pub party_id: String,
    pub asset: u32,
    pub order_id: u64,
}

/// V4 撮合 use case 的 authoritative after truth。
///
/// 这里故意只表达订单生命周期推进与本次新成交，不表达资金、voucher 或 balance ledger。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV4AfterChanges {
    Resting,
    PartiallyFilled {
        taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        created_trades: Vec<SpotTrade>,
    },
    Filled {
        filled_taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        created_trades: Vec<SpotTrade>,
    },
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV2,
        maker_orders_after: Vec<SpotOrderV2>,
        created_trades: Vec<SpotTrade>,
    },
    Rejected {
        rejected_taker_order_after: SpotOrderV2,
    },
}

/// V4 撮合 use case 的 replayable changes。
///
/// 订单更新以 before/after pair 为唯一真相；`created_trades` 是本次撮合新产生的成交事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV4Changes {
    Resting,
    PartiallyFilled {
        updated_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        created_trades: Vec<SpotTrade>,
    },
    Filled {
        filled_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        created_trades: Vec<SpotTrade>,
    },
    CanceledAfterPartialFill {
        canceled_taker_order: UpdatedEntityPair<SpotOrderV2>,
        updated_maker_orders: Vec<UpdatedEntityPair<SpotOrderV2>>,
        created_trades: Vec<SpotTrade>,
    },
    Rejected {
        rejected_taker_order: UpdatedEntityPair<SpotOrderV2>,
    },
}

impl ReplayableChanges for MatchSpotOrderV4Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        match self {
            Self::Resting => Ok(Vec::with_capacity(0)),
            Self::PartiallyFilled { updated_taker_order, updated_maker_orders, created_trades }
            | Self::Filled {
                filled_taker_order: updated_taker_order,
                updated_maker_orders,
                created_trades,
            }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order,
                updated_maker_orders,
                created_trades,
            } => replay_events(updated_taker_order, updated_maker_orders, created_trades),
            Self::Rejected { rejected_taker_order } => Ok(vec![
                rejected_taker_order.after.track_update_event_from(&rejected_taker_order.before)?,
            ]),
        }
    }
}

fn replay_events(
    updated_taker_order: &UpdatedEntityPair<SpotOrderV2>,
    updated_maker_orders: &[UpdatedEntityPair<SpotOrderV2>],
    created_trades: &[SpotTrade],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut events = Vec::with_capacity(
        1_usize.saturating_add(created_trades.len()).saturating_add(updated_maker_orders.len()),
    );
    events.push(updated_taker_order.after.track_update_event_from(&updated_taker_order.before)?);
    for trade in created_trades {
        events.push(trade.track_create_event()?);
    }
    for maker in updated_maker_orders {
        events.push(maker.after.track_update_event_from(&maker.before)?);
    }
    Ok(events)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderV4State {
    pub taker_order: SpotOrderV2,
    pub maker_orders: Vec<SpotOrderV2>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderV4UseCase;

impl StateMachineV2Unchecked for MatchSpotOrderV4UseCase {
    type Command = MatchSpotOrderV4Cmd;
    type StateGiven = MatchSpotOrderV4State;
    type Error = MatchSpotOrderV4Error;
    type StateChanged = MatchSpotOrderV4AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchSpotOrderV4Error::InvalidPartyId);
        }
        if cmd.order_id == 0 {
            return Err(MatchSpotOrderV4Error::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.taker_order.order_id() != cmd.order_id {
            return Err(MatchSpotOrderV4Error::TakerOrderIdMismatch);
        }
        if state.taker_order.account_id() != cmd.party_id {
            return Err(MatchSpotOrderV4Error::TakerAccountIdMismatch);
        }
        if state.taker_order.asset() != cmd.asset {
            return Err(MatchSpotOrderV4Error::TakerAssetMismatch);
        }
        if !state.taker_order.can_enter_matching() {
            return Err(MatchSpotOrderV4Error::OrderMatch(
                SpotOrderV2MatchError::OrderNotMatchable,
            ));
        }
        validate_all_reservations_for_order(
            &state.taker_order,
            &state.base_asset_id,
            &state.quote_asset_id,
        )?;
        for maker in &state.maker_orders {
            maker.ensure_matchable()?;
            validate_all_reservations_for_order(
                maker,
                &state.base_asset_id,
                &state.quote_asset_id,
            )?;
        }
        state.taker_order.ensure_matchable()?;
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        _cmd: &Self::Command,
        state: &Self::StateGiven,
        context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        compute_match_after(state, context)
    }
}

impl StateMachineOwnedV2Diff for MatchSpotOrderV4UseCase {
    type StateDiff = MatchSpotOrderV4Changes;

    fn do_compute_state_diff(
        state: MatchSpotOrderV4State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        match after {
            MatchSpotOrderV4AfterChanges::Resting => Ok(MatchSpotOrderV4Changes::Resting),
            MatchSpotOrderV4AfterChanges::Rejected { rejected_taker_order_after } => {
                Ok(MatchSpotOrderV4Changes::Rejected {
                    rejected_taker_order: UpdatedEntityPair {
                        before: state.taker_order,
                        after: rejected_taker_order_after,
                    },
                })
            }
            MatchSpotOrderV4AfterChanges::PartiallyFilled {
                taker_order_after,
                maker_orders_after,
                created_trades,
            } => {
                if created_trades.is_empty() {
                    return Err(MatchSpotOrderV4Error::OrderMatch(
                        SpotOrderV2MatchError::NoTradesMatched,
                    ));
                }
                let (updated_taker_order, updated_maker_orders) =
                    build_matched_pairs(state, taker_order_after, maker_orders_after)?;
                Ok(MatchSpotOrderV4Changes::PartiallyFilled {
                    updated_taker_order,
                    updated_maker_orders,
                    created_trades,
                })
            }
            MatchSpotOrderV4AfterChanges::Filled {
                filled_taker_order_after,
                maker_orders_after,
                created_trades,
            } => {
                if created_trades.is_empty() {
                    return Err(MatchSpotOrderV4Error::OrderMatch(
                        SpotOrderV2MatchError::NoTradesMatched,
                    ));
                }
                let (filled_taker_order, updated_maker_orders) =
                    build_matched_pairs(state, filled_taker_order_after, maker_orders_after)?;
                Ok(MatchSpotOrderV4Changes::Filled {
                    filled_taker_order,
                    updated_maker_orders,
                    created_trades,
                })
            }
            MatchSpotOrderV4AfterChanges::CanceledAfterPartialFill {
                canceled_taker_order_after,
                maker_orders_after,
                created_trades,
            } => {
                if created_trades.is_empty() {
                    return Err(MatchSpotOrderV4Error::OrderMatch(
                        SpotOrderV2MatchError::NoTradesMatched,
                    ));
                }
                let (canceled_taker_order, updated_maker_orders) =
                    build_matched_pairs(state, canceled_taker_order_after, maker_orders_after)?;
                Ok(MatchSpotOrderV4Changes::CanceledAfterPartialFill {
                    canceled_taker_order,
                    updated_maker_orders,
                    created_trades,
                })
            }
        }
    }
}

fn compute_match_after(
    state: &MatchSpotOrderV4State,
    context: &ExecutionContext,
) -> Result<MatchSpotOrderV4AfterChanges, MatchSpotOrderV4Error> {
    let mut taker_after = state.taker_order.clone();
    let mut maker_orders_after = state.maker_orders.clone();
    let executed_at_ms = context.execution_time_ns / 1_000_000;
    let timestamp = context.execution_time_ns;

    match taker_after.time_in_force() {
        SpotOrderTif::Gtc | SpotOrderTif::Ioc => {
            let taker_before_match = taker_after.clone();
            let match_outcome = taker_after.match_with_makers(
                &mut maker_orders_after,
                MatchSpotOrderV2Input {
                    match_id: concat2("spot-match:", taker_after.order_id().to_string().as_str()),
                    maker_fee_bps: state.maker_fee_bps,
                    taker_fee_bps: state.taker_fee_bps,
                    executed_at_ms,
                    timestamp,
                },
            )?;
            if matches!(taker_after.time_in_force(), SpotOrderTif::Gtc)
                && match_outcome.trades.is_empty()
            {
                return Ok(MatchSpotOrderV4AfterChanges::Resting);
            }

            finish_match_after(
                taker_before_match,
                taker_after,
                maker_orders_after,
                match_outcome.trades,
                timestamp,
            )
        }
        SpotOrderTif::Alo => {
            let Some(best_maker) = maker_orders_after.first() else {
                return Ok(MatchSpotOrderV4AfterChanges::Resting);
            };
            if !taker_after.crosses_maker(best_maker)? {
                return Ok(MatchSpotOrderV4AfterChanges::Resting);
            }

            taker_after.reject_as_bad_alo(timestamp)?;
            Ok(MatchSpotOrderV4AfterChanges::Rejected { rejected_taker_order_after: taker_after })
        }
    }
}

fn finish_match_after(
    taker_before_match: SpotOrderV2,
    mut taker_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
    created_trades: Vec<SpotTrade>,
    timestamp: u64,
) -> Result<MatchSpotOrderV4AfterChanges, MatchSpotOrderV4Error> {
    let total_taker_fill = created_trades.iter().try_fold(0_u64, |acc, trade| {
        acc.checked_add(trade.qty).ok_or(MatchSpotOrderV4Error::ArithmeticOverflow)
    })?;
    let taker_reservation_after_match = taker_after.reservation.clone();
    let taker_fee_reservation_after_match = taker_after.fee_reservation.clone();
    taker_after = taker_before_match;
    taker_after.reservation = taker_reservation_after_match;
    taker_after.fee_reservation = taker_fee_reservation_after_match;
    taker_after.finish_after_match(total_taker_fill, timestamp)?;

    match taker_after.status() {
        SpotOrderStatus::PartiallyFilled => Ok(MatchSpotOrderV4AfterChanges::PartiallyFilled {
            taker_order_after: taker_after,
            maker_orders_after,
            created_trades,
        }),
        SpotOrderStatus::Filled => Ok(MatchSpotOrderV4AfterChanges::Filled {
            filled_taker_order_after: taker_after,
            maker_orders_after,
            created_trades,
        }),
        SpotOrderStatus::Canceled => Ok(MatchSpotOrderV4AfterChanges::CanceledAfterPartialFill {
            canceled_taker_order_after: taker_after,
            maker_orders_after,
            created_trades,
        }),
        SpotOrderStatus::Rejected => {
            Ok(MatchSpotOrderV4AfterChanges::Rejected { rejected_taker_order_after: taker_after })
        }
        SpotOrderStatus::Open | SpotOrderStatus::Pending => {
            Err(MatchSpotOrderV4Error::OrderMatch(SpotOrderV2MatchError::NoTradesMatched))
        }
    }
}

fn build_matched_pairs(
    state: MatchSpotOrderV4State,
    taker_order_after: SpotOrderV2,
    maker_orders_after: Vec<SpotOrderV2>,
) -> Result<
    (UpdatedEntityPair<SpotOrderV2>, Vec<UpdatedEntityPair<SpotOrderV2>>),
    MatchSpotOrderV4Error,
> {
    let updated_taker_order =
        UpdatedEntityPair { before: state.taker_order, after: taker_order_after };
    let updated_maker_orders = zip_pairs(state.maker_orders, maker_orders_after)?;
    Ok((updated_taker_order, updated_maker_orders))
}

impl MatchSpotOrderV4Changes {
    pub fn taker_order_after(&self) -> Option<&SpotOrderV2> {
        match self {
            Self::Resting => None,
            Self::PartiallyFilled { updated_taker_order, .. }
            | Self::Filled { filled_taker_order: updated_taker_order, .. }
            | Self::CanceledAfterPartialFill {
                canceled_taker_order: updated_taker_order, ..
            }
            | Self::Rejected { rejected_taker_order: updated_taker_order } => {
                Some(&updated_taker_order.after)
            }
        }
    }

    pub fn created_trades(&self) -> &[SpotTrade] {
        match self {
            Self::Resting | Self::Rejected { .. } => &[],
            Self::PartiallyFilled { created_trades, .. }
            | Self::Filled { created_trades, .. }
            | Self::CanceledAfterPartialFill { created_trades, .. } => created_trades,
        }
    }
}
