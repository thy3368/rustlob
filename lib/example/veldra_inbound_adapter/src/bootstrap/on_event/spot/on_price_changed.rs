use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use example_core_use_case::MatchSpotOrderV3Cmd;
use use_case_executor::trading::spot::match_spot_order_v3_executor::execute_match_spot_order_v3;

///todo 处理条件单

fn scheduled_place_commands(round: u64) -> [MatchSpotOrderV3Cmd; 2] {
    [scheduled_place_command(round, 1), scheduled_place_command(round, 2)]
}

fn scheduled_place_command(round: u64, sequence: u8) -> MatchSpotOrderV3Cmd {
    MatchSpotOrderV3Cmd {
        party_id: "buyer".to_string(),
        asset: 10001,
        order_id: round.saturating_mul(10).saturating_add(u64::from(sequence)),
    }
}

pub fn consume_place_order_event() {
    // todo 1 查询条件单(满足条件的），

    // todo 2 撮合

    let round = Arc::new(AtomicU64::new(1));
    let round_value = round.fetch_add(1, Ordering::Relaxed);
    for (sequence, command) in scheduled_place_commands(round_value).into_iter().enumerate() {
        let sequence = sequence + 1;
        let order_id = command.order_id;
        match execute_match_spot_order_v3(&command) {
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
