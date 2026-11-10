use std::time::Duration;

use example_core_use_case::account::coin::Coin;
use example_core_use_case::account::symbol::Symbol;
use example_core_use_case::{
    OrderTif, PlaceOnlySpotOrderV3Cmd, PlaceOnlySpotOrderV3OrderCmd, PlaceOnlySpotOrderV3OrderType,
    UserId,
};
use rust_decimal::Decimal;

const DEFAULT_HOTSTUFF_URL: &str = "http://127.0.0.1:39001/spot-block/commands";
const REQUEST_TIMEOUT: Duration = Duration::from_millis(800);
const REQUEST_INTERVAL: Duration = Duration::from_secs(1);

pub fn build_request(sequence: u64) -> PlaceOnlySpotOrderV3Cmd {
    PlaceOnlySpotOrderV3Cmd::Single(PlaceOnlySpotOrderV3OrderCmd {
        symbol: Symbol::new(10_001, Coin::BTC, Coin::USDT),
        owner: UserId(1),
        taker_max_fee: Decimal::new(10, 4),
        order_id: sequence.saturating_add(1),
        is_buy: true,
        price: "100".to_string(),
        size: "1".to_string(),
        order_type: PlaceOnlySpotOrderV3OrderType::None,
        reduce_only: false,
        tif: OrderTif::Ioc,
        cloid: Some(format!("0x{sequence:032x}")),
    })
}

pub fn run_request_per_second() -> Result<(), reqwest::Error> {
    let url =
        std::env::var("DEX_HOTSTUFF_URL").unwrap_or_else(|_| DEFAULT_HOTSTUFF_URL.to_string());
    let client = reqwest::blocking::Client::builder().timeout(REQUEST_TIMEOUT).build()?;
    let mut sequence = 0_u64;

    loop {
        let command = build_request(sequence);
        match client.post(&url).json(&command).send() {
            Ok(response) if response.status().is_success() => {
                println!("[dex_inbound_adapter] submitted sequence={sequence}");
            }
            Ok(response) => {
                eprintln!(
                    "[dex_inbound_adapter] submit sequence={sequence} failed: HTTP {}",
                    response.status()
                );
            }
            Err(error) => {
                eprintln!("[dex_inbound_adapter] submit sequence={sequence} failed: {error}");
            }
        }
        sequence = sequence.wrapping_add(1);
        std::thread::sleep(REQUEST_INTERVAL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn order_fields(command: &PlaceOnlySpotOrderV3Cmd) -> (u64, &str) {
        let PlaceOnlySpotOrderV3Cmd::Single(order) = command else {
            panic!("expected a single place command");
        };
        (order.order_id, order.cloid.as_deref().unwrap_or_default())
    }

    #[test]
    fn build_request_uses_unique_sequence_for_order_id_and_cloid() {
        let first = build_request(7);
        let second = build_request(8);

        assert_ne!(order_fields(&first).0, order_fields(&second).0);
        assert_ne!(order_fields(&first).1, order_fields(&second).1);
        assert_eq!(order_fields(&first).0, 8);
        assert_eq!(order_fields(&second).0, 9);
        assert_eq!(order_fields(&first).1, "0x00000000000000000000000000000007");
        assert_eq!(order_fields(&second).1, "0x00000000000000000000000000000008");
    }

    #[test]
    fn build_request_is_place_match_and_serializable() {
        let command = build_request(1);

        assert!(matches!(command, PlaceOnlySpotOrderV3Cmd::Single(_)));
        let payload = serde_json::to_vec(&command);
        assert!(payload.map(|bytes| !bytes.is_empty()).unwrap_or(false));
    }
}
