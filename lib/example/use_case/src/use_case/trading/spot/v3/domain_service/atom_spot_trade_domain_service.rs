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
    ///
    /// `lsn` 与 `logical_ts` 来自所属 use case 的 `LogEntry`；所有实际修改的聚合共享该元数据。
    pub fn settlement(
        trade: SpotTradeV3,
        buyer_base_balance: BalanceV2,
        buyer_quote_balance: BalanceV2,
        seller_base_balance: BalanceV2,
        seller_quote_balance: BalanceV2,
        lsn: u64,
        logical_ts: u64,
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
            timestamp: logical_ts,
            lsn,
        })?;
        buyer_base_balance_after.apply(BalanceV2Event::CreditAvailable {
            amount: amounts.base_amount,
            timestamp: logical_ts,
            lsn,
        })?;
        seller_base_balance_after.apply(BalanceV2Event::DebitFrozen {
            amount: amounts.base_amount,
            timestamp: logical_ts,
            lsn,
        })?;
        if amounts.seller_quote_receipt > Decimal::ZERO {
            seller_quote_balance_after.apply(BalanceV2Event::CreditAvailable {
                amount: amounts.seller_quote_receipt,
                timestamp: logical_ts,
                lsn,
            })?;
        }
        trade_after.apply(SpotTradeEvent::Settle { lsn, logical_ts })?;

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
        (
            buyer_base,
            buyer_party,
            u64::from(trade.symbol.base.index()),
        ),
        (
            buyer_quote,
            buyer_party,
            u64::from(trade.symbol.quote.index()),
        ),
        (
            seller_base,
            seller_party,
            u64::from(trade.symbol.base.index()),
        ),
        (
            seller_quote,
            seller_party,
            u64::from(trade.symbol.quote.index()),
        ),
    ];
    if expected
        .iter()
        .any(|(balance, party, _)| balance.party_id() != *party)
    {
        return Err(SpotTradeDomainServiceError::BalancePartyMismatch);
    }
    if expected
        .iter()
        .any(|(balance, _, coin)| balance.coin_idx() != *coin)
    {
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
