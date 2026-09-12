use std::time::{Duration, Instant, SystemTime};

use tokio_cron_scheduler::{Job, JobScheduler};

pub type TimerResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn run_timer() -> TimerResult<()> {
    const TIMER_NAME: &str = "veldra_timer";
    const INTERVAL: Duration = Duration::from_secs(10);

    let started_at = Instant::now();
    let scheduler = JobScheduler::new().await?;
    let job = Job::new_repeated_async(INTERVAL, move |job_id, _scheduler| {
        Box::pin(async move {
            println!(
                "hello timer_name={TIMER_NAME} job_id={job_id} interval={INTERVAL:?} \
                 triggered_at={:?} elapsed_secs={}",
                SystemTime::now(),
                started_at.elapsed().as_secs()
            );
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    let mut heartbeat = tokio::time::interval(Duration::from_secs(10));
    loop {
        heartbeat.tick().await;
    }
}
