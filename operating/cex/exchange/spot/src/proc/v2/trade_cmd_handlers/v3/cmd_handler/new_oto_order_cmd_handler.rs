use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler_update2::{
    CmdHandlerForUpdate2, CmdHandlerInternal, DomainEventSet,
};
use diff::diff_types::DomainEvent;

use crate::proc::behavior::v2::spot_trade_behavior::{NewOtoOrderCmd, OtoOrderResult};
use crate::proc::behavior::v2::spot_trade_error::SpotApiErrorAny;

#[derive(Debug, Clone)]
pub struct NewOtoOrderStateSet {
    pub order_list_id: u64,
}

pub struct NewOtoOrderStateChangedSet {
    pub orders: Vec<DomainEvent<SpotOrder>>,
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
    pub balances: Option<Vec<DomainEvent<Balance>>>,
}

impl DomainEventSet for NewOtoOrderStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        let mut count = self.orders.len();
        if let Some(ref trades) = self.trades {
            count += trades.len();
        }
        if let Some(ref balances) = self.balances {
            count += balances.len();
        }
        count
    }
}

pub struct NewOtoOrderCmdHandler;

impl NewOtoOrderCmdHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NewOtoOrderCmdHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CmdHandlerInternal for NewOtoOrderCmdHandler {
    type Command = NewOtoOrderCmd;
    type Reply = OtoOrderResult;
    type GivenStateSet = NewOtoOrderStateSet;
    type ThenStateSet = NewOtoOrderStateChangedSet;
    type Error = SpotApiErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        _cmd: &Self::Command,
        _state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        todo!()
    }

    fn state_changed_set_to_reply(&self, _state_changed_set: Self::ThenStateSet) -> Self::Reply {
        todo!()
    }
    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        todo!()
    }

    fn load_state_set_for_update(
        &self,
        _cmd: &Self::Command,
    ) -> Result<Self::GivenStateSet, Self::Error> {
        todo!()
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Self::Command,
        _state_set: &Self::GivenStateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn persist_domain_events(
        &self,
        _domain_events: &Self::ThenStateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn replay_domain_events_to_state(
        &self,
        _domain_events: &Self::ThenStateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn publish_domain_events(
        &self,
        _domain_events: &Self::ThenStateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl CmdHandlerForUpdate2 for NewOtoOrderCmdHandler {}
