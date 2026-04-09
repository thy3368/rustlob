use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{SpotOrder, TimeInForce};
use base_types::handler::handler_update2::{
    ApplyCommandChanges2, CmdHandlerForUpdate2, DomainEventSet,
};
use base_types::Quantity;
use db_repo::core::db_repo2::CmdRepo2;
use db_repo::core::event_publish::EventPublisher2;
use diff::diff_types::{track_create, DomainEvent};

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;

#[derive(Debug, Clone)]
pub struct PlaceOrderStateSet {
    pub order_id: u64,
}

pub struct PlaceOrderStateChangedSet {
    pub order: DomainEvent<SpotOrder>,
}

impl DomainEventSet for PlaceOrderStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        1
    }
}

pub struct PlaceOrderCmdHandler<R: CmdRepo2, P: EventPublisher2> {
    pub repo: R,
    pub publisher: P,
}

impl<R: CmdRepo2, P: EventPublisher2> PlaceOrderCmdHandler<R, P> {
    pub fn new(repo: R, publisher: P) -> Self {
        Self { repo, publisher }
    }

    fn generate_order_id(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
    }
}

impl<R: CmdRepo2, P: EventPublisher2> ApplyCommandChanges2 for PlaceOrderCmdHandler<R, P> {
    type Command = NewOrderCmd;
    type Reply = DomainEvent<SpotOrder>;
    type GivenStateSet = PlaceOrderStateSet;
    type ThenStateSet = PlaceOrderStateChangedSet;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Self::Command,
        state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        let symbol = cmd.symbol().clone();
        let side = *cmd.side();
        let quantity = cmd.quantity().clone().unwrap_or_default();
        let price = cmd.price().clone().unwrap_or_default();
        let time_in_force = cmd.time_in_force().clone().unwrap_or(TimeInForce::GTC);

        let actor_bytes =
            cmd.metadata().actor().as_ref().map(|s| s.as_bytes().to_owned()).unwrap_or_default();
        let mut trader_id_bytes = [0u8; 8];
        let len = actor_bytes.len().min(8);
        trader_id_bytes[..len].copy_from_slice(&actor_bytes[..len]);

        let order = SpotOrder::create_order(
            state_set.order_id.into(),
            TraderId::new(trader_id_bytes),
            symbol,
            side,
            price,
            quantity,
            time_in_force,
            cmd.new_client_order_id().clone(),
            Quantity::default(),
        );

        let change_log = track_create(&order)
            .map_err(|e| SpotCmdErrorAny::Common(CommonError::Other(format!("{}", e))))?;

        Ok(PlaceOrderStateChangedSet { order: DomainEvent::new(change_log, order) })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::ThenStateSet) -> Self::Reply {
        state_changed_set.order
    }
}

impl<R: CmdRepo2, P: EventPublisher2> CmdHandlerForUpdate2 for PlaceOrderCmdHandler<R, P> {
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &Self::Command,
    ) -> Result<Self::GivenStateSet, Self::Error> {
        Ok(PlaceOrderStateSet { order_id: self.generate_order_id() })
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
        self.repo
            .replay_event::<SpotOrder>(&domain_events.order)
            .map_err(|e| SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() }))
    }

    fn publish_domain_events(
        &self,
        domain_events: &Self::ThenStateSet,
    ) -> Result<(), Self::Error> {
        self.publisher.publish(&domain_events.order).map_err(|_e| {
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: "publish place order event failed".to_string(),
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderSide, OrderType, TimeInForce, TradingPair};
    use base_types::{Price, Quantity};

    use super::*;
    use crate::proc::v2::trade_cmd_handlers::v3::cmd_handler::mock_repo::{
        MockEventPublisher, MockMySqlRepo,
    };


    #[test]
    fn test_apply_command_and_collect_changes_builds_order_event() {
        let cmd = NewOrderCmd::new(
            CMetadata::default(),
            TradingPair::BtcUsdt,
            OrderSide::Buy,
            OrderType::Limit,
            Some(TimeInForce::GTC),
            Some(Quantity::from_f64(1.0)),
            None,
            Some(Price::from_f64(50000.0)),
            Some("test_order_001".to_string()),
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

        let handler = PlaceOrderCmdHandler::new(MockMySqlRepo, MockEventPublisher);
        let changes = handler
            .apply_command_and_collect_changes(&cmd, PlaceOrderStateSet { order_id: 42 })
            .expect("apply should succeed");

        let order = changes.order.object();
        assert_eq!(order.order_id, 42u64);
        assert_eq!(order.trading_pair, TradingPair::BtcUsdt);
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.price, Some(Price::from_f64(50000.0)));
        assert_eq!(order.total_base_qty, Quantity::from_f64(1.0));
        assert_eq!(order.time_in_force, TimeInForce::GTC);
        assert_eq!(order.client_order_id.as_deref(), Some("test_order_001"));
        assert_eq!(changes.domain_event_count(), 1);
    }
}
