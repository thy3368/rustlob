use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{
    OrderSide, SpotOrder, SpotTrade, TimeInForce, TradingPair,
};
use base_types::handler::handler_update2::{
    ApplyCommandChanges2, CmdHandlerForUpdate2, DomainEventSet,
};
use base_types::{Price, Quantity};
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher;
use diff::diff_types::DomainEvent;
use lob_repo::core::symbol_lob_repo2::MultiSymbolLobRepo2;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;

#[derive(Debug, Clone)]
pub struct MatchOrderStateSet {
    pub taker: SpotOrder,
    pub makers: Vec<SpotOrder>,
}

pub struct MatchOrderStateChangedSet {
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
}

impl DomainEventSet for MatchOrderStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        self.trades.as_ref().map(|trades| trades.len()).unwrap_or(0)
    }
}

pub struct MatchOrderCmdHandler<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo2> {
    pub repo: R,
    pub publisher: P,
    pub lob: L,
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo2> MatchOrderCmdHandler<R, P, L> {
    pub fn new(repo: R, publisher: P, lob: L) -> Self {
        Self { repo, publisher, lob }
    }
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo2> ApplyCommandChanges2
    for MatchOrderCmdHandler<R, P, L>
{
    type Command = NewOrderCmd;
    type Reply = Option<Vec<DomainEvent<SpotTrade>>>;
    type StateSet = MatchOrderStateSet;
    type StateChangedSet = MatchOrderStateChangedSet;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        _cmd: &Self::Command,
        _state_set: Self::StateSet,
    ) -> Result<Self::StateChangedSet, Self::Error> {
        Ok(MatchOrderStateChangedSet { trades: None })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::StateChangedSet) -> Self::Reply {
        state_changed_set.trades
    }
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo2> CmdHandlerForUpdate2
    for MatchOrderCmdHandler<R, P, L>
{
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &Self::Command,
    ) -> Result<Self::StateSet, Self::Error> {
        // 从lob加载taker和makers
        let taker = SpotOrder::create_order(
            0u64.into(),
            TraderId::new([0u8; 8]),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            Price::default(),
            Quantity::default(),
            TimeInForce::GTC,
            None,
            Quantity::default(),
        );
        //todo 完善
        let makers: Vec<SpotOrder> = self.lob.match_orders();

        Ok(MatchOrderStateSet { taker, makers })
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
        if let Some(ref trades) = domain_events.trades {
            for trade_event in trades {
                self.repo.replay_event::<SpotTrade>(trade_event).map_err(|e| {
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
        if let Some(ref trades) = domain_events.trades {
            self.publisher.publish_batch(trades).map_err(|_e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: "publish match order events failed".to_string(),
                })
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderType, TradingPair};
    use db_repo::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2};
    use diff::diff_types::DomainEvent;
    use diff::Entity;
    use lob_repo::core::symbol_lob_repo2::{LobError, MultiSymbolLobRepo2};

    use super::*;

    #[derive(Clone)]
    struct MockMySqlRepo;

    impl QueryRepo2 for MockMySqlRepo {
        fn find_by_sequence<E: Entity>(&self, _sequence: u64) -> Result<Option<E>, LobError> {
            todo!()
        }

        fn find_one_by_condition<E: Entity>(&self, _condition: E) -> Result<Option<E>, LobError> {
            todo!()
        }

        fn find_all_by_condition<E: Entity>(&self, _condition: E) -> Result<Vec<E>, LobError> {
            todo!()
        }

        fn find_by_id<E: Entity>(&self, _entity_id: &str) -> Result<Option<E>, LobError> {
            todo!()
        }

        fn find_range_by_sequence<E: Entity>(
            &self,
            _from_sequence: u64,
            _to_sequence: u64,
        ) -> Result<Vec<E>, LobError> {
            todo!()
        }

        fn count(&self) -> Result<u64, LobError> {
            todo!()
        }

        fn find_all_by_condition_paginated<E: Entity>(
            &self,
            _condition: E,
            _page_req: PageRequest,
        ) -> Result<PageResult<E>, LobError> {
            todo!()
        }

        fn find_range_by_sequence_paginated<E: Entity>(
            &self,
            _from_sequence: u64,
            _to_sequence: u64,
            _page_req: PageRequest,
        ) -> Result<PageResult<E>, LobError> {
            todo!()
        }

        fn find_by_cursor<E: Entity>(
            &self,
            _condition: E,
            _cursor: Option<String>,
            _limit: u64,
            _forward: bool,
        ) -> Result<(Vec<E>, Option<String>), LobError> {
            todo!()
        }
    }

    impl CmdRepo2 for MockMySqlRepo {
        fn replay_event<E>(&self, _event: &DomainEvent<E>) -> Result<(), LobError> {
            Ok(())
        }

        fn replay_events<E>(&self, _events: &[DomainEvent<E>]) -> Result<(), LobError> {
            Ok(())
        }

        fn replay_from_sequence<E: Clone + std::fmt::Debug>(
            &self,
            _events: &[DomainEvent<E>],
            _from_sequence: u64,
        ) -> Result<(), LobError> {
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

    #[derive(Clone)]
    struct MockLobRepo;

    impl MultiSymbolLobRepo2 for MockLobRepo {
        fn match_orders<O: base_types::lob::lob::LobOrder>(
            &self,
            _symbol: TradingPair,
            _side: OrderSide,
            _price: Price,
            quantity: Quantity,
        ) -> (Option<Vec<&O>>, Quantity) {
            (None, quantity)
        }

        fn best_bid(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn best_ask(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn contains_symbol(&self, _symbol: &TradingPair) -> bool {
            false
        }

        fn add_order<O: base_types::lob::lob::LobOrder>(
            &self,
            _symbol: TradingPair,
            _order: O,
        ) -> Result<(), LobError> {
            Ok(())
        }

        fn remove_order(&self, _symbol: TradingPair, _order_id: base_types::OrderId) -> bool {
            false
        }

        fn find_order<O: base_types::lob::lob::LobOrder>(
            &self,
            _p0: TradingPair,
            _p1: base_types::OrderId,
        ) -> Option<&O> {
            None
        }

        fn find_order_mut<O: base_types::lob::lob::LobOrder>(
            &self,
            _p0: TradingPair,
            _order_id: base_types::OrderId,
        ) -> Option<&mut O> {
            None
        }

        fn last_price(&self, _symbol: TradingPair) -> Option<Price> {
            None
        }

        fn update_last_price(&self, _symbol: TradingPair, _price: Price) {}
    }

    #[test]
    fn test_match_order_apply_returns_no_trades_for_now() {
        let handler = MatchOrderCmdHandler::new(MockMySqlRepo, MockEventPublisher, MockLobRepo);
        let cmd = NewOrderCmd::new(
            CMetadata::default(),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(1.0)),
            None,
            Some(Price::from_f64(50000.0)),
            Some("test_match_001".to_string()),
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
        let changes =
            handler.apply_command_and_collect_changes(&cmd, state).expect("apply should succeed");

        assert!(changes.trades.is_none());
        assert_eq!(changes.domain_event_count(), 0);
    }
}
