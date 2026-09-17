use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use example_core_use_case::PlaceSpotOrderV2Cmd;
use use_case_executor::trading::spot::place_spot_order_v2_executor::execute_place_spot_order_v2;

fn scheduled_place_commands(round: u64) -> [PlaceSpotOrderV2Cmd; 2] {
    [scheduled_place_command(round, 1), scheduled_place_command(round, 2)]
}

fn scheduled_place_command(round: u64, sequence: u8) -> PlaceSpotOrderV2Cmd {
    PlaceSpotOrderV2Cmd {
        party_id: "buyer".to_string(),
        asset: 10001,
        is_buy: true,
        price: "100".to_string(),
        size: "2".to_string(),
        tif: "ioc".to_string(),
        cloid: Some(format!("timer-{round}-{sequence}")),
    }
}

pub fn consume_place_order_event() {
    // todo 1 查询条件单(满足条件的），

    // todo 2 撮合

    let round = Arc::new(AtomicU64::new(1));
    for (sequence, command) in scheduled_place_commands(round).into_iter().enumerate() {
        let sequence = sequence + 1;
        let cloid = command.cloid.as_deref().unwrap_or("<none>");
        match execute_place_spot_order_v2(&command) {
            Ok(result) => println!(
                "timer spot order succeeded round={round} sequence={sequence} \
                         cloid={cloid} events={}",
                result.events.len()
            ),
            Err(error) => eprintln!(
                "timer spot order failed round={round} sequence={sequence} \
                         cloid={cloid} error={error:?}"
            ),
        }
    }
}
