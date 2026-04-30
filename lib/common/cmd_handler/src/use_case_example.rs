use super::use_case_def::{
    CommandUseCase, CommandUseCaseExecutor, DomainEventPipeline, LoadState, UseCaseReplyMapper,
};
use crate::DomainEventSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCommand {
    pub asset: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOrderError {
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderStateSnapshot {
    pub can_place: bool,
}

pub trait PlaceOrderLoadPort: Send + Sync {
    fn load_place_order_state(
        &self,
        cmd: &PlaceOrderCommand,
    ) -> Result<PlaceOrderStateSnapshot, PlaceOrderError>;
}

impl LoadState<PlaceOrderCommand, PlaceOrderState, PlaceOrderError> for dyn PlaceOrderLoadPort {
    fn load_state(&self, cmd: &PlaceOrderCommand) -> Result<PlaceOrderState, PlaceOrderError> {
        let snapshot = self.load_place_order_state(cmd)?;
        Ok(PlaceOrderState {
            can_place: snapshot.can_place,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderState {
    pub can_place: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderEvents {
    pub accepted: bool,
}

impl DomainEventSet for PlaceOrderEvents {
    fn domain_event_count(&self) -> usize {
        usize::from(self.accepted)
    }
}

pub struct PlaceOrderUseCase;

impl CommandUseCase for PlaceOrderUseCase {
    type Command = PlaceOrderCommand;
    type GivenState = PlaceOrderState;
    type Events = PlaceOrderEvents;
    type Error = PlaceOrderError;
    type LoadPort = dyn PlaceOrderLoadPort;

    fn actor(&self) -> &'static str {
        "OrderCheckingEngine"
    }

    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.can_place {
            Ok(())
        } else {
            Err(PlaceOrderError::Rejected)
        }
    }

    fn then_event_4_new_state(
        &self,
        _cmd: &Self::Command,
        _state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error> {
        Ok(PlaceOrderEvents { accepted: true })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderReply {
    pub accepted: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceOrderReplyMapper;

impl UseCaseReplyMapper<PlaceOrderEvents> for PlaceOrderReplyMapper {
    type Reply = PlaceOrderReply;

    fn map(&self, events: PlaceOrderEvents) -> Self::Reply {
        PlaceOrderReply {
            accepted: events.accepted,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoopDomainEventPipeline;

impl<E, Err> DomainEventPipeline<E, Err> for NoopDomainEventPipeline {
    fn persist(&self, _events: &E) -> Result<(), Err> {
        Ok(())
    }

    fn replay(&self, _events: &E) -> Result<(), Err> {
        Ok(())
    }

    fn publish(&self, _events: &E) -> Result<(), Err> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StubPlaceOrderLoadPort;

impl PlaceOrderLoadPort for StubPlaceOrderLoadPort {
    fn load_place_order_state(
        &self,
        _cmd: &PlaceOrderCommand,
    ) -> Result<PlaceOrderStateSnapshot, PlaceOrderError> {
        Ok(PlaceOrderStateSnapshot { can_place: true })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RejectPlaceOrderLoadPort;

impl PlaceOrderLoadPort for RejectPlaceOrderLoadPort {
    fn load_place_order_state(
        &self,
        _cmd: &PlaceOrderCommand,
    ) -> Result<PlaceOrderStateSnapshot, PlaceOrderError> {
        Ok(PlaceOrderStateSnapshot { can_place: false })
    }
}

#[derive(Debug, Default)]
pub struct SpyDomainEventPipeline {
    calls: std::sync::Mutex<Vec<&'static str>>,
}

impl SpyDomainEventPipeline {
    pub fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().unwrap().clone()
    }
}

impl DomainEventPipeline<PlaceOrderEvents, PlaceOrderError> for SpyDomainEventPipeline {
    fn persist(&self, _events: &PlaceOrderEvents) -> Result<(), PlaceOrderError> {
        self.calls.lock().unwrap().push("persist");
        Ok(())
    }

    fn replay(&self, _events: &PlaceOrderEvents) -> Result<(), PlaceOrderError> {
        self.calls.lock().unwrap().push("replay");
        Ok(())
    }

    fn publish(&self, _events: &PlaceOrderEvents) -> Result<(), PlaceOrderError> {
        self.calls.lock().unwrap().push("publish");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_place_order_when_loaded_state_allows_placing() {
        let executor = CommandUseCaseExecutor;
        let use_case = PlaceOrderUseCase;
        let load_port = StubPlaceOrderLoadPort;
        let pipeline = NoopDomainEventPipeline;

        let events = executor
            .execute(&use_case, PlaceOrderCommand { asset: 1 }, &load_port, &pipeline)
            .unwrap();

        assert_eq!(events, PlaceOrderEvents { accepted: true });
        assert_eq!(events.domain_event_count(), 1);
    }

    #[test]
    fn reject_place_order_when_loaded_state_disallows_placing() {
        let executor = CommandUseCaseExecutor;
        let use_case = PlaceOrderUseCase;
        let load_port = RejectPlaceOrderLoadPort;
        let pipeline = NoopDomainEventPipeline;

        let error = executor
            .execute(&use_case, PlaceOrderCommand { asset: 1 }, &load_port, &pipeline)
            .unwrap_err();

        assert_eq!(error, PlaceOrderError::Rejected);
    }

    #[test]
    fn map_reply_outside_use_case_core() {
        let executor = CommandUseCaseExecutor;
        let use_case = PlaceOrderUseCase;
        let load_port = StubPlaceOrderLoadPort;
        let pipeline = NoopDomainEventPipeline;
        let mapper = PlaceOrderReplyMapper;

        let reply = executor
            .execute_and_map_reply(
                &use_case,
                PlaceOrderCommand { asset: 1 },
                &load_port,
                &pipeline,
                &mapper,
            )
            .unwrap();

        assert_eq!(reply, PlaceOrderReply { accepted: true });
    }

    #[test]
    fn execute_runs_pipeline_in_order() {
        let executor = CommandUseCaseExecutor;
        let use_case = PlaceOrderUseCase;
        let load_port = StubPlaceOrderLoadPort;
        let pipeline = SpyDomainEventPipeline::default();

        executor
            .execute(&use_case, PlaceOrderCommand { asset: 1 }, &load_port, &pipeline)
            .unwrap();

        assert_eq!(pipeline.calls(), vec!["persist", "replay", "publish"]);
    }
}
