use example_composition_root::InMemoryExampleApplication;
use example_inbound_adapter::{
    CliInboundError, parse_withdraw_quote_cli_args, withdraw_quote_cli_usage,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("cli_withdraw_demo failed: {error}");
        eprintln!("{}", withdraw_quote_cli_usage());
        std::process::exit(error.exit_code());
    }
}

fn run() -> Result<(), CliInboundError> {
    let app = InMemoryExampleApplication::new_in_memory()
        .map_err(|error| CliInboundError::runtime("app_init_failed", error))?;
    let command = parse_withdraw_quote_cli_args(std::env::args().skip(1))
        .map_err(CliInboundError::from_parse_error)?;

    println!("== CLI Withdraw Command ==");
    println!("{command:#?}");

    let response =
        app.handle_withdraw_cli(command).map_err(CliInboundError::from_execution_error)?;
    println!("== CLI Withdraw Response ==");
    println!("{response:#?}");

    let snapshot =
        app.snapshot().map_err(|error| CliInboundError::runtime("snapshot_failed", error))?;
    println!("== Store Snapshot ==");
    println!("{snapshot:#?}");

    Ok(())
}
