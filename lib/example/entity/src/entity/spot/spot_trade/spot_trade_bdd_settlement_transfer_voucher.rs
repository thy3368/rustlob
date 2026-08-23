use super::*;

fn trade() -> SpotTrade {
    SpotTrade::new(
        "match-1-1".to_string(),
        "match-1".to_string(),
        10_001,
        "BTCUSDT".to_string(),
        "taker-1".to_string(),
        "maker-1".to_string(),
        "buyer".to_string(),
        "seller".to_string(),
        SpotOrderSide::Buy,
        100,
        2,
        2,
        1,
    )
}

#[test]
fn derive_spot_principal_settlement_transfer_voucher_creates_expected_legs() {
    // Given: 一笔 buy taker 成交，成交 quote notional 为 200。
    let trade = trade();

    // When: 从成交事实派生 principal-only settlement transfer voucher。
    let voucher = trade
        .derive_spot_principal_settlement_transfer_voucher(
            "voucher-1".to_string(),
            "settle-1".to_string(),
            "BTC",
            "USDT",
            "fee-account".to_string(),
        )
        .unwrap();

    // Then: voucher 指向该成交，并包含买方收 base / 付 quote、卖方收 quote / 交 base 四条腿。
    assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
    assert_eq!(voucher.trade_id(), "match-1-1");
    assert_eq!(
        voucher.amount_received_by_for_purpose(
            "buyer",
            SettlementTransferPurpose::SpotBuyerReceiveBase
        ),
        Some(2)
    );
    assert_eq!(
        voucher.amount_sent_by_for_purpose("buyer", SettlementTransferPurpose::SpotBuyerPayQuote),
        Some(200)
    );
    assert_eq!(
        voucher.amount_received_by_for_purpose(
            "seller",
            SettlementTransferPurpose::SpotSellerReceiveQuote
        ),
        Some(200)
    );
    assert_eq!(
        voucher
            .amount_sent_by_for_purpose("seller", SettlementTransferPurpose::SpotSellerDeliverBase),
        Some(2)
    );
}

#[test]
fn derive_spot_settlement_transfer_voucher_with_fees_uses_trade_fee_facts() {
    // Given: 一笔成交已经记录 taker fee=2、maker fee=1。
    let trade = trade();

    // When: 派生包含 principal 与 trading fee 的 settlement transfer voucher。
    let voucher = trade
        .derive_spot_settlement_transfer_voucher_with_fees(
            "voucher-1".to_string(),
            "settle-1".to_string(),
            "BTC",
            "USDT",
            "fee-account".to_string(),
        )
        .unwrap();

    // Then: fee legs 复用真实买卖方 fee 角色金额，并追加到 principal voucher 之后。
    assert_eq!(voucher.settlement_kind(), SettlementKind::Spot);
    assert_eq!(voucher.trade_id(), "match-1-1");
    assert_eq!(
        voucher.amount_received_by_for_purpose(
            "buyer",
            SettlementTransferPurpose::SpotBuyerReceiveBase
        ),
        Some(2)
    );
    assert_eq!(
        voucher.amount_sent_by_for_purpose("buyer", SettlementTransferPurpose::SpotBuyerPayQuote),
        Some(200)
    );
    assert_eq!(voucher.fee_amount_paid_by("buyer"), Some(2));
    assert_eq!(voucher.fee_amount_paid_by("seller"), Some(1));
    assert_eq!(voucher.transfers_for_purpose(SettlementTransferPurpose::TradingFee).len(), 2);
}

#[test]
fn derive_spot_settlement_transfer_voucher_returns_none_when_notional_overflows() {
    // Given: price * qty 会溢出的成交事实。
    let overflow_trade = SpotTrade { price: u64::MAX, qty: 2, ..trade() };

    // When / Then: principal voucher 派生无法获得 quote notional，返回 None。
    assert_eq!(
        overflow_trade.derive_spot_principal_settlement_transfer_voucher(
            "voucher-overflow".to_string(),
            "settle-overflow".to_string(),
            "BTC",
            "USDT",
            "fee-account".to_string(),
        ),
        None
    );
}
