use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler};

pub type TimerResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn run_timer() -> TimerResult<()> {
    let scheduler = JobScheduler::new().await?;
    let job = Job::new_repeated_async(Duration::from_secs(10), |_job_id, _scheduler| {
        Box::pin(async {
            println!("hello");
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    let mut heartbeat = tokio::time::interval(Duration::from_secs(10));
    loop {
        heartbeat.tick().await;
    }
}
