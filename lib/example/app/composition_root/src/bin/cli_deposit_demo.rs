use example_composition_root::InMemoryExampleApplication;
use example_inbound_adapter::{deposit_quote_cli_usage, parse_deposit_quote_cli_args};

fn main() {
    if let Err(error) = run() {
        eprintln!("cli_deposit_demo failed: {error}");
        eprintln!("{}", deposit_quote_cli_usage());
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = InMemoryExampleApplication::new_in_memory()?;
    let command = parse_deposit_quote_cli_args(std::env::args().skip(1))?;

    println!("== CLI Deposit Command ==");
    println!("{command:#?}");

    let response = app.handle_deposit_cli(command)?;
    println!("== CLI Deposit Response ==");
    println!("{response:#?}");

    let snapshot = app.snapshot()?;
    println!("== Store Snapshot ==");
    println!("{snapshot:#?}");

    Ok(())
}
