use common_entity::Fsm;
use example_core_entity::spot::SpotTradeV3;
use example_core_entity::spot::spot_trade_v3::{
    SpotTradeEvent, SpotTradeSettlementError, SpotTradeStateError,
};
use example_core_entity::{BalanceV2, BalanceV2Error, BalanceV2Event, OrderSide};
use rust_decimal::Decimal;
use thiserror::Error;

/// 一笔现货成交成功结算后的聚合快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeSettlementOutcome {
    pub trade_after: SpotTradeV3,
    pub buyer_base_balance_after: BalanceV2,
    pub buyer_quote_balance_after: BalanceV2,
    pub seller_base_balance_after: BalanceV2,
    pub seller_quote_balance_after: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotTradeDomainServiceError {
    #[error("trade participants must be distinct")]
    SameParticipant,
    #[error("balance party does not match its settlement role")]
    BalancePartyMismatch,
    #[error("balance coin does not match its settlement role")]
    BalanceCoinMismatch,
    #[error("a participant's base and quote balances must belong to the same account")]
    AccountMismatch,
    #[error("settlement balance identities must be distinct")]
    DuplicateBalance,
    #[error(transparent)]
    Settlement(#[from] SpotTradeSettlementError),
    #[error(transparent)]
    TradeState(#[from] SpotTradeStateError),
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SpotTradeDomainService;

impl SpotTradeDomainService {
    /// 协调成交与买卖双方四个余额聚合，原输入只作为结算前快照使用。
    pub fn settlement(
        trade: SpotTradeV3,
        buyer_base_balance: BalanceV2,
        buyer_quote_balance: BalanceV2,
        seller_base_balance: BalanceV2,
        seller_quote_balance: BalanceV2,
        timestamp: u64,
    ) -> Result<SpotTradeSettlementOutcome, SpotTradeDomainServiceError> {
        validate_balances(
            &trade,
            &buyer_base_balance,
            &buyer_quote_balance,
            &seller_base_balance,
            &seller_quote_balance,
        )?;
        let amounts = trade.settlement_amounts()?;

        // 所有聚合仅在工作副本上推进，任一步失败都不会暴露部分结算结果。
        let mut trade_after = trade.clone();
        let mut buyer_base_balance_after = buyer_base_balance.clone();
        let mut buyer_quote_balance_after = buyer_quote_balance.clone();
        let mut seller_base_balance_after = seller_base_balance.clone();
        let mut seller_quote_balance_after = seller_quote_balance.clone();

        buyer_quote_balance_after.apply(BalanceV2Event::DebitFrozen {
            amount: amounts.buyer_quote_payment,
            timestamp,
        })?;
        buyer_base_balance_after
            .apply(BalanceV2Event::CreditAvailable { amount: amounts.base_amount, timestamp })?;
        seller_base_balance_after
            .apply(BalanceV2Event::DebitFrozen { amount: amounts.base_amount, timestamp })?;
        if amounts.seller_quote_receipt > Decimal::ZERO {
            seller_quote_balance_after.apply(BalanceV2Event::CreditAvailable {
                amount: amounts.seller_quote_receipt,
                timestamp,
            })?;
        }
        trade_after.apply(SpotTradeEvent::Settle)?;

        Ok(SpotTradeSettlementOutcome {
            trade_after,
            buyer_base_balance_after,
            buyer_quote_balance_after,
            seller_base_balance_after,
            seller_quote_balance_after,
        })
    }
}

fn validate_balances(
    trade: &SpotTradeV3,
    buyer_base: &BalanceV2,
    buyer_quote: &BalanceV2,
    seller_base: &BalanceV2,
    seller_quote: &BalanceV2,
) -> Result<(), SpotTradeDomainServiceError> {
    let trade = &trade.comon_trade;
    if trade.taker_user == trade.maker_user {
        return Err(SpotTradeDomainServiceError::SameParticipant);
    }
    let (buyer_party, seller_party) = match trade.taker_side {
        OrderSide::Buy => (trade.taker_user.0, trade.maker_user.0),
        OrderSide::Sell => (trade.maker_user.0, trade.taker_user.0),
    };
    let expected = [
        (buyer_base, buyer_party, u64::from(trade.symbol.base.index())),
        (buyer_quote, buyer_party, u64::from(trade.symbol.quote.index())),
        (seller_base, seller_party, u64::from(trade.symbol.base.index())),
        (seller_quote, seller_party, u64::from(trade.symbol.quote.index())),
    ];
    if expected.iter().any(|(balance, party, _)| balance.party_id() != *party) {
        return Err(SpotTradeDomainServiceError::BalancePartyMismatch);
    }
    if expected.iter().any(|(balance, _, coin)| balance.coin_idx() != *coin) {
        return Err(SpotTradeDomainServiceError::BalanceCoinMismatch);
    }
    if buyer_base.account_id() != buyer_quote.account_id()
        || seller_base.account_id() != seller_quote.account_id()
    {
        return Err(SpotTradeDomainServiceError::AccountMismatch);
    }
    for left in 0..expected.len() {
        for right in (left + 1)..expected.len() {
            let a = expected[left].0;
            let b = expected[right].0;
            if a.account_id() == b.account_id() && a.coin_idx() == b.coin_idx() {
                return Err(SpotTradeDomainServiceError::DuplicateBalance);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::entity::common::UserId;
    use example_core_entity::entity::common::common_trade::TradeBase;
    use example_core_entity::spot::SpotTradeSettlementStatus;

    use super::*;

    const TIMESTAMP: u64 = 500;

    fn balance(party: u64, account: u64, coin: Coin, available: i64, frozen: i64) -> BalanceV2 {
        BalanceV2::new(party, account, u64::from(coin.index()), available, frozen, 4, 100)
    }

    fn inputs(
        side: OrderSide,
        seller_fee: i64,
    ) -> (SpotTradeV3, BalanceV2, BalanceV2, BalanceV2, BalanceV2) {
        let (buyer, seller) = match side {
            OrderSide::Buy => (10, 20),
            OrderSide::Sell => (20, 10),
        };
        let trade = SpotTradeV3 {
            comon_trade: TradeBase {
                trade_id: 7,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                taker_user: UserId(10),
                maker_user: UserId(20),
                taker_order_id: 1,
                maker_order_id: 2,
                taker_side: side,
                price: Decimal::from(2),
                size: Decimal::from(3),
                taker_fee: if side == OrderSide::Buy {
                    Decimal::ONE
                } else {
                    Decimal::from(seller_fee)
                },
                maker_fee: if side == OrderSide::Buy {
                    Decimal::from(seller_fee)
                } else {
                    Decimal::from(2)
                },
                created_at: 0,
                updated_at: 0,
                version: 0,
            },
            settlement_status: SpotTradeSettlementStatus::Unsettled,
        };
        (
            trade,
            balance(buyer, buyer, Coin::BTC, 10, 0),
            balance(buyer, buyer, Coin::USDT, 10, 20),
            balance(seller, seller, Coin::BTC, 10, 20),
            balance(seller, seller, Coin::USDT, 10, 0),
        )
    }

    fn settle(
        values: &(SpotTradeV3, BalanceV2, BalanceV2, BalanceV2, BalanceV2),
    ) -> Result<SpotTradeSettlementOutcome, SpotTradeDomainServiceError> {
        SpotTradeDomainService::settlement(
            values.0.clone(),
            values.1.clone(),
            values.2.clone(),
            values.3.clone(),
            values.4.clone(),
            TIMESTAMP,
        )
    }

    #[test]
    fn settles_taker_buy_and_keeps_inputs_unchanged() {
        let values = inputs(OrderSide::Buy, 2);
        let before = values.clone();
        let outcome = settle(&values).unwrap();

        assert_eq!(values, before);
        assert_eq!(outcome.trade_after.settlement_status, SpotTradeSettlementStatus::Settled);
        assert_eq!(outcome.buyer_quote_balance_after.frozen(), Decimal::from(13));
        assert_eq!(outcome.buyer_base_balance_after.available(), Decimal::from(13));
        assert_eq!(outcome.seller_base_balance_after.frozen(), Decimal::from(17));
        assert_eq!(outcome.seller_quote_balance_after.available(), Decimal::from(14));
        for balance in [
            &outcome.buyer_quote_balance_after,
            &outcome.buyer_base_balance_after,
            &outcome.seller_base_balance_after,
            &outcome.seller_quote_balance_after,
        ] {
            assert_eq!(balance.version(), 5);
            assert_eq!(balance.updated_at(), TIMESTAMP);
        }
    }

    #[test]
    fn taker_sell_maps_parties_and_fees() {
        let values = inputs(OrderSide::Sell, 1);
        let outcome = settle(&values).unwrap();

        assert_eq!(outcome.buyer_base_balance_after.party_id(), 20);
        assert_eq!(outcome.seller_base_balance_after.party_id(), 10);
        assert_eq!(outcome.buyer_quote_balance_after.frozen(), Decimal::from(12));
        assert_eq!(outcome.seller_quote_balance_after.available(), Decimal::from(15));
    }

    #[test]
    fn zero_seller_receipt_does_not_update_seller_quote() {
        let values = inputs(OrderSide::Buy, 6);
        let outcome = settle(&values).unwrap();

        assert_eq!(outcome.seller_quote_balance_after, values.4);
    }

    #[test]
    fn aggregate_failures_are_atomic() {
        let mut settled = inputs(OrderSide::Buy, 2);
        settled.0.settlement_status = SpotTradeSettlementStatus::Settled;
        let before = settled.clone();
        assert_eq!(
            settle(&settled),
            Err(SpotTradeDomainServiceError::TradeState(SpotTradeStateError::AlreadySettled))
        );
        assert_eq!(settled, before);

        let mut insufficient = inputs(OrderSide::Buy, 2);
        insufficient.2 = balance(10, 10, Coin::USDT, 10, 6);
        let before = insufficient.clone();
        assert_eq!(
            settle(&insufficient),
            Err(SpotTradeDomainServiceError::Balance(BalanceV2Error::InsufficientFrozenBalance))
        );
        assert_eq!(insufficient, before);

        let mut closed = inputs(OrderSide::Buy, 2);
        closed.1 = BalanceV2::new(10, 10, u64::from(Coin::BTC.index()), 0, 0, 4, 100);
        closed.1.apply(BalanceV2Event::Close { timestamp: 200 }).unwrap();
        let before = closed.clone();
        assert_eq!(
            settle(&closed),
            Err(SpotTradeDomainServiceError::Balance(BalanceV2Error::BalanceClosed))
        );
        assert_eq!(closed, before);
    }

    #[test]
    fn rejects_identity_coin_account_and_duplicate_balances_without_mutation() {
        let mut party = inputs(OrderSide::Buy, 2);
        party.1 = balance(99, 10, Coin::BTC, 10, 0);
        assert_eq!(settle(&party), Err(SpotTradeDomainServiceError::BalancePartyMismatch));

        let mut coin = inputs(OrderSide::Buy, 2);
        coin.1 = balance(10, 10, Coin::USDT, 10, 0);
        assert_eq!(settle(&coin), Err(SpotTradeDomainServiceError::BalanceCoinMismatch));

        let mut account = inputs(OrderSide::Buy, 2);
        account.2 = balance(10, 11, Coin::USDT, 10, 20);
        assert_eq!(settle(&account), Err(SpotTradeDomainServiceError::AccountMismatch));

        let mut duplicate = inputs(OrderSide::Buy, 2);
        duplicate.3 = balance(20, 10, Coin::BTC, 10, 20);
        duplicate.4 = balance(20, 10, Coin::USDT, 10, 0);
        let before = duplicate.clone();
        assert_eq!(settle(&duplicate), Err(SpotTradeDomainServiceError::DuplicateBalance));
        assert_eq!(duplicate, before);
    }
}
