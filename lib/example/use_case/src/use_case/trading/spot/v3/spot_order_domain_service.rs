use std::collections::HashSet;

use common_entity::{EntityError, Fsm};
use example_core_entity::entity::common::common_trade::TradeBase;
use example_core_entity::spot::spot_order_v7::{
    ActivateSpotOrderV7Output, CancelSpotOrderV7Output, ModifySpotOrderV7Output, SpotOrderV7Output,
};
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3};
use example_core_entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, OrderSide, OrderTif, SpotOrderStatus, SpotOrderV7,
    SpotOrderV7Event, SpotOrderV7HoldAdjustment,
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

pub struct SpotOrderDomainService;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderV3Input {
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderV3Outcome {
    Resting {
        taker_order_after: SpotOrderV7,
    },
    PartiallyFilled {
        taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
    },
    Filled {
        filled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
        release_requirement: example_core_entity::account::coin::CoinAmount,
    },
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        created_trades: Vec<SpotTradeV3>,
        release_requirement: example_core_entity::account::coin::CoinAmount,
    },
    Rejected {
        rejected_taker_order_after: SpotOrderV7,
        release_requirement: example_core_entity::account::coin::CoinAmount,
    },
}

impl SpotOrderDomainService {
    /// 只读规划全部成交，验证成功后分别推进 taker 与 maker 聚合。
    pub fn match_order(
        mut taker: SpotOrderV7,
        mut makers: Vec<SpotOrderV7>,
        input: MatchSpotOrderV3Input,
    ) -> Result<MatchSpotOrderV3Outcome, EntityError> {
        let t = taker.order_base().clone();
        taker.validate_matchable_as_taker()?;

        let mut ids = HashSet::with_capacity(makers.len() + 1);
        ids.insert(t.order_id);
        for maker in &makers {
            let m = maker.order_base();
            maker.validate_matchable_as_maker()?;
            if !ids.insert(m.order_id) {
                return Err(match_error("order ids must be unique"));
            }
            if m.symbol != t.symbol || m.side == t.side {
                return Err(match_error("maker market or side mismatch"));
            }
        }

        let crosses = |maker: &SpotOrderV7| match t.side {
            OrderSide::Buy => t.limit_px >= maker.order_base().limit_px,
            OrderSide::Sell => t.limit_px <= maker.order_base().limit_px,
        };
        if t.effective_tif() == OrderTif::Alo && makers.iter().any(crosses) {
            taker.apply(SpotOrderV7Event::RejectedAsTaker { timestamp: input.timestamp })?;
            let release_requirement = taker.release_requirement().map_err(conversion_error)?;
            return Ok(MatchSpotOrderV3Outcome::Rejected {
                rejected_taker_order_after: taker,
                release_requirement,
            });
        }

        #[derive(Clone, Copy)]
        struct PlannedFill {
            maker_index: usize,
            size: Decimal,
            taker_fee: Decimal,
            maker_fee: Decimal,
            trade_id: u64,
        }

        let mut remaining = t.size;
        let mut taker_fee_total = Decimal::ZERO;
        let mut fills = Vec::new();
        for (maker_index, maker) in makers.iter().enumerate() {
            if remaining == Decimal::ZERO {
                break;
            }
            if !crosses(maker) {
                continue;
            }
            let size = remaining.min(maker.order_base().size);
            let notional = maker
                .order_base()
                .limit_px
                .checked_mul(size)
                .ok_or_else(|| match_error("notional overflow"))?;
            let taker_fee = match_fee(notional, input.taker_fee_bps, t.symbol.quote.scale())?;
            let maker_fee = match_fee(notional, input.maker_fee_bps, t.symbol.quote.scale())?;
            taker_fee_total = taker_fee_total
                .checked_add(taker_fee)
                .ok_or_else(|| match_error("fee overflow"))?;
            let trade_id = input
                .trade_id_start
                .checked_add(fills.len() as u64)
                .ok_or_else(|| match_error("trade id overflow"))?;
            remaining = remaining.checked_sub(size).ok_or_else(|| match_error("size underflow"))?;
            fills.push(PlannedFill { maker_index, size, taker_fee, maker_fee, trade_id });
        }

        let filled_size =
            t.size.checked_sub(remaining).ok_or_else(|| match_error("size underflow"))?;
        if filled_size == Decimal::ZERO && t.effective_tif() != OrderTif::Ioc {
            return Ok(MatchSpotOrderV3Outcome::Resting { taker_order_after: taker });
        }

        let mut trades = Vec::with_capacity(fills.len());
        for fill in fills {
            let maker = makers
                .get_mut(fill.maker_index)
                .ok_or_else(|| match_error("planned maker index is missing"))?;
            let maker_before = maker.order_base().clone();
            maker.apply(SpotOrderV7Event::MatchedAsMaker {
                filled_size: fill.size,
                quote_fee: fill.maker_fee,
                timestamp: input.timestamp,
            })?;
            trades.push(SpotTradeV3 {
                comon_trade: TradeBase {
                    trade_id: fill.trade_id,
                    symbol: t.symbol,
                    taker_user: t.owner,
                    maker_user: maker_before.owner,
                    taker_order_id: t.order_id,
                    maker_order_id: maker_before.order_id,
                    taker_side: t.side,
                    price: maker_before.limit_px,
                    size: fill.size,
                    taker_fee: fill.taker_fee,
                    maker_fee: fill.maker_fee,
                    created_at: input.timestamp,
                    updated_at: input.timestamp,
                    version: 0,
                },
                settlement_status: SpotTradeSettlementStatus::Unsettled,
            });
        }
        taker.apply(SpotOrderV7Event::MatchedAsTaker {
            filled_size,
            quote_fee: taker_fee_total,
            timestamp: input.timestamp,
        })?;

        match taker.status() {
            SpotOrderStatus::Filled => {
                let release_requirement = taker.release_requirement().map_err(conversion_error)?;
                Ok(MatchSpotOrderV3Outcome::Filled {
                    filled_taker_order_after: taker,
                    maker_orders_after: makers,
                    created_trades: trades,
                    release_requirement,
                })
            }
            SpotOrderStatus::Canceled => {
                let release_requirement = taker.release_requirement().map_err(conversion_error)?;
                Ok(MatchSpotOrderV3Outcome::CanceledAfterPartialFill {
                    canceled_taker_order_after: taker,
                    maker_orders_after: makers,
                    created_trades: trades,
                    release_requirement,
                })
            }
            SpotOrderStatus::Rejected => {
                let release_requirement = taker.release_requirement().map_err(conversion_error)?;
                Ok(MatchSpotOrderV3Outcome::Rejected {
                    rejected_taker_order_after: taker,
                    release_requirement,
                })
            }
            SpotOrderStatus::PartiallyFilled => Ok(MatchSpotOrderV3Outcome::PartiallyFilled {
                taker_order_after: taker,
                maker_orders_after: makers,
                created_trades: trades,
            }),
            _ => Err(match_error("unexpected taker state after matching")),
        }
    }

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

fn match_fee(notional: Decimal, bps: u64, scale: u32) -> Result<Decimal, EntityError> {
    notional
        .checked_mul(Decimal::from(bps))
        .and_then(|value| value.checked_div(Decimal::from(10_000u64)))
        .map(|value| {
            value.round_dp_with_strategy(scale, rust_decimal::RoundingStrategy::ToPositiveInfinity)
        })
        .ok_or_else(|| match_error("fee overflow"))
}

fn match_error(message: &str) -> EntityError {
    EntityError::Custom(message.to_owned())
}

fn conversion_error(error: impl std::fmt::Display) -> EntityError {
    match_error(&error.to_string())
}

#[cfg(test)]
mod tests {
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::PriceType;
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

        let (order_after, balance_after) = SpotOrderDomainService::modify(
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

        let (_, balance_after) = SpotOrderDomainService::modify(
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

        let (_, balance_after) = SpotOrderDomainService::modify(
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

        let (order_after, balance_after) = SpotOrderDomainService::modify(
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
        let increased = SpotOrderDomainService::modify(
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

        let decreased = SpotOrderDomainService::modify(
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

        let (order_after, balance_after) = SpotOrderDomainService::modify(
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
        let result = SpotOrderDomainService::modify(
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
        let insufficient_available = SpotOrderDomainService::modify(
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

        let insufficient_frozen = SpotOrderDomainService::modify(
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
            let result = SpotOrderDomainService::modify(
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
            SpotOrderDomainService::cancel(order, balance, TIMESTAMP).unwrap();

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
            SpotOrderDomainService::cancel(order, balance, TIMESTAMP).unwrap();

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

        let (_, balance_after) = SpotOrderDomainService::cancel(order, balance, TIMESTAMP).unwrap();

        assert_eq!(balance_after.available(), Decimal::from(5));
        assert_eq!(balance_after.frozen(), Decimal::ZERO);
        assert_eq!(balance_after.version(), 10);
        assert_eq!(balance_after.updated_at(), TIMESTAMP);
    }

    #[test]
    fn cancel_rejects_release_asset_mismatch() {
        let result = SpotOrderDomainService::cancel(
            order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0),
            balance(Coin::BTC, 0, 20),
            TIMESTAMP,
        );

        assert_eq!(result, Err(CancelSpotOrderV7Error::AssetMismatch));
    }

    #[test]
    fn cancel_rejects_insufficient_frozen_balance_without_partial_result() {
        let result = SpotOrderDomainService::cancel(
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
        let result = SpotOrderDomainService::cancel(
            order(OrderSide::Sell, SpotOrderStatus::Canceled, 1, 0, 0),
            balance(Coin::BTC, 0, 1),
            TIMESTAMP,
        );

        assert!(matches!(result, Err(CancelSpotOrderV7Error::Entity(_))));
    }

    fn match_order(
        id: u64,
        side: OrderSide,
        tif: OrderTif,
        price: i64,
        size: i64,
        fee_reserve: i64,
        version: u64,
    ) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(id + 100),
            order_id: id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(price),
            size: Decimal::from(size),
            orig_size: Decimal::from(size),
            tif,
            status: SpotOrderStatus::Open,
            execution: PriceType::Limit,
            buy_quote_fee_reserve_limit: Decimal::from(fee_reserve),
            version,
            ..OrderBase::default()
        })
        .unwrap()
    }

    fn match_input() -> MatchSpotOrderV3Input {
        MatchSpotOrderV3Input {
            trade_id_start: 900,
            maker_fee_bps: 100,
            taker_fee_bps: 200,
            timestamp: TIMESTAMP,
        }
    }

    #[test]
    fn match_order_plans_multiple_fills_and_commits_each_order_once() {
        let taker = match_order(1, OrderSide::Buy, OrderTif::Gtc, 100, 3, 10, 7);
        let skipped = match_order(2, OrderSide::Sell, OrderTif::Gtc, 101, 1, 0, 4);
        let maker_a = match_order(3, OrderSide::Sell, OrderTif::Gtc, 99, 1, 0, 5);
        let maker_b = match_order(4, OrderSide::Sell, OrderTif::Gtc, 98, 2, 0, 6);

        let outcome = SpotOrderDomainService::match_order(
            taker,
            vec![skipped.clone(), maker_a, maker_b],
            match_input(),
        )
        .unwrap();
        let MatchSpotOrderV3Outcome::Filled {
            filled_taker_order_after,
            maker_orders_after,
            created_trades,
            ..
        } = outcome
        else {
            panic!("expected filled outcome")
        };
        assert_eq!(filled_taker_order_after.order_base().version, 8);
        assert_eq!(
            filled_taker_order_after.order_base().buy_quote_fee_reserve_consumed,
            Decimal::from(6)
        );
        assert_eq!(maker_orders_after[0], skipped);
        assert_eq!(maker_orders_after[1].order_base().version, 6);
        assert_eq!(maker_orders_after[2].order_base().version, 7);
        assert_eq!(created_trades.len(), 2);
        assert_eq!(created_trades[0].comon_trade.trade_id, 900);
        assert_eq!(created_trades[1].comon_trade.trade_id, 901);
        assert_eq!(created_trades[0].comon_trade.price, Decimal::from(99));
        assert_eq!(created_trades[1].comon_trade.price, Decimal::from(98));
        assert_eq!(created_trades[0].comon_trade.taker_fee, Decimal::from(2));
        assert_eq!(created_trades[0].comon_trade.maker_fee, Decimal::ONE);
    }

    #[test]
    fn buy_maker_consumes_its_fee_reserve_while_sell_taker_does_not() {
        let taker = match_order(1, OrderSide::Sell, OrderTif::Gtc, 99, 1, 0, 7);
        let maker = match_order(2, OrderSide::Buy, OrderTif::Gtc, 100, 1, 2, 4);
        let MatchSpotOrderV3Outcome::Filled {
            filled_taker_order_after, maker_orders_after, ..
        } = SpotOrderDomainService::match_order(taker, vec![maker], match_input()).unwrap()
        else {
            panic!("expected fill")
        };
        assert_eq!(
            filled_taker_order_after.order_base().buy_quote_fee_reserve_consumed,
            Decimal::ZERO
        );
        assert_eq!(maker_orders_after[0].order_base().buy_quote_fee_reserve_consumed, Decimal::ONE);
    }

    #[test]
    fn alo_rejects_without_changing_makers_and_ioc_zero_fill_rejects() {
        let alo = match_order(1, OrderSide::Buy, OrderTif::Alo, 100, 1, 2, 7);
        let maker = match_order(2, OrderSide::Sell, OrderTif::Gtc, 99, 1, 0, 4);
        let maker_before = maker.clone();
        let outcome = SpotOrderDomainService::match_order(alo, vec![maker], match_input()).unwrap();
        assert!(matches!(outcome, MatchSpotOrderV3Outcome::Rejected { .. }));
        assert_eq!(maker_before.order_base().version, 4);

        let ioc = match_order(3, OrderSide::Buy, OrderTif::Ioc, 90, 1, 2, 9);
        let non_crossing = match_order(4, OrderSide::Sell, OrderTif::Gtc, 99, 1, 0, 2);
        let MatchSpotOrderV3Outcome::Rejected { rejected_taker_order_after, .. } =
            SpotOrderDomainService::match_order(ioc, vec![non_crossing], match_input()).unwrap()
        else {
            panic!("expected IOC rejection")
        };
        assert_eq!(rejected_taker_order_after.order_base().version, 10);
    }

    #[test]
    fn planning_error_returns_no_partial_outcome() {
        let taker = match_order(1, OrderSide::Buy, OrderTif::Gtc, 100, 2, 1, 7);
        let maker = match_order(2, OrderSide::Sell, OrderTif::Gtc, 100, 2, 0, 4);
        let mut costly = match_input();
        costly.taker_fee_bps = 10_000;
        assert!(
            SpotOrderDomainService::match_order(taker.clone(), vec![maker.clone()], costly)
                .is_err()
        );
    }
}
