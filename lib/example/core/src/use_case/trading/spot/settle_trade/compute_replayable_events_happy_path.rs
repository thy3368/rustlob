use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

use super::*;

// 目的:
// - 把 `SettleSpotTradeUseCase::compute_replayable_events` 的成功业务语义写成规格测试。
// - 重点保护 settlement 事实、余额聚合、参与方推导和事件顺序。
//
// 适用范围:
// - 这里只覆盖 use case 层 happy path。
// - `pre_check_command`、`validate_against_state`、overflow/reject 场景留在内联测试。
//
// 规格矩阵:
// - trade batch: single / multi-trade aggregate
// - taker side: buy / sell
// - balance impact: buyer aggregate / per-seller update / unaffected untouched
// - event expectation: settlement create count / balance update count / ledger create count / event order
//
// current coverage:
// - 单笔 buy taker 清结算，释放 buyer quote 并交付 base
// - 单笔 sell taker 清结算，从 maker 推导 buyer
// - 多笔批量清结算，buyer 聚合、seller 分别更新
// - 未受影响余额不会产生 update event
//
// 断言规范:
// - settlement event 必须断言业务身份字段和数量字段。
// - balance update event 必须断言 account / asset / 可用或冻结变化 / version。
// - 多事件场景必须断言顺序：先全部 settlement create，再全部 balance update，最后全部 ledger create。

fn compute_events(
    cmd: &SettleSpotTradeCmd,
    state: SettleSpotTradeState,
) -> Result<Vec<cmd_handler::EntityReplayableEvent>, SettleSpotTradeError> {
    Ok(CommandUseCase4::compute_changes(&SettleSpotTradeUseCase, cmd, state)?
        .to_replayable_events()
        .map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?)
}

#[test]
fn single_buy_taker_trade_releases_buyer_quote_and_delivers_base()
-> Result<(), SettleSpotTradeError> {
    // Rule:
    // - 当 buy taker 成交被清结算时，买方收到 base，买方冻结 quote 被释放，
    //   卖方收到 quote，卖方冻结 base 被释放。
    //
    // Given:
    // - 只有 1 笔买方为 taker 的 trade。
    // - 4 条余额都足以支撑交割。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 先产生 1 条 settlement create event。
    // - 再按余额输入顺序产生 4 条 balance update event。
    // - 最后产生 4 条 balance ledger create event。
    // - 每条余额只表达本次真实变化的字段。

    // arrange
    let state = state(
        vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
        vec![
            balance("buyer", "BTC", 0, 0),
            balance("buyer", "USDT", 0, 200),
            balance("seller", "USDT", 0, 0),
            balance("seller", "BTC", 0, 2),
        ],
    );

    // act
    let events = compute_events(&cmd(vec!["trade-1"]), state)?;

    // assert
    assert_eq!(events.len(), 13);
    assert_settlement_event(&events[0], "settle-1-1", "trade-1", "buyer", "seller", 2, 200, 100);
    assert_eq!(event_field(&events[1], "reservation_id"), Some("reservation:trade-1-taker"));
    assert_eq!(event_field(&events[2], "reservation_id"), Some("reservation:trade-1-maker"));
    assert_eq!(event_field(&events[3], "caused_by_ref_id"), Some("trade-1"));
    assert_eq!(event_field(&events[4], "caused_by_ref_id"), Some("trade-1"));
    assert_balance_update_event(&events[5], "buyer", "BTC", Some(2), None, 3, 4);
    assert_balance_update_event(&events[6], "buyer", "USDT", None, Some(0), 3, 4);
    assert_balance_update_event(&events[7], "seller", "USDT", Some(200), None, 3, 4);
    assert_balance_update_event(&events[8], "seller", "BTC", None, Some(0), 3, 4);
    assert_balance_ledger_event(
        &events[9],
        "buyer",
        "BTC",
        "settle_spot_trade_buyer_receive_base",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[10],
        "buyer",
        "USDT",
        "settle_spot_trade_buyer_release_frozen_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[11],
        "seller",
        "USDT",
        "settle_spot_trade_seller_receive_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[12],
        "seller",
        "BTC",
        "settle_spot_trade_seller_release_frozen_base",
        "trade-1",
        "settle-1-1",
    );

    Ok(())
}

#[test]
fn single_sell_taker_trade_derives_buyer_from_maker_and_delivers_quote()
-> Result<(), SettleSpotTradeError> {
    // Rule:
    // - 当 sell taker 成交被清结算时，buyer 必须从 maker 一侧推导，而不是默认 taker。
    //
    // Given:
    // - 只有 1 笔卖方为 taker 的 trade。
    // - maker 是 buyer，taker 是 seller。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - settlement 事件里的 buyer / seller 账户方向必须翻转正确。
    // - buyer 的 quote frozen 被释放，seller 的 quote available 增加。

    // arrange
    let state = state(
        vec![trade("trade-1", SpotOrderSide::Sell, "seller", "buyer", 100, 2)],
        vec![
            balance("buyer", "BTC", 0, 0),
            balance("buyer", "USDT", 0, 200),
            balance("seller", "USDT", 0, 0),
            balance("seller", "BTC", 0, 2),
        ],
    );

    // act
    let events = compute_events(&cmd(vec!["trade-1"]), state)?;

    // assert
    assert_eq!(events.len(), 13);
    assert_settlement_event(&events[0], "settle-1-1", "trade-1", "buyer", "seller", 2, 200, 100);
    assert_eq!(event_field(&events[1], "reservation_id"), Some("reservation:trade-1-maker"));
    assert_eq!(event_field(&events[2], "reservation_id"), Some("reservation:trade-1-taker"));
    assert_balance_update_event(&events[5], "buyer", "BTC", Some(2), None, 3, 4);
    assert_balance_update_event(&events[6], "buyer", "USDT", None, Some(0), 3, 4);
    assert_balance_update_event(&events[7], "seller", "USDT", Some(200), None, 3, 4);
    assert_balance_update_event(&events[8], "seller", "BTC", None, Some(0), 3, 4);
    assert_balance_ledger_event(
        &events[9],
        "buyer",
        "BTC",
        "settle_spot_trade_buyer_receive_base",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[10],
        "buyer",
        "USDT",
        "settle_spot_trade_buyer_release_frozen_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[11],
        "seller",
        "USDT",
        "settle_spot_trade_seller_receive_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[12],
        "seller",
        "BTC",
        "settle_spot_trade_seller_release_frozen_base",
        "trade-1",
        "settle-1-1",
    );

    Ok(())
}

#[test]
fn batch_settlement_aggregates_buyer_deltas_and_updates_each_seller_once()
-> Result<(), SettleSpotTradeError> {
    // Rule:
    // - 同一批次多笔 trade 清结算时，buyer 同资产余额应按 trade 聚合，
    //   而每个 seller 仍按各自资产余额分别更新一次。
    //
    // Given:
    // - buyer 作为 taker 连续买入两笔，分别来自 seller-1 和 seller-2。
    // - buyer 的 BTC / USDT 余额各只有 1 条。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 先产生 2 条 settlement create event。
    // - buyer BTC 只更新 1 次到聚合后的 3。
    // - buyer USDT 只更新 1 次并释放聚合后的 290。
    // - 每个 seller 的 BTC / USDT 各更新 1 次。
    // - buyer 两条 ledger 要聚合记录两个 trade / settlement 引用。

    // arrange
    let state = state(
        vec![
            trade("trade-1", SpotOrderSide::Buy, "buyer", "seller-1", 100, 2),
            trade("trade-2", SpotOrderSide::Buy, "buyer", "seller-2", 90, 1),
        ],
        vec![
            balance("buyer", "BTC", 0, 0),
            balance("buyer", "USDT", 0, 290),
            balance("seller-1", "USDT", 0, 0),
            balance("seller-1", "BTC", 0, 2),
            balance("seller-2", "USDT", 0, 0),
            balance("seller-2", "BTC", 0, 1),
        ],
    );

    // act
    let events = compute_events(&cmd(vec!["trade-1", "trade-2"]), state)?;

    // assert
    assert_eq!(events.len(), 22);
    assert_settlement_event(&events[0], "settle-1-1", "trade-1", "buyer", "seller-1", 2, 200, 100);
    assert_settlement_event(&events[1], "settle-1-2", "trade-2", "buyer", "seller-2", 1, 90, 90);
    assert_balance_update_event(&events[10], "buyer", "BTC", Some(3), None, 3, 4);
    assert_balance_update_event(&events[11], "buyer", "USDT", None, Some(0), 3, 4);
    assert_balance_update_event(&events[12], "seller-1", "USDT", Some(200), None, 3, 4);
    assert_balance_update_event(&events[13], "seller-1", "BTC", None, Some(0), 3, 4);
    assert_balance_update_event(&events[14], "seller-2", "USDT", Some(90), None, 3, 4);
    assert_balance_update_event(&events[15], "seller-2", "BTC", None, Some(0), 3, 4);
    assert_balance_ledger_event(
        &events[16],
        "buyer",
        "BTC",
        "settle_spot_trade_buyer_receive_base",
        "trade-1,trade-2",
        "settle-1-1,settle-1-2",
    );
    assert_balance_ledger_event(
        &events[17],
        "buyer",
        "USDT",
        "settle_spot_trade_buyer_release_frozen_quote",
        "trade-1,trade-2",
        "settle-1-1,settle-1-2",
    );
    assert_balance_ledger_event(
        &events[18],
        "seller-1",
        "USDT",
        "settle_spot_trade_seller_receive_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[19],
        "seller-1",
        "BTC",
        "settle_spot_trade_seller_release_frozen_base",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[20],
        "seller-2",
        "USDT",
        "settle_spot_trade_seller_receive_quote",
        "trade-2",
        "settle-1-2",
    );
    assert_balance_ledger_event(
        &events[21],
        "seller-2",
        "BTC",
        "settle_spot_trade_seller_release_frozen_base",
        "trade-2",
        "settle-1-2",
    );

    Ok(())
}

#[test]
fn settlement_emits_only_affected_balance_updates() -> Result<(), SettleSpotTradeError> {
    // Rule:
    // - `compute_replayable_events` 只应为真正受交割影响的余额生成 update event。
    //
    // Given:
    // - 清结算仍只涉及 buyer / seller 在 BTC 和 USDT 上的 4 条余额。
    // - 额外加载 1 条未参与交割的 buyer ETH 余额。
    //
    // When:
    // - 调用 `compute_replayable_events`。
    //
    // Then:
    // - 不应出现 buyer ETH 的 update event。
    // - 总事件数仍等于 1 条 settlement + 4 条受影响余额 update + 4 条 ledger create。

    // arrange
    let state = state(
        vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
        vec![
            balance("buyer", "BTC", 0, 0),
            balance("buyer", "USDT", 0, 200),
            balance("buyer", "ETH", 5, 0),
            balance("seller", "USDT", 0, 0),
            balance("seller", "BTC", 0, 2),
        ],
    );

    // act
    let events = compute_events(&cmd(vec!["trade-1"]), state)?;

    // assert
    assert_eq!(events.len(), 13);
    assert!(balance_event(&events, "buyer", "ETH").is_none());
    assert!(ledger_event(&events, "buyer", "ETH").is_none());
    assert_settlement_event(&events[0], "settle-1-1", "trade-1", "buyer", "seller", 2, 200, 100);
    assert_balance_update_event(&events[5], "buyer", "BTC", Some(2), None, 3, 4);
    assert_balance_update_event(&events[6], "buyer", "USDT", None, Some(0), 3, 4);
    assert_balance_update_event(&events[7], "seller", "USDT", Some(200), None, 3, 4);
    assert_balance_update_event(&events[8], "seller", "BTC", None, Some(0), 3, 4);
    assert_balance_ledger_event(
        &events[9],
        "buyer",
        "BTC",
        "settle_spot_trade_buyer_receive_base",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[10],
        "buyer",
        "USDT",
        "settle_spot_trade_buyer_release_frozen_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[11],
        "seller",
        "USDT",
        "settle_spot_trade_seller_receive_quote",
        "trade-1",
        "settle-1-1",
    );
    assert_balance_ledger_event(
        &events[12],
        "seller",
        "BTC",
        "settle_spot_trade_seller_release_frozen_base",
        "trade-1",
        "settle-1-1",
    );

    Ok(())
}
