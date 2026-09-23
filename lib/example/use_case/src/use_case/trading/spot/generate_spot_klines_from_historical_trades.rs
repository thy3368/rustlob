use std::collections::BTreeMap;

use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, IssuedByParty, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use thiserror::Error;

use crate::entity::{SpotKline, SpotTrade};

/// 从历史现货成交批量生成固定毫秒周期 K 线的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateSpotKlinesFromHistoricalTradesCmd {
    /// 发起本次历史行情生成的业务主体。
    pub party_id: String,
    /// Hyperliquid 现货资产编号。
    pub asset: u32,
    /// 交易对。
    pub symbol: String,
    /// K 线周期，单位毫秒。
    pub interval_ms: u64,
    /// 包含的历史时间范围起点，单位毫秒。
    pub range_start_ms: u64,
    /// 不包含的历史时间范围终点，单位毫秒。
    pub range_end_ms: u64,
}

impl IssuedByParty for GenerateSpotKlinesFromHistoricalTradesCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 生成历史现货 K 线时加载的成交事实。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateSpotKlinesFromHistoricalTradesState {
    /// 已按业务权限和查询范围加载的候选成交集合。
    pub trades: Vec<SpotTrade>,
}

/// 历史现货 K 线生成的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GenerateSpotKlinesFromHistoricalTradesError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 交易对不能为空。
    #[error("symbol must not be empty")]
    InvalidSymbol,
    /// K 线周期必须大于零。
    #[error("interval_ms must be greater than zero")]
    InvalidIntervalMs,
    /// 时间范围必须是非空半开区间。
    #[error("range_start_ms must be less than range_end_ms")]
    InvalidRange,
    /// 成交所属 asset 与命令不一致。
    #[error("trade {trade_id} asset does not match command")]
    TradeAssetMismatch { trade_id: String },
    /// 成交所属交易对与命令不一致。
    #[error("trade {trade_id} symbol does not match command")]
    TradeSymbolMismatch { trade_id: String },
    /// 成交时间不在命令的半开时间范围内。
    #[error("trade {trade_id} executed_at_ms is outside the requested range")]
    TradeOutsideRange { trade_id: String, executed_at_ms: u64 },
    /// 聚合 K 线时发生整数溢出。
    #[error("arithmetic overflow while generating spot klines")]
    ArithmeticOverflow,
}

/// 历史现货 K 线生成后的业务变化。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateSpotKlinesFromHistoricalTradesChanges {
    /// 按 K 线开盘时间升序排列的新建派生快照。
    pub created_klines: Vec<SpotKline>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GenerateSpotKlinesFromHistoricalTradesUseCase;

impl ReplayableChanges for GenerateSpotKlinesFromHistoricalTradesChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
        self.created_klines.iter().map(Entity::track_create_event).collect()
    }
}

impl StateMachineV2Unchecked for GenerateSpotKlinesFromHistoricalTradesUseCase {
    type Command = GenerateSpotKlinesFromHistoricalTradesCmd;
    type StateGiven = GenerateSpotKlinesFromHistoricalTradesState;
    type Error = GenerateSpotKlinesFromHistoricalTradesError;
    type StateChanged = GenerateSpotKlinesFromHistoricalTradesChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidPartyId);
        }
        if cmd.symbol.is_empty() {
            return Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidSymbol);
        }
        if cmd.interval_ms == 0 {
            return Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidIntervalMs);
        }
        if cmd.range_start_ms >= cmd.range_end_ms {
            return Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidRange);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        for trade in &state.trades {
            if trade.asset != cmd.asset {
                return Err(GenerateSpotKlinesFromHistoricalTradesError::TradeAssetMismatch {
                    trade_id: trade.trade_id.clone(),
                });
            }
            if trade.symbol != cmd.symbol {
                return Err(GenerateSpotKlinesFromHistoricalTradesError::TradeSymbolMismatch {
                    trade_id: trade.trade_id.clone(),
                });
            }
            if !(cmd.range_start_ms..cmd.range_end_ms).contains(&trade.executed_at_ms) {
                return Err(GenerateSpotKlinesFromHistoricalTradesError::TradeOutsideRange {
                    trade_id: trade.trade_id.clone(),
                    executed_at_ms: trade.executed_at_ms,
                });
            }
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
        _context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut trades = state.trades.clone();
        trades.sort_by(|left, right| {
            left.executed_at_ms
                .cmp(&right.executed_at_ms)
                .then_with(|| left.trade_id.cmp(&right.trade_id))
        });

        let mut buckets = BTreeMap::<u64, KlineAccumulator>::new();
        for trade in trades {
            let bucket_index = trade.executed_at_ms / cmd.interval_ms;
            let open_time_ms = bucket_index
                .checked_mul(cmd.interval_ms)
                .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
            let notional_quote = trade
                .price
                .checked_mul(trade.qty)
                .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;

            match buckets.get_mut(&open_time_ms) {
                Some(bucket) => bucket.add(&trade, notional_quote)?,
                None => {
                    buckets.insert(open_time_ms, KlineAccumulator::first(&trade, notional_quote));
                }
            }
        }

        let mut created_klines = Vec::with_capacity(buckets.len());
        for (open_time_ms, bucket) in buckets {
            let close_time_ms = open_time_ms
                .checked_add(
                    cmd.interval_ms
                        .checked_sub(1)
                        .ok_or(GenerateSpotKlinesFromHistoricalTradesError::InvalidIntervalMs)?,
                )
                .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
            created_klines.push(SpotKline::new(
                cmd.asset,
                cmd.symbol.clone(),
                cmd.interval_ms,
                open_time_ms,
                close_time_ms,
                bucket.open_price,
                bucket.high_price,
                bucket.low_price,
                bucket.close_price,
                bucket.volume_base,
                bucket.volume_quote,
                bucket.trade_count,
                bucket.first_trade_id,
                bucket.last_trade_id,
            ));
        }

        Ok(GenerateSpotKlinesFromHistoricalTradesChanges { created_klines })
    }
}

impl StateMachineOwnedV2Diff for GenerateSpotKlinesFromHistoricalTradesUseCase {
    type StateDiff = GenerateSpotKlinesFromHistoricalTradesChanges;

    fn do_compute_state_diff(
        _given_state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(after)
    }
}

struct KlineAccumulator {
    open_price: u64,
    high_price: u64,
    low_price: u64,
    close_price: u64,
    volume_base: u64,
    volume_quote: u64,
    trade_count: u64,
    first_trade_id: String,
    last_trade_id: String,
}

impl KlineAccumulator {
    fn first(trade: &SpotTrade, notional_quote: u64) -> Self {
        Self {
            open_price: trade.price,
            high_price: trade.price,
            low_price: trade.price,
            close_price: trade.price,
            volume_base: trade.qty,
            volume_quote: notional_quote,
            trade_count: 1,
            first_trade_id: trade.trade_id.clone(),
            last_trade_id: trade.trade_id.clone(),
        }
    }

    fn add(
        &mut self,
        trade: &SpotTrade,
        notional_quote: u64,
    ) -> Result<(), GenerateSpotKlinesFromHistoricalTradesError> {
        self.high_price = self.high_price.max(trade.price);
        self.low_price = self.low_price.min(trade.price);
        self.close_price = trade.price;
        self.volume_base = self
            .volume_base
            .checked_add(trade.qty)
            .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
        self.volume_quote = self
            .volume_quote
            .checked_add(notional_quote)
            .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
        self.trade_count = self
            .trade_count
            .checked_add(1)
            .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
        self.last_trade_id = trade.trade_id.clone();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{ReplayableChanges, StateMachineOwnedV2Diff};

    use super::*;
    use crate::entity::SpotOrderSide;

    const ASSET: u32 = 10_001;
    const SYMBOL: &str = "BTCUSDT";
    const INTERVAL_MS: u64 = 60_000;

    fn command() -> GenerateSpotKlinesFromHistoricalTradesCmd {
        GenerateSpotKlinesFromHistoricalTradesCmd {
            party_id: "market-data".to_string(),
            asset: ASSET,
            symbol: SYMBOL.to_string(),
            interval_ms: INTERVAL_MS,
            range_start_ms: 0,
            range_end_ms: 180_000,
        }
    }

    fn trade(trade_id: &str, executed_at_ms: u64, price: u64, qty: u64) -> SpotTrade {
        SpotTrade::new(
            trade_id.to_string(),
            format!("match-{trade_id}"),
            ASSET,
            SYMBOL.to_string(),
            format!("taker-{trade_id}"),
            format!("maker-{trade_id}"),
            "buyer".to_string(),
            "seller".to_string(),
            SpotOrderSide::Buy,
            price,
            qty,
            0,
            0,
            executed_at_ms,
        )
    }

    #[test]
    fn one_bucket_aggregates_ohlcv_and_trade_identity() {
        // 规则：同一固定时间桶内的历史成交应生成一根完整 K 线，并保留首尾成交身份。
        // 给定：三笔成交落在 60_000..120_000 毫秒区间，成交时间顺序与输入顺序不一致。
        // 执行：按历史成交生成 K 线。
        // 预期：open/close 按成交时间取值，high/low、双边成交量、成交笔数和首尾成交 ID 正确。
        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![
                trade("trade-2", 61_000, 110, 2),
                trade("trade-1", 60_000, 100, 3),
                trade("trade-3", 62_000, 90, 1),
            ],
        };

        let changes = GenerateSpotKlinesFromHistoricalTradesUseCase
            .compute_state_diff(&command(), state)
            .expect("one bucket");

        assert_eq!(changes.created_klines.len(), 1);
        assert_eq!(
            changes.created_klines[0],
            SpotKline::new(
                ASSET,
                SYMBOL.to_string(),
                INTERVAL_MS,
                60_000,
                119_999,
                100,
                110,
                90,
                90,
                6,
                610,
                3,
                "trade-1".to_string(),
                "trade-3".to_string(),
            )
        );
    }

    #[test]
    fn multiple_buckets_are_returned_in_open_time_order() {
        // 规则：不同时间桶的 K 线应彼此独立，并按 open_time_ms 升序输出。
        // 给定：两笔成交分别落在 60_000 毫秒桶和 120_000 毫秒桶。
        // 执行：按 60_000 毫秒周期生成历史 K 线。
        // 预期：输出两根 K 线，且不受历史成交输入顺序影响。
        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![trade("trade-2", 120_001, 120, 1), trade("trade-1", 60_001, 100, 2)],
        };

        let changes = GenerateSpotKlinesFromHistoricalTradesUseCase
            .compute_state_diff(&command(), state)
            .expect("multiple buckets");

        assert_eq!(
            changes.created_klines.iter().map(|kline| kline.open_time_ms).collect::<Vec<_>>(),
            vec![60_000, 120_000]
        );
    }

    #[test]
    fn same_timestamp_uses_trade_id_for_open_and_close_order() {
        // 规则：同一毫秒发生多笔成交时，必须用 trade_id 提供稳定的开收盘排序。
        // 给定：trade-a 与 trade-b 的 executed_at_ms 相同，但输入顺序相反。
        // 执行：生成它们所在时间桶的 K 线。
        // 预期：按 trade_id 升序确定 open、close 及 first/last_trade_id，结果可重复。
        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![trade("trade-b", 60_000, 120, 1), trade("trade-a", 60_000, 100, 1)],
        };

        let changes = GenerateSpotKlinesFromHistoricalTradesUseCase
            .compute_state_diff(&command(), state)
            .expect("stable same-time ordering");

        assert_eq!(changes.created_klines[0].open_price, 100);
        assert_eq!(changes.created_klines[0].close_price, 120);
        assert_eq!(changes.created_klines[0].first_trade_id, "trade-a");
        assert_eq!(changes.created_klines[0].last_trade_id, "trade-b");
    }

    #[test]
    fn validation_rejects_mismatches_and_invalid_ranges() {
        // 规则：命令参数和已加载成交事实必须满足 K 线生成边界。
        // 给定：分别构造 party/symbol 为空、interval_ms 为零、时间范围为空、
        //       成交 asset/symbol 不匹配以及成交时间越界的输入。
        // 执行：运行命令预检查和基于状态的校验。
        // 预期：每类非法输入都返回对应的领域错误，而不是进入聚合过程。
        let use_case = GenerateSpotKlinesFromHistoricalTradesUseCase;
        let mut invalid = command();
        invalid.interval_ms = 0;
        assert_eq!(
            use_case.check_command(&invalid),
            Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidIntervalMs)
        );

        let mut invalid = command();
        invalid.range_start_ms = invalid.range_end_ms;
        assert_eq!(
            use_case.check_command(&invalid),
            Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidRange)
        );

        let mut invalid = command();
        invalid.party_id.clear();
        assert_eq!(
            use_case.check_command(&invalid),
            Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidPartyId)
        );

        let mut invalid = command();
        invalid.symbol.clear();
        assert_eq!(
            use_case.check_command(&invalid),
            Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidSymbol)
        );

        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![SpotTrade { asset: 10_002, ..trade("trade-1", 60_000, 100, 1) }],
        };
        assert_eq!(
            use_case.validate_state_given(&command(), &state),
            Err(GenerateSpotKlinesFromHistoricalTradesError::TradeAssetMismatch {
                trade_id: "trade-1".to_string(),
            })
        );

        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![SpotTrade {
                symbol: "ETHUSDT".to_string(),
                ..trade("trade-2", 60_000, 100, 1)
            }],
        };
        assert_eq!(
            use_case.validate_state_given(&command(), &state),
            Err(GenerateSpotKlinesFromHistoricalTradesError::TradeSymbolMismatch {
                trade_id: "trade-2".to_string(),
            })
        );

        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![trade("trade-3", 180_000, 100, 1)],
        };
        assert_eq!(
            use_case.validate_state_given(&command(), &state),
            Err(GenerateSpotKlinesFromHistoricalTradesError::TradeOutsideRange {
                trade_id: "trade-3".to_string(),
                executed_at_ms: 180_000,
            })
        );
    }

    #[test]
    fn empty_history_has_no_snapshots_and_no_events() {
        // 规则：历史区间没有成交时，不应人为补齐空 K 线。
        // 给定：合法的交易对、周期和时间范围，但成交集合为空。
        // 执行：生成 K 线并投影 replay events。
        // 预期：created_klines 和 replay events 都为空。
        let changes = GenerateSpotKlinesFromHistoricalTradesUseCase
            .compute_state_diff(
                &command(),
                GenerateSpotKlinesFromHistoricalTradesState { trades: vec![] },
            )
            .expect("empty history");

        assert!(changes.created_klines.is_empty());
        assert!(changes.to_replayable_events().expect("empty events").is_empty());
    }

    #[test]
    fn quote_notional_overflow_is_a_domain_error() {
        // 规则：volume_quote 的 price * qty 计算溢出时必须拒绝本次聚合。
        // 给定：价格为 u64::MAX、数量为 2 的成交。
        // 执行：生成其所在时间桶的 K 线。
        // 预期：返回 ArithmeticOverflow，不产生截断后的错误行情数据。
        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![trade("overflow", 60_000, u64::MAX, 2)],
        };

        assert_eq!(
            GenerateSpotKlinesFromHistoricalTradesUseCase.compute_state_diff(&command(), state),
            Err(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)
        );
    }

    #[test]
    fn replay_projection_matches_created_kline_count_and_identity() {
        // 规则：K 线 Changes 是唯一业务真相，replay events 必须逐根从 created_klines 投影。
        // 给定：两笔成交分别生成两个不同时间桶的 K 线。
        // 执行：将聚合 Changes 转换为 replay events。
        // 预期：事件数量与 K 线数量一致，事件身份与对应 K 线身份一致，且全部为创建事件。
        let state = GenerateSpotKlinesFromHistoricalTradesState {
            trades: vec![trade("trade-1", 60_000, 100, 1), trade("trade-2", 120_000, 101, 1)],
        };
        let changes = GenerateSpotKlinesFromHistoricalTradesUseCase
            .compute_state_diff(&command(), state)
            .expect("replayable changes");
        let events = changes.to_replayable_events().expect("replayable events");

        assert_eq!(events.len(), changes.created_klines.len());
        assert_eq!(events[0].entity_id, changes.created_klines[0].replay_entity_id().unwrap());
        assert!(events.iter().all(common_entity::EntityReplayableEvent::is_created));
    }
}
