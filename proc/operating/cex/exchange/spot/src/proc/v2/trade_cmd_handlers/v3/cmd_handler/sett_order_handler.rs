use std::collections::HashMap;

use base_types::account::balance::{Balance as AccountBalance, Balance};
use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::handler_update2::{
    CmdHandlerForUpdate2, CmdHandlerInternal, DomainEventSet,
};
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::DomainEvent;
use diff::Entity;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

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

    fn apply_command_and_collect_changes(
        &self,
        _cmd: &Self::Command,
        state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        let Some(trade) = state_set.trades.first() else {
            return Ok(SettStateChangedSet { balances: None });
        };

        let mut taker_quote_balance = AccountBalance::new(
            trade.taker_order_id.into(),
            trade.trading_pair.quote_asset(),
            trade.timestamp,
        );
        taker_quote_balance.add_balance(trade.quote_qty, trade.timestamp);
        let taker_quote_balance_event = DomainEvent::new(
            taker_quote_balance.track_create().map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
            })?,
            taker_quote_balance,
        );

        let mut maker_base_balance = AccountBalance::new(
            trade.maker_order_id.into(),
            trade.trading_pair.base_asset(),
            trade.timestamp,
        );
        maker_base_balance.add_balance(trade.base_qty, trade.timestamp);
        let maker_base_balance_event = DomainEvent::new(
            maker_base_balance.track_create().map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
            })?,
            maker_base_balance,
        );

        Ok(SettStateChangedSet {
            balances: Some(vec![taker_quote_balance_event, maker_base_balance_event]),
        })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::ThenStateSet) -> Self::Reply {
        state_changed_set.balances
    }
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        todo!()
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Self::Command,
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
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::ThenStateSet,
    ) -> Result<(), Self::Error> {
        if let Some(ref balances) = domain_events.balances {
            for balance_event in balances {
                self.repo.replay_event::<AccountBalance>(balance_event).map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                })?;
            }
        }
        Ok(())
    }

    fn publish_domain_events(&self, domain_events: &Self::ThenStateSet) -> Result<(), Self::Error> {
        if let Some(ref balances) = domain_events.balances {
            self.publisher.publish_batch(balances).map_err(|_e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "publish settlement events failed".to_string(),
                })
            })?;
        }
        Ok(())
    }
}

impl<R: CmdRepo2, P: EventPublisher2> CmdHandlerForUpdate2 for SettOrderCmdHandler<R, P> {}

#[cfg(test)]
mod tests {
    use base_types::exchange::spot::spot_types::SpotTrade;
    use db_repo::adapter::mysql_repo::MySqlRepo;
    use diff::diff_types::DomainEvent;

    use super::*;

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
    fn test_settlement_apply_returns_no_balance_events_for_now() {
        use db_repo::MySqlRepo;

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

        let handler = SettOrderCmdHandler::<MySqlRepo, MockPublisher>::new(
            MySqlRepo::new_mock(),
            MockPublisher,
        );
        let cmd = SettlementCmd { trades: Vec::<SpotTrade>::new() };

        let state = handler
            .load_state_set_for_update(&cmd)
            .expect("load_state_set_for_update should succeed");
        let changes =
            handler.apply_command_and_collect_changes(&cmd, state).expect("apply should succeed");

        assert!(changes.balances.is_none());
        assert_eq!(changes.domain_event_count(), 0);
    }
}
