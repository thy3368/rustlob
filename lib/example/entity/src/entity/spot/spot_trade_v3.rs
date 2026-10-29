use rust_decimal::Decimal;
use thiserror::Error;

use crate::OrderSide;
use crate::entity::common::common_trade::TradeBase;

/// 一笔现货成交在买卖双方的结算金额。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpotTradeSettlementAmounts {
    pub base_amount: Decimal,
    pub buyer_quote_payment: Decimal,
    pub seller_quote_receipt: Decimal,
}

/// 现货成交结算金额派生错误。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum SpotTradeSettlementError {
    #[error("trade price and size must be greater than zero")]
    InvalidTradeAmount,
    #[error("trade fees must not be negative")]
    InvalidFee,
    #[error("trade amount arithmetic overflow")]
    ArithmeticOverflow,
    #[error("seller fee exceeds quote proceeds")]
    SellerFeeExceedsProceeds,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotTradeV3 {
    pub comon_trade: TradeBase,
}

impl SpotTradeV3 {
    /// 派生本笔成交的买方支付、卖方所得和 base 交付金额。
    ///
    /// 费用均以 quote 计价；买方支付本金加买方费用，卖方所得为本金减卖方费用。
    /// 账户余额的实际变更由 settlement use case 编排。
    pub fn settlement_amounts(
        &self,
    ) -> Result<SpotTradeSettlementAmounts, SpotTradeSettlementError> {
        let trade = &self.comon_trade;
        if trade.price <= Decimal::ZERO || trade.size <= Decimal::ZERO {
            return Err(SpotTradeSettlementError::InvalidTradeAmount);
        }
        if trade.taker_fee < Decimal::ZERO || trade.maker_fee < Decimal::ZERO {
            return Err(SpotTradeSettlementError::InvalidFee);
        }

        let quote_notional = trade
            .price
            .checked_mul(trade.size)
            .ok_or(SpotTradeSettlementError::ArithmeticOverflow)?;
        let (buyer_fee, seller_fee) = match trade.taker_side {
            OrderSide::Buy => (trade.taker_fee, trade.maker_fee),
            OrderSide::Sell => (trade.maker_fee, trade.taker_fee),
        };
        let buyer_quote_payment = quote_notional
            .checked_add(buyer_fee)
            .ok_or(SpotTradeSettlementError::ArithmeticOverflow)?;
        let seller_quote_receipt = quote_notional
            .checked_sub(seller_fee)
            .ok_or(SpotTradeSettlementError::SellerFeeExceedsProceeds)?;

        Ok(SpotTradeSettlementAmounts {
            base_amount: trade.size,
            buyer_quote_payment,
            seller_quote_receipt,
        })
    }

    /// Compatibility alias for callers expressing the operation as settlement.
    pub fn settle_amounts(&self) -> Result<SpotTradeSettlementAmounts, SpotTradeSettlementError> {
        self.settlement_amounts()
    }

    /// Retained as a side-effect-free settlement entry point for API compatibility.
    pub fn settle(&self) -> Result<SpotTradeSettlementAmounts, SpotTradeSettlementError> {
        self.settlement_amounts()
    }
}

#[cfg(test)]
mod tests {
    use base_types::UserId;
    use rust_decimal::Decimal;

    use super::{SpotTradeSettlementError, SpotTradeV3};
    use crate::OrderSide;
    use crate::account::coin::Coin;
    use crate::account::symbol::Symbol;
    use crate::entity::common::common_trade::TradeBase;

    fn trade(taker_side: OrderSide, taker_fee: i64, maker_fee: i64) -> SpotTradeV3 {
        SpotTradeV3 {
            comon_trade: TradeBase {
                trade_id: 7,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                taker_user: UserId(10),
                maker_user: UserId(20),
                taker_order_id: 1,
                maker_order_id: 2,
                taker_side,
                price: Decimal::from(2),
                size: Decimal::from(3),
                taker_fee: Decimal::from(taker_fee),
                maker_fee: Decimal::from(maker_fee),
                created_at: 0,
                updated_at: 0,
                version: 0,
            },
        }
    }

    #[test]
    fn settlement_amounts_map_fees_for_taker_buy_and_sell() {
        let buy = trade(OrderSide::Buy, 1, 2).settlement_amounts().unwrap();
        assert_eq!(buy.base_amount, Decimal::from(3));
        assert_eq!(buy.buyer_quote_payment, Decimal::from(7));
        assert_eq!(buy.seller_quote_receipt, Decimal::from(4));

        let sell = trade(OrderSide::Sell, 1, 2).settlement_amounts().unwrap();
        assert_eq!(sell.buyer_quote_payment, Decimal::from(8));
        assert_eq!(sell.seller_quote_receipt, Decimal::from(5));
    }

    #[test]
    fn seller_fee_equal_to_or_greater_than_proceeds_is_handled() {
        assert_eq!(
            trade(OrderSide::Buy, 1, 6).settlement_amounts().unwrap().seller_quote_receipt,
            Decimal::ZERO
        );
        assert_eq!(
            trade(OrderSide::Buy, 1, 7).settlement_amounts(),
            Err(SpotTradeSettlementError::SellerFeeExceedsProceeds)
        );
    }
}
