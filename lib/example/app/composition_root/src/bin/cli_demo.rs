use example_composition_root::InMemoryExampleApplication;
use example_inbound_adapter::{parse_place_order_cli_args, place_order_cli_usage};

fn main() {
    if let Err(error) = run() {
        eprintln!("cli_demo failed: {error}");
        eprintln!("{}", place_order_cli_usage());
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = InMemoryExampleApplication::new_in_memory()?;
    let command = parse_place_order_cli_args(std::env::args().skip(1))?;

    println!("== CLI Command ==");
    println!("{command:#?}");

    let response = app.handle_cli(command)?;
    println!("== CLI Response ==");
    println!("{response:#?}");

    let snapshot = app.snapshot()?;
    println!("== Store Snapshot ==");
    println!("{snapshot:#?}");

    Ok(())
}
