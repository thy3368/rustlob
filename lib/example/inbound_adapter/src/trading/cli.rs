use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    MiFamilyExecutionError, MiFamilyOutbound, MiStateMachineFamilyExecutor, UseCaseReplyMapper,
};
use example_core::{
    PlaceSpotOrderV2CmdV3, SpotOrderV2CommandV3, SpotOrderV2UseCaseFamilyV3,
    SpotOrderV2UseCaseFamilyV3Error,
};
use serde::Serialize;

use crate::common::{ExampleCliParseErrorMapping, find_string_field, find_u64_field};

pub const PLACE_ORDER_CLI_BIN: &str = "cli_demo";
pub const PLACE_ORDER_CLI_DEFAULT_TRADER_ID: &str = "trader-1";
pub const PLACE_ORDER_CLI_DEFAULT_SYMBOL: &str = "BTCUSDT";
pub const PLACE_ORDER_CLI_DEFAULT_QTY: u64 = 2;
pub const PLACE_ORDER_CLI_DEFAULT_PRICE: u64 = 100;
const PLACE_ORDER_CLI_USAGE: &str = "usage: cargo run -p example_inbound_adapter --example cli_demo -- <trader_id> <symbol> <qty> <price>";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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

impl ExampleCliParseErrorMapping for ParsePlaceOrderCliArgsError {
    fn cli_error_code(&self) -> &'static str {
        match self {
            Self::TooManyArgs => "too_many_args",
            Self::InvalidQty(_) => "invalid_qty",
            Self::InvalidPrice(_) => "invalid_price",
        }
    }
}

impl PlaceOrderCliCommand {
    fn into_command(self) -> SpotOrderV2CommandV3 {
        let _adapter_symbol = self.symbol;
        SpotOrderV2CommandV3::Place(PlaceSpotOrderV2CmdV3 {
            party_id: self.trader_id,
            asset: 10_001,
            is_buy: true,
            price: self.price.to_string(),
            size: self.qty.to_string(),
            tif: "Gtc".to_string(),
            cloid: None,
        })
    }
}

struct PlaceOrderCliExecutionSpec;

impl cmd_handler::command_use_case_def2::MiFamilyExecutionSpec<SpotOrderV2UseCaseFamilyV3>
    for PlaceOrderCliExecutionSpec
{
    type Request = SpotOrderV2CommandV3;

    fn command(request: &Self::Request) -> SpotOrderV2CommandV3 {
        request.clone()
    }
}

pub fn place_order_cli_usage() -> &'static str {
    PLACE_ORDER_CLI_USAGE
}

pub fn parse_place_order_cli_args<I, S>(
    args: I,
) -> Result<PlaceOrderCliCommand, ParsePlaceOrderCliArgsError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = args.into_iter().map(Into::into);

    let trader_id = args.next().unwrap_or_else(|| PLACE_ORDER_CLI_DEFAULT_TRADER_ID.to_string());
    let symbol = args.next().unwrap_or_else(|| PLACE_ORDER_CLI_DEFAULT_SYMBOL.to_string());
    let qty = parse_or_default(
        args.next(),
        PLACE_ORDER_CLI_DEFAULT_QTY,
        ParsePlaceOrderCliArgsError::InvalidQty,
    )?;
    let price = parse_or_default(
        args.next(),
        PLACE_ORDER_CLI_DEFAULT_PRICE,
        ParsePlaceOrderCliArgsError::InvalidPrice,
    )?;

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
        let remaining_quote = find_u64_field(&events, "available").unwrap_or(0);

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
) -> Result<PlaceOrderCliResponse, MiFamilyExecutionError<SpotOrderV2UseCaseFamilyV3Error, OB::Error>>
where
    OB: MiFamilyOutbound<SpotOrderV2UseCaseFamilyV3>,
{
    let command = command.into_command();
    let result = MiStateMachineFamilyExecutor.execute::<
        SpotOrderV2UseCaseFamilyV3,
        PlaceOrderCliExecutionSpec,
        OB,
    >(&SpotOrderV2UseCaseFamilyV3, &command, outbound)?;
    Ok(PlaceOrderCliReplyMapper.map(result.events))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::PlaceOrderTestOutbound;

    #[test]
    fn cli_adapter_translates_command_and_maps_text_response()
    -> Result<(), Box<dyn std::error::Error>> {
        let outbound = PlaceOrderTestOutbound::default();
        let command = PlaceOrderCliCommand {
            trader_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 2,
            price: 100,
        };

        let response =
            run_place_order_cli(command, &outbound).expect("v3 place order should execute");
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.order_id, "trader-1-BTCUSDT-11");
        assert_eq!(
            response.summary,
            "accepted order_id=trader-1-BTCUSDT-11 reserved_quote=200 remaining_quote=800"
        );
        assert_eq!(counts, (3, 3));

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
