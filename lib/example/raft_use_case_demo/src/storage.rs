use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::{Cursor, Error as IoError};
use std::ops::RangeBounds;
use std::sync::Arc;

use example_core_use_case::BalanceV2;
use example_outbound_adapter::database::{
    State4BuildBlock, decode_build_block_state, encode_build_block_state,
};
use lob_repo_v2::imp::lob_btree::LocalLobBtree;
use openraft::storage::{LogFlushed, RaftLogStorage, RaftStateMachine};
use openraft::{
    BasicNode, Entry, EntryPayload, ErrorSubject, ErrorVerb, LogId, LogState, RaftLogReader,
    RaftSnapshotBuilder, Snapshot, SnapshotMeta, StorageError, StoredMembership, Vote,
};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::Mutex;
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::build_block_spot_v3;

use crate::model::{SpotRaftRequest, SpotRaftResponse, USER_ID, symbol};

openraft::declare_raft_types!(
    pub TypeConfig:
        D = SpotRaftRequest,
        R = SpotRaftResponse,
        NodeId = u64,
        Node = BasicNode,
        Entry = Entry<TypeConfig>,
        SnapshotData = Cursor<Vec<u8>>,
);

#[derive(Debug, Default)]
struct LogData {
    logs: BTreeMap<u64, Entry<TypeConfig>>,
    vote: Option<Vote<u64>>,
    committed: Option<LogId<u64>>,
    last_purged: Option<LogId<u64>>,
}

#[derive(Debug, Clone, Default)]
pub struct MemLogStore {
    inner: Arc<Mutex<LogData>>,
}

impl RaftLogReader<TypeConfig> for MemLogStore {
    async fn try_get_log_entries<RB>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<TypeConfig>>, StorageError<u64>>
    where
        RB: RangeBounds<u64> + Clone + Debug + Send,
    {
        let data = self.inner.lock().await;
        Ok(data
            .logs
            .range(range)
            .map(|(_, entry)| entry.clone())
            .collect())
    }
}

impl RaftLogStorage<TypeConfig> for MemLogStore {
    type LogReader = Self;

    async fn get_log_state(&mut self) -> Result<LogState<TypeConfig>, StorageError<u64>> {
        let data = self.inner.lock().await;
        let last_log_id = data
            .logs
            .last_key_value()
            .map(|(_, entry)| entry.log_id)
            .or(data.last_purged);
        Ok(LogState {
            last_purged_log_id: data.last_purged,
            last_log_id,
        })
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn save_vote(&mut self, vote: &Vote<u64>) -> Result<(), StorageError<u64>> {
        self.inner.lock().await.vote = Some(*vote);
        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<u64>>, StorageError<u64>> {
        Ok(self.inner.lock().await.vote)
    }

    async fn save_committed(
        &mut self,
        committed: Option<LogId<u64>>,
    ) -> Result<(), StorageError<u64>> {
        self.inner.lock().await.committed = committed;
        Ok(())
    }

    async fn read_committed(&mut self) -> Result<Option<LogId<u64>>, StorageError<u64>> {
        Ok(self.inner.lock().await.committed)
    }

    async fn append<I>(
        &mut self,
        entries: I,
        callback: LogFlushed<TypeConfig>,
    ) -> Result<(), StorageError<u64>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
        I::IntoIter: Send,
    {
        let mut data = self.inner.lock().await;
        for entry in entries {
            data.logs.insert(entry.log_id.index, entry);
        }
        callback.log_io_completed(Ok(()));
        Ok(())
    }

    async fn truncate(&mut self, log_id: LogId<u64>) -> Result<(), StorageError<u64>> {
        let mut data = self.inner.lock().await;
        let retained = data.logs.split_off(&log_id.index);
        drop(retained);
        Ok(())
    }

    async fn purge(&mut self, log_id: LogId<u64>) -> Result<(), StorageError<u64>> {
        let mut data = self.inner.lock().await;
        let keep_from = log_id.index.saturating_add(1);
        data.logs = data.logs.split_off(&keep_from);
        data.last_purged = Some(log_id);
        Ok(())
    }
}

#[derive(Clone)]
pub struct SpotStateMachine {
    inner: Arc<Mutex<StateMachineData>>,
}

#[derive(Clone)]
struct StateMachineData {
    last_applied: Option<LogId<u64>>,
    membership: StoredMembership<u64, BasicNode>,
    state: State4BuildBlock,
    snapshot: Option<StoredSnapshot>,
    snapshot_sequence: u64,
}

#[derive(Clone)]
struct StoredSnapshot {
    meta: SnapshotMeta<u64, BasicNode>,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct SnapshotWire {
    last_applied: Option<LogId<u64>>,
    membership: StoredMembership<u64, BasicNode>,
    canonical_state: Vec<u8>,
}

impl Default for SpotStateMachine {
    fn default() -> Self {
        Self::new(initial_state())
    }
}

impl SpotStateMachine {
    pub fn new(state: State4BuildBlock) -> Self {
        Self {
            inner: Arc::new(Mutex::new(StateMachineData {
                last_applied: None,
                membership: StoredMembership::default(),
                state,
                snapshot: None,
                snapshot_sequence: 0,
            })),
        }
    }

    pub async fn canonical_state(&self) -> Result<Vec<u8>, StorageError<u64>> {
        encode_build_block_state(&self.inner.lock().await.state).map_err(storage_error)
    }

    pub async fn last_applied(&self) -> Option<LogId<u64>> {
        self.inner.lock().await.last_applied
    }
}

impl RaftStateMachine<TypeConfig> for SpotStateMachine {
    type SnapshotBuilder = SpotSnapshotBuilder;

    async fn applied_state(
        &mut self,
    ) -> Result<(Option<LogId<u64>>, StoredMembership<u64, BasicNode>), StorageError<u64>> {
        let data = self.inner.lock().await;
        Ok((data.last_applied, data.membership.clone()))
    }

    async fn apply<I>(&mut self, entries: I) -> Result<Vec<SpotRaftResponse>, StorageError<u64>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
        I::IntoIter: Send,
    {
        let mut data = self.inner.lock().await;
        let entries = entries.into_iter();
        let mut responses = Vec::with_capacity(entries.size_hint().0);
        for entry in entries {
            let log_id = entry.log_id;
            data.last_applied = Some(log_id);
            let response = match entry.payload {
                EntryPayload::Blank => SpotRaftResponse::blank(log_id.index),
                EntryPayload::Membership(membership) => {
                    data.membership = StoredMembership::new(Some(log_id), membership);
                    SpotRaftResponse::blank(log_id.index)
                }
                EntryPayload::Normal(request) => {
                    let mut candidate = data.state.clone();
                    ///调用状态机的地方，重要！！
                    match build_block_spot_v3(&request.block.items, &mut candidate) {
                        Ok(execution) => {
                            data.state = candidate;
                            SpotRaftResponse::from_execution(log_id.index, &execution)
                        }
                        Err(error) => SpotRaftResponse::rejected_block(log_id.index, error),
                    }
                }
            };
            responses.push(response);
        }
        Ok(responses)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        SpotSnapshotBuilder {
            inner: Arc::clone(&self.inner),
        }
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<Cursor<Vec<u8>>>, StorageError<u64>> {
        Ok(Box::new(Cursor::new(Vec::new())))
    }

    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta<u64, BasicNode>,
        mut snapshot: Box<Cursor<Vec<u8>>>,
    ) -> Result<(), StorageError<u64>> {
        snapshot.rewind().await.map_err(storage_error)?;
        let mut bytes = Vec::new();
        snapshot
            .read_to_end(&mut bytes)
            .await
            .map_err(storage_error)?;
        let wire: SnapshotWire = serde_json::from_slice(&bytes).map_err(storage_error)?;
        let state = decode_build_block_state(&wire.canonical_state).map_err(storage_error)?;
        let mut data = self.inner.lock().await;
        data.last_applied = wire.last_applied;
        data.membership = wire.membership;
        data.state = state;
        data.snapshot = Some(StoredSnapshot {
            meta: meta.clone(),
            data: bytes,
        });
        Ok(())
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<TypeConfig>>, StorageError<u64>> {
        Ok(self
            .inner
            .lock()
            .await
            .snapshot
            .clone()
            .map(|snapshot| Snapshot {
                meta: snapshot.meta,
                snapshot: Box::new(Cursor::new(snapshot.data)),
            }))
    }
}

pub struct SpotSnapshotBuilder {
    inner: Arc<Mutex<StateMachineData>>,
}

impl RaftSnapshotBuilder<TypeConfig> for SpotSnapshotBuilder {
    async fn build_snapshot(&mut self) -> Result<Snapshot<TypeConfig>, StorageError<u64>> {
        let mut data = self.inner.lock().await;
        let wire = SnapshotWire {
            last_applied: data.last_applied,
            membership: data.membership.clone(),
            canonical_state: encode_build_block_state(&data.state).map_err(storage_error)?,
        };
        let bytes = serde_json::to_vec(&wire).map_err(storage_error)?;
        data.snapshot_sequence = data.snapshot_sequence.saturating_add(1);
        let meta = SnapshotMeta {
            last_log_id: data.last_applied,
            last_membership: data.membership.clone(),
            snapshot_id: format!(
                "{}-{}",
                data.last_applied.map_or(0, |id| id.index),
                data.snapshot_sequence
            ),
        };
        data.snapshot = Some(StoredSnapshot {
            meta: meta.clone(),
            data: bytes.clone(),
        });
        Ok(Snapshot {
            meta,
            snapshot: Box::new(Cursor::new(bytes)),
        })
    }
}

fn storage_error(error: impl std::fmt::Display) -> StorageError<u64> {
    StorageError::from_io_error(
        ErrorSubject::StateMachine,
        ErrorVerb::Write,
        IoError::other(error.to_string()),
    )
}

fn initial_state() -> State4BuildBlock {
    let symbol = symbol();
    let buyer = common_entity::stable_string_entity_id(USER_ID);
    State4BuildBlock {
        next_trade_id: 1,
        maker_fee_bps: 5,
        taker_fee_bps: 10,
        spot_symbol_lob: [(symbol, LocalLobBtree::new(symbol))].into(),
        user_symbol_quote_balance: [(
            (USER_ID.to_owned(), symbol),
            BalanceV2::new(buyer, buyer, 2, 100_000, 0, 0, 0),
        )]
        .into(),
        user_symbol_base_balance: [(
            (USER_ID.to_owned(), symbol),
            BalanceV2::new(buyer, buyer, 0, 10, 0, 0, 0),
        )]
        .into(),
        user_symbol_trades: Default::default(),
        user_symbol_orders: Default::default(),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use openraft::testing::StoreBuilder;

    use super::*;

    pub(crate) struct Builder;

    impl StoreBuilder<TypeConfig, MemLogStore, SpotStateMachine> for Builder {
        async fn build(&self) -> Result<((), MemLogStore, SpotStateMachine), StorageError<u64>> {
            Ok(((), MemLogStore::default(), SpotStateMachine::default()))
        }
    }

    #[test]
    fn openraft_storage_suite() -> Result<(), StorageError<u64>> {
        openraft::testing::Suite::test_all(Builder)
    }
}
