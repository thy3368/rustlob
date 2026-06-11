//! `compute_output_and_events(...).events` 规格风格模板
//!
//! 目的:
//! - 把 use case 层测试写成“测试即需求规格”的形式。
//! - 先表达业务规则，再表达输入状态、动作和期望事件。
//!
//! 适用范围:
//! - 适用于 `MatchSpotOrderUseCase::compute_output_and_events(...).events`
//!   这类 use case 层业务场景测试。
//! - 重点是事件顺序、订单生命周期状态、拒绝原因和停止条件。
//!
//! 使用方式:
//! - 这是仓库内的模板文件，不是当前编译中的测试模块。
//! - 复制一个样例或空白骨架到真实测试文件中，再替换成实际场景。
//! - helper 名称请沿用现有测试术语，例如 `sample_cmd()`、
//!   `taker_buy_limit(...)`、`maker_sell(...)`、`assert_order_update_event(...)`、
//!   `assert_trade_event_for_accounts(...)`。

// 建议 helper:
// fn compute_events(
//     cmd: &MatchSpotOrderCmd,
//     state: MatchSpotOrderState,
// ) -> Result<Vec<EntityReplayableEvent>, MatchSpotOrderError> {
//     Ok(CommandUseCase3::compute_output_and_events(&MatchSpotOrderUseCase, cmd, state)?.events)
// }

// -----------------------------------------------------------------------------
// 规格矩阵模板
// -----------------------------------------------------------------------------
//
// Rule:
// - 用一句业务规则概括这个测试为什么存在。
//
// Matrix:
// - taker side: buy / sell
// - execution + tif: GTC limit / IOC limit / IOC market / ALO limit
// - outcome: full fill / partial fill / zero fill reject / pre-reject / stop scanning
// - event expectation: trade count / maker update count / taker update count / event order
// - current coverage / uncovered:
//   - covered:
//   - uncovered:
//
// 建议先维护规格矩阵，再补具体测试，避免只按实现细节零散加 case。

// -----------------------------------------------------------------------------
// 命名规范
// -----------------------------------------------------------------------------
//
// 规则:
// - 测试名必须表达业务规则，不表达实现细节。
// - 优先使用 `who + condition + outcome`。
// - 名称里尽量显式带出关键业务条件，例如 `ioc`、`alo`、`no_crossing`、
//   `partial_fill`、`stop_at_first_non_crossing_maker`。
//
// bad example:
// - `test_compute_events_1`
// - `match_order_returns_expected_vec`
// - `should_work_for_ioc`
//
// good example:
// - `ioc_limit_taker_with_no_crossing_maker_rejects_single_taker_update`
// - `gtc_limit_taker_partially_fills_and_stops_at_first_non_crossing_maker`
// - `alo_rejects_before_trade_loop_and_emits_single_taker_update`
// - `market_ioc_buy_with_no_liquidity_rejects_with_market_reason`
// - `partially_filled_taker_continues_matching_until_filled`
// - `partially_filled_maker_continues_matching_until_filled`

// -----------------------------------------------------------------------------
// 断言规范
// -----------------------------------------------------------------------------
//
// - 只要发生真实成交，就断言 trade event。
// - 每个发生了成交数量或生命周期变化的 maker / taker，都断言 update event。
// - `filled_qty` 未变化时，应在 `assert_order_update_event(...)` 中断言 `None`，
//   不要伪造 `Some(0)`。
// - 零成交拒绝、ALO 预拒绝、IOC 剩余取消等“收尾语义”，必须断言 `status_reason`。
// - 正常成交收尾且没有拒绝原因时，断言 `status_reason == None`。
// - 多事件场景必须断言顺序，顺序本身就是业务规格的一部分:
//   - 先 trade
//   - 再 maker update
//   - 最后 taker update

// -----------------------------------------------------------------------------
// 完整样例 1: buy IOC zero-fill reject
// -----------------------------------------------------------------------------

#[test]
fn ioc_limit_taker_with_no_crossing_maker_rejects_single_taker_update_template()
-> Result<(), MatchSpotOrderError> {
    // Rule:
    // - IOC taker 在没有任何可成交 maker 时，不返回业务错误；
    //   而是产出一条带拒绝原因的 taker update event。
    //
    // Given:
    // - taker 是 buy side IOC limit。
    // - 最优 maker 价格也不交叉。
    // - 因为零成交，所以不会产生 trade event。
    //
    // When:
    // - 调用 `compute_output_and_events(...).events`。
    //
    // Then:
    // - 只产生 1 条 event。
    // - 该 event 是 taker update。
    // - `filled_qty` 未变化，因此断言 `None`。
    // - `status` 是 `Rejected`。
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

// -----------------------------------------------------------------------------
// 完整样例 2: GTC partial fill + stop at first non-crossing maker
// -----------------------------------------------------------------------------

#[test]
fn gtc_limit_taker_partially_fills_and_stops_at_first_non_crossing_maker_template()
-> Result<(), MatchSpotOrderError> {
    // Rule:
    // - maker 已按撮合优先级排序。
    // - 一旦遇到首个不再交叉的 maker，扫描必须立即停止；
    //   即使后面还有会交叉的 maker，也不能继续撮合。
    //
    // Given:
    // - taker 是 buy side GTC limit。
    // - maker-1 可成交。
    // - maker-2 是首个不交叉价格。
    // - maker-3 即使价格可成交，也不应再被扫描。
    //
    // When:
    // - 调用 `compute_output_and_events(...).events`。
    //
    // Then:
    // - 先出现 1 条 trade event。
    // - 再出现 maker-1 update event。
    // - 最后出现 taker partial-fill update event。
    // - 不应出现与 maker-3 相关的任何事件。

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

// -----------------------------------------------------------------------------
// 空白骨架: 复制后替换业务语义
// -----------------------------------------------------------------------------

#[test]
fn replace_with_who_condition_outcome_template_name() -> Result<(), MatchSpotOrderError> {
    // Rule:
    // -
    //
    // Given:
    // -
    //
    // When:
    // -
    //
    // Then:
    // -

    // arrange
    let state = MatchSpotOrderState {
        taker_order: todo!("replace taker"),
        maker_orders: vec![
            todo!("replace makers"),
        ],
    };
    let cmd = sample_cmd();

    // act
    let events = compute_events(&cmd, state)?;

    // assert
    assert_eq!(events.len(), todo!("expected event count"));

    // trade event expectations
    // assert_trade_event_for_accounts(&events[0], ...);

    // maker update expectations
    // assert_order_update_event(&events[1], Some(...), SpotOrderStatus::..., None, 1, 2);

    // taker update expectations
    // assert_order_update_event(
    //     &events[n],
    //     Some(...) or None,
    //     SpotOrderStatus::...,
    //     Some(SpotOrderStatusReason::...) or None,
    //     1,
    //     2,
    // );

    Ok(())
}

// -----------------------------------------------------------------------------
// 命名对照表
// -----------------------------------------------------------------------------
//
// bad example:
// - `gtc_case_1`
// - `partial_fill_test`
// - `compute_output_and_events_should_return_three_events`
//
// good example:
// - `gtc_limit_taker_matches_multiple_makers_and_fills_completely`
// - `ioc_limit_taker_partially_fills_and_cancels_remaining_qty`
// - `sell_ioc_limit_with_no_crossing_maker_rejects_single_taker_update`
// - `market_ioc_buy_matches_using_maker_price`
//
// 额外命名示例:
// - 零成交:
//   `market_ioc_sell_with_no_liquidity_rejects_with_market_reason`
// - 预拒绝:
//   `alo_limit_taker_rejects_when_best_maker_would_cross`
// - 停止扫描:
//   `first_non_crossing_maker_stops_scan_even_if_later_maker_would_cross`
// - 部分成交收尾:
//   `sell_ioc_limit_partially_fills_and_cancels_remaining_qty`
