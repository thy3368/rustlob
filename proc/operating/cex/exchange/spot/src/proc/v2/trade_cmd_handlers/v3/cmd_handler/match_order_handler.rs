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
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};

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

pub struct MatchCmd {
    pub taker_order: SpotOrder,
}

pub struct MatchOrderCmdHandler<
    R: CmdRepo2,
    P: EventPublisher,
    L: MultiSymbolLobRepo<Order = SpotOrder>,
> {
    pub repo: R,
    pub publisher: P,
    pub lob: L,
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo<Order = SpotOrder>>
    MatchOrderCmdHandler<R, P, L>
{
    pub fn new(repo: R, publisher: P, lob: L) -> Self {
        Self { repo, publisher, lob }
    }
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo<Order = SpotOrder>> ApplyCommandChanges2
    for MatchOrderCmdHandler<R, P, L>
{
    type Command = MatchCmd;
    type Reply = Option<Vec<DomainEvent<SpotTrade>>>;
    type StateSet = MatchOrderStateSet;
    type StateChangedSet = MatchOrderStateChangedSet;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        _cmd: &Self::Command,
        _state_set: Self::StateSet,
    ) -> Result<Self::StateChangedSet, Self::Error> {
        //todo 根据撮合 生成trade
        Ok(MatchOrderStateChangedSet { trades: None })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::StateChangedSet) -> Self::Reply {
        state_changed_set.trades
    }
}

impl<R: CmdRepo2, P: EventPublisher, L: MultiSymbolLobRepo<Order = SpotOrder>> CmdHandlerForUpdate2
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
        let (matched, _) = self.lob.match_orders(
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            Price::default(),
            Quantity::default(),
        );
        let makers: Vec<SpotOrder> =
            matched.map(|v| v.into_iter().map(|o| o.clone()).collect()).unwrap_or_default();

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
    use base_types::exchange::spot::spot_types::TradingPair;
    use diff::diff_types::DomainEvent;
    use lob_repo::adapter::embedded_lob_repo::EmbeddedLobRepo;

    use super::*;
    use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::mock_repo::MockMySqlRepo;

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
    fn test_match_order_apply_returns_no_trades_for_now() {
        let handler = MatchOrderCmdHandler::new(
            MockMySqlRepo,
            MockEventPublisher,
            EmbeddedLobRepo::<SpotOrder>::new(Vec::new()),
        );
        let cmd = MatchCmd {
            taker_order: SpotOrder::create_order(
                1u64.into(),
                TraderId::new([0u8; 8]),
                TradingPair::BtcUsdt,
                OrderSide::Buy,
                Price::from_f64(50000.0),
                Quantity::from_f64(1.0),
                TimeInForce::GTC,
                Some("test_match_001".to_string()),
                Quantity::default(),
            ),
        };

        let state = handler
            .load_state_set_for_update(&cmd)
            .expect("load_state_set_for_update should succeed");
        let changes =
            handler.apply_command_and_collect_changes(&cmd, state).expect("apply should succeed");

        assert!(changes.trades.is_none());
        assert_eq!(changes.domain_event_count(), 0);
    }
}
