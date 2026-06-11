use cmd_handler::EntityReplayableEvent;

use super::*;
use crate::entity::{SpotOrderExecution, SpotOrderStatusReason, SpotOrderTimeInForce};

// 这组测试只关心 `compute_replayable_events` 的成功业务语义：
// 1. 当前实现支持哪些 happy path。
// 2. 每种场景应产出什么顺序、什么内容的 replayable events。
// 这里不用 Rustdoc，原因是该文件是私有测试模块，不是对外 API；
// 维护价值主要在于阅读测试时快速理解设计意图，而不是生成文档页面。

fn sample_cmd() -> MatchSpotOrderCmd {
    MatchSpotOrderCmd {
        party_id: "buyer".to_string(),
        taker_order_id: "taker-1".to_string(),
        match_id: "match-1".to_string(),
    }
}

fn build_limit_order(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    price: u64,
    time_in_force: SpotOrderTimeInForce,
    qty: u64,
) -> SpotOrder {
    let (reserved_base, reserved_quote) = match side {
        SpotOrderSide::Buy => (0, qty * price),
        SpotOrderSide::Sell => (qty, 0),
    };
    SpotOrder::new(
        order_id.to_string(),
        10_001,
        Some(42),
        account_id.to_string(),
        "BTCUSDT".to_string(),
        side,
        SpotOrderExecution::Limit { price },
        time_in_force,
        qty,
        reserved_base,
        reserved_quote,
        None,
    )
}

fn build_market_order(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    aggressive_price: u64,
    qty: u64,
) -> SpotOrder {
    let (reserved_base, reserved_quote) = match side {
        SpotOrderSide::Buy => (0, qty * aggressive_price),
        SpotOrderSide::Sell => (qty, 0),
    };
    SpotOrder::new(
        order_id.to_string(),
        10_001,
        Some(42),
        account_id.to_string(),
        "BTCUSDT".to_string(),
        side,
        SpotOrderExecution::Market { aggressive_price },
        SpotOrderTimeInForce::Ioc,
        qty,
        reserved_base,
        reserved_quote,
        None,
    )
}

fn taker_buy_limit(qty: u64, price: u64, tif: SpotOrderTimeInForce) -> SpotOrder {
    build_limit_order("taker-1", "buyer", SpotOrderSide::Buy, price, tif, qty)
}

fn taker_sell_limit(qty: u64, price: u64, tif: SpotOrderTimeInForce) -> SpotOrder {
    build_limit_order("taker-1", "seller", SpotOrderSide::Sell, price, tif, qty)
}

fn taker_buy_market(qty: u64, aggressive_price: u64) -> SpotOrder {
    build_market_order("taker-1", "buyer", SpotOrderSide::Buy, aggressive_price, qty)
}

fn taker_sell_market(qty: u64, aggressive_price: u64) -> SpotOrder {
    build_market_order("taker-1", "seller", SpotOrderSide::Sell, aggressive_price, qty)
}

fn maker_sell(order_id: &str, qty: u64, price: u64, tif: SpotOrderTimeInForce) -> SpotOrder {
    build_limit_order(order_id, "seller", SpotOrderSide::Sell, price, tif, qty)
}

fn maker_buy(order_id: &str, qty: u64, price: u64, tif: SpotOrderTimeInForce) -> SpotOrder {
    build_limit_order(order_id, "buyer", SpotOrderSide::Buy, price, tif, qty)
}

fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

fn event_field_u64(event: &EntityReplayableEvent, field_name: &str) -> Option<u64> {
    event_field(event, field_name)?.parse::<u64>().ok()
}

// 成交事件断言聚焦业务身份和成交事实：
// trade_id / maker_id / account_id / taker_side / maker price / qty。
fn assert_trade_event_for_accounts(
    event: &EntityReplayableEvent,
    expected_trade_id: &str,
    expected_maker_order_id: &str,
    expected_taker_account_id: &str,
    expected_maker_account_id: &str,
    expected_taker_side: SpotOrderSide,
    expected_price: u64,
    expected_qty: u64,
) {
    assert!(event.is_created());
    assert_eq!(event_field(event, "trade_id"), Some(expected_trade_id));
    assert_eq!(event_field(event, "match_id"), Some("match-1"));
    assert_eq!(event_field(event, "taker_order_id"), Some("taker-1"));
    assert_eq!(event_field(event, "maker_order_id"), Some(expected_maker_order_id));
    assert_eq!(event_field(event, "taker_account_id"), Some(expected_taker_account_id));
    assert_eq!(event_field(event, "maker_account_id"), Some(expected_maker_account_id));
    assert_eq!(event_field(event, "taker_side"), Some(expected_taker_side.as_str()));
    assert_eq!(event_field_u64(event, "price"), Some(expected_price));
    assert_eq!(event_field_u64(event, "qty"), Some(expected_qty));
}

// 订单更新事件采用 diff 语义：
// `filled_qty` 只有变化时才会出现在 field changes；
// 版本递增记录在 event header，而不是字段列表里。
fn assert_order_update_event(
    event: &EntityReplayableEvent,
    expected_filled_qty: Option<u64>,
    expected_status: SpotOrderStatus,
    expected_status_reason: Option<SpotOrderStatusReason>,
    expected_old_version: u64,
    expected_new_version: u64,
) {
    assert!(event.is_updated());
    assert_eq!(event.old_version, expected_old_version);
    assert_eq!(event.new_version, expected_new_version);
    assert_eq!(event_field_u64(event, "filled_qty"), expected_filled_qty);
    assert_eq!(event_field(event, "status"), Some(expected_status.as_str()));
    assert_eq!(
        event_field(event, "status_reason"),
        expected_status_reason.map(SpotOrderStatusReason::as_str),
    );
}

#[test]
fn gtc_limit_taker_matches_multiple_makers_and_fills_completely() -> Result<(), MatchSpotOrderError>
{
    // GTC 主路径：连续吃掉多个 maker，直到 taker 完全成交。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 2, 100, SpotOrderTimeInForce::Alo),
        ],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 5);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        99,
        1,
    );
    assert_order_update_event(&events[1], Some(1), SpotOrderStatus::Filled, None, 1, 2);
    assert_trade_event_for_accounts(
        &events[2],
        "match-1-2",
        "maker-2",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        100,
        2,
    );
    assert_order_update_event(&events[3], Some(2), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[4], Some(3), SpotOrderStatus::Filled, None, 1, 2);

    Ok(())
}

#[test]
fn gtc_limit_taker_partially_fills_and_stops_at_first_non_crossing_maker()
-> Result<(), MatchSpotOrderError> {
    // 一旦遇到首个不再交叉的 maker，撮合必须立即停止，后续 maker 不再扫描。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 1, 101, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-3", 1, 100, SpotOrderTimeInForce::Gtc),
        ],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        99,
        1,
    );
    assert_order_update_event(&events[1], Some(1), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[2], Some(1), SpotOrderStatus::PartiallyFilled, None, 1, 2);

    Ok(())
}

#[test]
fn ioc_limit_taker_fills_completely() -> Result<(), MatchSpotOrderError> {
    // IOC 如果刚好全部成交，最终状态应是 `Filled`，而不是 `Canceled`。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(2, 100, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_sell("maker-1", 2, 99, SpotOrderTimeInForce::Gtc)],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        99,
        2,
    );
    assert_order_update_event(&events[1], Some(2), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[2], Some(2), SpotOrderStatus::Filled, None, 1, 2);

    Ok(())
}

#[test]
fn ioc_limit_taker_partially_fills_and_cancels_remaining_qty() -> Result<(), MatchSpotOrderError> {
    // IOC 部分成交后，剩余数量不能继续留在簿上，必须以取消语义结束。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc)],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        99,
        1,
    );
    assert_order_update_event(&events[1], Some(1), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(
        &events[2],
        Some(1),
        SpotOrderStatus::Canceled,
        Some(SpotOrderStatusReason::IocCancelRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn alo_limit_taker_rejects_when_best_maker_would_cross() -> Result<(), MatchSpotOrderError> {
    // ALO 的 happy path 不是“成交成功”，而是“按业务规则正常拒绝并产出拒绝事件”。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Alo),
        maker_orders: vec![maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc)],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 1);
    assert_order_update_event(
        &events[0],
        None,
        SpotOrderStatus::Rejected,
        Some(SpotOrderStatusReason::BadAloPxRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn alo_rejects_before_trade_loop_and_emits_single_taker_update() -> Result<(), MatchSpotOrderError>
{
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Alo),
        maker_orders: vec![
            maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 2, 100, SpotOrderTimeInForce::Gtc),
        ],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 1);
    assert_order_update_event(
        &events[0],
        None,
        SpotOrderStatus::Rejected,
        Some(SpotOrderStatusReason::BadAloPxRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn market_ioc_buy_matches_using_maker_price() -> Result<(), MatchSpotOrderError> {
    // 市价意图只用于 crossing 检查；真实成交价仍然取 maker limit price。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_market(2, 105),
        maker_orders: vec![maker_sell("maker-1", 2, 103, SpotOrderTimeInForce::Gtc)],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        103,
        2,
    );
    assert_order_update_event(&events[1], Some(2), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[2], Some(2), SpotOrderStatus::Filled, None, 1, 2);

    Ok(())
}

#[test]
fn market_ioc_sell_matches_using_maker_price() -> Result<(), MatchSpotOrderError> {
    // 卖方向也要验证同一语义，避免 happy path 只覆盖 buy side。
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_market(2, 95),
        maker_orders: vec![maker_buy("maker-1", 2, 96, SpotOrderTimeInForce::Gtc)],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&cmd, state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "seller",
        "buyer",
        SpotOrderSide::Sell,
        96,
        2,
    );
    assert_order_update_event(&events[1], Some(2), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[2], Some(2), SpotOrderStatus::Filled, None, 1, 2);

    Ok(())
}

#[test]
fn partially_filled_taker_can_continue_matching_to_filled() -> Result<(), MatchSpotOrderError> {
    // 已部分成交的 taker 仍可继续撮合；本次只验证剩余量上的增量行为。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc)
            .with_execution_state(SpotOrderStatus::PartiallyFilled, 1)
            .with_version(7),
        maker_orders: vec![maker_sell("maker-1", 2, 100, SpotOrderTimeInForce::Gtc)],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        100,
        2,
    );
    assert_order_update_event(&events[1], Some(2), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[2], Some(3), SpotOrderStatus::Filled, None, 7, 8);

    Ok(())
}

#[test]
fn partially_filled_maker_can_be_completed_by_current_match() -> Result<(), MatchSpotOrderError> {
    // maker 的已有 filled_qty 也必须被正确累加，并推动 version 前进。
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 3, 99, SpotOrderTimeInForce::Gtc)
                .with_execution_state(SpotOrderStatus::PartiallyFilled, 1)
                .with_version(5),
        ],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "buyer",
        "seller",
        SpotOrderSide::Buy,
        99,
        2,
    );
    assert_order_update_event(&events[1], Some(3), SpotOrderStatus::Filled, None, 5, 6);
    assert_order_update_event(&events[2], Some(2), SpotOrderStatus::PartiallyFilled, None, 1, 2);

    Ok(())
}

#[test]
fn sell_limit_gtc_partially_fills_against_buy_maker() -> Result<(), MatchSpotOrderError> {
    // 覆盖 sell taker + buy maker，避免测试矩阵只锁定一个方向。
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_limit(3, 95, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_buy("maker-1", 1, 97, SpotOrderTimeInForce::Gtc),
            maker_buy("maker-2", 1, 94, SpotOrderTimeInForce::Gtc),
        ],
    };

    let events = MatchSpotOrderUseCase.compute_replayable_events(&cmd, state)?;

    assert_eq!(events.len(), 3);
    assert_trade_event_for_accounts(
        &events[0],
        "match-1-1",
        "maker-1",
        "seller",
        "buyer",
        SpotOrderSide::Sell,
        97,
        1,
    );
    assert_order_update_event(&events[1], Some(1), SpotOrderStatus::Filled, None, 1, 2);
    assert_order_update_event(&events[2], Some(1), SpotOrderStatus::PartiallyFilled, None, 1, 2);

    Ok(())
}

#[test]
fn first_non_crossing_maker_stops_scan_even_if_later_maker_would_cross()
-> Result<(), MatchSpotOrderError> {
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 1, 101, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 1, 100, SpotOrderTimeInForce::Gtc),
        ],
    };

    let result = MatchSpotOrderUseCase.compute_replayable_events(&sample_cmd(), state);

    assert_eq!(result, Err(MatchSpotOrderError::NoTradesMatched));

    Ok(())
}
