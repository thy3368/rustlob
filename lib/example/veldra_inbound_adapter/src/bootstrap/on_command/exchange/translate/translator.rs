use common_entity::ExecutionErrorV3;
use example_core_use_case::pipe::match_spot_order::MatchSpotOrderPipeChanges;
use example_core_use_case::pipe::place_spot_order_pipeline_v2::PlaceSpotOrderPipelineV2Changes;
use rust_decimal::Decimal;
use serde::Serialize;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::{
    SpotBlockCommandExecutionV3, SpotBlockExecutionV3,
};

use crate::bootstrap::on_command::exchange::actions::cancel::reply::{
    CancelResponseWire, CancelStatusWire,
};
use crate::bootstrap::on_command::exchange::actions::order::reply::{
    FilledOrderStatusWire, OrderResponseWire, OrderStatusWire, RestingOrderStatusWire,
};
use crate::bootstrap::on_command::exchange::common::wire::ok_statuses_response;

/// 一个 spot block 按原有 exchange action 类型拆分后的响应。
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum SpotBlockExchangeResponseWire {
    Order(OrderResponseWire),
    Cancel(CancelResponseWire),
}

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
) -> Result<Vec<SpotBlockExchangeResponseWire>, SpotBlockTranslationError> {
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
) -> Vec<SpotBlockExchangeResponseWire> {
    group_order
        .into_iter()
        .map(|group| match group {
            ResponseGroup::Order => SpotBlockExchangeResponseWire::Order(ok_statuses_response(
                "order",
                std::mem::take(&mut order_statuses),
            )),
            ResponseGroup::Cancel => SpotBlockExchangeResponseWire::Cancel(ok_statuses_response(
                "cancel",
                std::mem::take(&mut cancel_statuses),
            )),
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
