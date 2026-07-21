pub use inbound_adapter_support::{
    CliApiDescriptor, CliArgDescriptor, CliErrorCodeDescriptor, HttpApiDescriptor,
    HttpErrorCodeDescriptor, InboundApiDescriptor,
};
use inbound_adapter_support::{
    build_api_manifest, build_cli_schema, build_http_openapi, cli_arg, cli_error, insert_schema,
    object_schema, optional_string_schema, string_schema, u64_schema,
};
use serde_json::{Map, Value, json};

use crate::funding::{
    DEPOSIT_QUOTE_CLI_BIN, DEPOSIT_QUOTE_CLI_DEFAULT_AMOUNT, DEPOSIT_QUOTE_CLI_DEFAULT_TRADER_ID,
    WITHDRAW_QUOTE_CLI_BIN, WITHDRAW_QUOTE_CLI_DEFAULT_AMOUNT,
    WITHDRAW_QUOTE_CLI_DEFAULT_TRADER_ID, create_deposit_http_api_descriptor,
    create_withdraw_http_api_descriptor, deposit_quote_cli_usage, withdraw_quote_cli_usage,
};
use crate::trading::{
    PLACE_ORDER_CLI_BIN, PLACE_ORDER_CLI_DEFAULT_PRICE, PLACE_ORDER_CLI_DEFAULT_QTY,
    PLACE_ORDER_CLI_DEFAULT_SYMBOL, PLACE_ORDER_CLI_DEFAULT_TRADER_ID,
    create_order_http_api_descriptor, place_order_cli_usage,
};

pub const OPENAPI_REL_PATH: &str = "lib/example/openapi.json";
pub const CLI_SCHEMA_REL_PATH: &str = "lib/example/cli-schema.json";
pub const API_MANIFEST_REL_PATH: &str = "lib/example/api-manifest.json";

const SERVICE_NAME: &str = "example_inbound_adapter";
const SERVICE_SUMMARY: &str =
    "Unified inbound contract manifest for the example HTTP and CLI adapters.";
const SCHEMA_VERSION: &str = "1.0.0";
const HTTP_DOC_SUMMARY: &str =
    "OpenAPI source of truth for the formal HTTP endpoints exposed by example inbound adapters.";
const CLI_DOC_SUMMARY: &str =
    "Machine-readable CLI contract for the example inbound adapter demo binaries.";

pub fn example_inbound_descriptors() -> Vec<InboundApiDescriptor> {
    vec![
        InboundApiDescriptor::Http(create_order_http_api_descriptor()),
        InboundApiDescriptor::Http(create_deposit_http_api_descriptor()),
        InboundApiDescriptor::Http(create_withdraw_http_api_descriptor()),
        InboundApiDescriptor::Cli(CliApiDescriptor {
            name: "place_order_cli",
            bin: PLACE_ORDER_CLI_BIN,
            usage: place_order_cli_usage(),
            summary: "Run the spot order demo command from the CLI.",
            args: vec![
                cli_arg(
                    "trader_id",
                    0,
                    "string",
                    false,
                    Some(json!(PLACE_ORDER_CLI_DEFAULT_TRADER_ID)),
                    "Trader account identifier.",
                ),
                cli_arg(
                    "symbol",
                    1,
                    "string",
                    false,
                    Some(json!(PLACE_ORDER_CLI_DEFAULT_SYMBOL)),
                    "Trading symbol.",
                ),
                cli_arg(
                    "qty",
                    2,
                    "u64",
                    false,
                    Some(json!(PLACE_ORDER_CLI_DEFAULT_QTY)),
                    "Requested order quantity.",
                ),
                cli_arg(
                    "price",
                    3,
                    "u64",
                    false,
                    Some(json!(PLACE_ORDER_CLI_DEFAULT_PRICE)),
                    "Requested order limit price.",
                ),
            ],
            command_schema_name: "PlaceOrderCliCommand",
            stdout_schema_name: "PlaceOrderCliResponse",
            error_codes: vec![
                cli_error("parse", "too_many_args"),
                cli_error("parse", "invalid_qty"),
                cli_error("parse", "invalid_price"),
                cli_error("business", "invalid_qty"),
                cli_error("business", "invalid_price"),
                cli_error("business", "qty_below_min"),
                cli_error("business", "trading_disabled"),
                cli_error("business", "symbol_not_tradable"),
                cli_error("business", "insufficient_quote_balance"),
                cli_error("business", "arithmetic_overflow"),
                cli_error("outbound", "outbound_load_state_failed"),
                cli_error("outbound", "outbound_persist_failed"),
                cli_error("outbound", "outbound_replay_failed"),
                cli_error("outbound", "outbound_publish_failed"),
            ],
            defaults: json!({
                "trader_id": PLACE_ORDER_CLI_DEFAULT_TRADER_ID,
                "symbol": PLACE_ORDER_CLI_DEFAULT_SYMBOL,
                "qty": PLACE_ORDER_CLI_DEFAULT_QTY,
                "price": PLACE_ORDER_CLI_DEFAULT_PRICE,
            }),
        }),
        InboundApiDescriptor::Cli(CliApiDescriptor {
            name: "deposit_quote_cli",
            bin: DEPOSIT_QUOTE_CLI_BIN,
            usage: deposit_quote_cli_usage(),
            summary: "Run the deposit quote demo command from the CLI.",
            args: vec![
                cli_arg(
                    "trader_id",
                    0,
                    "string",
                    false,
                    Some(json!(DEPOSIT_QUOTE_CLI_DEFAULT_TRADER_ID)),
                    "Trader account identifier.",
                ),
                cli_arg(
                    "amount",
                    1,
                    "u64",
                    false,
                    Some(json!(DEPOSIT_QUOTE_CLI_DEFAULT_AMOUNT)),
                    "Deposit amount.",
                ),
            ],
            command_schema_name: "DepositQuoteCliCommand",
            stdout_schema_name: "DepositQuoteCliResponse",
            error_codes: vec![
                cli_error("parse", "too_many_args"),
                cli_error("parse", "invalid_amount"),
                cli_error("business", "invalid_amount"),
                cli_error("business", "arithmetic_overflow"),
                cli_error("outbound", "outbound_load_state_failed"),
                cli_error("outbound", "outbound_persist_failed"),
                cli_error("outbound", "outbound_replay_failed"),
                cli_error("outbound", "outbound_publish_failed"),
            ],
            defaults: json!({
                "trader_id": DEPOSIT_QUOTE_CLI_DEFAULT_TRADER_ID,
                "amount": DEPOSIT_QUOTE_CLI_DEFAULT_AMOUNT,
            }),
        }),
        InboundApiDescriptor::Cli(CliApiDescriptor {
            name: "withdraw_quote_cli",
            bin: WITHDRAW_QUOTE_CLI_BIN,
            usage: withdraw_quote_cli_usage(),
            summary: "Run the withdraw quote demo command from the CLI.",
            args: vec![
                cli_arg(
                    "trader_id",
                    0,
                    "string",
                    false,
                    Some(json!(WITHDRAW_QUOTE_CLI_DEFAULT_TRADER_ID)),
                    "Trader account identifier.",
                ),
                cli_arg(
                    "amount",
                    1,
                    "u64",
                    false,
                    Some(json!(WITHDRAW_QUOTE_CLI_DEFAULT_AMOUNT)),
                    "Withdraw amount.",
                ),
            ],
            command_schema_name: "WithdrawQuoteCliCommand",
            stdout_schema_name: "WithdrawQuoteCliResponse",
            error_codes: vec![
                cli_error("parse", "too_many_args"),
                cli_error("parse", "invalid_amount"),
                cli_error("business", "invalid_amount"),
                cli_error("business", "insufficient_quote_balance"),
                cli_error("business", "arithmetic_overflow"),
                cli_error("outbound", "outbound_load_state_failed"),
                cli_error("outbound", "outbound_persist_failed"),
                cli_error("outbound", "outbound_replay_failed"),
                cli_error("outbound", "outbound_publish_failed"),
            ],
            defaults: json!({
                "trader_id": WITHDRAW_QUOTE_CLI_DEFAULT_TRADER_ID,
                "amount": WITHDRAW_QUOTE_CLI_DEFAULT_AMOUNT,
            }),
        }),
    ]
}

pub fn example_http_openapi() -> Value {
    let descriptors = example_inbound_descriptors()
        .into_iter()
        .filter_map(|descriptor| match descriptor {
            InboundApiDescriptor::Http(http) => Some(http),
            InboundApiDescriptor::Cli(_) => None,
        })
        .collect::<Vec<_>>();

    build_http_openapi(
        "Example Inbound Adapter HTTP API",
        SCHEMA_VERSION,
        HTTP_DOC_SUMMARY,
        &descriptors,
        http_schema_components(),
    )
}

pub fn example_cli_schema() -> Value {
    let descriptors = example_inbound_descriptors()
        .into_iter()
        .filter_map(|descriptor| match descriptor {
            InboundApiDescriptor::Cli(cli) => Some(cli),
            InboundApiDescriptor::Http(_) => None,
        })
        .collect::<Vec<_>>();

    build_cli_schema(
        SCHEMA_VERSION,
        SERVICE_NAME,
        CLI_DOC_SUMMARY,
        &descriptors,
        cli_schema_components(),
    )
}

pub fn example_api_manifest() -> Value {
    let descriptors = example_inbound_descriptors();
    build_api_manifest(
        SERVICE_NAME,
        SERVICE_SUMMARY,
        SCHEMA_VERSION,
        OPENAPI_REL_PATH,
        CLI_SCHEMA_REL_PATH,
        &descriptors,
    )
}

pub fn write_generated_api_docs(
    root_dir: impl AsRef<Path>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let root_dir = root_dir.as_ref();
    let docs = [
        (OPENAPI_REL_PATH, example_http_openapi()),
        (CLI_SCHEMA_REL_PATH, example_cli_schema()),
        (API_MANIFEST_REL_PATH, example_api_manifest()),
    ];

    let mut written_paths = Vec::with_capacity(docs.len());
    for (relative_path, document) in docs {
        let path = write_json_artifact(root_dir, relative_path, &document)?;
        written_paths.push(path);
    }

    Ok(written_paths)
}

fn write_json_artifact(
    root_dir: &Path,
    relative_path: &str,
    document: &serde_json::Value,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output_path = root_dir.join(relative_path);
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&output_path, serde_json::to_vec_pretty(document)?)?;
    Ok(output_path)
}

fn http_schema_components() -> Map<String, Value> {
    let mut schemas = Map::new();
    insert_schema(
        &mut schemas,
        "PlaceOrderHttpRequest",
        object_schema(
            &["trader_id", "symbol", "qty", "price"],
            vec![
                ("trace_id", optional_string_schema()),
                ("command_id", optional_string_schema()),
                ("trader_id", string_schema()),
                ("symbol", string_schema()),
                ("qty", u64_schema()),
                ("price", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "PlaceOrderHttpResponse",
        object_schema(
            &["order_id", "principal_reservation_amount", "remaining_quote", "domain_event_count"],
            vec![
                ("order_id", string_schema()),
                ("principal_reservation_amount", u64_schema()),
                ("remaining_quote", u64_schema()),
                ("domain_event_count", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "DepositQuoteHttpRequest",
        object_schema(
            &["trader_id", "amount"],
            vec![
                ("trace_id", optional_string_schema()),
                ("command_id", optional_string_schema()),
                ("trader_id", string_schema()),
                ("amount", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "DepositQuoteHttpResponse",
        object_schema(
            &["account_id", "available_quote", "frozen_quote", "domain_event_count"],
            vec![
                ("account_id", string_schema()),
                ("available_quote", u64_schema()),
                ("frozen_quote", u64_schema()),
                ("domain_event_count", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "WithdrawQuoteHttpRequest",
        object_schema(
            &["trader_id", "amount"],
            vec![
                ("trace_id", optional_string_schema()),
                ("command_id", optional_string_schema()),
                ("trader_id", string_schema()),
                ("amount", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "WithdrawQuoteHttpResponse",
        object_schema(
            &["account_id", "available_quote", "frozen_quote", "domain_event_count"],
            vec![
                ("account_id", string_schema()),
                ("available_quote", u64_schema()),
                ("frozen_quote", u64_schema()),
                ("domain_event_count", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "ExampleHttpErrorResponse",
        object_schema(
            &["error"],
            vec![(
                "error",
                object_schema(
                    &["code", "message"],
                    vec![("code", string_schema()), ("message", string_schema())],
                ),
            )],
        ),
    );
    schemas
}

fn cli_schema_components() -> Map<String, Value> {
    let mut schemas = Map::new();
    insert_schema(
        &mut schemas,
        "PlaceOrderCliCommand",
        object_schema(
            &["trader_id", "symbol", "qty", "price"],
            vec![
                ("trader_id", string_schema()),
                ("symbol", string_schema()),
                ("qty", u64_schema()),
                ("price", u64_schema()),
            ],
        ),
    );
    insert_schema(
        &mut schemas,
        "PlaceOrderCliResponse",
        object_schema(
            &["summary", "order_id"],
            vec![("summary", string_schema()), ("order_id", string_schema())],
        ),
    );
    insert_schema(
        &mut schemas,
        "DepositQuoteCliCommand",
        object_schema(
            &["trader_id", "amount"],
            vec![("trader_id", string_schema()), ("amount", u64_schema())],
        ),
    );
    insert_schema(
        &mut schemas,
        "DepositQuoteCliResponse",
        object_schema(
            &["summary", "account_id"],
            vec![("summary", string_schema()), ("account_id", string_schema())],
        ),
    );
    insert_schema(
        &mut schemas,
        "WithdrawQuoteCliCommand",
        object_schema(
            &["trader_id", "amount"],
            vec![("trader_id", string_schema()), ("amount", u64_schema())],
        ),
    );
    insert_schema(
        &mut schemas,
        "WithdrawQuoteCliResponse",
        object_schema(
            &["summary", "account_id"],
            vec![("summary", string_schema()), ("account_id", string_schema())],
        ),
    );
    schemas
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use serde::Serialize;

    use super::*;
    use crate::common::ExampleHttpErrorResponse;
    use crate::funding::{
        DepositQuoteCliCommand, DepositQuoteCliResponse, DepositQuoteHttpRequest,
        DepositQuoteHttpResponse, WithdrawQuoteCliCommand, WithdrawQuoteCliResponse,
        WithdrawQuoteHttpRequest, WithdrawQuoteHttpResponse, parse_deposit_quote_cli_args,
        parse_withdraw_quote_cli_args,
    };
    use crate::trading::{
        PlaceOrderCliCommand, PlaceOrderCliResponse, PlaceOrderHttpRequest, PlaceOrderHttpResponse,
        parse_place_order_cli_args,
    };

    #[test]
    fn descriptor_registry_covers_three_http_and_three_cli_contracts() {
        let descriptors = example_inbound_descriptors();
        let http_count = descriptors
            .iter()
            .filter(|descriptor| matches!(descriptor, InboundApiDescriptor::Http(_)))
            .count();
        let cli_count = descriptors
            .iter()
            .filter(|descriptor| matches!(descriptor, InboundApiDescriptor::Cli(_)))
            .count();

        assert_eq!(descriptors.len(), 6);
        assert_eq!(http_count, 3);
        assert_eq!(cli_count, 3);
    }

    #[test]
    fn http_descriptor_schema_names_are_derived_from_handler_signatures() {
        let order = create_order_http_api_descriptor();
        let deposit = create_deposit_http_api_descriptor();
        let withdraw = create_withdraw_http_api_descriptor();

        assert_eq!(
            order.request_schema_name,
            simple_type_name::<crate::trading::PlaceOrderHttpRequest>()
        );
        assert_eq!(
            order.success_response_schema_name,
            simple_type_name::<crate::trading::PlaceOrderHttpResponse>()
        );
        assert_eq!(
            deposit.request_schema_name,
            simple_type_name::<crate::funding::DepositQuoteHttpRequest>()
        );
        assert_eq!(
            deposit.success_response_schema_name,
            simple_type_name::<crate::funding::DepositQuoteHttpResponse>()
        );
        assert_eq!(
            withdraw.request_schema_name,
            simple_type_name::<crate::funding::WithdrawQuoteHttpRequest>()
        );
        assert_eq!(
            withdraw.success_response_schema_name,
            simple_type_name::<crate::funding::WithdrawQuoteHttpResponse>()
        );
    }

    #[test]
    fn http_openapi_contains_all_business_paths_and_descriptor_refs() {
        let openapi = example_http_openapi();

        assert_eq!(openapi["openapi"], json!("3.1.0"));
        assert!(openapi["paths"]["/orders"].is_object());
        assert!(openapi["paths"]["/deposits/quote"].is_object());
        assert!(openapi["paths"]["/withdrawals/quote"].is_object());

        for descriptor in example_inbound_descriptors() {
            let InboundApiDescriptor::Http(http) = descriptor else {
                continue;
            };

            let operation = &openapi["paths"][http.path][http.method.to_ascii_lowercase()];
            assert_eq!(
                operation["requestBody"]["content"]["application/json"]["schema"]["$ref"],
                json!(format!("#/components/schemas/{}", http.request_schema_name))
            );
            assert_eq!(
                operation["responses"]["200"]["content"]["application/json"]["schema"]["$ref"],
                json!(format!("#/components/schemas/{}", http.success_response_schema_name))
            );
            assert_eq!(
                operation["responses"]["400"]["content"]["application/json"]["schema"]["$ref"],
                json!(format!("#/components/schemas/{}", http.error_response_schema_name))
            );
            assert_eq!(
                operation["responses"]["500"]["content"]["application/json"]["schema"]["$ref"],
                json!(format!("#/components/schemas/{}", http.error_response_schema_name))
            );
            assert_eq!(
                operation["x-error-codes"],
                serde_json::to_value(&http.error_codes).unwrap()
            );
        }
    }

    #[test]
    fn http_openapi_schema_properties_track_http_dtos() {
        let openapi = example_http_openapi();

        assert_eq!(
            serialized_object_keys(&PlaceOrderHttpRequest {
                trace_id: Some("trace-1".to_string()),
                command_id: Some("cmd-1".to_string()),
                trader_id: "trader-1".to_string(),
                symbol: "BTCUSDT".to_string(),
                qty: 2,
                price: 100,
            }),
            schema_property_names(&openapi, "PlaceOrderHttpRequest")
        );
        assert_eq!(
            serialized_object_keys(&PlaceOrderHttpResponse {
                order_id: "order-1".to_string(),
                principal_reservation_amount: 200,
                remaining_quote: 800,
                domain_event_count: 2,
            }),
            schema_property_names(&openapi, "PlaceOrderHttpResponse")
        );
        assert_eq!(
            serialized_object_keys(&DepositQuoteHttpRequest {
                trace_id: Some("trace-2".to_string()),
                command_id: Some("cmd-2".to_string()),
                trader_id: "trader-1".to_string(),
                amount: 200,
            }),
            schema_property_names(&openapi, "DepositQuoteHttpRequest")
        );
        assert_eq!(
            serialized_object_keys(&DepositQuoteHttpResponse {
                account_id: "trader-1".to_string(),
                available_quote: 1_200,
                frozen_quote: 0,
                domain_event_count: 1,
            }),
            schema_property_names(&openapi, "DepositQuoteHttpResponse")
        );
        assert_eq!(
            serialized_object_keys(&WithdrawQuoteHttpRequest {
                trace_id: Some("trace-3".to_string()),
                command_id: Some("cmd-3".to_string()),
                trader_id: "trader-1".to_string(),
                amount: 150,
            }),
            schema_property_names(&openapi, "WithdrawQuoteHttpRequest")
        );
        assert_eq!(
            serialized_object_keys(&WithdrawQuoteHttpResponse {
                account_id: "trader-1".to_string(),
                available_quote: 850,
                frozen_quote: 0,
                domain_event_count: 1,
            }),
            schema_property_names(&openapi, "WithdrawQuoteHttpResponse")
        );

        let error_schema = &openapi["components"]["schemas"]["ExampleHttpErrorResponse"];
        assert_eq!(
            nested_property_names(error_schema, &["properties", "error"]),
            BTreeSet::from(["code".to_string(), "message".to_string()])
        );
        assert_eq!(
            serialized_object_keys(&ExampleHttpErrorResponse::new("invalid_qty", "invalid qty")),
            schema_property_names(&openapi, "ExampleHttpErrorResponse")
        );
    }

    #[test]
    fn cli_schema_matches_bins_argument_positions_defaults_and_usage() {
        let cli_schema = example_cli_schema();

        for descriptor in example_inbound_descriptors() {
            let InboundApiDescriptor::Cli(cli) = descriptor else {
                continue;
            };

            let command = cli_schema["commands"]
                .as_array()
                .unwrap()
                .iter()
                .find(|item| item["name"] == cli.name)
                .unwrap();

            assert_eq!(command["bin"], json!(cli.bin));
            assert_eq!(command["usage"], json!(cli.usage));
            assert_eq!(command["defaults"], cli.defaults);
            assert_eq!(command["command_schema_name"], json!(cli.command_schema_name));
            assert_eq!(command["stdout_schema_name"], json!(cli.stdout_schema_name));

            let args = command["args"].as_array().unwrap();
            assert_eq!(args.len(), cli.args.len());
            for (index, arg) in args.iter().enumerate() {
                assert_eq!(arg["position"], json!(index));
                assert_eq!(arg["name"], json!(cli.args[index].name));
                assert_eq!(arg["default"], cli.args[index].default.clone().unwrap_or(Value::Null));
            }
        }

        assert_eq!(
            cli_schema_default_values("place_order_cli", &cli_schema),
            serde_json::to_value(parse_place_order_cli_args(std::iter::empty::<String>()).unwrap())
                .unwrap()
        );
        assert_eq!(
            cli_schema_default_values("deposit_quote_cli", &cli_schema),
            serde_json::to_value(
                parse_deposit_quote_cli_args(std::iter::empty::<String>()).unwrap()
            )
            .unwrap()
        );
        assert_eq!(
            cli_schema_default_values("withdraw_quote_cli", &cli_schema),
            serde_json::to_value(
                parse_withdraw_quote_cli_args(std::iter::empty::<String>()).unwrap()
            )
            .unwrap()
        );
    }

    #[test]
    fn cli_schema_component_properties_track_cli_dtos() {
        let cli_schema = example_cli_schema();

        assert_eq!(
            serialized_object_keys(&PlaceOrderCliCommand {
                trader_id: "trader-1".to_string(),
                symbol: "BTCUSDT".to_string(),
                qty: 2,
                price: 100,
            }),
            schema_property_names_in_components(&cli_schema, "PlaceOrderCliCommand")
        );
        assert_eq!(
            serialized_object_keys(&PlaceOrderCliResponse {
                summary: "accepted".to_string(),
                order_id: "order-1".to_string(),
            }),
            schema_property_names_in_components(&cli_schema, "PlaceOrderCliResponse")
        );
        assert_eq!(
            serialized_object_keys(&DepositQuoteCliCommand {
                trader_id: "trader-1".to_string(),
                amount: 200,
            }),
            schema_property_names_in_components(&cli_schema, "DepositQuoteCliCommand")
        );
        assert_eq!(
            serialized_object_keys(&DepositQuoteCliResponse {
                summary: "quoted".to_string(),
                account_id: "trader-1".to_string(),
            }),
            schema_property_names_in_components(&cli_schema, "DepositQuoteCliResponse")
        );
        assert_eq!(
            serialized_object_keys(&WithdrawQuoteCliCommand {
                trader_id: "trader-1".to_string(),
                amount: 200,
            }),
            schema_property_names_in_components(&cli_schema, "WithdrawQuoteCliCommand")
        );
        assert_eq!(
            serialized_object_keys(&WithdrawQuoteCliResponse {
                summary: "accepted".to_string(),
                account_id: "trader-1".to_string(),
            }),
            schema_property_names_in_components(&cli_schema, "WithdrawQuoteCliResponse")
        );
    }

    #[test]
    fn manifest_references_generated_specs_and_matches_registry() {
        let manifest = example_api_manifest();
        let descriptors = example_inbound_descriptors();
        let covered = manifest["covered_interfaces"].as_array().unwrap();

        assert_eq!(manifest["http_spec_path"], json!(OPENAPI_REL_PATH));
        assert_eq!(manifest["cli_spec_path"], json!(CLI_SCHEMA_REL_PATH));
        assert_eq!(covered.len(), descriptors.len());

        let covered_names = covered
            .iter()
            .map(|entry| entry["name"].as_str().unwrap().to_string())
            .collect::<BTreeSet<_>>();
        let descriptor_names = descriptors
            .iter()
            .map(|descriptor| match descriptor {
                InboundApiDescriptor::Http(http) => http.name.to_string(),
                InboundApiDescriptor::Cli(cli) => cli.name.to_string(),
            })
            .collect::<BTreeSet<_>>();

        assert_eq!(covered_names, descriptor_names);
    }

    fn schema_property_names(document: &Value, schema_name: &str) -> BTreeSet<String> {
        schema_property_names_in(&document["components"]["schemas"][schema_name]["properties"])
    }

    fn schema_property_names_in_components(
        document: &Value,
        schema_name: &str,
    ) -> BTreeSet<String> {
        schema_property_names(document, schema_name)
    }

    fn schema_property_names_in(properties: &Value) -> BTreeSet<String> {
        properties.as_object().unwrap().keys().cloned().collect::<BTreeSet<_>>()
    }

    fn nested_property_names(schema: &Value, path: &[&str]) -> BTreeSet<String> {
        let mut current = schema;
        for segment in path {
            current = &current[*segment];
        }
        schema_property_names_in(&current["properties"])
    }

    fn serialized_object_keys<T: Serialize>(value: &T) -> BTreeSet<String> {
        serde_json::to_value(value)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
    }

    fn cli_schema_default_values(command_name: &str, cli_schema: &Value) -> Value {
        cli_schema["commands"]
            .as_array()
            .unwrap()
            .iter()
            .find(|item| item["name"] == command_name)
            .unwrap()["defaults"]
            .clone()
    }

    fn simple_type_name<T>() -> &'static str {
        std::any::type_name::<T>().rsplit("::").next().unwrap()
    }
}
use std::fs;
use std::path::{Path, PathBuf};
