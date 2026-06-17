use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

use super::*;

// 目的:
// - 把 `compute_changes()` 与 `to_replayable_events()` 的成功路径写成“测试即规格”。
// - 先断言 `Changes` 业务真相，再断言 replayable event 投影与顺序。
//
// 规格矩阵:
// - 仓位意图: flat open / same-side increase / opposite partial reduce / opposite exact close / opposite flip / reduce_only
// - 下单方向: buy / sell
// - 执行意图: limit(Gtc/Ioc/Alo) / market(Ioc)
// - 业务结果: freeze full order margin / freeze net-new margin only / freeze zero
// - 事件期望: order create 必有；balance update 仅在余额真实变化时存在；多事件顺序固定为 order create -> balance update
//
// current coverage:
// - flat buy limit: Gtc / Ioc / Alo
// - flat sell limit
// - flat buy market
// - flat sell market
// - long buy same-side increase
// - short sell same-side increase
// - long sell reduce-only
// - short buy reduce-only
// - long sell opposite partial reduce
// - long sell opposite exact close
// - long sell opposite flip with net-new short margin
// - short buy opposite partial reduce
// - short buy opposite flip with net-new long margin

fn compute_changes_and_events(
    cmd: &PlaceHyperliquidPerpOrderCmd,
    state: PlaceHyperliquidPerpOrderState,
) -> (PlaceHyperliquidPerpOrderChanges, Vec<cmd_handler::EntityReplayableEvent>) {
    let changes = use_case().compute_changes(cmd, state).unwrap();
    let events = changes.to_replayable_events().unwrap();
    (changes, events)
}

fn assert_single_balance_pair(
    changes: &PlaceHyperliquidPerpOrderChanges,
    expected_before_available: u64,
    expected_before_frozen: u64,
    expected_after_available: u64,
    expected_after_frozen: u64,
) {
    assert_eq!(changes.updated_margin_balances.len(), 1);
    let pair = &changes.updated_margin_balances[0];
    assert_balance_snapshot(&pair.before, expected_before_available, expected_before_frozen, 4);
    assert_balance_snapshot(&pair.after, expected_after_available, expected_after_frozen, 5);
}

#[test]
fn flat_account_buys_limit_order_and_freezes_margin_for_each_supported_tif() {
    for (time_in_force, expected_tif) in [
        (HyperliquidPerpOrderTimeInForce::Gtc, "gtc"),
        (HyperliquidPerpOrderTimeInForce::Ioc, "ioc"),
        (HyperliquidPerpOrderTimeInForce::Alo, "alo"),
    ] {
        // Rule:
        // - 空仓账户提交 buy limit 时，应创建 open 订单并冻结整单所需 Cross 保证金。
        //
        // Given:
        // - 账户当前是 flat position。
        // - 执行方式是 buy limit。
        // - tif 在 gtc / ioc / alo 三种成功路径内切换。
        //
        // When:
        // - 调用 `compute_changes()`，再投影 replayable events。
        //
        // Then:
        // - `Changes` 先记录 created order 和一条 balance before/after pair。
        // - 事件顺序必须是 order create 在前，balance update 在后。

        // arrange
        let cmd = PlaceHyperliquidPerpOrderCmd {
            execution: PlaceHyperliquidPerpOrderExecution::Limit { price: 101, time_in_force },
            ..limit_cmd()
        };
        assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

        // act
        let (changes, events) = compute_changes_and_events(&cmd, state());

        // assert
        assert_order_snapshot(
            &changes.created_order,
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 101 },
            time_in_force,
            3,
            false,
        );
        assert_single_balance_pair(&changes, 10_000, 500, 9_939, 561);
        assert_eq!(events.len(), 2);
        assert_order_created_event(&events[0], "buy", "limit", expected_tif, 3, 101, false);
        assert_balance_updated_event(&events[1], 9_939, 561);
    }
}

#[test]
fn flat_account_sells_limit_order_and_freezes_full_order_margin() {
    // Rule:
    // - 空仓账户提交 sell limit 时，也按整单名义价值冻结 Cross 保证金。
    //
    // Given:
    // - 账户当前是 flat position。
    // - 命令是 sell limit。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 保留 sell open order 和单条 balance pair。
    // - 事件顺序必须是 order create -> balance update。

    // arrange
    let cmd = sell_limit_cmd();
    assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, state());

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_939, 561);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, false);
    assert_balance_updated_event(&events[1], 9_939, 561);
}

#[test]
fn flat_account_buys_market_order_and_projects_market_ioc_with_aggressive_price() {
    // Rule:
    // - 空仓账户提交 market buy 意图时，订单快照应固化为 market + ioc，并按 aggressive price 冻结保证金。
    //
    // Given:
    // - 账户当前是 flat position。
    // - 命令是 market buy，aggressive price 为 111。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 记录 market + ioc 订单和余额冻结 pair。
    // - 事件记录 execution=market、time_in_force=ioc、price=111。

    // arrange
    let cmd = market_cmd();
    assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, state());

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Market { aggressive_price: 111 },
        HyperliquidPerpOrderTimeInForce::Ioc,
        3,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_933, 567);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "buy", "market", "ioc", 3, 111, false);
    assert_balance_updated_event(&events[1], 9_933, 567);
}

#[test]
fn flat_account_sells_market_order_and_projects_market_ioc_with_aggressive_price() {
    // Rule:
    // - 空仓账户提交 market sell 意图时，也应固化为 market + ioc，并按 aggressive price 冻结保证金。
    //
    // Given:
    // - 账户当前是 flat position。
    // - 命令是 market sell，aggressive price 为 111。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 记录 sell market + ioc 订单和余额冻结 pair。
    // - 事件记录 sell / market / ioc / 111。

    // arrange
    let cmd = sell_market_cmd();
    assert_eq!(use_case().validate_against_state(&cmd, &state()), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, state());

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Market { aggressive_price: 111 },
        HyperliquidPerpOrderTimeInForce::Ioc,
        3,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_933, 567);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "sell", "market", "ioc", 3, 111, false);
    assert_balance_updated_event(&events[1], 9_933, 567);
}

#[test]
fn long_position_buys_same_side_and_freezes_full_new_order_margin() {
    // Rule:
    // - 同向加 long 仓时，仍按整单新下单数量冻结新增保证金。
    //
    // Given:
    // - 当前已有 long position。
    // - 命令继续 buy，同向增加敞口。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 仍产生单条余额冻结 pair。
    // - 事件顺序仍是 order create -> balance update。

    // arrange
    let cmd = limit_cmd();
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_939, 561);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 3, 101, false);
    assert_balance_updated_event(&events[1], 9_939, 561);
}

#[test]
fn short_position_sells_same_side_and_freezes_full_new_order_margin() {
    // Rule:
    // - 同向加 short 仓时，也按整单新下单数量冻结新增保证金。
    //
    // Given:
    // - 当前已有 short position。
    // - 命令继续 sell，同向增加敞口。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 仍产生单条余额冻结 pair。
    // - 事件顺序仍是 order create -> balance update。

    // arrange
    let cmd = sell_limit_cmd();
    let short_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, short_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_939, 561);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, false);
    assert_balance_updated_event(&events[1], 9_939, 561);
}

#[test]
fn long_position_places_sell_reduce_only_order_without_freezing_margin() {
    // Rule:
    // - long 仓位上的 sell reduce-only 只允许减仓，不应新增冻结保证金。
    //
    // Given:
    // - 当前已有 long position。
    // - 命令是 sell reduce-only。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let mut cmd = sell_limit_cmd();
    cmd.reduce_only = true;
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        true,
    );
    assert!(changes.updated_margin_balances.is_empty());
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, true);
}

#[test]
fn short_position_places_buy_reduce_only_order_without_freezing_margin() {
    // Rule:
    // - short 仓位上的 buy reduce-only 只允许减仓，不应新增冻结保证金。
    //
    // Given:
    // - 当前已有 short position。
    // - 命令是 buy reduce-only。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let mut cmd = limit_cmd();
    cmd.reduce_only = true;
    let short_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, short_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        true,
    );
    assert!(changes.updated_margin_balances.is_empty());
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 3, 101, true);
}

#[test]
fn long_position_places_opposite_sell_that_only_reduces_without_new_margin() {
    // Rule:
    // - 非 reduce-only 的 opposite-side sell，只要仍在 long 净仓内，就只减仓不新增冻结保证金。
    //
    // Given:
    // - 当前已有 5 手 long position。
    // - 命令是 3 手 sell limit，仍小于当前净仓。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let cmd = sell_limit_cmd();
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        false,
    );
    assert!(changes.updated_margin_balances.is_empty());
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, false);
}

#[test]
fn long_position_places_opposite_sell_that_exactly_closes_without_new_margin() {
    // Rule:
    // - 非 reduce-only 的 opposite-side sell 在数量刚好等于当前 long 净仓时，应精确平仓且不新增冻结保证金。
    //
    // Given:
    // - 当前已有 5 手 long position。
    // - 命令是 5 手 sell limit，刚好等于当前净仓。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let mut cmd = sell_limit_cmd();
    cmd.size = 5;
    cmd.reduce_only = false;
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        5,
        false,
    );
    assert!(changes.updated_margin_balances.is_empty());
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 5, 101, false);
}

#[test]
fn long_position_places_opposite_sell_flip_and_only_freezes_net_new_short_margin() {
    // Rule:
    // - opposite-side sell 超过当前 long 净仓时，只按净新增 short 敞口冻结保证金，而不是按整单冻结。
    //
    // Given:
    // - 当前已有 5 手 long position。
    // - 命令是 8 手 sell limit，净新增 short 只有 3 手。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 中的 balance pair 只反映 3 手净新增敞口的冻结。
    // - 事件顺序必须是 order create -> balance update。

    // arrange
    let mut cmd = sell_limit_cmd();
    cmd.size = 8;
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        8,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_939, 561);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 8, 101, false);
    assert_balance_updated_event(&events[1], 9_939, 561);
}

#[test]
fn short_position_places_opposite_buy_that_only_reduces_without_new_margin() {
    // Rule:
    // - 非 reduce-only 的 opposite-side buy，只要仍在 short 净仓内，就只减仓不新增冻结保证金。
    //
    // Given:
    // - 当前已有 5 手 short position。
    // - 命令是 3 手 buy limit，仍小于当前净仓。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let cmd = limit_cmd();
    let short_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, short_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        false,
    );
    assert!(changes.updated_margin_balances.is_empty());
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 3, 101, false);
}

#[test]
fn short_position_places_opposite_buy_flip_and_only_freezes_net_new_long_margin() {
    // Rule:
    // - opposite-side buy 超过当前 short 净仓时，只按净新增 long 敞口冻结保证金，而不是按整单冻结。
    //
    // Given:
    // - 当前已有 5 手 short position。
    // - 命令是 8 手 buy limit，净新增 long 只有 3 手。
    //
    // When:
    // - 调用 `compute_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 中的 balance pair 只反映 3 手净新增敞口的冻结。
    // - 事件顺序必须是 order create -> balance update。

    // arrange
    let mut cmd = limit_cmd();
    cmd.size = 8;
    let short_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_changes_and_events(&cmd, short_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        8,
        false,
    );
    assert_single_balance_pair(&changes, 10_000, 500, 9_939, 561);
    assert_eq!(events.len(), 2);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 8, 101, false);
    assert_balance_updated_event(&events[1], 9_939, 561);
}
