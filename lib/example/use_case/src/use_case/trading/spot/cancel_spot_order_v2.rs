use std::collections::{HashMap, HashSet};

use common_entity::{
    Entity, EntityReplayableEvent, LogEntry, ReplayableChanges, StateDiff, UpdatedEntityPair,
    UseCase,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::common_entity::account::balance_ledger_reason::BalanceLedgerReason;
use crate::common_entity::account::balance_ledger_v2::{
    BalanceLedgerEntryV2Error, BalanceLedgerOperation, BalanceLedgerV2,
};
use crate::common_entity::spot::spot_order_v2::{
    SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError,
};
use crate::common_entity::{Balance, CancelSpotOrderV2Input, OrderSide};
use crate::support::{concat3, concat4};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CancelSpotOrderV2Cmd {
    pub party_id: String,
    pub asset: u32,
    pub lookup: CancelSpotOrderV2Lookup,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CancelSpotOrderV2Lookup {
    #[default]
    Missing,
    Oid(u64),
    Cloid(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2State {
    pub order: SpotOrderV2,
    pub balances: Vec<Balance>,
    pub base_asset_id: String,
    pub quote_asset_id: String,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2AfterChanges {
    pub order_after: SpotOrderV2,
    pub balances_after: Vec<Balance>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerV2>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CancelSpotOrderV2Changes {
    pub updated_order: UpdatedEntityPair<SpotOrderV2>,
    pub updated_balances: Vec<UpdatedEntityPair<Balance>>,
    pub created_balance_ledger_entries: Vec<BalanceLedgerV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CancelSpotOrderV2Error {
    #[error("balance not found")]
    BalanceNotFound,
    #[error("frozen balance is insufficient")]
    InsufficientFrozenBalance,
    #[error("arithmetic overflow while computing spot order v2 cancel")]
    ArithmeticOverflow,
    #[error(transparent)]
    OrderMatch(#[from] SpotOrderV2MatchError),
    #[error(transparent)]
    OrderBehavior(#[from] SpotOrderV2BehaviorError),
    #[error(transparent)]
    BalanceLedger(#[from] BalanceLedgerEntryV2Error),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancelSpotOrderV2UseCase;

impl ReplayableChanges for CancelSpotOrderV2Changes {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
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

impl UseCase for CancelSpotOrderV2UseCase {
    type Command = LogEntry<CancelSpotOrderV2Cmd>;
    type StateGiven = CancelSpotOrderV2State;
    type Error = CancelSpotOrderV2Error;
    type StateChanged = CancelSpotOrderV2AfterChanges;

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        let mut order_after = given_state.order.clone();
        order_after.cancel(CancelSpotOrderV2Input { timestamp: cmd.logical_ts() })?;
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let mut order_after = given_state.order.clone();
        let mut balance_book = BalanceMap::new(&given_state.balances);
        let mut created_balance_ledger_entries = Vec::with_capacity(0);

        order_after.cancel(CancelSpotOrderV2Input { timestamp: cmd.logical_ts() })?;

        release_remaining_for_cancel(
            &order_after,
            &mut balance_book,
            &mut created_balance_ledger_entries,
            &given_state.base_asset_id,
            &given_state.quote_asset_id,
            given_state.maker_fee_bps,
            given_state.taker_fee_bps,
        )?;

        // 订单版本由改变它的命令 LSN 定义；撤单行为内部的递增版本仅是实体默认值。
        order_after.version = cmd.lsn();

        Ok(CancelSpotOrderV2AfterChanges {
            order_after,
            balances_after: balance_book.into_balances(),
            created_balance_ledger_entries,
        })
    }
}

impl StateDiff for CancelSpotOrderV2UseCase {
    type StateDiff = CancelSpotOrderV2Changes;

    fn do_compute_state_diff(
        given_state: CancelSpotOrderV2State,
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(CancelSpotOrderV2Changes {
            updated_order: UpdatedEntityPair {
                before: given_state.order,
                after: after.order_after,
            },
            updated_balances: merge_balance_pairs(given_state.balances, after.balances_after)?,
            created_balance_ledger_entries: after.created_balance_ledger_entries,
        })
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
                .map(|balance| (balance.business_entity_id(), balance))
                .collect(),
        }
    }

    fn get_mut(
        &mut self,
        account_id: &str,
        asset_id: &str,
    ) -> Result<&mut Balance, CancelSpotOrderV2Error> {
        self.balances
            .get_mut(&concat3(account_id, ":", asset_id))
            .ok_or(CancelSpotOrderV2Error::BalanceNotFound)
    }

    fn get_by_entity_id_mut(
        &mut self,
        entity_id: &str,
    ) -> Result<&mut Balance, CancelSpotOrderV2Error> {
        self.balances.get_mut(entity_id).ok_or(CancelSpotOrderV2Error::BalanceNotFound)
    }

    fn into_balances(self) -> Vec<Balance> {
        let mut balances = self.balances.into_values().collect::<Vec<_>>();
        balances.sort_by_key(|lhs| lhs.entity_id());
        balances
    }
}

fn balance_entity_id_for_account_asset(
    balances: &[Balance],
    account_id: &str,
    asset_id: &str,
) -> Result<String, CancelSpotOrderV2Error> {
    balances
        .iter()
        .find(|balance| {
            balance.account_id == common_entity::stable_string_entity_id(account_id)
                && balance.asset_id == common_entity::stable_string_entity_id(asset_id)
        })
        .map(Balance::business_entity_id)
        .ok_or(CancelSpotOrderV2Error::BalanceNotFound)
}

fn balance_entity_id_for_order(
    balances: &[Balance],
    order: &SpotOrderV2,
) -> Result<String, CancelSpotOrderV2Error> {
    let asset = match order.side() {
        OrderSide::Buy => &order.asset().to_string(),
        OrderSide::Sell => &order.asset().to_string(),
    };
    balance_entity_id_for_account_asset(balances, order.account_id(), asset)
}

fn release_remaining_for_cancel(
    order: &SpotOrderV2,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerV2>,
    base_asset_id: &str,
    quote_asset_id: &str,
    maker_fee_bps: u64,
    taker_fee_bps: u64,
) -> Result<(), CancelSpotOrderV2Error> {
    let (principal, fee) = order.terminal_release_requirements(maker_fee_bps, taker_fee_bps);
    for requirement in [principal, fee].into_iter().flatten() {
        let asset_id = match requirement.asset {
            example_core_entity::SpotOrderHoldAsset::Base => base_asset_id.to_string(),
            example_core_entity::SpotOrderHoldAsset::Quote => quote_asset_id.to_string(),
        };
        release_to_balance(order, &asset_id, requirement.amount, balance_book, ledger_entries)?;
    }
    Ok(())
}

fn release_to_balance(
    order: &SpotOrderV2,
    asset_id: &str,
    amount: u64,
    balance_book: &mut BalanceMap,
    ledger_entries: &mut Vec<BalanceLedgerV2>,
) -> Result<(), CancelSpotOrderV2Error> {
    let reason = match order.side() {
        OrderSide::Buy => {
            BalanceLedgerReason::CancelSpotOrderReleaseQuote { order_id: order.order_id() }
        }
        OrderSide::Sell => {
            BalanceLedgerReason::CancelSpotOrderReleaseBase { order_id: order.order_id() }
        }
    };
    let balance = balance_book.get_mut(order.account_id(), asset_id)?;
    let next_release_index =
        ledger_entries.len().checked_add(1).ok_or(CancelSpotOrderV2Error::ArithmeticOverflow)?;
    let entry = apply_balance_ledger_entry(
        BalanceLedgerOperation::Unfreeze,
        concat4(
            "balance-ledger:",
            order.order_id().to_string().as_str(),
            ":release:",
            next_release_index.to_string().as_str(),
        ),
        balance,
        amount,
        reason,
    )?;
    ledger_entries.push(entry);
    Ok(())
}

fn apply_balance_ledger_entry(
    operation: BalanceLedgerOperation,
    entry_id: String,
    balance: &mut Balance,
    amount: u64,
    reason: BalanceLedgerReason,
) -> Result<BalanceLedgerV2, BalanceLedgerEntryV2Error> {
    let account_id = balance.account_id;
    let asset_id = balance.asset_id;
    let balance_entity_id = balance.business_entity_id();
    let mut entry = match operation {
        BalanceLedgerOperation::Unfreeze => BalanceLedgerV2::unfreeze(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::CreditAvailable => BalanceLedgerV2::credit_available(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::DebitFrozen => BalanceLedgerV2::debit_frozen(
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            amount,
            reason,
        ),
        BalanceLedgerOperation::Freeze | BalanceLedgerOperation::DebitAvailable => {
            return Err(BalanceLedgerEntryV2Error::InvalidAmount);
        }
    }?;
    entry.apply_to(balance)?;
    Ok(entry)
}

fn apply_behavior_ledger_entry(
    mut entry: BalanceLedgerV2,
    balance_book: &mut BalanceMap,
) -> Result<BalanceLedgerV2, CancelSpotOrderV2Error> {
    let balance = balance_book.get_by_entity_id_mut(&concat3(
        &entry.account_id.to_string(),
        ":",
        &entry.asset_id.to_string(),
    ))?;
    entry.apply_to(balance)?;
    Ok(entry)
}

fn merge_balance_pairs(
    before: Vec<Balance>,
    after: Vec<Balance>,
) -> Result<Vec<UpdatedEntityPair<Balance>>, CancelSpotOrderV2Error> {
    let before_map = before
        .into_iter()
        .map(|balance| (balance.business_entity_id(), balance))
        .collect::<HashMap<_, _>>();
    let mut seen = HashSet::new();
    let mut pairs = Vec::with_capacity(after.len());
    for balance in after {
        let balance_id = balance.business_entity_id();
        let before_balance =
            before_map.get(&balance_id).cloned().ok_or(CancelSpotOrderV2Error::BalanceNotFound)?;
        if !seen.insert(balance_id) {
            return Err(CancelSpotOrderV2Error::BalanceNotFound);
        }
        pairs.push(UpdatedEntityPair { before: before_balance, after: balance });
    }
    Ok(pairs)
}

fn balance_replay_events_from_ledger_entries(
    updated_balances: &[UpdatedEntityPair<Balance>],
    ledger_entries: &[BalanceLedgerV2],
) -> Result<Vec<EntityReplayableEvent>, common_entity::EntityError> {
    let mut current_balances = HashMap::<String, Balance>::with_capacity(updated_balances.len());
    for balance in updated_balances {
        let balance_id = balance.before.business_entity_id();
        current_balances.insert(balance_id, balance.before.clone());
    }

    let mut events = Vec::with_capacity(ledger_entries.len());
    for entry in ledger_entries {
        let Some(before) = current_balances
            .get(&concat3(&entry.account_id.to_string(), ":", &entry.asset_id.to_string()))
            .cloned()
        else {
            return Err(common_entity::EntityError::Custom(
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
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry has not been applied".to_string(),
            ));
        };
        if before.available != entry_before_available || before.frozen != entry_before_frozen {
            return Err(common_entity::EntityError::Custom(
                "balance ledger entry breaks balance replay chain".to_string(),
            ));
        }
        let next_version = before
            .version
            .checked_add(1)
            .ok_or(common_entity::EntityError::VersionOverflow { version: before.version })?;
        let after = Balance::new(
            before.party_id,
            before.account_id,
            before.asset_id,
            entry_after_available,
            entry_after_frozen,
            next_version,
        );
        events.push(after.track_update_event_from(&before)?);
        current_balances.insert(
            concat3(&entry.account_id.to_string(), ":", &entry.asset_id.to_string()).clone(),
            after,
        );
    }

    for balance in updated_balances {
        let balance_id = balance.after.business_entity_id();
        if current_balances.get(&balance_id) != Some(&balance.after) {
            return Err(common_entity::EntityError::Custom(
                "balance replay chain does not reach case-level balance after state".to_string(),
            ));
        }
    }
    Ok(events)
}
