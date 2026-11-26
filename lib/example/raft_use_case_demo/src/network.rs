use std::collections::HashMap;
use std::io::{Error as IoError, ErrorKind};
use std::sync::Arc;

use openraft::error::{InstallSnapshotError, RPCError, RaftError, RemoteError, Unreachable};
use openraft::network::{RPCOption, RaftNetwork, RaftNetworkFactory};
use openraft::raft::{
    AppendEntriesRequest, AppendEntriesResponse, InstallSnapshotRequest, InstallSnapshotResponse,
    VoteRequest, VoteResponse,
};
use openraft::{BasicNode, Raft};
use tokio::sync::RwLock;

use crate::storage::TypeConfig;

pub(crate) type SpotRaft = Raft<TypeConfig>;

#[derive(Clone, Default)]
pub(crate) struct InProcessNetworkFactory {
    registry: Arc<RwLock<HashMap<u64, SpotRaft>>>,
}

impl InProcessNetworkFactory {
    pub(crate) async fn register(&self, id: u64, raft: SpotRaft) {
        self.registry.write().await.insert(id, raft);
    }

    pub(crate) async fn unregister(&self, id: u64) {
        self.registry.write().await.remove(&id);
    }
}

impl RaftNetworkFactory<TypeConfig> for InProcessNetworkFactory {
    type Network = InProcessNetwork;

    async fn new_client(&mut self, target: u64, _node: &BasicNode) -> Self::Network {
        InProcessNetwork {
            target,
            registry: Arc::clone(&self.registry),
        }
    }
}

pub(crate) struct InProcessNetwork {
    target: u64,
    registry: Arc<RwLock<HashMap<u64, SpotRaft>>>,
}

impl InProcessNetwork {
    async fn target(&self) -> Result<SpotRaft, Unreachable> {
        self.registry
            .read()
            .await
            .get(&self.target)
            .cloned()
            .ok_or_else(|| {
                Unreachable::new(&IoError::new(
                    ErrorKind::NotConnected,
                    format!("raft node {} is unavailable", self.target),
                ))
            })
    }
}

impl RaftNetwork<TypeConfig> for InProcessNetwork {
    async fn append_entries(
        &mut self,
        rpc: AppendEntriesRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<AppendEntriesResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        self.target()
            .await?
            .append_entries(rpc)
            .await
            .map_err(|error| RemoteError::new(self.target, error).into())
    }

    async fn install_snapshot(
        &mut self,
        rpc: InstallSnapshotRequest<TypeConfig>,
        _option: RPCOption,
    ) -> Result<
        InstallSnapshotResponse<u64>,
        RPCError<u64, BasicNode, RaftError<u64, InstallSnapshotError>>,
    > {
        self.target()
            .await?
            .install_snapshot(rpc)
            .await
            .map_err(|error| RemoteError::new(self.target, error).into())
    }

    async fn vote(
        &mut self,
        rpc: VoteRequest<u64>,
        _option: RPCOption,
    ) -> Result<VoteResponse<u64>, RPCError<u64, BasicNode, RaftError<u64>>> {
        self.target()
            .await?
            .vote(rpc)
            .await
            .map_err(|error| RemoteError::new(self.target, error).into())
    }
}
