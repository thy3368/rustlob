use std::collections::{HashMap, HashSet};

#[cfg(test)]
use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, MiStateMachine};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry::BalanceLedgerCommand;
use crate::entity::{Balance, SpotOrderSide, SpotSettlement, SpotTrade};
use crate::{BalanceLedgerEntry, BalanceLedgerReason};

/// 批量清结算现货成交的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleSpotTradeCmd {
    /// 发起清结算的业务主体。
    pub party_id: String,
    /// 清结算批次 ID，用于稳定生成 settlement id。
    pub settlement_batch_id: String,
    /// 本批次要清结算的 trade id，顺序必须和已加载 trades 一致。
    pub trade_ids: Vec<String>,
}

impl IssuedByParty for SettleSpotTradeCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 清结算现货成交时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleSpotTradeState {
    /// 要清结算的成交事实。
    pub trades: Vec<SpotTrade>,
    /// base 资产 ID。
    pub base_asset_id: String,
    /// quote 资产 ID。
    pub quote_asset_id: String,
    /// 本批次涉及的资产余额快照。
    pub balances: Vec<Balance>,
    /// 已经存在清算记录的 trade id，用于 core 层幂等拒绝。///todo 有点问题
    pub settled_trade_ids: Vec<String>,
}

/// 现货成交清结算可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleSpotTradeError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 清结算批次 ID 不能为空。
    #[error("settlement_batch_id must not be empty")]
    InvalidSettlementBatchId,
    /// 清结算命令必须包含至少一笔 trade。
    #[error("trade_ids must not be empty")]
    EmptyTradeIds,
    /// 命令 trade id 与已加载 trade 不一致。
    #[error("trade ids do not match loaded trades")]
    TradeIdsMismatch,
    /// trade 已经清结算。
    #[error("trade was already settled")]
    TradeAlreadySettled,
    /// 已加载账户缺少买方或卖方账户。
    #[error("settlement account was not found")]
    AccountNotFound,
    /// 买方冻结 quote 不足以完成清结算。
    #[error("buyer frozen quote is insufficient")]
    InsufficientBuyerFrozenQuote,
    /// 卖方冻结 base 不足以完成清结算。
    #[error("seller frozen base is insufficient")]
    InsufficientSellerFrozenBase,
    /// 生成清结算结果时发生整数溢出。
    #[error("arithmetic overflow while deriving settlement result")]
    ArithmeticOverflow,
}

/// 本批次清结算的 typed output。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleSpotTradeChanges {
    /// 本批次创建出的 settlement 事实。
    pub settlements: Vec<SpotSettlement>,
    /// 本批次实际受影响的余额 before/after。
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    /// 本批次对应的余额流水。
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntry>,
}

/// Use case that settles matched spot trades into account balance changes.
///
/// 用例只处理 base/quote 资产交割和 settlement 事实记录；手续费由独立 use case 处理。
#[derive(Debug, Clone, Copy, Default)]
pub struct SettleSpotTradeUseCase;

impl ReplayableChanges for SettleSpotTradeChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(
            self.settlements.len()
                + self.updated_balances.len()
                + self.created_balance_ledger_entries.len(),
        );
        for settlement in &self.settlements {
            events.push(settlement.track_create_event()?);
        }
        for balance in &self.updated_balances {
            events.push(balance.after.track_update_event_from(&balance.before)?);
        }
        for ledger_entry in &self.created_balance_ledger_entries {
            events.push(ledger_entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl CommandUseCase4 for SettleSpotTradeUseCase {
    type Command = SettleSpotTradeCmd;
    type GivenState = SettleSpotTradeState;
    type Error = SettleSpotTradeError;
    type Changes = SettleSpotTradeChanges;

    fn role(&self) -> &'static str {
        "ClearingHouse"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SettleSpotTradeError::InvalidPartyId);
        }
        if cmd.settlement_batch_id.is_empty() {
            return Err(SettleSpotTradeError::InvalidSettlementBatchId);
        }
        if cmd.trade_ids.is_empty() {
            return Err(SettleSpotTradeError::EmptyTradeIds);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        validate_trade_ids_match(cmd, state)?;
        ensure_not_settled(state)?;

        let balances = balance_map(&state.balances);
        let deltas = settlement_deltas(&state.trades, &state.base_asset_id, &state.quote_asset_id)?;
        validate_balances_can_settle(
            &balances,
            &deltas,
            &state.base_asset_id,
            &state.quote_asset_id,
        )
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let deltas = settlement_deltas(&state.trades, &state.base_asset_id, &state.quote_asset_id)?;
        let mut settlements = Vec::new();
        let mut updated_balances = Vec::new();

        for (index, trade) in state.trades.iter().enumerate() {
            let settlement = trade
                .to_settlement(format!("{}-{}", cmd.settlement_batch_id, index + 1))
                .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;
            settlements.push(settlement);
        }

        let balance_ledger_reasons = settlement_balance_ledger_reasons(
            &state.trades,
            &settlements,
            &state.base_asset_id,
            &state.quote_asset_id,
        );

        for mut balance in state.balances {
            let Some(delta) = deltas.get(&balance_key(&balance.account_id, &balance.asset_id))
            else {
                continue;
            };
            let previous_balance = balance.clone();
            let next_available = balance
                .available
                .checked_add(delta.available_add)
                .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;
            let next_frozen = balance
                .frozen
                .checked_sub(delta.frozen_sub)
                .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;
            let next_version =
                balance.version.checked_add(1).ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

            balance.apply_after(next_available, next_frozen, next_version);
            updated_balances.push(UpdatedEntityPair { before: previous_balance, after: balance });
        }

        let mut created_balance_ledger_entries = Vec::with_capacity(updated_balances.len());
        for updated_balance in &updated_balances {
            let balance_id =
                balance_key(&updated_balance.after.account_id, &updated_balance.after.asset_id);
            let reason = balance_ledger_reasons
                .get(&balance_id)
                .cloned()
                .ok_or(SettleSpotTradeError::AccountNotFound)?;
            let balance_command =
                if updated_balance.after.available > updated_balance.before.available {
                    BalanceLedgerCommand::CreditAvailable {
                        balance: updated_balance.before.clone(),
                        amount: updated_balance.after.available - updated_balance.before.available,
                    }
                } else {
                    BalanceLedgerCommand::DebitFrozen {
                        balance: updated_balance.before.clone(),
                        amount: updated_balance.before.frozen - updated_balance.after.frozen,
                    }
                };
            created_balance_ledger_entries.push(
                BalanceLedgerEntry::draft_from_balance(
                    format!(
                        "balance-ledger:{}:{}",
                        cmd.settlement_batch_id,
                        updated_balance.after.entity_id()
                    ),
                    &updated_balance.before,
                    balance_command.clone(),
                    reason,
                )
                .and_then(|draft| draft.compute_changes(&balance_command))
                .map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?
                .updated_entry
                .after,
            );
        }

        Ok(SettleSpotTradeChanges { settlements, updated_balances, created_balance_ledger_entries })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BalanceSettlementDelta {
    available_add: u64,
    frozen_sub: u64,
}

fn validate_trade_ids_match(
    cmd: &SettleSpotTradeCmd,
    state: &SettleSpotTradeState,
) -> Result<(), SettleSpotTradeError> {
    if cmd.trade_ids.len() != state.trades.len() {
        return Err(SettleSpotTradeError::TradeIdsMismatch);
    }
    if cmd.trade_ids.iter().zip(&state.trades).any(|(trade_id, trade)| trade_id != &trade.trade_id)
    {
        return Err(SettleSpotTradeError::TradeIdsMismatch);
    }
    Ok(())
}

fn ensure_not_settled(state: &SettleSpotTradeState) -> Result<(), SettleSpotTradeError> {
    let settled: HashSet<&str> = state.settled_trade_ids.iter().map(String::as_str).collect();
    if state.trades.iter().any(|trade| settled.contains(trade.trade_id.as_str())) {
        return Err(SettleSpotTradeError::TradeAlreadySettled);
    }
    Ok(())
}

fn balance_map(balances: &[Balance]) -> HashMap<String, &Balance> {
    balances
        .iter()
        .map(|balance| (balance_key(&balance.account_id, &balance.asset_id), balance))
        .collect()
}

fn settlement_deltas(
    trades: &[SpotTrade],
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<HashMap<String, BalanceSettlementDelta>, SettleSpotTradeError> {
    let mut deltas: HashMap<String, BalanceSettlementDelta> = HashMap::new();
    for trade in trades {
        let quote_qty = trade.notional_quote().ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let buyer_base_delta =
            deltas.entry(balance_key(trade.buyer_account_id(), base_asset_id)).or_default();
        buyer_base_delta.available_add = buyer_base_delta
            .available_add
            .checked_add(trade.qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let buyer_quote_delta =
            deltas.entry(balance_key(trade.buyer_account_id(), quote_asset_id)).or_default();
        buyer_quote_delta.frozen_sub = buyer_quote_delta
            .frozen_sub
            .checked_add(quote_qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let seller_quote_delta =
            deltas.entry(balance_key(trade.seller_account_id(), quote_asset_id)).or_default();
        seller_quote_delta.available_add = seller_quote_delta
            .available_add
            .checked_add(quote_qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let seller_base_delta =
            deltas.entry(balance_key(trade.seller_account_id(), base_asset_id)).or_default();
        seller_base_delta.frozen_sub = seller_base_delta
            .frozen_sub
            .checked_add(trade.qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;
    }
    Ok(deltas)
}

fn settlement_balance_ledger_reasons(
    trades: &[SpotTrade],
    settlements: &[SpotSettlement],
    base_asset_id: &str,
    quote_asset_id: &str,
) -> HashMap<String, BalanceLedgerReason> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum BalanceLedgerKind {
        BuyerReceiveBase,
        BuyerReleaseFrozenQuote,
        SellerReceiveQuote,
        SellerReleaseFrozenBase,
    }

    #[derive(Debug, Clone, Default)]
    struct BalanceLedgerRefBatch {
        trade_ids: Vec<String>,
        settlement_ids: Vec<String>,
    }

    let mut refs: HashMap<(String, BalanceLedgerKind), BalanceLedgerRefBatch> = HashMap::new();

    for (trade, settlement) in trades.iter().zip(settlements) {
        let keys = [
            (
                balance_key(trade.buyer_account_id(), base_asset_id),
                BalanceLedgerKind::BuyerReceiveBase,
            ),
            (
                balance_key(trade.buyer_account_id(), quote_asset_id),
                BalanceLedgerKind::BuyerReleaseFrozenQuote,
            ),
            (
                balance_key(trade.seller_account_id(), quote_asset_id),
                BalanceLedgerKind::SellerReceiveQuote,
            ),
            (
                balance_key(trade.seller_account_id(), base_asset_id),
                BalanceLedgerKind::SellerReleaseFrozenBase,
            ),
        ];

        for (balance_id, kind) in keys {
            let batch = refs.entry((balance_id, kind)).or_default();
            batch.trade_ids.push(trade.trade_id.clone());
            batch.settlement_ids.push(settlement.settlement_id.clone());
        }
    }

    refs.into_iter()
        .map(|((balance_id, kind), batch)| {
            let reason = match kind {
                BalanceLedgerKind::BuyerReceiveBase => {
                    BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                        trade_ids: batch.trade_ids,
                        settlement_ids: batch.settlement_ids,
                    }
                }
                BalanceLedgerKind::BuyerReleaseFrozenQuote => {
                    BalanceLedgerReason::SettleSpotTradeBuyerReleaseFrozenQuote {
                        trade_ids: batch.trade_ids,
                        settlement_ids: batch.settlement_ids,
                    }
                }
                BalanceLedgerKind::SellerReceiveQuote => {
                    BalanceLedgerReason::SettleSpotTradeSellerReceiveQuote {
                        trade_ids: batch.trade_ids,
                        settlement_ids: batch.settlement_ids,
                    }
                }
                BalanceLedgerKind::SellerReleaseFrozenBase => {
                    BalanceLedgerReason::SettleSpotTradeSellerReleaseFrozenBase {
                        trade_ids: batch.trade_ids,
                        settlement_ids: batch.settlement_ids,
                    }
                }
            };
            (balance_id, reason)
        })
        .collect()
}

fn validate_balances_can_settle(
    balances: &HashMap<String, &Balance>,
    deltas: &HashMap<String, BalanceSettlementDelta>,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), SettleSpotTradeError> {
    for (balance_id, delta) in deltas {
        let balance = balances.get(balance_id).ok_or(SettleSpotTradeError::AccountNotFound)?;
        if balance.frozen < delta.frozen_sub && delta.frozen_sub > 0 {
            if balance.asset_id == quote_asset_id {
                return Err(SettleSpotTradeError::InsufficientBuyerFrozenQuote);
            }
            if balance.asset_id == base_asset_id {
                return Err(SettleSpotTradeError::InsufficientSellerFrozenBase);
            }
            return Err(SettleSpotTradeError::AccountNotFound);
        }
    }
    Ok(())
}

fn balance_key(account_id: &str, asset_id: &str) -> String {
    format!("{account_id}:{asset_id}")
}

#[cfg(test)]
fn cmd(trade_ids: Vec<&str>) -> SettleSpotTradeCmd {
    SettleSpotTradeCmd {
        party_id: "clearing-house".to_string(),
        settlement_batch_id: "settle-1".to_string(),
        trade_ids: trade_ids.into_iter().map(str::to_string).collect(),
    }
}

#[cfg(test)]
fn trade(
    trade_id: &str,
    taker_side: SpotOrderSide,
    taker_account_id: &str,
    maker_account_id: &str,
    price: u64,
    qty: u64,
) -> SpotTrade {
    SpotTrade::new(
        trade_id.to_string(),
        "match-1".to_string(),
        10_001,
        "BTCUSDT".to_string(),
        format!("{trade_id}-taker"),
        format!("{trade_id}-maker"),
        taker_account_id.to_string(),
        maker_account_id.to_string(),
        taker_side,
        price,
        qty,
    )
}

#[cfg(test)]
fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
    Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 3)
}

#[cfg(test)]
fn state(trades: Vec<SpotTrade>, balances: Vec<Balance>) -> SettleSpotTradeState {
    SettleSpotTradeState {
        trades,
        base_asset_id: "BTC".to_string(),
        quote_asset_id: "USDT".to_string(),
        balances,
        settled_trade_ids: Vec::new(),
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
fn balance_event<'a>(
    events: &'a [EntityReplayableEvent],
    account_id: &str,
    asset_id: &str,
) -> Option<&'a EntityReplayableEvent> {
    events.iter().find(|event| {
        event.is_updated()
            && event_field(event, "account_id") == Some(account_id)
            && event_field(event, "asset_id") == Some(asset_id)
    })
}

#[cfg(test)]
fn ledger_event<'a>(
    events: &'a [EntityReplayableEvent],
    account_id: &str,
    asset_id: &str,
) -> Option<&'a EntityReplayableEvent> {
    events.iter().find(|event| {
        event.is_created()
            && event_field(event, "account_id") == Some(account_id)
            && event_field(event, "asset_id") == Some(asset_id)
            && event_field(event, "entry_id").is_some()
    })
}

#[cfg(test)]
fn assert_settlement_event(
    event: &EntityReplayableEvent,
    expected_settlement_id: &str,
    expected_trade_id: &str,
    expected_buyer_account_id: &str,
    expected_seller_account_id: &str,
    expected_base_qty: u64,
    expected_quote_qty: u64,
    expected_price: u64,
) {
    assert!(event.is_created());
    assert_eq!(event_field(event, "settlement_id"), Some(expected_settlement_id));
    assert_eq!(event_field(event, "trade_id"), Some(expected_trade_id));
    assert_eq!(event_field(event, "match_id"), Some("match-1"));
    assert_eq!(event_field(event, "buyer_account_id"), Some(expected_buyer_account_id));
    assert_eq!(event_field(event, "seller_account_id"), Some(expected_seller_account_id));
    assert_eq!(event_field_u64(event, "base_qty"), Some(expected_base_qty));
    assert_eq!(event_field_u64(event, "quote_qty"), Some(expected_quote_qty));
    assert_eq!(event_field_u64(event, "price"), Some(expected_price));
}

#[cfg(test)]
fn assert_balance_update_event(
    event: &EntityReplayableEvent,
    expected_account_id: &str,
    expected_asset_id: &str,
    expected_available: Option<u64>,
    expected_frozen: Option<u64>,
    expected_old_version: u64,
    expected_new_version: u64,
) {
    assert!(event.is_updated());
    assert_eq!(event.old_version, expected_old_version);
    assert_eq!(event.new_version, expected_new_version);
    assert_eq!(event_field(event, "account_id"), Some(expected_account_id));
    assert_eq!(event_field(event, "asset_id"), Some(expected_asset_id));
    assert_eq!(event_field_u64(event, "available"), expected_available);
    assert_eq!(event_field_u64(event, "frozen"), expected_frozen);
}

#[cfg(test)]
fn assert_balance_ledger_event(
    event: &EntityReplayableEvent,
    expected_account_id: &str,
    expected_asset_id: &str,
    expected_reason: &str,
    expected_trade_ids: &str,
    expected_settlement_ids: &str,
) {
    assert!(event.is_created());
    assert_eq!(event_field(event, "account_id"), Some(expected_account_id));
    assert_eq!(event_field(event, "asset_id"), Some(expected_asset_id));
    assert_eq!(event_field(event, "reason"), Some(expected_reason));
    assert_eq!(event_field(event, "reason_trade_ids"), Some(expected_trade_ids));
    assert_eq!(event_field(event, "reason_settlement_ids"), Some(expected_settlement_ids));
}

#[cfg(test)]
mod compute_replayable_events_happy_path;

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;

    #[test]
    fn role_is_clearing_house() {
        assert_eq!(CommandUseCase4::role(&SettleSpotTradeUseCase), "ClearingHouse");
    }

    #[test]
    fn pre_check_rejects_empty_trade_ids() {
        assert_eq!(
            CommandUseCase4::pre_check_command(&SettleSpotTradeUseCase, &cmd(Vec::new())),
            Err(SettleSpotTradeError::EmptyTradeIds)
        );
    }

    #[test]
    fn validate_rejects_trade_ids_mismatch() {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![balance("buyer", "USDT", 0, 200), balance("seller", "BTC", 0, 2)],
        );

        assert_eq!(
            CommandUseCase4::validate_against_state(
                &SettleSpotTradeUseCase,
                &cmd(vec!["different"]),
                &state,
            ),
            Err(SettleSpotTradeError::TradeIdsMismatch)
        );
    }

    #[test]
    fn validate_rejects_already_settled_trade() {
        let mut state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![balance("buyer", "USDT", 0, 200), balance("seller", "BTC", 0, 2)],
        );
        state.settled_trade_ids = vec!["trade-1".to_string()];

        assert_eq!(
            CommandUseCase4::validate_against_state(
                &SettleSpotTradeUseCase,
                &cmd(vec!["trade-1"]),
                &state,
            ),
            Err(SettleSpotTradeError::TradeAlreadySettled)
        );
    }

    #[test]
    fn validate_rejects_insufficient_buyer_frozen_quote() {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 199),
                balance("seller", "USDT", 0, 0),
                balance("seller", "BTC", 0, 2),
            ],
        );

        assert_eq!(
            CommandUseCase4::validate_against_state(
                &SettleSpotTradeUseCase,
                &cmd(vec!["trade-1"]),
                &state,
            ),
            Err(SettleSpotTradeError::InsufficientBuyerFrozenQuote)
        );
    }

    #[test]
    fn validate_rejects_insufficient_seller_frozen_base() {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 200),
                balance("seller", "USDT", 0, 0),
                balance("seller", "BTC", 0, 1),
            ],
        );

        assert_eq!(
            CommandUseCase4::validate_against_state(
                &SettleSpotTradeUseCase,
                &cmd(vec!["trade-1"]),
                &state,
            ),
            Err(SettleSpotTradeError::InsufficientSellerFrozenBase)
        );
    }

    #[test]
    fn compute_changes_keeps_changes_and_events_consistent() -> Result<(), SettleSpotTradeError> {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 200),
                balance("seller", "USDT", 0, 0),
                balance("seller", "BTC", 0, 2),
            ],
        );

        let result = cmd_handler::command_use_case_def2::CommandUseCase4::compute_changes(
            &SettleSpotTradeUseCase,
            &cmd(vec!["trade-1"]),
            state,
        )?;
        let events =
            result.to_replayable_events().map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?;

        assert_eq!(result.settlements.len(), 1);
        assert_eq!(result.settlements[0].settlement_id, "settle-1-1");
        assert!(result.updated_balances.iter().any(|balance| {
            balance.after.account_id == "buyer"
                && balance.after.asset_id == "BTC"
                && balance.after.available == 2
        }));
        assert_eq!(result.created_balance_ledger_entries.len(), 4);
        assert!(result.created_balance_ledger_entries.iter().any(|entry| {
            entry.account_id == "buyer"
                && entry.asset_id == "BTC"
                && entry.reason
                    == BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                        trade_ids: vec!["trade-1".to_string()],
                        settlement_ids: vec!["settle-1-1".to_string()],
                    }
        }));
        assert!(
            events.iter().any(|event| event_field(event, "settlement_id") == Some("settle-1-1"))
        );
        assert!(events.iter().any(|event| {
            event_field(event, "reason") == Some("settle_spot_trade_buyer_receive_base")
        }));

        Ok(())
    }

    #[test]
    fn compute_rejects_notional_overflow() {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", u64::MAX, 2)],
            vec![balance("buyer", "USDT", 0, u64::MAX), balance("seller", "BTC", 0, 2)],
        );

        assert_eq!(
            CommandUseCase4::compute_changes(&SettleSpotTradeUseCase, &cmd(vec!["trade-1"]), state,),
            Err(SettleSpotTradeError::ArithmeticOverflow)
        );
    }
}
