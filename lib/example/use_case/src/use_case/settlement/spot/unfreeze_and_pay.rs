use common_entity::{LogEntry, StateMachineV2};
use example_core_entity::spot::spot_trade_v3::SpotTradeV3;
use example_core_entity::{BalanceV2, Open};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UnFreezeSpotOrderV2Cmd {
    pub order_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2State {
    pub trade: SpotTradeV3,
    pub buy_balance: BalanceV2<Open>,
    pub sell_balance: BalanceV2<Open>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnFreezeSpotOrderV2AfterChanges {
    pub buy_balance_after: BalanceV2<Open>,
    pub sell_balance_after: BalanceV2<Open>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UnFreeze4SpotOrderV2UseCase;

impl StateMachineV2 for UnFreeze4SpotOrderV2UseCase {
    type Command = LogEntry<UnFreezeSpotOrderV2Cmd>;
    type StateGiven = UnFreezeSpotOrderV2State;
    type Error = ();
    type StateChanged = UnFreezeSpotOrderV2AfterChanges;

    fn compute_state_changed_only(
        &self,
        _cmd: &Self::Command,
        _given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        _given_state.trade.settle(_given_state.buy_balance, _given_state.sell_balance);

        //todo

        todo!()
    }
}
