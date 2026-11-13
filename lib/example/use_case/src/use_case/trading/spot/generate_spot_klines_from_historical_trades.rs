use std::collections::BTreeMap;

use common_entity::{
    Entity, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff, UseCase,
};
use example_core_entity::TradeBase;
use example_core_entity::account::symbol::Symbol;
use example_core_entity::spot::SpotTradeV3;
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::SpotKline;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateSpotKlinesFromHistoricalTradesCmd {
    pub party_id: String,
    pub symbol: Symbol,
    pub interval_ns: u64,
    pub range_start_ns: u64,
    pub range_end_ns: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateSpotKlinesFromHistoricalTradesState {
    pub trades: Vec<SpotTradeV3>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GenerateSpotKlinesFromHistoricalTradesError {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("interval_ns must be greater than zero")]
    InvalidIntervalNs,
    #[error("range_start_ns must be less than range_end_ns")]
    InvalidRange,
    #[error("trade {trade_id} symbol does not match command")]
    TradeSymbolMismatch { trade_id: u64 },
    #[error("trade {trade_id} created_at is outside the requested range")]
    TradeOutsideRange { trade_id: u64, created_at_ns: u64 },
    #[error("trade {trade_id} price must be greater than zero")]
    InvalidTradePrice { trade_id: u64 },
    #[error("trade {trade_id} amount must be greater than zero")]
    InvalidTradeAmount { trade_id: u64 },
    #[error("arithmetic overflow while generating spot klines")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateSpotKlinesFromHistoricalTradesChanges {
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

impl UseCase for GenerateSpotKlinesFromHistoricalTradesUseCase {
    type Command = LogEntry<GenerateSpotKlinesFromHistoricalTradesCmd>;
    type StateGiven = GenerateSpotKlinesFromHistoricalTradesState;
    type Error = GenerateSpotKlinesFromHistoricalTradesError;
    type StateChanged = GenerateSpotKlinesFromHistoricalTradesChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(Self::Error::InvalidPartyId);
        }
        if cmd.interval_ns == 0 {
            return Err(Self::Error::InvalidIntervalNs);
        }
        if cmd.range_start_ns >= cmd.range_end_ns {
            return Err(Self::Error::InvalidRange);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        for trade in &state.trades {
            let fact = &trade.comon_trade;
            if fact.symbol != cmd.symbol {
                return Err(Self::Error::TradeSymbolMismatch { trade_id: fact.trade_id });
            }
            if !(cmd.range_start_ns..cmd.range_end_ns).contains(&fact.created_at) {
                return Err(Self::Error::TradeOutsideRange {
                    trade_id: fact.trade_id,
                    created_at_ns: fact.created_at,
                });
            }
            if fact.price <= Decimal::ZERO {
                return Err(Self::Error::InvalidTradePrice { trade_id: fact.trade_id });
            }
            if fact.size <= Decimal::ZERO {
                return Err(Self::Error::InvalidTradeAmount { trade_id: fact.trade_id });
            }
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut trades: Vec<_> = state.trades.iter().map(|trade| &trade.comon_trade).collect();
        trades.sort_unstable_by_key(|trade| (trade.created_at, trade.trade_id));
        let mut buckets = BTreeMap::<u64, KlineAccumulator>::new();
        for trade in trades {
            let open_time_ns = (trade.created_at / cmd.interval_ns)
                .checked_mul(cmd.interval_ns)
                .ok_or(Self::Error::ArithmeticOverflow)?;
            let notional =
                trade.price.checked_mul(trade.size).ok_or(Self::Error::ArithmeticOverflow)?;
            match buckets.get_mut(&open_time_ns) {
                Some(bucket) => bucket.add(trade, notional)?,
                None => {
                    buckets.insert(open_time_ns, KlineAccumulator::first(trade, notional));
                }
            }
        }
        let mut created_klines = Vec::with_capacity(buckets.len());
        for (open_time_ns, bucket) in buckets {
            let close_time_ns = open_time_ns
                .checked_add(cmd.interval_ns - 1)
                .ok_or(Self::Error::ArithmeticOverflow)?;
            created_klines.push(SpotKline::new(
                cmd.symbol,
                cmd.interval_ns,
                open_time_ns,
                close_time_ns,
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

impl StateDiff for GenerateSpotKlinesFromHistoricalTradesUseCase {
    type StateDiff = GenerateSpotKlinesFromHistoricalTradesChanges;
    fn do_compute_state_diff(
        _given: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(after)
    }
}

struct KlineAccumulator {
    open_price: Decimal,
    high_price: Decimal,
    low_price: Decimal,
    close_price: Decimal,
    volume_base: Decimal,
    volume_quote: Decimal,
    trade_count: u64,
    first_trade_id: u64,
    last_trade_id: u64,
}

impl KlineAccumulator {
    fn first(trade: &TradeBase, notional: Decimal) -> Self {
        Self {
            open_price: trade.price,
            high_price: trade.price,
            low_price: trade.price,
            close_price: trade.price,
            volume_base: trade.size,
            volume_quote: notional,
            trade_count: 1,
            first_trade_id: trade.trade_id,
            last_trade_id: trade.trade_id,
        }
    }
    fn add(
        &mut self,
        trade: &TradeBase,
        notional: Decimal,
    ) -> Result<(), GenerateSpotKlinesFromHistoricalTradesError> {
        self.high_price = self.high_price.max(trade.price);
        self.low_price = self.low_price.min(trade.price);
        self.close_price = trade.price;
        self.volume_base = self
            .volume_base
            .checked_add(trade.size)
            .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
        self.volume_quote = self
            .volume_quote
            .checked_add(notional)
            .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
        self.trade_count = self
            .trade_count
            .checked_add(1)
            .ok_or(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)?;
        self.last_trade_id = trade.trade_id;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use common_entity::{LogEntrySequencer, ReplayableChanges};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::spot::SpotTradeSettlementStatus;
    use example_core_entity::{OrderSide, UserId};

    use super::*;

    fn symbol() -> Symbol {
        Symbol::new(7, Coin::BTC, Coin::USDT)
    }

    fn command(
        start: u64,
        end: u64,
        interval: u64,
    ) -> LogEntry<GenerateSpotKlinesFromHistoricalTradesCmd> {
        LogEntrySequencer::from_metadata(
            1,
            1,
            1,
            1,
            "market-data".to_owned(),
            GenerateSpotKlinesFromHistoricalTradesCmd {
                party_id: "market-data".into(),
                symbol: symbol(),
                interval_ns: interval,
                range_start_ns: start,
                range_end_ns: end,
            },
        )
    }

    fn trade(id: u64, created_at: u64, price: Decimal, size: Decimal) -> SpotTradeV3 {
        SpotTradeV3 {
            comon_trade: TradeBase {
                trade_id: id,
                symbol: symbol(),
                taker_user: UserId(1),
                maker_user: UserId(2),
                taker_order_id: id + 100,
                maker_order_id: id + 200,
                taker_side: OrderSide::Buy,
                price,
                size,
                taker_fee: Decimal::ZERO,
                maker_fee: Decimal::ZERO,
                created_at,
                updated_at: created_at,
                version: 1,
            },
            settlement_status: SpotTradeSettlementStatus::Unsettled,
        }
    }

    fn compute(
        cmd: &LogEntry<GenerateSpotKlinesFromHistoricalTradesCmd>,
        trades: Vec<SpotTradeV3>,
    ) -> Result<
        GenerateSpotKlinesFromHistoricalTradesChanges,
        GenerateSpotKlinesFromHistoricalTradesError,
    > {
        GenerateSpotKlinesFromHistoricalTradesUseCase
            .compute_state_changed(cmd, &GenerateSpotKlinesFromHistoricalTradesState { trades })
    }

    #[test]
    fn aggregates_decimal_trades_into_epoch_aligned_windows_in_stable_order() {
        let cmd = command(65, 185, 60);
        let changes = compute(
            &cmd,
            vec![
                trade(4, 181, Decimal::new(1305, 1), Decimal::new(5, 1)),
                trade(3, 119, Decimal::new(1105, 1), Decimal::new(2, 1)),
                trade(1, 65, Decimal::new(1005, 1), Decimal::new(15, 1)),
                trade(2, 90, Decimal::new(905, 1), Decimal::new(25, 1)),
            ],
        )
        .expect("valid aggregation");

        assert_eq!(changes.created_klines.len(), 2);
        let first = &changes.created_klines[0];
        assert_eq!((first.open_time_ns, first.close_time_ns), (60, 119));
        assert_eq!(
            (first.open_price, first.high_price, first.low_price, first.close_price),
            (
                Decimal::new(1005, 1),
                Decimal::new(1105, 1),
                Decimal::new(905, 1),
                Decimal::new(1105, 1)
            )
        );
        assert_eq!(first.volume_base, Decimal::new(42, 1));
        assert_eq!(first.volume_quote, Decimal::new(39910, 2));
        assert_eq!((first.first_trade_id, first.last_trade_id), (1, 3));
        assert_eq!(
            (changes.created_klines[1].open_time_ns, changes.created_klines[1].close_time_ns),
            (180, 239)
        );

        let events = changes.to_replayable_events().expect("create events");
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].entity_id, first.id);
        assert_eq!(events[1].entity_id, changes.created_klines[1].id);
    }

    #[test]
    fn equal_timestamp_uses_trade_id_for_open_and_close() {
        let cmd = command(0, 60, 60);
        let changes = compute(
            &cmd,
            vec![
                trade(9, 10, Decimal::new(90, 0), Decimal::ONE),
                trade(3, 10, Decimal::new(30, 0), Decimal::ONE),
            ],
        )
        .expect("valid aggregation");
        let kline = &changes.created_klines[0];
        assert_eq!(
            (kline.open_price, kline.close_price),
            (Decimal::new(30, 0), Decimal::new(90, 0))
        );
        assert_eq!((kline.first_trade_id, kline.last_trade_id), (3, 9));
    }

    #[test]
    fn empty_trades_produce_no_changes() {
        assert!(
            compute(&command(1, 2, 1), vec![]).expect("empty is valid").created_klines.is_empty()
        );
    }

    #[test]
    fn validates_symbol_range_and_non_positive_amounts() {
        let cmd = command(10, 20, 10);
        let mut wrong_symbol = trade(1, 10, Decimal::ONE, Decimal::ONE);
        wrong_symbol.comon_trade.symbol = Symbol::new(8, Coin::ETH, Coin::USDT);
        assert_eq!(
            compute(&cmd, vec![wrong_symbol]),
            Err(GenerateSpotKlinesFromHistoricalTradesError::TradeSymbolMismatch { trade_id: 1 })
        );
        assert_eq!(
            compute(&cmd, vec![trade(2, 20, Decimal::ONE, Decimal::ONE)]),
            Err(GenerateSpotKlinesFromHistoricalTradesError::TradeOutsideRange {
                trade_id: 2,
                created_at_ns: 20
            })
        );
        assert_eq!(
            compute(&cmd, vec![trade(3, 10, Decimal::ZERO, Decimal::ONE)]),
            Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidTradePrice { trade_id: 3 })
        );
        assert_eq!(
            compute(&cmd, vec![trade(4, 10, Decimal::ONE, -Decimal::ONE)]),
            Err(GenerateSpotKlinesFromHistoricalTradesError::InvalidTradeAmount { trade_id: 4 })
        );
    }

    #[test]
    fn rejects_decimal_and_close_time_overflow() {
        let decimal_cmd = command(0, 2, 1);
        assert_eq!(
            compute(&decimal_cmd, vec![trade(1, 0, Decimal::MAX, Decimal::new(2, 0))]),
            Err(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)
        );

        let time_cmd = command(u64::MAX - 1, u64::MAX, u64::MAX - 1);
        assert_eq!(
            compute(&time_cmd, vec![trade(2, u64::MAX - 1, Decimal::ONE, Decimal::ONE)]),
            Err(GenerateSpotKlinesFromHistoricalTradesError::ArithmeticOverflow)
        );
    }
}
