use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use example_core_use_case::MatchSpotOrderV2Cmd;
use use_case_executor::trading::spot::open_match_spot_order_v2_executor::execute_place_spot_order_v2;

fn scheduled_place_commands(round: u64) -> [MatchSpotOrderV2Cmd; 2] {
    [scheduled_place_command(round, 1), scheduled_place_command(round, 2)]
}

fn scheduled_place_command(round: u64, sequence: u8) -> MatchSpotOrderV2Cmd {
    MatchSpotOrderV2Cmd {
        party_id: "buyer".to_string(),
        asset: 10001,
        order_id: format!("timer-{round}-{sequence}"),
    }
}

pub fn consume_place_order_event() {
    // todo 1 查询条件单(满足条件的），

    // todo 2 撮合

    let round = Arc::new(AtomicU64::new(1));
    let round_value = round.fetch_add(1, Ordering::Relaxed);
    for (sequence, command) in scheduled_place_commands(round_value).into_iter().enumerate() {
        let sequence = sequence + 1;
        let order_id = command.order_id.as_str();
        match execute_place_spot_order_v2(&command) {
            Ok(result) => println!(
                "timer spot order succeeded round={round_value} sequence={sequence} \
                         order_id={order_id} events={}",
                result.events.len()
            ),
            Err(error) => eprintln!(
                "timer spot order failed round={round_value} sequence={sequence} \
                         order_id={order_id} error={error:?}"
            ),
        }
    }
}
