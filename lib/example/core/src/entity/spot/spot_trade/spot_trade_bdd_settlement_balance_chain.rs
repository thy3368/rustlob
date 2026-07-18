use cmd_handler::command_use_case_def2::UpdatedEntityPair;

use super::*;
use crate::entity::{
    AssetReservation, Balance, BalanceLedgerEntryV2, BalanceLedgerReason, Reservation,
    ReservationCloseReason, ReservationConsumed, ReservationKind, ReservationMarketKind,
    ReservationStatus, SettlementKind, SettlementTransferPurpose, SettlementTransferVoucher,
};

// 目的:
// - 把 `SpotTrade -> ReservationConsumed -> SettlementTransferVoucher
//   -> BalanceLedgerEntryV2 -> Balance snapshot updated` 的成功业务语义写成规格测试。
// - 测试只在 BDD 内部编排实体方法，不调用 settlement use case。
// - `Balance` 只表达流水落账后的状态快照/投影结果，不作为 MI 终局事实。
//
// 规格矩阵:
// - taker side: buy / sell
// - trade batch: single / multi-trade
// - settlement facts: reservation consumed / principal voucher / balance ledger / balance after
//
// current coverage:
// - buy taker 成交消耗买方 quote 与卖方 base 冻结，并更新 4 条余额。
// - sell taker 成交先从 `SpotTrade` 推导 buyer / seller，再验证余额方向。
// - 批量成交每笔生成一张 principal voucher，余额结果累计到最终快照。

#[test]
fn buy_taker_trade_consumes_frozen_reservations_and_updates_balances() -> Result<(), String> {
    // Rule:
    // - buy taker 成交事实驱动 principal 清结算时，买方收到 base，
    //   买方冻结 quote 被消耗，卖方收到 quote，卖方冻结 base 被消耗。
    //
    // Given:
    // - 一笔买方为 taker 的现货成交。
    // - 买方 quote reservation 与卖方 base reservation 都覆盖成交数量。
    //
    // When:
    // - BDD 直接按实体方法构造 reservation consumed、principal voucher、ledger 和 balance after。
    //
    // Then:
    // - reservation consumed 的 cause 指向 trade id。
    // - voucher 指向同一 trade，并包含 4 条 principal legs。
    // - 4 条 ledger 与各自 balance before/after 一致。
    // - 最终余额反映买方收 base、买方扣 frozen quote、卖方收 quote、卖方扣 frozen base。

    // arrange
    let trade = trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2);
    let balances = vec![
        balance("buyer", "BTC", 0, 0),
        balance("buyer", "USDT", 0, 200),
        balance("seller", "USDT", 0, 0),
        balance("seller", "BTC", 0, 2),
    ];

    // act
    let chain = settle_principal_chain(vec![trade.clone()], balances)?;

    // assert
    assert_eq!(chain.consumed_reservations.len(), 2);
    assert_eq!(chain.consumed_reservation_updates.len(), 2);
    assert_consumed_pair(
        &chain.consumed_reservation_updates[0],
        &chain.consumed_reservations[0],
        "reservation:trade-1-taker",
        "buyer",
        "USDT",
        200,
        "trade-1",
    );
    assert_consumed_pair(
        &chain.consumed_reservation_updates[1],
        &chain.consumed_reservations[1],
        "reservation:trade-1-maker",
        "seller",
        "BTC",
        2,
        "trade-1",
    );

    assert_eq!(chain.vouchers.len(), 1);
    assert_spot_principal_voucher(&chain.vouchers[0], &trade, "settlement:trade-1");

    assert_eq!(chain.ledger_entries.len(), 4);
    assert_voucher_references_ledgers(&chain.vouchers[0], &chain.ledger_entries[0..4]);
    assert_ledger_matches_update(&chain.ledger_entries[0], &chain.balance_updates[0]);
    assert_ledger_matches_update(&chain.ledger_entries[1], &chain.balance_updates[1]);
    assert_ledger_matches_update(&chain.ledger_entries[2], &chain.balance_updates[2]);
    assert_ledger_matches_update(&chain.ledger_entries[3], &chain.balance_updates[3]);
    assert_ledger_reason_refs(&chain.ledger_entries[0], "trade-1", "settlement:trade-1");
    assert_ledger_reason_refs(&chain.ledger_entries[1], "trade-1", "settlement:trade-1");
    assert_ledger_reason_refs(&chain.ledger_entries[2], "trade-1", "settlement:trade-1");
    assert_ledger_reason_refs(&chain.ledger_entries[3], "trade-1", "settlement:trade-1");

    assert_balance(&chain.final_balances, "buyer", "BTC", 2, 0, 4);
    assert_balance(&chain.final_balances, "buyer", "USDT", 0, 0, 4);
    assert_balance(&chain.final_balances, "seller", "USDT", 200, 0, 4);
    assert_balance(&chain.final_balances, "seller", "BTC", 0, 0, 4);

    Ok(())
}

#[test]
fn sell_taker_trade_derives_buyer_seller_before_settlement_balance_changes() -> Result<(), String> {
    // Rule:
    // - sell taker 成交清结算时，buyer 必须从 maker 一侧推导，
    //   seller 必须从 taker 一侧推导。
    //
    // Given:
    // - taker account 是 seller，maker account 是 buyer。
    //
    // When:
    // - BDD 直接从 `SpotTrade` 的 buyer/seller 查询构造后续结算事实。
    //
    // Then:
    // - reservation、voucher、ledger 和 balance after 都使用推导后的 buyer/seller。

    // arrange
    let trade = trade("trade-1", SpotOrderSide::Sell, "seller", "buyer", 100, 2);
    let balances = vec![
        balance("buyer", "BTC", 0, 0),
        balance("buyer", "USDT", 0, 200),
        balance("seller", "USDT", 0, 0),
        balance("seller", "BTC", 0, 2),
    ];

    // act
    let chain = settle_principal_chain(vec![trade.clone()], balances)?;

    // assert
    assert_eq!(trade.buyer_account_id(), "buyer");
    assert_eq!(trade.seller_account_id(), "seller");
    assert_eq!(chain.consumed_reservation_updates.len(), 2);
    assert_consumed_pair(
        &chain.consumed_reservation_updates[0],
        &chain.consumed_reservations[0],
        "reservation:trade-1-maker",
        "buyer",
        "USDT",
        200,
        "trade-1",
    );
    assert_consumed_pair(
        &chain.consumed_reservation_updates[1],
        &chain.consumed_reservations[1],
        "reservation:trade-1-taker",
        "seller",
        "BTC",
        2,
        "trade-1",
    );
    assert_spot_principal_voucher(&chain.vouchers[0], &trade, "settlement:trade-1");
    assert_eq!(chain.ledger_entries.len(), 4);
    assert_voucher_references_ledgers(&chain.vouchers[0], &chain.ledger_entries[0..4]);
    assert_balance(&chain.final_balances, "buyer", "BTC", 2, 0, 4);
    assert_balance(&chain.final_balances, "buyer", "USDT", 0, 0, 4);
    assert_balance(&chain.final_balances, "seller", "USDT", 200, 0, 4);
    assert_balance(&chain.final_balances, "seller", "BTC", 0, 0, 4);

    Ok(())
}

#[test]
fn batch_trades_create_one_voucher_per_trade_and_aggregate_balance_effects() -> Result<(), String> {
    // Rule:
    // - 批量成交逐笔形成 principal voucher，余额快照按多笔成交的净业务结果累计。
    //
    // Given:
    // - buyer 连续买入两笔，分别来自 seller-1 与 seller-2。
    //
    // When:
    // - BDD 逐笔驱动 reservation consumed、voucher、ledger，并把 ledger 应用到同一组余额。
    //
    // Then:
    // - 每笔 trade 各有 1 张 voucher。
    // - 每笔 trade 各消耗买方 quote 与对应卖方 base reservation。
    // - buyer 最终收到两笔 base 合计，quote frozen 被两笔成交合计扣减。

    // arrange
    let trade_1 = trade("trade-1", SpotOrderSide::Buy, "buyer", "seller-1", 100, 2);
    let trade_2 = trade("trade-2", SpotOrderSide::Buy, "buyer", "seller-2", 90, 1);
    let balances = vec![
        balance("buyer", "BTC", 0, 0),
        balance("buyer", "USDT", 0, 290),
        balance("seller-1", "USDT", 0, 0),
        balance("seller-1", "BTC", 0, 2),
        balance("seller-2", "USDT", 0, 0),
        balance("seller-2", "BTC", 0, 1),
    ];

    // act
    let chain = settle_principal_chain(vec![trade_1.clone(), trade_2.clone()], balances)?;

    // assert
    assert_eq!(chain.vouchers.len(), 2);
    assert_spot_principal_voucher(&chain.vouchers[0], &trade_1, "settlement:trade-1");
    assert_spot_principal_voucher(&chain.vouchers[1], &trade_2, "settlement:trade-2");
    assert_eq!(chain.consumed_reservations.len(), 4);
    assert_eq!(chain.consumed_reservation_updates.len(), 4);
    assert_consumed_pair(
        &chain.consumed_reservation_updates[0],
        &chain.consumed_reservations[0],
        "reservation:trade-1-taker",
        "buyer",
        "USDT",
        200,
        "trade-1",
    );
    assert_consumed_pair(
        &chain.consumed_reservation_updates[1],
        &chain.consumed_reservations[1],
        "reservation:trade-1-maker",
        "seller-1",
        "BTC",
        2,
        "trade-1",
    );
    assert_consumed_pair(
        &chain.consumed_reservation_updates[2],
        &chain.consumed_reservations[2],
        "reservation:trade-2-taker",
        "buyer",
        "USDT",
        90,
        "trade-2",
    );
    assert_consumed_pair(
        &chain.consumed_reservation_updates[3],
        &chain.consumed_reservations[3],
        "reservation:trade-2-maker",
        "seller-2",
        "BTC",
        1,
        "trade-2",
    );
    assert_eq!(chain.ledger_entries.len(), 8);
    assert_voucher_references_ledgers(&chain.vouchers[0], &chain.ledger_entries[0..4]);
    assert_voucher_references_ledgers(&chain.vouchers[1], &chain.ledger_entries[4..8]);

    assert_balance(&chain.final_balances, "buyer", "BTC", 3, 0, 5);
    assert_balance(&chain.final_balances, "buyer", "USDT", 0, 0, 5);
    assert_balance(&chain.final_balances, "seller-1", "USDT", 200, 0, 4);
    assert_balance(&chain.final_balances, "seller-1", "BTC", 0, 0, 4);
    assert_balance(&chain.final_balances, "seller-2", "USDT", 90, 0, 4);
    assert_balance(&chain.final_balances, "seller-2", "BTC", 0, 0, 4);

    Ok(())
}

struct SettlementChain {
    consumed_reservations: Vec<ReservationConsumed>,
    consumed_reservation_updates: Vec<UpdatedEntityPair<AssetReservation>>,
    vouchers: Vec<SettlementTransferVoucher>,
    ledger_entries: Vec<BalanceLedgerEntryV2>,
    balance_updates: Vec<UpdatedEntityPair<Balance>>,
    final_balances: Vec<Balance>,
}

fn settle_principal_chain(
    trades: Vec<SpotTrade>,
    mut balances: Vec<Balance>,
) -> Result<SettlementChain, String> {
    let mut consumed_reservations = Vec::new();
    let mut consumed_reservation_updates = Vec::new();
    let mut vouchers = Vec::new();
    let mut ledger_entries = Vec::new();
    let mut balance_updates = Vec::new();

    for trade in &trades {
        let settlement_id = settlement_id_for(trade);
        let quote_amount =
            trade.notional_quote().ok_or_else(|| "trade notional overflowed".to_string())?;

        let buyer_reservation = reservation_for_trade_buyer_quote(trade, quote_amount)?;
        let seller_reservation = reservation_for_trade_seller_base(trade)?;
        consume_reservation(
            &buyer_reservation,
            quote_amount,
            trade,
            &mut consumed_reservations,
            &mut consumed_reservation_updates,
        )?;
        consume_reservation(
            &seller_reservation,
            trade.qty,
            trade,
            &mut consumed_reservations,
            &mut consumed_reservation_updates,
        )?;

        let voucher = SettlementTransferVoucher::build_spot_principal_voucher(
            format!("voucher:{settlement_id}"),
            settlement_id.clone(),
            trade,
            "BTC",
            "USDT",
            "fee-account".to_string(),
        )
        .ok_or_else(|| "failed to build spot principal voucher".to_string())?;

        apply_ledger(
            &mut balances,
            &mut ledger_entries,
            &mut balance_updates,
            trade.buyer_account_id(),
            "BTC",
            BalanceDelta::CreditAvailable(trade.qty),
            format!(
                "balance-ledger:{}:{}",
                settlement_id,
                SettlementTransferPurpose::SpotBuyerReceiveBase.as_str()
            ),
            BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                trade_ids: vec![trade.trade_id.clone()],
                settlement_ids: vec![settlement_id.clone()],
            },
        )?;
        apply_ledger(
            &mut balances,
            &mut ledger_entries,
            &mut balance_updates,
            trade.buyer_account_id(),
            "USDT",
            BalanceDelta::DebitFrozen(quote_amount),
            format!(
                "balance-ledger:{}:{}",
                settlement_id,
                SettlementTransferPurpose::SpotBuyerPayQuote.as_str()
            ),
            BalanceLedgerReason::SettleSpotTradeBuyerReleaseFrozenQuote {
                trade_ids: vec![trade.trade_id.clone()],
                settlement_ids: vec![settlement_id.clone()],
            },
        )?;
        apply_ledger(
            &mut balances,
            &mut ledger_entries,
            &mut balance_updates,
            trade.seller_account_id(),
            "USDT",
            BalanceDelta::CreditAvailable(quote_amount),
            format!(
                "balance-ledger:{}:{}",
                settlement_id,
                SettlementTransferPurpose::SpotSellerReceiveQuote.as_str()
            ),
            BalanceLedgerReason::SettleSpotTradeSellerReceiveQuote {
                trade_ids: vec![trade.trade_id.clone()],
                settlement_ids: vec![settlement_id.clone()],
            },
        )?;
        apply_ledger(
            &mut balances,
            &mut ledger_entries,
            &mut balance_updates,
            trade.seller_account_id(),
            "BTC",
            BalanceDelta::DebitFrozen(trade.qty),
            format!(
                "balance-ledger:{}:{}",
                settlement_id,
                SettlementTransferPurpose::SpotSellerDeliverBase.as_str()
            ),
            BalanceLedgerReason::SettleSpotTradeSellerReleaseFrozenBase {
                trade_ids: vec![trade.trade_id.clone()],
                settlement_ids: vec![settlement_id.clone()],
            },
        )?;

        vouchers.push(voucher);
    }

    Ok(SettlementChain {
        consumed_reservations,
        consumed_reservation_updates,
        vouchers,
        ledger_entries,
        balance_updates,
        final_balances: balances,
    })
}

fn consume_reservation(
    reservation: &AssetReservation,
    amount: u64,
    trade: &SpotTrade,
    consumed_reservations: &mut Vec<ReservationConsumed>,
    consumed_reservation_updates: &mut Vec<UpdatedEntityPair<AssetReservation>>,
) -> Result<(), String> {
    let before = reservation.clone();
    let after = reservation
        .consume(amount, Some(ReservationCloseReason::Filled))
        .map_err(|err| err.to_string())?;
    consumed_reservations.push(ReservationConsumed::new(
        format!("reservation-consumed:{}:{}", trade.trade_id, reservation.reservation_id),
        &after,
        amount,
        trade.trade_id.clone(),
    ));
    consumed_reservation_updates.push(UpdatedEntityPair { before, after });
    Ok(())
}

fn reservation_for_trade_buyer_quote(
    trade: &SpotTrade,
    quote_amount: u64,
) -> Result<AssetReservation, String> {
    Reservation::new(
        format!("reservation:{}", buyer_order_id(trade)),
        trade.buyer_account_id().to_string(),
        buyer_order_id(trade).to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotBuyQuote,
        "USDT".to_string(),
        quote_amount,
    )
    .map_err(|err| err.to_string())
}

fn reservation_for_trade_seller_base(trade: &SpotTrade) -> Result<AssetReservation, String> {
    Reservation::new(
        format!("reservation:{}", seller_order_id(trade)),
        trade.seller_account_id().to_string(),
        seller_order_id(trade).to_string(),
        ReservationMarketKind::Spot,
        ReservationKind::SpotSellBase,
        "BTC".to_string(),
        trade.qty,
    )
    .map_err(|err| err.to_string())
}

fn apply_ledger(
    balances: &mut [Balance],
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    balance_updates: &mut Vec<UpdatedEntityPair<Balance>>,
    account_id: &str,
    asset_id: &str,
    delta: BalanceDelta,
    entry_id: String,
    reason: BalanceLedgerReason,
) -> Result<(), String> {
    let balance = balances
        .iter_mut()
        .find(|balance| balance.belongs_to_account(account_id) && balance.is_asset(asset_id))
        .ok_or_else(|| format!("missing balance for {account_id}:{asset_id}"))?;
    let before = balance.clone();
    let entry = match delta {
        BalanceDelta::CreditAvailable(amount) => {
            BalanceLedgerEntryV2::credit_available(entry_id, balance, amount, reason)
        }
        BalanceDelta::DebitFrozen(amount) => {
            BalanceLedgerEntryV2::debit_frozen(entry_id, balance, amount, reason)
        }
    }
    .map_err(|err| err.to_string())?;
    balance_updates.push(UpdatedEntityPair { before, after: balance.clone() });
    ledger_entries.push(entry);
    Ok(())
}

enum BalanceDelta {
    CreditAvailable(u64),
    DebitFrozen(u64),
}

fn trade(
    trade_id: &str,
    taker_side: SpotOrderSide,
    taker_account_id: &str,
    maker_account_id: &str,
    price: u64,
    qty: u64,
) -> SpotTrade {
    SpotTrade::new(
        trade_id.to_string(),
        "match-1".to_string(),
        10_001,
        "BTCUSDT".to_string(),
        format!("{trade_id}-taker"),
        format!("{trade_id}-maker"),
        taker_account_id.to_string(),
        maker_account_id.to_string(),
        taker_side,
        price,
        qty,
        0,
        0,
    )
}

fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
    Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 3)
}

fn buyer_order_id(trade: &SpotTrade) -> &str {
    match trade.taker_side {
        SpotOrderSide::Buy => trade.taker_order_id.as_str(),
        SpotOrderSide::Sell => trade.maker_order_id.as_str(),
    }
}

fn seller_order_id(trade: &SpotTrade) -> &str {
    match trade.taker_side {
        SpotOrderSide::Buy => trade.maker_order_id.as_str(),
        SpotOrderSide::Sell => trade.taker_order_id.as_str(),
    }
}

fn settlement_id_for(trade: &SpotTrade) -> String {
    format!("settlement:{}", trade.trade_id)
}

fn assert_consumed(
    consumed: &ReservationConsumed,
    reservation_id: &str,
    owner_account_id: &str,
    asset_id: &str,
    amount: u64,
    trade_id: &str,
) {
    assert_eq!(consumed.reservation_id, reservation_id);
    assert_eq!(consumed.owner_account_id, owner_account_id);
    assert_eq!(consumed.asset_id, asset_id);
    assert_eq!(consumed.amount, amount);
    assert_eq!(consumed.caused_by_ref_id, trade_id);
    assert_eq!(consumed.remaining_amount_after, 0);
}

fn assert_consumed_pair(
    reservation_update: &UpdatedEntityPair<AssetReservation>,
    consumed: &ReservationConsumed,
    reservation_id: &str,
    owner_account_id: &str,
    asset_id: &str,
    amount: u64,
    trade_id: &str,
) {
    assert_consumed(consumed, reservation_id, owner_account_id, asset_id, amount, trade_id);
    assert_eq!(reservation_update.before.reservation_id, reservation_id);
    assert_eq!(reservation_update.before.owner_account_id, owner_account_id);
    assert_eq!(reservation_update.before.asset_id, asset_id);
    assert_eq!(reservation_update.before.remaining_amount, amount);
    assert_eq!(reservation_update.after.reservation_id, reservation_id);
    assert_eq!(reservation_update.after.owner_account_id, owner_account_id);
    assert_eq!(reservation_update.after.asset_id, asset_id);
    assert_eq!(reservation_update.after.remaining_amount, 0);
    assert_eq!(reservation_update.after.status, ReservationStatus::ExhaustedByConsume);
    assert_eq!(reservation_update.after.close_reason, Some(ReservationCloseReason::Filled));
    assert_eq!(consumed.remaining_amount_after, reservation_update.after.remaining_amount);
}

fn assert_spot_principal_voucher(
    voucher: &SettlementTransferVoucher,
    trade: &SpotTrade,
    settlement_id: &str,
) {
    assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
    assert_eq!(voucher.trade_id(), trade.trade_id);
    assert_eq!(voucher.settlement_id(), settlement_id);
    assert_eq!(voucher.match_id(), Some(trade.match_id.as_str()));
    assert_eq!(voucher.fee_account_id(), "fee-account");

    assert_transfer(
        voucher,
        SettlementTransferPurpose::SpotBuyerReceiveBase,
        trade.seller_account_id(),
        trade.buyer_account_id(),
        "BTC",
        trade.qty,
    );
    assert_transfer(
        voucher,
        SettlementTransferPurpose::SpotBuyerPayQuote,
        trade.buyer_account_id(),
        trade.seller_account_id(),
        "USDT",
        trade.notional_quote().unwrap_or(u64::MAX),
    );
    assert_transfer(
        voucher,
        SettlementTransferPurpose::SpotSellerReceiveQuote,
        trade.buyer_account_id(),
        trade.seller_account_id(),
        "USDT",
        trade.notional_quote().unwrap_or(u64::MAX),
    );
    assert_transfer(
        voucher,
        SettlementTransferPurpose::SpotSellerDeliverBase,
        trade.seller_account_id(),
        trade.buyer_account_id(),
        "BTC",
        trade.qty,
    );
}

fn assert_voucher_references_ledgers(
    voucher: &SettlementTransferVoucher,
    ledger_entries: &[BalanceLedgerEntryV2],
) {
    assert_eq!(ledger_entries.len(), 4);
    for ledger_entry in ledger_entries {
        assert!(voucher.references_ledger_entry(ledger_entry.entry_id.as_str()));
    }
}

fn assert_transfer(
    voucher: &SettlementTransferVoucher,
    purpose: SettlementTransferPurpose,
    from_account_id: &str,
    to_account_id: &str,
    asset_id: &str,
    amount: u64,
) {
    let transfers = voucher.transfers_for_purpose(purpose);
    assert_eq!(transfers.len(), 1);
    assert_eq!(transfers[0].from_account_id(), from_account_id);
    assert_eq!(transfers[0].to_account_id(), to_account_id);
    assert_eq!(transfers[0].asset_id(), asset_id);
    assert_eq!(transfers[0].amount(), amount);
}

fn assert_ledger_matches_update(
    ledger_entry: &BalanceLedgerEntryV2,
    balance_update: &UpdatedEntityPair<Balance>,
) {
    assert!(ledger_entry.matches_balance_update(balance_update));
    assert_eq!(ledger_entry.after_available, balance_update.after.available);
    assert_eq!(ledger_entry.after_frozen, balance_update.after.frozen);
}

fn assert_ledger_reason_refs(
    ledger_entry: &BalanceLedgerEntryV2,
    trade_id: &str,
    settlement_id: &str,
) {
    assert_eq!(ledger_entry.reason.trade_ids(), &[trade_id.to_string()]);
    assert_eq!(ledger_entry.reason.settlement_ids(), &[settlement_id.to_string()]);
}

fn assert_balance(
    balances: &[Balance],
    account_id: &str,
    asset_id: &str,
    available: u64,
    frozen: u64,
    version: u64,
) {
    let Some(balance) = balances
        .iter()
        .find(|balance| balance.belongs_to_account(account_id) && balance.is_asset(asset_id))
    else {
        panic!("missing balance for {account_id}:{asset_id}");
    };
    assert_eq!(balance.available, available);
    assert_eq!(balance.frozen, frozen);
    assert_eq!(balance.version, version);
}
