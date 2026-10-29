use rust_decimal::Decimal;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::{OrderBase, SpotAmountConversionError};
use crate::entity::spot::spot_order_primitives::OrderSide;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3 {
    pub comon_order: OrderBase,
}

impl SpotOrderV3 {
    /// 返回订单需要预冻结的单币种金额，不修改订单状态。
    ///
    /// 买单冻结 quote 币的原始数量限价金额及最大 taker 手续费；卖单冻结
    /// base 币原始数量，手续费从卖出所得中支付。
    pub fn freeze_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let order = &self.comon_order;
        let orig_size = order.orig_size;
        let fee = order.taker_max_fee;

        if orig_size <= Decimal::ZERO || fee < Decimal::ZERO {
            return Err(SpotAmountConversionError::InvalidAmount);
        }

        match order.side {
            OrderSide::Buy => {
                if order.limit_px <= Decimal::ZERO {
                    return Err(SpotAmountConversionError::InvalidAmount);
                }

                let amount = order
                    .limit_px
                    .checked_mul(orig_size)
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?
                    .checked_add(fee)
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;

                CoinAmount::new(order.symbol.quote, amount).map_err(map_coin_amount_error)
            }
            OrderSide::Sell => {
                CoinAmount::new(order.symbol.base, orig_size).map_err(map_coin_amount_error)
            }
        }
    }
}

fn map_coin_amount_error(_: CoinAmountError) -> SpotAmountConversionError {
    SpotAmountConversionError::InexactConversion
}

#[cfg(test)]
mod tests {
    use base_types::UserId;
    use rust_decimal::Decimal;

    use super::SpotOrderV3;
    use crate::account::coin::{Coin, CoinAmount};
    use crate::account::symbol::Symbol;
    use crate::entity::common::common_order::{
        CommonOrderExecution, CommonOrderTrigger, OrderBase, SpotAmountConversionError,
    };
    use crate::entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
    use crate::entity::spot::spot_order_v2::OrderGroupRelation;

    fn order(side: OrderSide, limit_px: Decimal, orig_size: Decimal, fee: Decimal) -> SpotOrderV3 {
        SpotOrderV3 {
            comon_order: OrderBase {
                owner: UserId(1),
                order_id: 1,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                side,
                limit_px,
                size: Decimal::ZERO,
                orig_size,
                execution: CommonOrderExecution::Limit,
                tif: OrderTif::Gtc,
                trigger: CommonOrderTrigger::None,
                trigger_px: None,
                group_relation: OrderGroupRelation::Standalone,
                is_position_tpsl: false,
                reduce_only: false,
                status: SpotOrderStatus::Open,
                taker_max_fee: fee,
                cloid: None,
                created_at: 0,
                updated_at: 0,
                version: 0,
            },
        }
    }

    #[test]
    fn buy_freezes_original_quote_notional_plus_fee() {
        let order =
            order(OrderSide::Buy, Decimal::new(250, 2), Decimal::new(3, 0), Decimal::new(25, 2));

        assert_eq!(
            order.freeze_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(775, 2)).unwrap())
        );
    }

    #[test]
    fn sell_freezes_original_base_size_independent_of_quote_fee() {
        let order =
            order(OrderSide::Sell, Decimal::new(250, 2), Decimal::new(3, 0), Decimal::new(900, 0));

        assert_eq!(
            order.freeze_requirement(),
            Ok(CoinAmount::new(Coin::BTC, Decimal::new(3, 0)).unwrap())
        );
    }

    #[test]
    fn rejects_non_positive_size_negative_fee_and_non_positive_buy_price() {
        let invalid_size = order(OrderSide::Sell, Decimal::ONE, Decimal::ZERO, Decimal::ZERO);
        assert_eq!(
            invalid_size.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let negative_size =
            order(OrderSide::Buy, Decimal::ONE, Decimal::NEGATIVE_ONE, Decimal::ZERO);
        assert_eq!(
            negative_size.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let negative_fee =
            order(OrderSide::Sell, Decimal::ONE, Decimal::ONE, Decimal::NEGATIVE_ONE);
        assert_eq!(
            negative_fee.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        for price in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            let invalid_price = order(OrderSide::Buy, price, Decimal::ONE, Decimal::ZERO);
            assert_eq!(
                invalid_price.freeze_requirement(),
                Err(SpotAmountConversionError::InvalidAmount)
            );
        }
    }

    #[test]
    fn enforces_coin_precision_for_freeze_amount() {
        let exact = order(OrderSide::Buy, Decimal::new(123, 2), Decimal::ONE, Decimal::ZERO);
        assert_eq!(
            exact.freeze_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(123, 2)).unwrap())
        );

        let excessive = order(OrderSide::Buy, Decimal::new(1, 7), Decimal::ONE, Decimal::ZERO);
        assert_eq!(
            excessive.freeze_requirement(),
            Err(SpotAmountConversionError::InexactConversion)
        );
    }

    #[test]
    fn reports_checked_decimal_multiplication_overflow() {
        let order = order(OrderSide::Buy, Decimal::MAX, Decimal::MAX, Decimal::ZERO);

        assert_eq!(order.freeze_requirement(), Err(SpotAmountConversionError::ArithmeticOverflow));
    }

    #[test]
    fn reports_checked_decimal_addition_overflow() {
        let order = order(OrderSide::Buy, Decimal::MAX, Decimal::ONE, Decimal::ONE);

        assert_eq!(order.freeze_requirement(), Err(SpotAmountConversionError::ArithmeticOverflow));
    }
}
