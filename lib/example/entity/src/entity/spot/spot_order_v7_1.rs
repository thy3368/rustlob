use crate::SpotOrderV7;
use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::SpotAmountConversionError;
use crate::entity::common::order_primitives::{OrderSide, SpotOrderStatus};

impl SpotOrderV7 {
    /// 返回订单首次作为 taker 时应冻结的资产金额。
    ///
    /// 买单冻结原始数量对应的 quote 本金与创建时手续费预留之和；卖单冻结原始 base
    /// 数量。本查询只读取订单事实，不检查生命周期状态，也不修改订单。算术溢出返回
    /// [`SpotAmountConversionError::ArithmeticOverflow`]，结果超出币种精度时返回
    /// [`SpotAmountConversionError::InexactConversion`]。
    pub fn taker_freeze_amount(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let order = self.order_base();
        if order.orig_size <= rust_decimal::Decimal::ZERO
            || order.buy_quote_fee_reserve_limit < rust_decimal::Decimal::ZERO
        {
            return Err(SpotAmountConversionError::InvalidAmount);
        }
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

    /// 返回订单进入终态时应释放的累计冻结金额。
    ///
    /// 仅接受 `Canceled`、`Filled` 或 `Rejected` 订单。买单返回累计释放的 quote
    /// 本金与手续费预留，卖单返回终态剩余 base 数量。本查询不修改订单。
    pub fn terminal_release_amount(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        if !matches!(
            self.status(),
            SpotOrderStatus::Canceled | SpotOrderStatus::Filled | SpotOrderStatus::Rejected
        ) {
            return Err(SpotAmountConversionError::OrderNotCanceled);
        }
        super::spot_order_v7::terminal_release_amount(
            self.order_base(),
            self.status() == SpotOrderStatus::Rejected,
        )
    }

    /// 返回改单导致的冻结金额有符号差额。
    ///
    /// `self` 是改单后的订单，`before` 是同一订单的改单前事实。正数表示追加冻结，
    /// 负数表示释放，零表示余额不变。两个订单必须身份、市场、方向一致，且都处于
    /// `Open` 或 `PartiallyFilled`；算术溢出或币种精度错误会显式返回错误。
    pub fn modify_freeze_delta(
        &self,
        before: &Self,
    ) -> Result<CoinAmount, SpotAmountConversionError> {
        let after = self.order_base();
        let before = before.order_base();
        if after.order_id != before.order_id || after.owner != before.owner {
            return Err(SpotAmountConversionError::InvalidAmount);
        }
        if after.symbol != before.symbol || after.side != before.side {
            return Err(SpotAmountConversionError::SymbolMismatch);
        }
        if !matches!(
            before.status,
            SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) || !matches!(
            after.status,
            SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) {
            return Err(SpotAmountConversionError::InvalidAmount);
        }
        let held = |order: &crate::entity::common::common_order::OrderBase| match order.side {
            OrderSide::Buy => order
                .limit_px
                .checked_mul(order.size)
                .and_then(|value| value.checked_add(order.buy_quote_fee_reserve_limit))
                .and_then(|value| value.checked_sub(order.buy_quote_fee_reserve_consumed))
                .ok_or(SpotAmountConversionError::ArithmeticOverflow),
            OrderSide::Sell => Ok(order.size),
        };
        let delta = held(after)?
            .checked_sub(held(before)?)
            .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
        CoinAmount::new(super::spot_order_v7::held_currency(after), delta)
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

    fn assert_terminal_query_preserves_order(
        order: &SpotOrderV7,
        expected: Result<CoinAmount, SpotAmountConversionError>,
    ) {
        let before = order.clone();
        assert_eq!(order.terminal_release_amount(), expected);
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

    #[test]
    fn terminal_release_reads_buy_sell_and_reject_facts_without_mutation() {
        let mut buy = order(OrderSide::Buy).order_base().clone();
        buy.status = SpotOrderStatus::Canceled;
        buy.buy_quote_principal_released = Decimal::from(40);
        buy.buy_quote_fee_reserve_released = Decimal::from(2);
        let buy = SpotOrderV7::from_record(buy).unwrap();
        assert_terminal_query_preserves_order(
            &buy,
            Ok(CoinAmount::new(Coin::USDT, Decimal::from(42)).unwrap()),
        );

        let mut sell = order(OrderSide::Sell).order_base().clone();
        sell.status = SpotOrderStatus::Filled;
        sell.size = Decimal::ZERO;
        let sell = SpotOrderV7::from_record(sell).unwrap();
        assert_terminal_query_preserves_order(
            &sell,
            Ok(CoinAmount::new(Coin::BTC, Decimal::ZERO).unwrap()),
        );

        let mut rejected = order(OrderSide::Sell).order_base().clone();
        rejected.status = SpotOrderStatus::Rejected;
        rejected.size = Decimal::ZERO;
        let rejected = SpotOrderV7::from_record(rejected).unwrap();
        assert_terminal_query_preserves_order(
            &rejected,
            Ok(CoinAmount::new(Coin::BTC, Decimal::from(2)).unwrap()),
        );
    }

    #[test]
    fn terminal_release_rejects_non_terminal_order() {
        assert_terminal_query_preserves_order(
            &order(OrderSide::Buy),
            Err(SpotAmountConversionError::OrderNotCanceled),
        );
    }

    #[test]
    fn modify_delta_reports_increase_decrease_and_unchanged() {
        let mut before = order(OrderSide::Buy).order_base().clone();
        before.status = SpotOrderStatus::Open;
        let before = SpotOrderV7::from_record(before).unwrap();

        for (size, expected) in [
            (Decimal::from(3), Decimal::from(100)),
            (Decimal::ONE, Decimal::from(-100)),
            (Decimal::from(2), Decimal::ZERO),
        ] {
            let mut after = before.order_base().clone();
            after.size = size;
            after.orig_size = size;
            let after = SpotOrderV7::from_record(after).unwrap();
            let before_snapshot = before.clone();
            let after_snapshot = after.clone();
            assert_eq!(
                after.modify_freeze_delta(&before).unwrap(),
                CoinAmount::new(Coin::USDT, expected).unwrap()
            );
            assert_eq!(before, before_snapshot);
            assert_eq!(after, after_snapshot);
        }
    }

    #[test]
    fn modify_delta_validates_identity_market_state_precision_and_overflow() {
        let mut before = order(OrderSide::Buy).order_base().clone();
        before.status = SpotOrderStatus::Open;
        let before = SpotOrderV7::from_record(before).unwrap();

        let mut wrong_id = before.order_base().clone();
        wrong_id.order_id += 1;
        assert_eq!(
            SpotOrderV7::from_record(wrong_id)
                .unwrap()
                .modify_freeze_delta(&before),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let mut pending = before.order_base().clone();
        pending.status = SpotOrderStatus::Pending;
        assert_eq!(
            SpotOrderV7::from_record(pending)
                .unwrap()
                .modify_freeze_delta(&before),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let mut overflow = before.order_base().clone();
        overflow.limit_px = Decimal::MAX;
        overflow.size = Decimal::from(2);
        assert_eq!(
            SpotOrderV7::from_record(overflow)
                .unwrap()
                .modify_freeze_delta(&before),
            Err(SpotAmountConversionError::ArithmeticOverflow)
        );

        let mut precise_before = before.order_base().clone();
        precise_before.limit_px = Decimal::ONE;
        let precise_before = SpotOrderV7::from_record(precise_before).unwrap();
        let mut inexact = precise_before.order_base().clone();
        inexact.size = Decimal::new(20_000_001, 7);
        inexact.orig_size = inexact.size;
        assert_eq!(
            SpotOrderV7::from_record(inexact)
                .unwrap()
                .modify_freeze_delta(&precise_before),
            Err(SpotAmountConversionError::InexactConversion)
        );
    }
}
