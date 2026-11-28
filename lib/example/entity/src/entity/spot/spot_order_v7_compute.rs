use super::spot_order_v7::buy_total_held;
use crate::SpotOrderV7;
use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::common::OrderBase;
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
        let (currency, amount) = match order.side {
            OrderSide::Buy => {
                let amount = order
                    .limit_px
                    .checked_mul(order.orig_size)
                    .and_then(|principal| principal.checked_add(order.buy_quote_fee_reserve_frozen))
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
                (order.symbol.quote, amount)
            }
            OrderSide::Sell => (order.symbol.base, order.orig_size),
        };

        CoinAmount::new(currency, amount)
            .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
    }

    /// 返回本次终态迁移新增的解冻金额。
    pub fn terminal_unfreeze_delta(
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
        if matches!(
            before.status,
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) || !matches!(
            after.status,
            SpotOrderStatus::Filled | SpotOrderStatus::Canceled | SpotOrderStatus::Rejected
        ) {
            return Err(SpotAmountConversionError::OrderNotCanceled);
        }
        let amount = match after.side {
            OrderSide::Buy => after
                .buy_quote_principal_unfrozen
                .checked_sub(before.buy_quote_principal_unfrozen)
                .and_then(|v| {
                    after
                        .buy_quote_fee_reserve_unfrozen
                        .checked_sub(before.buy_quote_fee_reserve_unfrozen)
                        .and_then(|fee| v.checked_add(fee))
                })
                .filter(|v| *v >= rust_decimal::Decimal::ZERO)
                .ok_or(SpotAmountConversionError::InvalidAmount)?,
            OrderSide::Sell => after
                .sell_base_principal_unfrozen
                .checked_sub(before.sell_base_principal_unfrozen)
                .filter(|v| *v >= rust_decimal::Decimal::ZERO)
                .ok_or(SpotAmountConversionError::InvalidAmount)?,
        };
        CoinAmount::new(held_currency(after), amount)
            .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
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
            OrderSide::Buy => buy_total_held(order),
            OrderSide::Sell => order
                .sell_base_principal_frozen
                .checked_sub(
                    order
                        .orig_size
                        .checked_sub(order.size)
                        .ok_or(SpotAmountConversionError::ArithmeticOverflow)?,
                )
                .and_then(|v| v.checked_sub(order.sell_base_principal_unfrozen))
                .filter(|v| *v >= rust_decimal::Decimal::ZERO)
                .ok_or(SpotAmountConversionError::ArithmeticOverflow),
        };
        let delta = held(after)?
            .checked_sub(held(before)?)
            .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;
        CoinAmount::new(held_currency(after), delta)
            .map_err(|_: CoinAmountError| SpotAmountConversionError::InexactConversion)
    }
}

pub fn held_currency(o: &OrderBase) -> crate::account::coin::Coin {
    match o.side {
        OrderSide::Buy => o.symbol.quote,
        OrderSide::Sell => o.symbol.base,
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::account::coin::Coin;
    use crate::account::symbol::Symbol;
    use crate::entity::common::PartyId;
    use crate::entity::common::common_order::OrderBase;

    fn order(side: OrderSide) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: PartyId("11".to_owned()),
            order_id: 7,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(100),
            size: Decimal::from(2),
            orig_size: Decimal::from(2),
            buy_quote_fee_reserve_frozen: Decimal::from(3),
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
        after: &SpotOrderV7,
        before: &SpotOrderV7,
        expected: Result<CoinAmount, SpotAmountConversionError>,
    ) {
        let before_snapshot = before.clone();
        let after_snapshot = after.clone();

        assert_eq!(after.terminal_unfreeze_delta(before), expected);
        assert_eq!(before, &before_snapshot);
        assert_eq!(after, &after_snapshot);
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
        record.buy_quote_fee_reserve_frozen = Decimal::ZERO;
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
        record.buy_quote_fee_reserve_frozen = Decimal::ZERO;
        let order = SpotOrderV7::from_record(record).unwrap();

        assert_query_preserves_order(&order, Err(SpotAmountConversionError::ArithmeticOverflow));
    }

    #[test]
    fn buy_reports_addition_overflow_without_mutation() {
        let mut record = order(OrderSide::Buy).order_base().clone();
        record.limit_px = Decimal::MAX;
        record.orig_size = Decimal::ONE;
        record.size = Decimal::ONE;
        record.buy_quote_fee_reserve_frozen = Decimal::ONE;
        let order = SpotOrderV7::from_record(record).unwrap();

        assert_query_preserves_order(&order, Err(SpotAmountConversionError::ArithmeticOverflow));
    }

    #[test]
    fn terminal_release_reads_buy_sell_and_reject_facts_without_mutation() {
        let buy_before = order(OrderSide::Buy);
        let mut buy_after = buy_before.order_base().clone();
        buy_after.status = SpotOrderStatus::Canceled;
        buy_after.buy_quote_principal_unfrozen = Decimal::from(40);
        buy_after.buy_quote_fee_reserve_unfrozen = Decimal::from(2);
        let buy_after = SpotOrderV7::from_record(buy_after).unwrap();
        assert_terminal_query_preserves_order(
            &buy_after,
            &buy_before,
            Ok(CoinAmount::new(Coin::USDT, Decimal::from(42)).unwrap()),
        );

        let sell_before = order(OrderSide::Sell);
        let mut sell_after = sell_before.order_base().clone();
        sell_after.status = SpotOrderStatus::Filled;
        sell_after.size = Decimal::ZERO;
        let sell_after = SpotOrderV7::from_record(sell_after).unwrap();
        assert_terminal_query_preserves_order(
            &sell_after,
            &sell_before,
            Ok(CoinAmount::new(Coin::BTC, Decimal::ZERO).unwrap()),
        );

        let rejected_before = order(OrderSide::Sell);
        let mut rejected_after = rejected_before.order_base().clone();
        rejected_after.status = SpotOrderStatus::Rejected;
        rejected_after.size = Decimal::ZERO;
        let rejected_after = SpotOrderV7::from_record(rejected_after).unwrap();
        assert_terminal_query_preserves_order(
            &rejected_after,
            &rejected_before,
            Ok(CoinAmount::new(Coin::BTC, Decimal::from(2)).unwrap()),
        );
    }

    #[test]
    fn terminal_release_rejects_non_terminal_order() {
        let order = order(OrderSide::Buy);
        assert_terminal_query_preserves_order(
            &order,
            &order,
            Err(SpotAmountConversionError::OrderNotCanceled),
        );
    }

    #[test]
    fn terminal_release_reports_buy_addition_overflow_without_mutation() {
        let before = order(OrderSide::Buy);
        let mut record = before.order_base().clone();
        record.status = SpotOrderStatus::Canceled;
        record.buy_quote_principal_unfrozen = Decimal::MAX;
        record.buy_quote_fee_reserve_unfrozen = Decimal::ONE;
        let after = SpotOrderV7::from_record(record).unwrap();

        assert_terminal_query_preserves_order(
            &after,
            &before,
            Err(SpotAmountConversionError::ArithmeticOverflow),
        );
    }

    #[test]
    fn modify_delta_reports_increase_decrease_and_unchanged() {
        let mut before = order(OrderSide::Buy).order_base().clone();
        before.status = SpotOrderStatus::Open;
        let before = SpotOrderV7::from_record(before).unwrap();

        for (principal_frozen, principal_unfrozen, expected) in [
            (Decimal::from(300), Decimal::ZERO, Decimal::from(100)),
            (Decimal::from(200), Decimal::from(100), Decimal::from(-100)),
            (Decimal::from(200), Decimal::ZERO, Decimal::ZERO),
        ] {
            let mut after = before.order_base().clone();
            after.buy_quote_principal_frozen = principal_frozen;
            after.buy_quote_principal_unfrozen = principal_unfrozen;
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

        let mut wrong_symbol = before.order_base().clone();
        wrong_symbol.symbol = Symbol::new(2, Coin::ETH, Coin::USDT);
        assert_eq!(
            SpotOrderV7::from_record(wrong_symbol)
                .unwrap()
                .modify_freeze_delta(&before),
            Err(SpotAmountConversionError::SymbolMismatch)
        );

        let mut wrong_side = before.order_base().clone();
        wrong_side.side = OrderSide::Sell;
        assert_eq!(
            SpotOrderV7::from_record(wrong_side)
                .unwrap()
                .modify_freeze_delta(&before),
            Err(SpotAmountConversionError::SymbolMismatch)
        );

        let mut overflow = before.order_base().clone();
        overflow.buy_quote_principal_frozen = Decimal::MAX;
        overflow.buy_quote_fee_reserve_frozen = Decimal::ONE;
        assert_eq!(
            SpotOrderV7::from_record(overflow)
                .unwrap()
                .modify_freeze_delta(&before),
            Err(SpotAmountConversionError::ArithmeticOverflow)
        );

        let precise_before = before.clone();
        let mut inexact = precise_before.order_base().clone();
        inexact.buy_quote_principal_frozen = Decimal::new(2_000_000_001, 7);
        assert_eq!(
            SpotOrderV7::from_record(inexact)
                .unwrap()
                .modify_freeze_delta(&precise_before),
            Err(SpotAmountConversionError::InexactConversion)
        );
    }
}
