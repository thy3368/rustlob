use std::collections::{BTreeMap, HashSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff, UpdatedEntityPair,
};
use thiserror::Error;

use crate::entity::{
    Balance, BalanceLedgerEntry, BalanceLedgerReason, HyperliquidPerpFundingSettlement,
    HyperliquidPerpMarginMode, HyperliquidPerpPositionError, PerpPosition,
};
use crate::support::{concat3, concat4};

/// 批量结算 Hyperliquid perp 资金费的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingCmd {
    /// 发起资金费结算的业务主体。
    pub party_id: String,
    /// 资金费批次 ID；建议按小时窗口稳定生成。
    pub funding_batch_id: String,
    /// 资金费结算时间，建议使用毫秒时间戳。
    pub funding_time: u64,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// 本次资金费使用的 oracle 价格。
    pub oracle_price: u64,
    /// signed 1e8 刻度资金费率；`0.01% = 10000`。
    pub funding_rate_e8: i64,
    /// 本批次要结算的仓位 ID，顺序必须与 state.positions 一致。
    pub position_ids: Vec<String>,
}

/// 结算 Hyperliquid perp 资金费所需的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingState {
    /// 当前批次涉及的仓位快照。
    pub positions: Vec<PerpPosition>,
    /// 当前批次涉及账户的 Cross 保证金币种余额。
    pub margin_balances: Vec<Balance>,
    /// 保证金币种，例如 `USDC`。
    pub margin_asset_id: String,
    /// 当前资金费批次已经结算过的仓位 ID。
    pub settled_position_ids: Vec<String>,
    /// 当前批次的保证金模式。
    pub margin_mode: HyperliquidPerpMarginMode,
}

/// 批量结算 Hyperliquid perp 资金费可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SettleHyperliquidPerpFundingError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 资金费批次 ID 不能为空。
    #[error("funding_batch_id must not be empty")]
    InvalidFundingBatchId,
    /// 合约展示名不能为空。
    #[error("symbol must not be empty")]
    InvalidSymbol,
    /// 资金费命令必须包含至少一个仓位。
    #[error("position_ids must not be empty")]
    EmptyPositionIds,
    /// 结算价格必须大于零。
    #[error("oracle_price must be greater than zero")]
    InvalidOraclePrice,
    /// 当前版本暂不支持该保证金模式的资金费结算。
    #[error("margin mode is not supported yet")]
    UnsupportedMarginMode,
    /// 命令中的仓位 ID 与已加载仓位不一致。
    #[error("position ids do not match loaded positions")]
    PositionIdsMismatch,
    /// 仓位 market 与命令不一致。
    #[error("position does not match on_command market")]
    PositionMarketMismatch,
    /// 仓位状态不允许参与资金费结算。
    #[error("position is not eligible for funding settlement")]
    PositionNotFundingEligible,
    /// 仓位已经在当前批次结算过。
    #[error("position was already settled in current funding batch")]
    PositionAlreadySettled,
    /// 缺少账户保证金余额。
    #[error("margin balance was not found")]
    MarginBalanceNotFound,
    /// 已加载余额不是当前保证金币种余额。
    #[error("margin balance asset does not match state margin asset")]
    InvalidMarginBalance,
    /// 可用余额不足以支付资金费。
    #[error("insufficient available margin balance")]
    InsufficientAvailableMargin,
    /// 推导资金费结果时发生整数溢出。
    #[error("arithmetic overflow while deriving funding settlement")]
    ArithmeticOverflow,
}

/// Use case that settles one Hyperliquid perp funding batch into funding facts and balance deltas.
///
/// 第一版只完整支持 Cross 路径：为每个仓位创建资金费事实，并按账户聚合更新可用保证金余额。
/// Isolated 路径先显式拒绝，等独立保证金桶模型落地后再补齐。
#[derive(Debug, Clone, Copy, Default)]
pub struct SettleHyperliquidPerpFundingUseCase;

/// 结算一批资金费后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingChanges {
    /// 本批次新创建的资金费事实。
    pub created_settlements: Vec<HyperliquidPerpFundingSettlement>,
    /// 本批次实际受影响保证金余额的 before/after，顺序与输入状态稳定对齐。
    pub changed_margin_balances: Vec<UpdatedEntityPair<Balance>>,
    /// 本批次新创建的余额流水。
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntry>,
}

impl ReplayableChanges for SettleHyperliquidPerpFundingChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = Vec::with_capacity(0);
        for settlement in &self.created_settlements {
            events.push(settlement.track_create_event()?);
        }
        for balance in &self.changed_margin_balances {
            events.push(balance.after.track_update_event_from(&balance.before)?);
        }
        for entry in &self.created_balance_ledger_entries {
            events.push(entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl StateMachineV2 for SettleHyperliquidPerpFundingUseCase {
    type Command = LogEntry<SettleHyperliquidPerpFundingCmd>;
    type StateGiven = SettleHyperliquidPerpFundingState;
    type Error = SettleHyperliquidPerpFundingError;
    type StateChanged = SettleHyperliquidPerpFundingChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::InvalidPartyId);
        }
        if cmd.funding_batch_id.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::InvalidFundingBatchId);
        }
        if cmd.symbol.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::InvalidSymbol);
        }
        if cmd.position_ids.is_empty() {
            return Err(SettleHyperliquidPerpFundingError::EmptyPositionIds);
        }
        if cmd.oracle_price == 0 {
            return Err(SettleHyperliquidPerpFundingError::InvalidOraclePrice);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        validate_common(cmd, state)?;
        match state.margin_mode {
            HyperliquidPerpMarginMode::Cross => validate_cross(cmd, state),
            HyperliquidPerpMarginMode::Isolated => {
                Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode)
            }
        }
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let state = state.clone();
        let outcome = match state.margin_mode {
            HyperliquidPerpMarginMode::Cross => derive_cross_outcome(cmd, &state)?,
            HyperliquidPerpMarginMode::Isolated => {
                return Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode);
            }
        };
        let balance_ledger_reasons = settlement_balance_ledger_reasons(
            &state.positions,
            &outcome.settlements,
            state.margin_asset_id.as_str(),
        );
        let mut changed_margin_balances = Vec::with_capacity(0);
        let mut created_balance_ledger_entries = Vec::with_capacity(0);
        for mut balance in state.margin_balances {
            let key = balance.business_entity_id();
            let Some(delta) = outcome.balance_deltas.get(&key) else {
                continue;
            };
            if delta.available_delta == 0 {
                continue;
            }
            let balance_id = balance.business_entity_id();
            let reason = balance_ledger_reasons
                .get(&balance_id)
                .cloned()
                .ok_or(SettleHyperliquidPerpFundingError::MarginBalanceNotFound)?;
            let before = balance.clone();
            let amount = u64::try_from(delta.available_delta.unsigned_abs())
                .map_err(|_| SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
            let entry_id = concat4(
                "balance-ledger:funding:",
                cmd.funding_batch_id.as_str(),
                ":",
                balance.business_entity_id().as_str(),
            );
            let mut entry = if delta.available_delta > 0 {
                BalanceLedgerEntry::credit_available(
                    entry_id,
                    balance.account_id.clone(),
                    balance.asset_id.clone(),
                    balance.business_entity_id(),
                    amount,
                    reason,
                )
            } else {
                BalanceLedgerEntry::debit_available(
                    entry_id,
                    balance.account_id.clone(),
                    balance.asset_id.clone(),
                    balance.business_entity_id(),
                    amount,
                    reason,
                )
            }
            .map_err(map_funding_balance_ledger_error)?;
            entry.apply_to(&mut balance).map_err(map_funding_balance_ledger_error)?;

            changed_margin_balances.push(UpdatedEntityPair { before, after: balance });
            created_balance_ledger_entries.push(entry);
        }

        Ok(SettleHyperliquidPerpFundingChanges {
            created_settlements: outcome.settlements,
            changed_margin_balances,
            created_balance_ledger_entries,
        })
    }
}

impl SettleHyperliquidPerpFundingUseCase {
    pub const fn role(&self) -> &'static str {
        "ClearingHouse"
    }
}
impl StateMachineV2Diff for SettleHyperliquidPerpFundingUseCase {
    type StateDiff = SettleHyperliquidPerpFundingChanges;
    fn do_compute_state_diff(
        _: SettleHyperliquidPerpFundingState,
        changed: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(changed)
    }
}

#[derive(Debug, Clone, Default)]
struct FundingOutcome {
    settlements: Vec<HyperliquidPerpFundingSettlement>,
    balance_deltas: BTreeMap<String, SignedBalanceDelta>,
}

#[derive(Debug, Clone, Copy, Default)]
struct SignedBalanceDelta {
    available_delta: i128,
}

fn validate_common(
    cmd: &SettleHyperliquidPerpFundingCmd,
    state: &SettleHyperliquidPerpFundingState,
) -> Result<(), SettleHyperliquidPerpFundingError> {
    if cmd.position_ids.len() != state.positions.len() {
        return Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch);
    }

    let settled: HashSet<&str> = state.settled_position_ids.iter().map(String::as_str).collect();
    for (expected_position_id, position) in cmd.position_ids.iter().zip(&state.positions) {
        if expected_position_id != &position.position_key {
            return Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch);
        }
        if !position.trades_asset(cmd.asset) || !position.trades_symbol(cmd.symbol.as_str()) {
            return Err(SettleHyperliquidPerpFundingError::PositionMarketMismatch);
        }
        if !position.is_funding_eligible() {
            return Err(SettleHyperliquidPerpFundingError::PositionNotFundingEligible);
        }
        if settled.contains(position.position_key.as_str()) {
            return Err(SettleHyperliquidPerpFundingError::PositionAlreadySettled);
        }
    }

    Ok(())
}

fn validate_cross(
    cmd: &SettleHyperliquidPerpFundingCmd,
    state: &SettleHyperliquidPerpFundingState,
) -> Result<(), SettleHyperliquidPerpFundingError> {
    let balances = balance_map(&state.margin_balances, state.margin_asset_id.as_str())?;
    let outcome = derive_cross_outcome(cmd, state)?;

    for (key, delta) in outcome.balance_deltas {
        let balance = balances
            .get(&key)
            .copied()
            .ok_or(SettleHyperliquidPerpFundingError::MarginBalanceNotFound)?;
        if delta.available_delta >= 0 {
            continue;
        }
        let amount = delta
            .available_delta
            .checked_neg()
            .and_then(|value| u64::try_from(value).ok())
            .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
        if !balance.can_debit_available(amount) {
            return Err(SettleHyperliquidPerpFundingError::InsufficientAvailableMargin);
        }
    }

    Ok(())
}

fn derive_cross_outcome(
    cmd: &SettleHyperliquidPerpFundingCmd,
    state: &SettleHyperliquidPerpFundingState,
) -> Result<FundingOutcome, SettleHyperliquidPerpFundingError> {
    let mut outcome = FundingOutcome::default();

    for position in &state.positions {
        let Some(settlement) = position
            .derive_funding_settlement(
                cmd.funding_batch_id.as_str(),
                cmd.funding_time,
                cmd.oracle_price,
                cmd.funding_rate_e8,
            )
            .map_err(map_position_funding_error)?
        else {
            continue;
        };

        let signed_usdc_delta = settlement.signed_usdc_delta;
        let account_id = settlement.account_id.clone();
        outcome.settlements.push(settlement);
        let delta = outcome
            .balance_deltas
            .entry(balance_key(&account_id, state.margin_asset_id.as_str()))
            .or_default();
        delta.available_delta = delta
            .available_delta
            .checked_add(signed_usdc_delta)
            .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
    }

    Ok(outcome)
}

fn map_position_funding_error(
    error: HyperliquidPerpPositionError,
) -> SettleHyperliquidPerpFundingError {
    match error {
        HyperliquidPerpPositionError::ArithmeticOverflow => {
            SettleHyperliquidPerpFundingError::ArithmeticOverflow
        }
        HyperliquidPerpPositionError::InvalidTradeQty
        | HyperliquidPerpPositionError::InvalidTradePrice
        | HyperliquidPerpPositionError::InvalidLeverage
        | HyperliquidPerpPositionError::InconsistentState
        | HyperliquidPerpPositionError::MarginModeMismatch => {
            SettleHyperliquidPerpFundingError::PositionNotFundingEligible
        }
    }
}

fn balance_map<'a>(
    balances: &'a [Balance],
    margin_asset_id: &str,
) -> Result<BTreeMap<String, &'a Balance>, SettleHyperliquidPerpFundingError> {
    let mut map = BTreeMap::new();
    for balance in balances {
        if !balance.is_asset(common_entity::stable_string_entity_id(margin_asset_id)) {
            return Err(SettleHyperliquidPerpFundingError::InvalidMarginBalance);
        }
        map.insert(balance.business_entity_id(), balance);
    }
    Ok(map)
}

fn settlement_balance_ledger_reasons(
    positions: &[PerpPosition],
    settlements: &[HyperliquidPerpFundingSettlement],
    margin_asset_id: &str,
) -> BTreeMap<String, BalanceLedgerReason> {
    #[derive(Debug, Clone, Default)]
    struct FundingLedgerRefs {
        settlement_ids: Vec<String>,
        position_ids: Vec<String>,
        funding_batch_id: String,
    }

    let positions_by_id: BTreeMap<&str, &PerpPosition> =
        positions.iter().map(|position| (position.position_key.as_str(), position)).collect();

    let mut refs_by_balance: BTreeMap<String, FundingLedgerRefs> = BTreeMap::new();
    for settlement in settlements {
        let Some(position) = positions_by_id.get(settlement.position_id.as_str()) else {
            continue;
        };
        let balance_id = balance_key(position.account_id.as_str(), margin_asset_id);
        let refs = refs_by_balance.entry(balance_id).or_default();
        refs.funding_batch_id = settlement.funding_batch_id.clone();
        refs.settlement_ids.push(settlement.funding_settlement_id.clone());
        refs.position_ids.push(settlement.position_id.clone());
    }

    refs_by_balance
        .into_iter()
        .map(|(balance_id, refs)| {
            (
                balance_id,
                BalanceLedgerReason::SettlePerpFunding {
                    funding_batch_id: refs.funding_batch_id,
                    settlement_ids: refs.settlement_ids,
                    position_ids: refs.position_ids,
                },
            )
        })
        .collect()
}

fn balance_key(account_id: &str, asset_id: &str) -> String {
    concat3(account_id, ":", asset_id)
}

fn map_funding_balance_ledger_error(
    error: crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error,
) -> SettleHyperliquidPerpFundingError {
    match error {
        crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error::InsufficientAvailableBalance => {
            SettleHyperliquidPerpFundingError::InsufficientAvailableMargin
        }
        crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error::InvalidAmount
        | crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error::InsufficientFrozenBalance
        | crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error::ArithmeticOverflow
        | crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error::BalanceIdentityMismatch
        | crate::entity::account::balance_ledger_v2::BalanceLedgerEntryV2Error::AlreadyApplied => {
            SettleHyperliquidPerpFundingError::ArithmeticOverflow
        }
    }
}
