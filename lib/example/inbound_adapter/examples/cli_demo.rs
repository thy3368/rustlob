use example_inbound_adapter::{
    PlaceOrderOutboundAccess, parse_place_order_cli_args, place_order_cli_usage,
    run_place_order_cli,
};
use example_outbound_adapter::{
    InMemoryPlaceOrderOutbound, InMemorySpotPipelineBroker, InMemoryStore, StoreError,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("cli_demo failed: {error}");
        eprintln!("{}", place_order_cli_usage());
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
    let broker = InMemorySpotPipelineBroker::default();
    let app = InMemoryCliApp {
        store: store.clone(),
        place_order_outbound: InMemoryPlaceOrderOutbound::from_store_with_broker(store, broker),
    };
    let command = parse_place_order_cli_args(std::env::args().skip(1))
        .map_err(example_inbound_adapter::CliInboundError::from_parse_error)?;

    println!("== CLI Command ==");
    println!("{command:#?}");

    let response = run_place_order_cli(command, app.place_order_outbound())
        .map_err(example_inbound_adapter::CliInboundError::from_mi_execution_error)?;
    println!("== CLI Response ==");
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
    place_order_outbound: InMemoryPlaceOrderOutbound,
}

impl InMemoryCliApp {
    fn snapshot(&self) -> Result<example_outbound_adapter::StoreSnapshot, StoreError> {
        self.store.snapshot()
    }
}

impl PlaceOrderOutboundAccess for InMemoryCliApp {
    type OutboundError = example_outbound_adapter::PlaceOrderOutboundError;
    type Outbound = InMemoryPlaceOrderOutbound;

    fn place_order_outbound(&self) -> &Self::Outbound {
        &self.place_order_outbound
    }
}
