use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Duration;

use openraft::{BasicNode, Config, Raft};
use tokio::time::{Instant, sleep};

use crate::DemoResult;
use crate::model::{SpotRaftRequest, SpotRaftResponse};
use crate::network::{InProcessNetworkFactory, SpotRaft};
use crate::storage::{MemLogStore, SpotStateMachine, TypeConfig};

const NODE_IDS: [u64; 3] = [1, 2, 3];
const WAIT_TIMEOUT: Duration = Duration::from_secs(10);

struct NodeRuntime {
    raft: SpotRaft,
    state_machine: SpotStateMachine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeSnapshot {
    pub node_id: u64,
    pub applied_index: Option<u64>,
    pub canonical_state: Vec<u8>,
}

pub struct SpotRaftCluster {
    network: InProcessNetworkFactory,
    nodes: BTreeMap<u64, NodeRuntime>,
}

impl SpotRaftCluster {
    pub async fn start() -> DemoResult<Self> {
        let config = Arc::new(
            Config {
                cluster_name: "spot-block-demo".to_owned(),
                election_timeout_min: 150,
                election_timeout_max: 300,
                heartbeat_interval: 50,
                ..Config::default()
            }
            .validate()?,
        );
        let network = InProcessNetworkFactory::default();
        let mut nodes = BTreeMap::new();
        for node_id in NODE_IDS {
            let state_machine = SpotStateMachine::default();
            let raft = Raft::<TypeConfig>::new(
                node_id,
                Arc::clone(&config),
                network.clone(),
                MemLogStore::default(),
                state_machine.clone(),
            )
            .await?;
            network.register(node_id, raft.clone()).await;
            nodes.insert(
                node_id,
                NodeRuntime {
                    raft,
                    state_machine,
                },
            );
        }

        let members: BTreeMap<_, _> = NODE_IDS
            .into_iter()
            .map(|id| (id, BasicNode::new(format!("memory://node-{id}"))))
            .collect();
        let bootstrap = nodes.get(&NODE_IDS[0]).ok_or("bootstrap node is missing")?;
        bootstrap.raft.initialize(members).await?;

        let cluster = Self { network, nodes };
        let _ = cluster.wait_for_leader().await?;
        Ok(cluster)
    }

    pub async fn leader(&self) -> DemoResult<u64> {
        let leaders: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|(node_id, runtime)| {
                (runtime.raft.metrics().borrow().state == openraft::ServerState::Leader)
                    .then_some(*node_id)
            })
            .collect();
        if let [leader] = leaders.as_slice() {
            return Ok(*leader);
        }
        Err("cluster has no live leader".into())
    }

    pub async fn wait_for_leader(&self) -> DemoResult<u64> {
        let deadline = Instant::now() + WAIT_TIMEOUT;
        loop {
            if let Ok(leader) = self.leader().await {
                return Ok(leader);
            }
            if Instant::now() >= deadline {
                return Err("timed out waiting for a unique Raft leader".into());
            }
            sleep(Duration::from_millis(20)).await;
        }
    }

    pub async fn submit(&self, request: SpotRaftRequest) -> DemoResult<SpotRaftResponse> {
        let leader = self.wait_for_leader().await?;
        let runtime = self
            .nodes
            .get(&leader)
            .ok_or("leader stopped before write")?;
        let response = runtime.raft.client_write(request).await?;
        self.wait_for_applied(response.log_id.index).await?;
        Ok(response.data)
    }

    pub async fn wait_for_applied(&self, index: u64) -> DemoResult<()> {
        let deadline = Instant::now() + WAIT_TIMEOUT;
        loop {
            let all_applied = self.nodes.values().all(|runtime| {
                runtime
                    .raft
                    .metrics()
                    .borrow()
                    .last_applied
                    .is_some_and(|log_id| log_id.index >= index)
            });
            if all_applied {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err(format!("timed out waiting for applied index {index}").into());
            }
            sleep(Duration::from_millis(20)).await;
        }
    }

    pub async fn stop_node(&mut self, node_id: u64) -> DemoResult<()> {
        let runtime = self.nodes.remove(&node_id).ok_or("node is not running")?;
        self.network.unregister(node_id).await;
        runtime.raft.shutdown().await?;
        Ok(())
    }

    pub async fn stop_leader(&mut self) -> DemoResult<u64> {
        let leader = self.wait_for_leader().await?;
        self.stop_node(leader).await?;
        let election_node = self
            .nodes
            .values()
            .next()
            .ok_or("no node remains to elect a leader")?;
        election_node.raft.trigger().elect().await?;
        let _ = self.wait_for_leader().await?;
        Ok(leader)
    }

    pub async fn snapshots(&self) -> DemoResult<Vec<NodeSnapshot>> {
        let mut snapshots = Vec::with_capacity(self.nodes.len());
        for (node_id, runtime) in &self.nodes {
            snapshots.push(NodeSnapshot {
                node_id: *node_id,
                applied_index: runtime
                    .state_machine
                    .last_applied()
                    .await
                    .map(|log_id| log_id.index),
                canonical_state: runtime.state_machine.canonical_state().await?,
            });
        }
        snapshots.sort_by_key(|snapshot| snapshot.node_id);
        Ok(snapshots)
    }

    pub async fn states_are_identical(&self) -> DemoResult<bool> {
        let snapshots = self.snapshots().await?;
        Ok(snapshots.windows(2).all(|pair| match pair {
            [left, right] => left.canonical_state == right.canonical_state,
            _ => true,
        }))
    }

    pub fn live_node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use example_core_use_case::SpotOrderStatus;
    use example_outbound_adapter::database::decode_build_block_state;

    use super::*;
    use crate::model::{
        CLOID, CommandStatus, USER_ID, cancel_request, invalid_request, place_and_cancel_request,
        place_request, symbol,
    };

    #[tokio::test]
    async fn three_nodes_apply_place_and_cancel_identically() -> DemoResult<()> {
        let cluster = SpotRaftCluster::start().await?;
        let leader = cluster.leader().await?;
        assert!(NODE_IDS.contains(&leader));

        let response = cluster.submit(place_and_cancel_request()).await?;
        assert_eq!(response.commands.len(), 2);
        assert!(
            response
                .commands
                .iter()
                .all(|result| result.status == CommandStatus::Applied)
        );
        assert!(cluster.states_are_identical().await?);

        let snapshots = cluster.snapshots().await?;
        assert_eq!(snapshots.len(), 3);
        let first = snapshots.first().ok_or("missing state snapshot")?;
        let state = decode_build_block_state(&first.canonical_state)?;
        let orders = state
            .user_symbol_orders
            .get(&(example_core_use_case::PartyId::from(USER_ID), symbol()))
            .ok_or("buyer orders are missing")?;
        let order = orders.first().ok_or("placed order is missing")?;
        assert_eq!(order.order_base().status, SpotOrderStatus::Canceled);
        Ok(())
    }

    #[tokio::test]
    async fn remaining_majority_commits_after_leader_failure() -> DemoResult<()> {
        let mut cluster = SpotRaftCluster::start().await?;
        let placed = cluster.submit(place_request(1)).await?;
        assert_eq!(
            placed.commands.first().map(|item| &item.status),
            Some(&CommandStatus::Applied)
        );

        let old_leader = cluster.stop_leader().await?;
        assert_ne!(cluster.leader().await?, old_leader);
        let canceled = cluster.submit(cancel_request(2)).await?;
        assert_eq!(
            canceled.commands.first().map(|item| &item.status),
            Some(&CommandStatus::Applied)
        );
        assert_eq!(cluster.live_node_count(), 2);
        assert!(cluster.states_are_identical().await?);
        Ok(())
    }

    #[tokio::test]
    async fn invalid_block_is_committed_as_business_rejection() -> DemoResult<()> {
        let cluster = SpotRaftCluster::start().await?;
        let before = cluster.snapshots().await?;
        let response = cluster.submit(invalid_request()).await?;
        let after = cluster.snapshots().await?;

        assert!(response.block_error.is_some());
        assert!(response.commands.is_empty());
        assert!(before.iter().zip(after.iter()).all(|(left, right)| {
            left.canonical_state == right.canonical_state
                && left.applied_index < right.applied_index
        }));
        Ok(())
    }

    #[tokio::test]
    async fn missing_order_cancel_is_deterministically_rejected() -> DemoResult<()> {
        let cluster = SpotRaftCluster::start().await?;
        let response = cluster.submit(cancel_request(1)).await?;

        assert_eq!(
            response.commands.first().map(|item| &item.status),
            Some(&CommandStatus::Rejected)
        );
        assert!(
            response
                .commands
                .first()
                .and_then(|item| item.error.as_ref())
                .is_some()
        );
        assert!(cluster.states_are_identical().await?);
        let _ = CLOID;
        Ok(())
    }
}
