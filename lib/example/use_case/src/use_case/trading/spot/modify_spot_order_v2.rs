use std::collections::{HashMap, HashSet};

use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{
    Entity, EntityError, EntityReplayableEvent, IssuedByParty, ReplayableChanges,
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
/// `Oid` 和 `Cloid` 对应 Hyperliquid 顶层 `oid` 的两种形式。该值只用于
/// 查找原订单，不表示改单后订单的 `cloid`。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderId {
    Oid(u64),
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
            OrderId::Oid(oid) if *oid > 0 => {}
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
        sync_order_identity(&mut order_after);

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
        OrderId::Oid(oid) => order.exchange_oid == Some(*oid),
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

fn sync_order_identity(order: &mut SpotOrderV2) {
    order.identity.order_id = order.order_id.clone();
    order.identity.asset = order.asset;
    order.identity.exchange_oid = order.exchange_oid;
    order.identity.account_id = order.account_id.clone();
    order.identity.symbol = order.symbol.clone();
    order.identity.client_order_id = order.client_order_id.clone();
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

#[cfg(test)]
mod tests {
    use common_entity::{ReplayableChanges, StateMachineOwnedV2Diff};

    use super::*;

    fn limit_command(price: &str, size: &str) -> ModifySpotOrderV2Cmd {
        ModifySpotOrderV2Cmd {
            party_id: "buyer".to_owned(),
            asset: 10_001,
            order_id: OrderId::Oid(777),
            is_buy: true,
            price: price.to_owned(),
            size: size.to_owned(),
            order_type: ModifySpotOrderV2OrderType::Limit { tif: "gtc".to_owned() },
            cloid: None,
        }
    }

    fn buy_order(price: u64, qty: u64) -> Result<SpotOrderV2, SpotOrderV2BehaviorError> {
        SpotOrderV2::new_active(
            "order-1".to_owned(),
            10_001,
            Some(777),
            "buyer".to_owned(),
            "BTCUSDT".to_owned(),
            SpotOrderSide::Buy,
            price,
            SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
            qty,
            "BTC",
            "USDT",
            5,
            10,
            Some("original-cloid".to_owned()),
        )
    }

    fn state(order: SpotOrderV2, available: u64) -> ModifySpotOrderV2State {
        let frozen = order.reservation.remaining_amount + order.fee_reservation.remaining_amount;
        ModifySpotOrderV2State {
            order,
            balances: vec![Balance::new(
                "buyer".to_owned(),
                "USDT".to_owned(),
                available,
                frozen,
                1,
            )],
            base_asset_id: "BTC".to_owned(),
            quote_asset_id: "USDT".to_owned(),
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        }
    }

    #[test]
    fn rejects_invalid_command_values_and_trigger_modification() {
        let use_case = ModifySpotOrderV2UseCase;
        let mut empty_party = limit_command("100", "2");
        empty_party.party_id.clear();
        assert_eq!(use_case.check_command(&empty_party), Err(ModifySpotOrderV2Error::EmptyPartyId));

        let mut invalid_lookup = limit_command("100", "2");
        invalid_lookup.order_id = OrderId::Cloid(String::new());
        assert_eq!(
            use_case.check_command(&invalid_lookup),
            Err(ModifySpotOrderV2Error::InvalidLookup)
        );

        let mut invalid_price = limit_command("0", "2");
        assert_eq!(
            use_case.check_command(&invalid_price),
            Err(ModifySpotOrderV2Error::InvalidPrice)
        );
        invalid_price.price = "1.5".to_owned();
        assert_eq!(
            use_case.check_command(&invalid_price),
            Err(ModifySpotOrderV2Error::InvalidPrice)
        );

        let invalid_size = limit_command("100", "0");
        assert_eq!(use_case.check_command(&invalid_size), Err(ModifySpotOrderV2Error::InvalidSize));

        let mut invalid_tif = limit_command("100", "2");
        invalid_tif.order_type = ModifySpotOrderV2OrderType::Limit { tif: "day".to_owned() };
        assert_eq!(
            use_case.check_command(&invalid_tif),
            Err(ModifySpotOrderV2Error::InvalidTimeInForce)
        );

        let mut trigger = limit_command("100", "2");
        trigger.order_type = ModifySpotOrderV2OrderType::Trigger {
            is_market: false,
            trigger_price: "99".to_owned(),
            trigger_role: "tp".to_owned(),
        };
        assert_eq!(
            use_case.check_command(&trigger),
            Err(ModifySpotOrderV2Error::OnlyLimitOrderCanBeModified)
        );
    }

    #[test]
    fn rejects_state_identity_lifecycle_and_side_mismatches()
    -> Result<(), Box<dyn std::error::Error>> {
        let use_case = ModifySpotOrderV2UseCase;
        let order = buy_order(100, 2)?;
        let valid_state = state(order.clone(), 1000);

        let mut party_mismatch = limit_command("101", "2");
        party_mismatch.party_id = "seller".to_owned();
        assert_eq!(
            use_case.validate_state_given(&party_mismatch, &valid_state),
            Err(ModifySpotOrderV2Error::PartyMismatch)
        );

        let mut asset_mismatch = limit_command("101", "2");
        asset_mismatch.asset = 10_002;
        assert_eq!(
            use_case.validate_state_given(&asset_mismatch, &valid_state),
            Err(ModifySpotOrderV2Error::AssetMismatch)
        );

        let mut lookup_mismatch = limit_command("101", "2");
        lookup_mismatch.order_id = OrderId::Cloid("unknown".to_owned());
        assert_eq!(
            use_case.validate_state_given(&lookup_mismatch, &valid_state),
            Err(ModifySpotOrderV2Error::OrderLookupMismatch)
        );

        let mut side_change = limit_command("101", "2");
        side_change.is_buy = false;
        assert_eq!(
            use_case.validate_state_given(&side_change, &valid_state),
            Err(ModifySpotOrderV2Error::SideChangeUnsupported)
        );

        let mut trigger_order = order.clone();
        trigger_order.order_type = SpotOrderType::Trigger {
            is_market: false,
            trigger_price: 99,
            tpsl: crate::entity::SpotOrderTriggerRole::TakeProfit,
        };
        assert_eq!(
            use_case.validate_state_given(&limit_command("101", "2"), &state(trigger_order, 1000)),
            Err(ModifySpotOrderV2Error::OnlyLimitOrderCanBeModified)
        );

        let mut closed_order = order.clone();
        closed_order.status = SpotOrderStatus::Canceled;
        assert_eq!(
            use_case.validate_state_given(&limit_command("101", "2"), &state(closed_order, 1000)),
            Err(ModifySpotOrderV2Error::OrderNotModifiable)
        );

        let mut filled_order = order;
        filled_order.filled_qty = 1;
        assert_eq!(
            use_case.validate_state_given(&limit_command("101", "2"), &state(filled_order, 1000)),
            Err(ModifySpotOrderV2Error::OrderNotModifiable)
        );
        Ok(())
    }

    #[test]
    fn modifies_buy_order_and_adjusts_principal_and_fee_freezes()
    -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 2)?;
        let old_principal = order.reservation.remaining_amount;
        let old_fee = order.fee_reservation.remaining_amount;
        let state = state(order, 100_000);
        let changes =
            ModifySpotOrderV2UseCase.compute_state_diff(&limit_command("12000", "3"), state)?;

        assert_eq!(changes.updated_order.before.order_id(), "order-1");
        assert_eq!(changes.updated_order.after.order_id(), "order-1");
        assert_eq!(changes.updated_order.after.exchange_oid, Some(777));
        assert_eq!(changes.updated_order.after.version, changes.updated_order.before.version + 1);
        assert_eq!(changes.updated_order.after.order_price(), 12_000);
        assert_eq!(changes.updated_order.after.qty(), 3);
        assert_eq!(changes.updated_order.after.client_order_id.as_deref(), Some("original-cloid"));
        assert_eq!(changes.created_balance_ledger_entries.len(), 2);
        assert_eq!(
            changes.created_balance_ledger_entries[0].entry_id,
            "balance-ledger:order-1:modify:principal:freeze"
        );
        assert_eq!(
            changes.created_balance_ledger_entries[1].entry_id,
            "balance-ledger:order-1:modify:fee:freeze"
        );
        assert_eq!(
            changes.created_balance_ledger_entries[0].amount,
            changes.updated_order.after.reservation.remaining_amount - old_principal
        );
        assert_eq!(
            changes.created_balance_ledger_entries[1].amount,
            changes.updated_order.after.fee_reservation.remaining_amount - old_fee
        );
        assert_eq!(changes.updated_balances[0].after.available, 83_984);
        assert_eq!(changes.updated_balances[0].after.frozen, 36_036);
        Ok(())
    }

    #[test]
    fn modifies_smaller_order_and_unfreezes_principal_and_fee()
    -> Result<(), Box<dyn std::error::Error>> {
        let order = buy_order(10_000, 3)?;
        let state = state(order, 50_000);
        let changes =
            ModifySpotOrderV2UseCase.compute_state_diff(&limit_command("9000", "1"), state)?;

        assert_eq!(changes.created_balance_ledger_entries.len(), 2);
        assert!(changes.created_balance_ledger_entries.iter().all(|entry| {
            entry.operation == BalanceLedgerOperation::Unfreeze
                && entry.reason
                    == BalanceLedgerReason::ModifySpotOrderUnfreeze {
                        order_id: "order-1".to_owned(),
                    }
        }));
        assert_eq!(changes.updated_balances[0].after.available, 71_021);
        assert_eq!(changes.updated_balances[0].after.frozen, 9_009);
        Ok(())
    }

    #[test]
    fn replay_orders_order_then_balance_chain_then_ledger_creation()
    -> Result<(), Box<dyn std::error::Error>> {
        let state = state(buy_order(10_000, 2)?, 100_000);
        let changes = ModifySpotOrderV2UseCase.compute_state_diff(
            &ModifySpotOrderV2Cmd {
                cloid: Some("replacement".to_owned()),
                ..limit_command("12000", "3")
            },
            state,
        )?;
        let events = changes.to_replayable_events()?;

        assert_eq!(events.len(), 5);
        assert!(events[0].is_updated());
        assert!(events[1].is_updated());
        assert!(events[2].is_updated());
        assert!(events[3].is_created());
        assert!(events[4].is_created());
        assert!(events.iter().all(|event| event.new_version == event.old_version + 1));
        Ok(())
    }
}
