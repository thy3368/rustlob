use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::CommandUseCase3;

use super::*;
use crate::entity::{
    SpotOrderExecution, SpotOrderSide, SpotOrderStatusReason, SpotOrderTimeInForce,
};

// 目的:
// - 把 `compute_replayable_events` 的 happy path 测试写成“测试即需求规格”。
// - 每个测试先表达业务规则，再表达输入状态、动作和期望事件。
//
// 适用范围:
// - 这里只覆盖 use case 层成功业务语义。
// - 重点是事件顺序、订单生命周期状态、拒绝原因和停止扫描条件。
//
// 规格矩阵:
// - taker side: buy / sell
// - execution + tif: GTC limit / IOC limit / IOC market / ALO limit
// - outcome: full fill / partial fill / zero fill reject / pre-reject / stop scanning
// - event expectation: trade count / maker update count / taker update count / event order
//
// current coverage:
// - GTC buy full fill
// - GTC buy partial fill + stop at first non-crossing maker
// - GTC sell partial fill
// - IOC limit buy full / partial / zero-fill reject
// - IOC limit sell partial / zero-fill reject
// - IOC market buy full / zero-liquidity reject
// - IOC market sell full / zero-liquidity reject
// - ALO buy pre-reject
// - partially filled taker continue matching to filled
// - partially filled maker continue matching to filled
// - first non-crossing maker stops scan even if later maker would cross
//
// 断言规范:
// - 只要发生真实成交，就断言 trade event。
// - 每个发生了成交数量或生命周期变化的 maker / taker，都断言 update event。
// - `filled_qty` 未变化时，断言 `None`，不要伪造 `Some(0)`。
// - 多事件场景必须断言顺序: 先 trade，再 maker update，最后 taker update。

fn compute_events(
    cmd: &MatchSpotOrderCmd,
    state: MatchSpotOrderState,
) -> Result<Vec<EntityReplayableEvent>, MatchSpotOrderError> {
    Ok(CommandUseCase3::compute_output_and_events(&MatchSpotOrderUseCase, cmd, state)?.events)
}

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
    // Rule:
    // - GTC taker 可以连续吃掉多个按优先级排序的 maker，直到自身完全成交。
    //
    // Given:
    // - taker 是 buy side GTC limit。
    // - maker-1 和 maker-2 都与 taker 价格交叉。
    // - 两个 maker 的数量之和刚好填满 taker。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 事件顺序必须是 trade -> maker update -> trade -> maker update -> taker update。
    // - 两个 maker 都变成 `Filled`。
    // - taker 最终变成 `Filled`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 2, 100, SpotOrderTimeInForce::Alo),
        ],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
    // Rule:
    // - 一旦遇到首个不再交叉的 maker，扫描必须立即停止。
    //
    // Given:
    // - taker 是 buy side GTC limit。
    // - maker-1 可成交。
    // - maker-2 是首个不交叉价格。
    // - maker-3 即使可成交，也不应再被扫描。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生与 maker-1 相关的成交和更新事件。
    // - taker 最终停在 `PartiallyFilled`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 1, 101, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-3", 1, 100, SpotOrderTimeInForce::Gtc),
        ],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
    // Rule:
    // - IOC taker 如果刚好全部成交，最终状态应是 `Filled`，而不是 `Canceled`。
    //
    // Given:
    // - taker 是 buy side IOC limit。
    // - 唯一 maker 的数量刚好填满 taker。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 产生 1 条 trade、1 条 maker update、1 条 taker update。
    // - maker 和 taker 都变成 `Filled`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(2, 100, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_sell("maker-1", 2, 99, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
    // Rule:
    // - IOC taker 部分成交后，剩余数量不能继续留在簿上，必须以取消语义结束。
    //
    // Given:
    // - taker 是 buy side IOC limit。
    // - 只有一张 maker 可成交，且数量不足以填满 taker。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 先产生 trade 和 maker update。
    // - taker 最后以 `Canceled + IocCancelRejected` 收尾。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
fn ioc_limit_taker_with_no_crossing_maker_rejects_single_taker_update()
-> Result<(), MatchSpotOrderError> {
    // Rule:
    // - IOC taker 零成交时不会返回业务错误，而是产出一条带拒绝原因的 taker update event。
    //
    // Given:
    // - taker 是 buy side IOC limit。
    // - 最优 maker 也不与 taker 价格交叉。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生 1 条 taker update event。
    // - `filled_qty` 未变化，因此断言 `None`。
    // - `status_reason` 是 `IocCancelRejected`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_sell("maker-1", 1, 101, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
    assert_eq!(events.len(), 1);
    assert_order_update_event(
        &events[0],
        None,
        SpotOrderStatus::Rejected,
        Some(SpotOrderStatusReason::IocCancelRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn alo_limit_taker_rejects_when_best_maker_would_cross() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // - ALO taker 的 happy path 可以是“按业务规则正常预拒绝”。
    //
    // Given:
    // - taker 是 buy side ALO limit。
    // - 当前最优 maker 会与 taker 立即成交。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生 1 条 taker reject update event。
    // - `status_reason` 是 `BadAloPxRejected`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Alo),
        maker_orders: vec![maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
    // Rule:
    // - ALO 预拒绝必须发生在 trade loop 之前。
    //
    // Given:
    // - taker 是 buy side ALO limit。
    // - 最优 maker 已经会与 taker 立即成交。
    // - 后面即使还有更多可成交 maker，也不应进入 trade loop。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只能看到 1 条 taker reject update event。
    // - 不会出现任何 trade event。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Alo),
        maker_orders: vec![
            maker_sell("maker-1", 1, 99, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 2, 100, SpotOrderTimeInForce::Gtc),
        ],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
fn market_ioc_buy_with_no_liquidity_rejects_with_market_reason() -> Result<(), MatchSpotOrderError>
{
    // Rule:
    // - market IOC taker 零成交时，拒绝原因必须保留为 market 语义。
    //
    // Given:
    // - taker 是 buy side market IOC。
    // - 最优 maker 价格高于 aggressive price，因此无法成交。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生 1 条 taker reject update event。
    // - `status_reason` 必须是 `MarketOrderNoLiquidityRejected`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_market(2, 105),
        maker_orders: vec![maker_sell("maker-1", 2, 106, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
    assert_eq!(events.len(), 1);
    assert_order_update_event(
        &events[0],
        None,
        SpotOrderStatus::Rejected,
        Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn market_ioc_sell_with_no_liquidity_rejects_with_market_reason() -> Result<(), MatchSpotOrderError>
{
    // Rule:
    // - sell side market IOC 零成交时，也必须保留同一条 market reject 语义。
    //
    // Given:
    // - taker 是 sell side market IOC。
    // - 最优 maker 价格低于 aggressive price，因此无法成交。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生 1 条 taker reject update event。
    // - `status_reason` 必须是 `MarketOrderNoLiquidityRejected`。

    // arrange
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_market(2, 95),
        maker_orders: vec![maker_buy("maker-1", 2, 94, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&cmd, state)?;

    // assert
    assert_eq!(events.len(), 1);
    assert_order_update_event(
        &events[0],
        None,
        SpotOrderStatus::Rejected,
        Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn market_ioc_buy_matches_using_maker_price() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // - market taker 的 aggressive price 只用于 crossing 检查，真实成交价仍然取 maker limit price。
    //
    // Given:
    // - taker 是 buy side market IOC。
    // - maker 与 taker 可成交。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - trade event 中的价格必须等于 maker limit price。
    // - maker 和 taker 都变成 `Filled`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_market(2, 105),
        maker_orders: vec![maker_sell("maker-1", 2, 103, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
fn sell_ioc_limit_partially_fills_and_cancels_remaining_qty() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // - IOC 的“部分成交后取消剩余”不能只在 buy side 成立，sell side 也必须成立。
    //
    // Given:
    // - taker 是 sell side IOC limit。
    // - 只有一张 buy maker 可成交，且数量不足以填满 taker。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 先产生 trade 和 maker update。
    // - taker 最后以 `Canceled + IocCancelRejected` 收尾。

    // arrange
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_limit(3, 95, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_buy("maker-1", 1, 97, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&cmd, state)?;

    // assert
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
fn sell_ioc_limit_with_no_crossing_maker_rejects_single_taker_update()
-> Result<(), MatchSpotOrderError> {
    // Rule:
    // - sell side IOC 零成交时，也应以单条 reject update event 结束，而不是返回业务错误。
    //
    // Given:
    // - taker 是 sell side IOC limit。
    // - 最优 buy maker 价格不交叉。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生 1 条 taker reject update event。
    // - `filled_qty` 未变化，因此断言 `None`。
    // - `status_reason` 是 `IocCancelRejected`。

    // arrange
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_limit(3, 95, SpotOrderTimeInForce::Ioc),
        maker_orders: vec![maker_buy("maker-1", 1, 94, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&cmd, state)?;

    // assert
    assert_eq!(events.len(), 1);
    assert_order_update_event(
        &events[0],
        None,
        SpotOrderStatus::Rejected,
        Some(SpotOrderStatusReason::IocCancelRejected),
        1,
        2,
    );

    Ok(())
}

#[test]
fn market_ioc_sell_matches_using_maker_price() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // - sell side market taker 也必须以 maker limit price 成交。
    //
    // Given:
    // - taker 是 sell side market IOC。
    // - maker 与 taker 可成交。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - trade event 中的价格必须等于 maker limit price。
    // - maker 和 taker 都变成 `Filled`。

    // arrange
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_market(2, 95),
        maker_orders: vec![maker_buy("maker-1", 2, 96, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&cmd, state)?;

    // assert
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
fn partially_filled_taker_continues_matching_until_filled() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // - 已部分成交的 taker 仍可继续撮合，且本次只应在剩余量上追加成交。
    //
    // Given:
    // - taker 初始状态已经是 `PartiallyFilled`，`filled_qty = 1`，版本是 7。
    // - 当前 maker 的剩余数量刚好可以补齐 taker。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - trade event 只记录本次新增成交量。
    // - maker 变成 `Filled`。
    // - taker 从 version 7 前进到 8，并最终变成 `Filled`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc)
            .with_execution_state(SpotOrderStatus::PartiallyFilled, 1)
            .with_version(7),
        maker_orders: vec![maker_sell("maker-1", 2, 100, SpotOrderTimeInForce::Gtc)],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
fn partially_filled_maker_continues_matching_until_filled() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // - 已部分成交的 maker 也可以继续撮合，且已有 filled_qty 必须被正确累加。
    //
    // Given:
    // - maker 初始状态已经是 `PartiallyFilled`，`filled_qty = 1`，版本是 5。
    // - 当前 taker 还能再吃掉 maker 剩余的 2。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - trade event 只记录本次新增成交量。
    // - maker 从 version 5 前进到 6，并最终变成 `Filled`。
    // - taker 最终停在 `PartiallyFilled`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 3, 99, SpotOrderTimeInForce::Gtc)
                .with_execution_state(SpotOrderStatus::PartiallyFilled, 1)
                .with_version(5),
        ],
    };

    // act
    let events = compute_events(&sample_cmd(), state)?;

    // assert
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
    // Rule:
    // - GTC sell taker 也必须遵守与 buy side 对称的撮合语义。
    //
    // Given:
    // - taker 是 sell side GTC limit。
    // - maker-1 可成交。
    // - maker-2 不交叉，因此扫描应在 maker-1 后停止。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 只产生与 maker-1 相关的成交和更新事件。
    // - taker 最终停在 `PartiallyFilled`。

    // arrange
    let cmd = MatchSpotOrderCmd { party_id: "seller".to_string(), ..sample_cmd() };
    let state = MatchSpotOrderState {
        taker_order: taker_sell_limit(3, 95, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_buy("maker-1", 1, 97, SpotOrderTimeInForce::Gtc),
            maker_buy("maker-2", 1, 94, SpotOrderTimeInForce::Gtc),
        ],
    };

    // act
    let events = compute_events(&cmd, state)?;

    // assert
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
    // Rule:
    // - 撮合遵循订单簿顺序语义，而不是“价格集合里是否存在可成交单”的语义。
    //
    // Given:
    // - taker 是 buy side GTC limit。
    // - maker-1 是首个不交叉价格。
    // - maker-2 虽然会交叉，也不应被继续扫描。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 因为首个 maker 已不交叉，整次撮合应直接结束。
    // - 返回 `Err(MatchSpotOrderError::NoTradesMatched)`。

    // arrange
    let state = MatchSpotOrderState {
        taker_order: taker_buy_limit(3, 100, SpotOrderTimeInForce::Gtc),
        maker_orders: vec![
            maker_sell("maker-1", 1, 101, SpotOrderTimeInForce::Gtc),
            maker_sell("maker-2", 1, 100, SpotOrderTimeInForce::Gtc),
        ],
    };

    // act
    let result = compute_events(&sample_cmd(), state);

    // assert
    assert_eq!(result, Err(MatchSpotOrderError::NoTradesMatched));

    Ok(())
}
