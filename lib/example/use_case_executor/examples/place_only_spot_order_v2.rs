//! 运行：`cargo run -p use_case_executor --example place_only_spot_order_v2`

use cmd_handler::EntityReplayableEvent;
use common_entity::{EntityChangeType, LogEntry, LogEntrySequenceError, LogEntrySequencer};
use example_core_use_case::{
    PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType,
};
use example_outbound_adapter::NoopSpotOrderV2PlaceOnlyOutbound;
use use_case_executor::trading::spot::place_only_spot_order_v2_executor::execute_place_only_spot_order_v2_with_outbound;

fn command() -> Result<LogEntry<PlaceOnlySpotOrderV2Cmd>, LogEntrySequenceError> {
    let payload = PlaceOnlySpotOrderV2Cmd::Single(PlaceOnlySpotOrderV2OrderCmd {
        party_id: "example-trader".to_string(),
        asset: 10_001,
        order_id: 1,
        symbol: "BTCUSDT".to_string(),
        is_buy: true,
        price: "100".to_string(),
        size: "2".to_string(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: "gtc".to_string() },
        reduce_only: false,
        cloid: Some("example-cloid".to_string()),
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        maker_fee_bps: 1,
        taker_fee_bps: 5,
    });
    let mut sequencer = LogEntrySequencer::new(None);
    sequencer.sequence(7, 9, payload)
}

fn main() {
    let command = match command() {
        Ok(command) => command,
        Err(error) => {
            eprintln!("place-only spot order v2 command sequencing failed: {error}");
            std::process::exit(1);
        }
    };
    let outbound = NoopSpotOrderV2PlaceOnlyOutbound;
    let result = match execute_place_only_spot_order_v2_with_outbound(&command, &outbound) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("place-only spot order v2 execution failed: {error:?}");
            std::process::exit(1);
        }
    };

    let created_orders = match &result.changes {
        example_core_use_case::PlaceOnlySpotOrderV2Changes::Single { .. } => 1,
        example_core_use_case::PlaceOnlySpotOrderV2Changes::NormalTpsl {
            created_child_orders,
            ..
        } => 1 + created_child_orders.len(),
    };
    assert_eq!(created_orders, 1, "single command should create one order");
    assert_eq!(result.events.len(), 1, "single command should create one event");

    let event: &EntityReplayableEvent = &result.events[0];
    assert_eq!(event.lsn, command.lsn());
    assert_eq!(event.logical_ts, command.logical_ts());
    assert_eq!(event.change_type, EntityChangeType::Created.as_tag());

    println!("created_orders={created_orders}");
    println!("events={}", result.events.len());
    println!("event_lsn={}", event.lsn);
    println!("event_logical_ts={}", event.logical_ts);
    println!("event_entity_type={}", event.entity_type);
}
