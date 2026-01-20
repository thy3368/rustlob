// ============================================================================
// 行情查询处理器
// ============================================================================

use crate::proc::trading_market_data_proc::*;

/// 行情查询处理器
///
/// 负责处理行情查询命令并返回查询结果
pub struct MarketDataQueryProcessorImpl<S, I>
where
    S: Level3SnapshotRepo,
    I: IncrementalDataRepo
{
    /// 快照数据仓储（订单簿）
    snapshot_repo: S,
    /// 增量数据仓储
    incremental_repo: I
}

impl<S, I> MarketDataQueryProcessorImpl<S, I>
where
    S: Level3SnapshotRepo,
    I: IncrementalDataRepo
{
    /// 创建查询处理器
    pub fn new(snapshot_repo: S, incremental_repo: I) -> Self {
        Self {
            snapshot_repo,
            incremental_repo
        }
    }
}

impl<S, I> MarketDataQueryProc for MarketDataQueryProcessorImpl<S, I>
where
    S: Level3SnapshotRepo,
    I: IncrementalDataRepo
{
    // ========================================================================
    // Level 1 查询处理
    // ========================================================================

    /// 处理 Level 1 查询
    fn query_level1(&self, query: QueryLevel1) -> Result<Level1QueryResult, MarketDataQueryError> {
        self.snapshot_repo
            .query_level1(query.symbol_id, query.sequence)
            .map(|snapshot| Level1QueryResult {
                snapshot
            })
            .ok_or(MarketDataQueryError::EmptyOrderBook {
                symbol_id: query.symbol_id
            })
    }

    /// 处理 Level 1 批量查询
    fn query_level1_batch(&self, query: QueryLevel1Batch) -> Level1BatchQueryResult {
        let mut snapshots = Vec::new();
        let mut failed_symbols = Vec::new();

        for symbol_id in query.symbol_ids {
            match self.snapshot_repo.query_level1(symbol_id, query.sequence) {
                Some(snapshot) => snapshots.push((symbol_id, snapshot)),
                None => failed_symbols.push(symbol_id)
            }
        }

        Level1BatchQueryResult {
            snapshots,
            failed_symbols
        }
    }

    // ========================================================================
    // Level 2 查询处理
    // ========================================================================

    /// 处理 Level 2 查询
    fn query_level2(&self, query: QueryLevel2) -> Level2QueryResult {
        let snapshot = self.snapshot_repo.query_level2(query.symbol_id, query.sequence, query.depth);

        Level2QueryResult {
            snapshot
        }
    }

    // ========================================================================
    // Level 3 查询处理
    // ========================================================================

    /// 处理 Level 3 查询
    fn query_level3(&self, query: QueryLevel3) -> Level3QueryResult {
        let snapshot = self.snapshot_repo.query_level3(query.symbol_id, query.sequence);

        Level3QueryResult {
            snapshot
        }
    }

    // ========================================================================
    // 增量数据查询处理
    // ========================================================================

    /// 处理增量数据查询
    fn query_incremental_data(
        &self, query: QueryIncrementalData
    ) -> Result<IncrementalDataRes, MarketDataQueryError> {
        // 验证序列号范围
        if query.from_sequence >= query.to_sequence {
            return Err(MarketDataQueryError::InvalidParameter {
                field: "sequence_range",
                reason: "from_sequence must be less than to_sequence"
            });
        }

        // 查询增量数据
        let deltas =
            self.incremental_repo.query_incremental_data(query.symbol_id, query.from_sequence, query.to_sequence)?;

        // 获取最新序列号，判断是否还有更多数据
        let latest_sequence = self.incremental_repo.get_latest_sequence(query.symbol_id).unwrap_or(query.to_sequence);

        let has_more = query.to_sequence < latest_sequence;

        Ok(IncrementalDataRes {
            symbol_id: query.symbol_id,
            deltas,
            from_sequence: query.from_sequence,
            to_sequence: query.to_sequence,
            has_more
        })
    }
}
