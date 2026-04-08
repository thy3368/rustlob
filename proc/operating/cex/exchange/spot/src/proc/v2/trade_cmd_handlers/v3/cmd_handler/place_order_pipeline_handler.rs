use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher;

use lob_repo::core::symbol_lob_repo2::MultiSymbolLobRepo2;

use super::match_order_handler::MatchOrderCmdHandler;
use super::place_order_handler::PlaceOrderCmdHandler;
use super::sett_order_handler::SettOrderCmdHandler;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceOrderPipelineStage {
    Place,
    Match,
    Settle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceOrderPipelineRoute {
    pub from: PlaceOrderPipelineStage,
    pub to: PlaceOrderPipelineStage,
}

pub struct PlaceOrderPipelineHandler<
    R: CmdRepo2 + Clone,
    P: EventPublisher + Clone,
    L: MultiSymbolLobRepo2 + Clone,
> {
    place_order_handler: PlaceOrderCmdHandler<R, P>,
    match_order_handler: MatchOrderCmdHandler<R, P, L>,
    sett_order_handler: SettOrderCmdHandler<R, P>,
}

impl<R: CmdRepo2 + Clone, P: EventPublisher + Clone, L: MultiSymbolLobRepo2 + Clone>
    PlaceOrderPipelineHandler<R, P, L>
{
    pub const ROUTES: [PlaceOrderPipelineRoute; 2] = [
        PlaceOrderPipelineRoute {
            from: PlaceOrderPipelineStage::Place,
            to: PlaceOrderPipelineStage::Match,
        },
        PlaceOrderPipelineRoute {
            from: PlaceOrderPipelineStage::Match,
            to: PlaceOrderPipelineStage::Settle,
        },
    ];

    pub fn new(repo: R, publisher: P, lob: L) -> Self {
        Self {
            place_order_handler: PlaceOrderCmdHandler::new(repo.clone(), publisher.clone()),
            match_order_handler: MatchOrderCmdHandler::new(repo.clone(), publisher.clone(), lob),
            sett_order_handler: SettOrderCmdHandler::new(repo, publisher),
        }
    }

    pub fn routes(&self) -> &'static [PlaceOrderPipelineRoute] {
        &Self::ROUTES
    }

    pub fn place_order_handler(&self) -> &PlaceOrderCmdHandler<R, P> {
        &self.place_order_handler
    }

    pub fn match_order_handler(&self) -> &MatchOrderCmdHandler<R, P, L> {
        &self.match_order_handler
    }

    pub fn sett_order_handler(&self) -> &SettOrderCmdHandler<R, P> {
        &self.sett_order_handler
    }
}

#[cfg(test)]
mod tests {
    use db_repo::core::db_repo2::{CmdRepo2, PageRequest, PageResult, QueryRepo2, RepoError};
    use diff::diff_types::DomainEvent;
    use diff::Entity;

    use lob_repo::core::symbol_lob_repo2::{MultiSymbolLobRepo2, LobError};

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
            _symbol: base_types::TradingPair,
            _side: base_types::OrderSide,
            _price: base_types::Price,
            quantity: base_types::Quantity,
        ) -> (Option<Vec<&O>>, base_types::Quantity) {
            (None, quantity)
        }

        fn best_bid(&self, _symbol: base_types::TradingPair) -> Option<base_types::Price> {
            None
        }

        fn best_ask(&self, _symbol: base_types::TradingPair) -> Option<base_types::Price> {
            None
        }

        fn contains_symbol(&self, _symbol: &base_types::TradingPair) -> bool {
            false
        }

        fn add_order<O: base_types::lob::lob::LobOrder>(
            &self,
            _symbol: base_types::TradingPair,
            _order: O,
        ) -> Result<(), LobError> {
            Ok(())
        }

        fn remove_order(
            &self,
            _symbol: base_types::TradingPair,
            _order_id: base_types::OrderId,
        ) -> bool {
            false
        }

        fn find_order<O: base_types::lob::lob::LobOrder>(
            &self,
            _p0: base_types::TradingPair,
            _p1: base_types::OrderId,
        ) -> Option<&O> {
            None
        }

        fn find_order_mut<O: base_types::lob::lob::LobOrder>(
            &self,
            _p0: base_types::TradingPair,
            _order_id: base_types::OrderId,
        ) -> Option<&mut O> {
            None
        }

        fn last_price(&self, _symbol: base_types::TradingPair) -> Option<base_types::Price> {
            None
        }

        fn update_last_price(&self, _symbol: base_types::TradingPair, _price: base_types::Price) {}
    }

    #[test]
    fn test_pipeline_builds_handlers_and_routes() {
        let pipeline = PlaceOrderPipelineHandler::new(
            MockMySqlRepo,
            MockEventPublisher,
            MockLobRepo,
        );

        assert_eq!(
            pipeline.routes(),
            &PlaceOrderPipelineHandler::<MockMySqlRepo, MockEventPublisher, MockLobRepo>::ROUTES
        );
        assert_eq!(pipeline.routes().len(), 2);
        assert_eq!(pipeline.routes()[0].from, PlaceOrderPipelineStage::Place);
        assert_eq!(pipeline.routes()[0].to, PlaceOrderPipelineStage::Match);
        assert_eq!(pipeline.routes()[1].from, PlaceOrderPipelineStage::Match);
        assert_eq!(pipeline.routes()[1].to, PlaceOrderPipelineStage::Settle);

        let _ = pipeline.place_order_handler();
        let _ = pipeline.match_order_handler();
        let _ = pipeline.sett_order_handler();
    }
}
