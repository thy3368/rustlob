use thiserror::Error;

use crate::command_use_case_def2::{
    CommandUseCase3, IssuedByParty, UseCaseOutput, UseCaseReplyMapper3,
};
use crate::{EntityReplayableEvent, ReplayFieldChange};

const PLACE_ORDER_ENTITY_TYPE: u8 = 1;
const FIELD_TYPE_STRING: u8 = 0;
const FIELD_TYPE_INT: u8 = 1;

fn stable_entity_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

fn string_field(name: &str, value: &str) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.as_bytes(),
        FIELD_TYPE_STRING,
    )
}

fn int_field(name: &str, value: u64) -> ReplayFieldChange {
    ReplayFieldChange::new(
        ReplayFieldChange::field_name_from_str(name),
        &[],
        value.to_string().as_bytes(),
        FIELD_TYPE_INT,
    )
}

fn event_has_symbol(events: &[EntityReplayableEvent], symbol: &str) -> bool {
    events.iter().any(|event| {
        event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("symbol")
                && std::str::from_utf8(change.new_value_bytes()).ok() == Some(symbol)
        })
    })
}

// Shared good example for both writing and reviewing use cases:
// - `party_id` is carried by the business command.
// - `role()` names the business-game role, not a technical component.
// - `pre_check_command`, `validate_against_state`, and `compute_output_and_events`
//   each do one job.
// - typed output and emitted events come from the same business decision.
// - reply mapping stays outside the core use case.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCmd {
    pub party_id: String,
    pub symbol: String,
    pub qty: u64,
}

impl IssuedByParty for PlaceOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOrderError {
    #[error("qty must be greater than zero")]
    InvalidQty,
    #[error("trading is disabled")]
    TradingDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderState {
    pub trading_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderOutput {
    pub order_key: String,
    pub accepted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderReply {
    pub accepted: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceOrderReplyMapper;

impl UseCaseReplyMapper3 for PlaceOrderReplyMapper {
    type Output = PlaceOrderOutput;
    type Reply = PlaceOrderReply;

    fn map(&self, result: UseCaseOutput<Self::Output>) -> Self::Reply {
        PlaceOrderReply {
            accepted: result.output.accepted && event_has_symbol(&result.events, "BTCUSDT"),
        }
    }
}

pub struct PlaceOrderUseCase;

impl CommandUseCase3 for PlaceOrderUseCase {
    type Command = PlaceOrderCmd;
    type GivenState = PlaceOrderState;
    type Error = PlaceOrderError;
    type Output = PlaceOrderOutput;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.qty == 0 {
            return Err(PlaceOrderError::InvalidQty);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        _cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if state.trading_enabled { Ok(()) } else { Err(PlaceOrderError::TradingDisabled) }
    }

    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        _state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        let order_key = format!("{}:{}", cmd.party_id, cmd.symbol);
        let output = PlaceOrderOutput { order_key: order_key.clone(), accepted: true };

        let mut event = EntityReplayableEvent::new_created(
            0,
            0,
            stable_entity_id(&order_key),
            PLACE_ORDER_ENTITY_TYPE,
        );
        event.add_field_change(string_field("party_id", &cmd.party_id));
        event.add_field_change(string_field("symbol", &cmd.symbol));
        event.add_field_change(int_field("qty", cmd.qty));

        Ok(UseCaseOutput { output, events: vec![event] })
    }
}
