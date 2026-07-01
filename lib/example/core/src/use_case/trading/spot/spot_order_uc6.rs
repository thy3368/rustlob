use std::collections::HashMap;

use common_entity::{
    CommandUseCase6, CommandWithGivenState, Entity, EntityReplayableEvent, EventProjectError,
    IssuedByParty, MiStateMachineOwned, ReplayableChanges, UpdatedEntityPair,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry::{BalanceLedgerReason, SpotSettlementLeg};
use crate::entity::account::balance_ledger_entry_v2::BalanceLedgerEntryV2Error;
use crate::entity::spot::{
    CancelSpotOrderCmd as EntityCancelSpotOrderCmd, MatchSpotOrderCmd as EntityMatchSpotOrderCmd,
    PlaceSpotOrderCmd as EntityPlaceSpotOrderCmd, SpotOrderMiChanges, SpotOrderMiCommand,
    SpotOrderMiStateMachineError,
};
use crate::entity::{
    Balance, BalanceLedgerEntryV2, SpotOrder, SpotOrderStatus, SpotOrderTimeInForce, SpotTrade,
};

/// `SpotOrder` V6 总命令：只保留完整下单与撤单两个分支。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpotOrderUc6Cmd {
    Place(PlaceSpotOrderUc6Cmd),
    Cancel(CancelSpotOrderUc6Cmd),
}

impl IssuedByParty for SpotOrderUc6Cmd {
    fn party_id(&self) -> Option<&str> {
        match self {
            Self::Place(cmd) => cmd.party_id(),
            Self::Cancel(cmd) => cmd.party_id(),
        }
    }
}

impl CommandWithGivenState for SpotOrderUc6Cmd {
    type GivenState = SpotOrderUc6State;
}

/// 完整下单命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceSpotOrderUc6Cmd {
    pub party_id: String,
    pub match_id: String,
    pub settlement_batch_id: String,
}

impl IssuedByParty for PlaceSpotOrderUc6Cmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 撤单命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancelSpotOrderUc6Cmd {
    pub party_id: String,
}

impl IssuedByParty for CancelSpotOrderUc6Cmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// V6 双命令分支的 GivenState。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderUc6State {
    Place(PlaceSpotOrderUc6State),
    Cancel(CancelSpotOrderUc6State),
}

/// `Place` 分支已加载状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderUc6State {
    //todo 这个是创建出来的
    pub taker_order: SpotOrder,
    pub taker_base_balance: Balance,
    pub taker_quote_balance: Balance,
    pub taker_reservation_balance: Balance,
    pub maker_orders: Vec<SpotOrder>,
    pub settlement_balances: Vec<Balance>,
}

/// `Cancel` 分支已加载状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderUc6State {
    pub order: SpotOrder,
    /// 当前订单剩余冻结对应的单条余额快照。
    pub release_balance: Balance,
}

/// `Place` 分支的统一业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderUc6Changes {
    pub updated_taker_order: UpdatedEntityPair<SpotOrder>,
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrder>>,
    pub created_trades: Vec<SpotTrade>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

/// `Cancel` 分支的统一业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderUc6Changes {
    pub updated_order: UpdatedEntityPair<SpotOrder>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

/// V6 changes 总枚举。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotOrderUc6Changes {
    Place(PlaceSpotOrderUc6Changes),
    Cancel(CancelSpotOrderUc6Changes),
}

impl ReplayableChanges for PlaceSpotOrderUc6Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(
            1 + self.created_trades.len()
                + self.updated_maker_orders.len()
                + self.created_ledger_entries.len() * 2,
        );
        events.push(
            self.updated_taker_order
                .after
                .track_update_event_from(&self.updated_taker_order.before)?,
        );
        for (trade, maker) in self.created_trades.iter().zip(&self.updated_maker_orders) {
            events.push(trade.track_create_event()?);
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        events.extend(balance_replay_events_from_ledger_entries(
            &self.updated_balances,
            &self.created_ledger_entries,
        )?);
        for entry in &self.created_ledger_entries {
            events.push(entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl ReplayableChanges for CancelSpotOrderUc6Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events =
            Vec::with_capacity(1 + self.updated_balances.len() + self.created_ledger_entries.len());
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
        events.extend(balance_replay_events_from_ledger_entries(
            &self.updated_balances,
            &self.created_ledger_entries,
        )?);
        for entry in &self.created_ledger_entries {
            events.push(entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl ReplayableChanges for SpotOrderUc6Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        match self {
            Self::Place(changes) => changes.to_replayable_events(),
            Self::Cancel(changes) => changes.to_replayable_events(),
        }
    }
}

/// `SpotOrder` CommandUseCase6 入口。
#[derive(Debug, Clone, Copy, Default)]
pub struct SpotOrderUseCase6;

impl CommandUseCase6 for SpotOrderUseCase6 {
    type Command = SpotOrderUc6Cmd;
    type Error = SpotOrderUc6Error;
    type Changes = SpotOrderUc6Changes;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd {
            SpotOrderUc6Cmd::Place(cmd) => {
                if cmd.party_id.is_empty() {
                    return Err(SpotOrderUc6Error::InvalidPartyId);
                }
                if cmd.match_id.is_empty() {
                    return Err(SpotOrderUc6Error::InvalidMatchId);
                }
                if cmd.settlement_batch_id.is_empty() {
                    return Err(SpotOrderUc6Error::InvalidSettlementBatchId);
                }
                Ok(())
            }
            SpotOrderUc6Cmd::Cancel(cmd) => {
                if cmd.party_id.is_empty() {
                    return Err(SpotOrderUc6Error::InvalidPartyId);
                }
                Ok(())
            }
        }
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<(), Self::Error> {
        match (cmd, state) {
            (SpotOrderUc6Cmd::Place(cmd), SpotOrderUc6State::Place(state)) => {
                if !state.taker_order.belongs_to_account(&cmd.party_id) {
                    return Err(SpotOrderUc6Error::OrderOwnerMismatch);
                }
                if !state.taker_base_balance.belongs_to_account(&cmd.party_id)
                    || !state.taker_quote_balance.belongs_to_account(&cmd.party_id)
                    || !state.taker_reservation_balance.belongs_to_account(&cmd.party_id)
                {
                    return Err(SpotOrderUc6Error::BalanceOwnerMismatch);
                }
                Ok(())
            }
            (SpotOrderUc6Cmd::Cancel(cmd), SpotOrderUc6State::Cancel(state)) => {
                if !state.order.belongs_to_account(&cmd.party_id) {
                    return Err(SpotOrderUc6Error::OrderOwnerMismatch);
                }
                if !state.release_balance.belongs_to_account(&cmd.party_id) {
                    return Err(SpotOrderUc6Error::BalanceOwnerMismatch);
                }
                Ok(())
            }
            (SpotOrderUc6Cmd::Place(_), SpotOrderUc6State::Cancel(_)) => {
                Err(SpotOrderUc6Error::BranchMismatch {
                    command_kind: "place",
                    state_kind: "cancel",
                })
            }
            (SpotOrderUc6Cmd::Cancel(_), SpotOrderUc6State::Place(_)) => {
                Err(SpotOrderUc6Error::BranchMismatch {
                    command_kind: "cancel",
                    state_kind: "place",
                })
            }
        }
    }

    fn compute_before_after_changes(
        &self,
        cmd: &Self::Command,
        state: &<Self::Command as CommandWithGivenState>::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        match (cmd, state) {
            (SpotOrderUc6Cmd::Place(cmd), SpotOrderUc6State::Place(state)) => {
                Ok(SpotOrderUc6Changes::Place(compute_place_changes(cmd, state)?))
            }
            (SpotOrderUc6Cmd::Cancel(cmd), SpotOrderUc6State::Cancel(state)) => {
                Ok(SpotOrderUc6Changes::Cancel(compute_cancel_changes(cmd, state)?))
            }
            (SpotOrderUc6Cmd::Place(_), SpotOrderUc6State::Cancel(_)) => {
                Err(SpotOrderUc6Error::BranchMismatch {
                    command_kind: "place",
                    state_kind: "cancel",
                })
            }
            (SpotOrderUc6Cmd::Cancel(_), SpotOrderUc6State::Place(_)) => {
                Err(SpotOrderUc6Error::BranchMismatch {
                    command_kind: "cancel",
                    state_kind: "place",
                })
            }
        }
    }
}

/// V6 业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum SpotOrderUc6Error {
    #[error("party_id must not be empty")]
    InvalidPartyId,
    #[error("match_id must not be empty")]
    InvalidMatchId,
    #[error("settlement_batch_id must not be empty")]
    InvalidSettlementBatchId,
    #[error("branch mismatch: command={command_kind}, state={state_kind}")]
    BranchMismatch { command_kind: &'static str, state_kind: &'static str },
    #[error("order does not belong to command party")]
    OrderOwnerMismatch,
    #[error("balance snapshot does not belong to command party")]
    BalanceOwnerMismatch,
    #[error("conflicting balance snapshots for {balance_id}")]
    ConflictingBalanceSnapshot { balance_id: String },
    #[error("settlement balance was not found for account={account_id}, asset={asset_id}")]
    SettlementBalanceNotFound { account_id: String, asset_id: String },
    #[error("cancel release balance has no frozen amount")]
    MissingReleaseBalanceFrozen,
    #[error("balance transition was rejected for account={account_id}, asset={asset_id}")]
    BalanceTransitionRejected { account_id: String, asset_id: String },
    #[error("arithmetic overflow while deriving V6 spot order changes")]
    ArithmeticOverflow,
    #[error(transparent)]
    SpotOrderStateMachine(#[from] SpotOrderMiStateMachineError),
}

fn compute_place_changes(
    cmd: &PlaceSpotOrderUc6Cmd,
    state: &PlaceSpotOrderUc6State,
) -> Result<PlaceSpotOrderUc6Changes, SpotOrderUc6Error> {
    // `Place` 先只推进 taker 自身状态机，完成下单时的首笔冻结，
    // 拿到后续撮合流水要继续叠加的初始 changes。
    let place_changes = MiStateMachineOwned::compute_after_changes(
        &state.taker_order,
        &SpotOrderMiCommand::Place(EntityPlaceSpotOrderCmd {
            taker_base_balance: state.taker_base_balance.clone(),
            taker_quote_balance: state.taker_quote_balance.clone(),
        }),
        &(),
    )?;
    let SpotOrderMiChanges::Place(place_changes) = place_changes else {
        return Err(SpotOrderUc6Error::SpotOrderStateMachine(
            SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "place branch produced non-place changes",
            },
        ));
    };

    let mut all_ledger_entries = place_changes.created_ledger_entries.clone();
    // 余额上下文统一放进当前视图里维护：
    // 一份保留原始快照用于最终 before/after 对比，一份持续吸收冻结、成交、释放后的最新余额。
    let mut current_balances = build_original_balance_map(
        &state.taker_base_balance,
        &state.taker_quote_balance,
        &state.taker_reservation_balance,
        &state.settlement_balances,
    )?;
    let original_balances = current_balances.clone();
    let freeze_balance_after = place_changes
        .updated_balances
        .first()
        .ok_or(SpotOrderUc6Error::ArithmeticOverflow)?
        .after
        .clone();
    let freeze_balance_id = freeze_balance_after.entity_id();
    // 吸收 `Place` 阶段已经产生的首笔余额变化，后续所有结算都基于这个 authoritative 余额视图继续滚动。
    current_balances.insert(freeze_balance_id.clone(), freeze_balance_after);
    let mut touched_balance_ids = vec![freeze_balance_id];

    let mut updated_taker_order = UpdatedEntityPair {
        before: state.taker_order.clone(),
        after: place_changes.updated_order.after.clone(),
    };
    let mut updated_maker_orders = Vec::new();
    let mut created_trades = Vec::new();

    // 只有订单在冻结后仍满足进入撮合条件时，才继续走 maker 撮合分支；
    // 否则这里直接保留纯 place 结果，后续只考虑终态释放。
    if place_changes
        .updated_order
        .after
        .should_enter_matching(state.maker_orders.first())
        .map_err(SpotOrderMiStateMachineError::from)?
    {
        let match_changes = MiStateMachineOwned::compute_after_changes(
            &place_changes.updated_order.after,
            &SpotOrderMiCommand::Match(EntityMatchSpotOrderCmd {
                match_id: cmd.match_id.clone(),
                makers: state.maker_orders.clone(),
            }),
            &(),
        )?;
        let SpotOrderMiChanges::Match(match_changes) = match_changes else {
            return Err(SpotOrderUc6Error::SpotOrderStateMachine(
                SpotOrderMiStateMachineError::InvalidCommandFields {
                    reason: "place branch produced non-match changes",
                },
            ));
        };

        // 撮合状态机给出 taker/maker/trade 三类结果，这里把它们吸收到 place 主流水里，
        // 后续统一按 trade 逐笔做资金结算。
        updated_taker_order.after = match_changes.updated_taker_order.after;
        updated_maker_orders =
            match_changes.updated_maker_orders.into_iter().map(convert_pair).collect();
        created_trades = match_changes.created_trades;

        // 每笔成交都要拆成标准现货结算分录，分别更新买卖双方的 base/quote 余额。
        for trade in &created_trades {
            apply_settlement_trade(
                cmd,
                trade,
                &mut current_balances,
                &mut touched_balance_ids,
                &mut all_ledger_entries,
                &state.taker_base_balance.asset_id,
                &state.taker_quote_balance.asset_id,
            )?;
        }
    }

    // place 流水结束后，如果订单终态要求把剩余冻结释放回来
    // （例如 IOC 未完全成交），就在这里补一笔统一的 reservation release。
    if updated_taker_order.after.should_release_reservation_after_place() {
        release_terminal_reservation(
            &updated_taker_order.after,
            &state.taker_reservation_balance,
            &mut current_balances,
            &mut touched_balance_ids,
            &mut all_ledger_entries,
        )?;
    }

    // 无论中间经历了多少次冻结、撮合和释放，订单最终 authoritative after
    // 都统一通过 place pipeline 投影一次，确保 case 级输出口径一致。
    updated_taker_order.after = updated_taker_order
        .after
        .project_authoritative_after_place_pipeline(&updated_taker_order.before)?;

    // 最后只从被触达过的余额里提炼真正发生 before/after 差异的条目，
    // 组合成 `Place` 分支的统一 changes 输出。
    let updated_balances =
        collect_updated_balances(&touched_balance_ids, &original_balances, &current_balances)?;

    Ok(PlaceSpotOrderUc6Changes {
        updated_taker_order,
        updated_maker_orders,
        created_trades,
        updated_balances,
        created_ledger_entries: all_ledger_entries,
    })
}

fn compute_cancel_changes(
    _cmd: &CancelSpotOrderUc6Cmd,
    state: &CancelSpotOrderUc6State,
) -> Result<CancelSpotOrderUc6Changes, SpotOrderUc6Error> {
    let cancel_changes = MiStateMachineOwned::compute_after_changes(
        &state.order,
        &SpotOrderMiCommand::Cancel(EntityCancelSpotOrderCmd),
        &(),
    )?;
    let SpotOrderMiChanges::Cancel(cancel_changes) = cancel_changes else {
        return Err(SpotOrderUc6Error::SpotOrderStateMachine(
            SpotOrderMiStateMachineError::InvalidCommandFields {
                reason: "cancel branch produced non-cancel changes",
            },
        ));
    };

    let release_amount = state.release_balance.frozen;
    if release_amount == 0 {
        return Err(SpotOrderUc6Error::MissingReleaseBalanceFrozen);
    }

    let mut after_balance = state.release_balance.clone();
    let entry = BalanceLedgerEntryV2::unfreeze(
        format!(
            "balance-ledger:cancel:{}:{}",
            cancel_changes.updated_order.after.order_id,
            after_balance.entity_id()
        ),
        &mut after_balance,
        release_amount,
        BalanceLedgerReason::UnfreezeForCancel {
            order_id: cancel_changes.updated_order.after.order_id.clone(),
        },
    )
    .map_err(|error| map_balance_error(&state.release_balance, error))?;

    Ok(CancelSpotOrderUc6Changes {
        updated_order: convert_pair(cancel_changes.updated_order),
        updated_balances: vec![UpdatedEntityPair {
            before: state.release_balance.clone(),
            after: after_balance,
        }],
        created_ledger_entries: vec![entry],
    })
}

fn build_original_balance_map(
    taker_base_balance: &Balance,
    taker_quote_balance: &Balance,
    taker_reservation_balance: &Balance,
    settlement_balances: &[Balance],
) -> Result<HashMap<String, Balance>, SpotOrderUc6Error> {
    let mut balances = HashMap::new();
    insert_balance_snapshot(&mut balances, taker_base_balance.clone())?;
    insert_balance_snapshot(&mut balances, taker_quote_balance.clone())?;
    insert_balance_snapshot(&mut balances, taker_reservation_balance.clone())?;
    for balance in settlement_balances {
        insert_balance_snapshot(&mut balances, balance.clone())?;
    }
    Ok(balances)
}

fn insert_balance_snapshot(
    balances: &mut HashMap<String, Balance>,
    balance: Balance,
) -> Result<(), SpotOrderUc6Error> {
    let balance_id = balance.entity_id();
    if let Some(existing) = balances.get(&balance_id) {
        if existing != &balance {
            return Err(SpotOrderUc6Error::ConflictingBalanceSnapshot { balance_id });
        }
        return Ok(());
    }
    balances.insert(balance_id, balance);
    Ok(())
}

fn apply_settlement_trade(
    cmd: &PlaceSpotOrderUc6Cmd,
    trade: &SpotTrade,
    balances: &mut HashMap<String, Balance>,
    touched_balance_ids: &mut Vec<String>,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    base_asset_id: &str,
    quote_asset_id: &str,
) -> Result<(), SpotOrderUc6Error> {
    // 一笔现货成交在账本上固定拆成四条腿：
    // 买方收 base、买方扣冻结 quote、卖方收 quote、卖方扣冻结 base。
    // 这里集中展开，保证 place 主流程只关心“逐笔结算 trade”。
    let quote_qty = trade.notional_quote().ok_or(SpotOrderUc6Error::ArithmeticOverflow)?;
    apply_balance_entry(
        balances,
        touched_balance_ids,
        ledger_entries,
        trade.buyer_account_id(),
        base_asset_id,
        trade.qty,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: trade.trade_id.clone(),
            match_id: trade.match_id.clone(),
            settlement_batch_id: cmd.settlement_batch_id.clone(),
            leg: SpotSettlementLeg::BuyerReceiveBase,
        },
        BalanceOperation::CreditAvailable,
        format!(
            "balance-ledger:{}:{}:{}",
            cmd.settlement_batch_id,
            trade.trade_id,
            SpotSettlementLeg::BuyerReceiveBase.as_str()
        ),
    )?;
    apply_balance_entry(
        balances,
        touched_balance_ids,
        ledger_entries,
        trade.buyer_account_id(),
        quote_asset_id,
        quote_qty,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: trade.trade_id.clone(),
            match_id: trade.match_id.clone(),
            settlement_batch_id: cmd.settlement_batch_id.clone(),
            leg: SpotSettlementLeg::BuyerDebitFrozenQuote,
        },
        BalanceOperation::DebitFrozen,
        format!(
            "balance-ledger:{}:{}:{}",
            cmd.settlement_batch_id,
            trade.trade_id,
            SpotSettlementLeg::BuyerDebitFrozenQuote.as_str()
        ),
    )?;
    apply_balance_entry(
        balances,
        touched_balance_ids,
        ledger_entries,
        trade.seller_account_id(),
        quote_asset_id,
        quote_qty,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: trade.trade_id.clone(),
            match_id: trade.match_id.clone(),
            settlement_batch_id: cmd.settlement_batch_id.clone(),
            leg: SpotSettlementLeg::SellerReceiveQuote,
        },
        BalanceOperation::CreditAvailable,
        format!(
            "balance-ledger:{}:{}:{}",
            cmd.settlement_batch_id,
            trade.trade_id,
            SpotSettlementLeg::SellerReceiveQuote.as_str()
        ),
    )?;
    apply_balance_entry(
        balances,
        touched_balance_ids,
        ledger_entries,
        trade.seller_account_id(),
        base_asset_id,
        trade.qty,
        BalanceLedgerReason::SettleSpotTrade {
            trade_id: trade.trade_id.clone(),
            match_id: trade.match_id.clone(),
            settlement_batch_id: cmd.settlement_batch_id.clone(),
            leg: SpotSettlementLeg::SellerDebitFrozenBase,
        },
        BalanceOperation::DebitFrozen,
        format!(
            "balance-ledger:{}:{}:{}",
            cmd.settlement_batch_id,
            trade.trade_id,
            SpotSettlementLeg::SellerDebitFrozenBase.as_str()
        ),
    )?;
    Ok(())
}

fn release_terminal_reservation(
    order: &SpotOrder,
    reservation_balance: &Balance,
    balances: &mut HashMap<String, Balance>,
    touched_balance_ids: &mut Vec<String>,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), SpotOrderUc6Error> {
    // 这里处理的是 place 流水收尾时的“剩余冻结释放”，
    // 它复用 unfreeze 账本动作，但业务上不是一条独立 cancel 命令。
    let release_balance_id = reservation_balance.entity_id();
    let current_balance = balances.get(&release_balance_id).ok_or_else(|| {
        SpotOrderUc6Error::SettlementBalanceNotFound {
            account_id: reservation_balance.account_id.clone(),
            asset_id: reservation_balance.asset_id.clone(),
        }
    })?;
    let release_amount = current_balance.frozen.min(order.release_amount_after_place());
    if release_amount == 0 {
        return Ok(());
    }

    apply_balance_entry(
        balances,
        touched_balance_ids,
        ledger_entries,
        &reservation_balance.account_id,
        &reservation_balance.asset_id,
        release_amount,
        BalanceLedgerReason::UnfreezeForCancel { order_id: order.order_id.clone() },
        BalanceOperation::Unfreeze,
        format!("balance-ledger:release:{}:{}", order.order_id, release_balance_id),
    )
}

#[derive(Debug, Clone, Copy)]
enum BalanceOperation {
    CreditAvailable,
    DebitFrozen,
    Unfreeze,
}

fn apply_balance_entry(
    balances: &mut HashMap<String, Balance>,
    touched_balance_ids: &mut Vec<String>,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
    account_id: &str,
    asset_id: &str,
    amount: u64,
    reason: BalanceLedgerReason,
    operation: BalanceOperation,
    entry_id: String,
) -> Result<(), SpotOrderUc6Error> {
    // 余额账本分录统一从这里入账：
    // 负责定位目标余额、执行借贷/解冻动作，并记录这次 place 流水真正触达过哪些余额。
    let balance_id = format!("{account_id}:{asset_id}");
    let balance = balances.get_mut(&balance_id).ok_or_else(|| {
        SpotOrderUc6Error::SettlementBalanceNotFound {
            account_id: account_id.to_string(),
            asset_id: asset_id.to_string(),
        }
    })?;
    let entry = match operation {
        BalanceOperation::CreditAvailable => {
            BalanceLedgerEntryV2::credit_available(entry_id, balance, amount, reason)
        }
        BalanceOperation::DebitFrozen => {
            BalanceLedgerEntryV2::debit_frozen(entry_id, balance, amount, reason)
        }
        BalanceOperation::Unfreeze => {
            BalanceLedgerEntryV2::unfreeze(entry_id, balance, amount, reason)
        }
    }
    .map_err(|error| map_balance_error(balance, error))?;

    if !touched_balance_ids.iter().any(|existing| existing == &balance_id) {
        touched_balance_ids.push(balance_id);
    }
    ledger_entries.push(entry);
    Ok(())
}

fn map_balance_error(balance: &Balance, error: BalanceLedgerEntryV2Error) -> SpotOrderUc6Error {
    match error {
        BalanceLedgerEntryV2Error::ArithmeticOverflow => SpotOrderUc6Error::ArithmeticOverflow,
        BalanceLedgerEntryV2Error::InvalidAmount
        | BalanceLedgerEntryV2Error::InsufficientAvailableBalance
        | BalanceLedgerEntryV2Error::InsufficientFrozenBalance => {
            SpotOrderUc6Error::BalanceTransitionRejected {
                account_id: balance.account_id.clone(),
                asset_id: balance.asset_id.clone(),
            }
        }
    }
}

fn collect_updated_balances(
    touched_balance_ids: &[String],
    original_balances: &HashMap<String, Balance>,
    current_balances: &HashMap<String, Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, SpotOrderUc6Error> {
    // 账本滚动过程中只维护当前视图；直到收口时，
    // 才从原始快照和当前视图里提炼真正变化过的余额 before/after 对。
    let mut updated_balances = Vec::new();
    for balance_id in touched_balance_ids {
        let before = original_balances
            .get(balance_id)
            .cloned()
            .ok_or(SpotOrderUc6Error::ArithmeticOverflow)?;
        let after = current_balances
            .get(balance_id)
            .cloned()
            .ok_or(SpotOrderUc6Error::ArithmeticOverflow)?;
        if before != after {
            updated_balances.push(UpdatedEntityPair { before, after });
        }
    }
    Ok(updated_balances)
}

fn convert_pair<T>(
    pair: cmd_handler::command_use_case_def2::UpdatedEntityPair<T>,
) -> UpdatedEntityPair<T> {
    UpdatedEntityPair { before: pair.before, after: pair.after }
}

fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerEntryV2],
) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
    let mut current_balances = HashMap::<String, Balance>::with_capacity(updated_balances.len());
    let mut expected_after_balances =
        HashMap::<String, Balance>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.entity_id();
        current_balances.insert(balance_id.clone(), balance.before.clone());
        expected_after_balances.insert(balance_id, balance.after.clone());
    }

    let mut events = Vec::with_capacity(ledger_entries.len());
    for entry in ledger_entries {
        let Some(before) = current_balances.get(&entry.balance_entity_id).cloned() else {
            return Err(EventProjectError::Custom(
                "balance ledger entry does not belong to updated balances".to_string(),
            ));
        };
        if before.available != entry.before_available || before.frozen != entry.before_frozen {
            return Err(EventProjectError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let next_version = before
            .version
            .checked_add(1)
            .ok_or(EventProjectError::VersionOverflow { version: before.version })?;
        let after = Balance::new(
            before.account_id.clone(),
            before.asset_id.clone(),
            entry.after_available,
            entry.after_frozen,
            next_version,
        );
        events.push(after.track_update_event_from(&before)?);
        current_balances.insert(entry.balance_entity_id.clone(), after);
    }

    for (balance_id, expected_after) in expected_after_balances {
        if current_balances.get(&balance_id) != Some(&expected_after) {
            return Err(EventProjectError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    Ok(events)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use common_entity::{
        CommandEnvelope, CommandMeta, CommandUseCaseExecutor6, CommandUseCaseOutbound,
        CommandUseCaseOutboundPhase, HandlerLatencyMetrics, ObserveHandlerLatency,
    };

    use super::*;
    use crate::entity::{SpotOrderExecution, SpotOrderSide};

    fn taker_buy_order(tif: SpotOrderTimeInForce) -> SpotOrder {
        SpotOrder::new(
            "taker-1".to_string(),
            10_001,
            Some(11),
            "buyer".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Buy,
            SpotOrderExecution::Limit { price: 100 },
            tif,
            2,
            0,
            200,
            None,
        )
    }

    fn maker_sell_order(qty: u64, price: u64) -> SpotOrder {
        SpotOrder::new(
            "maker-1".to_string(),
            10_001,
            Some(12),
            "seller".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            qty,
            0,
            None,
        )
    }

    fn buyer_base_balance() -> Balance {
        Balance::new("buyer".to_string(), "BTC".to_string(), 0, 0, 1)
    }

    fn buyer_quote_balance() -> Balance {
        Balance::new("buyer".to_string(), "USDT".to_string(), 1_000, 0, 1)
    }

    fn seller_base_balance(frozen: u64) -> Balance {
        Balance::new("seller".to_string(), "BTC".to_string(), 0, frozen, 1)
    }

    fn seller_quote_balance() -> Balance {
        Balance::new("seller".to_string(), "USDT".to_string(), 0, 0, 1)
    }

    fn place_cmd() -> SpotOrderUc6Cmd {
        SpotOrderUc6Cmd::Place(PlaceSpotOrderUc6Cmd {
            party_id: "buyer".to_string(),
            match_id: "match-1".to_string(),
            settlement_batch_id: "settle-1".to_string(),
        })
    }

    fn cancel_cmd() -> SpotOrderUc6Cmd {
        SpotOrderUc6Cmd::Cancel(CancelSpotOrderUc6Cmd { party_id: "buyer".to_string() })
    }

    fn place_state_with_one_maker() -> SpotOrderUc6State {
        SpotOrderUc6State::Place(PlaceSpotOrderUc6State {
            taker_order: taker_buy_order(SpotOrderTimeInForce::Ioc),
            taker_base_balance: buyer_base_balance(),
            taker_quote_balance: buyer_quote_balance(),
            taker_reservation_balance: buyer_quote_balance(),
            maker_orders: vec![maker_sell_order(2, 90)],
            settlement_balances: vec![seller_base_balance(2), seller_quote_balance()],
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
    fn place_happy_path_freezes_matches_settles_and_advances_final_status() {
        let use_case = SpotOrderUseCase6;

        // Rule:
        // - `Place` 在一个分支内完成冻结、撮合、结算与主订单终态推进。
        //
        // Given:
        // - IOC taker buy order 有一张完全成交的 maker sell order。
        //
        // When:
        // - 计算 V6 changes，并投影 replayable events。
        //
        // Then:
        // - taker 最终进入 `Filled`。
        // - 生成 trade、maker update、结算余额变化和余额流水。

        // arrange
        let cmd = place_cmd();
        let state = place_state_with_one_maker();

        // act
        let changes = use_case.compute_before_after_changes(&cmd, &state).unwrap();
        let events = changes.to_replayable_events().unwrap();

        // assert
        let SpotOrderUc6Changes::Place(changes) = changes else {
            panic!("expected place changes");
        };
        assert_eq!(changes.updated_taker_order.after.status, SpotOrderStatus::Filled);
        assert_eq!(changes.updated_taker_order.after.filled_qty, 2);
        assert_eq!(changes.updated_taker_order.after.version, 2);
        assert_eq!(changes.updated_maker_orders.len(), 1);
        assert_eq!(changes.created_trades.len(), 1);
        assert_eq!(changes.updated_balances.len(), 4);
        assert_eq!(changes.created_ledger_entries.len(), 6);
        assert_eq!(events.len(), 15);
        assert_eq!(event_field(&events[0], "status"), Some("filled"));
        assert_eq!(event_field(&events[1], "trade_id"), Some("match-1-1"));
        assert_eq!(event_field(&events[2], "filled_qty"), Some("2"));
        assert_eq!(event_field(&events[2], "status"), Some("filled"));
        assert_eq!(event_field(&events[3], "asset_id"), Some("USDT"));
        assert_eq!(event_field(&events[9], "reason"), Some("freeze_for_order"));
        assert_eq!(event_field(&events[14], "reason"), Some("unfreeze_for_cancel"));
    }

    #[test]
    fn place_without_crossing_maker_keeps_gtc_order_open() {
        let use_case = SpotOrderUseCase6;
        let cmd = SpotOrderUc6Cmd::Place(PlaceSpotOrderUc6Cmd {
            party_id: "buyer".to_string(),
            match_id: "match-2".to_string(),
            settlement_batch_id: "settle-2".to_string(),
        });
        let state = SpotOrderUc6State::Place(PlaceSpotOrderUc6State {
            taker_order: taker_buy_order(SpotOrderTimeInForce::Gtc),
            taker_base_balance: buyer_base_balance(),
            taker_quote_balance: buyer_quote_balance(),
            taker_reservation_balance: buyer_quote_balance(),
            maker_orders: vec![maker_sell_order(2, 120)],
            settlement_balances: vec![seller_base_balance(2), seller_quote_balance()],
        });

        let changes = use_case.compute_before_after_changes(&cmd, &state).unwrap();

        let SpotOrderUc6Changes::Place(changes) = changes else {
            panic!("expected place changes");
        };
        assert_eq!(changes.updated_taker_order.after.status, SpotOrderStatus::Open);
        assert_eq!(changes.updated_taker_order.after.version, 2);
        assert!(changes.created_trades.is_empty());
        assert_eq!(changes.updated_balances.len(), 1);
        assert_eq!(changes.created_ledger_entries.len(), 1);
    }

    #[test]
    fn branch_mismatch_returns_business_error() {
        let use_case = SpotOrderUseCase6;

        let state = SpotOrderUc6State::Cancel(CancelSpotOrderUc6State {
            order: taker_buy_order(SpotOrderTimeInForce::Gtc),
            release_balance: buyer_quote_balance(),
        });
        let result = use_case.compute_before_after_changes(&place_cmd(), &state);

        assert_eq!(
            result,
            Err(SpotOrderUc6Error::BranchMismatch { command_kind: "place", state_kind: "cancel" })
        );
    }

    #[test]
    fn insufficient_balance_returns_business_error() {
        let use_case = SpotOrderUseCase6;
        let state = SpotOrderUc6State::Place(PlaceSpotOrderUc6State {
            taker_order: taker_buy_order(SpotOrderTimeInForce::Ioc),
            taker_base_balance: buyer_base_balance(),
            taker_quote_balance: Balance::new("buyer".to_string(), "USDT".to_string(), 10, 0, 1),
            taker_reservation_balance: Balance::new(
                "buyer".to_string(),
                "USDT".to_string(),
                10,
                0,
                1,
            ),
            maker_orders: vec![],
            settlement_balances: vec![],
        });

        let result = use_case.compute_before_after_changes(&place_cmd(), &state);

        assert_eq!(
            result,
            Err(SpotOrderUc6Error::SpotOrderStateMachine(
                SpotOrderMiStateMachineError::InsufficientFreezeBalance
            ))
        );
    }

    #[test]
    fn cancel_happy_path_cancels_order_and_unfreezes_remaining_balance() {
        let use_case = SpotOrderUseCase6;
        let order = taker_buy_order(SpotOrderTimeInForce::Gtc);
        let state = SpotOrderUc6State::Cancel(CancelSpotOrderUc6State {
            order,
            release_balance: Balance::new("buyer".to_string(), "USDT".to_string(), 800, 200, 1),
        });

        let changes = use_case.compute_before_after_changes(&cancel_cmd(), &state).unwrap();
        let events = changes.to_replayable_events().unwrap();

        let SpotOrderUc6Changes::Cancel(changes) = changes else {
            panic!("expected cancel changes");
        };
        assert_eq!(changes.updated_order.after.status, SpotOrderStatus::Canceled);
        assert_eq!(changes.updated_balances.len(), 1);
        assert_eq!(changes.updated_balances[0].after.available, 1_000);
        assert_eq!(changes.updated_balances[0].after.frozen, 0);
        assert_eq!(changes.created_ledger_entries.len(), 1);
        assert_eq!(
            changes.created_ledger_entries[0].reason,
            BalanceLedgerReason::UnfreezeForCancel {
                order_id: changes.updated_order.after.order_id.clone(),
            }
        );
        assert_eq!(events.len(), 3);
        assert_eq!(event_field(&events[0], "status"), Some("canceled"));
        assert_eq!(event_field(&events[2], "reason"), Some("unfreeze_for_cancel"));
    }

    #[test]
    fn place_ioc_partial_fill_releases_remaining_reservation() {
        let use_case = SpotOrderUseCase6;
        let state = SpotOrderUc6State::Place(PlaceSpotOrderUc6State {
            taker_order: taker_buy_order(SpotOrderTimeInForce::Ioc),
            taker_base_balance: buyer_base_balance(),
            taker_quote_balance: buyer_quote_balance(),
            taker_reservation_balance: buyer_quote_balance(),
            maker_orders: vec![maker_sell_order(1, 90)],
            settlement_balances: vec![seller_base_balance(1), seller_quote_balance()],
        });

        let changes = use_case.compute_before_after_changes(&place_cmd(), &state).unwrap();

        let SpotOrderUc6Changes::Place(changes) = changes else {
            panic!("expected place changes");
        };
        assert_eq!(changes.updated_taker_order.after.status, SpotOrderStatus::Canceled);
        assert_eq!(changes.updated_taker_order.after.filled_qty, 1);
        assert_eq!(changes.updated_taker_order.after.version, 2);
        assert_eq!(changes.created_trades.len(), 1);
        assert_eq!(changes.created_ledger_entries.len(), 6);
        let taker_quote = changes
            .updated_balances
            .iter()
            .find(|pair| pair.after.account_id == "buyer" && pair.after.asset_id == "USDT")
            .unwrap();
        assert_eq!(taker_quote.after.available, 910);
        assert_eq!(taker_quote.after.frozen, 0);
    }

    #[test]
    fn place_ioc_without_match_releases_full_reservation() {
        let use_case = SpotOrderUseCase6;
        let state = SpotOrderUc6State::Place(PlaceSpotOrderUc6State {
            taker_order: taker_buy_order(SpotOrderTimeInForce::Ioc),
            taker_base_balance: buyer_base_balance(),
            taker_quote_balance: buyer_quote_balance(),
            taker_reservation_balance: buyer_quote_balance(),
            maker_orders: vec![],
            settlement_balances: vec![],
        });

        let changes = use_case.compute_before_after_changes(&place_cmd(), &state).unwrap();

        let SpotOrderUc6Changes::Place(changes) = changes else {
            panic!("expected place changes");
        };
        assert_eq!(changes.updated_taker_order.after.status, SpotOrderStatus::Rejected);
        assert_eq!(changes.updated_taker_order.after.version, 2);
        assert!(changes.created_trades.is_empty());
        assert_eq!(changes.updated_balances.len(), 1);
        assert_eq!(changes.created_ledger_entries.len(), 2);
        assert_eq!(changes.updated_balances[0].after.available, 1_000);
        assert_eq!(changes.updated_balances[0].after.frozen, 0);
    }

    #[derive(Debug, Default)]
    struct StubObserver {
        observed: Mutex<Vec<HandlerLatencyMetrics>>,
    }

    impl ObserveHandlerLatency for StubObserver {
        fn observe_latency(&self, metrics: &HandlerLatencyMetrics) {
            self.observed.lock().unwrap().push(*metrics);
        }
    }

    #[derive(Debug)]
    struct StubOutbound {
        state: Mutex<SpotOrderUc6State>,
        calls: Mutex<Vec<&'static str>>,
    }

    impl StubOutbound {
        fn new(state: SpotOrderUc6State) -> Self {
            Self { state: Mutex::new(state), calls: Mutex::new(Vec::new()) }
        }

        fn calls(&self) -> Vec<&'static str> {
            self.calls.lock().unwrap().clone()
        }

        fn record(&self, phase: &'static str) {
            self.calls.lock().unwrap().push(phase);
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Error)]
    #[error("outbound failure")]
    struct StubOutboundError;

    impl CommandUseCaseOutbound for StubOutbound {
        type Command = SpotOrderUc6Cmd;
        type State = SpotOrderUc6State;
        type Error = StubOutboundError;

        fn load_state(&self, _cmd: &Self::Command) -> Result<Self::State, Self::Error> {
            self.record("load_state");
            Ok(self.state.lock().unwrap().clone())
        }

        fn persist(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record("persist");
            Ok(())
        }

        fn replay(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record("replay");
            Ok(())
        }

        fn publish(&self, _events: &[EntityReplayableEvent]) -> Result<(), Self::Error> {
            self.record("publish");
            Ok(())
        }
    }

    #[test]
    fn executor6_place_happy_path_runs_full_pipeline() {
        let executor = CommandUseCaseExecutor6;
        let outbound = StubOutbound::new(place_state_with_one_maker());
        let observer = StubObserver::default();

        let result = executor
            .execute(
                &SpotOrderUseCase6,
                CommandEnvelope {
                    meta: CommandMeta {
                        trace_id: Some("trace-place".to_string()),
                        command_id: Some("cmd-place".to_string()),
                    },
                    command: place_cmd(),
                },
                &outbound,
                &observer,
            )
            .unwrap();

        assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
        assert_eq!(observer.observed.lock().unwrap().len(), 1);
        assert!(!result.events.is_empty());
    }

    #[test]
    fn executor6_cancel_happy_path_runs_full_pipeline() {
        let executor = CommandUseCaseExecutor6;
        let outbound = StubOutbound::new(SpotOrderUc6State::Cancel(CancelSpotOrderUc6State {
            order: taker_buy_order(SpotOrderTimeInForce::Gtc),
            release_balance: Balance::new("buyer".to_string(), "USDT".to_string(), 800, 200, 1),
        }));
        let observer = StubObserver::default();

        let result = executor
            .execute(
                &SpotOrderUseCase6,
                CommandEnvelope { meta: CommandMeta::default(), command: cancel_cmd() },
                &outbound,
                &observer,
            )
            .unwrap();

        assert!(!result.events.is_empty());
        assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct BrokenChanges(PlaceSpotOrderUc6Changes);

    impl ReplayableChanges for BrokenChanges {
        fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
            self.0.to_replayable_events()
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct BrokenUseCase;

    impl CommandUseCase6 for BrokenUseCase {
        type Command = SpotOrderUc6Cmd;
        type Error = SpotOrderUc6Error;
        type Changes = BrokenChanges;

        fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
            SpotOrderUseCase6.pre_check_command(cmd)
        }

        fn validate_against_state(
            &self,
            cmd: &Self::Command,
            state: &<Self::Command as CommandWithGivenState>::GivenState,
        ) -> Result<(), Self::Error> {
            SpotOrderUseCase6.validate_against_state(cmd, state)
        }

        fn compute_before_after_changes(
            &self,
            cmd: &Self::Command,
            state: &<Self::Command as CommandWithGivenState>::GivenState,
        ) -> Result<Self::Changes, Self::Error> {
            let SpotOrderUc6Changes::Place(changes) =
                SpotOrderUseCase6.compute_before_after_changes(cmd, state)?
            else {
                unreachable!("broken test only exercises place branch");
            };
            Ok(BrokenChanges(changes))
        }
    }

    #[test]
    fn executor6_accepts_replayable_changes_without_main_mi_metadata() {
        let executor = CommandUseCaseExecutor6;
        let outbound = StubOutbound::new(place_state_with_one_maker());
        let observer = StubObserver::default();

        let result = executor
            .execute(
                &BrokenUseCase,
                CommandEnvelope { meta: CommandMeta::default(), command: place_cmd() },
                &outbound,
                &observer,
            )
            .unwrap();

        assert_eq!(result.events.len(), 15);
        assert_eq!(outbound.calls(), vec!["load_state", "persist", "replay", "publish"]);
    }

    #[test]
    fn outbound_phase_enum_keeps_expected_labels() {
        assert_eq!(CommandUseCaseOutboundPhase::LoadState.to_string(), "load_state");
    }
}
