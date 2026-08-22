use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::{Map, Number, Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HttpErrorCodeDescriptor {
    pub status_code: u16,
    pub code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HttpApiDescriptor {
    pub name: &'static str,
    pub method: &'static str,
    pub path: &'static str,
    pub summary: &'static str,
    pub request_schema_name: &'static str,
    pub success_response_schema_name: &'static str,
    pub error_response_schema_name: &'static str,
    pub error_codes: Vec<HttpErrorCodeDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CliArgDescriptor {
    pub name: &'static str,
    pub position: usize,
    pub value_type: &'static str,
    pub required: bool,
    pub default: Option<Value>,
    pub summary: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CliErrorCodeDescriptor {
    pub category: &'static str,
    pub code: &'static str,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CliApiDescriptor {
    pub name: &'static str,
    pub bin: &'static str,
    pub usage: &'static str,
    pub summary: &'static str,
    pub args: Vec<CliArgDescriptor>,
    pub command_schema_name: &'static str,
    pub stdout_schema_name: &'static str,
    pub error_codes: Vec<CliErrorCodeDescriptor>,
    pub defaults: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InboundApiDescriptor {
    Http(HttpApiDescriptor),
    Cli(CliApiDescriptor),
}

pub fn build_http_openapi(
    title: &str,
    version: &str,
    summary: &str,
    descriptors: &[HttpApiDescriptor],
    schema_components: Map<String, Value>,
) -> Value {
    let paths = descriptors.iter().fold(Map::new(), |mut paths, descriptor| {
        let method_name = descriptor.method.to_ascii_lowercase();
        let mut method = Map::new();
        method.insert(method_name, http_operation(descriptor));
        paths.insert(descriptor.path.to_string(), Value::Object(method));
        paths
    });

    object([
        ("openapi", string_value("3.1.0")),
        (
            "info",
            object([
                ("title", string_value(title)),
                ("version", string_value(version)),
                ("summary", string_value(summary)),
            ]),
        ),
        ("paths", Value::Object(paths)),
        ("components", object([("schemas", Value::Object(schema_components))])),
    ])
}

pub fn build_cli_schema(
    schema_version: &str,
    service: &str,
    summary: &str,
    descriptors: &[CliApiDescriptor],
    schema_components: Map<String, Value>,
) -> Value {
    let commands = descriptors.iter().map(cli_command_schema_entry).collect::<Vec<_>>();

    object([
        ("schema_version", string_value(schema_version)),
        ("service", string_value(service)),
        ("summary", string_value(summary)),
        ("commands", Value::Array(commands)),
        ("components", object([("schemas", Value::Object(schema_components))])),
    ])
}

pub fn build_api_manifest(
    service: &str,
    summary: &str,
    schema_version: &str,
    http_spec_path: &str,
    cli_spec_path: &str,
    covered_interfaces: &[InboundApiDescriptor],
) -> Value {
    let covered_interfaces = covered_interfaces
        .iter()
        .map(|descriptor| match descriptor {
            InboundApiDescriptor::Http(http) => object([
                ("kind", string_value("http")),
                ("name", string_value(http.name)),
                ("summary", string_value(http.summary)),
                ("method", string_value(http.method)),
                ("path", string_value(http.path)),
            ]),
            InboundApiDescriptor::Cli(cli) => object([
                ("kind", string_value("cli")),
                ("name", string_value(cli.name)),
                ("summary", string_value(cli.summary)),
                ("bin", string_value(cli.bin)),
            ]),
        })
        .collect::<Vec<_>>();

    object([
        ("service", string_value(service)),
        ("summary", string_value(summary)),
        ("schema_version", string_value(schema_version)),
        ("http_spec_path", string_value(http_spec_path)),
        ("cli_spec_path", string_value(cli_spec_path)),
        ("covered_interfaces", Value::Array(covered_interfaces)),
    ])
}

pub fn http_error(status_code: u16, code: &'static str) -> HttpErrorCodeDescriptor {
    HttpErrorCodeDescriptor { status_code, code }
}

pub fn cli_arg(
    name: &'static str,
    position: usize,
    value_type: &'static str,
    required: bool,
    default: Option<Value>,
    summary: &'static str,
) -> CliArgDescriptor {
    CliArgDescriptor { name, position, value_type, required, default, summary }
}

pub fn cli_error(category: &'static str, code: &'static str) -> CliErrorCodeDescriptor {
    CliErrorCodeDescriptor { category, code }
}

pub fn schema_ref(schema_name: &str) -> Value {
    object([("$ref", string_value(format!("#/components/schemas/{schema_name}")))])
}

pub fn object_schema(required: &[&str], properties: Vec<(&str, Value)>) -> Value {
    let mut property_map = Map::new();
    for (name, schema) in properties {
        property_map.insert(name.to_string(), schema);
    }

    object([
        ("type", string_value("object")),
        ("additionalProperties", Value::Bool(false)),
        ("required", string_array(required.iter().copied())),
        ("properties", Value::Object(property_map)),
    ])
}

pub fn string_schema() -> Value {
    object([("type", string_value("string"))])
}

pub fn optional_string_schema() -> Value {
    object([("type", string_array(["string", "null"]))])
}

pub fn u64_schema() -> Value {
    object([("type", string_value("integer")), ("minimum", u64_value(0))])
}

pub fn insert_schema(target: &mut Map<String, Value>, name: &str, schema: Value) {
    target.insert(name.to_string(), schema);
}

fn http_operation(descriptor: &HttpApiDescriptor) -> Value {
    let mut responses = Map::new();
    responses.insert(
        "200".to_string(),
        response_schema("Successful use case response.", descriptor.success_response_schema_name),
    );

    for status_code in
        descriptor.error_codes.iter().map(|error| error.status_code).collect::<BTreeSet<_>>()
    {
        responses.insert(
            status_code.to_string(),
            response_schema(
                format!("Error responses for {}.", descriptor.name),
                descriptor.error_response_schema_name,
            ),
        );
    }

    object([
        ("operationId", string_value(descriptor.name)),
        ("summary", string_value(descriptor.summary)),
        (
            "requestBody",
            object([
                ("required", Value::Bool(true)),
                ("content", json_content_schema(descriptor.request_schema_name)),
            ]),
        ),
        ("responses", Value::Object(responses)),
        ("x-error-codes", http_error_codes(&descriptor.error_codes)),
    ])
}

fn cli_command_schema_entry(descriptor: &CliApiDescriptor) -> Value {
    object([
        ("name", string_value(descriptor.name)),
        ("bin", string_value(descriptor.bin)),
        ("usage", string_value(descriptor.usage)),
        ("summary", string_value(descriptor.summary)),
        ("args", cli_args(&descriptor.args)),
        ("command_schema_name", string_value(descriptor.command_schema_name)),
        ("command_schema", schema_ref(descriptor.command_schema_name)),
        ("stdout_schema_name", string_value(descriptor.stdout_schema_name)),
        ("stdout_schema", schema_ref(descriptor.stdout_schema_name)),
        ("error_codes", cli_error_codes(&descriptor.error_codes)),
        ("defaults", descriptor.defaults.clone()),
    ])
}

fn object<const N: usize>(entries: [(&str, Value); N]) -> Value {
    let mut map = Map::new();
    for (name, value) in entries {
        map.insert(name.to_string(), value);
    }
    Value::Object(map)
}

fn string_value(value: impl Into<String>) -> Value {
    Value::String(value.into())
}

fn u64_value(value: u64) -> Value {
    Value::Number(Number::from(value))
}

fn string_array<'a>(values: impl IntoIterator<Item = &'a str>) -> Value {
    Value::Array(values.into_iter().map(string_value).collect())
}

fn json_content_schema(schema_name: &str) -> Value {
    object([("application/json", object([("schema", schema_ref(schema_name))]))])
}

fn response_schema(description: impl Into<String>, schema_name: &str) -> Value {
    object([
        ("description", string_value(description)),
        ("content", json_content_schema(schema_name)),
    ])
}

fn http_error_codes(error_codes: &[HttpErrorCodeDescriptor]) -> Value {
    Value::Array(
        error_codes
            .iter()
            .map(|error| {
                object([
                    ("status_code", u64_value(u64::from(error.status_code))),
                    ("code", string_value(error.code)),
                ])
            })
            .collect(),
    )
}

fn cli_args(args: &[CliArgDescriptor]) -> Value {
    Value::Array(
        args.iter()
            .map(|arg| {
                let mut value = match object([
                    ("name", string_value(arg.name)),
                    ("position", u64_value(arg.position as u64)),
                    ("value_type", string_value(arg.value_type)),
                    ("required", Value::Bool(arg.required)),
                    ("summary", string_value(arg.summary)),
                ]) {
                    Value::Object(map) => map,
                    _ => Map::new(),
                };
                value.insert("default".to_string(), arg.default.clone().unwrap_or(Value::Null));
                Value::Object(value)
            })
            .collect(),
    )
}

fn cli_error_codes(error_codes: &[CliErrorCodeDescriptor]) -> Value {
    Value::Array(
        error_codes
            .iter()
            .map(|error| {
                object([
                    ("category", string_value(error.category)),
                    ("code", string_value(error.code)),
                ])
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn build_http_openapi_uses_descriptors_and_schema_components() {
        let openapi = build_http_openapi(
            "Test HTTP API",
            "1.0.0",
            "Test summary",
            &[HttpApiDescriptor {
                name: "create_order",
                method: "POST",
                path: "/orders",
                summary: "Create order.",
                request_schema_name: "OrderRequest",
                success_response_schema_name: "OrderResponse",
                error_response_schema_name: "ApiError",
                error_codes: vec![
                    http_error(400, "invalid_qty"),
                    http_error(500, "outbound_failed"),
                ],
            }],
            Map::from_iter([(
                "OrderRequest".to_string(),
                object_schema(&["qty"], vec![("qty", u64_schema())]),
            )]),
        );

        assert_eq!(openapi["openapi"], json!("3.1.0"));
        assert_eq!(openapi["info"]["title"], json!("Test HTTP API"));
        assert_eq!(
            openapi["paths"]["/orders"]["post"]["requestBody"]["content"]["application/json"]["schema"]
                ["$ref"],
            json!("#/components/schemas/OrderRequest")
        );
        assert_eq!(
            openapi["paths"]["/orders"]["post"]["responses"]["400"]["content"]["application/json"]
                ["schema"]["$ref"],
            json!("#/components/schemas/ApiError")
        );
    }

    #[test]
    fn build_cli_schema_and_manifest_cover_cli_and_http_entries() {
        let cli_schema = build_cli_schema(
            "1.0.0",
            "example_service",
            "CLI summary",
            &[CliApiDescriptor {
                name: "place_order_cli",
                bin: "cli_demo",
                usage: "usage: cli_demo",
                summary: "Run demo.",
                args: vec![cli_arg("qty", 0, "u64", true, None, "Qty.")],
                command_schema_name: "PlaceOrderCliCommand",
                stdout_schema_name: "PlaceOrderCliResponse",
                error_codes: vec![cli_error("parse", "invalid_qty")],
                defaults: json!({}),
            }],
            Map::new(),
        );

        let manifest = build_api_manifest(
            "example_service",
            "Manifest summary",
            "1.0.0",
            "openapi.json",
            "cli-schema.json",
            &[
                InboundApiDescriptor::Http(HttpApiDescriptor {
                    name: "create_order",
                    method: "POST",
                    path: "/orders",
                    summary: "Create order.",
                    request_schema_name: "OrderRequest",
                    success_response_schema_name: "OrderResponse",
                    error_response_schema_name: "ApiError",
                    error_codes: vec![http_error(400, "invalid_qty")],
                }),
                InboundApiDescriptor::Cli(CliApiDescriptor {
                    name: "place_order_cli",
                    bin: "cli_demo",
                    usage: "usage: cli_demo",
                    summary: "Run demo.",
                    args: vec![],
                    command_schema_name: "PlaceOrderCliCommand",
                    stdout_schema_name: "PlaceOrderCliResponse",
                    error_codes: vec![],
                    defaults: json!({}),
                }),
            ],
        );

        assert_eq!(cli_schema["service"], json!("example_service"));
        assert_eq!(
            cli_schema["commands"][0]["command_schema"]["$ref"],
            json!("#/components/schemas/PlaceOrderCliCommand")
        );
        assert_eq!(manifest["http_spec_path"], json!("openapi.json"));
        assert_eq!(manifest["covered_interfaces"][0]["kind"], json!("http"));
        assert_eq!(manifest["covered_interfaces"][1]["kind"], json!("cli"));
    }
}
