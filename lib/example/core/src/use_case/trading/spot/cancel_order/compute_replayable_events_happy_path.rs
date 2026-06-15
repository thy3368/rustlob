use cmd_handler::EntityReplayableEvent;

use super::*;
use crate::entity::{
    Balance, SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
    SpotOrderTimeInForce,
};

// 目的:
// - 把 `CancelSpotOrderUseCase::compute_replayable_events` 的成功业务语义写成规格测试。
// - 重点保护撤单后删除订单、释放剩余冻结余额和事件顺序。
//
// 适用范围:
// - 这里只覆盖 use case 层 happy path。
// - `pre_check_command`、`validate_against_state` 和 overflow 场景留在内联测试与场景测试。
//
// 规格矩阵:
// - order side: buy / sell
// - cancelable status: open / partially filled
// - release target: quote balance / base balance
// - event expectation: order cancel update / single balance update / event order
//
// current coverage:
// - buy open 撤单释放 quote
// - sell open 撤单释放 base
// - partially filled 撤单只释放剩余 reservation
// - 未受影响的另一条余额不产生 update event

fn compute_events(
    cmd: &CancelSpotOrderCmd,
    state: CancelSpotOrderState,
) -> Result<Vec<EntityReplayableEvent>, CancelSpotOrderError> {
    CancelSpotOrderUseCase.compute_replayable_events(cmd, state)
}

fn cmd() -> CancelSpotOrderCmd {
    CancelSpotOrderCmd { party_id: "trader-1".to_string(), asset: 10_001, order_id: 42 }
}

fn buy_open_order() -> SpotOrder {
    SpotOrder::new(
        "42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: 10 },
        SpotOrderTimeInForce::Gtc,
        2,
        0,
        20,
        None,
    )
}

fn sell_open_order() -> SpotOrder {
    SpotOrder::new(
        "42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Sell,
        SpotOrderExecution::Limit { price: 10 },
        SpotOrderTimeInForce::Gtc,
        2,
        2,
        0,
        None,
    )
}

fn partially_filled_buy_order() -> SpotOrder {
    SpotOrder::new(
        "42".to_string(),
        10_001,
        Some(42),
        "trader-1".to_string(),
        "BTCUSDT".to_string(),
        SpotOrderSide::Buy,
        SpotOrderExecution::Limit { price: 10 },
        SpotOrderTimeInForce::Gtc,
        3,
        0,
        11,
        None,
    )
    .with_execution_state(SpotOrderStatus::PartiallyFilled, 1)
}

fn base_balance(available: u64, frozen: u64) -> Balance {
    Balance {
        account_id: "trader-1".to_string(),
        asset_id: "BTC".to_string(),
        available,
        frozen,
        version: 3,
    }
}

fn quote_balance(available: u64, frozen: u64) -> Balance {
    Balance {
        account_id: "trader-1".to_string(),
        asset_id: "USDT".to_string(),
        available,
        frozen,
        version: 3,
    }
}

fn state(
    open_order: SpotOrder,
    base_balance: Balance,
    quote_balance: Balance,
) -> CancelSpotOrderState {
    CancelSpotOrderState {
        open_order: Some(open_order),
        account_id: "trader-1".to_string(),
        base_balance,
        quote_balance,
    }
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
    event_field(event, field_name)?.parse().ok()
}

fn assert_order_deleted_event(
    event: &EntityReplayableEvent,
    expected_status: SpotOrderStatus,
    expected_status_reason: SpotOrderStatusReason,
    expected_old_version: u64,
    expected_new_version: u64,
) {
    assert!(event.is_updated());
    assert_eq!(event.old_version, expected_old_version);
    assert_eq!(event.new_version, expected_new_version);
    assert_eq!(event_field(event, "status"), Some(expected_status.as_str()));
    assert_eq!(event_field(event, "status_reason"), Some(expected_status_reason.as_str()));
}

fn assert_balance_update_event(
    event: &EntityReplayableEvent,
    expected_account_id: &str,
    expected_asset_id: &str,
    expected_available: u64,
    expected_frozen: u64,
    expected_old_version: u64,
    expected_new_version: u64,
) {
    assert!(event.is_updated());
    assert_eq!(event.old_version, expected_old_version);
    assert_eq!(event.new_version, expected_new_version);
    assert_eq!(event_field(event, "account_id"), Some(expected_account_id));
    assert_eq!(event_field(event, "asset_id"), Some(expected_asset_id));
    assert_eq!(event_field_u64(event, "available"), Some(expected_available));
    assert_eq!(event_field_u64(event, "frozen"), Some(expected_frozen));
}

#[test]
fn buy_order_cancel_marks_order_canceled_and_releases_quote() -> Result<(), CancelSpotOrderError> {
    // Rule:
    // - 买单撤单时，应把订单更新为用户取消，并释放冻结 quote。
    //
    // Given:
    // - 1 个仍可撤销的 buy open order。
    // - quote balance 的 frozen 正好等于该订单剩余 reservation。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 先产生 1 条 order cancel update event。
    // - 再产生 1 条 USDT balance update event。
    // - 可用余额增加，冻结余额归零。

    // arrange
    let state = state(buy_open_order(), base_balance(5, 0), quote_balance(80, 20));

    // act
    let events = compute_events(&cmd(), state)?;

    // assert
    assert_eq!(events.len(), 2);
    assert_order_deleted_event(
        &events[0],
        SpotOrderStatus::Canceled,
        SpotOrderStatusReason::CanceledByUser,
        1,
        2,
    );
    assert_balance_update_event(&events[1], "trader-1", "USDT", 100, 0, 3, 4);

    Ok(())
}

#[test]
fn sell_order_cancel_marks_order_canceled_and_releases_base() -> Result<(), CancelSpotOrderError> {
    // Rule:
    // - 卖单撤单时，应把订单更新为用户取消，并释放冻结 base。
    //
    // Given:
    // - 1 个仍可撤销的 sell open order。
    // - base balance 的 frozen 正好等于该订单剩余 reservation。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 先产生 1 条 order cancel update event。
    // - 再产生 1 条 BTC balance update event。
    // - 可用余额增加，冻结余额归零。

    // arrange
    let state = state(sell_open_order(), base_balance(5, 2), quote_balance(80, 0));

    // act
    let events = compute_events(&cmd(), state)?;

    // assert
    assert_eq!(events.len(), 2);
    assert_order_deleted_event(
        &events[0],
        SpotOrderStatus::Canceled,
        SpotOrderStatusReason::CanceledByUser,
        1,
        2,
    );
    assert_balance_update_event(&events[1], "trader-1", "BTC", 7, 0, 3, 4);

    Ok(())
}

#[test]
fn partially_filled_buy_cancel_marks_order_canceled_and_releases_only_remaining_quote_reservation()
-> Result<(), CancelSpotOrderError> {
    // Rule:
    // - 部分成交订单撤单时，只释放剩余冻结量，不回滚已成交部分。
    //
    // Given:
    // - 1 个仍可撤销的 partially filled buy order。
    // - 剩余 reservation quote 是 11。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 订单取消更新事件仍先发生。
    // - 余额事件只释放剩余 11 个 quote。

    // arrange
    let state = state(partially_filled_buy_order(), base_balance(5, 0), quote_balance(80, 11));

    // act
    let events = compute_events(&cmd(), state)?;

    // assert
    assert_eq!(events.len(), 2);
    assert_order_deleted_event(
        &events[0],
        SpotOrderStatus::Canceled,
        SpotOrderStatusReason::CanceledByUser,
        1,
        2,
    );
    assert_balance_update_event(&events[1], "trader-1", "USDT", 91, 0, 3, 4);

    Ok(())
}
