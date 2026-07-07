use std::collections::{BTreeMap, HashMap, HashSet};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    Balance, HyperliquidPerpPosition, HyperliquidPerpPositionSide, HyperliquidPerpTrade,
    SettlementTransferVoucher, required_position_margin,
};

const FEE_BPS_DENOMINATOR: u64 = 10_000;

/// 清结算 Hyperliquid perp 成交可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpTradeError {
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
    /// 已加载仓位缺少对应账户的合约仓位槽位。
    #[error("position slot was not found")]
    PositionNotFound,
    /// 已加载仓位账户、asset 或 symbol 与成交不一致。
    #[error("position does not match trade account or market")]
    PositionMismatch,
    /// 仓位状态、数量或保证金不一致。
    #[error("position state is inconsistent")]
    InconsistentPositionState,
    /// 仓位杠杆必须至少为 1。
    #[error("position leverage must be greater than or equal to one")]
    InvalidLeverage,
    /// 已加载账户缺少保证金币种余额。
    #[error("margin balance was not found")]
    MarginBalanceNotFound,
    /// 保证金余额资产必须等于 state.margin_asset_id。
    #[error("margin balance asset does not match state margin asset")]
    InvalidMarginBalance,
    /// 可用保证金余额不足以支付手续费、亏损或追加仓位保证金。
    #[error("insufficient available margin balance")]
    InsufficientAvailableMargin,
    /// 冻结保证金余额不足以释放仓位保证金。
    #[error("insufficient frozen margin balance")]
    InsufficientFrozenMargin,
    /// 生成清结算结果时发生整数溢出。
    #[error("arithmetic overflow while deriving settlement result")]
    ArithmeticOverflow,
}

/// 批量清结算 Hyperliquid perp 成交的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeCmd {
    /// 发起清结算的业务主体。
    pub party_id: String,
    /// 清结算批次 ID，用于稳定生成 settlement id。
    pub settlement_batch_id: String,
    /// 本批次要清结算的 trade id，顺序必须和已加载 trades 一致。
    pub trade_ids: Vec<String>,
}

impl IssuedByParty for SettleHyperliquidPerpTradeCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 清结算 Hyperliquid perp 成交时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeState {
    /// 要清结算的成交事实。
    pub trades: Vec<HyperliquidPerpTrade>,
    /// 本批次涉及账户在成交合约上的仓位槽位；空仓但可创建仓位时传 `version == 0` 的 flat 槽位。
    pub positions: Vec<HyperliquidPerpPosition>,
    /// 本批次涉及账户的 Cross 保证金币种余额。
    pub margin_balances: Vec<Balance>,
    /// Cross 保证金币种，例如 `USDC`。
    pub margin_asset_id: String,
    /// 平台手续费账户 ID，用于 perp settlement voucher fee legs。
    pub fee_account_id: String,
    /// taker 手续费 bps，分母为 10_000。
    pub taker_fee_bps: u64,
    /// maker 手续费 bps，分母为 10_000。
    pub maker_fee_bps: u64,
    /// 已经完成清结算的 trade id，用于 core 层幂等拒绝。
    pub settled_trade_ids: Vec<String>,
}

/// Use case that settles matched Hyperliquid perp trades into positions and Cross margin balances.
///
/// 用例处理成交落仓位、Cross 保证金重算、已实现 PnL 和手续费；资金费、强平和 ADL
/// 由独立 use case 处理。
#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpTradeUseCase;

/// 批量清结算成交后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeChanges {
    /// 本批次新创建的 perp 清结算转账凭证。
    pub created_vouchers: Vec<SettlementTransferVoucher>,
    /// 本批次仓位槽位的 before/after，顺序与输入状态稳定对齐。
    pub changed_positions: Vec<UpdatedEntityPair<HyperliquidPerpPosition>>,
    /// 本批次实际受影响保证金余额的 before/after，顺序与输入状态稳定对齐。
    pub changed_margin_balances: Vec<UpdatedEntityPair<Balance>>,
}

impl ReplayableChanges for SettleHyperliquidPerpTradeChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::new();
        for voucher in &self.created_vouchers {
            events.push(voucher.track_create_event()?);
        }
        for position in &self.changed_positions {
            if position.before.version == 0 && !position.after.is_flat() {
                events.push(position.after.track_create_event()?);
            } else if position.before != position.after {
                events.push(position.after.track_update_event_from(&position.before)?);
            }
        }
        for balance in &self.changed_margin_balances {
            events.push(balance.after.track_update_event_from(&balance.before)?);
        }
        Ok(events)
    }
}

impl CommandUseCase4 for SettleHyperliquidPerpTradeUseCase {
    type Command = SettleHyperliquidPerpTradeCmd;
    type GivenState = SettleHyperliquidPerpTradeState;
    type Error = SettleHyperliquidPerpTradeError;
    type Changes = SettleHyperliquidPerpTradeChanges;

    fn role(&self) -> &'static str {
        "ClearingHouse"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SettleHyperliquidPerpTradeError::InvalidPartyId);
        }
        if cmd.settlement_batch_id.is_empty() {
            return Err(SettleHyperliquidPerpTradeError::InvalidSettlementBatchId);
        }
        if cmd.trade_ids.is_empty() {
            return Err(SettleHyperliquidPerpTradeError::EmptyTradeIds);
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
        let outcome = derive_settlement_outcome(cmd, state)?;
        validate_balances_can_apply(state, &outcome.balance_deltas)
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let outcome = derive_settlement_outcome(cmd, &state)?;
        validate_balances_can_apply(&state, &outcome.balance_deltas)?;

        let changed_positions = state
            .positions
            .iter()
            .filter_map(|position| {
                let key =
                    position_key(&position.account_id, position.asset, position.symbol.as_str());
                outcome
                    .positions
                    .get(&key)
                    .cloned()
                    .map(|after| UpdatedEntityPair { before: position.clone(), after })
            })
            .collect::<Vec<_>>();
        let mut changed_margin_balances = Vec::new();
        for balance in &state.margin_balances {
            let key = balance_key(&balance.account_id, &balance.asset_id);
            let Some(delta) = outcome.balance_deltas.get(&key) else {
                continue;
            };
            let next_available = apply_signed_delta(balance.available, delta.available_delta)?;
            let next_frozen = apply_signed_delta(balance.frozen, delta.frozen_delta)?;
            let next_version = balance
                .version
                .checked_add(1)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let mut next_balance = balance.clone();
            next_balance.apply_after(next_available, next_frozen, next_version);
            changed_margin_balances
                .push(UpdatedEntityPair { before: balance.clone(), after: next_balance });
        }

        Ok(SettleHyperliquidPerpTradeChanges {
            created_vouchers: outcome.vouchers,
            changed_positions,
            changed_margin_balances,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct AccountRole {
    side: HyperliquidPerpPositionSide,
    is_taker: bool,
}

#[derive(Debug, Clone, Copy, Default)]
struct BalanceDelta {
    available_delta: i128,
    frozen_delta: i128,
}

#[derive(Debug, Clone)]
struct SettlementOutcome {
    vouchers: Vec<SettlementTransferVoucher>,
    positions: BTreeMap<String, HyperliquidPerpPosition>,
    balance_deltas: BTreeMap<String, BalanceDelta>,
}

#[derive(Debug, Clone, Copy)]
struct PositionAfter {
    side: HyperliquidPerpPositionSide,
    qty: u64,
    entry_price: u64,
    margin: u64,
    realized_pnl_delta: i64,
}

fn validate_trade_ids_match(
    cmd: &SettleHyperliquidPerpTradeCmd,
    state: &SettleHyperliquidPerpTradeState,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    if cmd.trade_ids.len() != state.trades.len() {
        return Err(SettleHyperliquidPerpTradeError::TradeIdsMismatch);
    }
    if cmd.trade_ids.iter().zip(&state.trades).any(|(trade_id, trade)| trade_id != &trade.trade_id)
    {
        return Err(SettleHyperliquidPerpTradeError::TradeIdsMismatch);
    }
    Ok(())
}

fn ensure_not_settled(
    state: &SettleHyperliquidPerpTradeState,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    let settled: HashSet<&str> = state.settled_trade_ids.iter().map(String::as_str).collect();
    if state.trades.iter().any(|trade| settled.contains(trade.trade_id.as_str())) {
        return Err(SettleHyperliquidPerpTradeError::TradeAlreadySettled);
    }
    Ok(())
}

fn derive_settlement_outcome(
    cmd: &SettleHyperliquidPerpTradeCmd,
    state: &SettleHyperliquidPerpTradeState,
) -> Result<SettlementOutcome, SettleHyperliquidPerpTradeError> {
    let mut positions = position_map(state.positions.clone().into_iter());
    let mut balance_deltas: BTreeMap<String, BalanceDelta> = BTreeMap::new();
    let mut vouchers = Vec::with_capacity(state.trades.len());

    for (index, trade) in state.trades.iter().enumerate() {
        let notional =
            trade.notional_quote().ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
        let taker_fee = fee_from_bps(notional, state.taker_fee_bps)?;
        let maker_fee = fee_from_bps(notional, state.maker_fee_bps)?;

        let mut taker_realized_pnl = 0_i64;
        let mut maker_realized_pnl = 0_i64;

        let account_roles = [
            (
                trade.taker_account_id.as_str(),
                AccountRole { side: taker_position_side(trade), is_taker: true },
            ),
            (
                trade.maker_account_id.as_str(),
                AccountRole { side: maker_position_side(trade), is_taker: false },
            ),
        ];

        for (account_id, role) in account_roles {
            let position_key = position_key(account_id, trade.asset, trade.symbol.as_str());
            let Some(position) = positions.get(&position_key).cloned() else {
                return Err(SettleHyperliquidPerpTradeError::PositionNotFound);
            };
            validate_position_for_trade(&position, account_id, trade)?;

            let after = apply_trade_to_position(&position, role.side, trade.qty, trade.price)?;
            let mut next_position = position.clone();
            let next_realized_pnl = position
                .realized_pnl
                .checked_add(after.realized_pnl_delta)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let next_version = if position.version == 0 {
                1
            } else {
                position
                    .version
                    .checked_add(1)
                    .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?
            };
            next_position.apply_after(
                after.side,
                after.qty,
                after.entry_price,
                after.margin,
                None,
                0,
                next_realized_pnl,
                next_version,
            );

            let margin_delta = i128::from(after.margin) - i128::from(position.margin);
            add_margin_delta(
                &mut balance_deltas,
                account_id,
                state.margin_asset_id.as_str(),
                margin_delta,
            )?;
            let fee = if role.is_taker { taker_fee } else { maker_fee };
            add_available_delta(
                &mut balance_deltas,
                account_id,
                state.margin_asset_id.as_str(),
                i128::from(after.realized_pnl_delta) - i128::from(fee),
            )?;

            if role.is_taker {
                taker_realized_pnl = after.realized_pnl_delta;
            } else {
                maker_realized_pnl = after.realized_pnl_delta;
            }

            positions.insert(position_key, next_position);
        }

        let settlement_id = settlement_id(cmd.settlement_batch_id.as_str(), index + 1);
        let voucher_id =
            format!("perp-voucher:{}:{}", cmd.settlement_batch_id, trade.trade_id.as_str());
        let voucher = SettlementTransferVoucher::build_perp_voucher(
            voucher_id,
            settlement_id,
            trade,
            state.margin_asset_id.as_str(),
            state.fee_account_id.clone(),
            taker_fee,
            maker_fee,
            taker_realized_pnl,
            maker_realized_pnl,
        )
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
        vouchers.push(voucher);
    }

    Ok(SettlementOutcome { vouchers, positions, balance_deltas })
}

fn validate_position_for_trade(
    position: &HyperliquidPerpPosition,
    account_id: &str,
    trade: &HyperliquidPerpTrade,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    if !position.belongs_to_account(account_id)
        || !position.trades_asset(trade.asset)
        || !position.trades_symbol(trade.symbol.as_str())
    {
        return Err(SettleHyperliquidPerpTradeError::PositionMismatch);
    }
    if position.leverage == 0 {
        return Err(SettleHyperliquidPerpTradeError::InvalidLeverage);
    }
    if !position.has_consistent_state() || position.required_margin() != Some(position.margin) {
        return Err(SettleHyperliquidPerpTradeError::InconsistentPositionState);
    }
    Ok(())
}

fn validate_balances_can_apply(
    state: &SettleHyperliquidPerpTradeState,
    deltas: &BTreeMap<String, BalanceDelta>,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    let balances = balance_map(&state.margin_balances);
    for (key, delta) in deltas {
        let balance =
            balances.get(key).ok_or(SettleHyperliquidPerpTradeError::MarginBalanceNotFound)?;
        if !balance.is_asset(state.margin_asset_id.as_str()) {
            return Err(SettleHyperliquidPerpTradeError::InvalidMarginBalance);
        }
        if apply_signed_delta(balance.available, delta.available_delta).is_err() {
            return Err(SettleHyperliquidPerpTradeError::InsufficientAvailableMargin);
        }
        if apply_signed_delta(balance.frozen, delta.frozen_delta).is_err() {
            return Err(SettleHyperliquidPerpTradeError::InsufficientFrozenMargin);
        }
    }
    Ok(())
}

fn apply_trade_to_position(
    position: &HyperliquidPerpPosition,
    incoming_side: HyperliquidPerpPositionSide,
    trade_qty: u64,
    trade_price: u64,
) -> Result<PositionAfter, SettleHyperliquidPerpTradeError> {
    let (side, qty, entry_price, realized_pnl_delta) =
        if position.is_flat() || position.side == incoming_side {
            let next_qty = position
                .qty
                .checked_add(trade_qty)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let old_notional = position
                .qty
                .checked_mul(position.entry_price)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let add_notional = trade_qty
                .checked_mul(trade_price)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let total_notional = old_notional
                .checked_add(add_notional)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            (incoming_side, next_qty, total_notional / next_qty, 0_i64)
        } else {
            let close_qty = position.qty.min(trade_qty);
            let pnl_per_unit = match position.side {
                HyperliquidPerpPositionSide::Long => {
                    i128::from(trade_price) - i128::from(position.entry_price)
                }
                HyperliquidPerpPositionSide::Short => {
                    i128::from(position.entry_price) - i128::from(trade_price)
                }
                HyperliquidPerpPositionSide::Flat => 0,
            };
            let realized = checked_i128_to_i64(pnl_per_unit * i128::from(close_qty))?;

            if trade_qty < position.qty {
                (position.side, position.qty - trade_qty, position.entry_price, realized)
            } else if trade_qty == position.qty {
                (HyperliquidPerpPositionSide::Flat, 0, 0, realized)
            } else {
                (incoming_side, trade_qty - position.qty, trade_price, realized)
            }
        };

    let margin = required_position_margin(qty, entry_price, position.leverage)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
    Ok(PositionAfter { side, qty, entry_price, margin, realized_pnl_delta })
}

fn fee_from_bps(notional: u64, fee_bps: u64) -> Result<u64, SettleHyperliquidPerpTradeError> {
    notional
        .checked_mul(fee_bps)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)
        .map(|value| value / FEE_BPS_DENOMINATOR)
}

fn add_margin_delta(
    deltas: &mut BTreeMap<String, BalanceDelta>,
    account_id: &str,
    asset_id: &str,
    margin_delta: i128,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    let delta = deltas.entry(balance_key(account_id, asset_id)).or_default();
    delta.available_delta = delta
        .available_delta
        .checked_sub(margin_delta)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
    delta.frozen_delta = delta
        .frozen_delta
        .checked_add(margin_delta)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
    Ok(())
}

fn add_available_delta(
    deltas: &mut BTreeMap<String, BalanceDelta>,
    account_id: &str,
    asset_id: &str,
    amount: i128,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    let delta = deltas.entry(balance_key(account_id, asset_id)).or_default();
    delta.available_delta = delta
        .available_delta
        .checked_add(amount)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
    Ok(())
}

fn apply_signed_delta(value: u64, delta: i128) -> Result<u64, SettleHyperliquidPerpTradeError> {
    let next = i128::from(value)
        .checked_add(delta)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
    if next < 0 || next > i128::from(u64::MAX) {
        return Err(SettleHyperliquidPerpTradeError::ArithmeticOverflow);
    }
    Ok(next as u64)
}

fn checked_i128_to_i64(value: i128) -> Result<i64, SettleHyperliquidPerpTradeError> {
    i64::try_from(value).map_err(|_| SettleHyperliquidPerpTradeError::ArithmeticOverflow)
}

fn taker_position_side(trade: &HyperliquidPerpTrade) -> HyperliquidPerpPositionSide {
    match trade.taker_side {
        crate::entity::HyperliquidPerpOrderSide::Buy => HyperliquidPerpPositionSide::Long,
        crate::entity::HyperliquidPerpOrderSide::Sell => HyperliquidPerpPositionSide::Short,
    }
}

fn maker_position_side(trade: &HyperliquidPerpTrade) -> HyperliquidPerpPositionSide {
    match trade.taker_side {
        crate::entity::HyperliquidPerpOrderSide::Buy => HyperliquidPerpPositionSide::Short,
        crate::entity::HyperliquidPerpOrderSide::Sell => HyperliquidPerpPositionSide::Long,
    }
}

fn position_map(
    positions: impl Iterator<Item = HyperliquidPerpPosition>,
) -> BTreeMap<String, HyperliquidPerpPosition> {
    positions
        .map(|position| {
            (position_key(&position.account_id, position.asset, position.symbol.as_str()), position)
        })
        .collect()
}

fn balance_map(balances: &[Balance]) -> HashMap<String, &Balance> {
    balances
        .iter()
        .map(|balance| (balance_key(&balance.account_id, &balance.asset_id), balance))
        .collect()
}

fn position_key(account_id: &str, asset: u32, symbol: &str) -> String {
    format!("{account_id}:{asset}:{symbol}")
}

fn balance_key(account_id: &str, asset_id: &str) -> String {
    format!("{account_id}:{asset_id}")
}

fn settlement_id(settlement_batch_id: &str, index: usize) -> String {
    format!("{settlement_batch_id}-{index}")
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};
    use proptest::prelude::*;

    use super::*;
    use crate::entity::{HyperliquidPerpOrderSide, required_position_margin};
    use crate::use_case::support::field_as_u64;

    fn cmd(trade_ids: Vec<&str>) -> SettleHyperliquidPerpTradeCmd {
        SettleHyperliquidPerpTradeCmd {
            party_id: "clearing-house".to_string(),
            settlement_batch_id: "settle-1".to_string(),
            trade_ids: trade_ids.into_iter().map(str::to_string).collect(),
        }
    }

    fn trade(
        trade_id: &str,
        taker_side: HyperliquidPerpOrderSide,
        taker_account_id: &str,
        maker_account_id: &str,
        price: u64,
        qty: u64,
    ) -> HyperliquidPerpTrade {
        HyperliquidPerpTrade::new(
            trade_id.to_string(),
            "match-1".to_string(),
            0,
            "BTC-PERP".to_string(),
            format!("{trade_id}-taker"),
            format!("{trade_id}-maker"),
            taker_account_id.to_string(),
            maker_account_id.to_string(),
            taker_side,
            price,
            qty,
            1_717_171_717_000,
        )
    }

    fn empty_position(account_id: &str, leverage: u64) -> HyperliquidPerpPosition {
        HyperliquidPerpPosition::empty_slot(
            format!("{account_id}:BTC-PERP"),
            account_id.to_string(),
            0,
            "BTC-PERP".to_string(),
            leverage,
        )
    }

    fn position(
        account_id: &str,
        side: HyperliquidPerpPositionSide,
        qty: u64,
        entry_price: u64,
        leverage: u64,
        realized_pnl: i64,
    ) -> HyperliquidPerpPosition {
        let margin = required_position_margin(qty, entry_price, leverage).unwrap_or(0);
        HyperliquidPerpPosition::new(
            format!("{account_id}:BTC-PERP"),
            account_id.to_string(),
            0,
            "BTC-PERP".to_string(),
            side,
            qty,
            entry_price,
            leverage,
            crate::entity::HyperliquidPerpMarginMode::Cross,
            margin,
            None,
            0,
            realized_pnl,
            3,
        )
    }

    fn balance(account_id: &str, available: u64, frozen: u64) -> Balance {
        Balance::new(account_id.to_string(), "USDC".to_string(), available, frozen, 7)
    }

    fn state(
        trades: Vec<HyperliquidPerpTrade>,
        positions: Vec<HyperliquidPerpPosition>,
        balances: Vec<Balance>,
    ) -> SettleHyperliquidPerpTradeState {
        SettleHyperliquidPerpTradeState {
            trades,
            positions,
            margin_balances: balances,
            margin_asset_id: "USDC".to_string(),
            fee_account_id: "fee-account".to_string(),
            taker_fee_bps: 5,
            maker_fee_bps: 2,
            settled_trade_ids: Vec::new(),
        }
    }

    fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }
            std::str::from_utf8(change.new_value_bytes()).ok()
        })
    }

    fn event_field_i64(event: &EntityReplayableEvent, field_name: &str) -> Option<i64> {
        event_field(event, field_name)?.parse::<i64>().ok()
    }

    fn updated_event<'a>(
        events: &'a [EntityReplayableEvent],
        account_id: &str,
        field_name: &str,
    ) -> Option<&'a EntityReplayableEvent> {
        events.iter().find(|event| {
            event.is_updated()
                && event_field(event, "account_id") == Some(account_id)
                && event_field(event, field_name).is_some()
        })
    }

    fn updated_event_with_field<'a>(
        events: &'a [EntityReplayableEvent],
        field_name: &str,
    ) -> Option<&'a EntityReplayableEvent> {
        events.iter().find(|event| event.is_updated() && event_field(event, field_name).is_some())
    }

    fn settlement_voucher(
        changes: &SettleHyperliquidPerpTradeChanges,
    ) -> &SettlementTransferVoucher {
        &changes.created_vouchers[0]
    }

    #[test]
    fn role_is_clearing_house() {
        assert_eq!(SettleHyperliquidPerpTradeUseCase.role(), "ClearingHouse");
    }

    #[test]
    fn pre_check_rejects_empty_command_fields() {
        let mut invalid_party = cmd(vec!["trade-1"]);
        invalid_party.party_id.clear();
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase.pre_check_command(&invalid_party),
            Err(SettleHyperliquidPerpTradeError::InvalidPartyId)
        );

        let mut invalid_batch = cmd(vec!["trade-1"]);
        invalid_batch.settlement_batch_id.clear();
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase.pre_check_command(&invalid_batch),
            Err(SettleHyperliquidPerpTradeError::InvalidSettlementBatchId)
        );

        assert_eq!(
            SettleHyperliquidPerpTradeUseCase.pre_check_command(&cmd(Vec::new())),
            Err(SettleHyperliquidPerpTradeError::EmptyTradeIds)
        );
    }

    #[test]
    fn validate_rejects_trade_id_mismatch_and_already_settled() {
        let trade = trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", 100, 2);
        let state = state(
            vec![trade],
            vec![empty_position("buyer", 10), empty_position("seller", 10)],
            vec![balance("buyer", 1_000, 0), balance("seller", 1_000, 0)],
        );

        assert_eq!(
            SettleHyperliquidPerpTradeUseCase
                .validate_against_state(&cmd(vec!["different"]), &state),
            Err(SettleHyperliquidPerpTradeError::TradeIdsMismatch)
        );

        let mut settled_state = state;
        settled_state.settled_trade_ids = vec!["trade-1".to_string()];
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase
                .validate_against_state(&cmd(vec!["trade-1"]), &settled_state),
            Err(SettleHyperliquidPerpTradeError::TradeAlreadySettled)
        );
    }

    #[test]
    fn validate_rejects_missing_balance_position_and_invalid_leverage() {
        let trade = trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", 100, 2);
        let missing_position = state(
            vec![trade.clone()],
            vec![empty_position("buyer", 10)],
            vec![balance("buyer", 1_000, 0), balance("seller", 1_000, 0)],
        );
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase
                .validate_against_state(&cmd(vec!["trade-1"]), &missing_position),
            Err(SettleHyperliquidPerpTradeError::PositionNotFound)
        );

        let missing_balance = state(
            vec![trade.clone()],
            vec![empty_position("buyer", 10), empty_position("seller", 10)],
            vec![balance("buyer", 1_000, 0)],
        );
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase
                .validate_against_state(&cmd(vec!["trade-1"]), &missing_balance),
            Err(SettleHyperliquidPerpTradeError::MarginBalanceNotFound)
        );

        let invalid_leverage = state(
            vec![trade],
            vec![empty_position("buyer", 0), empty_position("seller", 10)],
            vec![balance("buyer", 1_000, 0), balance("seller", 1_000, 0)],
        );
        assert_eq!(
            SettleHyperliquidPerpTradeUseCase
                .validate_against_state(&cmd(vec!["trade-1"]), &invalid_leverage),
            Err(SettleHyperliquidPerpTradeError::InvalidLeverage)
        );
    }

    #[test]
    fn compute_opens_positions_and_reserves_cross_margin()
    -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![empty_position("buyer", 10), empty_position("seller", 10)],
            vec![balance("buyer", 1_000, 0), balance("seller", 1_000, 0)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let voucher = settlement_voucher(&changes);

        assert_eq!(events.len(), 5);
        assert_eq!(changes.created_vouchers.len(), 1);
        assert_eq!(changes.changed_positions.len(), 2);
        assert_eq!(changes.changed_margin_balances.len(), 2);
        assert_eq!(changes.changed_positions[0].before.version, 0);
        assert_eq!(changes.changed_positions[0].after.margin, 20);
        assert_eq!(changes.changed_margin_balances[0].before.available, 1_000);
        assert_eq!(changes.changed_margin_balances[0].after.frozen, 20);
        assert_eq!(voucher.voucher_id(), "perp-voucher:settle-1:trade-1");
        assert_eq!(voucher.settlement_id(), "settle-1-1");
        assert_eq!(voucher.trade_id(), "trade-1");
        assert_eq!(voucher.match_id(), Some("match-1"));
        assert_eq!(voucher.settlement_kind(), crate::entity::SettlementKind::Perp);
        assert!(
            voucher
                .transfers_for_purpose(
                    crate::entity::SettlementTransferPurpose::PerpRealizedPnlTransfer
                )
                .is_empty()
        );
        assert!(
            voucher
                .transfers_for_purpose(crate::entity::SettlementTransferPurpose::TradingFee)
                .is_empty()
        );
        assert!(events[0].is_created());
        assert_eq!(event_field(&events[0], "voucher_id"), Some("perp-voucher:settle-1:trade-1"));
        assert_eq!(event_field(&events[0], "settlement_kind"), Some("perp"));
        assert_eq!(event_field(&events[0], "settlement_id"), Some("settle-1-1"));
        assert_eq!(event_field(&events[0], "trade_id"), Some("trade-1"));
        assert_eq!(event_field(&events[0], "fee_account_id"), Some("fee-account"));
        assert_eq!(field_as_u64(&events[0], "leg_count"), Some(0));
        assert_eq!(field_as_u64(&events[0], "notional"), None);
        assert_eq!(field_as_u64(&events[0], "taker_fee"), None);
        assert_eq!(field_as_u64(&events[0], "maker_fee"), None);

        assert_eq!(event_field(&events[1], "account_id"), Some("buyer"));
        assert_eq!(event_field(&events[1], "side"), Some("long"));
        assert_eq!(field_as_u64(&events[1], "margin"), Some(20));
        assert_eq!(event_field(&events[2], "account_id"), Some("seller"));
        assert_eq!(event_field(&events[2], "side"), Some("short"));
        assert_eq!(field_as_u64(&events[2], "margin"), Some(20));

        assert_eq!(
            event_field(updated_event(&events, "buyer", "frozen").unwrap(), "frozen"),
            Some("20")
        );
        assert_eq!(
            event_field(updated_event(&events, "seller", "frozen").unwrap(), "available"),
            Some("980")
        );

        Ok(())
    }

    #[test]
    fn compute_adds_same_side_position_with_weighted_entry()
    -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", 120, 2)],
            vec![
                position("buyer", HyperliquidPerpPositionSide::Long, 2, 100, 10, 0),
                empty_position("seller", 10),
            ],
            vec![balance("buyer", 1_000, 20), balance("seller", 1_000, 0)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let buyer_position = updated_event_with_field(&events, "entry_price").unwrap();
        let voucher = settlement_voucher(&changes);

        assert_eq!(changes.changed_positions[0].before.qty, 2);
        assert_eq!(changes.changed_positions[0].after.qty, 4);
        assert!(
            voucher
                .transfers_for_purpose(
                    crate::entity::SettlementTransferPurpose::PerpRealizedPnlTransfer
                )
                .is_empty()
        );
        assert_eq!(event_field(buyer_position, "qty"), Some("4"));
        assert_eq!(event_field(buyer_position, "entry_price"), Some("110"));
        assert_eq!(event_field(buyer_position, "margin"), Some("44"));

        Ok(())
    }

    #[test]
    fn compute_partial_close_without_counterparty_close_creates_no_realized_pnl_transfer()
    -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Sell, "buyer", "seller", 130, 1)],
            vec![
                position("buyer", HyperliquidPerpPositionSide::Long, 3, 100, 10, 0),
                empty_position("seller", 10),
            ],
            vec![balance("buyer", 1_000, 30), balance("seller", 1_000, 0)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let buyer_position = updated_event_with_field(&events, "realized_pnl").unwrap();
        let voucher = settlement_voucher(&changes);

        assert_eq!(changes.changed_positions[0].before.qty, 3);
        assert_eq!(changes.changed_positions[0].after.qty, 2);
        assert!(
            voucher
                .transfers_for_purpose(
                    crate::entity::SettlementTransferPurpose::PerpRealizedPnlTransfer
                )
                .is_empty()
        );
        assert_eq!(event_field(buyer_position, "qty"), Some("2"));
        assert_eq!(event_field(buyer_position, "margin"), Some("20"));
        assert_eq!(event_field_i64(buyer_position, "realized_pnl"), Some(30));
        assert_eq!(
            event_field(updated_event(&events, "buyer", "available").unwrap(), "available"),
            Some("1040")
        );

        Ok(())
    }

    #[test]
    fn compute_partial_close_with_counterparty_close_creates_realized_pnl_transfer_leg()
    -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Sell, "buyer", "seller", 130, 1)],
            vec![
                position("buyer", HyperliquidPerpPositionSide::Long, 3, 100, 10, 0),
                position("seller", HyperliquidPerpPositionSide::Short, 2, 100, 10, 0),
            ],
            vec![balance("buyer", 1_000, 30), balance("seller", 1_000, 20)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let voucher = settlement_voucher(&changes);
        let pnl_legs = voucher.transfers_for_purpose(
            crate::entity::SettlementTransferPurpose::PerpRealizedPnlTransfer,
        );

        assert_eq!(pnl_legs.len(), 1);
        assert_eq!(pnl_legs[0].from_account_id(), "seller");
        assert_eq!(pnl_legs[0].to_account_id(), "buyer");
        assert_eq!(pnl_legs[0].asset_id(), "USDC");
        assert_eq!(pnl_legs[0].amount(), 30);
        assert_eq!(field_as_u64(&events[0], "leg_count"), Some(1));
        assert_eq!(event_field(&events[0], "leg_0_purpose"), Some("perp_realized_pnl_transfer"));
        assert_eq!(event_field(&events[0], "leg_0_from_account_id"), Some("seller"));
        assert_eq!(event_field(&events[0], "leg_0_to_account_id"), Some("buyer"));
        assert_eq!(
            event_field(updated_event(&events, "buyer", "available").unwrap(), "available"),
            Some("1040")
        );
        assert_eq!(
            event_field(updated_event(&events, "seller", "available").unwrap(), "available"),
            Some("980")
        );

        Ok(())
    }

    #[test]
    fn compute_flattens_positions_to_flat() -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Sell, "buyer", "seller", 120, 1)],
            vec![
                position("buyer", HyperliquidPerpPositionSide::Long, 1, 100, 10, 0),
                position("seller", HyperliquidPerpPositionSide::Short, 1, 100, 10, 0),
            ],
            vec![balance("buyer", 1_000, 10), balance("seller", 1_000, 10)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let buyer_position = updated_event_with_field(&events, "realized_pnl").unwrap();
        let voucher = settlement_voucher(&changes);

        assert!(changes.changed_positions.iter().all(|pair| pair.after.is_flat()));
        assert_eq!(changes.changed_positions[0].after.side, HyperliquidPerpPositionSide::Flat);
        assert_eq!(changes.changed_positions[1].after.side, HyperliquidPerpPositionSide::Flat);
        assert_eq!(event_field(buyer_position, "side"), Some("flat"));
        assert_eq!(event_field(buyer_position, "qty"), Some("0"));
        assert_eq!(
            voucher.transfers_for_purpose(
                crate::entity::SettlementTransferPurpose::PerpRealizedPnlTransfer
            )[0]
            .amount(),
            20
        );

        Ok(())
    }

    #[test]
    fn compute_flips_position_after_over_close() -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Sell, "buyer", "seller", 90, 3)],
            vec![
                position("buyer", HyperliquidPerpPositionSide::Long, 2, 100, 10, 0),
                empty_position("seller", 10),
            ],
            vec![balance("buyer", 1_000, 20), balance("seller", 1_000, 0)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let buyer_position = updated_event_with_field(&events, "side").unwrap();
        let voucher = settlement_voucher(&changes);

        assert_eq!(changes.changed_positions[0].before.side, HyperliquidPerpPositionSide::Long);
        assert_eq!(changes.changed_positions[0].after.side, HyperliquidPerpPositionSide::Short);
        assert!(
            voucher
                .transfers_for_purpose(
                    crate::entity::SettlementTransferPurpose::PerpRealizedPnlTransfer
                )
                .is_empty()
        );
        assert_eq!(event_field(buyer_position, "side"), Some("short"));
        assert_eq!(event_field(buyer_position, "qty"), Some("1"));
        assert_eq!(event_field(buyer_position, "entry_price"), Some("90"));
        assert_eq!(event_field(buyer_position, "margin"), Some("9"));

        Ok(())
    }

    #[test]
    fn validate_rejects_insufficient_available_margin() {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", 100, 2)],
            vec![empty_position("buyer", 10), empty_position("seller", 10)],
            vec![balance("buyer", 19, 0), balance("seller", 1_000, 0)],
        );

        assert_eq!(
            SettleHyperliquidPerpTradeUseCase.validate_against_state(&cmd(vec!["trade-1"]), &state),
            Err(SettleHyperliquidPerpTradeError::InsufficientAvailableMargin)
        );
    }

    #[test]
    fn compute_creates_taker_and_maker_fee_legs() -> Result<(), SettleHyperliquidPerpTradeError> {
        let state = state(
            vec![trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", 10_000, 10)],
            vec![empty_position("buyer", 10), empty_position("seller", 10)],
            vec![balance("buyer", 100_000, 0), balance("seller", 100_000, 0)],
        );

        let changes =
            SettleHyperliquidPerpTradeUseCase.compute_changes(&cmd(vec!["trade-1"]), state)?;
        let events = changes.to_replayable_events().unwrap();
        let voucher = settlement_voucher(&changes);
        let fee_legs =
            voucher.transfers_for_purpose(crate::entity::SettlementTransferPurpose::TradingFee);

        assert_eq!(fee_legs.len(), 2);
        assert_eq!(voucher.fee_amount_paid_by("buyer"), Some(50));
        assert_eq!(voucher.fee_amount_paid_by("seller"), Some(20));
        assert_eq!(field_as_u64(&events[0], "leg_count"), Some(2));
        assert_eq!(event_field(&events[0], "leg_0_purpose"), Some("trading_fee"));
        assert_eq!(event_field(&events[0], "leg_0_to_account_id"), Some("fee-account"));
        assert_eq!(event_field(&events[0], "leg_1_purpose"), Some("trading_fee"));
        assert_eq!(event_field(&events[0], "leg_1_to_account_id"), Some("fee-account"));

        Ok(())
    }

    proptest! {
        #[test]
        fn opening_positions_keep_margin_and_voucher_first_event_consistent(
            price in 1_u64..10_000,
            qty in 1_u64..100,
            leverage in 1_u64..50,
        ) {
            let state = state(
                vec![trade("trade-1", HyperliquidPerpOrderSide::Buy, "buyer", "seller", price, qty)],
                vec![empty_position("buyer", leverage), empty_position("seller", leverage)],
                vec![balance("buyer", u64::MAX / 4, 0), balance("seller", u64::MAX / 4, 0)],
            );

            let changes = SettleHyperliquidPerpTradeUseCase
                .compute_changes(&cmd(vec!["trade-1"]), state)
                .expect("generated safe opening scenario settles");
            let events = changes.to_replayable_events().unwrap();
            let notional = price * qty;
            let expected_leg_count = u64::from(notional * 5 / FEE_BPS_DENOMINATOR > 0)
                + u64::from(notional * 2 / FEE_BPS_DENOMINATOR > 0);
            let margin = required_position_margin(qty, price, leverage).unwrap();

            prop_assert_eq!(event_field(&events[0], "settlement_kind"), Some("perp"));
            prop_assert_eq!(field_as_u64(&events[0], "leg_count"), Some(expected_leg_count));
            prop_assert_eq!(field_as_u64(&events[1], "margin"), Some(margin));
            prop_assert_eq!(field_as_u64(&events[2], "margin"), Some(margin));
            prop_assert_eq!(event_field(&events[1], "side"), Some("long"));
            prop_assert_eq!(event_field(&events[2], "side"), Some("short"));
        }
    }
}
