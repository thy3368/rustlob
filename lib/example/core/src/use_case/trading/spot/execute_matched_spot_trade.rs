#[cfg(test)]
use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges,
};
use thiserror::Error;

use super::{
    MatchSpotOrderChanges, MatchSpotOrderCmd, MatchSpotOrderError, MatchSpotOrderState,
    MatchSpotOrderUseCase, SettleSpotTradeChanges, SettleSpotTradeCmd, SettleSpotTradeError,
    SettleSpotTradeState, SettleSpotTradeUseCase,
};
use crate::entity::{AssetReservation, Balance, SpotOrder, SpotOrderExecution, SpotOrderSide};

/// 执行一次“撮合后立即清结算”的现货成交命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteMatchedSpotTradeCmd {
    /// 发起撮合结算的业务账户。
    pub party_id: String,
    /// 本次撮合作为主动吃单方的 taker 订单 ID。
    pub taker_order_id: String,
    /// 本次撮合批次 ID。
    pub match_id: String,
    /// 本次清结算批次 ID。
    pub settlement_batch_id: String,
}

impl IssuedByParty for ExecuteMatchedSpotTradeCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 执行现货撮合并立即清结算时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteMatchedSpotTradeState {
    /// 本次作为主动吃单方的订单。
    pub taker_order: SpotOrder,
    /// 已按撮合优先级排好的被动挂单。
    pub maker_orders: Vec<SpotOrder>,
    /// 本批次参与交割的余额快照。
    pub settlement_balances: Vec<Balance>,
    /// 已经存在清算记录的 trade id，用于 core 层幂等拒绝。
    pub settled_trade_ids: Vec<String>,
    /// base 资产 ID。
    pub base_asset_id: String,
    /// quote 资产 ID。
    pub quote_asset_id: String,
}

/// 现货撮合并立即清结算可能出现的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ExecuteMatchedSpotTradeError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// taker order id 不能为空。
    #[error("taker_order_id must not be empty")]
    InvalidTakerOrderId,
    /// match id 不能为空。
    #[error("match_id must not be empty")]
    InvalidMatchId,
    /// settlement batch id 不能为空。
    #[error("settlement_batch_id must not be empty")]
    InvalidSettlementBatchId,
    /// 撮合阶段失败。
    #[error(transparent)]
    Match(#[from] MatchSpotOrderError),
    /// 清结算阶段失败。
    #[error(transparent)]
    Settle(#[from] SettleSpotTradeError),
}

/// 现货撮合并立即清结算的 typed output。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteMatchedSpotTradeChanges {
    /// 撮合阶段结果，始终存在。
    pub match_changes: MatchSpotOrderChanges,
    /// 清结算阶段结果；没有成交时为空。
    pub settle_changes: Option<SettleSpotTradeChanges>,
}

/// 复合 use case：给定 taker / maker / balance 快照后，先撮合再立即清结算。
#[derive(Debug, Clone, Copy, Default)]
pub struct ExecuteMatchedSpotTradeUseCase;

impl ReplayableChanges for ExecuteMatchedSpotTradeChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events = self.match_changes.to_replayable_events()?;
        if let Some(settle_changes) = &self.settle_changes {
            events.extend(settle_changes.to_replayable_events()?);
        }
        Ok(events)
    }
}

impl CommandUseCase4 for ExecuteMatchedSpotTradeUseCase {
    type Command = ExecuteMatchedSpotTradeCmd;
    type GivenState = ExecuteMatchedSpotTradeState;
    type Error = ExecuteMatchedSpotTradeError;
    type Changes = ExecuteMatchedSpotTradeChanges;

    fn role(&self) -> &'static str {
        "MatchingEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ExecuteMatchedSpotTradeError::InvalidPartyId);
        }
        if cmd.taker_order_id.is_empty() {
            return Err(ExecuteMatchedSpotTradeError::InvalidTakerOrderId);
        }
        if cmd.match_id.is_empty() {
            return Err(ExecuteMatchedSpotTradeError::InvalidMatchId);
        }
        if cmd.settlement_batch_id.is_empty() {
            return Err(ExecuteMatchedSpotTradeError::InvalidSettlementBatchId);
        }

        MatchSpotOrderUseCase.pre_check_command(&match_cmd_from_execute_cmd(cmd))?;
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        MatchSpotOrderUseCase.validate_against_state(
            &match_cmd_from_execute_cmd(cmd),
            &match_state_from_execute_state(state),
        )?;
        Ok(())
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let match_cmd = match_cmd_from_execute_cmd(cmd);
        let match_state = MatchSpotOrderState {
            taker_order: state.taker_order,
            maker_orders: state.maker_orders,
        };
        let match_changes = MatchSpotOrderUseCase.compute_changes(&match_cmd, match_state)?;
        let trades = match_changes.trades.clone();

        if trades.is_empty() {
            return Ok(ExecuteMatchedSpotTradeChanges { match_changes, settle_changes: None });
        }

        let settle_cmd = SettleSpotTradeCmd {
            party_id: cmd.party_id.clone(),
            settlement_batch_id: cmd.settlement_batch_id.clone(),
            trade_ids: trades.iter().map(|trade| trade.trade_id.clone()).collect(),
        };
        let reservations = settlement_reservations_from_match(
            &match_changes,
            state.base_asset_id.as_str(),
            state.quote_asset_id.as_str(),
        )
        .map_err(ExecuteMatchedSpotTradeError::Settle)?;
        let settle_state = SettleSpotTradeState {
            trades,
            base_asset_id: state.base_asset_id,
            quote_asset_id: state.quote_asset_id,
            balances: state.settlement_balances,
            reservations,
            settled_trade_ids: state.settled_trade_ids,
        };

        SettleSpotTradeUseCase.pre_check_command(&settle_cmd)?;
        SettleSpotTradeUseCase.validate_against_state(&settle_cmd, &settle_state)?;
        let settle_changes = SettleSpotTradeUseCase.compute_changes(&settle_cmd, settle_state)?;

        Ok(ExecuteMatchedSpotTradeChanges { match_changes, settle_changes: Some(settle_changes) })
    }
}

fn match_cmd_from_execute_cmd(cmd: &ExecuteMatchedSpotTradeCmd) -> MatchSpotOrderCmd {
    MatchSpotOrderCmd {
        party_id: cmd.party_id.clone(),
        taker_order_id: cmd.taker_order_id.clone(),
        match_id: cmd.match_id.clone(),
    }
}

fn match_state_from_execute_state(state: &ExecuteMatchedSpotTradeState) -> MatchSpotOrderState {
    MatchSpotOrderState {
        taker_order: state.taker_order.clone(),
        maker_orders: state.maker_orders.clone(),
    }
}

fn settlement_reservations_from_match(
    match_changes: &MatchSpotOrderChanges,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<Vec<AssetReservation>, SettleSpotTradeError> {
    let mut reservations = vec![
        match_changes
            .updated_taker_order
            .before
            .to_reservation(base_asset_id, quote_asset_id)
            .map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?,
    ];
    for maker in &match_changes.updated_maker_orders {
        reservations.push(
            maker
                .before
                .to_reservation(base_asset_id, quote_asset_id)
                .map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?,
        );
    }
    Ok(reservations)
}

#[cfg(test)]
fn execute_cmd() -> ExecuteMatchedSpotTradeCmd {
    ExecuteMatchedSpotTradeCmd {
        party_id: "buyer".to_string(),
        taker_order_id: "taker-1".to_string(),
        match_id: "match-1".to_string(),
        settlement_batch_id: "settle-1".to_string(),
    }
}

#[cfg(test)]
fn build_limit_order(
    order_id: &str,
    account_id: &str,
    side: SpotOrderSide,
    price: u64,
    time_in_force: crate::entity::SpotOrderTimeInForce,
    qty: u64,
) -> SpotOrder {
    let (reserved_base, reserved_quote) = match side {
        SpotOrderSide::Buy => (0, qty * price),
        SpotOrderSide::Sell => (qty, 0),
    };
    SpotOrder::new(
        order_id.to_string(),
        10_001,
        Some(42),
        account_id.to_string(),
        "BTCUSDT".to_string(),
        side,
        SpotOrderExecution::Limit { price },
        time_in_force,
        qty,
        reserved_base,
        reserved_quote,
        None,
    )
}

#[cfg(test)]
fn taker_buy_limit(qty: u64, price: u64) -> SpotOrder {
    build_limit_order(
        "taker-1",
        "buyer",
        SpotOrderSide::Buy,
        price,
        crate::entity::SpotOrderTimeInForce::Gtc,
        qty,
    )
}

#[cfg(test)]
fn taker_buy_alo(qty: u64, price: u64) -> SpotOrder {
    build_limit_order(
        "taker-1",
        "buyer",
        SpotOrderSide::Buy,
        price,
        crate::entity::SpotOrderTimeInForce::Alo,
        qty,
    )
}

#[cfg(test)]
fn maker_sell(order_id: &str, account_id: &str, qty: u64, price: u64) -> SpotOrder {
    build_limit_order(
        order_id,
        account_id,
        SpotOrderSide::Sell,
        price,
        crate::entity::SpotOrderTimeInForce::Gtc,
        qty,
    )
}

#[cfg(test)]
fn maker_buy(order_id: &str, account_id: &str, qty: u64, price: u64) -> SpotOrder {
    build_limit_order(
        order_id,
        account_id,
        SpotOrderSide::Buy,
        price,
        crate::entity::SpotOrderTimeInForce::Gtc,
        qty,
    )
}

#[cfg(test)]
fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
    Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 3)
}

#[cfg(test)]
fn execute_state(
    taker_order: SpotOrder,
    maker_orders: Vec<SpotOrder>,
    settlement_balances: Vec<Balance>,
) -> ExecuteMatchedSpotTradeState {
    ExecuteMatchedSpotTradeState {
        taker_order,
        maker_orders,
        settlement_balances,
        settled_trade_ids: Vec::new(),
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
    }
}

#[cfg(test)]
fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
    event.field_changes.iter().find_map(|change| {
        if change.field_name_as_str().ok() != Some(field_name) {
            return None;
        }
        std::str::from_utf8(change.new_value_bytes()).ok()
    })
}

#[cfg(test)]
fn event_field_u64(event: &EntityReplayableEvent, field_name: &str) -> Option<u64> {
    event_field(event, field_name)?.parse::<u64>().ok()
}

#[cfg(test)]
fn assert_trade_event(
    event: &EntityReplayableEvent,
    expected_trade_id: &str,
    expected_maker_order_id: &str,
    expected_price: u64,
    expected_qty: u64,
) {
    assert!(event.is_created());
    assert_eq!(event_field(event, "trade_id"), Some(expected_trade_id));
    assert_eq!(event_field(event, "maker_order_id"), Some(expected_maker_order_id));
    assert_eq!(event_field_u64(event, "price"), Some(expected_price));
    assert_eq!(event_field_u64(event, "qty"), Some(expected_qty));
}

#[cfg(test)]
mod compute_replayable_events_happy_path;

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::SpotOrderStatus;

    #[test]
    fn role_is_matching_engine() {
        assert_eq!(ExecuteMatchedSpotTradeUseCase.role(), "MatchingEngine");
    }

    #[test]
    fn pre_check_rejects_empty_command_fields() {
        let mut cmd = execute_cmd();
        cmd.party_id.clear();
        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.pre_check_command(&cmd),
            Err(ExecuteMatchedSpotTradeError::InvalidPartyId)
        );

        let mut cmd = execute_cmd();
        cmd.taker_order_id.clear();
        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.pre_check_command(&cmd),
            Err(ExecuteMatchedSpotTradeError::InvalidTakerOrderId)
        );

        let mut cmd = execute_cmd();
        cmd.match_id.clear();
        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.pre_check_command(&cmd),
            Err(ExecuteMatchedSpotTradeError::InvalidMatchId)
        );

        let mut cmd = execute_cmd();
        cmd.settlement_batch_id.clear();
        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.pre_check_command(&cmd),
            Err(ExecuteMatchedSpotTradeError::InvalidSettlementBatchId)
        );
    }

    #[test]
    fn validate_rejects_when_taker_order_does_not_match_command() {
        let mut state = execute_state(
            taker_buy_limit(2, 100),
            vec![maker_sell("maker-1", "seller-1", 2, 100)],
            vec![],
        );
        state.taker_order.order_id = "other-taker".to_string();

        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.validate_against_state(&execute_cmd(), &state),
            Err(ExecuteMatchedSpotTradeError::Match(MatchSpotOrderError::TakerOrderMismatch))
        );
    }

    #[test]
    fn validate_reuses_match_validation_for_invalid_maker() {
        let state = execute_state(
            taker_buy_limit(2, 100),
            vec![maker_buy("maker-1", "buyer-2", 2, 100)],
            vec![],
        );

        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.validate_against_state(&execute_cmd(), &state),
            Err(ExecuteMatchedSpotTradeError::Match(MatchSpotOrderError::SameSideMaker))
        );
    }

    #[test]
    fn compute_changes_returns_settle_error_when_balances_are_insufficient() {
        let state = execute_state(
            taker_buy_limit(2, 100),
            vec![maker_sell("maker-1", "seller-1", 2, 100)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 199),
                balance("seller-1", "USDT", 0, 0),
                balance("seller-1", "BTC", 0, 2),
            ],
        );

        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.compute_changes(&execute_cmd(), state),
            Err(ExecuteMatchedSpotTradeError::Settle(
                SettleSpotTradeError::InsufficientBuyerFrozenQuote,
            ))
        );
    }

    #[test]
    fn compute_changes_returns_settle_error_when_trade_was_already_settled() {
        let mut state = execute_state(
            taker_buy_limit(2, 100),
            vec![maker_sell("maker-1", "seller-1", 2, 100)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 200),
                balance("seller-1", "USDT", 0, 0),
                balance("seller-1", "BTC", 0, 2),
            ],
        );
        state.settled_trade_ids = vec!["match-1-1".to_string()];

        assert_eq!(
            ExecuteMatchedSpotTradeUseCase.compute_changes(&execute_cmd(), state),
            Err(ExecuteMatchedSpotTradeError::Settle(SettleSpotTradeError::TradeAlreadySettled))
        );
    }

    #[test]
    fn compute_changes_keeps_match_and_settle_changes_consistent()
    -> Result<(), ExecuteMatchedSpotTradeError> {
        let state = execute_state(
            taker_buy_limit(2, 100),
            vec![maker_sell("maker-1", "seller-1", 2, 100)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 200),
                balance("seller-1", "USDT", 0, 0),
                balance("seller-1", "BTC", 0, 2),
            ],
        );

        let changes = ExecuteMatchedSpotTradeUseCase.compute_changes(&execute_cmd(), state)?;
        let events = changes.to_replayable_events().map_err(|_| {
            ExecuteMatchedSpotTradeError::Settle(SettleSpotTradeError::ArithmeticOverflow)
        })?;

        assert_eq!(changes.match_changes.trades.len(), 1);
        assert_eq!(changes.match_changes.trades[0].trade_id, "match-1-1");
        assert_eq!(changes.match_changes.updated_taker_order.after.status, SpotOrderStatus::Filled);
        let settle_changes = changes.settle_changes.as_ref().expect("expected settlement changes");
        assert_eq!(settle_changes.settlements.len(), 1);
        assert_eq!(settle_changes.settlements[0].settlement_id, "settle-1-1");
        assert!(events.iter().any(|event| event_field(event, "trade_id") == Some("match-1-1")));
        assert!(
            events.iter().any(|event| event_field(event, "settlement_id") == Some("settle-1-1"))
        );

        Ok(())
    }
}
