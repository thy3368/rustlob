use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    ExecutionError, StateMachineExecutor, StateSink, StateSource, UseCaseReplyMapper,
};
use example_core_use_case::{
    PlaceMatchSpotOrderV2Error, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
};
use serde::Serialize;

use crate::common::{ExampleCliParseErrorMapping, find_last_u64_field, find_u64_field};

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
    fn into_command(self) -> PlaceOnlySpotOrderV2Cmd {
        PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
            order_id: 11,
            party_id: self.trader_id,
            asset: 10_001,
            symbol: self.symbol,
            is_buy: true,
            price: self.price.to_string(),
            size: self.qty.to_string(),
            order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "Gtc".to_string() },
            reduce_only: false,
            cloid: None,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        })
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
    pub order_id: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct PlaceOrderCliReplyMapper;

impl UseCaseReplyMapper for PlaceOrderCliReplyMapper {
    type Reply = PlaceOrderCliResponse;

    fn map(&self, events: Vec<EntityReplayableEvent>) -> Self::Reply {
        let order_id = find_u64_field(&events, "order_id").unwrap_or(0);
        let principal_reservation_amount =
            find_last_u64_field(&events, "reservation_original_amount").unwrap_or(0);
        let remaining_quote = find_u64_field(&events, "available").unwrap_or(0);

        PlaceOrderCliResponse {
            summary: format!(
                "accepted order_id={order_id} principal_reservation_amount={principal_reservation_amount} remaining_quote={remaining_quote}"
            ),
            order_id,
        }
    }
}

pub fn run_place_order_cli<OB>(
    command: PlaceOrderCliCommand,
    outbound: &OB,
) -> Result<
    PlaceOrderCliResponse,
    ExecutionError<
        PlaceMatchSpotOrderV2Error,
        <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
    >,
>
where
    OB: StateSource<
            PlaceMatchSpotOrderV2UseCase,
            Error = <OB as StateSink<PlaceMatchSpotOrderV2UseCase>>::Error,
        > + StateSink<PlaceMatchSpotOrderV2UseCase>,
{
    let command = command.into_command();
    let result = StateMachineExecutor.execute::<PlaceMatchSpotOrderV2UseCase, OB, OB>(
        &PlaceMatchSpotOrderV2UseCase,
        &command,
        outbound,
        outbound,
    )?;
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
            run_place_order_cli(command, &outbound).expect("place-match order should execute");
        let counts = outbound.snapshot_event_counts()?;

        assert_eq!(response.order_id, 11);
        assert_eq!(
            response.summary,
            "accepted order_id=11 principal_reservation_amount=200 remaining_quote=800"
        );
        assert_eq!(counts, (6, 6));

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
