use cmd_handler::command_use_case_def2::ReplayableChanges;
use common_entity::{MiStateMachineV2, MiStateMachineV2Unchecked};

use super::*;

// 目的:
// - 把 `compute_after_changes()` 与 `to_replayable_events()` 的成功路径写成“测试即规格”。
// - 先断言 `Changes` 业务真相，再断言 replayable event 投影与顺序。
//
// 规格矩阵:
// - 仓位意图: flat open / same-side increase / opposite partial close / opposite exact close / reduce_only
// - 下单方向: buy / sell
// - 执行意图: limit(Gtc/Ioc/Alo) / market(Ioc)
// - 业务结果: open freezes full order margin / close freezes zero / flip requires split
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
// - long sell opposite partial close
// - long sell opposite exact close
// - short buy opposite partial close

fn compute_after_changes_and_events(
    cmd: &PlaceHyperliquidPerpOrderCmd,
    state: PlaceHyperliquidPerpOrderState,
) -> (PlaceHyperliquidPerpOrderChanges, Vec<cmd_handler::EntityReplayableEvent>) {
    let changes = use_case().compute_after_changes(cmd, &state).unwrap();
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

fn assert_freeze_ledger_entry_matches_balance_pair(changes: &PlaceHyperliquidPerpOrderChanges) {
    let reservation =
        changes.created_order.reservation.as_ref().expect("open order must record reservation");
    let ledger_entry = changes
        .created_balance_ledger_entry
        .as_ref()
        .expect("open order must create freeze ledger entry");

    assert_eq!(ledger_entry.operation, crate::entity::BalanceLedgerOperation::Freeze);
    assert_eq!(ledger_entry.amount, reservation.original_amount);
    assert_eq!(
        ledger_entry.reason,
        crate::entity::BalanceLedgerReason::FreezeForOrder {
            order_id: changes.created_order.order_id.clone()
        }
    );
    assert!(ledger_entry.matches_balance_update(&changes.updated_margin_balances[0]));
}

fn assert_no_margin_ledger_or_balance_pair(changes: &PlaceHyperliquidPerpOrderChanges) {
    assert!(changes.created_balance_ledger_entry.is_none());
    assert!(changes.updated_margin_balances.is_empty());
}

fn assert_balance_ledger_created_event(event: &cmd_handler::EntityReplayableEvent) {
    assert_eq!(event.change_type, 0);
    assert_eq!(event.entity_type, <crate::entity::BalanceLedgerEntryV2 as Entity>::entity_type());
}

fn use_case() -> PlaceHyperliquidPerpOrderUseCase {
    PlaceHyperliquidPerpOrderUseCase
}

fn limit_cmd() -> PlaceHyperliquidPerpOrderCmd {
    PlaceHyperliquidPerpOrderCmd {
        party_id: "acct-1".to_string(),
        asset: 0,
        symbol: "BTC-PERP".to_string(),
        is_buy: true,
        size: 3,
        reduce_only: false,
        execution: PlaceHyperliquidPerpOrderExecution::Limit {
            price: 101,
            time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
        },
        cloid: Some("client-order-1".to_string()),
    }
}

fn sell_limit_cmd() -> PlaceHyperliquidPerpOrderCmd {
    PlaceHyperliquidPerpOrderCmd { is_buy: false, ..limit_cmd() }
}

fn market_cmd() -> PlaceHyperliquidPerpOrderCmd {
    PlaceHyperliquidPerpOrderCmd {
        execution: PlaceHyperliquidPerpOrderExecution::Market { aggressive_price: 111 },
        ..limit_cmd()
    }
}

fn sell_market_cmd() -> PlaceHyperliquidPerpOrderCmd {
    PlaceHyperliquidPerpOrderCmd { is_buy: false, ..market_cmd() }
}

fn state() -> PlaceHyperliquidPerpOrderState {
    PlaceHyperliquidPerpOrderState {
        trading_enabled: true,
        next_order_sequence: 42,
        account_id: "acct-1".to_string(),
        margin_balance: Balance::new("acct-1".to_string(), "USDC".to_string(), 10_000, 500, 4),
        margin_asset_id: "USDC".to_string(),
        market_rules: MarketRules { symbol: "BTC-PERP".to_string(), min_qty: 1 },
        position: HyperliquidPerpPosition::empty_slot(
            "position-1".to_string(),
            "acct-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            5,
        ),
    }
}

fn non_flat_position(side: HyperliquidPerpPositionSide, qty: u64) -> HyperliquidPerpPosition {
    HyperliquidPerpPosition::new(
        "position-1".to_string(),
        "acct-1".to_string(),
        0,
        "BTC-PERP".to_string(),
        side,
        qty,
        100,
        5,
        crate::entity::HyperliquidPerpMarginMode::Cross,
        qty * 20,
        None,
        0,
        0,
        4,
    )
}

fn assert_balance_snapshot(
    balance: &Balance,
    expected_available: u64,
    expected_frozen: u64,
    expected_version: u64,
) {
    assert_eq!(balance.account_id, "acct-1");
    assert_eq!(balance.asset_id, "USDC");
    assert_eq!(balance.available, expected_available);
    assert_eq!(balance.frozen, expected_frozen);
    assert_eq!(balance.version, expected_version);
}

fn assert_order_snapshot(
    order: &HyperliquidPerpOrder,
    expected_side: HyperliquidPerpOrderSide,
    expected_execution: HyperliquidPerpOrderExecution,
    expected_time_in_force: HyperliquidPerpOrderTimeInForce,
    expected_qty: u64,
    expected_reduce_only: bool,
) {
    assert_eq!(order.order_id, "acct-1-BTC-PERP-42");
    assert_eq!(order.account_id, "acct-1");
    assert_eq!(order.asset, 0);
    assert_eq!(order.symbol, "BTC-PERP");
    assert_eq!(order.side, expected_side);
    assert_eq!(order.execution, expected_execution);
    assert_eq!(order.time_in_force, expected_time_in_force);
    assert_eq!(order.qty, expected_qty);
    assert_eq!(order.reduce_only, expected_reduce_only);
    assert_eq!(order.filled_qty, 0);
    assert_eq!(order.version, 1);
    assert_eq!(order.reservation.is_some(), !expected_reduce_only);
}

fn event_field<'a>(
    event: &'a cmd_handler::EntityReplayableEvent,
    field_name: &str,
) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

fn assert_order_created_event(
    event: &cmd_handler::EntityReplayableEvent,
    expected_side: &str,
    expected_execution: &str,
    expected_time_in_force: &str,
    expected_qty: u64,
    expected_price: u64,
    expected_reduce_only: bool,
) {
    assert_eq!(event.change_type, 0);
    assert_eq!(event.entity_type, <HyperliquidPerpOrder as Entity>::entity_type());
    assert_eq!(event_field(event, "order_id"), Some("acct-1-BTC-PERP-42"));
    assert_eq!(event_field(event, "side"), Some(expected_side));
    assert_eq!(event_field(event, "execution"), Some(expected_execution));
    assert_eq!(event_field(event, "time_in_force"), Some(expected_time_in_force));
    assert_eq!(event_field(event, "qty"), Some(expected_qty.to_string().as_str()));
    assert_eq!(event_field(event, "price"), Some(expected_price.to_string().as_str()));
    assert_eq!(event_field(event, "reduce_only"), Some(expected_reduce_only.to_string().as_str()));
}

fn assert_balance_updated_event(
    event: &cmd_handler::EntityReplayableEvent,
    expected_available: u64,
    expected_frozen: u64,
) {
    assert_eq!(event.change_type, 1);
    assert_eq!(event.entity_type, <Balance as Entity>::entity_type());
    assert_eq!(event_field(event, "available"), Some(expected_available.to_string().as_str()));
    assert_eq!(event_field(event, "frozen"), Some(expected_frozen.to_string().as_str()));
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
        // - 调用 `compute_after_changes()`，再投影 replayable events。
        //
        // Then:
        // - `Changes` 先记录 created order 和一条 balance before/after pair。
        // - 事件顺序必须是 order create 在前，balance update 在后。

        // arrange
        let cmd = PlaceHyperliquidPerpOrderCmd {
            execution: PlaceHyperliquidPerpOrderExecution::Limit { price: 101, time_in_force },
            ..limit_cmd()
        };
        assert_eq!(use_case().validate_against_given_state(&cmd, &state()), Ok(()));

        // act
        let (changes, events) = compute_after_changes_and_events(&cmd, state());

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
        assert_freeze_ledger_entry_matches_balance_pair(&changes);
        assert_eq!(events.len(), 3);
        assert_order_created_event(&events[0], "buy", "limit", expected_tif, 3, 101, false);
        assert_balance_ledger_created_event(&events[1]);
        assert_balance_updated_event(&events[2], 9_939, 561);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 保留 sell open order 和单条 balance pair。
    // - 事件顺序必须是 order create -> balance update。

    // arrange
    let cmd = sell_limit_cmd();
    assert_eq!(use_case().validate_against_given_state(&cmd, &state()), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, state());

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
    assert_freeze_ledger_entry_matches_balance_pair(&changes);
    assert_eq!(events.len(), 3);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, false);
    assert_balance_ledger_created_event(&events[1]);
    assert_balance_updated_event(&events[2], 9_939, 561);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 记录 market + ioc 订单和余额冻结 pair。
    // - 事件记录 execution=market、time_in_force=ioc、price=111。

    // arrange
    let cmd = market_cmd();
    assert_eq!(use_case().validate_against_given_state(&cmd, &state()), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, state());

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
    assert_freeze_ledger_entry_matches_balance_pair(&changes);
    assert_eq!(events.len(), 3);
    assert_order_created_event(&events[0], "buy", "market", "ioc", 3, 111, false);
    assert_balance_ledger_created_event(&events[1]);
    assert_balance_updated_event(&events[2], 9_933, 567);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 记录 sell market + ioc 订单和余额冻结 pair。
    // - 事件记录 sell / market / ioc / 111。

    // arrange
    let cmd = sell_market_cmd();
    assert_eq!(use_case().validate_against_given_state(&cmd, &state()), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, state());

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
    assert_freeze_ledger_entry_matches_balance_pair(&changes);
    assert_eq!(events.len(), 3);
    assert_order_created_event(&events[0], "sell", "market", "ioc", 3, 111, false);
    assert_balance_ledger_created_event(&events[1]);
    assert_balance_updated_event(&events[2], 9_933, 567);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
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
    assert_eq!(use_case().validate_against_given_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, long_state);

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
    assert_freeze_ledger_entry_matches_balance_pair(&changes);
    assert_eq!(events.len(), 3);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 3, 101, false);
    assert_balance_ledger_created_event(&events[1]);
    assert_balance_updated_event(&events[2], 9_939, 561);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
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
    assert_eq!(use_case().validate_against_given_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, short_state);

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
    assert_freeze_ledger_entry_matches_balance_pair(&changes);
    assert_eq!(events.len(), 3);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, false);
    assert_balance_ledger_created_event(&events[1]);
    assert_balance_updated_event(&events[2], 9_939, 561);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
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
    assert_eq!(use_case().validate_against_given_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        true,
    );
    assert_no_margin_ledger_or_balance_pair(&changes);
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
    // - 调用 `compute_after_changes()`，再投影 replayable events。
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
    assert_eq!(use_case().validate_against_given_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, short_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        true,
    );
    assert_no_margin_ledger_or_balance_pair(&changes);
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 3, 101, true);
}

#[test]
fn long_position_places_opposite_sell_that_closes_without_new_margin() {
    // Rule:
    // - 非 reduce-only 命令的 opposite-side sell，只要仍在 long 净仓内，就派生为 Close 订单。
    //
    // Given:
    // - 当前已有 5 手 long position。
    // - 命令是 3 手 sell limit，仍小于当前净仓。
    //
    // When:
    // - 调用 `compute_after_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 reduce-only created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let cmd = sell_limit_cmd();
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_given_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        true,
    );
    assert_no_margin_ledger_or_balance_pair(&changes);
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 3, 101, true);
}

#[test]
fn long_position_places_opposite_sell_that_exactly_closes_without_new_margin() {
    // Rule:
    // - 非 reduce-only 命令的 opposite-side sell 在数量刚好等于当前 long 净仓时，应派生为 Close 订单。
    //
    // Given:
    // - 当前已有 5 手 long position。
    // - 命令是 5 手 sell limit，刚好等于当前净仓。
    //
    // When:
    // - 调用 `compute_after_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 reduce-only created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let mut cmd = sell_limit_cmd();
    cmd.size = 5;
    cmd.reduce_only = false;
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_given_state(&cmd, &long_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, long_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Sell,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        5,
        true,
    );
    assert_no_margin_ledger_or_balance_pair(&changes);
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "sell", "limit", "gtc", 5, 101, true);
}

#[test]
fn long_position_rejects_single_opposite_sell_flip_until_split_workflow_handles_it() {
    // Rule:
    // - opposite-side sell 超过当前 long 净仓时，不能作为单笔普通订单表达反手。
    //
    // Given:
    // - 当前已有 5 手 long position。
    // - 命令是 8 手 sell limit，净新增 short 只有 3 手。
    //
    // When:
    // - 校验下单状态。
    //
    // Then:
    // - 返回明确错误，要求上层拆成 Close + Open。

    // arrange
    let mut cmd = sell_limit_cmd();
    cmd.size = 8;
    let long_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Long, 5),
        ..state()
    };
    assert_eq!(
        use_case().validate_against_given_state(&cmd, &long_state),
        Err(PlaceHyperliquidPerpOrderError::FlipOrderRequiresSplit)
    );
}

#[test]
fn short_position_places_opposite_buy_that_closes_without_new_margin() {
    // Rule:
    // - 非 reduce-only 命令的 opposite-side buy，只要仍在 short 净仓内，就派生为 Close 订单。
    //
    // Given:
    // - 当前已有 5 手 short position。
    // - 命令是 3 手 buy limit，仍小于当前净仓。
    //
    // When:
    // - 调用 `compute_after_changes()`，再投影 replayable events。
    //
    // Then:
    // - `Changes` 只有 reduce-only created order，没有 balance pair。
    // - 只产生一条 order create event。

    // arrange
    let cmd = limit_cmd();
    let short_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
        ..state()
    };
    assert_eq!(use_case().validate_against_given_state(&cmd, &short_state), Ok(()));

    // act
    let (changes, events) = compute_after_changes_and_events(&cmd, short_state);

    // assert
    assert_order_snapshot(
        &changes.created_order,
        HyperliquidPerpOrderSide::Buy,
        HyperliquidPerpOrderExecution::Limit { price: 101 },
        HyperliquidPerpOrderTimeInForce::Gtc,
        3,
        true,
    );
    assert_no_margin_ledger_or_balance_pair(&changes);
    assert_eq!(events.len(), 1);
    assert_order_created_event(&events[0], "buy", "limit", "gtc", 3, 101, true);
}

#[test]
fn short_position_rejects_single_opposite_buy_flip_until_split_workflow_handles_it() {
    // Rule:
    // - opposite-side buy 超过当前 short 净仓时，不能作为单笔普通订单表达反手。
    //
    // Given:
    // - 当前已有 5 手 short position。
    // - 命令是 8 手 buy limit，净新增 long 只有 3 手。
    //
    // When:
    // - 校验下单状态。
    //
    // Then:
    // - 返回明确错误，要求上层拆成 Close + Open。

    // arrange
    let mut cmd = limit_cmd();
    cmd.size = 8;
    let short_state = PlaceHyperliquidPerpOrderState {
        position: non_flat_position(HyperliquidPerpPositionSide::Short, 5),
        ..state()
    };
    assert_eq!(
        use_case().validate_against_given_state(&cmd, &short_state),
        Err(PlaceHyperliquidPerpOrderError::FlipOrderRequiresSplit)
    );
}
