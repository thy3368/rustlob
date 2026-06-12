use example_inbound_adapter::{
    DepositQuoteOutboundAccess, deposit_quote_cli_usage, parse_deposit_quote_cli_args,
    run_deposit_quote_cli,
};
use example_outbound_adapter::{InMemoryDepositQuoteOutbound, InMemoryStore, StoreError};

fn main() {
    if let Err(error) = run() {
        eprintln!("cli_deposit_demo failed: {error}");
        eprintln!("{}", deposit_quote_cli_usage());
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
        deposit_quote_outbound: InMemoryDepositQuoteOutbound::from_store(store),
    };
    let command = parse_deposit_quote_cli_args(std::env::args().skip(1))
        .map_err(example_inbound_adapter::CliInboundError::from_parse_error)?;

    println!("== CLI Deposit Command ==");
    println!("{command:#?}");

    let response = run_deposit_quote_cli(command, app.deposit_quote_outbound())
        .map_err(example_inbound_adapter::CliInboundError::from_execution_error)?;
    println!("== CLI Deposit Response ==");
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
    deposit_quote_outbound: InMemoryDepositQuoteOutbound,
}

impl InMemoryCliApp {
    fn snapshot(&self) -> Result<example_outbound_adapter::StoreSnapshot, StoreError> {
        self.store.snapshot()
    }
}

impl DepositQuoteOutboundAccess for InMemoryCliApp {
    type OutboundError = example_outbound_adapter::DepositQuoteOutboundError;
    type Outbound = InMemoryDepositQuoteOutbound;

    fn deposit_quote_outbound(&self) -> &Self::Outbound {
        &self.deposit_quote_outbound
    }
}
