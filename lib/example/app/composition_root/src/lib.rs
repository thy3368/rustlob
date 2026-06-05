use std::fs;
use std::path::{Path, PathBuf};

use cmd_handler::use_case_def2::{CommandUseCaseExecutionError, CommandUseCaseOutbound};
use example_core::{
    DepositQuoteCmd, DepositQuoteError, DepositQuoteState, MarketRules, PlaceOrderError,
    PlaceOrderState, TradingAccount, WithdrawQuoteCmd, WithdrawQuoteError, WithdrawQuoteState,
};
use example_inbound_adapter::{
    API_MANIFEST_REL_PATH, CLI_SCHEMA_REL_PATH, DepositQuoteCliCommand, DepositQuoteCliResponse,
    DepositQuoteHttpRequest, DepositQuoteHttpResponse, DepositQuoteOutboundAccess,
    OPENAPI_REL_PATH, PlaceOrderCliCommand, PlaceOrderCliResponse, PlaceOrderHttpRequest,
    PlaceOrderHttpResponse, PlaceOrderOutboundAccess, WithdrawQuoteCliCommand,
    WithdrawQuoteCliResponse, WithdrawQuoteHttpRequest, WithdrawQuoteHttpResponse,
    WithdrawQuoteOutboundAccess, example_api_manifest, example_cli_schema, example_http_openapi,
    handle_deposit_quote_http, handle_place_order_http, handle_withdraw_quote_http,
    run_deposit_quote_cli, run_place_order_cli, run_withdraw_quote_cli,
};
use example_outbound_adapter::{
    DepositQuoteOutboundError, InMemoryDepositQuoteOutbound, InMemoryPlaceOrderOutbound,
    InMemoryStore, InMemoryWithdrawQuoteOutbound, MySqlDepositQuoteOutbound,
    MySqlPlaceOrderOutbound, MySqlStore, MySqlWithdrawQuoteOutbound, PlaceOrderOutboundError,
    StoreError, StoreSnapshot, WithdrawQuoteOutboundError,
};

#[derive(Debug)]
pub struct ExampleApplication<S, PO, DQ, WQ> {
    store: S,
    place_order_outbound: PO,
    deposit_quote_outbound: DQ,
    withdraw_quote_outbound: WQ,
}

pub type InMemoryExampleApplication = ExampleApplication<
    InMemoryStore,
    InMemoryPlaceOrderOutbound,
    InMemoryDepositQuoteOutbound,
    InMemoryWithdrawQuoteOutbound,
>;
pub type MySqlExampleApplication = ExampleApplication<
    MySqlStore,
    MySqlPlaceOrderOutbound,
    MySqlDepositQuoteOutbound,
    MySqlWithdrawQuoteOutbound,
>;

fn demo_account() -> TradingAccount {
    TradingAccount {
        account_id: "trader-1".to_string(),
        available_quote: 1_000,
        frozen_quote: 0,
        version: 2,
    }
}

fn demo_market_rules() -> MarketRules {
    MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 }
}

impl<S, PO, DQ, WQ> ExampleApplication<S, PO, DQ, WQ>
where
    PO: CommandUseCaseOutbound<
            Command = example_core::PlaceOrderCmd,
            State = PlaceOrderState,
            Error = PlaceOrderOutboundError,
        >,
    DQ: CommandUseCaseOutbound<
            Command = DepositQuoteCmd,
            State = DepositQuoteState,
            Error = DepositQuoteOutboundError,
        >,
    WQ: CommandUseCaseOutbound<
            Command = WithdrawQuoteCmd,
            State = WithdrawQuoteState,
            Error = WithdrawQuoteOutboundError,
        >,
{
    pub fn new(
        store: S,
        place_order_outbound: PO,
        deposit_quote_outbound: DQ,
        withdraw_quote_outbound: WQ,
    ) -> Self {
        Self { store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound }
    }

    pub fn handle_http(
        &self,
        request: PlaceOrderHttpRequest,
    ) -> Result<
        PlaceOrderHttpResponse,
        CommandUseCaseExecutionError<PlaceOrderError, PlaceOrderOutboundError>,
    > {
        handle_place_order_http(request, &self.place_order_outbound)
    }

    pub fn handle_cli(
        &self,
        command: PlaceOrderCliCommand,
    ) -> Result<
        PlaceOrderCliResponse,
        CommandUseCaseExecutionError<PlaceOrderError, PlaceOrderOutboundError>,
    > {
        run_place_order_cli(command, &self.place_order_outbound)
    }

    pub fn handle_deposit_http(
        &self,
        request: DepositQuoteHttpRequest,
    ) -> Result<
        DepositQuoteHttpResponse,
        CommandUseCaseExecutionError<DepositQuoteError, DepositQuoteOutboundError>,
    > {
        handle_deposit_quote_http(request, &self.deposit_quote_outbound)
    }

    pub fn handle_deposit_cli(
        &self,
        command: DepositQuoteCliCommand,
    ) -> Result<
        DepositQuoteCliResponse,
        CommandUseCaseExecutionError<DepositQuoteError, DepositQuoteOutboundError>,
    > {
        run_deposit_quote_cli(command, &self.deposit_quote_outbound)
    }

    pub fn handle_withdraw_http(
        &self,
        request: WithdrawQuoteHttpRequest,
    ) -> Result<
        WithdrawQuoteHttpResponse,
        CommandUseCaseExecutionError<WithdrawQuoteError, WithdrawQuoteOutboundError>,
    > {
        handle_withdraw_quote_http(request, &self.withdraw_quote_outbound)
    }

    pub fn handle_withdraw_cli(
        &self,
        command: WithdrawQuoteCliCommand,
    ) -> Result<
        WithdrawQuoteCliResponse,
        CommandUseCaseExecutionError<WithdrawQuoteError, WithdrawQuoteOutboundError>,
    > {
        run_withdraw_quote_cli(command, &self.withdraw_quote_outbound)
    }
}

impl<S, PO, DQ, WQ> PlaceOrderOutboundAccess for ExampleApplication<S, PO, DQ, WQ>
where
    PO: CommandUseCaseOutbound<
            Command = example_core::PlaceOrderCmd,
            State = PlaceOrderState,
            Error = PlaceOrderOutboundError,
        >,
{
    type OutboundError = PlaceOrderOutboundError;
    type Outbound = PO;

    fn place_order_outbound(&self) -> &Self::Outbound {
        &self.place_order_outbound
    }
}

impl<S, PO, DQ, WQ> DepositQuoteOutboundAccess for ExampleApplication<S, PO, DQ, WQ>
where
    DQ: CommandUseCaseOutbound<
            Command = DepositQuoteCmd,
            State = DepositQuoteState,
            Error = DepositQuoteOutboundError,
        >,
{
    type OutboundError = DepositQuoteOutboundError;
    type Outbound = DQ;

    fn deposit_quote_outbound(&self) -> &Self::Outbound {
        &self.deposit_quote_outbound
    }
}

impl<S, PO, DQ, WQ> WithdrawQuoteOutboundAccess for ExampleApplication<S, PO, DQ, WQ>
where
    WQ: CommandUseCaseOutbound<
            Command = WithdrawQuoteCmd,
            State = WithdrawQuoteState,
            Error = WithdrawQuoteOutboundError,
        >,
{
    type OutboundError = WithdrawQuoteOutboundError;
    type Outbound = WQ;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound {
        &self.withdraw_quote_outbound
    }
}

impl InMemoryExampleApplication {
    pub fn new_in_memory() -> Result<Self, StoreError> {
        let store = InMemoryStore::seeded(demo_account(), demo_market_rules())?;
        let place_order_outbound = InMemoryPlaceOrderOutbound::from_store(store.clone());
        let deposit_quote_outbound = InMemoryDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = InMemoryWithdrawQuoteOutbound::from_store(store.clone());

        Ok(Self::new(store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound))
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        self.store.snapshot()
    }
}

impl MySqlExampleApplication {
    pub fn new_mysql(database_url: &str) -> Result<Self, StoreError> {
        let store = MySqlStore::new(database_url)?;
        let place_order_outbound = MySqlPlaceOrderOutbound::from_store(store.clone());
        let deposit_quote_outbound = MySqlDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = MySqlWithdrawQuoteOutbound::from_store(store.clone());

        Ok(Self::new(store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound))
    }

    pub fn new_mysql_seeded(database_url: &str) -> Result<Self, StoreError> {
        let store = MySqlStore::new(database_url)?;
        store.seed_account(demo_account())?;
        store.seed_market_rules(demo_market_rules())?;

        let place_order_outbound = MySqlPlaceOrderOutbound::from_store(store.clone());
        let deposit_quote_outbound = MySqlDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = MySqlWithdrawQuoteOutbound::from_store(store.clone());

        Ok(Self::new(store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound))
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        self.store.snapshot()
    }
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
    let path = root_dir.join(relative_path);
    let parent = path.parent().ok_or_else(|| std::io::Error::other("missing parent directory"))?;
    fs::create_dir_all(parent)?;
    let body = serde_json::to_string_pretty(document)?;
    fs::write(&path, format!("{body}\n"))?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use serde_json::Value;

    use super::*;

    #[test]
    fn composition_root_wires_http_flow() -> Result<(), Box<dyn std::error::Error>> {
        let app = ExampleApplication::new_in_memory()?;

        let response = app.handle_http(PlaceOrderHttpRequest {
            trace_id: Some("trace-http".to_string()),
            command_id: Some("cmd-http".to_string()),
            trader_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 3,
            price: 100,
        })?;
        let snapshot = app.snapshot()?;

        assert_eq!(response.order_id, "trader-1-BTCUSDT-1");
        assert_eq!(response.remaining_quote, 700);
        assert_eq!(snapshot.persisted_event_count, 2);
        assert!(snapshot.orders.contains_key("trader-1-BTCUSDT-1"));

        Ok(())
    }

    #[test]
    fn composition_root_wires_cli_flow() -> Result<(), Box<dyn std::error::Error>> {
        let app = ExampleApplication::new_in_memory()?;

        let response = app.handle_cli(PlaceOrderCliCommand {
            trader_id: "trader-1".to_string(),
            symbol: "BTCUSDT".to_string(),
            qty: 2,
            price: 100,
        })?;
        let snapshot = app.snapshot()?;

        assert_eq!(response.order_id, "trader-1-BTCUSDT-1");
        assert_eq!(
            response.summary,
            "accepted order_id=trader-1-BTCUSDT-1 reserved_quote=200 remaining_quote=800"
        );
        assert_eq!(snapshot.published_event_count, 2);

        Ok(())
    }

    #[test]
    fn composition_root_wires_deposit_http_flow() -> Result<(), Box<dyn std::error::Error>> {
        let app = ExampleApplication::new_in_memory()?;

        let response = app.handle_deposit_http(DepositQuoteHttpRequest {
            trace_id: Some("trace-deposit-http".to_string()),
            command_id: Some("cmd-deposit-http".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        })?;
        let snapshot = app.snapshot()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 1_250);
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 1_250);

        Ok(())
    }

    #[test]
    fn composition_root_wires_deposit_cli_flow() -> Result<(), Box<dyn std::error::Error>> {
        let app = ExampleApplication::new_in_memory()?;

        let response = app.handle_deposit_cli(DepositQuoteCliCommand {
            trader_id: "trader-1".to_string(),
            amount: 300,
        })?;
        let snapshot = app.snapshot()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(
            response.summary,
            "quoted deposit account_id=trader-1 available_quote=1300 frozen_quote=0"
        );
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 1_300);

        Ok(())
    }

    #[test]
    fn composition_root_wires_withdraw_http_flow() -> Result<(), Box<dyn std::error::Error>> {
        let app = ExampleApplication::new_in_memory()?;

        let response = app.handle_withdraw_http(WithdrawQuoteHttpRequest {
            trace_id: Some("trace-withdraw-http".to_string()),
            command_id: Some("cmd-withdraw-http".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        })?;
        let snapshot = app.snapshot()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 750);
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 750);

        Ok(())
    }

    #[test]
    fn composition_root_wires_withdraw_cli_flow() -> Result<(), Box<dyn std::error::Error>> {
        let app = ExampleApplication::new_in_memory()?;

        let response = app.handle_withdraw_cli(WithdrawQuoteCliCommand {
            trader_id: "trader-1".to_string(),
            amount: 300,
        })?;
        let snapshot = app.snapshot()?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(
            response.summary,
            "accepted account_id=trader-1 available_quote=700 frozen_quote=0"
        );
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 700);

        Ok(())
    }

    #[test]
    fn composition_root_writes_generated_api_docs_to_expected_paths()
    -> Result<(), Box<dyn std::error::Error>> {
        let unique = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let root_dir = std::env::temp_dir().join(format!("rustlob-example-docs-{unique}"));
        let written = write_generated_api_docs(&root_dir)?;

        assert_eq!(written.len(), 3);
        assert!(root_dir.join(OPENAPI_REL_PATH).exists());
        assert!(root_dir.join(CLI_SCHEMA_REL_PATH).exists());
        assert!(root_dir.join(API_MANIFEST_REL_PATH).exists());

        let openapi: Value =
            serde_json::from_str(&fs::read_to_string(root_dir.join(OPENAPI_REL_PATH))?)?;
        let cli_schema: Value =
            serde_json::from_str(&fs::read_to_string(root_dir.join(CLI_SCHEMA_REL_PATH))?)?;
        let manifest: Value =
            serde_json::from_str(&fs::read_to_string(root_dir.join(API_MANIFEST_REL_PATH))?)?;

        assert_eq!(openapi["openapi"], "3.1.0");
        assert_eq!(cli_schema["service"], "example_inbound_adapter");
        assert_eq!(manifest["http_spec_path"], OPENAPI_REL_PATH);
        assert_eq!(manifest["cli_spec_path"], CLI_SCHEMA_REL_PATH);

        fs::remove_dir_all(&root_dir)?;
        Ok(())
    }
}
