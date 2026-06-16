use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges,
};
use thiserror::Error;

use super::place_order::{
    PlaceImmediateOrderChanges, PlaceImmediateOrderCmd, PlaceImmediateOrderState,
    PlaceImmediateOrderUseCase, PlaceOrderError,
};
use super::{
    MatchSpotOrderChanges, MatchSpotOrderCmd, MatchSpotOrderError, MatchSpotOrderState,
    MatchSpotOrderUseCase, SettleSpotTradeCmd, SettleSpotTradeError, SettleSpotTradeState,
    SettleSpotTradeUseCase,
};
use crate::entity::{Balance, SpotOrder, SpotOrderTimeInForce};

/// 串联立即下单、撮合、清结算三段现货执行流程的业务命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteImmediateSpotOrderPipelineCmd {
    /// 第 1 段立即下单命令。
    pub place: PlaceImmediateOrderCmd,
    /// 第 2 段撮合批次 ID。
    pub match_id: String,
    /// 第 3 段清结算批次 ID。
    pub settlement_batch_id: String,
}

impl IssuedByParty for ExecuteImmediateSpotOrderPipelineCmd {
    fn party_id(&self) -> Option<&str> {
        self.place.party_id()
    }
}

/// 复合现货执行流程所需的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteImmediateSpotOrderPipelineState {
    /// 第 1 段立即下单所需状态。
    pub place_state: PlaceImmediateOrderState,
    /// 已按撮合优先级排好的 maker 订单。
    pub maker_orders: Vec<SpotOrder>,
    /// 参与清结算的余额快照。
    pub settlement_balances: Vec<Balance>,
    /// 本批次已清结算过的 trade id。
    pub settled_trade_ids: Vec<String>,
    /// base 资产 ID。
    pub base_asset_id: String,
    /// quote 资产 ID。
    pub quote_asset_id: String,
}

/// 复合现货执行流程可能出现的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ExecuteImmediateSpotOrderPipelineError {
    /// 撮合批次 ID 不能为空。
    #[error("match_id must not be empty")]
    InvalidMatchId,
    /// 清结算批次 ID 不能为空。
    #[error("settlement_batch_id must not be empty")]
    InvalidSettlementBatchId,
    /// 第 1 段立即下单失败。
    #[error(transparent)]
    Place(#[from] PlaceOrderError),
    /// 第 2 段撮合失败。
    #[error(transparent)]
    Match(#[from] MatchSpotOrderError),
    /// 第 3 段清结算失败。
    #[error(transparent)]
    Settle(#[from] SettleSpotTradeError),
}

/// 立即下单执行流水线的 typed output。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteImmediateSpotOrderPipelineChanges {
    /// 第 1 段立即下单产出的 taker 订单与受影响余额。
    pub place_output: PlaceImmediateOrderChanges,
    /// 第 2 段撮合结果；未进入撮合时为空。
    pub match_output: Option<MatchSpotOrderChanges>,
    pub settle_changes: Option<super::SettleSpotTradeChanges>,
}

/// 用于示例目录的同步现货执行编排 use case。
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteImmediateSpotOrderPipelineUseCase;

impl ReplayableChanges for ExecuteImmediateSpotOrderPipelineChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events = self.place_output.to_replayable_events()?;
        if let Some(match_output) = &self.match_output {
            events.extend(match_output.to_replayable_events()?);
        }
        if let Some(settle_changes) = &self.settle_changes {
            events.extend(settle_changes.to_replayable_events()?);
        }
        Ok(events)
    }
}

impl CommandUseCase4 for ExecuteImmediateSpotOrderPipelineUseCase {
    type Command = ExecuteImmediateSpotOrderPipelineCmd;
    type GivenState = ExecuteImmediateSpotOrderPipelineState;
    type Error = ExecuteImmediateSpotOrderPipelineError;
    type Changes = ExecuteImmediateSpotOrderPipelineChanges;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.match_id.is_empty() {
            return Err(ExecuteImmediateSpotOrderPipelineError::InvalidMatchId);
        }
        if cmd.settlement_batch_id.is_empty() {
            return Err(ExecuteImmediateSpotOrderPipelineError::InvalidSettlementBatchId);
        }

        CommandUseCase4::pre_check_command(&PlaceImmediateOrderUseCase, &cmd.place)?;
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        CommandUseCase4::validate_against_state(
            &PlaceImmediateOrderUseCase,
            &cmd.place,
            &state.place_state,
        )?;
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let place_output =
            PlaceImmediateOrderUseCase.compute_changes(&cmd.place, state.place_state)?;
        let taker_order = place_output.order.clone();
        let affected_balance_after = place_output.affected_balance_after.clone();

        if !should_enter_matching(&taker_order, &state.maker_orders)? {
            return Ok(ExecuteImmediateSpotOrderPipelineChanges {
                place_output,
                match_output: None,
                settle_changes: None,
            });
        }

        let match_cmd = MatchSpotOrderCmd {
            party_id: cmd.place.party_id.clone(),
            taker_order_id: taker_order.order_id.clone(),
            match_id: cmd.match_id.clone(),
        };
        let match_state = MatchSpotOrderState { taker_order, maker_orders: state.maker_orders };

        CommandUseCase4::pre_check_command(&MatchSpotOrderUseCase, &match_cmd)?;
        CommandUseCase4::validate_against_state(&MatchSpotOrderUseCase, &match_cmd, &match_state)?;
        let match_output =
            CommandUseCase4::compute_changes(&MatchSpotOrderUseCase, &match_cmd, match_state)?;
        let trades = match_output.trades.clone();

        if trades.is_empty() {
            return Ok(ExecuteImmediateSpotOrderPipelineChanges {
                place_output,
                match_output: Some(match_output),
                settle_changes: None,
            });
        }

        let settle_cmd = SettleSpotTradeCmd {
            party_id: cmd.place.party_id.clone(),
            settlement_batch_id: cmd.settlement_batch_id.clone(),
            trade_ids: trades.iter().map(|trade| trade.trade_id.clone()).collect(),
        };
        let settle_state = SettleSpotTradeState {
            trades,
            base_asset_id: state.base_asset_id,
            quote_asset_id: state.quote_asset_id,
            balances: settlement_balances_after_place(
                state.settlement_balances,
                affected_balance_after,
            ),
            settled_trade_ids: state.settled_trade_ids,
        };

        CommandUseCase4::pre_check_command(&SettleSpotTradeUseCase, &settle_cmd)?;
        CommandUseCase4::validate_against_state(
            &SettleSpotTradeUseCase,
            &settle_cmd,
            &settle_state,
        )?;
        let settle_changes =
            CommandUseCase4::compute_changes(&SettleSpotTradeUseCase, &settle_cmd, settle_state)?;

        Ok(ExecuteImmediateSpotOrderPipelineChanges {
            place_output,
            match_output: Some(match_output),
            settle_changes: Some(settle_changes),
        })
    }
}

fn should_enter_matching(
    taker_order: &SpotOrder,
    maker_orders: &[SpotOrder],
) -> Result<bool, ExecuteImmediateSpotOrderPipelineError> {
    if matches!(taker_order.time_in_force, SpotOrderTimeInForce::Ioc) {
        return Ok(true);
    }

    let Some(best_maker) = maker_orders.first() else {
        return Ok(false);
    };

    match taker_order.crosses_order(best_maker) {
        Ok(crosses) => Ok(crosses),
        Err(_) => Ok(true),
    }
}

#[cfg(test)]
fn has_field(event: &EntityReplayableEvent, field_name: &str) -> bool {
    event.field_changes.iter().any(|change| change.field_name_as_str().ok() == Some(field_name))
}

fn settlement_balances_after_place(
    mut balances: Vec<Balance>,
    affected_balance_after: Balance,
) -> Vec<Balance> {
    upsert_balance(&mut balances, affected_balance_after);
    balances
}

fn upsert_balance(balances: &mut Vec<Balance>, next_balance: Balance) {
    if let Some(balance) = balances.iter_mut().find(|balance| {
        balance.account_id == next_balance.account_id && balance.asset_id == next_balance.asset_id
    }) {
        *balance = next_balance;
        return;
    }

    balances.push(next_balance);
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::{SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason};
    use crate::{MarketRules, PlaceImmediateOrderExecution, PlaceOrderTimeInForce};

    fn pipeline_cmd() -> ExecuteImmediateSpotOrderPipelineCmd {
        ExecuteImmediateSpotOrderPipelineCmd {
            place: PlaceImmediateOrderCmd {
                party_id: "trader-1".to_string(),
                asset: 10_001,
                symbol: "BTCUSDT".to_string(),
                is_buy: true,
                size: 2,
                reduce_only: false,
                execution: PlaceImmediateOrderExecution::Limit {
                    price: 100,
                    time_in_force: PlaceOrderTimeInForce::Gtc,
                },
                cloid: Some("cloid-1".to_string()),
            },
            match_id: "match-1".to_string(),
            settlement_batch_id: "settle-1".to_string(),
        }
    }

    fn place_state() -> PlaceImmediateOrderState {
        PlaceImmediateOrderState {
            trading_enabled: true,
            next_order_sequence: 7,
            account_id: "trader-1".to_string(),
            base_balance: Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 1),
            quote_balance: Balance::new("trader-1".to_string(), "USDT".to_string(), 500, 0, 1),
            market_rules: MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
        }
    }

    fn pipeline_state() -> ExecuteImmediateSpotOrderPipelineState {
        ExecuteImmediateSpotOrderPipelineState {
            place_state: place_state(),
            maker_orders: Vec::new(),
            settlement_balances: vec![
                Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 1),
                Balance::new("trader-1".to_string(), "USDT".to_string(), 500, 0, 1),
                Balance::new("maker-1".to_string(), "BTC".to_string(), 0, 1, 1),
                Balance::new("maker-1".to_string(), "USDT".to_string(), 0, 0, 1),
            ],
            settled_trade_ids: Vec::new(),
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
        }
    }

    fn maker_sell(order_id: &str, qty: u64, price: u64) -> SpotOrder {
        SpotOrder::new(
            order_id.to_string(),
            10_001,
            None,
            "maker-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            crate::entity::SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            qty,
            0,
            None,
        )
    }

    fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }

            std::str::from_utf8(change.new_value_bytes()).ok()
        })
    }

    fn pipeline_events(
        cmd: &ExecuteImmediateSpotOrderPipelineCmd,
        state: ExecuteImmediateSpotOrderPipelineState,
    ) -> Result<Vec<EntityReplayableEvent>, ExecuteImmediateSpotOrderPipelineError> {
        Ok(ExecuteImmediateSpotOrderPipelineUseCase
            .compute_changes(cmd, state)?
            .to_replayable_events()
            .map_err(|_| {
                ExecuteImmediateSpotOrderPipelineError::Settle(
                    SettleSpotTradeError::ArithmeticOverflow,
                )
            })?)
    }

    #[test]
    fn role_returns_trader() {
        assert_eq!(ExecuteImmediateSpotOrderPipelineUseCase.role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_missing_pipeline_ids() {
        let mut cmd = pipeline_cmd();
        cmd.match_id.clear();

        assert_eq!(
            ExecuteImmediateSpotOrderPipelineUseCase.pre_check_command(&cmd),
            Err(ExecuteImmediateSpotOrderPipelineError::InvalidMatchId)
        );

        let mut cmd = pipeline_cmd();
        cmd.settlement_batch_id.clear();

        assert_eq!(
            ExecuteImmediateSpotOrderPipelineUseCase.pre_check_command(&cmd),
            Err(ExecuteImmediateSpotOrderPipelineError::InvalidSettlementBatchId)
        );
    }

    #[test]
    fn validate_against_state_reuses_place_validation() {
        let mut state = pipeline_state();
        state.place_state.quote_balance.available = 10;

        assert_eq!(
            ExecuteImmediateSpotOrderPipelineUseCase
                .validate_against_state(&pipeline_cmd(), &state),
            Err(ExecuteImmediateSpotOrderPipelineError::Place(
                PlaceOrderError::InsufficientQuoteBalance
            ))
        );
    }

    #[test]
    fn compute_replayable_events_skips_match_and_settle_when_gtc_does_not_cross()
    -> Result<(), ExecuteImmediateSpotOrderPipelineError> {
        let mut state = pipeline_state();
        state.maker_orders = vec![maker_sell("maker-1", 2, 105)];

        let events = pipeline_events(&pipeline_cmd(), state)?;

        assert_eq!(events.len(), 2);
        assert!(events.iter().all(|event| !has_field(event, "trade_id")));

        Ok(())
    }

    #[test]
    fn compute_replayable_events_returns_place_and_match_reject_for_market_no_liquidity()
    -> Result<(), ExecuteImmediateSpotOrderPipelineError> {
        let mut cmd = pipeline_cmd();
        cmd.place.execution = PlaceImmediateOrderExecution::Market { aggressive_price: 100 };
        let state = pipeline_state();

        let events = pipeline_events(&cmd, state)?;

        assert_eq!(events.len(), 3);
        let taker_update = events
            .iter()
            .find(|event| event.is_updated() && event_field(event, "status") == Some("rejected"))
            .expect("expected taker reject event");
        assert_eq!(
            event_field(taker_update, "status_reason"),
            Some(SpotOrderStatusReason::MarketOrderNoLiquidityRejected.as_str())
        );
        assert!(events.iter().all(|event| !has_field(event, "settlement_id")));

        Ok(())
    }

    #[test]
    fn compute_replayable_events_executes_full_pipeline_when_trade_is_matched()
    -> Result<(), ExecuteImmediateSpotOrderPipelineError> {
        let mut state = pipeline_state();
        state.maker_orders = vec![maker_sell("maker-1", 1, 100)];
        let mut cmd = pipeline_cmd();
        cmd.place.size = 1;

        let events = pipeline_events(&cmd, state)?;

        assert!(events.iter().any(|event| has_field(event, "trade_id")));
        assert!(events.iter().any(|event| has_field(event, "settlement_id")));

        let buyer_base_balance = events
            .iter()
            .find(|event| {
                event.is_updated()
                    && event_field(event, "account_id") == Some("trader-1")
                    && event_field(event, "asset_id") == Some("BTC")
            })
            .expect("expected buyer base balance update");
        assert_eq!(event_field(buyer_base_balance, "available"), Some("1"));

        let buyer_quote_balance = events
            .iter()
            .find(|event| {
                event.is_updated()
                    && event_field(event, "account_id") == Some("trader-1")
                    && event_field(event, "asset_id") == Some("USDT")
                    && event_field(event, "frozen") == Some("0")
            })
            .expect("expected buyer quote settlement update");
        assert_eq!(event_field(buyer_quote_balance, "frozen"), Some("0"));

        Ok(())
    }

    #[test]
    fn compute_replayable_events_bubbles_settle_rejections() {
        let mut state = pipeline_state();
        state.maker_orders = vec![maker_sell("maker-1", 1, 100)];
        state.settled_trade_ids = vec!["match-1-1".to_string()];
        let mut cmd = pipeline_cmd();
        cmd.place.size = 1;

        let result = pipeline_events(&cmd, state);

        assert_eq!(
            result,
            Err(ExecuteImmediateSpotOrderPipelineError::Settle(
                SettleSpotTradeError::TradeAlreadySettled
            ))
        );
    }

    #[test]
    fn compute_replayable_events_returns_match_reject_for_crossing_alo()
    -> Result<(), ExecuteImmediateSpotOrderPipelineError> {
        let mut state = pipeline_state();
        state.maker_orders = vec![maker_sell("maker-1", 2, 100)];
        let mut cmd = pipeline_cmd();
        cmd.place.execution = PlaceImmediateOrderExecution::Limit {
            price: 100,
            time_in_force: PlaceOrderTimeInForce::Alo,
        };

        let events = pipeline_events(&cmd, state)?;

        let taker_update = events
            .iter()
            .find(|event| {
                event.is_updated()
                    && event_field(event, "status") == Some(SpotOrderStatus::Rejected.as_str())
            })
            .expect("expected alo reject update");
        assert_eq!(
            event_field(taker_update, "status_reason"),
            Some(SpotOrderStatusReason::BadAloPxRejected.as_str())
        );
        assert!(events.iter().all(|event| !has_field(event, "settlement_id")));

        Ok(())
    }
}
