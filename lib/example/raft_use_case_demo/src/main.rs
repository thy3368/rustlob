// Demo CLI 按计划打印 leader、提交结果和状态一致性摘要。
#![allow(clippy::print_stdout)]

use raft_use_case_demo::{CommandStatus, DemoResult, SpotRaftCluster, place_and_cancel_request};

#[tokio::main]
async fn main() -> DemoResult<()> {
    let cluster = SpotRaftCluster::start().await?;
    let leader = cluster.leader().await?;
    let response = cluster.submit(place_and_cancel_request()).await?;
    let snapshots = cluster.snapshots().await?;

    println!("leader: {leader}");
    println!("committed/applied index: {}", response.applied_log_index);
    for (index, command) in response.commands.iter().enumerate() {
        let status = match command.status {
            CommandStatus::Applied => "Applied",
            CommandStatus::Rejected => "Rejected",
        };
        println!(
            "command {}: status={status}, replayable_events={}, error={:?}",
            index + 1,
            command.replayable_event_count,
            command.error
        );
    }
    println!(
        "nodes={}, canonical_state_identical={}",
        snapshots.len(),
        cluster.states_are_identical().await?
    );
    Ok(())
}
