use std::collections::HashMap;

use base_types::account::balance::Balance as AccountBalance;
use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::handler::handler_update2::{
    ApplyCommandChanges2, CmdHandlerForUpdate2, DomainEventSet,
};
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher;
use diff::diff_types::DomainEvent;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::spot_user_data_behavior::Balance;

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

pub struct SettOrderCmdHandler<R: CmdRepo2, P: EventPublisher> {
    pub repo: R,
    pub publisher: P,
}

impl<R: CmdRepo2, P: EventPublisher> SettOrderCmdHandler<R, P> {
    pub fn new(repo: R, publisher: P) -> Self {
        Self { repo, publisher }
    }
}


#[derive(Debug, Clone)]
pub struct SettlementCmd {
    pub trades: Vec<SpotTrade>,
}

impl<R: CmdRepo2, P: EventPublisher> ApplyCommandChanges2 for SettOrderCmdHandler<R, P> {
    type Command = SettlementCmd;
    type Reply = Option<Vec<DomainEvent<AccountBalance>>>;
    type StateSet = SettStateSet;
    type StateChangedSet = SettStateChangedSet;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        _cmd: &Self::Command,
        _state_set: Self::StateSet,
    ) -> Result<Self::StateChangedSet, Self::Error> {
        Ok(SettStateChangedSet { balances: None })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::StateChangedSet) -> Self::Reply {
        state_changed_set.balances
    }
}

impl<R: CmdRepo2, P: EventPublisher> CmdHandlerForUpdate2 for SettOrderCmdHandler<R, P> {
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &Self::Command,
    ) -> Result<Self::StateSet, Self::Error> {
        // 从repo查所有balance和trade
        Ok(SettStateSet { trades: vec![], map: HashMap::new() })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Self::Command,
        _state_set: &Self::StateSet,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn persist_domain_events(
        &self,
        _domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::StateChangedSet,
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

    fn publish_domain_events(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
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

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderSide, OrderType, TimeInForce, TradingPair};
    use base_types::{Price, Quantity};
    use db_repo::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2, RepoError};
    use diff::diff_types::DomainEvent;
    use diff::Entity;

    use super::*;

    #[derive(Clone)]
    struct MockMySqlRepo;

    impl QueryRepo2 for MockMySqlRepo {
        fn find_by_sequence<E: Entity>(&self, _sequence: u64) -> Result<Option<E>, RepoError> {
            todo!()
        }

        fn find_one_by_condition<E: Entity>(&self, _condition: E) -> Result<Option<E>, RepoError> {
            todo!()
        }

        fn find_all_by_condition<E: Entity>(&self, _condition: E) -> Result<Vec<E>, RepoError> {
            todo!()
        }

        fn find_by_id<E: Entity>(&self, _entity_id: &str) -> Result<Option<E>, RepoError> {
            todo!()
        }

        fn find_range_by_sequence<E: Entity>(
            &self,
            _from_sequence: u64,
            _to_sequence: u64,
        ) -> Result<Vec<E>, RepoError> {
            todo!()
        }

        fn count(&self) -> Result<u64, RepoError> {
            todo!()
        }

        fn find_all_by_condition_paginated<E: Entity>(
            &self,
            _condition: E,
            _page_req: PageRequest,
        ) -> Result<PageResult<E>, RepoError> {
            todo!()
        }

        fn find_range_by_sequence_paginated<E: Entity>(
            &self,
            _from_sequence: u64,
            _to_sequence: u64,
            _page_req: PageRequest,
        ) -> Result<PageResult<E>, RepoError> {
            todo!()
        }

        fn find_by_cursor<E: Entity>(
            &self,
            _condition: E,
            _cursor: Option<String>,
            _limit: u64,
            _forward: bool,
        ) -> Result<(Vec<E>, Option<String>), RepoError> {
            todo!()
        }
    }

    impl CmdRepo2 for MockMySqlRepo {
        fn replay_event<E>(&self, _event: &DomainEvent<E>) -> Result<(), RepoError> {
            Ok(())
        }

        fn replay_events<E>(&self, _events: &[DomainEvent<E>]) -> Result<(), RepoError> {
            Ok(())
        }

        fn replay_from_sequence<E: Clone + std::fmt::Debug>(
            &self,
            _events: &[DomainEvent<E>],
            _from_sequence: u64,
        ) -> Result<(), RepoError> {
            Ok(())
        }
    }

    struct MockEventPublisher;

    impl EventPublisher for MockEventPublisher {
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
        let handler = SettOrderCmdHandler::new(MockMySqlRepo, MockEventPublisher);
        let cmd = NewOrderCmd::new(
            CMetadata::default(),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(1.0)),
            None,
            Some(Price::from_f64(50000.0)),
            Some("test_sett_001".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let state = handler
            .load_state_set_for_update(&cmd)
            .expect("load_state_set_for_update should succeed");
        let changes = handler
            .apply_command_and_collect_changes(&cmd, state)
            .expect("apply should succeed");

        assert!(changes.balances.is_none());
        assert_eq!(changes.domain_event_count(), 0);
    }
}
