use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

use super::*;
use crate::entity::SpotOrderStatus;

// 目的:
// - 把 `ExecuteMatchedSpotTradeUseCase::compute_changes` 的 happy path 写成业务规格测试。
// - 重点保护复合用例的阶段串联、typed changes 语义和事件顺序。
//
// 规格矩阵:
// - match outcome: full fill / partial fill / zero trade
// - settlement outcome: settled / skipped
// - event expectation: match events first / clearing truth events appended / no clearing events when no trade
//
// current coverage:
// - 单笔完全成交后立即清结算
// - taker 部分成交后立即清结算
// - 无成交时只返回 match changes，不进入 settlement

fn compute_changes(
    cmd: &ExecuteMatchedSpotTradeCmd,
    state: ExecuteMatchedSpotTradeState,
) -> Result<ExecuteMatchedSpotTradeChanges, ExecuteMatchedSpotTradeError> {
    CommandUseCase4::compute_changes(&ExecuteMatchedSpotTradeUseCase, cmd, state)
}

#[test]
fn full_fill_matches_and_settles_in_single_business_result()
-> Result<(), ExecuteMatchedSpotTradeError> {
    // Rule:
    // - 当 taker 被 maker 完全吃掉时，复合用例必须同时返回撮合结果和清结算结果。
    //
    // Given:
    // - buy taker 与单个 sell maker 完全成交。
    // - 双方余额都足以完成清结算。
    //
    // When:
    // - 调用 `compute_changes`。
    //
    // Then:
    // - `match_changes` 里有 1 笔 trade，taker 变成 `Filled`。
    // - `settle_changes` 存在，且只暴露 reservation / balance / ledger 真相。
    // - 投影事件顺序必须先 trade/match update，再 reservation/balance/ledger。

    // arrange
    let state = execute_state(
        taker_buy_limit(2, 100),
        vec![maker_sell("maker-1", "seller-1", 2, 100)],
        vec![
            balance("buyer", "BTC", 0, 0),
            balance("buyer", "USDT", 0, 200),
            balance("seller-1", "USDT", 0, 0),
            balance("seller-1", "BTC", 0, 2),
        ],
    );

    // act
    let changes = compute_changes(&execute_cmd(), state)?;
    let events = changes.to_replayable_events().map_err(|_| {
        ExecuteMatchedSpotTradeError::Settle(SettleSpotTradeError::ArithmeticOverflow)
    })?;

    // assert
    assert_eq!(changes.match_changes.trades.len(), 1);
    assert_eq!(changes.match_changes.updated_maker_orders.len(), 1);
    assert_eq!(changes.match_changes.updated_taker_order.after.status, SpotOrderStatus::Filled);
    let settle_changes = changes.settle_changes.as_ref().expect("expected settlement changes");
    assert_eq!(settle_changes.updated_reservations.len(), 2);
    assert_eq!(settle_changes.created_reservation_consumed.len(), 2);
    assert_eq!(settle_changes.updated_balances.len(), 4);
    assert_eq!(settle_changes.created_balance_ledger_entries.len(), 4);

    assert_eq!(events.len(), 15);
    assert_trade_event(&events[0], "match-1-1", "maker-1", 100, 2);
    assert_eq!(event_field(&events[3], "reservation_id"), Some("reservation:taker-1"));
    assert_eq!(event_field(&events[7], "account_id"), Some("buyer"));
    assert_eq!(event_field(&events[11], "reason"), Some("settle_spot_trade_buyer_receive_base"));
    assert!(events.iter().all(|event| event_field(event, "settlement_id").is_none()));

    Ok(())
}

#[test]
fn partial_fill_settles_only_the_matched_quantity() -> Result<(), ExecuteMatchedSpotTradeError> {
    // Rule:
    // - 当 taker 只部分成交时，复合用例只清结算已成交部分，并保留 taker 的部分成交状态。
    //
    // Given:
    // - buy taker 数量大于唯一可成交 maker 数量。
    // - 余额只覆盖已成交数量对应的交割。
    //
    // When:
    // - 调用 `compute_changes`。
    //
    // Then:
    // - `match_changes` 里只有 1 笔部分成交 trade。
    // - taker 状态是 `PartiallyFilled`。
    // - reservation consume / ledger 数量只对应已撮合的 1 手。

    // arrange
    let state = execute_state(
        taker_buy_limit(3, 100),
        vec![maker_sell("maker-1", "seller-1", 1, 100)],
        vec![
            balance("buyer", "BTC", 0, 0),
            balance("buyer", "USDT", 0, 100),
            balance("seller-1", "USDT", 0, 0),
            balance("seller-1", "BTC", 0, 1),
        ],
    );

    // act
    let changes = compute_changes(&execute_cmd(), state)?;
    let events = changes.to_replayable_events().map_err(|_| {
        ExecuteMatchedSpotTradeError::Settle(SettleSpotTradeError::ArithmeticOverflow)
    })?;

    // assert
    assert_eq!(changes.match_changes.trades.len(), 1);
    assert_eq!(
        changes.match_changes.updated_taker_order.after.status,
        SpotOrderStatus::PartiallyFilled
    );
    let settle_changes = changes.settle_changes.as_ref().expect("expected settlement changes");
    assert_eq!(settle_changes.updated_reservations.len(), 2);
    assert_eq!(settle_changes.created_reservation_consumed.len(), 2);
    assert_eq!(events.len(), 15);
    assert_trade_event(&events[0], "match-1-1", "maker-1", 100, 1);
    assert_eq!(event_field(&events[3], "reservation_id"), Some("reservation:taker-1"));
    assert_eq!(event_field(&events[11], "reason_settlement_ids"), Some("settle-1-1"));
    assert!(events.iter().all(|event| event_field(event, "settlement_id").is_none()));

    Ok(())
}

#[test]
fn alo_pre_reject_skips_settlement_and_only_projects_match_events()
-> Result<(), ExecuteMatchedSpotTradeError> {
    // Rule:
    // - 当 ALO taker 会立刻成交时，复合用例必须停在撮合阶段，不生成 settlement。
    //
    // Given:
    // - buy taker 是 ALO。
    // - 最优 maker 与 taker 价格交叉，因此 taker 会被预拒绝。
    //
    // When:
    // - 调用 `compute_changes`。
    //
    // Then:
    // - `match_changes.trades` 为空。
    // - `settle_changes` 为 `None`。
    // - 事件里只包含 taker 的 ALO 拒绝更新。

    // arrange
    let state = execute_state(
        taker_buy_alo(3, 100),
        vec![maker_sell("maker-1", "seller-1", 1, 99)],
        vec![],
    );

    // act
    let changes = compute_changes(&execute_cmd(), state)?;
    let events = changes.to_replayable_events().map_err(|_| {
        ExecuteMatchedSpotTradeError::Settle(SettleSpotTradeError::ArithmeticOverflow)
    })?;

    // assert
    assert!(changes.match_changes.trades.is_empty());
    assert!(changes.settle_changes.is_none());
    assert_eq!(changes.match_changes.updated_taker_order.after.status, SpotOrderStatus::Rejected);
    assert_eq!(events.len(), 1);
    assert_eq!(event_field(&events[0], "status"), Some("rejected"));
    assert_eq!(event_field(&events[0], "settlement_id"), None);

    Ok(())
}
