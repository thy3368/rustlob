mod common;
mod descriptor;
mod funding;
mod trading;

pub use common::{
    CliInboundError, CliInboundErrorCategory, ExampleBusinessErrorMapping,
    ExampleCliParseErrorMapping, ExampleHttpErrorBody, ExampleHttpErrorResponse, HttpInboundError,
    outbound_phase_code,
};
pub use descriptor::{
    API_MANIFEST_REL_PATH, CLI_SCHEMA_REL_PATH, CliApiDescriptor, CliArgDescriptor,
    CliErrorCodeDescriptor, HttpApiDescriptor, HttpErrorCodeDescriptor, InboundApiDescriptor,
    OPENAPI_REL_PATH, example_api_manifest, example_cli_schema, example_http_openapi,
    example_inbound_descriptors,
};
pub use funding::{
    DEPOSIT_QUOTE_CLI_BIN, DEPOSIT_QUOTE_CLI_DEFAULT_AMOUNT, DEPOSIT_QUOTE_CLI_DEFAULT_TRADER_ID,
    DepositQuoteCliCommand, DepositQuoteCliResponse, DepositQuoteHttpRequest,
    DepositQuoteHttpResponse, DepositQuoteOutboundAccess, ParseDepositQuoteCliArgsError,
    ParseWithdrawQuoteCliArgsError, WITHDRAW_QUOTE_CLI_BIN, WITHDRAW_QUOTE_CLI_DEFAULT_AMOUNT,
    WITHDRAW_QUOTE_CLI_DEFAULT_TRADER_ID, WithdrawQuoteCliCommand, WithdrawQuoteCliResponse,
    WithdrawQuoteHttpRequest, WithdrawQuoteHttpResponse, WithdrawQuoteOutboundAccess,
    build_deposit_actix_scope, build_deposit_http_router, build_withdraw_actix_scope,
    build_withdraw_http_router, deposit_quote_cli_usage, handle_deposit_quote_http,
    handle_withdraw_quote_http, parse_deposit_quote_cli_args, parse_withdraw_quote_cli_args,
    run_deposit_quote_cli, run_withdraw_quote_cli, withdraw_quote_cli_usage,
};
pub use trading::{
    PLACE_ORDER_CLI_BIN, PLACE_ORDER_CLI_DEFAULT_PRICE, PLACE_ORDER_CLI_DEFAULT_QTY,
    PLACE_ORDER_CLI_DEFAULT_SYMBOL, PLACE_ORDER_CLI_DEFAULT_TRADER_ID, ParsePlaceOrderCliArgsError,
    PlaceOrderCliCommand, PlaceOrderCliResponse, PlaceOrderHttpRequest, PlaceOrderHttpResponse,
    PlaceOrderOutboundAccess, build_orders_actix_scope, build_orders_http_router,
    handle_place_order_http, parse_place_order_cli_args, place_order_cli_usage,
    run_place_order_cli,
};
