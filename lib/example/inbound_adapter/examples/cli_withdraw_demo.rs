use example_inbound_adapter::{
    WithdrawQuoteOutboundAccess, parse_withdraw_quote_cli_args, run_withdraw_quote_cli,
    withdraw_quote_cli_usage,
};
use example_outbound_adapter::{InMemoryStore, InMemoryWithdrawQuoteOutbound, StoreError};

fn main() {
    if let Err(error) = run() {
        eprintln!("cli_withdraw_demo failed: {error}");
        eprintln!("{}", withdraw_quote_cli_usage());
        std::process::exit(error.exit_code());
    }
}

fn run() -> Result<(), example_inbound_adapter::CliInboundError> {
    let store = InMemoryStore::seed_balances(
        example_core::Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 2),
        example_core::Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 2),
        example_core::MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
    )
    .map_err(|error| example_inbound_adapter::CliInboundError::runtime("app_init_failed", error))?;
    let app = InMemoryCliApp {
        store: store.clone(),
        withdraw_quote_outbound: InMemoryWithdrawQuoteOutbound::from_store(store),
    };
    let command = parse_withdraw_quote_cli_args(std::env::args().skip(1))
        .map_err(example_inbound_adapter::CliInboundError::from_parse_error)?;

    println!("== CLI Withdraw Command ==");
    println!("{command:#?}");

    let response = run_withdraw_quote_cli(command, app.withdraw_quote_outbound())
        .map_err(example_inbound_adapter::CliInboundError::from_execution_error)?;
    println!("== CLI Withdraw Response ==");
    println!("{response:#?}");

    let snapshot = app.snapshot().map_err(|error| {
        example_inbound_adapter::CliInboundError::runtime("snapshot_failed", error)
    })?;
    println!("== Store Snapshot ==");
    println!("{snapshot:#?}");

    Ok(())
}

struct InMemoryCliApp {
    store: InMemoryStore,
    withdraw_quote_outbound: InMemoryWithdrawQuoteOutbound,
}

impl InMemoryCliApp {
    fn snapshot(&self) -> Result<example_outbound_adapter::StoreSnapshot, StoreError> {
        self.store.snapshot()
    }
}

impl WithdrawQuoteOutboundAccess for InMemoryCliApp {
    type OutboundError = example_outbound_adapter::WithdrawQuoteOutboundError;
    type Outbound = InMemoryWithdrawQuoteOutbound;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound {
        &self.withdraw_quote_outbound
    }
}
