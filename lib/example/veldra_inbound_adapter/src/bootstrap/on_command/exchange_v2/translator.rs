use common_entity::ExecutionErrorV3;
use example_core_use_case::pipe::match_spot_order::MatchSpotOrderPipeChanges;
use example_core_use_case::pipe::place_spot_order_pipeline_v2::PlaceSpotOrderPipelineV2Changes;
use rust_decimal::Decimal;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCommandExecutionV3, SpotBlockExecutionV3,
};

use super::{
    CancelStatusWire, ExchangeResponseWire, FilledOrderStatusWire, OrderStatusWire,
    RestingOrderStatusWire,
};

/// core 执行结果无法无损投影为 exchange wire 时的错误。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SpotBlockTranslationError {
    #[error("filled order {oid} has no taker trade facts")]
    MissingTakerTrade { oid: u64 },
    #[error("filled order {oid} has zero total trade size")]
    ZeroTotalTradeSize { oid: u64 },
    #[error("filled order {oid} trade aggregation overflowed")]
    ArithmeticOverflow { oid: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponseGroup {
    Order,
    Cancel,
}

/// 将 block 执行事实投影为 Hyperliquid 兼容的 order/cancel responses。
pub fn translate_spot_block_execution(
    execution: &SpotBlockExecutionV3,
) -> Result<Vec<ExchangeResponseWire>, SpotBlockTranslationError> {
    let mut group_order = Vec::with_capacity(2);
    let mut order_statuses = Vec::new();
    let mut cancel_statuses = Vec::new();

    for item in &execution.command_results {
        match item {
            SpotBlockCommandExecutionV3::Place { result, .. } => {
                if order_statuses.is_empty() {
                    group_order.push(ResponseGroup::Order);
                }
                order_statuses.push(match result {
                    Ok(result) => order_status_from_changes(&result.state_changed)?,
                    Err(error) => OrderStatusWire::Error {
                        error: execution_error_message(error),
                    },
                });
            }
            SpotBlockCommandExecutionV3::Cancel { result, .. } => {
                if cancel_statuses.is_empty() {
                    group_order.push(ResponseGroup::Cancel);
                }
                cancel_statuses.push(match result {
                    Ok(_) => CancelStatusWire::Success("success"),
                    Err(error) => CancelStatusWire::Error {
                        error: execution_error_message(error),
                    },
                });
            }
        }
    }

    Ok(grouped_responses(
        group_order,
        order_statuses,
        cancel_statuses,
    ))
}

fn grouped_responses(
    group_order: Vec<ResponseGroup>,
    mut order_statuses: Vec<OrderStatusWire>,
    mut cancel_statuses: Vec<CancelStatusWire>,
) -> Vec<ExchangeResponseWire> {
    group_order
        .into_iter()
        .map(|group| match group {
            ResponseGroup::Order => {
                ExchangeResponseWire::order(std::mem::take(&mut order_statuses))
            }
            ResponseGroup::Cancel => {
                ExchangeResponseWire::cancel(std::mem::take(&mut cancel_statuses))
            }
        })
        .collect()
}

fn order_status_from_changes(
    changes: &PlaceSpotOrderPipelineV2Changes,
) -> Result<OrderStatusWire, SpotBlockTranslationError> {
    match changes {
        PlaceSpotOrderPipelineV2Changes::Resting { created_order, .. } => {
            Ok(OrderStatusWire::Resting {
                resting: RestingOrderStatusWire {
                    oid: created_order.after().order_base().order_id,
                },
            })
        }
        PlaceSpotOrderPipelineV2Changes::Rejected { .. } => Ok(OrderStatusWire::Error {
            error: "order rejected by matching policy".to_owned(),
        }),
        PlaceSpotOrderPipelineV2Changes::PartiallyFilled {
            created_order,
            matched,
            ..
        }
        | PlaceSpotOrderPipelineV2Changes::Filled {
            created_order,
            matched,
            ..
        }
        | PlaceSpotOrderPipelineV2Changes::CanceledAfterPartialFill {
            created_order,
            matched,
            ..
        } => filled_status(created_order.after().order_base().order_id, matched),
    }
}

fn filled_status(
    oid: u64,
    matched: &MatchSpotOrderPipeChanges,
) -> Result<OrderStatusWire, SpotBlockTranslationError> {
    let trades = match matched {
        MatchSpotOrderPipeChanges::PartiallyFilled { trades, .. }
        | MatchSpotOrderPipeChanges::Filled { trades, .. }
        | MatchSpotOrderPipeChanges::CanceledAfterPartialFill { trades, .. } => trades,
        MatchSpotOrderPipeChanges::Resting | MatchSpotOrderPipeChanges::Rejected { .. } => {
            return Err(SpotBlockTranslationError::MissingTakerTrade { oid });
        }
    };
    let taker_trades = trades
        .iter()
        .map(|trade| &trade.after().comon_trade)
        .filter(|trade| trade.taker_order_id == oid)
        .map(|trade| (trade.price, trade.size));
    let (total_size, average_price) = aggregate_fills(oid, taker_trades)?;

    Ok(OrderStatusWire::Filled {
        filled: FilledOrderStatusWire {
            total_sz: total_size.normalize().to_string(),
            avg_px: average_price.normalize().to_string(),
            oid,
        },
    })
}

fn aggregate_fills(
    oid: u64,
    fills: impl IntoIterator<Item = (Decimal, Decimal)>,
) -> Result<(Decimal, Decimal), SpotBlockTranslationError> {
    let mut fills = fills.into_iter().peekable();
    if fills.peek().is_none() {
        return Err(SpotBlockTranslationError::MissingTakerTrade { oid });
    }

    let mut total_size = Decimal::ZERO;
    let mut total_notional = Decimal::ZERO;
    for (price, size) in fills {
        total_size = total_size
            .checked_add(size)
            .ok_or(SpotBlockTranslationError::ArithmeticOverflow { oid })?;
        let notional = price
            .checked_mul(size)
            .ok_or(SpotBlockTranslationError::ArithmeticOverflow { oid })?;
        total_notional = total_notional
            .checked_add(notional)
            .ok_or(SpotBlockTranslationError::ArithmeticOverflow { oid })?;
    }
    if total_size == Decimal::ZERO {
        return Err(SpotBlockTranslationError::ZeroTotalTradeSize { oid });
    }
    let average_price = total_notional
        .checked_div(total_size)
        .ok_or(SpotBlockTranslationError::ArithmeticOverflow { oid })?;
    Ok((total_size, average_price))
}

fn execution_error_message<BE, LE, OE>(error: &ExecutionErrorV3<BE, LE, OE>) -> String
where
    BE: std::fmt::Display,
    LE: std::fmt::Display,
    OE: std::fmt::Display,
{
    match error {
        ExecutionErrorV3::Business(error) => format!("business failed: {error}"),
        ExecutionErrorV3::Contract(error) => format!("contract failed: {error:?}"),
        ExecutionErrorV3::LoadState(error) => format!("load_state failed: {error}"),
        ExecutionErrorV3::Persist(error) => format!("persist failed: {error}"),
        ExecutionErrorV3::Replay(error) => format!("replay failed: {error}"),
        ExecutionErrorV3::Publish(error) => format!("publish failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    fn order_status(oid: u64) -> OrderStatusWire {
        OrderStatusWire::Resting {
            resting: RestingOrderStatusWire { oid },
        }
    }

    #[test]
    fn empty_execution_returns_no_responses() {
        let execution = SpotBlockExecutionV3 {
            command_results: Vec::new(),
        };

        assert_eq!(translate_spot_block_execution(&execution), Ok(Vec::new()));
    }

    #[test]
    fn groups_only_order_statuses() {
        let expected = vec![ExchangeResponseWire::order(vec![order_status(7)])];

        assert_eq!(
            grouped_responses(vec![ResponseGroup::Order], vec![order_status(7)], vec![]),
            expected
        );
    }

    #[test]
    fn groups_only_cancel_statuses() {
        let expected = vec![ExchangeResponseWire::cancel(vec![
            CancelStatusWire::Success("success"),
        ])];

        assert_eq!(
            grouped_responses(
                vec![ResponseGroup::Cancel],
                vec![],
                vec![CancelStatusWire::Success("success")],
            ),
            expected
        );
    }

    #[test]
    fn preserves_order_then_cancel_group_order() {
        let expected = vec![
            ExchangeResponseWire::order(vec![order_status(7)]),
            ExchangeResponseWire::cancel(vec![CancelStatusWire::Success("success")]),
        ];

        assert_eq!(
            grouped_responses(
                vec![ResponseGroup::Order, ResponseGroup::Cancel],
                vec![order_status(7)],
                vec![CancelStatusWire::Success("success")],
            ),
            expected
        );
    }

    #[test]
    fn preserves_cancel_then_order_group_order() {
        let expected = vec![
            ExchangeResponseWire::cancel(vec![CancelStatusWire::Success("success")]),
            ExchangeResponseWire::order(vec![order_status(7)]),
        ];

        assert_eq!(
            grouped_responses(
                vec![ResponseGroup::Cancel, ResponseGroup::Order],
                vec![order_status(7)],
                vec![CancelStatusWire::Success("success")],
            ),
            expected
        );
    }

    #[test]
    fn aggregates_total_size_and_volume_weighted_average_price() {
        let result = aggregate_fills(
            7,
            [
                (Decimal::new(100, 0), Decimal::new(2, 0)),
                (Decimal::new(110, 0), Decimal::new(3, 0)),
            ],
        );

        assert_eq!(result, Ok((Decimal::new(5, 0), Decimal::new(106, 0))));
    }

    #[test]
    fn rejects_missing_fills() {
        assert_eq!(
            aggregate_fills(7, []),
            Err(SpotBlockTranslationError::MissingTakerTrade { oid: 7 })
        );
    }

    #[test]
    fn rejects_zero_total_size() {
        assert_eq!(
            aggregate_fills(
                7,
                [
                    (Decimal::new(100, 0), Decimal::ONE),
                    (Decimal::new(100, 0), -Decimal::ONE),
                ],
            ),
            Err(SpotBlockTranslationError::ZeroTotalTradeSize { oid: 7 })
        );
    }

    #[test]
    fn rejects_decimal_overflow() {
        assert_eq!(
            aggregate_fills(7, [(Decimal::MAX, Decimal::new(2, 0))]),
            Err(SpotBlockTranslationError::ArithmeticOverflow { oid: 7 })
        );
    }
}
