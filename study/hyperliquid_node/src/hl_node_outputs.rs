//! 从 `hl-node` ELF 静态线索整理出的候选输出 DTO。
//!
//! 这些类型只用于 adapter/research 数据保存，不代表已确认的线上协议或核心
//! 领域实体。字段来自静态字符串和 serde 类型线索；`trade`、`fill` 与
//! `raw-book-diff` 的最终封装格式尚未由运行时样本确认，`ExchangeMisc` 的真实
//! 12 字段也尚未恢复。因此无法确认必填性的字段均使用 [`Option`]。

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum HlNodeRecord {
    Trade(TradeRecord),
    Fill(FillRecord),
    OrderStatus(OrderStatusRecord),
    RawBookDiff(RawBookDiffRecord),
    Misc(ExchangeMiscRecord),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Open,
    Filled,
    Canceled,
    Rejected,
    Triggered,
    MarginCanceled,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StatusReason {
    User,
    Liquidation,
    Expired,
    ReduceOnly,
    Invalid,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TakeProfit,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Alo,
    Fok,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BookSide {
    Bid,
    Ask,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BookDiffAction {
    Insert,
    Update,
    Delete,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TradeRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<BookSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tid: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FillRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub px: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sz: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<BookSide>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_pnl: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_position: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_fee: Option<Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crossed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OrderStatusRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    pub oid: Option<u64>,
    pub status: Option<OrderStatus>,
    pub status_reason: Option<StatusReason>,
    pub order_type: Option<OrderType>,
    pub tif: Option<TimeInForce>,
    pub px: Option<Decimal>,
    pub sz: Option<Decimal>,
    pub filled_sz: Option<Decimal>,
    pub resting_sz: Option<Decimal>,
    pub time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RawBookDiffRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    pub side: Option<BookSide>,
    pub action: Option<BookDiffAction>,
    pub px: Option<Decimal>,
    pub sz: Option<Decimal>,
    pub sequence: Option<u64>,
    pub time: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeMiscRecord {
    pub event_kind: MiscEventKind,
    pub payload: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum MiscEventKind {
    #[default]
    Unknown,
    Funding,
    Liquidation,
    Asset,
    Governance,
}

#[cfg(test)]
#[allow(clippy::disallowed_methods, clippy::indexing_slicing)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn records_round_trip_and_use_discriminator() {
        let records = vec![
            HlNodeRecord::Trade(TradeRecord { coin: Some("BTC".into()), ..Default::default() }),
            HlNodeRecord::Fill(FillRecord {
                closed_pnl: Some(Decimal::new(12, 1)),
                ..Default::default()
            }),
            HlNodeRecord::OrderStatus(OrderStatusRecord {
                resting_sz: Some(Decimal::ONE),
                ..Default::default()
            }),
            HlNodeRecord::RawBookDiff(RawBookDiffRecord::default()),
            HlNodeRecord::Misc(ExchangeMiscRecord {
                event_kind: MiscEventKind::Unknown,
                payload: json!({"x": 1}),
            }),
        ];
        for record in records {
            let encoded = serde_json::to_string(&record).expect("serialize DTO");
            assert!(encoded.contains("\"type\""));
            assert_eq!(record, serde_json::from_str(&encoded).expect("deserialize DTO"));
        }
    }

    #[test]
    fn optional_fields_are_omitted_and_names_are_camel_case() {
        let value = serde_json::to_value(FillRecord {
            builder_fee: Some(Decimal::ONE),
            ..Default::default()
        })
        .expect("serialize");
        assert_eq!(value["builderFee"], json!("1"));
        assert!(value.get("closedPnl").is_none());
        let value = serde_json::to_value(OrderStatusRecord {
            resting_sz: Some(Decimal::ONE),
            ..Default::default()
        })
        .expect("serialize");
        assert_eq!(value["restingSz"], json!("1"));
    }
}
