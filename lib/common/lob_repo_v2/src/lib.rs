use std::collections::HashSet;
use std::io::Write;

use common_entity::{Entity, EntityReplayableEvent};
use example_core_entity::{
    SpotOrderSide, SpotOrderStatus, SpotOrderTif, SpotOrderType, SpotOrderV2,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

pub mod lob;

pub mod imp;

pub mod lob_repo;
pub use lob_repo::LobRepo;

#[derive(Debug, Error)]
pub enum LobError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("invalid stored data: {0}")]
    Codec(String),
    #[error("invalid event: {0}")]
    Event(String),
    #[error("unsupported snapshot version {0}")]
    SnapshotVersion(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobSnapshot {
    pub format_version: u32,
    pub sequence: u64,
    pub orders: Vec<SpotOrderV2>,
}

pub(crate) fn create_order(
    event: &EntityReplayableEvent,
    id: u64,
) -> Result<SpotOrderV2, LobError> {
    let field = |name: &str| -> Result<String, LobError> {
        let bytes =
            event.field_value(name).ok_or_else(|| LobError::Event(format!("missing {name}")))?;
        String::from_utf8(bytes.to_vec())
            .map_err(|_| LobError::Event(format!("invalid UTF-8 in {name}")))
    };
    let number = |name| -> Result<u64, LobError> {
        field(name)?.parse().map_err(|_| LobError::Event(format!("invalid {name}")))
    };
    if number("order_id")? != id {
        return Err(LobError::Event("entity id differs from order_id".into()));
    }
    let asset =
        u32::try_from(number("asset")?).map_err(|_| LobError::Event("invalid asset".into()))?;
    let side = match field("side")?.as_str() {
        "buy" => SpotOrderSide::Buy,
        "sell" => SpotOrderSide::Sell,
        _ => return Err(LobError::Event("invalid side".into())),
    };
    let base = SpotOrderV2::new_pending_limit(
        id,
        asset,
        field("account_id")?,
        field("symbol")?,
        side,
        number("qty")?,
        number("limit_price")?,
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
        None,
        1,
        number("created_at")?,
    );
    apply_fields(base, event)
}

pub(crate) fn apply_fields(
    order: SpotOrderV2,
    event: &EntityReplayableEvent,
) -> Result<SpotOrderV2, LobError> {
    let mut value = serde_json::to_value(&order).map_err(|e| LobError::Codec(e.to_string()))?;
    let mut seen = HashSet::new();
    for change in &event.field_changes {
        let name = change.field_name_as_str().map_err(|e| LobError::Event(e.to_string()))?;
        if !seen.insert(name) {
            return Err(LobError::Event(format!("duplicate field {name}")));
        }
        if change.new_value_len > 64 || change.old_value_len > 64 {
            return Err(LobError::Event(format!("invalid length for {name}")));
        }
        let raw = std::str::from_utf8(change.new_value_bytes())
            .map_err(|e| LobError::Event(e.to_string()))?;
        if name == "version" {
            if raw.parse::<u64>().map_err(|_| LobError::Event("invalid version".into()))?
                != event.new_version
            {
                return Err(LobError::Event("version field mismatch".into()));
            }
            continue;
        }
        if name == "order_id" && raw != order.order_id.to_string() {
            return Err(LobError::Event("order id changed".into()));
        }
        if matches!(name, "order_type" | "effective_tif" | "trigger_price" | "tpsl") {
            continue;
        }
        let (object, key) = if let Some(key) = name.strip_prefix("fee_reservation_") {
            (
                &mut value["fee_reservation"],
                if key == "id" {
                    "reservation_id"
                } else if key == "kind" {
                    "reservation_kind"
                } else {
                    key
                },
            )
        } else if let Some(key) = name.strip_prefix("reservation_") {
            (
                &mut value["reservation"],
                if key == "id" {
                    "reservation_id"
                } else if key == "kind" {
                    "reservation_kind"
                } else {
                    key
                },
            )
        } else {
            (&mut value, name)
        };
        let parsed = match key {
            "order_id" | "asset" | "limit_price" | "qty" | "filled_qty" | "created_at"
            | "updated_at" | "original_amount" | "consumed_amount" | "released_amount"
            | "remaining_amount" => {
                json!(raw.parse::<u64>().map_err(|_| LobError::Event(format!("invalid {name}")))?)
            }
            "reduce_only" => json!(
                raw.parse::<bool>().map_err(|_| LobError::Event("invalid reduce_only".into()))?
            ),
            "side" => json!(enum_name(raw, &["buy", "sell"])?),
            "status" if name == "status" => json!(enum_name(
                raw,
                &["pending", "open", "partially_filled", "filled", "canceled", "rejected"]
            )?),
            "status" => json!(enum_name(
                raw,
                &["active", "exhausted_by_consume", "closed_by_release", "closed_mixed"]
            )?),
            "status_reason" => {
                if raw.is_empty() {
                    Value::Null
                } else {
                    json!(status_reason_name(raw)?)
                }
            }
            "client_order_id" => {
                if raw.is_empty() {
                    Value::Null
                } else {
                    json!(raw)
                }
            }
            "group_relation" => match raw {
                "standalone" => json!("Standalone"),
                "normal_tpsl_parent" => json!("NormalTpslParent"),
                child if child.starts_with("normal_tpsl_child:") => {
                    json!({"NormalTpslChild": {"parent_order_id": child[18..].parse::<u64>().map_err(|_| LobError::Event("invalid group relation".into()))?}})
                }
                _ => return Err(LobError::Event("invalid group relation".into())),
            },
            "reservation_kind" => json!(enum_name(
                raw,
                &[
                    "spot_buy_quote",
                    "spot_sell_base",
                    "spot_buy_fee_quote",
                    "spot_sell_fee_quote",
                    "perp_open_margin",
                    "perp_flip_net_new_margin"
                ]
            )?),
            "account_id" | "symbol" | "reservation_id" | "asset_id" => json!(raw),
            _ => return Err(LobError::Event(format!("unknown field {name}"))),
        };
        object[key] = parsed;
    }
    if seen.contains("order_type")
        || seen.contains("effective_tif")
        || seen.contains("trigger_price")
        || seen.contains("tpsl")
    {
        let field = |name: &str| -> Result<&str, LobError> {
            event
                .field_value(name)
                .map(std::str::from_utf8)
                .transpose()
                .map_err(|e| LobError::Event(e.to_string()))?
                .ok_or_else(|| LobError::Event(format!("missing {name}")))
        };
        let kind = field("order_type")?;
        value["order_type"] = match kind {
            "limit" => {
                json!({"Limit": {"tif": enum_name(field("effective_tif")?, &["gtc", "ioc", "alo"])?}})
            }
            "trigger_market" | "trigger_limit" => {
                json!({"Trigger": {"is_market": kind == "trigger_market", "trigger_price": field("trigger_price")?.parse::<u64>().map_err(|_| LobError::Event("invalid trigger_price".into()))?, "tpsl": enum_name(field("tpsl")?, &["take_profit", "stop_loss"])?}})
            }
            _ => return Err(LobError::Event("invalid order_type".into())),
        };
    }
    value["version"] = json!(event.new_version);
    let result: SpotOrderV2 =
        serde_json::from_value(value).map_err(|e| LobError::Event(e.to_string()))?;
    if result.order_id != event.entity_id as u64 {
        return Err(LobError::Event("entity id changed".into()));
    }
    Ok(result)
}

fn enum_name(raw: &str, allowed: &[&str]) -> Result<String, LobError> {
    if !allowed.contains(&raw) {
        return Err(LobError::Event(format!("invalid enum value {raw}")));
    }
    Ok(raw
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
                .unwrap_or_default()
        })
        .collect())
}

fn status_reason_name(raw: &str) -> Result<String, LobError> {
    let result = match raw {
        "filled" => "Filled",
        "canceled" => "CanceledByUser",
        "triggered" => "Triggered",
        "rejected" => "RejectedAtPlacement",
        other => {
            if !STATUS_REASONS.contains(&other) {
                return Err(LobError::Event(format!("invalid status_reason {raw}")));
            }
            return Ok(other
                .chars()
                .next()
                .map(|first| first.to_uppercase().collect::<String>() + &other[first.len_utf8()..])
                .unwrap_or_default());
        }
    };
    Ok(result.into())
}

const STATUS_REASONS: &[&str] = &[
    "marginCanceled",
    "vaultWithdrawalCanceled",
    "openInterestCapCanceled",
    "selfTradeCanceled",
    "reduceOnlyCanceled",
    "siblingFilledCanceled",
    "delistedCanceled",
    "liquidatedCanceled",
    "scheduledCancel",
    "tickRejected",
    "minTradeNtlRejected",
    "perpMarginRejected",
    "reduceOnlyRejected",
    "badAloPxRejected",
    "iocCancelRejected",
    "badTriggerPxRejected",
    "marketOrderNoLiquidityRejected",
    "positionIncreaseAtOpenInterestCapRejected",
    "positionFlipAtOpenInterestCapRejected",
    "tooAggressiveAtOpenInterestCapRejected",
    "openInterestIncreaseRejected",
    "insufficientSpotBalanceRejected",
    "oracleRejected",
    "perpMaxPositionRejected",
];

fn is_active(order: &SpotOrderV2) -> bool {
    matches!(order.status(), SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
        && order.filled_qty() < order.qty()
}
