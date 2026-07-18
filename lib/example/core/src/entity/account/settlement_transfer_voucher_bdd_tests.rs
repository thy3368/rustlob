use super::{BalanceLedgerOperation, BalanceLedgerReason, SettlementTransferPurpose};
use crate::{HyperliquidPerpTrade, SpotTrade};

fn balance_entity_id(account_id: &str, asset_id: &str) -> String {
    format!("balance:{account_id}:{asset_id}")
}

fn perp_trade() -> HyperliquidPerpTrade {
    HyperliquidPerpTrade::new(
        "trade-perp-1".to_string(),
        "match-perp-1".to_string(),
        0,
        "BTC-PERP".to_string(),
        "taker-order-1".to_string(),
        "maker-order-1".to_string(),
        "winner".to_string(),
        "loser".to_string(),
        crate::HyperliquidPerpOrderSide::Buy,
        100,
        1,
        1_717_171_717_000,
    )
}

#[test]
fn derive_balance_ledger_entries_maps_spot_principal_and_fee_legs() {
    let trade = SpotTrade::new(
        "trade-ledger-1".to_string(),
        "match-ledger-1".to_string(),
        10_001,
        "BTCUSDT".to_string(),
        "taker-ledger-1".to_string(),
        "maker-ledger-1".to_string(),
        "buyer".to_string(),
        "seller".to_string(),
        crate::SpotOrderSide::Buy,
        100,
        2,
        1,
        2,
    );
    let voucher = trade
        .derive_spot_settlement_transfer_voucher_with_fees(
            "voucher-ledger-1".to_string(),
            "settle-ledger-1".to_string(),
            "BTC",
            "USDT",
            "fee-account".to_string(),
        )
        .unwrap();

    let entries = voucher.derive_balance_ledger_entries(balance_entity_id).unwrap();

    assert_eq!(entries.len(), 12);

    let expected = [
        (
            "balance-ledger:settle-ledger-1:spot_buyer_receive_base:debit",
            "seller",
            "BTC",
            "balance:seller:BTC",
            BalanceLedgerOperation::DebitFrozen,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_buyer_receive_base:credit",
            "buyer",
            "BTC",
            "balance:buyer:BTC",
            BalanceLedgerOperation::CreditAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_buyer_pay_quote:debit",
            "buyer",
            "USDT",
            "balance:buyer:USDT",
            BalanceLedgerOperation::DebitFrozen,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_buyer_pay_quote:credit",
            "seller",
            "USDT",
            "balance:seller:USDT",
            BalanceLedgerOperation::CreditAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_seller_receive_quote:debit",
            "buyer",
            "USDT",
            "balance:buyer:USDT",
            BalanceLedgerOperation::DebitFrozen,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_seller_receive_quote:credit",
            "seller",
            "USDT",
            "balance:seller:USDT",
            BalanceLedgerOperation::CreditAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_seller_deliver_base:debit",
            "seller",
            "BTC",
            "balance:seller:BTC",
            BalanceLedgerOperation::DebitFrozen,
        ),
        (
            "balance-ledger:settle-ledger-1:spot_seller_deliver_base:credit",
            "buyer",
            "BTC",
            "balance:buyer:BTC",
            BalanceLedgerOperation::CreditAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:trading_fee:taker:debit",
            "buyer",
            "USDT",
            "balance:buyer:USDT",
            BalanceLedgerOperation::DebitAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:trading_fee:taker:credit",
            "fee-account",
            "USDT",
            "balance:fee-account:USDT",
            BalanceLedgerOperation::CreditAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:trading_fee:maker:debit",
            "seller",
            "USDT",
            "balance:seller:USDT",
            BalanceLedgerOperation::DebitAvailable,
        ),
        (
            "balance-ledger:settle-ledger-1:trading_fee:maker:credit",
            "fee-account",
            "USDT",
            "balance:fee-account:USDT",
            BalanceLedgerOperation::CreditAvailable,
        ),
    ];

    for (entry, (entry_id, account_id, asset_id, balance_id, operation)) in
        entries.iter().zip(expected)
    {
        assert_eq!(entry.entry_id, entry_id);
        assert_eq!(entry.account_id, account_id);
        assert_eq!(entry.asset_id, asset_id);
        assert_eq!(entry.balance_entity_id, balance_id);
        assert_eq!(entry.operation, operation);
        assert!(!entry.is_applied());
        assert_eq!(entry.before_available, None);
        assert_eq!(entry.before_frozen, None);
        assert_eq!(entry.after_available, None);
        assert_eq!(entry.after_frozen, None);
    }

    assert_eq!(
        entries[8].reason,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: "trade-ledger-1".to_string(),
            match_id: "match-ledger-1".to_string(),
            settlement_batch_id: "settle-ledger-1".to_string(),
            purpose: SettlementTransferPurpose::TradingFee,
        }
    );
    assert_eq!(
        entries[10].reason,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: "trade-ledger-1".to_string(),
            match_id: "match-ledger-1".to_string(),
            settlement_batch_id: "settle-ledger-1".to_string(),
            purpose: SettlementTransferPurpose::TradingFee,
        }
    );
}

#[test]
fn derive_balance_ledger_entries_maps_perp_pnl_and_fee_to_available() {
    let voucher = perp_trade()
        .derive_perp_settlement_transfer_voucher(
            "voucher-perp-ledger".to_string(),
            "settlement-perp-ledger".to_string(),
            "USDC",
            "fee-account".to_string(),
            1,
            0,
            25,
            -25,
        )
        .unwrap();

    let entries = voucher.derive_balance_ledger_entries(balance_entity_id).unwrap();

    assert_eq!(entries.len(), 4);
    assert_eq!(
        entries[0].entry_id,
        "balance-ledger:settlement-perp-ledger:perp_realized_pnl_transfer:loser:winner:debit"
    );
    assert_eq!(entries[0].account_id, "loser");
    assert_eq!(entries[0].asset_id, "USDC");
    assert_eq!(entries[0].balance_entity_id, "balance:loser:USDC");
    assert_eq!(entries[0].operation, BalanceLedgerOperation::DebitAvailable);
    assert!(!entries[0].is_applied());
    assert_eq!(
        entries[0].reason,
        BalanceLedgerReason::SettlePerpTrade {
            trade_id: "trade-perp-1".to_string(),
            match_id: "match-perp-1".to_string(),
            settlement_id: "settlement-perp-ledger".to_string(),
            purpose: SettlementTransferPurpose::PerpRealizedPnlTransfer,
        }
    );
    assert_eq!(
        entries[1].entry_id,
        "balance-ledger:settlement-perp-ledger:perp_realized_pnl_transfer:loser:winner:credit"
    );
    assert_eq!(entries[1].account_id, "winner");
    assert_eq!(entries[1].balance_entity_id, "balance:winner:USDC");
    assert_eq!(entries[1].operation, BalanceLedgerOperation::CreditAvailable);
    assert!(!entries[1].is_applied());
    assert_eq!(entries[1].before_available, None);
    assert_eq!(entries[1].before_frozen, None);
    assert_eq!(entries[1].after_available, None);
    assert_eq!(entries[1].after_frozen, None);
    assert_eq!(
        entries[2].entry_id,
        "balance-ledger:settlement-perp-ledger:trading_fee:taker:debit"
    );
    assert_eq!(entries[2].account_id, "winner");
    assert_eq!(entries[2].balance_entity_id, "balance:winner:USDC");
    assert_eq!(entries[2].operation, BalanceLedgerOperation::DebitAvailable);
    assert!(!entries[2].is_applied());
    assert_eq!(entries[2].before_available, None);
    assert_eq!(entries[2].before_frozen, None);
    assert_eq!(entries[2].after_available, None);
    assert_eq!(entries[2].after_frozen, None);
    assert_eq!(
        entries[3].entry_id,
        "balance-ledger:settlement-perp-ledger:trading_fee:taker:credit"
    );
    assert_eq!(entries[3].account_id, "fee-account");
    assert_eq!(entries[3].balance_entity_id, "balance:fee-account:USDC");
    assert_eq!(entries[3].operation, BalanceLedgerOperation::CreditAvailable);
    assert!(!entries[3].is_applied());
    assert_eq!(entries[3].before_available, None);
    assert_eq!(entries[3].before_frozen, None);
    assert_eq!(entries[3].after_available, None);
    assert_eq!(entries[3].after_frozen, None);
    assert_eq!(
        entries[2].reason,
        BalanceLedgerReason::SettlePerpTrade {
            trade_id: "trade-perp-1".to_string(),
            match_id: "match-perp-1".to_string(),
            settlement_id: "settlement-perp-ledger".to_string(),
            purpose: SettlementTransferPurpose::TradingFee,
        }
    );
    assert_eq!(
        entries[3].reason,
        BalanceLedgerReason::SettlePerpTrade {
            trade_id: "trade-perp-1".to_string(),
            match_id: "match-perp-1".to_string(),
            settlement_id: "settlement-perp-ledger".to_string(),
            purpose: SettlementTransferPurpose::TradingFee,
        }
    );
}
