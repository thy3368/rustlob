use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff, UpdatedEntityPair,
    UseCase,
};
use example_core_entity::common::identifiers::OrderId;
use example_core_entity::{BalanceV2, BalanceV2Error, OrderSide, SpotOrderStatus, SpotOrderV7};
use thiserror::Error;

use crate::use_case::trading::spot::v3::domain_service::atom_spot_order_domain_service::{
    CancelSpotOrderV7Error, SpotOrderDomainService,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3Cmd {
    pub order_id: OrderId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3State {
    pub order: SpotOrderV7,
    pub balance: BalanceV2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3AfterChanges {
    pub order_after: SpotOrderV7,
    pub balance_after: Option<BalanceV2>,
}

/// 订单 pair 是必有的取消事实；余额仅在实际释放冻结额时形成 update pair。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV3Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV7>,
    pub updated_balance: Option<UpdatedEntityPair<BalanceV2>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV3Error {
    #[error("order oid must be greater than zero")]
    InvalidOid,
    #[error("order cloid must not be empty")]
    EmptyCloid,
    #[error("order lookup does not match the loaded order")]
    OrderLookupMismatch,
    #[error("loaded order owner does not match the command user")]
    OwnerMismatch,
    #[error("loaded order has no client order id")]
    OrderCloidMissing,
    #[error("order is not cancelable")]
    OrderNotCancelable,
    #[error("command LSN must be newer than the loaded order")]
    StaleOrder,
    #[error("balance party does not match the order owner")]
    PartyMismatch,
    #[error("balance coin does not match the asset held by the order")]
    CoinMismatch,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("balance is closed")]
    BalanceClosed,
    #[error("invalid balance amount")]
    InvalidBalanceAmount,
    #[error("arithmetic overflow while canceling the order")]
    ArithmeticOverflow,
    #[error(transparent)]
    Entity(#[from] EntityError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV3UseCase;

impl ReplayableChanges for CancelSpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(1 + usize::from(self.updated_balance.is_some()));
        events.push(self.updated_order.to_replayable_event()?);
        if let Some(balance) = &self.updated_balance {
            events.push(balance.to_replayable_event()?);
        }
        Ok(events)
    }
}

impl UseCase for CancelSpotOrderV3UseCase {
    type Command = LogEntry<CancelSpotOrderV3Cmd>;
    type StateGiven = CancelSpotOrderV3State;
    type Error = CancelSpotOrderV3Error;
    type StateChanged = CancelSpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match &cmd.payload().order_id {
            OrderId::Oid(0) => Err(CancelSpotOrderV3Error::InvalidOid),
            OrderId::Cloid(cloid) if cloid.trim().is_empty() => {
                Err(CancelSpotOrderV3Error::EmptyCloid)
            }
            OrderId::Oid(_) | OrderId::Cloid(_) => Ok(()),
        }
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let order = state.order.order_base();
        if order.owner.0 != common_entity::stable_string_entity_id(cmd.user_id()) {
            return Err(CancelSpotOrderV3Error::OwnerMismatch);
        }
        match &cmd.payload().order_id {
            OrderId::Oid(oid) if *oid != order.order_id => {
                return Err(CancelSpotOrderV3Error::OrderLookupMismatch);
            }
            OrderId::Cloid(cloid) => match order.cloid.as_deref() {
                None => return Err(CancelSpotOrderV3Error::OrderCloidMissing),
                Some(loaded) if loaded != cloid => {
                    return Err(CancelSpotOrderV3Error::OrderLookupMismatch);
                }
                Some(_) => {}
            },
            OrderId::Oid(_) => {}
        }
        if !matches!(
            state.order.status(),
            SpotOrderStatus::Pending | SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled
        ) {
            return Err(CancelSpotOrderV3Error::OrderNotCancelable);
        }
        if cmd.lsn() <= order.lsn {
            return Err(CancelSpotOrderV3Error::StaleOrder);
        }
        if state.balance.party_id() != order.owner.0 {
            return Err(CancelSpotOrderV3Error::PartyMismatch);
        }
        let held_coin = match order.side {
            OrderSide::Buy => order.symbol.quote,
            OrderSide::Sell => order.symbol.base,
        };
        if state.balance.coin_idx() != u64::from(held_coin.index()) {
            return Err(CancelSpotOrderV3Error::CoinMismatch);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let (order_after, balance_after) = SpotOrderDomainService::cancel(
            state.order.clone(),
            state.balance.clone(),
            cmd.lsn(),
            cmd.logical_ts(),
        )
        .map_err(map_cancel_error)?;
        let balance_after = (balance_after != state.balance).then_some(balance_after);
        Ok(CancelSpotOrderV3AfterChanges { order_after, balance_after })
    }
}

impl StateDiff for CancelSpotOrderV3UseCase {
    type StateDiff = CancelSpotOrderV3Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(CancelSpotOrderV3Changes {
            updated_order: UpdatedEntityPair::new(state.order, changed.order_after),
            updated_balance: changed
                .balance_after
                .map(|after| UpdatedEntityPair::new(state.balance, after)),
        })
    }
}

fn map_cancel_error(error: CancelSpotOrderV7Error) -> CancelSpotOrderV3Error {
    match error {
        CancelSpotOrderV7Error::AssetMismatch => CancelSpotOrderV3Error::CoinMismatch,
        CancelSpotOrderV7Error::Balance(error) => match error {
            BalanceV2Error::InsufficientFrozenBalance => {
                CancelSpotOrderV3Error::InsufficientFrozenBalance
            }
            BalanceV2Error::BalanceClosed => CancelSpotOrderV3Error::BalanceClosed,
            BalanceV2Error::InvalidAmount | BalanceV2Error::InsufficientAvailableBalance => {
                CancelSpotOrderV3Error::InvalidBalanceAmount
            }
            BalanceV2Error::ArithmeticOverflow | BalanceV2Error::CannotCloseNonZeroBalance => {
                CancelSpotOrderV3Error::ArithmeticOverflow
            }
        },
        CancelSpotOrderV7Error::Entity(error) => CancelSpotOrderV3Error::Entity(error),
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, LogEntrySequencer};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::{OrderBase, OrderTrigger, PriceType};
    use example_core_entity::{OrderGroupRelation, OrderTif};
    use rust_decimal::Decimal;

    use super::*;

    const USER_ID: &str = "trader-1";
    const OID: u64 = 7;
    const LOGICAL_TS: u64 = 500;

    fn command(order_id: OrderId) -> LogEntry<CancelSpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(
            9,
            LOGICAL_TS,
            1,
            2,
            USER_ID.to_owned(),
            CancelSpotOrderV3Cmd { order_id },
        )
    }

    fn order(
        side: OrderSide,
        status: SpotOrderStatus,
        remaining: i64,
        fee_reserve: i64,
        fee_consumed: i64,
        cloid: Option<&str>,
    ) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: common_entity::stable_string_entity_id(USER_ID).into(),
            order_id: OID,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::from(10),
            size: Decimal::from(remaining),
            orig_size: Decimal::from(3),
            execution: PriceType::Limit,
            tif: OrderTif::Gtc,
            trigger: OrderTrigger::None,
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status,
            buy_quote_fee_reserve_limit: Decimal::from(fee_reserve),
            buy_quote_fee_reserve_consumed: Decimal::from(fee_consumed),
            cloid: cloid.map(str::to_owned),
            created_at: 1,
            updated_at: 100,
            lsn: 4,
        })
        .expect("test order must be valid")
    }

    fn balance(coin: Coin, party: u64, available: i64, frozen: i64) -> BalanceV2 {
        BalanceV2::new(
            party,
            22,
            u64::from(coin.index()),
            Decimal::from(available),
            Decimal::from(frozen),
            9,
            100,
        )
    }

    fn state(order: SpotOrderV7, balance: BalanceV2) -> CancelSpotOrderV3State {
        CancelSpotOrderV3State { order, balance }
    }

    #[test]
    fn oid_and_cloid_both_locate_and_cancel_open_buy() {
        for lookup in [OrderId::Oid(OID), OrderId::Cloid("client-7".into())] {
            let given = state(
                order(OrderSide::Buy, SpotOrderStatus::Open, 2, 5, 2, Some("client-7")),
                balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 7, 23),
            );

            let changes = CancelSpotOrderV3UseCase
                .compute_state_diff(&command(lookup), given)
                .expect("cancel must succeed");

            assert_eq!(changes.updated_order.after.status(), SpotOrderStatus::Canceled);
            assert_eq!(changes.updated_order.after.order_base().lsn, 9);
            assert_eq!(changes.updated_order.after.order_base().updated_at, LOGICAL_TS);
            let balance = changes.updated_balance.expect("open order must release its hold");
            assert_eq!(balance.after.available(), Decimal::from(30));
            assert_eq!(balance.after.frozen(), Decimal::ZERO);
            assert_eq!(balance.after.version(), 10);
            assert_eq!(balance.after.updated_at(), LOGICAL_TS);
            let events = CancelSpotOrderV3Changes {
                updated_order: changes.updated_order,
                updated_balance: Some(balance),
            }
            .to_replayable_events()
            .expect("pairs must project");
            assert_eq!(events.len(), 2);
            assert_eq!(events[0].entity_type, <SpotOrderV7 as Entity>::entity_type());
            assert_eq!(events[1].entity_type, <BalanceV2 as Entity>::entity_type());
        }
    }

    #[test]
    fn partially_filled_sell_releases_only_remaining_base() {
        let given = state(
            order(OrderSide::Sell, SpotOrderStatus::PartiallyFilled, 1, 0, 0, Some("client-7")),
            balance(Coin::BTC, common_entity::stable_string_entity_id(USER_ID), 4, 1),
        );

        let changed = CancelSpotOrderV3UseCase
            .compute_state_diff(&command(OrderId::Oid(OID)), given)
            .expect("cancel must succeed");
        let balance = changed.updated_balance.expect("sell hold must be released");
        assert_eq!(balance.after.available(), Decimal::from(5));
        assert_eq!(balance.after.frozen(), Decimal::ZERO);
    }

    #[test]
    fn pending_order_projects_only_order_and_keeps_balance_version() {
        let given = state(
            order(OrderSide::Buy, SpotOrderStatus::Pending, 3, 2, 0, Some("client-7")),
            balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 20, 32),
        );
        let balance_before = given.balance.clone();

        let changed = CancelSpotOrderV3UseCase
            .compute_state_diff(&command(OrderId::Oid(OID)), given)
            .expect("pending cancel must succeed");

        assert!(changed.updated_balance.is_none());
        assert_eq!(balance_before.version(), 9);
        assert_eq!(changed.to_replayable_events().expect("order pair must project").len(), 1);
    }

    #[test]
    fn rejects_invalid_lookup_state_and_balance_identity() {
        let valid = || {
            state(
                order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0, Some("client-7")),
                balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 0, 20),
            )
        };
        assert_eq!(
            CancelSpotOrderV3UseCase.check_command(&command(OrderId::Oid(0))),
            Err(CancelSpotOrderV3Error::InvalidOid)
        );
        assert_eq!(
            CancelSpotOrderV3UseCase.check_command(&command(OrderId::Cloid("  ".into()))),
            Err(CancelSpotOrderV3Error::EmptyCloid)
        );
        for (lookup, given, expected) in [
            (OrderId::Oid(99), valid(), CancelSpotOrderV3Error::OrderLookupMismatch),
            (OrderId::Cloid("wrong".into()), valid(), CancelSpotOrderV3Error::OrderLookupMismatch),
            (
                OrderId::Cloid("client-7".into()),
                state(
                    order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0, None),
                    balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 0, 20),
                ),
                CancelSpotOrderV3Error::OrderCloidMissing,
            ),
            (
                OrderId::Oid(OID),
                state(
                    order(OrderSide::Buy, SpotOrderStatus::Filled, 0, 0, 0, Some("client-7")),
                    balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 0, 0),
                ),
                CancelSpotOrderV3Error::OrderNotCancelable,
            ),
            (
                OrderId::Oid(OID),
                state(
                    order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0, Some("client-7")),
                    balance(Coin::USDT, 99, 0, 20),
                ),
                CancelSpotOrderV3Error::PartyMismatch,
            ),
            (
                OrderId::Oid(OID),
                state(
                    order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0, Some("client-7")),
                    balance(Coin::BTC, common_entity::stable_string_entity_id(USER_ID), 0, 20),
                ),
                CancelSpotOrderV3Error::CoinMismatch,
            ),
        ] {
            assert_eq!(
                CancelSpotOrderV3UseCase.validate_state_given(&command(lookup), &given),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_order_owned_by_a_different_command_user() {
        let given = state(
            order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0, Some("client-7")),
            balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 0, 20),
        );
        let command = LogEntrySequencer::from_metadata(
            9,
            LOGICAL_TS,
            1,
            2,
            "other-user".to_owned(),
            CancelSpotOrderV3Cmd { order_id: OrderId::Oid(OID) },
        );

        assert_eq!(
            CancelSpotOrderV3UseCase.validate_state_given(&command, &given),
            Err(CancelSpotOrderV3Error::OwnerMismatch)
        );
    }

    #[test]
    fn insufficient_frozen_balance_returns_error_without_mutating_input() {
        let given = state(
            order(OrderSide::Buy, SpotOrderStatus::Open, 2, 5, 2, Some("client-7")),
            balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 7, 22),
        );
        let before = given.clone();

        assert_eq!(
            CancelSpotOrderV3UseCase.compute_state_changed(&command(OrderId::Oid(OID)), &given),
            Err(CancelSpotOrderV3Error::InsufficientFrozenBalance)
        );
        assert_eq!(given, before);
    }

    #[test]
    fn rejects_loaded_order_at_or_after_command_lsn() {
        let mut record = order(OrderSide::Buy, SpotOrderStatus::Open, 2, 0, 0, Some("client-7"))
            .order_base()
            .clone();
        record.lsn = 9;
        let given = state(
            SpotOrderV7::from_record(record).unwrap(),
            balance(Coin::USDT, common_entity::stable_string_entity_id(USER_ID), 0, 20),
        );

        assert_eq!(
            CancelSpotOrderV3UseCase.validate_state_given(&command(OrderId::Oid(OID)), &given),
            Err(CancelSpotOrderV3Error::StaleOrder)
        );
    }
}
