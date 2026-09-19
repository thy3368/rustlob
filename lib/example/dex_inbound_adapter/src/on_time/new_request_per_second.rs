use std::time::Duration;

use example_core_use_case::{
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
    SpotBlockCommand,
};

const DEFAULT_HOTSTUFF_URL: &str = "http://127.0.0.1:39001/spot-block/commands";
const REQUEST_TIMEOUT: Duration = Duration::from_millis(800);
const REQUEST_INTERVAL: Duration = Duration::from_secs(1);

pub fn build_request(sequence: u64) -> SpotBlockCommand {
    SpotBlockCommand::PlaceMatch(PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "buyer".to_string(),
        asset: 10_001,
        order_id: format!("timer-order-{sequence}"),
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        price: "100".to_string(),
        size: "1".to_string(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "ioc".to_string() },
        reduce_only: false,
        cloid: Some(format!("timer-cloid-{sequence}")),
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 5,
        taker_fee_bps: 10,
    }))
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

    fn order_fields(command: &SpotBlockCommand) -> (&str, &str) {
        let SpotBlockCommand::PlaceMatch(PlaceOnlySpotOrderV2Cmd::Single(order)) = command else {
            panic!("expected a single PlaceMatch command");
        };
        (order.order_id.as_str(), order.cloid.as_deref().unwrap_or_default())
    }

    #[test]
    fn build_request_uses_unique_sequence_for_order_id_and_cloid() {
        let first = build_request(7);
        let second = build_request(8);

        assert_ne!(order_fields(&first).0, order_fields(&second).0);
        assert_ne!(order_fields(&first).1, order_fields(&second).1);
    }

    #[test]
    fn build_request_is_place_match_and_serializable() {
        let command = build_request(1);

        assert!(matches!(command, SpotBlockCommand::PlaceMatch(_)));
        let payload = serde_json::to_vec(&command);
        assert!(payload.map(|bytes| !bytes.is_empty()).unwrap_or(false));
    }
}
