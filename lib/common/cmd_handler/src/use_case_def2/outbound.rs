use diff::EntityReplayableEvent;

/// Use case 视角下统一的 outbound port。
///
/// `load_state / persist / replay / publish` 都属于 adapter.outbound，
/// executor 只依赖这一个抽象。
pub trait CommandUseCaseOutbound: Send + Sync {
    type Command;
    type State;
    type Error: std::error::Error;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error>;

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandUseCaseOutboundPhase {
    LoadState,
    Persist,
    Replay,
    Publish,
}

impl std::fmt::Display for CommandUseCaseOutboundPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadState => f.write_str("load_state"),
            Self::Persist => f.write_str("persist"),
            Self::Replay => f.write_str("replay"),
            Self::Publish => f.write_str("publish"),
        }
    }
}

impl<T> CommandUseCaseOutbound for &T
where
    T: ?Sized + CommandUseCaseOutbound,
{
    type Command = T::Command;
    type State = T::State;
    type Error = T::Error;

    fn load_state(&self, cmd: &Self::Command) -> Result<Self::State, Self::Error> {
        (*self).load_state(cmd)
    }

    fn persist(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).persist(events)
    }

    fn replay(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).replay(events)
    }

    fn publish(&self, events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
        (*self).publish(events)
    }
}
