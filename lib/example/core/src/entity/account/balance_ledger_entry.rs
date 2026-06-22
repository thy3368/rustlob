use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use common_entity::{Entity, EntityError, EntityFieldChange};
use serde::{Deserialize, Serialize};

use super::balance::Balance;

const BALANCE_LEDGER_ENTRY_ENTITY_TYPE: u8 = 8;

/// 余额变更原因。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BalanceLedgerReason {
    /// 立即单冻结余额。
    ReserveForImmediateOrder {
        /// 触发本次余额冻结的订单 ID。
        order_id: String,
    },
    /// 现货撤单为买单释放冻结 quote。
    CancelSpotOrderReleaseQuote {
        /// 被撤销订单 ID。
        order_id: String,
    },
    /// 现货撤单为卖单释放冻结 base。
    CancelSpotOrderReleaseBase {
        /// 被撤销订单 ID。
        order_id: String,
    },
    /// 现货清结算为买方增加 base 可用余额。
    SettleSpotTradeBuyerReceiveBase {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货清结算为买方释放冻结 quote。
    SettleSpotTradeBuyerReleaseFrozenQuote {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货清结算为卖方增加 quote 可用余额。
    SettleSpotTradeSellerReceiveQuote {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
    /// 现货清结算为卖方释放冻结 base。
    SettleSpotTradeSellerReleaseFrozenBase {
        /// 本次余额变化对应的 trade id 列表。
        trade_ids: Vec<String>,
        /// 本次余额变化对应的 settlement id 列表。
        settlement_ids: Vec<String>,
    },
}

impl BalanceLedgerReason {
    /// 返回稳定原因编码，供 replay event / 审计查询使用。
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::ReserveForImmediateOrder { .. } => "reserve_for_immediate_order",
            Self::CancelSpotOrderReleaseQuote { .. } => "cancel_spot_order_release_quote",
            Self::CancelSpotOrderReleaseBase { .. } => "cancel_spot_order_release_base",
            Self::SettleSpotTradeBuyerReceiveBase { .. } => "settle_spot_trade_buyer_receive_base",
            Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. } => {
                "settle_spot_trade_buyer_release_frozen_quote"
            }
            Self::SettleSpotTradeSellerReceiveQuote { .. } => {
                "settle_spot_trade_seller_receive_quote"
            }
            Self::SettleSpotTradeSellerReleaseFrozenBase { .. } => {
                "settle_spot_trade_seller_release_frozen_base"
            }
        }
    }

    /// 返回关联订单 ID；非下单冻结场景返回 `None`。
    pub fn order_id(&self) -> Option<&str> {
        match self {
            Self::ReserveForImmediateOrder { order_id }
            | Self::CancelSpotOrderReleaseQuote { order_id }
            | Self::CancelSpotOrderReleaseBase { order_id } => Some(order_id.as_str()),
            Self::SettleSpotTradeBuyerReceiveBase { .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { .. }
            | Self::SettleSpotTradeSellerReceiveQuote { .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { .. } => None,
        }
    }

    /// 返回关联 trade id 列表；非成交清结算场景返回空切片。
    pub fn trade_ids(&self) -> &[String] {
        match self {
            Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. } => &[],
            Self::SettleSpotTradeBuyerReceiveBase { trade_ids, .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { trade_ids, .. }
            | Self::SettleSpotTradeSellerReceiveQuote { trade_ids, .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { trade_ids, .. } => trade_ids,
        }
    }

    /// 返回关联 settlement id 列表；非成交清结算场景返回空切片。
    pub fn settlement_ids(&self) -> &[String] {
        match self {
            Self::ReserveForImmediateOrder { .. }
            | Self::CancelSpotOrderReleaseQuote { .. }
            | Self::CancelSpotOrderReleaseBase { .. } => &[],
            Self::SettleSpotTradeBuyerReceiveBase { settlement_ids, .. }
            | Self::SettleSpotTradeBuyerReleaseFrozenQuote { settlement_ids, .. }
            | Self::SettleSpotTradeSellerReceiveQuote { settlement_ids, .. }
            | Self::SettleSpotTradeSellerReleaseFrozenBase { settlement_ids, .. } => settlement_ids,
        }
    }
}

/// 一条余额变更流水事实。
///
/// 该实体是 create-only 审计记录：它说明哪条余额快照发生了什么变更，以及变更原因。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BalanceLedgerEntry {
    /// 流水记录 ID。
    pub entry_id: String,
    /// 账户 ID。
    pub account_id: String,
    /// 资产 ID。
    pub asset_id: String,
    /// 对应余额快照实体 ID。
    pub balance_entity_id: String,
    /// 变更前可用余额。
    pub before_available: u64,
    /// 变更前冻结余额。
    pub before_frozen: u64,
    /// 变更后可用余额。
    pub after_available: u64,
    /// 变更后冻结余额。
    pub after_frozen: u64,
    /// 余额变更原因。
    pub reason: BalanceLedgerReason,
}

impl BalanceLedgerEntry {
    /// 从已经校验过的余额前后快照与业务原因构造流水记录。
    pub fn new(
        entry_id: String,
        account_id: String,
        asset_id: String,
        balance_entity_id: String,
        before_available: u64,
        before_frozen: u64,
        after_available: u64,
        after_frozen: u64,
        reason: BalanceLedgerReason,
    ) -> Self {
        Self {
            entry_id,
            account_id,
            asset_id,
            balance_entity_id,
            before_available,
            before_frozen,
            after_available,
            after_frozen,
            reason,
        }
    }

    /// 基于余额前后快照创建“立即单冻结余额”流水。
    pub fn reserve_for_immediate_order(
        entry_id: String,
        updated_balance: &UpdatedEntityPair<Balance>,
        order_id: String,
    ) -> Self {
        Self::new(
            entry_id,
            updated_balance.after.account_id.clone(),
            updated_balance.after.asset_id.clone(),
            updated_balance.after.entity_id(),
            updated_balance.before.available,
            updated_balance.before.frozen,
            updated_balance.after.available,
            updated_balance.after.frozen,
            BalanceLedgerReason::ReserveForImmediateOrder { order_id },
        )
    }

    /// 返回这条流水是否对应指定余额实体。
    pub fn belongs_to_balance(&self, balance_entity_id: &str) -> bool {
        self.balance_entity_id == balance_entity_id
    }

    /// 返回流水记录的前后余额是否与给定 pair 完全一致。
    pub fn matches_balance_update(&self, updated_balance: &UpdatedEntityPair<Balance>) -> bool {
        self.account_id == updated_balance.after.account_id
            && self.asset_id == updated_balance.after.asset_id
            && self.balance_entity_id == updated_balance.after.entity_id()
            && self.before_available == updated_balance.before.available
            && self.before_frozen == updated_balance.before.frozen
            && self.after_available == updated_balance.after.available
            && self.after_frozen == updated_balance.after.frozen
    }
}

impl Entity for BalanceLedgerEntry {
    type Id = String;

    fn entity_id(&self) -> Self::Id {
        self.entry_id.clone()
    }

    fn entity_type() -> u8 {
        BALANCE_LEDGER_ENTRY_ENTITY_TYPE
    }

    fn entity_version(&self) -> u64 {
        1
    }

    fn created_field_changes(&self) -> Vec<EntityFieldChange> {
        vec![
            EntityFieldChange::new("entry_id", "", self.entry_id.clone()),
            EntityFieldChange::new("account_id", "", self.account_id.clone()),
            EntityFieldChange::new("asset_id", "", self.asset_id.clone()),
            EntityFieldChange::new("balance_entity_id", "", self.balance_entity_id.clone()),
            EntityFieldChange::new("before_available", "", self.before_available.to_string()),
            EntityFieldChange::new("before_frozen", "", self.before_frozen.to_string()),
            EntityFieldChange::new("after_available", "", self.after_available.to_string()),
            EntityFieldChange::new("after_frozen", "", self.after_frozen.to_string()),
            EntityFieldChange::new("reason", "", self.reason.as_str()),
            EntityFieldChange::new(
                "reason_order_id",
                "",
                self.reason.order_id().unwrap_or_default(),
            ),
            EntityFieldChange::new("reason_trade_ids", "", self.reason.trade_ids().join(",")),
            EntityFieldChange::new(
                "reason_settlement_ids",
                "",
                self.reason.settlement_ids().join(","),
            ),
        ]
    }

    fn diff(&self, _other: &Self) -> Vec<EntityFieldChange> {
        Vec::new()
    }

    fn replay_field_type(field_name: &str) -> u8 {
        match field_name {
            "entry_id"
            | "account_id"
            | "asset_id"
            | "balance_entity_id"
            | "reason"
            | "reason_order_id"
            | "reason_trade_ids"
            | "reason_settlement_ids" => 0,
            "before_available" | "before_frozen" | "after_available" | "after_frozen" => 1,
            _ => 0,
        }
    }

    fn replay_entity_id(&self) -> Result<i64, EntityError> {
        Ok(stable_balance_ledger_entry_id(&self.entry_id))
    }
}

fn stable_balance_ledger_entry_id(value: &str) -> i64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    (hasher.finish() & i64::MAX as u64) as i64
}

#[cfg(test)]
mod tests {
    use common_entity::Entity;

    use super::*;

    fn updated_balance() -> UpdatedEntityPair<Balance> {
        UpdatedEntityPair {
            before: Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 3),
            after: Balance::new("trader-1".to_string(), "USDT".to_string(), 800, 200, 4),
        }
    }

    #[test]
    fn constructor_keeps_balance_snapshot_and_reason() {
        let updated_balance = updated_balance();
        let entry = BalanceLedgerEntry::reserve_for_immediate_order(
            "ledger-trader-1-BTCUSDT-7-USDT".to_string(),
            &updated_balance,
            "trader-1-BTCUSDT-7".to_string(),
        );

        assert_eq!(entry.account_id, "trader-1");
        assert_eq!(entry.asset_id, "USDT");
        assert_eq!(entry.balance_entity_id, "trader-1:USDT");
        assert_eq!(entry.before_available, 1_000);
        assert_eq!(entry.before_frozen, 0);
        assert_eq!(entry.after_available, 800);
        assert_eq!(entry.after_frozen, 200);
        assert_eq!(
            entry.reason,
            BalanceLedgerReason::ReserveForImmediateOrder {
                order_id: "trader-1-BTCUSDT-7".to_string(),
            }
        );
        assert!(entry.belongs_to_balance("trader-1:USDT"));
        assert!(entry.matches_balance_update(&updated_balance));
    }

    #[test]
    fn create_event_contains_balance_transition_and_reason() {
        let entry = BalanceLedgerEntry::new(
            "ledger-trader-1-BTCUSDT-7-USDT".to_string(),
            "trader-1".to_string(),
            "USDT".to_string(),
            "trader-1:USDT".to_string(),
            1_000,
            0,
            800,
            200,
            BalanceLedgerReason::ReserveForImmediateOrder {
                order_id: "trader-1-BTCUSDT-7".to_string(),
            },
        );

        let event = entry.track_create_event().unwrap();

        assert!(event.is_created());
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("balance_entity_id")
                && change.new_value_bytes() == b"trader-1:USDT"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason")
                && change.new_value_bytes() == b"reserve_for_immediate_order"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_order_id")
                && change.new_value_bytes() == b"trader-1-BTCUSDT-7"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_trade_ids")
                && change.new_value_bytes().is_empty()
        }));
    }

    #[test]
    fn create_event_contains_settlement_reason_references() {
        let entry = BalanceLedgerEntry::new(
            "ledger-settle-1-1-buyer-BTC".to_string(),
            "buyer".to_string(),
            "BTC".to_string(),
            "buyer:BTC".to_string(),
            0,
            0,
            3,
            0,
            BalanceLedgerReason::SettleSpotTradeBuyerReceiveBase {
                trade_ids: vec!["trade-1".to_string(), "trade-2".to_string()],
                settlement_ids: vec!["settle-1-1".to_string(), "settle-1-2".to_string()],
            },
        );

        let event = entry.track_create_event().unwrap();

        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason")
                && change.new_value_bytes() == b"settle_spot_trade_buyer_receive_base"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_trade_ids")
                && change.new_value_bytes() == b"trade-1,trade-2"
        }));
        assert!(event.field_changes.iter().any(|change| {
            change.field_name_as_str().ok() == Some("reason_settlement_ids")
                && change.new_value_bytes() == b"settle-1-1,settle-1-2"
        }));
    }
}
