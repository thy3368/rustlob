use std::collections::{HashMap, HashSet};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{Balance, SpotOrderSide, SpotSettlement, SpotTrade};

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

/// Use case that settles matched spot trades into account balance changes.
///
/// 用例只处理 base/quote 资产交割和 settlement 事实记录；手续费由独立 use case 处理。
#[derive(Debug, Clone, Copy, Default)]
pub struct SettleSpotTradeUseCase;

impl CommandUseCase2 for SettleSpotTradeUseCase {
    type Command = SettleSpotTradeCmd;
    type GivenState = SettleSpotTradeState;
    type Error = SettleSpotTradeError;

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

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let deltas = settlement_deltas(&state.trades, &state.base_asset_id, &state.quote_asset_id)?;
        let mut events = Vec::new();

        for (index, trade) in state.trades.iter().enumerate() {
            let parties = settlement_parties(trade);
            let quote_qty =
                trade.notional_quote().ok_or(SettleSpotTradeError::ArithmeticOverflow)?;
            let settlement = SpotSettlement::new(
                format!("{}-{}", cmd.settlement_batch_id, index + 1),
                trade.trade_id.clone(),
                trade.match_id.clone(),
                parties.buyer_account_id.to_string(),
                parties.seller_account_id.to_string(),
                trade.qty,
                quote_qty,
                trade.price,
            );
            events.push(
                settlement
                    .track_create_event()
                    .map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?,
            );
        }

        for mut balance in state.balances {
            let Some(delta) = deltas.get(&balance_key(&balance.account_id, &balance.asset_id))
            else {
                continue;
            };
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

            events.push(
                balance
                    .track_update_event(|balance| {
                        balance.apply_after(next_available, next_frozen, next_version);
                    })
                    .map_err(|_| SettleSpotTradeError::ArithmeticOverflow)?,
            );
        }

        Ok(events)
    }
}

#[derive(Debug, Clone, Copy)]
struct SettlementParties<'a> {
    buyer_account_id: &'a str,
    seller_account_id: &'a str,
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
        let parties = settlement_parties(trade);
        let quote_qty = trade.notional_quote().ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let buyer_base_delta =
            deltas.entry(balance_key(parties.buyer_account_id, base_asset_id)).or_default();
        buyer_base_delta.available_add = buyer_base_delta
            .available_add
            .checked_add(trade.qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let buyer_quote_delta =
            deltas.entry(balance_key(parties.buyer_account_id, quote_asset_id)).or_default();
        buyer_quote_delta.frozen_sub = buyer_quote_delta
            .frozen_sub
            .checked_add(quote_qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let seller_quote_delta =
            deltas.entry(balance_key(parties.seller_account_id, quote_asset_id)).or_default();
        seller_quote_delta.available_add = seller_quote_delta
            .available_add
            .checked_add(quote_qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;

        let seller_base_delta =
            deltas.entry(balance_key(parties.seller_account_id, base_asset_id)).or_default();
        seller_base_delta.frozen_sub = seller_base_delta
            .frozen_sub
            .checked_add(trade.qty)
            .ok_or(SettleSpotTradeError::ArithmeticOverflow)?;
    }
    Ok(deltas)
}

fn settlement_parties(trade: &SpotTrade) -> SettlementParties<'_> {
    match trade.taker_side {
        SpotOrderSide::Buy => SettlementParties {
            buyer_account_id: trade.taker_account_id.as_str(),
            seller_account_id: trade.maker_account_id.as_str(),
        },
        SpotOrderSide::Sell => SettlementParties {
            buyer_account_id: trade.maker_account_id.as_str(),
            seller_account_id: trade.taker_account_id.as_str(),
        },
    }
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
mod tests {
    use cmd_handler::command_use_case_def2::CommandUseCase2;

    use super::*;

    fn cmd(trade_ids: Vec<&str>) -> SettleSpotTradeCmd {
        SettleSpotTradeCmd {
            party_id: "clearing-house".to_string(),
            settlement_batch_id: "settle-1".to_string(),
            trade_ids: trade_ids.into_iter().map(str::to_string).collect(),
        }
    }

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

    fn balance(account_id: &str, asset_id: &str, available: u64, frozen: u64) -> Balance {
        Balance::new(account_id.to_string(), asset_id.to_string(), available, frozen, 3)
    }

    fn state(trades: Vec<SpotTrade>, balances: Vec<Balance>) -> SettleSpotTradeState {
        SettleSpotTradeState {
            trades,
            base_asset_id: "BTC".to_string(),
            quote_asset_id: "USDT".to_string(),
            balances,
            settled_trade_ids: Vec::new(),
        }
    }

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

    fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }
            std::str::from_utf8(change.new_value_bytes()).ok()
        })
    }

    #[test]
    fn role_is_clearing_house() {
        assert_eq!(SettleSpotTradeUseCase.role(), "ClearingHouse");
    }

    #[test]
    fn pre_check_rejects_empty_trade_ids() {
        assert_eq!(
            SettleSpotTradeUseCase.pre_check_command(&cmd(Vec::new())),
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
            SettleSpotTradeUseCase.validate_against_state(&cmd(vec!["different"]), &state),
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
            SettleSpotTradeUseCase.validate_against_state(&cmd(vec!["trade-1"]), &state),
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
            SettleSpotTradeUseCase.validate_against_state(&cmd(vec!["trade-1"]), &state),
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
            SettleSpotTradeUseCase.validate_against_state(&cmd(vec!["trade-1"]), &state),
            Err(SettleSpotTradeError::InsufficientSellerFrozenBase)
        );
    }

    #[test]
    fn compute_settles_batch_and_aggregates_account_updates() -> Result<(), SettleSpotTradeError> {
        let state = state(
            vec![
                trade("trade-1", SpotOrderSide::Buy, "buyer", "seller-1", 100, 2),
                trade("trade-2", SpotOrderSide::Buy, "buyer", "seller-2", 90, 1),
            ],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 290),
                balance("seller-1", "USDT", 0, 0),
                balance("seller-1", "BTC", 0, 2),
                balance("seller-2", "USDT", 0, 0),
                balance("seller-2", "BTC", 0, 1),
            ],
        );

        let events = SettleSpotTradeUseCase
            .compute_replayable_events(&cmd(vec!["trade-1", "trade-2"]), state)?;

        assert_eq!(events.len(), 8);
        assert!(events[0].is_created());
        assert_eq!(event_field(&events[0], "settlement_id"), Some("settle-1-1"));
        assert_eq!(event_field(&events[0], "quote_qty"), Some("200"));
        assert!(events[1].is_created());
        assert_eq!(event_field(&events[1], "settlement_id"), Some("settle-1-2"));
        assert_eq!(event_field(&events[1], "quote_qty"), Some("90"));

        let buyer_base = balance_event(&events, "buyer", "BTC").unwrap();
        assert_eq!(event_field(buyer_base, "available"), Some("3"));
        assert_eq!(event_field(buyer_base, "frozen"), None);

        let buyer_quote = balance_event(&events, "buyer", "USDT").unwrap();
        assert_eq!(event_field(buyer_quote, "available"), None);
        assert_eq!(event_field(buyer_quote, "frozen"), Some("0"));

        let seller_1_quote = balance_event(&events, "seller-1", "USDT").unwrap();
        assert_eq!(event_field(seller_1_quote, "available"), Some("200"));

        let seller_1_base = balance_event(&events, "seller-1", "BTC").unwrap();
        assert_eq!(event_field(seller_1_base, "frozen"), Some("0"));

        let seller_2_quote = balance_event(&events, "seller-2", "USDT").unwrap();
        assert_eq!(event_field(seller_2_quote, "available"), Some("90"));

        let seller_2_base = balance_event(&events, "seller-2", "BTC").unwrap();
        assert_eq!(event_field(seller_2_base, "frozen"), Some("0"));

        Ok(())
    }

    #[test]
    fn compute_derives_buyer_from_maker_when_taker_sells() -> Result<(), SettleSpotTradeError> {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Sell, "seller", "buyer", 100, 2)],
            vec![
                balance("buyer", "BTC", 0, 0),
                balance("buyer", "USDT", 0, 200),
                balance("seller", "USDT", 0, 0),
                balance("seller", "BTC", 0, 2),
            ],
        );

        let events =
            SettleSpotTradeUseCase.compute_replayable_events(&cmd(vec!["trade-1"]), state)?;

        assert_eq!(event_field(&events[0], "buyer_account_id"), Some("buyer"));
        assert_eq!(event_field(&events[0], "seller_account_id"), Some("seller"));
        assert_eq!(
            event_field(balance_event(&events, "buyer", "USDT").unwrap(), "frozen"),
            Some("0")
        );
        assert_eq!(
            event_field(balance_event(&events, "seller", "USDT").unwrap(), "available"),
            Some("200")
        );

        Ok(())
    }

    #[test]
    fn compute_rejects_notional_overflow() {
        let state = state(
            vec![trade("trade-1", SpotOrderSide::Buy, "buyer", "seller", u64::MAX, 2)],
            vec![balance("buyer", "USDT", 0, u64::MAX), balance("seller", "BTC", 0, 2)],
        );

        assert_eq!(
            SettleSpotTradeUseCase.compute_replayable_events(&cmd(vec!["trade-1"]), state),
            Err(SettleSpotTradeError::ArithmeticOverflow)
        );
    }
}
