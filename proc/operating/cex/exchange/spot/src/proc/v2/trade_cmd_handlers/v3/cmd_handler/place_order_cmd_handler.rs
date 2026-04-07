use base_types::exchange::spot::spot_types::{OrderType, SpotOrder, SpotTrade, TimeInForce};
use base_types::handler::handler_update2::{
    ApplyCommandChanges2, CmdHandlerForUpdate2, DomainEventSet,
};
use base_types::{Price, Quantity};
use base_types::base_types::TraderId;
use db_repo::core::db_repo2::CmdRepo2;
use diff::diff_types::DomainEvent;
use diff::Entity;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::{
    Fill, NewOrderCmd, NewOrderFull, NewOrderResult,
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

pub struct PlaceOrderCmdHandler {
    pub repo: dyn CmdRepo2,
}

impl PlaceOrderCmdHandler {
    pub fn new() -> Self {
        Self { repo: todo!("需要注入 repo") }
    }

    fn generate_order_id(&self) -> u64 {
        todo!("生成 order_id")
    }
}

impl ApplyCommandChanges2 for PlaceOrderCmdHandler {
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
        let symbol = cmd.symbol;
        let side = cmd.side;
        let quantity = cmd.quantity.unwrap_or_default();
        let price = cmd.price.unwrap_or_default();
        let order_type = cmd.order_type;
        let time_in_force = cmd.time_in_force.unwrap_or(TimeInForce::GTC);

        let order = SpotOrder::create_order(
            state_set.order_id.into(),
            TraderId::new(cmd.metadata.trader_id.to_le_bytes()),
            symbol,
            side,
            price,
            quantity,
            time_in_force,
            cmd.new_client_order_id.clone(),
            Quantity::default(),
        );

        let order_event = order.track_create().map_err(|e| SpotCmdErrorAny(e.to_string()))?;

        let trades =
            if order_type == OrderType::Market { todo!("市价单撮合逻辑") } else { None };

        Ok(PlaceOrderStateChangedSet { order: Some(order_event), trades, balances: None })
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::StateChangedSet) -> Self::Reply {
        let order_event = state_changed_set.order.expect("order should exist");
        let order = &order_event.entity;

        let base = NewOrderResult {
            symbol: order.trading_pair,
            order_id: order.order_id.as_u64(),
            order_list_id: -1,
            client_order_id: order.client_order_id.clone().unwrap_or_default(),
            transact_time: order.timestamp,
            price: order.price.unwrap_or_default(),
            orig_qty: order.total_base_qty,
            executed_qty: order.state.filled_base_qty,
            orig_quote_order_qty: order.total_quote_qty,
            cummulative_quote_qty: Price::default(),
            status: order.state.status,
            time_in_force: order.time_in_force,
            order_type: order.order_type,
            side: order.side,
            working_time: order.timestamp,
            self_trade_prevention: order.self_trade_prevention,
            iceberg_qty: order.iceberg_qty,
            prevented_match_id: None,
            prevented_quantity: None,
            stop_price: order.stop_price,
            strategy_id: None,
            strategy_type: None,
            trailing_delta: None,
            trailing_time: None,
            used_sor: None,
            working_floor: None,
            peg_price_type: None,
            peg_offset_type: None,
            peg_offset_value: None,
            pegged_price: None,
        };

        let fills: Vec<Fill> = state_changed_set
            .trades
            .map(|trades| {
                trades
                    .iter()
                    .map(|t| Fill {
                        price: t.entity.price,
                        qty: t.entity.executed_base_qty,
                        commission: t.entity.commission,
                        commission_asset: t.entity.commission_asset,
                        trade_id: t.entity.trade_id.as_u64(),
                    })
                    .collect()
            })
            .unwrap_or_default();

        NewOrderFull { base, fills }
    }
}

impl CmdHandlerForUpdate2 for PlaceOrderCmdHandler {
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        // let quantity = cmd.quantity.unwrap_or_default();
        // if quantity.is_zero() {
        //     return Err(SpotCmdErrorAny("quantity must be greater than 0".to_string()));
        // }
        //
        // if cmd.order_type == OrderType::Limit && cmd.price.is_none() {
        //     return Err(SpotCmdErrorAny("price is required for limit orders".to_string()));
        // }

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
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        // if let Some(ref order_event) = domain_events.order {
        //     self.repo
        //         .put_obj(
        //             &[b"order", &order_event.entity.order_id.to_le_bytes()],
        //             &order_event.entity,
        //         )
        //         .map_err(|e| SpotCmdErrorAny(e.to_string()))?;
        // }
        //
        // if let Some(ref trades) = domain_events.trades {
        //     for trade_event in trades {
        //         self.repo
        //             .put_obj(
        //                 &[b"trade", &trade_event.entity.trade_id.to_le_bytes()],
        //                 &trade_event.entity,
        //             )
        //             .map_err(|e| SpotCmdErrorAny(e.to_string()))?;
        //     }
        // }

        Ok(())
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        if let Some(ref order_event) = domain_events.order {
            self.repo.replay_event(&order_event);
        }

        if let Some(ref trades) = domain_events.trades {
            for trade_event in trades {
                self.repo.replay_event(&trade_event);
            }
        }

        if let Some(ref balances) = domain_events.balances {
            for trade_event in balances {
                self.repo.replay_event(&trade_event);
            }
        }

        Ok(())
    }

    fn publish_domain_events(
        &self,
        _domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        todo!("发布 domain events 到消息队列")
    }
}
