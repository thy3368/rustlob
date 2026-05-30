use cmd_handler::use_case_def2::CommandUseCaseOutbound;
use example_core::{
    DepositQuoteCmd, DepositQuoteError, DepositQuoteState, MarketRules, PlaceOrderError,
    TradingAccount, WithdrawQuoteCmd, WithdrawQuoteError, WithdrawQuoteState,
};
use example_inbound_adapter::{
    DepositQuoteCliCommand, DepositQuoteCliResponse, DepositQuoteHttpRequest,
    DepositQuoteHttpResponse, DepositQuoteOutboundAccess, PlaceOrderCliCommand,
    PlaceOrderCliResponse, PlaceOrderHttpRequest, PlaceOrderHttpResponse, PlaceOrderOutboundAccess,
    WithdrawQuoteCliCommand, WithdrawQuoteCliResponse, WithdrawQuoteHttpRequest,
    WithdrawQuoteHttpResponse, WithdrawQuoteOutboundAccess, handle_deposit_quote_http,
    handle_place_order_http, handle_withdraw_quote_http, run_deposit_quote_cli,
    run_place_order_cli, run_withdraw_quote_cli,
};
use example_outbound_adapter::{
    InMemoryDepositQuoteOutbound, InMemoryPlaceOrderOutbound, InMemoryStore,
    InMemoryWithdrawQuoteOutbound, MySqlDepositQuoteOutbound, MySqlPlaceOrderOutbound, MySqlStore,
    MySqlWithdrawQuoteOutbound, StoreSnapshot,
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
            example_core::PlaceOrderCmd,
            example_core::PlaceOrderState,
            example_core::PlaceOrderError,
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
    ) -> Result<PlaceOrderHttpResponse, PlaceOrderError> {
        handle_place_order_http(request, &self.place_order_outbound)
    }

    pub fn handle_cli(
        &self,
        command: PlaceOrderCliCommand,
    ) -> Result<PlaceOrderCliResponse, PlaceOrderError> {
        run_place_order_cli(command, &self.place_order_outbound)
    }

    pub fn handle_deposit_http(
        &self,
        request: DepositQuoteHttpRequest,
    ) -> Result<DepositQuoteHttpResponse, DepositQuoteError>
    where
        DQ: CommandUseCaseOutbound<DepositQuoteCmd, DepositQuoteState, DepositQuoteError>,
    {
        handle_deposit_quote_http(request, &self.deposit_quote_outbound)
    }

    pub fn handle_deposit_cli(
        &self,
        command: DepositQuoteCliCommand,
    ) -> Result<DepositQuoteCliResponse, DepositQuoteError>
    where
        DQ: CommandUseCaseOutbound<DepositQuoteCmd, DepositQuoteState, DepositQuoteError>,
    {
        run_deposit_quote_cli(command, &self.deposit_quote_outbound)
    }

    pub fn handle_withdraw_http(
        &self,
        request: WithdrawQuoteHttpRequest,
    ) -> Result<WithdrawQuoteHttpResponse, WithdrawQuoteError>
    where
        WQ: CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>,
    {
        handle_withdraw_quote_http(request, &self.withdraw_quote_outbound)
    }

    pub fn handle_withdraw_cli(
        &self,
        command: WithdrawQuoteCliCommand,
    ) -> Result<WithdrawQuoteCliResponse, WithdrawQuoteError>
    where
        WQ: CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>,
    {
        run_withdraw_quote_cli(command, &self.withdraw_quote_outbound)
    }
}

impl<S, PO, DQ, WQ> PlaceOrderOutboundAccess for ExampleApplication<S, PO, DQ, WQ>
where
    PO: CommandUseCaseOutbound<
            example_core::PlaceOrderCmd,
            example_core::PlaceOrderState,
            PlaceOrderError,
        >,
{
    type Outbound = PO;

    fn place_order_outbound(&self) -> &Self::Outbound {
        &self.place_order_outbound
    }
}

impl<S, PO, DQ, WQ> DepositQuoteOutboundAccess for ExampleApplication<S, PO, DQ, WQ>
where
    DQ: CommandUseCaseOutbound<DepositQuoteCmd, DepositQuoteState, DepositQuoteError>,
{
    type Outbound = DQ;

    fn deposit_quote_outbound(&self) -> &Self::Outbound {
        &self.deposit_quote_outbound
    }
}

impl<S, PO, DQ, WQ> WithdrawQuoteOutboundAccess for ExampleApplication<S, PO, DQ, WQ>
where
    WQ: CommandUseCaseOutbound<WithdrawQuoteCmd, WithdrawQuoteState, WithdrawQuoteError>,
{
    type Outbound = WQ;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound {
        &self.withdraw_quote_outbound
    }
}

impl InMemoryExampleApplication {
    pub fn new_in_memory() -> Result<Self, PlaceOrderError> {
        let store = InMemoryStore::seeded(demo_account(), demo_market_rules())?;
        let place_order_outbound = InMemoryPlaceOrderOutbound::from_store(store.clone());
        let deposit_quote_outbound = InMemoryDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = InMemoryWithdrawQuoteOutbound::from_store(store.clone());

        Ok(Self::new(store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound))
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, PlaceOrderError> {
        self.store.snapshot()
    }
}

impl MySqlExampleApplication {
    pub fn new_mysql(database_url: &str) -> Result<Self, PlaceOrderError> {
        let store = MySqlStore::new(database_url)?;
        let place_order_outbound = MySqlPlaceOrderOutbound::from_store(store.clone());
        let deposit_quote_outbound = MySqlDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = MySqlWithdrawQuoteOutbound::from_store(store.clone());

        Ok(Self::new(store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound))
    }

    pub fn new_mysql_seeded(database_url: &str) -> Result<Self, PlaceOrderError> {
        let store = MySqlStore::new(database_url)?;
        store.seed_account(demo_account())?;
        store.seed_market_rules(demo_market_rules())?;

        let place_order_outbound = MySqlPlaceOrderOutbound::from_store(store.clone());
        let deposit_quote_outbound = MySqlDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = MySqlWithdrawQuoteOutbound::from_store(store.clone());

        Ok(Self::new(store, place_order_outbound, deposit_quote_outbound, withdraw_quote_outbound))
    }

    pub fn snapshot(&self) -> Result<StoreSnapshot, PlaceOrderError> {
        self.store.snapshot()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composition_root_wires_http_flow() -> Result<(), PlaceOrderError> {
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
    fn composition_root_wires_cli_flow() -> Result<(), PlaceOrderError> {
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
    fn composition_root_wires_deposit_http_flow() -> Result<(), DepositQuoteError> {
        let app =
            ExampleApplication::new_in_memory().map_err(|_| DepositQuoteError::StoreUnavailable)?;

        let response = app.handle_deposit_http(DepositQuoteHttpRequest {
            trace_id: Some("trace-deposit-http".to_string()),
            command_id: Some("cmd-deposit-http".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        })?;
        let snapshot = app.snapshot().map_err(|_| DepositQuoteError::StoreUnavailable)?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 1_250);
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 1_250);

        Ok(())
    }

    #[test]
    fn composition_root_wires_deposit_cli_flow() -> Result<(), DepositQuoteError> {
        let app =
            ExampleApplication::new_in_memory().map_err(|_| DepositQuoteError::StoreUnavailable)?;

        let response = app.handle_deposit_cli(DepositQuoteCliCommand {
            trader_id: "trader-1".to_string(),
            amount: 300,
        })?;
        let snapshot = app.snapshot().map_err(|_| DepositQuoteError::StoreUnavailable)?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(
            response.summary,
            "accepted account_id=trader-1 available_quote=1300 frozen_quote=0"
        );
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 1_300);

        Ok(())
    }

    #[test]
    fn composition_root_wires_withdraw_http_flow() -> Result<(), WithdrawQuoteError> {
        let app = ExampleApplication::new_in_memory()
            .map_err(|_| WithdrawQuoteError::StoreUnavailable)?;

        let response = app.handle_withdraw_http(WithdrawQuoteHttpRequest {
            trace_id: Some("trace-withdraw-http".to_string()),
            command_id: Some("cmd-withdraw-http".to_string()),
            trader_id: "trader-1".to_string(),
            amount: 250,
        })?;
        let snapshot = app.snapshot().map_err(|_| WithdrawQuoteError::StoreUnavailable)?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(response.available_quote, 750);
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 750);

        Ok(())
    }

    #[test]
    fn composition_root_wires_withdraw_cli_flow() -> Result<(), WithdrawQuoteError> {
        let app = ExampleApplication::new_in_memory()
            .map_err(|_| WithdrawQuoteError::StoreUnavailable)?;

        let response = app.handle_withdraw_cli(WithdrawQuoteCliCommand {
            trader_id: "trader-1".to_string(),
            amount: 300,
        })?;
        let snapshot = app.snapshot().map_err(|_| WithdrawQuoteError::StoreUnavailable)?;

        assert_eq!(response.account_id, "trader-1");
        assert_eq!(
            response.summary,
            "accepted account_id=trader-1 available_quote=700 frozen_quote=0"
        );
        assert_eq!(snapshot.accounts["trader-1"].available_quote, 700);

        Ok(())
    }
}
