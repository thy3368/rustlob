use std::sync::Arc;

use base_types::base_types::TraderId;
use base_types::exchange::spot::spot_types::{OrderType, SpotOrder, SpotTrade, TimeInForce};
use base_types::handler::handler_update2::{
    ApplyCommandChanges2, CmdHandlerForUpdate2, DomainEventSet,
};
use base_types::{Price, Quantity};
use db_repo::core::db_repo2::CmdRepo2;
use diff::diff_types::{ChangeLog, DomainEvent, track_create};

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    Fill, NewOrderCmd, NewOrderFull, NewOrderResult, SelfTradePreventionMode,
};

#[derive(Debug, Clone)]
pub struct PlaceOrderStateSet {
    pub order_id: u64,
}

pub struct PlaceOrderStateChangedSet {
    pub order: Option<DomainEvent<SpotOrder>>,
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
    pub balances: Option<Vec<DomainEvent<base_types::account::balance::Balance>>>,
}

impl DomainEventSet for PlaceOrderStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        let mut count = 0;
        if self.order.is_some() {
            count += 1;
        }
        if let Some(ref trades) = self.trades {
            count += trades.len();
        }
        if let Some(ref balances) = self.balances {
            count += balances.len();
        }
        count
    }
}

pub struct PlaceOrderCmdHandler<R: CmdRepo2> {
    pub repo: R,
}

impl<R: CmdRepo2> PlaceOrderCmdHandler<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    fn generate_order_id(&self) -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64
    }
}

impl<R: CmdRepo2> ApplyCommandChanges2 for PlaceOrderCmdHandler<R> {
    type Command = NewOrderCmd;
    type Reply = NewOrderFull;
    type StateSet = PlaceOrderStateSet;
    type StateChangedSet = PlaceOrderStateChangedSet;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Self::Command,
        state_set: Self::StateSet,
    ) -> Result<Self::StateChangedSet, Self::Error> {
        let symbol = cmd.symbol().clone();
        let side = *cmd.side();
        let quantity = cmd.quantity().clone().unwrap_or_default();
        let price = cmd.price().clone().unwrap_or_default();
        let order_type = *cmd.order_type();
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

        let order_event = DomainEvent::new(change_log, order);

        let trades =
            if order_type == OrderType::Market { todo!("市价单撮合逻辑") } else { None };

        Ok(PlaceOrderStateChangedSet { order: Some(order_event), trades, balances: None })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::StateChangedSet) -> Self::Reply {
        let order_event = state_changed_set.order.expect("order should exist");
        let order = order_event.object();

        let base = NewOrderResult::new(
            order.trading_pair,
            order.order_id,
            0u64,
            order.client_order_id.clone().unwrap_or_default(),
            order.timestamp,
            order.price.unwrap_or_default(),
            order.total_base_qty,
            order.state.filled_base_qty,
            order.total_quote_qty,
            Price::default(),
            order.state.status,
            order.time_in_force,
            order.order_type,
            order.side,
            order.timestamp,
            SelfTradePreventionMode::EXPIRE_TAKER,
            order.iceberg_qty,
            None,
            None,
            order.stop_price,
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

        let fills = state_changed_set
            .trades
            .map(|trades| -> Vec<Fill> {
                trades
                    .iter()
                    .map(|t| {
                        let trade = t.object();
                        Fill::new(
                            trade.price,
                            trade.base_qty,
                            trade.taker_commission_qty,
                            trade.commission_asset,
                            trade.trade_id,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();

        NewOrderFull::new(base, fills)
    }
}

impl<R: CmdRepo2> CmdHandlerForUpdate2 for PlaceOrderCmdHandler<R> {
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Self::Command,
    ) -> Result<Self::StateSet, Self::Error> {
        Ok(PlaceOrderStateSet { order_id: self.generate_order_id() })
    }

    fn validate_command_in_lock(
        &self,
        cmd: &Self::Command,
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
        if let Some(ref order_event) = domain_events.order {
            self.repo.replay_event::<SpotOrder>(order_event).map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
            })?;
        }
        if let Some(ref trades) = domain_events.trades {
            for trade_event in trades {
                self.repo.replay_event::<SpotTrade>(trade_event).map_err(|e| {
                    SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                })?;
            }
        }
        if let Some(ref balances) = domain_events.balances {
            for balance_event in balances {
                self.repo
                    .replay_event::<base_types::account::balance::Balance>(balance_event)
                    .map_err(|e| {
                        SpotCmdErrorAny::Common(CommonError::Internal { message: e.to_string() })
                    })?;
            }
        }
        Ok(())
    }

    fn publish_domain_events(
        &self,
        _domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use base_types::cqrs::cqrs_types::CMetadata;
    use base_types::exchange::spot::spot_types::{OrderSide, OrderType, TimeInForce, TradingPair};
    use base_types::{Price, Quantity};
    use db_repo::core::db_repo2::{CmdRepo2, RepoError};
    use diff::diff_types::DomainEvent;

    use super::*;

    struct MockMySqlRepo;

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

    #[test]
    fn test_place_order_cmd_handler() {
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

        let repo = MockMySqlRepo;
        let handler = PlaceOrderCmdHandler::new(repo);

        //todo apply_command_and_collect_changes需要重点单测
        // handler.apply_command_and_collect_changes()

        let result = handler.cmd_handle(cmd);

        println!("Test result: {:?}", result);
        assert!(result.is_ok(), "cmd_handle should succeed");
    }
}
