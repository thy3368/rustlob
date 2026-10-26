use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff,
};
use thiserror::Error;

use crate::entity::{Balance, SpotOrderV2};
use crate::{
    ActivateMatchSpotOrderV2AfterChanges, ActivateMatchSpotOrderV2Changes,
    ActivateMatchSpotOrderV2Cmd, ActivateMatchSpotOrderV2Error, ActivateMatchSpotOrderV2State,
    ActivateMatchSpotOrderV2UseCase, PlaceMatchSpotOrderV2Changes, PlaceMatchSpotOrderV2Error,
    PlaceMatchSpotOrderV2State, PlaceMatchSpotOrderV2UseCase, PlaceOnlySpotOrderV2Cmd,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// 现货下单并检查成交触发条件单时所需的业务状态快照。
///
/// 用于交易入口希望在同一次状态机计算中完成“新订单撮合后，按成交检查并激活已有条件单”
/// 的场景。调用方负责加载订单簿、结算余额和待触发订单；本用例不访问存储。
pub struct PlaceActivateV2State {
    pub maker_orders: Vec<SpotOrderV2>,
    pub settlement_balances: Vec<Balance>,
    pub fee_account_id: String,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
    pub pending_trigger_orders: Vec<SpotOrderV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// 下单撮合及其引发的条件单激活撮合所得到的业务结果。
///
/// `activated_after` 按触发成交及待触发订单的遍历顺序记录结果。
pub struct PlaceActivateV2AfterChanges {
    pub place_after: crate::PlaceMatchSpotOrderV2AfterChanges,
    pub activated_after: Vec<PlaceActivateV2ActivatedOrderAfterChanges>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// 单个条件单激活撮合的输入快照与计算后状态。
pub struct PlaceActivateV2ActivatedOrderAfterChanges {
    pub given_state: ActivateMatchSpotOrderV2State,
    pub after: ActivateMatchSpotOrderV2AfterChanges,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// 可投影为重放事件的下单及条件单变化。
///
/// 事件顺序为原始下单撮合在前，之后依次追加各条件单的激活与撮合事件。
pub struct PlaceActivateV2Changes {
    pub place: PlaceMatchSpotOrderV2Changes,
    pub activated: Vec<ActivateMatchSpotOrderV2Changes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
/// 下单撮合或任一条件单激活撮合失败时返回的业务错误。
pub enum PlaceActivateV2Error {
    #[error(transparent)]
    Place(#[from] PlaceMatchSpotOrderV2Error),
    #[error(transparent)]
    Activate(#[from] ActivateMatchSpotOrderV2Error),
    #[error("fee account id must not be empty")]
    InvalidFeeAccountId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// 触发“下单后检查条件单”流水线的命令。
///
/// 常用于交易所接收一笔普通现货下单后，希望同一业务执行中检查该下单产生的成交，
/// 并立即激活价格条件已满足的待触发订单。
pub struct PlaceActivateV2Cmd {
    pub order: PlaceOnlySpotOrderV2Cmd,
}

#[derive(Debug, Clone, Copy, Default)]
/// 先下单并撮合，再依据这些原始成交激活并撮合已满足触发价的条件单。
///
/// 没有原始成交时不会检查条件单；触发判断按成交资产和价格进行。被激活订单后续
/// 撮合产生的成交不会递归触发更多条件单。调用方应在 `PlaceActivateV2State` 中提供
/// 待检查订单及撮合所需状态，并在外层负责加载、持久化和发布结果。
pub struct PlaceActivateV2UseCase;

impl ReplayableChanges for PlaceActivateV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = self.place.to_replayable_events()?;
        for changes in &self.activated {
            events.extend(changes.to_replayable_events()?);
        }
        Ok(events)
    }
}

impl StateMachineV2 for PlaceActivateV2UseCase {
    type Command = LogEntry<PlaceActivateV2Cmd>;
    type StateGiven = PlaceActivateV2State;
    type Error = PlaceActivateV2Error;
    type StateChanged = PlaceActivateV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        PlaceMatchSpotOrderV2UseCase.check_command(&common_entity::LogEntrySequencer::derive(
            cmd,
            cmd.payload().order.clone(),
        ))?;
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if state.fee_account_id.is_empty() {
            return Err(PlaceActivateV2Error::InvalidFeeAccountId);
        }
        let place_state = PlaceMatchSpotOrderV2State {
            maker_orders: state.maker_orders.clone(),
            settlement_balances: state.settlement_balances.clone(),
            fee_account_id: state.fee_account_id.clone(),
        };
        PlaceMatchSpotOrderV2UseCase.validate_state_given(
            &common_entity::LogEntrySequencer::derive(cmd, cmd.payload().order.clone()),
            &place_state,
        )?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let place_cmd = common_entity::LogEntrySequencer::derive(cmd, cmd.payload().order.clone());
        let place_state = PlaceMatchSpotOrderV2State {
            maker_orders: state.maker_orders.clone(),
            settlement_balances: state.settlement_balances.clone(),
            fee_account_id: state.fee_account_id.clone(),
        };
        let place_after =
            PlaceMatchSpotOrderV2UseCase.compute_state_changed(&place_cmd, &place_state)?;
        let mut activated_after = Vec::new();
        let mut balances = state.settlement_balances.clone();
        let mut makers = state.maker_orders.clone();
        let mut triggered_order_ids = std::collections::HashSet::new();
        for trade in place_match_trades_after(&place_after) {
            for pending in &state.pending_trigger_orders {
                if !pending.trades_asset(trade.asset)
                    || !pending.is_triggered_by_market_price(trade.price)
                    || triggered_order_ids.contains(&pending.order_id())
                {
                    continue;
                }
                let activation_cmd = ActivateMatchSpotOrderV2Cmd {
                    party_id: pending.account_id().to_string(),
                    asset: trade.asset,
                    order_id: pending.order_id(),
                };
                let entry = common_entity::LogEntrySequencer::derive(&place_cmd, activation_cmd);
                let child_state = ActivateMatchSpotOrderV2State {
                    pending_order: pending.clone(),
                    maker_orders: makers.clone(),
                    settlement_balances: balances.clone(),
                    base_asset_id: state.base_asset_id.clone(),
                    quote_asset_id: state.quote_asset_id.clone(),
                    fee_account_id: state.fee_account_id.clone(),
                    maker_fee_bps: state.maker_fee_bps,
                    taker_fee_bps: state.taker_fee_bps,
                };
                let child_after =
                    ActivateMatchSpotOrderV2UseCase.compute_state_changed(&entry, &child_state)?;
                update_working_state(&mut makers, &mut balances, &child_after);
                triggered_order_ids.insert(pending.order_id());
                activated_after.push(PlaceActivateV2ActivatedOrderAfterChanges {
                    given_state: child_state,
                    after: child_after,
                });
            }
        }
        Ok(PlaceActivateV2AfterChanges { place_after, activated_after })
    }
}

impl StateMachineV2Diff for PlaceActivateV2UseCase {
    type StateDiff = PlaceActivateV2Changes;
    fn do_compute_state_diff(
        state: Self::StateGiven,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        let place_state = PlaceMatchSpotOrderV2State {
            maker_orders: state.maker_orders,
            settlement_balances: state.settlement_balances,
            fee_account_id: state.fee_account_id,
        };
        let place =
            PlaceMatchSpotOrderV2UseCase::do_compute_state_diff(place_state, after.place_after)?;
        let activated = after
            .activated_after
            .into_iter()
            .map(|child| {
                ActivateMatchSpotOrderV2UseCase::do_compute_state_diff(
                    child.given_state,
                    child.after,
                )
                .map_err(PlaceActivateV2Error::from)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(PlaceActivateV2Changes { place, activated })
    }
}

fn place_match_trades_after(
    after: &crate::PlaceMatchSpotOrderV2AfterChanges,
) -> Vec<&crate::SpotTrade> {
    match after {
        crate::PlaceMatchSpotOrderV2AfterChanges::SinglePlacedAndMatched {
            match_after, ..
        }
        | crate::PlaceMatchSpotOrderV2AfterChanges::NormalTpslPlacedAndMatched {
            match_after,
            ..
        } => match_after.created_trades().iter().collect(),
        _ => vec![],
    }
}

fn update_working_state(
    makers: &mut Vec<SpotOrderV2>,
    balances: &mut Vec<Balance>,
    after: &ActivateMatchSpotOrderV2AfterChanges,
) {
    use crate::MatchSpotOrderV3AfterChanges;
    let match_after = &after.match_after;
    let (maker_orders_after, balances_after) = match match_after {
        MatchSpotOrderV3AfterChanges::Resting => return,
        MatchSpotOrderV3AfterChanges::Rejected { balances_after, .. } => (None, balances_after),
        MatchSpotOrderV3AfterChanges::PartiallyFilled {
            maker_orders_after,
            balances_after,
            ..
        }
        | MatchSpotOrderV3AfterChanges::Filled { maker_orders_after, balances_after, .. }
        | MatchSpotOrderV3AfterChanges::CanceledAfterPartialFill {
            maker_orders_after,
            balances_after,
            ..
        } => (Some(maker_orders_after), balances_after),
    };
    if let Some(updated_makers) = maker_orders_after {
        for updated in updated_makers {
            if let Some(existing) =
                makers.iter_mut().find(|order| order.order_id() == updated.order_id())
            {
                *existing = updated.clone();
            } else {
                makers.push(updated.clone());
            }
        }
    }
    *balances = balances_after.clone();
}
