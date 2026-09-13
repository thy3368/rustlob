use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime};

use example_core_use_case::PlaceSpotOrderV2Cmd;
use tokio_cron_scheduler::{Job, JobScheduler};
use use_case_executor::trading::spot::place_spot_order_v2_executor::execute_place_spot_order_v2;

pub type TimerResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

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

pub async fn run_timer() -> TimerResult<()> {
    const TIMER_NAME: &str = "veldra_timer";
    const INTERVAL: Duration = Duration::from_secs(10);

    let started_at = Instant::now();
    let round = Arc::new(AtomicU64::new(1));
    let scheduler = JobScheduler::new().await?;
    let job = Job::new_repeated_async(INTERVAL, move |job_id, _scheduler| {
        let round = round.clone();
        Box::pin(async move {
            let round = round.fetch_add(1, Ordering::Relaxed);
            println!(
                "hello timer_name={TIMER_NAME} job_id={job_id} interval={INTERVAL:?} \
                 triggered_at={:?} elapsed_secs={} round={round}",
                SystemTime::now(),
                started_at.elapsed().as_secs()
            );

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
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    let mut heartbeat = tokio::time::interval(Duration::from_secs(10));
    loop {
        heartbeat.tick().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduled_place_commands_builds_two_distinct_orders() {
        let commands = scheduled_place_commands(7);

        assert_eq!(commands.len(), 2);
        assert_ne!(commands[0].cloid, commands[1].cloid);
        for command in &commands {
            assert_eq!(command.party_id, "buyer");
            assert_eq!(command.asset, 10001);
            assert!(command.is_buy);
            assert_eq!(command.price, "100");
            assert_eq!(command.size, "2");
            assert_eq!(command.tif, "ioc");
        }
        assert_eq!(commands[0].cloid.as_deref(), Some("timer-7-1"));
        assert_eq!(commands[1].cloid.as_deref(), Some("timer-7-2"));
    }
}
