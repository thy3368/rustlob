use common_entity::{StateMachineOwnedV2Diff, StateMachineV2Unchecked};

use crate::SpotTrade;
use crate::use_case::trading::spot::{MatchSpotOrderV3Changes, MatchSpotOrderV3UseCase};

/// 成交观察端口，只读成交事实。
pub trait SpotTradeObserver: Clone + std::fmt::Debug + Send + Sync {
    fn observe(&self, trade: &SpotTrade);
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StdoutSpotTradeObserver;

impl SpotTradeObserver for StdoutSpotTradeObserver {
    fn observe(&self, trade: &SpotTrade) {
        println!(
            "spot trade: trade_id={} match_id={} symbol={} asset={} taker_order_id={} maker_order_id={} taker_side={:?} price={} qty={} executed_at_ms={} taker_fee={} maker_fee={}",
            trade.trade_id,
            trade.match_id,
            trade.symbol,
            trade.asset,
            trade.taker_order_id,
            trade.maker_order_id,
            trade.taker_side,
            trade.price,
            trade.qty,
            trade.executed_at_ms,
            trade.taker_fee,
            trade.maker_fee,
        );
    }
}

/// 撮合后观察成交的 pipeline facade。
#[derive(Debug, Clone)]
pub struct MatchSpotOrderWithNplV3UseCase<O = StdoutSpotTradeObserver> {
    inner: MatchSpotOrderV3UseCase,
    observer: O,
}

impl Default for MatchSpotOrderWithNplV3UseCase {
    fn default() -> Self {
        Self::new(StdoutSpotTradeObserver)
    }
}

impl<O> MatchSpotOrderWithNplV3UseCase<O> {
    pub fn new(observer: O) -> Self {
        Self { inner: MatchSpotOrderV3UseCase, observer }
    }
}

impl<O: SpotTradeObserver> StateMachineV2Unchecked for MatchSpotOrderWithNplV3UseCase<O> {
    type Command = <MatchSpotOrderV3UseCase as StateMachineV2Unchecked>::Command;
    type StateGiven = <MatchSpotOrderV3UseCase as StateMachineV2Unchecked>::StateGiven;
    type Error = <MatchSpotOrderV3UseCase as StateMachineV2Unchecked>::Error;
    type StateChanged = <MatchSpotOrderV3UseCase as StateMachineV2Unchecked>::StateChanged;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        self.inner.check_command(cmd)
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        self.inner.validate_state_given(cmd, state)
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let after = self.inner.compute_state_changed_only(cmd, state)?;
        for trade in after.created_trades() {
            self.observer.observe(trade);
        }
        Ok(after)
    }
}

impl<O: SpotTradeObserver> StateMachineOwnedV2Diff for MatchSpotOrderWithNplV3UseCase<O> {
    type StateDiff = MatchSpotOrderV3Changes;

    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        <MatchSpotOrderV3UseCase as StateMachineOwnedV2Diff>::do_compute_state_diff(state, after)
    }
}
