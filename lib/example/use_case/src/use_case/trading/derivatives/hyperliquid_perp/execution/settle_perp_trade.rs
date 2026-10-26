use std::collections::{BTreeMap, HashMap, HashSet};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    Balance, HyperliquidPerpPositionError, HyperliquidPerpTrade, PerpPosition,
    SettlementTransferVoucher,
};
use crate::support::concat3;

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

/// 清结算 Hyperliquid perp 成交时需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpTradeState {
    /// 要清结算的成交事实。
    pub trades: Vec<HyperliquidPerpTrade>,
    /// 本批次涉及账户在成交合约上的仓位槽位；空仓但可创建仓位时传 `version == 0` 的 flat 槽位。
    pub positions: Vec<PerpPosition>,
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
    pub created_vouchers: Vec<SettlementTransferVoucher>, //todo delete
    /// 本批次仓位槽位的 before/after，顺序与输入状态稳定对齐。
    pub changed_positions: Vec<UpdatedEntityPair<PerpPosition>>,
    /// 本批次实际受影响保证金余额的 before/after，顺序与输入状态稳定对齐。
    pub changed_margin_balances: Vec<UpdatedEntityPair<Balance>>,
}

impl ReplayableChanges for SettleHyperliquidPerpTradeChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(0);
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
                let key = position_key(
                    &position.account_id,
                    position.perp_asset_id,
                    position.coin.as_str(),
                );
                outcome
                    .positions
                    .get(&key)
                    .cloned()
                    .map(|after| UpdatedEntityPair { before: position.clone(), after })
            })
            .collect::<Vec<_>>();
        let mut changed_margin_balances = Vec::with_capacity(0);
        for balance in &state.margin_balances {
            let key = balance.business_entity_id();
            let Some(delta) = outcome.balance_deltas.get(&key) else {
                continue;
            };
            let next_available = apply_signed_delta(balance.available, delta.available_delta)?;
            let next_frozen = apply_signed_delta(balance.frozen, delta.frozen_delta)?;
            let next_version = balance
                .version
                .checked_add(1)
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
            let next_balance = Balance::new_with_snapshot_facts(
                balance.party_id,
                balance.account_id,
                balance.asset_id,
                next_available,
                next_frozen,
                balance.entry_notional(),
                balance.identifier().map(str::to_string),
                next_version,
            );
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

#[derive(Debug, Clone, Copy, Default)]
struct BalanceDelta {
    available_delta: i128,
    frozen_delta: i128,
}

#[derive(Debug, Clone)]
struct SettlementOutcome {
    vouchers: Vec<SettlementTransferVoucher>,
    positions: BTreeMap<String, PerpPosition>,
    balance_deltas: BTreeMap<String, BalanceDelta>,
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

        let taker_position_key =
            position_key(&trade.taker_account_id, trade.asset, trade.symbol.as_str());
        let maker_position_key =
            position_key(&trade.maker_account_id, trade.asset, trade.symbol.as_str());
        let Some(taker_position) = positions.get(&taker_position_key).cloned() else {
            return Err(SettleHyperliquidPerpTradeError::PositionNotFound);
        };
        let Some(maker_position) = positions.get(&maker_position_key).cloned() else {
            return Err(SettleHyperliquidPerpTradeError::PositionNotFound);
        };
        validate_position_for_trade(&taker_position, trade.taker_account_id.as_str(), trade)?;
        validate_position_for_trade(&maker_position, trade.maker_account_id.as_str(), trade)?;

        let settlement = trade
            .settle_into_positions(&taker_position, &maker_position)
            .map_err(map_position_error)?;

        add_margin_delta(
            &mut balance_deltas,
            trade.taker_account_id.as_str(),
            state.margin_asset_id.as_str(),
            settlement.taker_outcome.required_margin_delta,
        )?;
        add_available_delta(
            &mut balance_deltas,
            trade.taker_account_id.as_str(),
            state.margin_asset_id.as_str(),
            i128::from(settlement.taker_outcome.realized_pnl_delta)
                .checked_sub(i128::from(taker_fee))
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?,
        )?;
        add_margin_delta(
            &mut balance_deltas,
            trade.maker_account_id.as_str(),
            state.margin_asset_id.as_str(),
            settlement.maker_outcome.required_margin_delta,
        )?;
        add_available_delta(
            &mut balance_deltas,
            trade.maker_account_id.as_str(),
            state.margin_asset_id.as_str(),
            i128::from(settlement.maker_outcome.realized_pnl_delta)
                .checked_sub(i128::from(maker_fee))
                .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?,
        )?;

        let taker_realized_pnl = settlement.taker_outcome.realized_pnl_delta;
        let maker_realized_pnl = settlement.maker_outcome.realized_pnl_delta;
        positions.insert(taker_position_key, settlement.taker_position_after);
        positions.insert(maker_position_key, settlement.maker_position_after);

        let next_index =
            index.checked_add(1).ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
        let settlement_id = settlement_id(cmd.settlement_batch_id.as_str(), next_index);
        let voucher = trade
            .derive_perp_settlement_transfer_voucher(
                settlement_id,
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
    position: &PerpPosition,
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
    if !position.has_consistent_state() {
        return Err(SettleHyperliquidPerpTradeError::InconsistentPositionState);
    }
    Ok(())
}

fn map_position_error(error: HyperliquidPerpPositionError) -> SettleHyperliquidPerpTradeError {
    match error {
        HyperliquidPerpPositionError::InvalidTradeQty
        | HyperliquidPerpPositionError::InvalidTradePrice
        | HyperliquidPerpPositionError::InconsistentState
        | HyperliquidPerpPositionError::MarginModeMismatch => {
            SettleHyperliquidPerpTradeError::InconsistentPositionState
        }
        HyperliquidPerpPositionError::InvalidLeverage => {
            SettleHyperliquidPerpTradeError::InvalidLeverage
        }
        HyperliquidPerpPositionError::ArithmeticOverflow => {
            SettleHyperliquidPerpTradeError::ArithmeticOverflow
        }
    }
}

fn validate_balances_can_apply(
    state: &SettleHyperliquidPerpTradeState,
    deltas: &BTreeMap<String, BalanceDelta>,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    let balances = balance_map(&state.margin_balances);
    for (key, delta) in deltas {
        let balance =
            balances.get(key).ok_or(SettleHyperliquidPerpTradeError::MarginBalanceNotFound)?;
        if !balance.is_asset(common_entity::stable_string_entity_id(&state.margin_asset_id)) {
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
    required_margin_delta: i128,
) -> Result<(), SettleHyperliquidPerpTradeError> {
    let delta = deltas.entry(balance_key(account_id, asset_id)).or_default();
    delta.available_delta = delta
        .available_delta
        .checked_sub(required_margin_delta)
        .ok_or(SettleHyperliquidPerpTradeError::ArithmeticOverflow)?;
    delta.frozen_delta = delta
        .frozen_delta
        .checked_add(required_margin_delta)
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

fn position_map(positions: impl Iterator<Item = PerpPosition>) -> BTreeMap<String, PerpPosition> {
    positions
        .map(|position| {
            (
                position_key(&position.account_id, position.perp_asset_id, position.coin.as_str()),
                position,
            )
        })
        .collect()
}

fn balance_map(balances: &[Balance]) -> HashMap<String, &Balance> {
    balances.iter().map(|balance| (balance.business_entity_id(), balance)).collect()
}

fn position_key(account_id: &str, asset: u32, symbol: &str) -> String {
    let asset = asset.to_string();
    concat3(account_id, ":", concat3(asset.as_str(), ":", symbol).as_str())
}

fn balance_key(account_id: &str, asset_id: &str) -> String {
    concat3(account_id, ":", asset_id)
}

fn settlement_id(settlement_batch_id: &str, index: usize) -> String {
    concat3(settlement_batch_id, "-", index.to_string().as_str())
}
