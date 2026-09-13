use cmd_handler::command_use_case_def2::UpdatedEntityPair;
use serde::{Deserialize, Serialize};

use crate::entity::Balance;
use crate::entity::account::balance_ledger_entry_v2::BalanceLedgerEntryV2;
use crate::entity::spot::spot_order_v2::SpotOrderV2;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_id_serializes_oid_and_cloid_as_distinct_values() {
        let oid = serde_json::to_value(OrderId::Oid(42)).expect("serialize oid");
        let cloid =
            serde_json::to_value(OrderId::Cloid("client-42".to_owned())).expect("serialize cloid");

        assert_eq!(oid, serde_json::json!({"Oid": 42}));
        assert_eq!(cloid, serde_json::json!({"Cloid": "client-42"}));
        assert_ne!(oid, cloid);
    }

    #[test]
    fn lookup_order_id_is_independent_from_replacement_cloid() {
        let command = ModifySpotOrderV2Cmd {
            party_id: "party-1".to_owned(),
            asset: 10001,
            order_id: OrderId::Oid(42),
            is_buy: true,
            price: "100".to_owned(),
            size: "2".to_owned(),
            order_type: ModifySpotOrderV2OrderType::Limit { tif: "Gtc".to_owned() },
            cloid: Some("replacement-cloid".to_owned()),
        };

        assert_eq!(command.order_id, OrderId::Oid(42));
        assert_eq!(command.cloid.as_deref(), Some("replacement-cloid"));
    }

    #[test]
    fn order_type_variants_encode_exclusive_limit_and_trigger_shapes() {
        let limit =
            serde_json::to_value(ModifySpotOrderV2OrderType::Limit { tif: "Ioc".to_owned() })
                .expect("serialize limit");
        let trigger = serde_json::to_value(ModifySpotOrderV2OrderType::Trigger {
            is_market: false,
            trigger_price: "105".to_owned(),
            trigger_role: "tp".to_owned(),
        })
        .expect("serialize trigger");

        assert_eq!(limit, serde_json::json!({"Limit": {"tif": "Ioc"}}));
        assert_eq!(
            trigger,
            serde_json::json!({
                "Trigger": {
                    "is_market": false,
                    "trigger_price": "105",
                    "trigger_role": "tp"
                }
            })
        );
        assert_ne!(limit, trigger);
    }

    #[test]
    fn command_carries_complete_replacement_values() {
        let command = ModifySpotOrderV2Cmd {
            party_id: "party-1".to_owned(),
            asset: 10001,
            order_id: OrderId::Cloid("original".to_owned()),
            is_buy: false,
            price: "101.25".to_owned(),
            size: "3.5".to_owned(),
            order_type: ModifySpotOrderV2OrderType::Trigger {
                is_market: true,
                trigger_price: "99".to_owned(),
                trigger_role: "sl".to_owned(),
            },
            cloid: None,
        };

        assert_eq!(command.price, "101.25");
        assert_eq!(command.size, "3.5");
        assert_eq!(command.cloid, None);
        assert!(matches!(
            command.order_type,
            ModifySpotOrderV2OrderType::Trigger { is_market: true, .. }
        ));
    }
}
