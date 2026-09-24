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
