pub mod on_command;
pub mod on_event;
pub mod on_time;

fn main() {
    if let Err(error) = on_time::new_request_per_second::run_request_per_second() {
        eprintln!("[dex_inbound_adapter] failed to start request producer: {error}");
        std::process::exit(1);
    }
}
