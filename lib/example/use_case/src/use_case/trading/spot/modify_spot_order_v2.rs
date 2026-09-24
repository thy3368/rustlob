use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityError, EntityReplayableEvent, ExecutionContext, IssuedByParty, ReplayableChanges,
    StateMachineOwnedV2Diff, StateMachineV2Unchecked,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::account::balance_ledger_entry_v2::{
    BalanceLedgerEntryV2, BalanceLedgerEntryV2Error, BalanceLedgerOperation,
};
use crate::entity::account::balance_ledger_reason::BalanceLedgerReason;
use crate::entity::spot::spot_order_v2::{SpotOrderV2, SpotOrderV2BehaviorError};
use crate::entity::{
    Balance, ReservationError, SpotOrderSide, SpotOrderStatus, SpotOrderTif, SpotOrderType,
};
use crate::support::concat3;

/// 定位一个已经存在的订单。
///
/// 现货改单只支持用 Hyperliquid `cloid` 查找原订单；不再维护 numeric `oid`
/// 到本地订单的映射。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderId {
    Cloid(String),
}

/// Hyperliquid 现货改单的订单类型。
///
/// 两个变体分别对应 wire 层的 `t.limit` 和 `t.trigger`，因此不能同时
/// 表达普通限价单和条件单。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifySpotOrderV2OrderType {
    /// 普通限价单。
    Limit { tif: String },
    /// 条件单。
    Trigger { is_market: bool, trigger_price: String, trigger_role: String },
}

/// 现货订单完整替换命令。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModifySpotOrderV2Cmd {
    /// 发起改单的业务主体。
    pub party_id: String,
    /// Hyperliquid `order.a`。
    pub asset: u32,
    /// 顶层 `oid`，用于定位原订单。
    pub order_id: OrderId,
    /// Hyperliquid `order.b`。
    pub is_buy: bool,
    /// 修改后的完整价格，而不是价格增量。
    pub price: String,
    /// 修改后的完整数量，而不是数量增量。
    pub size: String,
    /// 修改后的完整订单类型。
    pub order_type: ModifySpotOrderV2OrderType,
    /// Hyperliquid `order.c`，表示修改后订单的 client order id。
    ///
    /// 为空时由后续业务逻辑保留原订单的 client order id。
    pub cloid: Option<String>,
}

impl IssuedByParty for ModifySpotOrderV2Cmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 现货改单所需的 authoritative 状态快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2State {
    pub order: SpotOrderV2,
    pub balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

/// 现货改单的纯业务计算结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2AfterChanges {
    pub order_after: SpotOrderV2,
    pub balances_after: Vec<Balance>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

/// 现货改单的 before/after 变化。
///
/// 订单 pair 和余额 pairs 分别是各自变化的唯一 authoritative truth；after
/// 快照不在此类型中重复保存。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerEntryV2>,
}

/// 现货限价订单改单业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ModifySpotOrderV2Error {
    #[error("party id must not be empty")]
    EmptyPartyId,
    #[error("order lookup is missing or invalid")]
    InvalidLookup,
    #[error("order lookup does not match the loaded order")]
    OrderLookupMismatch,
    #[error("command party does not match the loaded order party")]
    PartyMismatch,
    #[error("command asset does not match the loaded order asset")]
    AssetMismatch,
    #[error("only limit orders can be modified")]
    OnlyLimitOrderCanBeModified,
    #[error("order is not an open, unfilled order")]
    OrderNotModifiable,
    #[error("changing order side is not supported")]
    SideChangeUnsupported,
    #[error("price must be a positive integer string")]
    InvalidPrice,
    #[error("size must be a positive integer string")]
    InvalidSize,
    #[error("time in force must be gtc, ioc, or alo")]
    InvalidTimeInForce,
    #[error("balance not found")]
    BalanceNotFound,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("arithmetic overflow while computing spot order v2 modification")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
    #[error(transparent)]
    Reservation(#[from] ReservationError),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ModifySpotOrderV2UseCase;

impl ReplayableChanges for ModifySpotOrderV2Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let event_capacity = 1usize
            .saturating_add(self.created_balance_ledger_entries.len())
            .saturating_add(self.created_balance_ledger_entries.len());
        let mut events = Vec::with_capacity(event_capacity);
        events.push(self.updated_order.after.track_update_event_from(&self.updated_order.before)?);
        events.extend(balance_replay_events_from_ledger_entries(
            &self.updated_balances,
            &self.created_balance_ledger_entries,
        )?);
        for entry in &self.created_balance_ledger_entries {
            events.push(entry.track_create_event()?);
        }
        Ok(events)
    }
}

impl StateMachineV2Unchecked for ModifySpotOrderV2UseCase {
    type Command = ModifySpotOrderV2Cmd;
    type StateGiven = ModifySpotOrderV2State;
    type Error = ModifySpotOrderV2Error;
    type StateChanged = ModifySpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ModifySpotOrderV2Error::EmptyPartyId);
        }
        match &cmd.order_id {
            OrderId::Cloid(cloid) if !cloid.is_empty() => {}
            _ => return Err(ModifySpotOrderV2Error::InvalidLookup),
        }
        parse_positive_u64(&cmd.price, ModifySpotOrderV2Error::InvalidPrice)?;
        parse_positive_u64(&cmd.size, ModifySpotOrderV2Error::InvalidSize)?;
        match &cmd.order_type {
            ModifySpotOrderV2OrderType::Limit { tif } => {
                parse_tif(tif)?;
            }
            ModifySpotOrderV2OrderType::Trigger { .. } => {
                return Err(ModifySpotOrderV2Error::OnlyLimitOrderCanBeModified);
            }
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let order = &given_state.order;
        if cmd.party_id != order.account_id {
            return Err(ModifySpotOrderV2Error::PartyMismatch);
        }
        if cmd.asset != order.asset {
            return Err(ModifySpotOrderV2Error::AssetMismatch);
        }
        if !lookup_matches_order(&cmd.order_id, order) {
            return Err(ModifySpotOrderV2Error::OrderLookupMismatch);
        }
        if !matches!(order.order_type, SpotOrderType::Limit { .. }) {
            return Err(ModifySpotOrderV2Error::OnlyLimitOrderCanBeModified);
        }
        if order.status != SpotOrderStatus::Open || order.filled_qty != 0 {
            return Err(ModifySpotOrderV2Error::OrderNotModifiable);
        }
        if side_from_command(cmd) != order.side {
            return Err(ModifySpotOrderV2Error::SideChangeUnsupported);
        }
        Ok(())
    }

    fn compute_state_changed_unchecked(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
        _context: &ExecutionContext,
    ) -> Result<Self::StateChanged, Self::Error> {
        let (price, qty, tif) = parsed_limit_command(cmd)?;
        let side = side_from_command(cmd);
        let order = &given_state.order;
        let principal_after = SpotOrderV2::principal_reservation(
            order.order_id(),
            order.account_id(),
            side,
            qty,
            price,
            &given_state.base_asset_id,
            &given_state.quote_asset_id,
        )?;
        let fee_after = SpotOrderV2::fee_reservation(
            order.order_id(),
            order.account_id(),
            side,
            qty,
            price,
            &given_state.quote_asset_id,
            given_state.maker_fee_bps,
            given_state.taker_fee_bps,
        )?;

        let mut order_after = order.clone();
        order_after.limit_price = price;
        order_after.qty = qty;
        order_after.order_type = SpotOrderType::Limit { tif };
        order_after.client_order_id = cmd.cloid.clone().or_else(|| order.client_order_id.clone());
        order_after.reservation = principal_after;
        order_after.fee_reservation = fee_after;
        order_after.version =
            order.version.checked_add(1).ok_or(ModifySpotOrderV2Error::ArithmeticOverflow)?;

        let mut balance_book = BalanceMap::new(&given_state.balances);
        let mut created_balance_ledger_entries = Vec::with_capacity(2);
        adjust_reservation_balance(
            order,
            &order_after,
            ReservationSlot::Principal,
            &mut balance_book,
            &mut created_balance_ledger_entries,
        )?;
        adjust_reservation_balance(
            order,
            &order_after,
            ReservationSlot::Fee,
            &mut balance_book,
            &mut created_balance_ledger_entries,
        )?;

        Ok(ModifySpotOrderV2AfterChanges {
            order_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
        })
    }
}

impl StateMachineOwnedV2Diff for ModifySpotOrderV2UseCase {
    type StateDiff = ModifySpotOrderV2Changes;

    fn do_compute_state_diff(
        given_state: ModifySpotOrderV2State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(ModifySpotOrderV2Changes {
            updated_order: UpdatedEntityPair {
                before: given_state.order,
                after: after.order_after,
            },
            updated_balances: merge_balance_pairs(given_state.balances, after.balances_after)?,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
    }
}

fn lookup_matches_order(lookup: &OrderId, order: &SpotOrderV2) -> bool {
    match lookup {
        OrderId::Cloid(cloid) => order.client_order_id.as_deref() == Some(cloid.as_str()),
    }
}

fn side_from_command(cmd: &ModifySpotOrderV2Cmd) -> SpotOrderSide {
    if cmd.is_buy { SpotOrderSide::Buy } else { SpotOrderSide::Sell }
}

fn parsed_limit_command(
    cmd: &ModifySpotOrderV2Cmd,
) -> Result<(u64, u64, SpotOrderTif), ModifySpotOrderV2Error> {
    let price = parse_positive_u64(&cmd.price, ModifySpotOrderV2Error::InvalidPrice)?;
    let qty = parse_positive_u64(&cmd.size, ModifySpotOrderV2Error::InvalidSize)?;
    let ModifySpotOrderV2OrderType::Limit { tif } = &cmd.order_type else {
        return Err(ModifySpotOrderV2Error::OnlyLimitOrderCanBeModified);
    };
    Ok((price, qty, parse_tif(tif)?))
}

fn parse_positive_u64(
    raw: &str,
    error: ModifySpotOrderV2Error,
) -> Result<u64, ModifySpotOrderV2Error> {
    match raw.parse::<u64>() {
        Ok(value) if value > 0 => Ok(value),
        _ => Err(error),
    }
}

fn parse_tif(raw: &str) -> Result<SpotOrderTif, ModifySpotOrderV2Error> {
    match raw {
        "gtc" | "Gtc" => Ok(SpotOrderTif::Gtc),
        "ioc" | "Ioc" => Ok(SpotOrderTif::Ioc),
        "alo" | "Alo" => Ok(SpotOrderTif::Alo),
        _ => Err(ModifySpotOrderV2Error::InvalidTimeInForce),
    }
}

struct BalanceMap {
    balances: HashMap<String, Balance>,
}

impl BalanceMap {
    fn new(balances: &[Balance]) -> Self {
        Self {
            balances: balances
                .iter()
                .cloned()
                .map(|balance| (balance.entity_id(), balance))
                .collect(),
        }
    }

    fn get_mut(
        &mut self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<&mut Balance, ModifySpotOrderV2Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(ModifySpotOrderV2Error::BalanceNotFound)
    }

    fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|balance| balance.entity_id());
        balances
    }
}

enum ReservationSlot {
    Principal,
    Fee,
}

fn adjust_reservation_balance(
    order_before: &SpotOrderV2,
    order_after: &SpotOrderV2,
    slot: ReservationSlot,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerEntryV2>,
) -> Result<(), ModifySpotOrderV2Error> {
    let (old_reservation, new_reservation, label) = match slot {
        ReservationSlot::Principal => {
            (&order_before.reservation, &order_after.reservation, "principal")
        }
        ReservationSlot::Fee => {
            (&order_before.fee_reservation, &order_after.fee_reservation, "fee")
        }
    };
    let old_remaining = old_reservation.remaining_amount;
    let new_remaining = new_reservation.remaining_amount;
    if old_remaining == new_remaining {
        return Ok(());
    }

    let balance = balance_book.get_mut(order_before.account_id(), &new_reservation.asset_id)?;
    let (operation, amount, action, reason) = if new_remaining > old_remaining {
        (
            BalanceLedgerOperation::Freeze,
            new_remaining
                .checked_sub(old_remaining)
                .ok_or(ModifySpotOrderV2Error::ArithmeticOverflow)?,
            "freeze",
            BalanceLedgerReason::ModifySpotOrderFreeze {
                order_id: order_before.order_id().to_owned(),
            },
        )
    } else {
        (
            BalanceLedgerOperation::Unfreeze,
            old_remaining
                .checked_sub(new_remaining)
                .ok_or(ModifySpotOrderV2Error::ArithmeticOverflow)?,
            "unfreeze",
            BalanceLedgerReason::ModifySpotOrderUnfreeze {
                order_id: order_before.order_id().to_owned(),
            },
        )
    };
    let entry_id = format!("balance-ledger:{}:modify:{label}:{action}", order_before.order_id());
    let entry = apply_balance_ledger_entry(operation, entry_id, balance, amount, reason)?;
    ledger_entries.push(entry);
    Ok(())
}

fn apply_balance_ledger_entry(
    operation: BalanceLedgerOperation,
    entry_id: String,
    balance: &mut Balance,
    amount: u64,
    reason: BalanceLedgerReason,
) -> Result<BalanceLedgerEntryV2, ModifySpotOrderV2Error> {
    let account_id = balance.account_id.clone();
    let asset_id = balance.asset_id.clone();
    let balance_entity_id = balance.entity_id();
    let mut entry = match operation {
        BalanceLedgerOperation::Freeze => BalanceLedgerEntryV2::freeze(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::Unfreeze => BalanceLedgerEntryV2::unfreeze(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::CreditAvailable
        | BalanceLedgerOperation::DebitAvailable
        | BalanceLedgerOperation::DebitFrozen => {
            return Err(ModifySpotOrderV2Error::BalanceLedger(
                BalanceLedgerEntryV2Error::InvalidAmount,
            ));
        }
    }?;
    entry.apply_to(balance).map_err(map_balance_ledger_error)?;
    Ok(entry)
}

fn map_balance_ledger_error(error: BalanceLedgerEntryV2Error) -> ModifySpotOrderV2Error {
    match error {
        BalanceLedgerEntryV2Error::InsufficientFrozenBalance => {
            ModifySpotOrderV2Error::InsufficientFrozenBalance
        }
        other => ModifySpotOrderV2Error::BalanceLedger(other),
    }
}

fn merge_balance_pairs(
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, ModifySpotOrderV2Error> {
    let before_map =
        before.into_iter().map(|balance| (balance.entity_id(), balance)).collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::new();
    for after_balance in after {
        let balance_id = after_balance.entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(ModifySpotOrderV2Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(ModifySpotOrderV2Error::BalanceNotFound);
        }
        if before_balance != after_balance {
            pairs.push(UpdatedEntityPair { before: before_balance, after: after_balance });
        }
    }
    Ok(pairs)
}

fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerEntryV2],
) -> Result<Vec<EntityReplayableEvent>, EntityError> {
    let mut current_balances = HashMap::<String, Balance>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.entity_id();
        current_balances.insert(balance_id, balance.before.clone());
    }

    let mut events = Vec::with_capacity(ledger_entries.len());
    for entry in ledger_entries {
        let Some(before) = current_balances.get(&entry.balance_entity_id).cloned() else {
            return Err(EntityError::Custom(
                "balance ledger entry does not belong to updated balances".to_string(),
            ));
        };
        let (
            Some(entry_before_available),
            Some(entry_before_frozen),
            Some(entry_after_available),
            Some(entry_after_frozen),
        ) = (
            entry.before_available,
            entry.before_frozen,
            entry.after_available,
            entry.after_frozen,
        )
        else {
            return Err(EntityError::Custom(
                "balance ledger entry has not been applied".to_string(),
            ));
        };
        if before.available != entry_before_available || before.frozen != entry_before_frozen {
            return Err(EntityError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let next_version = before
            .version
            .checked_add(1)
            .ok_or(EntityError::VersionOverflow { version: before.version })?;
        let after = Balance::new_with_snapshot_facts(
            before.account_id.clone(),
            before.asset_id.clone(),
            entry_after_available,
            entry_after_frozen,
            before.entry_notional,
            before.identifier.clone(),
            next_version,
        );
        events.push(after.track_update_event_from(&before)?);
        current_balances.insert(entry.balance_entity_id.clone(), after);
    }

    for balance in updated_balances {
        let balance_id = balance.after.entity_id();
        if current_balances.get(&balance_id) != Some(&balance.after) {
            return Err(EntityError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    Ok(events)
}
