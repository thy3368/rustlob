# Minimal Rust Use Case Skeleton Template

> Purpose: 在设计收敛后，用最小 Rust 类型骨架验证 Clean Architecture 边界。该模板不是完整实现，不应包含 HTTP / JSON / signer / gateway 细节。

```rust
use cmd_handler::{
    use_case_def::{CommandUseCase, DomainEventPipeline, UseCaseReplyMapper},
    DomainEventSet,
};

pub struct XxxUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XxxCommand {
    // business input fields
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XxxGivenState {
    // state needed for validation
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XxxEvents {
    // domain events or accepted business intents
}

impl DomainEventSet for XxxEvents {
    fn domain_event_count(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XxxError {
    InvalidCommand,
    StateNotFound,
    BusinessRuleViolated,
}

pub trait XxxLoadPort: Send + Sync {
    fn load_state(&self, cmd: &XxxCommand) -> Result<XxxGivenState, XxxError>;
}

impl CommandUseCase for XxxUseCase {
    type Command = XxxCommand;
    type GivenState = XxxGivenState;
    type Events = XxxEvents;
    type Error = XxxError;
    type LoadPort = dyn XxxLoadPort;

    fn pre_check_command(&self, _cmd: &Self::Command) -> Result<(), Self::Error> {
        Ok(())
    }

    fn load_state(
        &self,
        cmd: &Self::Command,
        load_port: &Self::LoadPort,
    ) -> Result<Self::GivenState, Self::Error> {
        load_port.load_state(cmd)
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        _state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn then(
        &self,
        _cmd: &Self::Command,
        _state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error> {
        Ok(XxxEvents {})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XxxReply {
    // adapter-facing reply
}

pub struct XxxReplyMapper;

impl UseCaseReplyMapper<XxxEvents> for XxxReplyMapper {
    type Reply = XxxReply;

    fn map(&self, _events: XxxEvents) -> Self::Reply {
        XxxReply {}
    }
}

pub struct XxxDomainEventPipeline;

impl DomainEventPipeline<XxxEvents, XxxError> for XxxDomainEventPipeline {
    fn persist(&self, _events: &XxxEvents) -> Result<(), XxxError> {
        Ok(())
    }

    fn replay(&self, _events: &XxxEvents) -> Result<(), XxxError> {
        Ok(())
    }

    fn publish(&self, _events: &XxxEvents) -> Result<(), XxxError> {
        Ok(())
    }
}
```

## Replacement Checklist

Replace `Xxx` with the concrete use case name, then fill:

- `XxxCommand` or use an existing command type
- `XxxGivenState`
- `XxxEvents`
- `XxxError`
- `XxxLoadPort`
- `XxxReply` only if adapter-facing reply is needed

## Keep Out of This Skeleton

- HTTP endpoint paths
- JSON field shape
- reqwest client
- wallet signing
- exchange raw response DTOs
- database schema
- real gateway implementation
