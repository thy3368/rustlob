use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler_update2::{
    ApplyCommandChanges2, CmdHandlerForUpdate2, DomainEventSet,
};
use diff::diff_types::DomainEvent;

use crate::proc::behavior::spot_trade_behavior::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior_v2::NewOrderCmd;

// 状态
#[derive(Debug, Clone)]
pub struct PlaceOrderStateSet {
    pub order_id: u64,
}

// Handler 实现
pub struct PlaceOrderCmdHandler;

impl PlaceOrderCmdHandler {
    pub fn new() -> Self {
        Self
    }
}

pub struct StateChangedSet3 {
    pub order: DomainEvent<SpotOrder>,
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
    pub balances: Option<Vec<DomainEvent<Balance>>>,
}

impl StateChangedSet3 {
    #[inline]
    pub fn domain_event_count(&self) -> usize {
        todo!()
        // 1 + self.trades.len() + self.balances.len()
    }
}

impl DomainEventSet for StateChangedSet3 {
    #[inline]
    fn domain_event_count(&self) -> usize {
        self.domain_event_count()
    }
}

impl ApplyCommandChanges2 for PlaceOrderCmdHandler {
    type Command = NewOrderCmd;
    type StateSet = PlaceOrderStateSet;
    type StateChangedSet = StateChangedSet3;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Self::Command,
        state_set: Self::StateSet,
    ) -> Result<Self::StateChangedSet, Self::Error> {
        todo!()
    }
}

impl CmdHandlerForUpdate2 for PlaceOrderCmdHandler {
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        todo!()
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Self::Command,
    ) -> Result<Self::StateSet, Self::Error> {
        todo!()
    }

    fn validate_command_in_lock(
        &self,
        cmd: &Self::Command,
        state_set: &Self::StateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn persist_domain_events(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn publish_domain_events(
        &self,
        domain_events: &Self::StateChangedSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
