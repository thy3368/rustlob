use crate::SpotOrderV7;
use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::SpotAmountConversionError;
use crate::entity::common::order_primitives::OrderSide;

impl SpotOrderV7 {
    /// 返回订单首次作为 taker 时应冻结的资产金额。
    ///
    /// 买单冻结原始数量对应的 quote 本金与创建时手续费预留之和；卖单冻结原始 base
    /// 数量。本查询只读取订单事实，不检查生命周期状态，也不修改订单。算术溢出返回
    /// [`SpotAmountConversionError::ArithmeticOverflow`]，结果超出币种精度时返回
    /// [`SpotAmountConversionError::InexactConversion`]。
    pub fn taker_freeze_amount(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let order = self.order_base();
        let (currency, amount) = match order.side {
            OrderSide::Buy => {
                let amount = order
                    .limit_px
                    .checked_mul(order.orig_size)
                    .and_then(|principal| principal.checked_add(order.buy_quote_fee_reserve_limit))
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
                (order.symbol.quote, amount)
            }
            OrderSide::Sell => (order.symbol.base, order.orig_size),
        };

        CoinAmount::new(currency, amount)
            .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::account::coin::Coin;
    use crate::account::symbol::Symbol;
    use crate::entity::common::UserId;
    use crate::entity::common::common_order::OrderBase;

    fn order(side: OrderSide) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(11),
            order_id: 7,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(2),
            orig_size: Decimal::from(2),
            buy_quote_fee_reserve_limit: Decimal::from(3),
            ..OrderBase::default()
        })
        .unwrap()
    }

    fn assert_query_preserves_order(
        order: &SpotOrderV7,
        expected: Result<CoinAmount, SpotAmountConversionError>,
    ) {
        let before = order.clone();

        assert_eq!(order.taker_freeze_amount(), expected);
        assert_eq!(order, &before);
    }

    #[test]
    fn buy_freezes_quote_principal_and_fee_reserve() {
        let order = order(OrderSide::Buy);

        assert_query_preserves_order(
            &order,
            Ok(CoinAmount::new(Coin::USDT, Decimal::from(203)).unwrap()),
        );
    }

    #[test]
    fn sell_freezes_original_base_quantity() {
        let order = order(OrderSide::Sell);

        assert_query_preserves_order(
            &order,
            Ok(CoinAmount::new(Coin::BTC, Decimal::from(2)).unwrap()),
        );
    }

    #[test]
    fn buy_with_zero_fee_reserve_freezes_quote_principal() {
        let mut record = order(OrderSide::Buy).order_base().clone();
        record.buy_quote_fee_reserve_limit = Decimal::ZERO;
        let order = SpotOrderV7::from_record(record).unwrap();

        assert_query_preserves_order(
            &order,
            Ok(CoinAmount::new(Coin::USDT, Decimal::from(200)).unwrap()),
        );
    }

    #[test]
    fn buy_reports_multiplication_overflow_without_mutation() {
        let mut record = order(OrderSide::Buy).order_base().clone();
        record.limit_px = Decimal::MAX;
        record.orig_size = Decimal::from(2);
        record.size = Decimal::from(2);
        record.buy_quote_fee_reserve_limit = Decimal::ZERO;
        let order = SpotOrderV7::from_record(record).unwrap();

        assert_query_preserves_order(&order, Err(SpotAmountConversionError::ArithmeticOverflow));
    }

    #[test]
    fn buy_reports_addition_overflow_without_mutation() {
        let mut record = order(OrderSide::Buy).order_base().clone();
        record.limit_px = Decimal::MAX;
        record.orig_size = Decimal::ONE;
        record.size = Decimal::ONE;
        record.buy_quote_fee_reserve_limit = Decimal::ONE;
        let order = SpotOrderV7::from_record(record).unwrap();

        assert_query_preserves_order(&order, Err(SpotAmountConversionError::ArithmeticOverflow));
    }
}
