use base_types::account::balance::Balance;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade};
use base_types::handler::handler_update2::{
    CmdHandlerForUpdate2, CmdHandlerInternal, DomainEventSet,
};
use diff::diff_types::DomainEvent;

use crate::proc::behavior::v2::spot_trade_error::SpotCmdErrorAny;
use crate::proc::behavior::v2::spot_trade_behavior::{
    CancelReplaceOrderCmd, CancelReplaceResult,
};

#[derive(Debug, Clone)]
pub struct CancelReplaceOrderStateSet {
    pub order_id: u64,
}

pub struct CancelReplaceOrderStateChangedSet {
    pub cancelled_order: Option<DomainEvent<SpotOrder>>,
    pub new_order: Option<DomainEvent<SpotOrder>>,
    pub trades: Option<Vec<DomainEvent<SpotTrade>>>,
    pub balances: Option<Vec<DomainEvent<Balance>>>,
}

impl DomainEventSet for CancelReplaceOrderStateChangedSet {
    #[inline]
    fn domain_event_count(&self) -> usize {
        let mut count = 0;
        if self.cancelled_order.is_some() {
            count += 1;
        }
        if self.new_order.is_some() {
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

pub struct CancelReplaceOrderCmdHandler;

impl CancelReplaceOrderCmdHandler {
    pub fn new() -> Self {
        Self
    }
}

impl CmdHandlerInternal for CancelReplaceOrderCmdHandler {
    type Command = CancelReplaceOrderCmd;
    type Reply = CancelReplaceResult;
    type GivenStateSet = CancelReplaceOrderStateSet;
    type ThenStateSet = CancelReplaceOrderStateChangedSet;
    type Error = SpotCmdErrorAny;

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Self::Command,
        state_set: Self::GivenStateSet,
    ) -> Result<Self::ThenStateSet, Self::Error> {
        todo!()
    }

    fn state_changed_set_to_reply(&self, state_changed_set: Self::ThenStateSet) -> Self::Reply {
        todo!()
    }
    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        todo!()
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Self::Command,
    ) -> Result<Self::GivenStateSet, Self::Error> {
        todo!()
    }

    fn validate_command_in_lock(
        &self,
        cmd: &Self::Command,
        state_set: &Self::GivenStateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn persist_domain_events(&self, domain_events: &Self::ThenStateSet) -> Result<(), Self::Error> {
        todo!()
    }

    fn replay_domain_events_to_state(
        &self,
        domain_events: &Self::ThenStateSet,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn publish_domain_events(&self, domain_events: &Self::ThenStateSet) -> Result<(), Self::Error> {
        todo!()
    }
}

impl CmdHandlerForUpdate2 for CancelReplaceOrderCmdHandler {}
