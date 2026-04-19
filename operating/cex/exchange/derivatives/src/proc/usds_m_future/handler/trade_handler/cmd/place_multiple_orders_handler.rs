use cmd_handler::{CmdHandlerForUpdate3, CmdHandlerInternal};
use db_repo::{CmdRepo2, EventPublisher2};

use crate::proc::usds_m_future::handler::trade_handler::EmptyStateSet;
use crate::proc::usds_m_future::behavior::trade_behavior::{
    PlaceMultipleOrdersCmd, UsdsMFutureTradeCmdError,
};

pub struct PlaceMultipleOrdersCmdHandler<R: CmdRepo2, P: EventPublisher2> {
    pub repo: R,
    pub publisher: P,
}

impl<R: CmdRepo2, P: EventPublisher2> PlaceMultipleOrdersCmdHandler<R, P> {
    pub fn new(repo: R, publisher: P) -> Self {
        Self { repo, publisher }
    }
}

impl<R: CmdRepo2, P: EventPublisher2> CmdHandlerInternal for PlaceMultipleOrdersCmdHandler<R, P> {
    type Command = PlaceMultipleOrdersCmd;
    type Reply = ();
    type GivenStateSet = ();
    type ThenStateSet = EmptyStateSet;
    type Error = UsdsMFutureTradeCmdError;

    type Repo = R;
    type Publisher = P;

    fn then(
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

    fn give(
        &self,
        _cmd: &Self::Command,
        _repo: &Self::Repo,
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
        _repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn replay_domain_events_to_state(
        &self,
        _domain_events: &Self::ThenStateSet,
        _repo: &Self::Repo,
    ) -> Result<(), Self::Error> {
        todo!()
    }

    fn publish_domain_events(
        &self,
        _domain_events: &Self::ThenStateSet,
        _publisher: Self::Publisher,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<R: CmdRepo2, P: EventPublisher2> CmdHandlerForUpdate3 for PlaceMultipleOrdersCmdHandler<R, P> {}
