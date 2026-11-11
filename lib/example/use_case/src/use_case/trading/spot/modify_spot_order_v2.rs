use std::collections::{HashMap, HashSet};

use common_entity::{
    Entity, EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff,
    UpdatedEntityPair, UseCase,
};
use example_core_entity::common::identifiers::OrderId;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common_entity::account::balance_ledger_v3::{
    BalanceLedgerV3, BalanceLedgerV3Error, LedgerOperation,
};
use crate::common_entity::spot::spot_order_v2::{
    SpotOrderHoldAsset, SpotOrderV2, SpotOrderV2BehaviorError,
};
use crate::common_entity::{BalanceV2, OrderSide, OrderTif, OrderType, SpotOrderStatus};
use crate::support::concat3;

// /// 定位一个已经存在的订单。
// ///
// /// 现货改单支持用 Hyperliquid numeric `oid` 或 `cloid` 查找原订单。
// #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// pub enum OrderId {
//     Oid(u64),
//     Cloid(String),
// }
//
// impl Default for OrderId {
//     fn default() -> Self {
//         Self::Oid(0)
//     }
// }

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

/// 现货改单所需的 authoritative 状态快照。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2State {
    pub order: SpotOrderV2,
    pub balances: Vec<BalanceV2>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

/// 现货改单的纯业务计算结果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2AfterChanges {
    pub order_after: SpotOrderV2,
    pub balances_after: Vec<BalanceV2>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerV3>,
}

/// 现货改单的 before/after 变化。
///
/// 订单 pair 和余额 pairs 分别是各自变化的唯一 authoritative truth；after
/// 快照不在此类型中重复保存。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifySpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_balances: Vec<UpdatedEntityPair<BalanceV2>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerV3>,
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
    #[error("on_command party does not match the loaded order party")]
    PartyMismatch,
    #[error("on_command asset does not match the loaded order asset")]
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
    BalanceLedger(#[from] BalanceLedgerV3Error),
    #[error(transparent)]
    Entity(#[from] EntityError),
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

impl UseCase for ModifySpotOrderV2UseCase {
    type Command = LogEntry<ModifySpotOrderV2Cmd>;
    type StateGiven = ModifySpotOrderV2State;
    type Error = ModifySpotOrderV2Error;
    type StateChanged = ModifySpotOrderV2AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(ModifySpotOrderV2Error::EmptyPartyId);
        }
        match &cmd.order_id {
            OrderId::Oid(order_id) if *order_id > 0 => {}
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
        if !matches!(order.order_type, OrderType::Limit { .. }) {
            return Err(ModifySpotOrderV2Error::OnlyLimitOrderCanBeModified);
        }
        if order.status != SpotOrderStatus::Open || order.filled_size != 0 {
            return Err(ModifySpotOrderV2Error::OrderNotModifiable);
        }
        if side_from_command(cmd) != order.side {
            return Err(ModifySpotOrderV2Error::SideChangeUnsupported);
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let (price, qty, tif) = parsed_limit_command(cmd)?;
        let order = &given_state.order;
        let holds_before = order
            .active_hold_requirements(given_state.maker_fee_bps, given_state.taker_fee_bps)?
            .ok_or(ModifySpotOrderV2Error::OrderNotModifiable)?;

        let mut order_after = order.clone();
        order_after.limit_price = price;
        order_after.size = qty;
        order_after.order_type = OrderType::Limit { tif };
        order_after.cloid = cmd.cloid.clone().or_else(|| order.cloid.clone());
        order_after.updated_at = cmd.logical_ts();
        // 订单版本由改变订单的命令序列号定义，而不是当前版本的递增值。
        order_after.version = cmd.lsn();
        let holds_after = order_after
            .active_hold_requirements(given_state.maker_fee_bps, given_state.taker_fee_bps)?
            .ok_or(ModifySpotOrderV2Error::OrderNotModifiable)?;

        let mut balance_book = BalanceMap::new(&given_state.balances);
        let mut created_balance_ledger_entries = Vec::with_capacity(2);
        apply_hold_requirement_delta(
            order,
            holds_before.principal.asset,
            holds_before.principal.amount,
            holds_after.principal.amount,
            "principal",
            &given_state.base_asset_id,
            &given_state.quote_asset_id,
            &mut balance_book,
            &mut created_balance_ledger_entries,
            cmd.logical_ts(),
        )?;
        apply_hold_requirement_delta(
            order,
            holds_before.fee.asset,
            holds_before.fee.amount,
            holds_after.fee.amount,
            "fee",
            &given_state.base_asset_id,
            &given_state.quote_asset_id,
            &mut balance_book,
            &mut created_balance_ledger_entries,
            cmd.logical_ts(),
        )?;

        Ok(ModifySpotOrderV2AfterChanges {
            order_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
        })
    }
}

impl StateDiff for ModifySpotOrderV2UseCase {
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
        OrderId::Oid(order_id) => order.order_id() == *order_id,
        OrderId::Cloid(cloid) => order.cloid.as_deref() == Some(cloid.as_str()),
    }
}

fn side_from_command(cmd: &ModifySpotOrderV2Cmd) -> OrderSide {
    if cmd.is_buy { OrderSide::Buy } else { OrderSide::Sell }
}

fn parsed_limit_command(
    cmd: &ModifySpotOrderV2Cmd,
) -> Result<(u64, u64, OrderTif), ModifySpotOrderV2Error> {
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

fn parse_tif(raw: &str) -> Result<OrderTif, ModifySpotOrderV2Error> {
    match raw {
        "gtc" | "Gtc" => Ok(OrderTif::Gtc),
        "ioc" | "Ioc" => Ok(OrderTif::Ioc),
        "alo" | "Alo" => Ok(OrderTif::Alo),
        _ => Err(ModifySpotOrderV2Error::InvalidTimeInForce),
    }
}

struct BalanceMap {
    balances: HashMap<String, BalanceV2>,
}

impl BalanceMap {
    fn new(balances: &[BalanceV2]) -> Self {
        Self {
            balances: balances
                .iter()
                .cloned()
                .map(|balance| (balance.business_entity_id(), balance))
                .collect(),
        }
    }

    fn get_mut(
        &mut self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<&mut BalanceV2, ModifySpotOrderV2Error> {
        let direct_key = concat3(account_id, ":", asset_id);
        let hashed_key = concat3(
            &common_entity::stable_string_entity_id(account_id).to_string(),
            ":",
            &common_entity::stable_string_entity_id(asset_id).to_string(),
        );
        let key = if self.balances.contains_key(&direct_key) { direct_key } else { hashed_key };
        self.balances.get_mut(&key).ok_or(ModifySpotOrderV2Error::BalanceNotFound)
    }

    fn into_balances(self) -> Vec<BalanceV2> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|balance| balance.business_entity_id());
        balances
    }
}

fn apply_hold_requirement_delta(
    order: &SpotOrderV2,
    asset: SpotOrderHoldAsset,
    before_amount: u64,
    after_amount: u64,
    label: &str,
    base_asset_id: &str,
    quote_asset_id: &str,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerV3>,
    timestamp: u64,
) -> Result<(), ModifySpotOrderV2Error> {
    if before_amount == after_amount {
        return Ok(());
    }

    let asset_id = match asset {
        SpotOrderHoldAsset::Base => base_asset_id,
        SpotOrderHoldAsset::Quote => quote_asset_id,
    };
    let balance = balance_book.get_mut(order.account_id(), asset_id)?;
    let (operation, amount, action) = if after_amount > before_amount {
        (
            LedgerOperation::Freeze,
            after_amount
                .checked_sub(before_amount)
                .ok_or(ModifySpotOrderV2Error::ArithmeticOverflow)?,
            "freeze",
        )
    } else {
        (
            LedgerOperation::Unfreeze,
            before_amount
                .checked_sub(after_amount)
                .ok_or(ModifySpotOrderV2Error::ArithmeticOverflow)?,
            "unfreeze",
        )
    };
    let entry_id = format!("balance-ledger:{}:modify:{label}:{action}", order.order_id());
    let entry = apply_balance_ledger_entry(operation, entry_id, balance, amount, timestamp)?;
    ledger_entries.push(entry);
    Ok(())
}

fn apply_balance_ledger_entry(
    operation: LedgerOperation,
    entry_id: String,
    balance: &mut BalanceV2,
    amount: u64,
    timestamp: u64,
) -> Result<BalanceLedgerV3, ModifySpotOrderV2Error> {
    let mut entry = BalanceLedgerV3::new(
        common_entity::stable_string_entity_id(&entry_id),
        balance,
        operation,
        Decimal::from(amount),
        timestamp,
    )?;
    entry.apply_to(balance, timestamp).map_err(map_balance_ledger_error)?;
    Ok(entry)
}

fn map_balance_ledger_error(error: BalanceLedgerV3Error) -> ModifySpotOrderV2Error {
    match error {
        BalanceLedgerV3Error::BalanceTransition(
            crate::common_entity::BalanceV2Error::InsufficientFrozenBalance,
        ) => ModifySpotOrderV2Error::InsufficientFrozenBalance,
        other => ModifySpotOrderV2Error::BalanceLedger(other),
    }
}

fn merge_balance_pairs(
    before: Vec<BalanceV2>,
    after: Vec<BalanceV2>,
) -> Result<Vec<UpdatedEntityPair<BalanceV2>>, ModifySpotOrderV2Error> {
    let before_map = before
        .into_iter()
        .map(|balance| (balance.business_entity_id(), balance))
        .collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::new();
    for after_balance in after {
        let balance_id = after_balance.business_entity_id();
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
    updated_balances: &[UpdatedEntityPair<BalanceV2>],
    ledger_entries: &[BalanceLedgerV3],
) -> Result<Vec<EntityReplayableEvent>, EntityError> {
    let mut current_balances = HashMap::<String, BalanceV2>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.business_entity_id();
        current_balances.insert(balance_id, balance.before.clone());
    }

    for entry in ledger_entries {
        let Some(before) = current_balances
            .get(&concat3(&entry.account_id.to_string(), ":", &entry.coin_idx.to_string()))
            .cloned()
        else {
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
        if before.available() != entry_before_available || before.frozen() != entry_before_frozen {
            return Err(EntityError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let after = BalanceV2::new(
            before.party_id(),
            before.account_id(),
            before.coin_idx(),
            entry_after_available,
            entry_after_frozen,
            before
                .version()
                .checked_add(1)
                .ok_or(EntityError::VersionOverflow { version: before.version() })?,
            before.created_at(),
        );
        current_balances.insert(
            concat3(&entry.account_id.to_string(), ":", &entry.coin_idx.to_string()).clone(),
            after,
        );
    }

    for balance in updated_balances {
        let balance_id = balance.after.business_entity_id();
        if !current_balances.get(&balance_id).is_some_and(|current| current == &balance.after) {
            return Err(EntityError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    updated_balances
        .iter()
        .map(|balance| balance.after.track_update_event_from(&balance.before))
        .collect()
}
