use std::collections::HashMap;

use base_types::account::balance::{Balance as AccountBalance, Balance};
use base_types::exchange::spot::spot_types::SpotTrade;
use cmd_handler::{CmdHandlerForUpdate3, CmdHandlerInternal, DomainEventSet};
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use diff::Entity;

use crate::proc::behavior::v2::spot_trade_error::{CommonError, SpotCmdErrorAny};

#[derive(Debug, Clone)]
pub struct SettStateSet {
    pub trades: Vec<SpotTrade>,
    pub map: HashMap<u64, Balance>,
}

pub struct SettStateChangedSet {
    pub balances: Option<Vec<DomainEvent<AccountBalance>>>,
}

impl DomainEventSet for SettStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        self.balances.as_ref().map(|balances| balances.len()).unwrap_or(0)
    }
}

pub struct SettOrderCmdHandler<R: CmdRepo2, P: EventPublisher2> {
    pub repo: R,
    pub publisher: P,
}

impl<R: CmdRepo2, P: EventPublisher2> SettOrderCmdHandler<R, P> {
    pub fn new(repo: R, publisher: P) -> Self {
        Self { repo, publisher }
    }
}

#[derive(Debug, Clone)]
pub struct SettlementCmd {
    pub trades: Vec<SpotTrade>,
}

impl<R: CmdRepo2, P: EventPublisher2> CmdHandlerInternal for SettOrderCmdHandler<R, P> {
    type Command = SettlementCmd;
    type Reply = Option<Vec<DomainEvent<AccountBalance>>>;
    type GivenStateSet = SettStateSet;
    type ThenStateSet = SettStateChangedSet;
    type Error = SpotCmdErrorAny;

    type Repo = R;
    type Publisher = P;

    fn then(
        &self,
        _cmd: &Self::Command,
        state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        if state_set.trades.is_empty() {
            return Ok(SettStateChangedSet { balances: None });
        }

        let mut balance_map: HashMap<String, AccountBalance> = HashMap::new();

        for trade in &state_set.trades {
            let (taker_asset, taker_qty, maker_asset, maker_qty) = match trade.taker_side {
                base_types::exchange::spot::spot_types::OrderSide::Buy => (
                    trade.trading_pair.base_asset(),
                    trade.base_qty,
                    trade.trading_pair.quote_asset(),
                    trade.quote_qty,
                ),
                base_types::exchange::spot::spot_types::OrderSide::Sell => (
                    trade.trading_pair.quote_asset(),
                    trade.quote_qty,
                    trade.trading_pair.base_asset(),
                    trade.base_qty,
                ),
            };

            let taker_key = format!("{}:{}", trade.taker_order_id, u32::from(taker_asset));
            let taker_balance = balance_map.entry(taker_key).or_insert_with(|| {
                AccountBalance::new(trade.taker_order_id.into(), taker_asset, trade.timestamp)
            });
            taker_balance.add_balance(taker_qty, trade.timestamp);

            let maker_key = format!("{}:{}", trade.maker_order_id, u32::from(maker_asset));
            let maker_balance = balance_map.entry(maker_key).or_insert_with(|| {
                AccountBalance::new(trade.maker_order_id.into(), maker_asset, trade.timestamp)
            });
            maker_balance.add_balance(maker_qty, trade.timestamp);
        }

        let mut balance_events = Vec::with_capacity(balance_map.len());
        for balance in balance_map.into_values() {
            let change_log = balance.track_create().map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
            })?;
            balance_events.push(DomainEvent::new(change_log, balance));
        }

        Ok(SettStateChangedSet { balances: Some(balance_events) })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::ThenStateSet) -> Self::Reply {
        state_changed_set.balances
    }
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn give(
        &self,
        cmd: &Self::Command,
        _repo: &Self::Repo,
    ) -> Result<Self::GivenStateSet, Self::Error> {
        Ok(SettStateSet { trades: cmd.trades.clone(), map: HashMap::new() })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Self::Command,
        _state_set: &Self::GivenStateSet,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn persist_domain_events(
        &self,
        _domain_events: &Self::ThenStateSet,
        _repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::ThenStateSet,
        repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        if let Some(ref balances) = domain_events.balances {
            for balance_event in balances {
                repo.replay_event::<AccountBalance>(balance_event).map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                })?;
            }
        }
        Ok(())
    }

    fn publish_domain_events(
        &self,
        domain_events: &Self::ThenStateSet,
        publisher: Self::Publisher,
    ) -> Result<(), Self::Error> {
        if let Some(ref balances) = domain_events.balances {
            publisher.publish_batch(balances).map_err(|_e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "publish settlement events failed".to_string(),
                })
            })?;
        }
        Ok(())
    }
}

impl<R: CmdRepo2, P: EventPublisher2> CmdHandlerForUpdate3 for SettOrderCmdHandler<R, P> {}

#[cfg(test)]
mod tests {
    use base_types::{AssetId, Price, Quantity, Timestamp, TradingPair};
    use db_repo::adapter::v2::memdb_repo::MemdbRepo;
    use db_repo::core::db_repo2::QueryRepo2;
    use diff::diff_types::DomainEvent;

    use super::*;

    fn create_trade(trade_id: u64, taker_order_id: u64, maker_order_id: u64) -> SpotTrade {
        SpotTrade::new(
            trade_id,
            TradingPair::BtcUsdt,
            taker_order_id,
            maker_order_id,
            Timestamp::default(),
            Price::from_f64(50000.0),
            Quantity::from_f64(1.0),
            base_types::exchange::spot::spot_types::OrderSide::Buy,
            Quantity::default(),
            Quantity::default(),
            AssetId::Usdt,
            0,
            0,
        )
    }

    struct MockEventPublisher;

    impl EventPublisher2 for MockEventPublisher {
        fn publish<E>(
            &self,
            _event: &DomainEvent<E>,
        ) -> Result<(), db_repo::core::event_publish::PublishError> {
            Ok(())
        }

        fn publish_batch<E>(
            &self,
            _events: &[DomainEvent<E>],
        ) -> Result<(), db_repo::core::event_publish::PublishError> {
            Ok(())
        }
    }

    #[test]
    fn test_settlement_apply_returns_no_balance_events_for_empty_trades() {
        use db_repo::adapter::v2::memdb_repo::MemdbRepo;

        struct MockPublisher;
        impl EventPublisher2 for MockPublisher {
            fn publish<E: serde::Serialize>(
                &self,
                _event: &DomainEvent<E>,
            ) -> Result<(), db_repo::core::event_publish::PublishError> {
                Ok(())
            }
            fn publish_batch<E: serde::Serialize>(
                &self,
                _events: &[DomainEvent<E>],
            ) -> Result<(), db_repo::core::event_publish::PublishError> {
                Ok(())
            }
        }

        let repo = MemdbRepo::default();
        let handler = SettOrderCmdHandler::<MemdbRepo, MockPublisher>::new(
            repo.clone(),
            MockPublisher,
        );
        let cmd = SettlementCmd { trades: Vec::<SpotTrade>::new() };

        let state = handler
            .give(&cmd, &repo)
            .expect("load_state_set_for_update should succeed");
        let changes =
            handler.then(&cmd, state).expect("apply should succeed");

        assert!(changes.balances.is_none());
        assert_eq!(changes.domain_event_count(), 0);
        assert_eq!(repo.count().unwrap(), 0);
    }

    #[test]
    fn test_settlement_apply_aggregates_balances_for_two_trades() {
        let repo = MemdbRepo::default();
        let handler = SettOrderCmdHandler::new(repo.clone(), MockEventPublisher);
        let balances = handler
            .cmd_handle(
                SettlementCmd {
                    trades: vec![create_trade(1, 101, 201), create_trade(2, 101, 202)],
                },
                repo.clone(),
                MockEventPublisher,
            )
            .expect("settlement should succeed")
            .expect("balances should exist");

        assert_eq!(balances.len(), 3);

        let taker_btc = repo
            .find_by_id::<AccountBalance>("101:2")
            .expect("query taker btc should succeed")
            .expect("taker btc balance should exist");
        let maker_one_usdt = repo
            .find_by_id::<AccountBalance>("201:1")
            .expect("query first maker usdt should succeed")
            .expect("first maker usdt balance should exist");
        let maker_two_usdt = repo
            .find_by_id::<AccountBalance>("202:1")
            .expect("query second maker usdt should succeed")
            .expect("second maker usdt balance should exist");

        assert_eq!(taker_btc.asset_id, AssetId::Btc);
        assert_eq!(taker_btc.available, Quantity::from_f64(2.0));
        assert_eq!(maker_one_usdt.asset_id, AssetId::Usdt);
        assert_eq!(maker_one_usdt.available, Quantity::from_f64(50000.0));
        assert_eq!(maker_two_usdt.asset_id, AssetId::Usdt);
        assert_eq!(maker_two_usdt.available, Quantity::from_f64(50000.0));
    }
}
