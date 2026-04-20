use crate::{DomainEventSet, HandlerLatencyMetrics};

/// 更贴近 Use Cases（用例）的命令型抽象：
/// 只定义业务输入、状态装载、业务校验与领域事件产出。
pub trait CommandUseCase: Send + Sync {
    type Command;
    type GivenState;
    type Events: DomainEventSet;
    type Error;
    type LoadPort: ?Sized + Send + Sync;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error>;

    fn load_state(
        &self,
        cmd: &Self::Command,
        load_port: &Self::LoadPort,
    ) -> Result<Self::GivenState, Self::Error>;

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error>;

    fn then(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error>;

    fn observe_latency(&self, _metrics: &HandlerLatencyMetrics) {}
}

/// 对外回复映射移出核心 Use Case，交给 Interface Adapters（接口适配器）。
pub trait UseCaseReplyMapper<E>: Send + Sync {
    type Reply;

    fn map(&self, events: E) -> Self::Reply;
}

/// 事件执行管线也从核心 Use Case 中拆出。
pub trait DomainEventPipeline<E, Err>: Send + Sync {
    fn persist(&self, events: &E) -> Result<(), Err>;

    fn replay(&self, events: &E) -> Result<(), Err>;

    fn publish(&self, events: &E) -> Result<(), Err>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CommandUseCaseExecutor;

impl CommandUseCaseExecutor {
    pub fn execute<U, P>(
        &self,
        use_case: &U,
        command: U::Command,
        load_port: &U::LoadPort,
        pipeline: &P,
    ) -> Result<U::Events, U::Error>
    where
        U: CommandUseCase,
        P: DomainEventPipeline<U::Events, U::Error>,
    {
        use minstant::Instant;

        let total_start = Instant::now();

        let pre_check_start = Instant::now();
        use_case.pre_check_command(&command)?;
        let pre_check_ns = pre_check_start.elapsed().as_nanos();

        let load_state_start = Instant::now();
        let state = use_case.load_state(&command, load_port)?;
        let load_state_ns = load_state_start.elapsed().as_nanos();

        let validate_start = Instant::now();
        use_case.validate_against_state(&command, &state)?;
        let validate_in_lock_ns = validate_start.elapsed().as_nanos();

        let then_start = Instant::now();
        let events = use_case.then(&command, state)?;
        let apply_changes_ns = then_start.elapsed().as_nanos();

        let persist_start = Instant::now();
        pipeline.persist(&events)?;
        let persist_domain_events_ns = persist_start.elapsed().as_nanos();

        let replay_start = Instant::now();
        pipeline.replay(&events)?;
        let replay_domain_events_ns = replay_start.elapsed().as_nanos();

        let publish_start = Instant::now();
        pipeline.publish(&events)?;
        let publish_domain_events_ns = publish_start.elapsed().as_nanos();

        let metrics = HandlerLatencyMetrics {
            total_ns: total_start.elapsed().as_nanos(),
            pre_check_ns,
            load_state_ns,
            validate_in_lock_ns,
            apply_changes_ns,
            persist_domain_events_ns,
            replay_domain_events_ns,
            publish_domain_events_ns,
            domain_event_count: events.domain_event_count(),
        };

        use_case.observe_latency(&metrics);
        Ok(events)
    }

    pub fn execute_and_map_reply<U, P, M>(
        &self,
        use_case: &U,
        command: U::Command,
        load_port: &U::LoadPort,
        pipeline: &P,
        mapper: &M,
    ) -> Result<M::Reply, U::Error>
    where
        U: CommandUseCase,
        P: DomainEventPipeline<U::Events, U::Error>,
        M: UseCaseReplyMapper<U::Events>,
    {
        let events = self.execute(use_case, command, load_port, pipeline)?;
        Ok(mapper.map(events))
    }
}

/// -------- Example below --------

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCommand {
    pub asset: u32,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOrderError {
    Rejected,
}

pub trait PlaceOrderLoadPort: Send + Sync {
    fn load_place_order_state(
        &self,
        cmd: &PlaceOrderCommand,
    ) -> Result<PlaceOrderState, PlaceOrderError>;
}

pub struct PlaceOrderUseCase;

impl CommandUseCase for PlaceOrderUseCase {
    type Command = PlaceOrderCommand;
    type GivenState = PlaceOrderState;
    type Events = PlaceOrderEvents;
    type Error = PlaceOrderError;
    type LoadPort = dyn PlaceOrderLoadPort;

    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn load_state(
        &self,
        cmd: &Self::Command,
        load_port: &Self::LoadPort,
    ) -> Result<Self::GivenState, Self::Error> {
        load_port.load_place_order_state(cmd)
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

    fn then(
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
    ) -> Result<PlaceOrderState, PlaceOrderError> {
        Ok(PlaceOrderState { can_place: true })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_returns_domain_events() {
        let executor = CommandUseCaseExecutor;
        let use_case = PlaceOrderUseCase;
        let load_port = StubPlaceOrderLoadPort;
        let pipeline = NoopDomainEventPipeline;

        let events = executor
            .execute(&use_case, PlaceOrderCommand { asset: 1 }, &load_port, &pipeline)
            .unwrap();

        assert_eq!(events, PlaceOrderEvents { accepted: true });
    }

    #[test]
    fn execute_and_map_reply_maps_events_outside_use_case() {
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
}
