use std::collections::{BTreeMap, HashSet};

use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, MiStateMachineOwned};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry::BalanceLedgerCommand;
use crate::entity::{
    Balance, BalanceLedgerEntry, BalanceLedgerReason, HyperliquidPerpFundingDirection,
    HyperliquidPerpFundingSettlement, HyperliquidPerpMarginMode, HyperliquidPerpPosition,
};

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

impl IssuedByParty for SettleHyperliquidPerpFundingCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 结算 Hyperliquid perp 资金费所需的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettleHyperliquidPerpFundingState {
    /// 当前批次涉及的仓位快照。
    pub positions: Vec<HyperliquidPerpPosition>,
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
    #[error("position does not match command market")]
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
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::new();
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

impl CommandUseCase4 for SettleHyperliquidPerpFundingUseCase {
    type Command = SettleHyperliquidPerpFundingCmd;
    type GivenState = SettleHyperliquidPerpFundingState;
    type Error = SettleHyperliquidPerpFundingError;
    type Changes = SettleHyperliquidPerpFundingChanges;

    fn role(&self) -> &'static str {
        "ClearingHouse"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
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

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        validate_common(cmd, state)?;
        match state.margin_mode {
            HyperliquidPerpMarginMode::Cross => validate_cross(cmd, state),
            HyperliquidPerpMarginMode::Isolated => {
                Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode)
            }
        }
    }

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
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
        let mut changed_margin_balances = Vec::new();
        for balance in &state.margin_balances {
            let key = balance_key(balance.account_id.as_str(), balance.asset_id.as_str());
            let Some(delta) = outcome.balance_deltas.get(&key) else {
                continue;
            };
            if delta.available_delta == 0 {
                continue;
            }
            let next_available = balance
                .available_after_signed_delta(delta.available_delta)
                .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
            let next_version = balance
                .version
                .checked_add(1)
                .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
            let mut next_balance = balance.clone();
            next_balance.apply_after(next_available, balance.frozen, next_version);
            changed_margin_balances
                .push(UpdatedEntityPair { before: balance.clone(), after: next_balance });
        }
        let mut created_balance_ledger_entries = Vec::with_capacity(changed_margin_balances.len());
        for updated_balance in &changed_margin_balances {
            let balance_id = balance_key(
                updated_balance.after.account_id.as_str(),
                updated_balance.after.asset_id.as_str(),
            );
            let reason = balance_ledger_reasons
                .get(&balance_id)
                .cloned()
                .ok_or(SettleHyperliquidPerpFundingError::MarginBalanceNotFound)?;
            let balance_command =
                if updated_balance.after.available > updated_balance.before.available {
                    BalanceLedgerCommand::CreditAvailable {
                        balance: updated_balance.before.clone(),
                        amount: updated_balance.after.available - updated_balance.before.available,
                    }
                } else {
                    BalanceLedgerCommand::DebitAvailable {
                        balance: updated_balance.before.clone(),
                        amount: updated_balance.before.available - updated_balance.after.available,
                    }
                };
            created_balance_ledger_entries.push(
                BalanceLedgerEntry::draft_from_balance(
                    format!(
                        "balance-ledger:funding:{}:{}",
                        cmd.funding_batch_id,
                        updated_balance.after.entity_id()
                    ),
                    &updated_balance.before,
                    balance_command.clone(),
                    reason,
                )
                .and_then(|draft| {
                    MiStateMachineOwned::compute_after_changes(&draft, &balance_command, ())
                })
                .map_err(|_| SettleHyperliquidPerpFundingError::ArithmeticOverflow)?
                .updated_entry
                .after,
            );
        }

        Ok(SettleHyperliquidPerpFundingChanges {
            created_settlements: outcome.settlements,
            changed_margin_balances,
            created_balance_ledger_entries,
        })
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
        if expected_position_id != &position.position_id {
            return Err(SettleHyperliquidPerpFundingError::PositionIdsMismatch);
        }
        if !position.trades_asset(cmd.asset) || !position.trades_symbol(cmd.symbol.as_str()) {
            return Err(SettleHyperliquidPerpFundingError::PositionMarketMismatch);
        }
        if !position.is_funding_eligible() {
            return Err(SettleHyperliquidPerpFundingError::PositionNotFundingEligible);
        }
        if settled.contains(position.position_id.as_str()) {
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
        let notional = position
            .funding_notional(cmd.oracle_price)
            .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
        let funding_fee = position
            .funding_fee(cmd.oracle_price, cmd.funding_rate_e8)
            .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
        let direction = position
            .funding_direction(cmd.funding_rate_e8)
            .unwrap_or(HyperliquidPerpFundingDirection::Receive);
        let is_payment = matches!(direction, HyperliquidPerpFundingDirection::Pay);

        outcome.settlements.push(HyperliquidPerpFundingSettlement::new(
            format!("{}-{}", cmd.funding_batch_id, position.position_id),
            cmd.funding_batch_id.clone(),
            position.account_id.clone(),
            position.position_id.clone(),
            position.asset,
            position.symbol.clone(),
            cmd.funding_time,
            position.side,
            position.qty,
            cmd.oracle_price,
            notional,
            cmd.funding_rate_e8,
            funding_fee,
            is_payment,
        ));

        let delta = outcome
            .balance_deltas
            .entry(balance_key(position.account_id.as_str(), state.margin_asset_id.as_str()))
            .or_default();
        let funding_fee = i128::from(funding_fee);
        if is_payment {
            delta.available_delta = delta
                .available_delta
                .checked_sub(funding_fee)
                .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
        } else {
            delta.available_delta = delta
                .available_delta
                .checked_add(funding_fee)
                .ok_or(SettleHyperliquidPerpFundingError::ArithmeticOverflow)?;
        }
    }

    Ok(outcome)
}

fn balance_map<'a>(
    balances: &'a [Balance],
    margin_asset_id: &str,
) -> Result<BTreeMap<String, &'a Balance>, SettleHyperliquidPerpFundingError> {
    let mut map = BTreeMap::new();
    for balance in balances {
        if !balance.is_asset(margin_asset_id) {
            return Err(SettleHyperliquidPerpFundingError::InvalidMarginBalance);
        }
        map.insert(balance_key(balance.account_id.as_str(), balance.asset_id.as_str()), balance);
    }
    Ok(map)
}

fn settlement_balance_ledger_reasons(
    positions: &[HyperliquidPerpPosition],
    settlements: &[HyperliquidPerpFundingSettlement],
    margin_asset_id: &str,
) -> BTreeMap<String, BalanceLedgerReason> {
    #[derive(Debug, Clone, Default)]
    struct FundingLedgerRefs {
        settlement_ids: Vec<String>,
        position_ids: Vec<String>,
        funding_batch_id: String,
    }

    let positions_by_id: BTreeMap<&str, &HyperliquidPerpPosition> =
        positions.iter().map(|position| (position.position_id.as_str(), position)).collect();

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
    format!("{account_id}:{asset_id}")
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::{CommandUseCase4, ReplayableChanges};

    use super::*;
    use crate::entity::HyperliquidPerpPositionSide;

    fn cross_state() -> SettleHyperliquidPerpFundingState {
        SettleHyperliquidPerpFundingState {
            positions: vec![
                HyperliquidPerpPosition::new(
                    "position-long".to_string(),
                    "trader-1".to_string(),
                    0,
                    "BTC-PERP".to_string(),
                    HyperliquidPerpPositionSide::Long,
                    2,
                    50_000,
                    10,
                    10_000,
                    0,
                    3,
                ),
                HyperliquidPerpPosition::new(
                    "position-short".to_string(),
                    "trader-2".to_string(),
                    0,
                    "BTC-PERP".to_string(),
                    HyperliquidPerpPositionSide::Short,
                    2,
                    50_000,
                    10,
                    10_000,
                    0,
                    5,
                ),
            ],
            margin_balances: vec![
                Balance::new("trader-1".to_string(), "USDC".to_string(), 1_000, 0, 7),
                Balance::new("trader-2".to_string(), "USDC".to_string(), 1_000, 0, 11),
            ],
            margin_asset_id: "USDC".to_string(),
            settled_position_ids: Vec::new(),
            margin_mode: HyperliquidPerpMarginMode::Cross,
        }
    }

    fn cross_cmd() -> SettleHyperliquidPerpFundingCmd {
        SettleHyperliquidPerpFundingCmd {
            party_id: "trader-1".to_string(),
            funding_batch_id: "funding-2026-06-10T08".to_string(),
            funding_time: 1_717_977_600_000,
            asset: 0,
            symbol: "BTC-PERP".to_string(),
            oracle_price: 50_000,
            funding_rate_e8: 10_000,
            position_ids: vec!["position-long".to_string(), "position-short".to_string()],
        }
    }

    fn same_account_cross_state() -> SettleHyperliquidPerpFundingState {
        SettleHyperliquidPerpFundingState {
            positions: vec![
                HyperliquidPerpPosition::new(
                    "position-long-1".to_string(),
                    "trader-1".to_string(),
                    0,
                    "BTC-PERP".to_string(),
                    HyperliquidPerpPositionSide::Long,
                    2,
                    50_000,
                    10,
                    10_000,
                    0,
                    3,
                ),
                HyperliquidPerpPosition::new(
                    "position-long-2".to_string(),
                    "trader-1".to_string(),
                    0,
                    "BTC-PERP".to_string(),
                    HyperliquidPerpPositionSide::Long,
                    1,
                    50_000,
                    10,
                    10_000,
                    0,
                    4,
                ),
            ],
            margin_balances: vec![Balance::new(
                "trader-1".to_string(),
                "USDC".to_string(),
                1_000,
                0,
                7,
            )],
            margin_asset_id: "USDC".to_string(),
            settled_position_ids: Vec::new(),
            margin_mode: HyperliquidPerpMarginMode::Cross,
        }
    }

    #[test]
    fn role_is_clearing_house() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        assert_eq!(use_case.role(), "ClearingHouse");
    }

    #[test]
    fn pre_check_rejects_empty_position_ids_and_zero_oracle_price() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let mut cmd = cross_cmd();
        cmd.position_ids.clear();
        assert_eq!(
            use_case.pre_check_command(&cmd),
            Err(SettleHyperliquidPerpFundingError::EmptyPositionIds)
        );

        let mut cmd = cross_cmd();
        cmd.oracle_price = 0;
        assert_eq!(
            use_case.pre_check_command(&cmd),
            Err(SettleHyperliquidPerpFundingError::InvalidOraclePrice)
        );
    }

    #[test]
    fn validate_rejects_unsupported_isolated() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let cmd = cross_cmd();
        let state = cross_state();
        let mut isolated_state = state;
        isolated_state.margin_balances =
            vec![Balance::new("trader-1".to_string(), "USDC".to_string(), 1_000, 0, 7)];
        isolated_state.margin_mode = HyperliquidPerpMarginMode::Isolated;
        assert_eq!(
            use_case.validate_against_state(&cmd, &isolated_state),
            Err(SettleHyperliquidPerpFundingError::UnsupportedMarginMode)
        );
    }

    #[test]
    fn compute_changes_updates_cross_balances_only() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let state = cross_state();
        let cmd = cross_cmd();

        let result = use_case.compute_changes(&cmd, state).unwrap();
        assert_eq!(result.created_settlements.len(), 2);
        assert_eq!(result.changed_margin_balances.len(), 2);
        assert_eq!(result.created_balance_ledger_entries.len(), 2);
        assert!(result.changed_margin_balances.iter().any(|pair| {
            pair.before.account_id == "trader-1"
                && pair.before.available == 1_000
                && pair.after.available == 990
                && pair.before.frozen == 0
                && pair.after.frozen == 0
        }));
        assert!(result.changed_margin_balances.iter().any(|pair| {
            pair.before.account_id == "trader-2"
                && pair.before.available == 1_000
                && pair.after.available == 1_010
                && pair.before.frozen == 0
                && pair.after.frozen == 0
        }));
        assert!(result.created_balance_ledger_entries.iter().all(|entry| {
            matches!(entry.reason, BalanceLedgerReason::SettlePerpFunding { .. })
        }));
        assert!(result.created_balance_ledger_entries.iter().all(|entry| {
            result.changed_margin_balances.iter().any(|pair| entry.matches_balance_update(pair))
        }));

        let events = result.to_replayable_events().unwrap();
        assert_eq!(events.len(), 6);
        assert_eq!(events.iter().filter(|event| event.is_created()).count(), 4);
        assert_eq!(events.iter().filter(|event| event.is_updated()).count(), 2);
        assert!(events.iter().filter(|event| event.is_updated()).any(|event| {
            event.field_changes.iter().any(|change| {
                change.field_name_as_str().ok() == Some("available")
                    && change.new_value_bytes() == b"990"
            })
        }));
        assert!(events.iter().filter(|event| event.is_updated()).any(|event| {
            event.field_changes.iter().any(|change| {
                change.field_name_as_str().ok() == Some("available")
                    && change.new_value_bytes() == b"1010"
            })
        }));
        assert!(!events.iter().filter(|event| event.is_updated()).any(|event| {
            event
                .field_changes
                .iter()
                .any(|change| change.field_name_as_str().ok() == Some("frozen"))
        }));
        assert!(events.iter().filter(|event| event.is_created()).any(|event| {
            event.field_changes.iter().any(|change| {
                change.field_name_as_str().ok() == Some("oracle_price")
                    && change.new_value_bytes() == b"50000"
            })
        }));
        assert!(events.iter().filter(|event| event.is_created()).any(|event| {
            event.field_changes.iter().any(|change| {
                change.field_name_as_str().ok() == Some("reason_funding_batch_id")
                    && change.new_value_bytes() == b"funding-2026-06-10T08"
            })
        }));
    }

    #[test]
    fn compute_changes_creates_funding_facts_then_applies_balance_side_effects() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let state = cross_state();
        let cmd = cross_cmd();

        let result = use_case.compute_changes(&cmd, state).unwrap();

        assert_eq!(
            result
                .created_settlements
                .iter()
                .map(|settlement| settlement.funding_batch_id.as_str())
                .collect::<Vec<_>>(),
            vec!["funding-2026-06-10T08", "funding-2026-06-10T08",]
        );
        assert_eq!(
            result
                .created_settlements
                .iter()
                .map(|settlement| settlement.position_id.as_str())
                .collect::<Vec<_>>(),
            vec!["position-long", "position-short"]
        );
        assert!(
            result
                .created_settlements
                .iter()
                .all(|settlement| settlement.funding_time == cmd.funding_time)
        );
        assert!(result.changed_margin_balances.iter().all(|pair| {
            result
                .created_settlements
                .iter()
                .any(|settlement| settlement.account_id == pair.before.account_id)
        }));
        assert!(result.created_balance_ledger_entries.iter().all(|entry| {
            result
                .created_settlements
                .iter()
                .any(|settlement| settlement.account_id == entry.account_id)
        }));
    }

    #[test]
    fn compute_changes_rejects_insufficient_balance() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let mut state = cross_state();
        state.positions.truncate(1);
        state.margin_balances =
            vec![Balance::new("trader-1".to_string(), "USDC".to_string(), 1, 0, 7)];
        let cmd = SettleHyperliquidPerpFundingCmd {
            party_id: "trader-1".to_string(),
            position_ids: vec!["position-long".to_string()],
            ..cross_cmd()
        };

        assert_eq!(
            use_case.validate_against_state(&cmd, &state),
            Err(SettleHyperliquidPerpFundingError::InsufficientAvailableMargin)
        );
    }

    #[test]
    fn compute_changes_aggregates_same_account_positions_into_one_ledger_entry() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let state = same_account_cross_state();
        let cmd = SettleHyperliquidPerpFundingCmd {
            party_id: "trader-1".to_string(),
            funding_batch_id: "funding-2026-06-10T08".to_string(),
            funding_time: 1_717_977_600_000,
            asset: 0,
            symbol: "BTC-PERP".to_string(),
            oracle_price: 50_000,
            funding_rate_e8: 10_000,
            position_ids: vec!["position-long-1".to_string(), "position-long-2".to_string()],
        };

        let result = use_case.compute_changes(&cmd, state).unwrap();

        assert_eq!(result.created_settlements.len(), 2);
        assert_eq!(result.changed_margin_balances.len(), 1);
        assert_eq!(result.created_balance_ledger_entries.len(), 1);
        let entry = &result.created_balance_ledger_entries[0];
        assert_eq!(entry.entry_id, "balance-ledger:funding:funding-2026-06-10T08:trader-1:USDC");
        assert_eq!(entry.before_available, 1_000);
        assert_eq!(entry.after_available, 985);
        assert!(entry.matches_balance_update(&result.changed_margin_balances[0]));
        assert_eq!(
            entry.reason,
            BalanceLedgerReason::SettlePerpFunding {
                funding_batch_id: "funding-2026-06-10T08".to_string(),
                settlement_ids: vec![
                    "funding-2026-06-10T08-position-long-1".to_string(),
                    "funding-2026-06-10T08-position-long-2".to_string(),
                ],
                position_ids: vec!["position-long-1".to_string(), "position-long-2".to_string()],
            }
        );
    }

    #[test]
    fn compute_changes_keeps_different_accounts_in_separate_ledgers() {
        let use_case = SettleHyperliquidPerpFundingUseCase;
        let result = use_case.compute_changes(&cross_cmd(), cross_state()).unwrap();

        let mut accounts = result
            .created_balance_ledger_entries
            .iter()
            .map(|entry| entry.account_id.as_str())
            .collect::<Vec<_>>();
        accounts.sort_unstable();

        assert_eq!(accounts, vec!["trader-1", "trader-2"]);
        assert!(result.created_balance_ledger_entries.iter().all(|entry| {
            match &entry.reason {
                BalanceLedgerReason::SettlePerpFunding {
                    funding_batch_id,
                    settlement_ids,
                    position_ids,
                } => {
                    funding_batch_id == "funding-2026-06-10T08"
                        && settlement_ids.len() == 1
                        && position_ids.len() == 1
                }
                _ => false,
            }
        }));
    }
}
