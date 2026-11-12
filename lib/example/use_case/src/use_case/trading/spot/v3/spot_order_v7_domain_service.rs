use common_entity::{EntityError, Fsm};
use example_core_entity::spot::spot_order_v7::{
    ActivateSpotOrderV7Output, CancelSpotOrderV7Output, ModifySpotOrderV7Output, SpotOrderV7Output,
};
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, OrderTif, SpotOrderV7, SpotOrderV7Event,
    SpotOrderV7HoldAdjustment,
};
use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ActivateSpotOrderV7Error {
    #[error("order id must be greater than zero")]
    InvalidOrderId,
    #[error("command order id does not match loaded order")]
    OrderMismatch,
    #[error("only pending orders can be activated")]
    OrderNotPending,
    #[error("balance owner does not match order owner")]
    OwnerMismatch,
    #[error("balance coin does not match freeze requirement")]
    CoinMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV7Error {
    #[error("balance asset does not match the order release asset")]
    AssetMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModifySpotOrderV7Error {
    #[error("balance asset does not match the order held asset")]
    AssetMismatch,
    #[error(transparent)]
    Balance(#[from] BalanceV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
}

pub struct SpotOrderV7DomainService;

impl SpotOrderV7DomainService {
    pub fn modify(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        price: Decimal,
        size: Decimal,
        tif: Option<OrderTif>,
        cloid: Option<String>,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2), ModifySpotOrderV7Error> {
        let (_, output) =
            order.apply(SpotOrderV7Event::Modify { price, size, tif, cloid, timestamp })?;
        let SpotOrderV7Output::Modified(ModifySpotOrderV7Output { order_after, hold_adjustment }) =
            output
        else {
            return Err(
                EntityError::Custom("modify event produced mismatched output".into()).into()
            );
        };

        if balance.coin_idx() != u64::from(hold_adjustment.currency().index()) {
            return Err(ModifySpotOrderV7Error::AssetMismatch);
        }
        match hold_adjustment {
            SpotOrderV7HoldAdjustment::Increase { amount } => {
                balance.apply(BalanceV2Event::Freeze { amount: amount.amount(), timestamp })?;
            }
            SpotOrderV7HoldAdjustment::Decrease { amount } => {
                balance.apply(BalanceV2Event::UnfreezeAndPay {
                    unfreeze_amount: amount.amount(),
                    payment_amount: Decimal::ZERO,
                    timestamp,
                })?;
            }
            SpotOrderV7HoldAdjustment::Unchanged { .. } => {}
        }

        Ok((order_after, balance))
    }

    pub fn freeze(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2), ActivateSpotOrderV7Error> {
        let (_, output) = order.apply(SpotOrderV7Event::Activate { timestamp })?;
        let SpotOrderV7Output::Activated(ActivateSpotOrderV7Output { freeze_requirement }) = output
        else {
            return Err(
                EntityError::Custom("activate event produced mismatched output".into()).into()
            );
        };

        if balance.coin_idx() != u64::from(freeze_requirement.currency().index()) {
            return Err(ActivateSpotOrderV7Error::CoinMismatch);
        }
        balance.apply(BalanceV2Event::Freeze { amount: freeze_requirement.amount(), timestamp })?;

        Ok((order, balance))
    }

    pub fn cancel(
        mut order: SpotOrderV7,
        mut balance: BalanceV2,
        timestamp: u64,
    ) -> Result<(SpotOrderV7, BalanceV2), CancelSpotOrderV7Error> {
        let (_, output) = order.apply(SpotOrderV7Event::Cancel { timestamp })?;
        let SpotOrderV7Output::Canceled(CancelSpotOrderV7Output { release_requirement, .. }) =
            output
        else {
            return Err(
                EntityError::Custom("cancel event produced mismatched output".into()).into()
            );
        };

        if balance.coin_idx() != u64::from(release_requirement.currency().index()) {
            return Err(CancelSpotOrderV7Error::AssetMismatch);
        }
        if release_requirement.amount() > Decimal::ZERO {
            balance.apply(BalanceV2Event::UnfreezeAndPay {
                unfreeze_amount: release_requirement.amount(),
                payment_amount: Decimal::ZERO,
                timestamp,
            })?;
        }

        Ok((order, balance))
    }
}

#[cfg(test)]
mod tests {
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::entity::common::{OrderBase, UserId};
    use example_core_entity::{OrderSide, SpotOrderStatus};

    use super::*;

    const OWNER_ID: u64 = 11;
    const ACCOUNT_ID: u64 = 22;
    const TIMESTAMP: u64 = 500;

    fn order(
        side: OrderSide,
        status: SpotOrderStatus,
        remaining_size: i64,
        fee_reserve: i64,
        fee_consumed: i64,
    ) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(OWNER_ID),
            order_id: 7,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(10),
            size: Decimal::from(remaining_size),
            orig_size: Decimal::from(3),
            status,
            buy_quote_fee_reserve_limit: Decimal::from(fee_reserve),
            buy_quote_fee_reserve_consumed: Decimal::from(fee_consumed),
            updated_at: 100,
            version: 4,
            ..OrderBase::default()
        })
        .unwrap()
    }

    fn balance(coin: Coin, available: i64, frozen: i64) -> BalanceV2 {
        BalanceV2::new(OWNER_ID, ACCOUNT_ID, u64::from(coin.index()), available, frozen, 9, 50)
    }

    #[test]
    fn modify_pending_order_updates_order_and_leaves_balance_unchanged() {
        let order = order(OrderSide::Buy, SpotOrderStatus::Pending, 3, 2, 0);
        let balance = balance(Coin::USDT, 20, 32);
        let balance_before = balance.clone();

        let (order_after, balance_after) = SpotOrderV7DomainService::modify(
            order,
            balance,
            Decimal::from(12),
            Decimal::from(4),
            Some(OrderTif::Alo),
            Some("replacement".to_owned()),
            TIMESTAMP,
        )
        .unwrap();

        assert_eq!(order_after.order_base().limit_px, Decimal::from(12));
        assert_eq!(order_after.order_base().size, Decimal::from(4));
        assert_eq!(order_after.order_base().orig_size, Decimal::from(4));
        assert_eq!(order_after.order_base().tif, OrderTif::Alo);
        assert_eq!(order_after.order_base().cloid.as_deref(), Some("replacement"));
        assert_eq!(order_after.order_base().version, 5);
        assert_eq!(order_after.order_base().updated_at, TIMESTAMP);
        assert_eq!(balance_after, balance_before);
    }

    #[test]
    fn modify_open_buy_increases_remaining_principal_and_preserves_unused_fee_reserve() {
        let order = order(OrderSide::Buy, SpotOrderStatus::Open, 3, 5, 2);
        let balance = balance(Coin::USDT, 30, 33);

        let (_, balance_after) = SpotOrderV7DomainService::modify(
            order,
            balance,
            Decimal::from(12),
            Decimal::from(3),
            None,
            None,
            TIMESTAMP,
        )
        .unwrap();

        assert_eq!(balance_after.available(), Decimal::from(24));
        assert_eq!(balance_after.frozen(), Decimal::from(39));
        assert_eq!(balance_after.version(), 10);
        assert_eq!(balance_after.updated_at(), TIMESTAMP);
    }

    #[test]
    fn modify_open_buy_decreases_freeze_requirement() {
        let order = order(OrderSide::Buy, SpotOrderStatus::Open, 3, 5, 2);
        let balance = balance(Coin::USDT, 4, 33);

        let (_, balance_after) = SpotOrderV7DomainService::modify(
            order,
            balance,
            Decimal::from(8),
            Decimal::from(2),
            None,
            None,
            TIMESTAMP,
        )
        .unwrap();

        assert_eq!(balance_after.available(), Decimal::from(18));
        assert_eq!(balance_after.frozen(), Decimal::from(19));
    }

    #[test]
    fn modify_partially_filled_buy_uses_only_remaining_size() {
        let order = order(OrderSide::Buy, SpotOrderStatus::PartiallyFilled, 1, 5, 2);
        let balance = balance(Coin::USDT, 20, 13);

        let (order_after, balance_after) = SpotOrderV7DomainService::modify(
            order,
            balance,
            Decimal::from(12),
            Decimal::from(3),
            None,
            None,
            TIMESTAMP,
        )
        .unwrap();

        assert_eq!(order_after.order_base().size, Decimal::ONE);
        assert_eq!(balance_after.available(), Decimal::from(18));
        assert_eq!(balance_after.frozen(), Decimal::from(15));
    }

    #[test]
    fn modify_sell_adjusts_only_remaining_base_quantity() {
        let increased = SpotOrderV7DomainService::modify(
            order(OrderSide::Sell, SpotOrderStatus::Open, 3, 0, 0),
            balance(Coin::BTC, 5, 3),
            Decimal::from(99),
            Decimal::from(4),
            None,
            None,
            TIMESTAMP,
        )
        .unwrap()
        .1;
        assert_eq!(increased.available(), Decimal::from(4));
        assert_eq!(increased.frozen(), Decimal::from(4));

        let decreased = SpotOrderV7DomainService::modify(
            order(OrderSide::Sell, SpotOrderStatus::Open, 3, 0, 0),
            balance(Coin::BTC, 5, 3),
            Decimal::ONE,
            Decimal::from(2),
            None,
            None,
            TIMESTAMP,
        )
        .unwrap()
        .1;
        assert_eq!(decreased.available(), Decimal::from(6));
        assert_eq!(decreased.frozen(), Decimal::from(2));
    }

    #[test]
    fn modify_unchanged_freeze_requirement_leaves_balance_metadata_unchanged() {
        let balance = balance(Coin::USDT, 7, 33);
        let balance_before = balance.clone();

        let (order_after, balance_after) = SpotOrderV7DomainService::modify(
            order(OrderSide::Buy, SpotOrderStatus::Open, 3, 5, 2),
            balance,
            Decimal::from(15),
            Decimal::from(2),
            Some(OrderTif::Ioc),
            None,
            TIMESTAMP,
        )
        .unwrap();

        assert_eq!(order_after.order_base().tif, OrderTif::Ioc);
        assert_eq!(balance_after, balance_before);
    }

    #[test]
    fn modify_rejects_asset_mismatch() {
        let result = SpotOrderV7DomainService::modify(
            order(OrderSide::Buy, SpotOrderStatus::Pending, 3, 0, 0),
            balance(Coin::BTC, 10, 0),
            Decimal::ONE,
            Decimal::ONE,
            None,
            None,
            TIMESTAMP,
        );

        assert_eq!(result, Err(ModifySpotOrderV7Error::AssetMismatch));
    }

    #[test]
    fn modify_propagates_balance_errors_without_partial_result() {
        let insufficient_available = SpotOrderV7DomainService::modify(
            order(OrderSide::Buy, SpotOrderStatus::Open, 3, 0, 0),
            balance(Coin::USDT, 1, 30),
            Decimal::from(11),
            Decimal::from(3),
            None,
            None,
            TIMESTAMP,
        );
        assert_eq!(
            insufficient_available,
            Err(ModifySpotOrderV7Error::Balance(BalanceV2Error::InsufficientAvailableBalance))
        );

        let insufficient_frozen = SpotOrderV7DomainService::modify(
            order(OrderSide::Sell, SpotOrderStatus::Open, 2, 0, 0),
            balance(Coin::BTC, 0, 1),
            Decimal::ONE,
            Decimal::ONE,
            None,
            None,
            TIMESTAMP,
        );
        assert_eq!(
            insufficient_frozen,
            Err(ModifySpotOrderV7Error::Balance(BalanceV2Error::InsufficientFrozenBalance))
        );
    }

    #[test]
    fn modify_propagates_invalid_values_and_terminal_state_errors() {
        for (price, size, status) in [
            (Decimal::ZERO, Decimal::ONE, SpotOrderStatus::Open),
            (Decimal::ONE, Decimal::ZERO, SpotOrderStatus::Open),
            (Decimal::ONE, Decimal::ONE, SpotOrderStatus::Canceled),
        ] {
            let result = SpotOrderV7DomainService::modify(
                order(OrderSide::Sell, status, 1, 0, 0),
                balance(Coin::BTC, 0, 1),
                price,
                size,
                None,
                None,
                TIMESTAMP,
            );
            assert!(matches!(result, Err(ModifySpotOrderV7Error::Entity(_))));
        }
    }

    #[test]
    fn cancel_pending_order_leaves_balance_unchanged() {
        let order = order(OrderSide::Buy, SpotOrderStatus::Pending, 3, 2, 0);
        let balance = balance(Coin::USDT, 20, 32);
        let balance_before = balance.clone();

        let (order_after, balance_after) =
            SpotOrderV7DomainService::cancel(order, balance, TIMESTAMP).unwrap();

        assert_eq!(order_after.status(), SpotOrderStatus::Canceled);
        assert_eq!(order_after.order_base().version, 5);
        assert_eq!(order_after.order_base().updated_at, TIMESTAMP);
        assert_eq!(balance_after, balance_before);
    }

    #[test]
    fn cancel_open_buy_releases_remaining_principal_and_unused_fee_reserve() {
        let order = order(OrderSide::Buy, SpotOrderStatus::Open, 2, 5, 2);
        let balance = balance(Coin::USDT, 7, 23);

        let (order_after, balance_after) =
            SpotOrderV7DomainService::cancel(order, balance, TIMESTAMP).unwrap();

        assert_eq!(order_after.status(), SpotOrderStatus::Canceled);
        assert_eq!(order_after.order_base().version, 5);
        assert_eq!(order_after.order_base().updated_at, TIMESTAMP);
        assert_eq!(balance_after.available(), Decimal::from(30));
        assert_eq!(balance_after.frozen(), Decimal::ZERO);
        assert_eq!(balance_after.version(), 10);
        assert_eq!(balance_after.updated_at(), TIMESTAMP);
    }

    #[test]
    fn cancel_partially_filled_sell_releases_only_remaining_base_quantity() {
        let order = order(OrderSide::Sell, SpotOrderStatus::PartiallyFilled, 1, 0, 0);
        let balance = balance(Coin::BTC, 4, 1);

        let (_, balance_after) =
            SpotOrderV7DomainService::cancel(order, balance, TIMESTAMP).unwrap();

        assert_eq!(balance_after.available(), Decimal::from(5));
        assert_eq!(balance_after.frozen(), Decimal::ZERO);
        assert_eq!(balance_after.version(), 10);
        assert_eq!(balance_after.updated_at(), TIMESTAMP);
    }

    #[test]
    fn cancel_rejects_release_asset_mismatch() {
        let result = SpotOrderV7DomainService::cancel(
            order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0),
            balance(Coin::BTC, 0, 20),
            TIMESTAMP,
        );

        assert_eq!(result, Err(CancelSpotOrderV7Error::AssetMismatch));
    }

    #[test]
    fn cancel_rejects_insufficient_frozen_balance_without_partial_result() {
        let result = SpotOrderV7DomainService::cancel(
            order(OrderSide::Buy, SpotOrderStatus::Open, 2, 5, 2),
            balance(Coin::USDT, 7, 22),
            TIMESTAMP,
        );

        assert_eq!(
            result,
            Err(CancelSpotOrderV7Error::Balance(BalanceV2Error::InsufficientFrozenBalance))
        );
    }

    #[test]
    fn cancel_propagates_terminal_order_error() {
        let result = SpotOrderV7DomainService::cancel(
            order(OrderSide::Sell, SpotOrderStatus::Canceled, 1, 0, 0),
            balance(Coin::BTC, 0, 1),
            TIMESTAMP,
        );

        assert!(matches!(result, Err(CancelSpotOrderV7Error::Entity(_))));
    }
}
