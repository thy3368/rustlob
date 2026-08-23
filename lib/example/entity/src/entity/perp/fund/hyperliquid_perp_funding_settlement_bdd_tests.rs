use super::hyperliquid_perp_funding_settlement::HyperliquidPerpFundingSettlement;
use crate::entity::{BalanceLedgerEntryV2Error, BalanceLedgerOperation, BalanceLedgerReason};

fn funding_settlement(signed_usdc_delta: i128) -> HyperliquidPerpFundingSettlement {
    HyperliquidPerpFundingSettlement::new(
        "funding-batch-1-position-1".to_string(),
        "funding-batch-1".to_string(),
        "trader-1".to_string(),
        "position-1".to_string(),
        "BTC-PERP".to_string(),
        1_717_977_600_000,
        2,
        50_000,
        10_000,
        signed_usdc_delta,
        None,
        None,
    )
}

fn assert_single_settlement_reason(reason: &BalanceLedgerReason) {
    match reason {
        BalanceLedgerReason::SettlePerpFunding {
            funding_batch_id,
            settlement_ids,
            position_ids,
        } => {
            assert_eq!(funding_batch_id, "funding-batch-1");
            assert_eq!(settlement_ids, &vec!["funding-batch-1-position-1".to_string()]);
            assert_eq!(position_ids, &vec!["position-1".to_string()]);
        }
        other => panic!("unexpected balance ledger reason: {other:?}"),
    }
}

#[test]
fn payment_funding_settlement_derives_debit_margin_ledger_entry() {
    let settlement = funding_settlement(-42);

    let entry = settlement
        .derive_margin_balance_ledger_entry(
            "ledger-1".to_string(),
            "USDC".to_string(),
            "balance:trader-1:USDC".to_string(),
        )
        .unwrap();

    assert_eq!(entry.entry_id, "ledger-1");
    assert_eq!(entry.account_id, "trader-1");
    assert_eq!(entry.asset_id, "USDC");
    assert_eq!(entry.balance_entity_id, "balance:trader-1:USDC");
    assert_eq!(entry.operation, BalanceLedgerOperation::DebitAvailable);
    assert_eq!(entry.amount, 42);
    assert_eq!(entry.before_available, None);
    assert_eq!(entry.after_available, None);
    assert_single_settlement_reason(&entry.reason);
}

#[test]
fn receivable_funding_settlement_derives_credit_margin_ledger_entry() {
    let settlement = funding_settlement(37);

    let entry = settlement
        .derive_margin_balance_ledger_entry(
            "ledger-2".to_string(),
            "USDC".to_string(),
            "balance:trader-1:USDC".to_string(),
        )
        .unwrap();

    assert_eq!(entry.entry_id, "ledger-2");
    assert_eq!(entry.account_id, "trader-1");
    assert_eq!(entry.asset_id, "USDC");
    assert_eq!(entry.balance_entity_id, "balance:trader-1:USDC");
    assert_eq!(entry.operation, BalanceLedgerOperation::CreditAvailable);
    assert_eq!(entry.amount, 37);
    assert_eq!(entry.before_available, None);
    assert_eq!(entry.after_available, None);
    assert_single_settlement_reason(&entry.reason);
}

#[test]
fn zero_funding_fee_cannot_derive_margin_ledger_entry() {
    let settlement = funding_settlement(0);

    let error = settlement
        .derive_margin_balance_ledger_entry(
            "ledger-zero".to_string(),
            "USDC".to_string(),
            "balance:trader-1:USDC".to_string(),
        )
        .unwrap_err();

    assert_eq!(error, BalanceLedgerEntryV2Error::InvalidAmount);
}
