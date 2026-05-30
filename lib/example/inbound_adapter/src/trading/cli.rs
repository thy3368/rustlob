use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{
    CommandEnvelope, CommandMeta, CommandUseCaseOutbound, UseCaseReplyMapper,
};
use example_core::{PlaceOrderCmd, PlaceOrderError, PlaceOrderState};

use crate::common::{execute_place_order_with_mapper, find_string_field, find_u64_field};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCliCommand {
    pub trader_id: String,
    pub symbol: String,
    pub qty: u64,
    pub price: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsePlaceOrderCliArgsError {
    TooManyArgs,
    InvalidQty(String),
    InvalidPrice(String),
}

impl std::fmt::Display for ParsePlaceOrderCliArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyArgs => f.write_str("too many CLI arguments"),
            Self::InvalidQty(raw) => write!(f, "invalid qty: {raw}"),
            Self::InvalidPrice(raw) => write!(f, "invalid price: {raw}"),
        }
    }
}

impl std::error::Error for ParsePlaceOrderCliArgsError {}

impl PlaceOrderCliCommand {
    fn into_envelope(self) -> CommandEnvelope<PlaceOrderCmd> {
        CommandEnvelope {
            meta: CommandMeta {
                trace_id: Some("cli-place-order".to_string()),
                command_id: Some(format!("cli:{}:{}:{}", self.trader_id, self.symbol, self.price)),
            },
            command: PlaceOrderCmd {
                party_id: self.trader_id,
                symbol: self.symbol,
                qty: self.qty,
                price: self.price,
            },
        }
    }
}

pub fn place_order_cli_usage() -> &'static str {
    "usage: cargo run -p example_composition_root --bin cli_demo -- <trader_id> <symbol> <qty> <price>"
}

pub fn parse_place_order_cli_args<I, S>(
    args: I,
) -> Result<PlaceOrderCliCommand, ParsePlaceOrderCliArgsError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);

    let trader_id = args.next().unwrap_or_else(|| "trader-1".to_string());
    let symbol = args.next().unwrap_or_else(|| "BTCUSDT".to_string());
    let qty = parse_or_default(args.next(), 2, ParsePlaceOrderCliArgsError::InvalidQty)?;
    let price = parse_or_default(args.next(), 100, ParsePlaceOrderCliArgsError::InvalidPrice)?;

    if args.next().is_some() {
        return Err(ParsePlaceOrderCliArgsError::TooManyArgs);
    }

    Ok(PlaceOrderCliCommand { trader_id, symbol, qty, price })
}

fn parse_or_default(
    value: Option<String>,
    default_value: u64,
    error_mapper: impl FnOnce(String) -> ParsePlaceOrderCliArgsError + Copy,
) -> Result<u64, ParsePlaceOrderCliArgsError> {
    match value {
        Some(raw) => raw.parse::<u64>().map_err(|_| error_mapper(raw)),
        None => Ok(default_value),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceOrderCliResponse {
    pub summary: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Copy, Default)]
struct PlaceOrderCliReplyMapper;

impl UseCaseReplyMapper for PlaceOrderCliReplyMapper {
    type Reply = PlaceOrderCliResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        let order_id = find_string_field(&events, "order_id")
            .unwrap_or_else(|| "missing-order-id".to_string());
        let reserved_quote = find_u64_field(&events, "reserved_quote").unwrap_or(0);
        let remaining_quote = find_u64_field(&events, "available_quote").unwrap_or(0);

        PlaceOrderCliResponse {
            summary: format!(
                "accepted order_id={order_id} reserved_quote={reserved_quote} remaining_quote={remaining_quote}"
            ),
            order_id,
        }
    }
}

pub fn run_place_order_cli<OB>(
    command: PlaceOrderCliCommand,
    outbound: &OB,
) -> Result<PlaceOrderCliResponse, PlaceOrderError>
where
    OB: ?Sized
        + Send
        + Sync
        + CommandUseCaseOutbound<PlaceOrderCmd, PlaceOrderState, PlaceOrderError>,
{
    execute_place_order_with_mapper(command.into_envelope(), outbound, &PlaceOrderCliReplyMapper)
}

#[cfg(test)]
mod tests {
    use example_core::PlaceOrderError;

    use super::*;
    use crate::common::tests::TestOutbound;

    #[test]
    fn cli_adapter_translates_command_and_maps_text_response() -> Result<(), PlaceOrderError> {
        let outbound = TestOutbound::default();
        let command = PlaceOrderCliCommand {
            trader_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 2,
            price: 100,
        };

        let response = run_place_order_cli(command, &outbound)?;
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.order_id, "trader-1-BTCUSDT-11");
        assert_eq!(
            response.summary,
            "accepted order_id=trader-1-BTCUSDT-11 reserved_quote=200 remaining_quote=800"
        );
        assert_eq!(counts, (2, 2));

        Ok(())
    }

    #[test]
    fn parse_cli_args_supports_defaults() -> Result<(), ParsePlaceOrderCliArgsError> {
        let command = parse_place_order_cli_args(std::iter::empty::<String>())?;

        assert_eq!(command.trader_id, "trader-1");
        assert_eq!(command.symbol, "BTCUSDT");
        assert_eq!(command.qty, 2);
        assert_eq!(command.price, 100);

        Ok(())
    }

    #[test]
    fn parse_cli_args_rejects_too_many_args() {
        let result = parse_place_order_cli_args(["a", "b", "1", "2", "extra"]);

        assert_eq!(result, Err(ParsePlaceOrderCliArgsError::TooManyArgs));
    }
}
