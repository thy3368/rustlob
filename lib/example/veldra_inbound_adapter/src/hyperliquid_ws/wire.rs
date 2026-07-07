use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::exchange::ExchangeActionRequestWire;
use crate::hyperliquid_ws::error::HyperliquidWsError;

type ExtraFields = BTreeMap<String, JsonValue>;

/// 客户端主动发给 Hyperliquid WebSocket 的四类方法。
/// 这里直接对齐官方 wire 形状，不引入额外 command 抽象层。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum WsClientMessageWire {
    #[serde(rename = "subscribe")]
    Subscribe { subscription: SubscriptionWire },
    #[serde(rename = "unsubscribe")]
    Unsubscribe { subscription: SubscriptionWire },
    #[serde(rename = "post")]
    Post { id: u64, request: WsPostRequestWire },
    #[serde(rename = "ping")]
    Ping(PingWire),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PingWire;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SubscriptionWire {
    #[serde(rename = "allMids")]
    AllMids {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    #[serde(rename = "notification")]
    Notification { user: String },
    #[serde(rename = "webData2")]
    WebData2 { user: String },
    #[serde(rename = "candle")]
    Candle { coin: String, interval: CandleIntervalWire },
    #[serde(rename = "l2Book")]
    L2Book {
        coin: String,
        #[serde(rename = "nSigFigs", default, skip_serializing_if = "Option::is_none")]
        n_sig_figs: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        mantissa: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        dex: Option<String>,
    },
    #[serde(rename = "trades")]
    Trades { coin: String },
    #[serde(rename = "orderUpdates")]
    OrderUpdates { user: String },
    #[serde(rename = "userEvents")]
    UserEvents { user: String },
    #[serde(rename = "userFills")]
    UserFills {
        user: String,
        #[serde(rename = "aggregateByTime", default, skip_serializing_if = "Option::is_none")]
        aggregate_by_time: Option<bool>,
    },
    #[serde(rename = "userFundings")]
    UserFundings { user: String },
    #[serde(rename = "userNonFundingLedgerUpdates")]
    UserNonFundingLedgerUpdates { user: String },
    #[serde(rename = "activeAssetCtx")]
    ActiveAssetCtx { coin: String },
    #[serde(rename = "activeAssetData")]
    ActiveAssetData { user: String, coin: String },
    #[serde(rename = "userTwapSliceFills")]
    UserTwapSliceFills { user: String },
    #[serde(rename = "userTwapHistory")]
    UserTwapHistory { user: String },
    #[serde(rename = "bbo")]
    Bbo { coin: String },
    #[serde(rename = "userToaster")]
    UserToaster { user: String },
    #[serde(rename = "userRequest")]
    UserRequest { user: String },
    #[serde(rename = "ledgerUpdates")]
    LedgerUpdates { user: String },
    #[serde(rename = "twapStates")]
    TwapStates { user: String },
    #[serde(rename = "allDexsClearinghouseState")]
    AllDexsClearinghouseState { user: String },
    #[serde(rename = "allDexsAssetCtxs")]
    AllDexsAssetCtxs {},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CandleIntervalWire(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsPostRequestWire {
    #[serde(rename = "info")]
    Info(JsonValue),
    #[serde(rename = "action")]
    Action(ExchangeActionRequestWire),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WsPostRequestTypeWire {
    Info,
    Action,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WsServerMessageWire {
    SubscriptionResponse(SubscriptionResponseWire),
    Channel(WsChannelMessageWire),
    Post(WsPostResponseWire),
    Pong,
    Unknown { raw: JsonValue },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionResponseWire {
    pub method: String,
    pub subscription: SubscriptionWire,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WsChannelMessageWire {
    AllMids(AllMidsWire),
    Notification(NotificationWire),
    WebData2(WebData2Wire),
    Candle(Vec<CandleWire>),
    L2Book(WsBookWire),
    Trades(Vec<WsTradeWire>),
    OrderUpdates(Vec<WsBasicOrder>),
    UserEvents(WsUserEventWire),
    UserFills(WsUserFillsWire),
    UserFundings(WsUserFundingsWire),
    UserNonFundingLedgerUpdates(WsUserNonFundingLedgerUpdatesWire),
    ActiveAssetCtx(WsActiveAssetCtxWire),
    ActiveAssetData(WsActiveAssetDataWire),
    UserTwapSliceFills(WsUserTwapSliceFillsWire),
    UserTwapHistory(WsUserTwapHistoryWire),
    Bbo(WsBboWire),
    UserToaster(NotificationWire),
    UserRequest(JsonValue),
    LedgerUpdates(WsUserNonFundingLedgerUpdatesWire),
    TwapStates(JsonValue),
    AllDexsClearinghouseState(AllDexsClearinghouseStateWire),
    AllDexsAssetCtxs(AllDexsAssetCtxsWire),
}

pub type AllMidsWire = BTreeMap<String, String>;
pub type AllDexsClearinghouseStateWire = BTreeMap<String, ClearinghouseStateWire>;
pub type AllDexsAssetCtxsWire = BTreeMap<String, Vec<WsActiveAssetCtxWire>>;
pub type AddressedSubscriptionWire = SubscriptionWire;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationWire {
    pub notification: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WebData2Wire {
    #[serde(rename = "clearinghouseState")]
    pub clearinghouse_state: ClearinghouseStateWire,
    #[serde(rename = "openOrders")]
    pub open_orders: Vec<WsBasicOrder>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearinghouseStateWire {
    #[serde(rename = "assetPositions", default)]
    pub asset_positions: Vec<WsAssetPositionWire>,
    #[serde(rename = "crossMarginSummary")]
    pub cross_margin_summary: Option<WsMarginSummaryWire>,
    #[serde(rename = "marginSummary")]
    pub margin_summary: Option<WsMarginSummaryWire>,
    pub withdrawable: Option<String>,
    #[serde(rename = "crossMaintenanceMarginUsed")]
    pub cross_maintenance_margin_used: Option<String>,
    pub time: Option<u64>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsAssetPositionWire {
    #[serde(rename = "type")]
    pub type_: String,
    pub position: WsPositionWire,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsPositionWire {
    pub coin: String,
    pub sz: String,
    pub side: Option<String>,
    #[serde(rename = "entryPx")]
    pub entry_px: Option<String>,
    pub leverage: Option<WsLeverageWire>,
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsLeverageWire {
    pub value: u32,
    #[serde(rename = "isCross")]
    pub is_cross: bool,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsMarginSummaryWire {
    #[serde(rename = "accountValue")]
    pub account_value: String,
    #[serde(rename = "totalNtlPos")]
    pub total_ntl_pos: String,
    #[serde(rename = "totalRawUsd")]
    pub total_raw_usd: String,
    #[serde(rename = "totalMarginUsed")]
    pub total_margin_used: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsBasicOrder {
    pub coin: String,
    pub side: String,
    pub sz: String,
    #[serde(rename = "limitPx")]
    pub limit_px: Option<String>,
    pub oid: u64,
    pub timestamp: u64,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

pub type OpenOrdersWire = Vec<WsBasicOrder>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandleWire {
    pub t: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    pub s: String,
    pub i: String,
    pub o: String,
    pub c: String,
    pub h: String,
    pub l: String,
    pub v: String,
    #[serde(rename = "n")]
    pub num_trades: u64,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsBookWire {
    pub coin: String,
    pub time: u64,
    pub levels: Vec<Vec<WsLevelWire>>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsLevelWire {
    pub px: String,
    pub sz: String,
    pub n: u64,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsTradeWire {
    pub coin: String,
    pub side: String,
    pub px: String,
    pub sz: String,
    pub hash: Option<String>,
    pub time: u64,
    pub tid: Option<u64>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserEventWire {
    #[serde(flatten, default)]
    pub fields: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserFillsWire {
    #[serde(default)]
    pub is_snapshot: Option<bool>,
    pub fills: Vec<WsFillWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsFillWire {
    pub coin: String,
    pub px: String,
    pub sz: String,
    pub side: String,
    pub time: u64,
    pub start_position: Option<String>,
    pub dir: Option<String>,
    #[serde(rename = "closedPnl")]
    pub closed_pnl: Option<String>,
    pub hash: Option<String>,
    pub oid: Option<u64>,
    pub crossed: Option<bool>,
    pub fee: Option<String>,
    #[serde(rename = "feeToken")]
    pub fee_token: Option<String>,
    #[serde(rename = "builderFee")]
    pub builder_fee: Option<String>,
    pub tid: Option<u64>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserFundingsWire {
    #[serde(default)]
    pub is_snapshot: Option<bool>,
    #[serde(default)]
    pub fundings: Vec<WsUserFundingWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserFundingWire {
    pub coin: Option<String>,
    pub usdc: Option<String>,
    pub funding: Option<String>,
    pub time: Option<u64>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserNonFundingLedgerUpdatesWire {
    #[serde(rename = "nonFundingLedgerUpdates", default)]
    pub non_funding_ledger_updates: Vec<WsUserNonFundingLedgerUpdateWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserNonFundingLedgerUpdateWire {
    pub delta: Option<JsonValue>,
    pub hash: Option<String>,
    pub time: Option<u64>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WsActiveAssetCtxWire {
    Perp(WsActiveAssetDataWire),
    Spot(WsSpotStateWire),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsActiveAssetDataWire {
    pub coin: Option<String>,
    #[serde(rename = "midPx")]
    pub mid_px: Option<String>,
    #[serde(rename = "markPx")]
    pub mark_px: Option<String>,
    #[serde(rename = "oraclePx")]
    pub oracle_px: Option<String>,
    pub funding: Option<String>,
    #[serde(rename = "openInterest")]
    pub open_interest: Option<String>,
    #[serde(rename = "dayNtlVlm")]
    pub day_ntl_vlm: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsSpotStateWire {
    #[serde(default)]
    pub tokens: Vec<JsonValue>,
    #[serde(default)]
    pub universe: Vec<JsonValue>,
    #[serde(rename = "assetCtxs", default)]
    pub asset_ctxs: Vec<JsonValue>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserTwapSliceFillsWire {
    #[serde(default)]
    pub fills: Vec<WsTwapSliceFillWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsTwapSliceFillWire {
    pub coin: Option<String>,
    pub px: Option<String>,
    pub sz: Option<String>,
    pub time: Option<u64>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsUserTwapHistoryWire {
    #[serde(default)]
    pub history: Vec<WsTwapHistoryWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsTwapHistoryWire {
    pub coin: Option<String>,
    pub side: Option<String>,
    pub time: Option<u64>,
    pub sz: Option<String>,
    #[serde(rename = "executedSz")]
    pub executed_sz: Option<String>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsBboWire {
    pub coin: Option<String>,
    pub time: Option<u64>,
    pub bid: Option<WsQuotedLevelWire>,
    pub ask: Option<WsQuotedLevelWire>,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsQuotedLevelWire {
    pub px: String,
    pub sz: String,
    #[serde(flatten, default)]
    pub extra: ExtraFields,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WsPostResponseWire {
    pub id: u64,
    pub response: WsPostResponsePayloadWire,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WsPostResponsePayloadWire {
    Ok { type_: WsPostResponseTypeWire, data: JsonValue },
    Error { raw: JsonValue },
    Unknown { raw: JsonValue },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WsPostResponseTypeWire {
    Info,
    Action,
}

/// 所有服务端文本消息都先过这一个入口做外层判型。
/// 设计上先识别 channel，再把 data 下沉到对应 payload，
/// 未知 channel 不报 fatal，保留 raw 以便前向兼容。
pub struct HyperliquidWsMessageParser;

impl HyperliquidWsMessageParser {
    pub fn parse_text(text: &str) -> Result<WsServerMessageWire, HyperliquidWsError> {
        let value = serde_json::from_str(text).map_err(HyperliquidWsError::Json)?;
        Self::parse_value(value)
    }

    pub fn parse_value(value: JsonValue) -> Result<WsServerMessageWire, HyperliquidWsError> {
        let Some(object) = value.as_object() else {
            return Err(HyperliquidWsError::Protocol(
                "outer message must be a JSON object".to_string(),
            ));
        };

        // Hyperliquid 的服务端推送外层靠 `channel` 做总分流。
        // 如果连这个关键字段都没有，就先落到 Unknown，让上层自己决定如何处理。
        let Some(channel) = object.get("channel").and_then(JsonValue::as_str) else {
            return Ok(WsServerMessageWire::Unknown { raw: value });
        };

        match channel {
            "subscriptionResponse" => {
                Ok(WsServerMessageWire::SubscriptionResponse(from_data(object, "data")?))
            }
            "pong" => Ok(WsServerMessageWire::Pong),
            "post" => Ok(WsServerMessageWire::Post(parse_post_response(object)?)),
            "allMids" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::AllMids(
                from_data(object, "data")?,
            ))),
            "notification" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::Notification(
                from_data(object, "data")?,
            ))),
            "webData2" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::WebData2(
                from_data(object, "data")?,
            ))),
            "candle" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::Candle(from_data(
                object, "data",
            )?))),
            "l2Book" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::L2Book(from_data(
                object, "data",
            )?))),
            "trades" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::Trades(from_data(
                object, "data",
            )?))),
            "orderUpdates" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::OrderUpdates(
                from_data(object, "data")?,
            ))),
            "userEvents" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::UserEvents(
                from_data(object, "data")?,
            ))),
            "userFills" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::UserFills(
                from_data(object, "data")?,
            ))),
            "userFundings" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::UserFundings(
                from_data(object, "data")?,
            ))),
            "userNonFundingLedgerUpdates" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::UserNonFundingLedgerUpdates(from_data(object, "data")?),
            )),
            "activeAssetCtx" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::ActiveAssetCtx(from_data(object, "data")?),
            )),
            "activeAssetData" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::ActiveAssetData(from_data(object, "data")?),
            )),
            "userTwapSliceFills" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::UserTwapSliceFills(from_data(object, "data")?),
            )),
            "userTwapHistory" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::UserTwapHistory(from_data(object, "data")?),
            )),
            "bbo" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::Bbo(from_data(
                object, "data",
            )?))),
            "userToaster" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::UserToaster(
                from_data(object, "data")?,
            ))),
            "userRequest" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::UserRequest(
                object.get("data").cloned().ok_or_else(|| {
                    HyperliquidWsError::Protocol(
                        "channel `userRequest` is missing `data`".to_string(),
                    )
                })?,
            ))),
            "ledgerUpdates" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::LedgerUpdates(from_data(object, "data")?),
            )),
            "twapStates" => Ok(WsServerMessageWire::Channel(WsChannelMessageWire::TwapStates(
                object.get("data").cloned().ok_or_else(|| {
                    HyperliquidWsError::Protocol(
                        "channel `twapStates` is missing `data`".to_string(),
                    )
                })?,
            ))),
            "allDexsClearinghouseState" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::AllDexsClearinghouseState(from_data(object, "data")?),
            )),
            "allDexsAssetCtxs" => Ok(WsServerMessageWire::Channel(
                WsChannelMessageWire::AllDexsAssetCtxs(from_data(object, "data")?),
            )),
            _ => Ok(WsServerMessageWire::Unknown { raw: value }),
        }
    }
}

fn from_data<T>(
    object: &serde_json::Map<String, JsonValue>,
    field: &str,
) -> Result<T, HyperliquidWsError>
where
    T: for<'de> Deserialize<'de>,
{
    // 已知 channel 默认要求存在 `data`；真正的兼容策略放在 payload 自身的 extra 字段里，
    // 而不是把缺少关键字段也默默吞掉。
    let value = object.get(field).cloned().ok_or_else(|| {
        HyperliquidWsError::Protocol(format!("channel payload is missing `{field}`"))
    })?;
    serde_json::from_value(value).map_err(HyperliquidWsError::Json)
}

fn parse_post_response(
    object: &serde_json::Map<String, JsonValue>,
) -> Result<WsPostResponseWire, HyperliquidWsError> {
    // `post` 是少数需要二次解包的服务端消息：
    // 外层是 channel=post，内层 `data` 再带 id/response。
    let data = object.get("data").and_then(JsonValue::as_object).ok_or_else(|| {
        HyperliquidWsError::Protocol("channel `post` must contain object `data`".to_string())
    })?;
    let id = data.get("id").and_then(JsonValue::as_u64).ok_or_else(|| {
        HyperliquidWsError::Protocol("channel `post` is missing numeric `data.id`".to_string())
    })?;
    let response = data.get("response").cloned().ok_or_else(|| {
        HyperliquidWsError::Protocol("channel `post` is missing `data.response`".to_string())
    })?;
    Ok(WsPostResponseWire { id, response: classify_post_response(response) })
}

fn classify_post_response(response: JsonValue) -> WsPostResponsePayloadWire {
    // `post` 的成功响应目前按 `type=info|action` 归类；
    // 错误响应优先识别 `error`，其余未知形状全部保留 raw，避免误判成功。
    let Some(object) = response.as_object() else {
        return WsPostResponsePayloadWire::Unknown { raw: response };
    };
    if let Some(type_) = object.get("type").and_then(JsonValue::as_str) {
        let kind = match type_ {
            "info" => WsPostResponseTypeWire::Info,
            "action" => WsPostResponseTypeWire::Action,
            _ => return WsPostResponsePayloadWire::Unknown { raw: response },
        };
        let data = object.get("payload").cloned().unwrap_or(JsonValue::Object(object.clone()));
        return WsPostResponsePayloadWire::Ok { type_: kind, data };
    }
    if object.contains_key("error") {
        return WsPostResponsePayloadWire::Error { raw: response };
    }
    WsPostResponsePayloadWire::Unknown { raw: response }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    #[test]
    fn serializes_all_subscription_variants_with_official_type_names() {
        let subscriptions = vec![
            (SubscriptionWire::AllMids { dex: None }, "allMids"),
            (SubscriptionWire::Notification { user: addr() }, "notification"),
            (SubscriptionWire::WebData2 { user: addr() }, "webData2"),
            (
                SubscriptionWire::Candle {
                    coin: "BTC".to_string(),
                    interval: CandleIntervalWire("1m".to_string()),
                },
                "candle",
            ),
            (
                SubscriptionWire::L2Book {
                    coin: "BTC".to_string(),
                    n_sig_figs: Some(2),
                    mantissa: Some(1),
                    dex: Some("".to_string()),
                },
                "l2Book",
            ),
            (SubscriptionWire::Trades { coin: "BTC".to_string() }, "trades"),
            (SubscriptionWire::OrderUpdates { user: addr() }, "orderUpdates"),
            (SubscriptionWire::UserEvents { user: addr() }, "userEvents"),
            (
                SubscriptionWire::UserFills { user: addr(), aggregate_by_time: Some(true) },
                "userFills",
            ),
            (SubscriptionWire::UserFundings { user: addr() }, "userFundings"),
            (
                SubscriptionWire::UserNonFundingLedgerUpdates { user: addr() },
                "userNonFundingLedgerUpdates",
            ),
            (SubscriptionWire::ActiveAssetCtx { coin: "BTC".to_string() }, "activeAssetCtx"),
            (
                SubscriptionWire::ActiveAssetData { user: addr(), coin: "BTC".to_string() },
                "activeAssetData",
            ),
            (SubscriptionWire::UserTwapSliceFills { user: addr() }, "userTwapSliceFills"),
            (SubscriptionWire::UserTwapHistory { user: addr() }, "userTwapHistory"),
            (SubscriptionWire::Bbo { coin: "BTC".to_string() }, "bbo"),
            (SubscriptionWire::UserToaster { user: addr() }, "userToaster"),
            (SubscriptionWire::UserRequest { user: addr() }, "userRequest"),
            (SubscriptionWire::LedgerUpdates { user: addr() }, "ledgerUpdates"),
            (SubscriptionWire::TwapStates { user: addr() }, "twapStates"),
            (
                SubscriptionWire::AllDexsClearinghouseState { user: addr() },
                "allDexsClearinghouseState",
            ),
            (SubscriptionWire::AllDexsAssetCtxs {}, "allDexsAssetCtxs"),
        ];

        for (subscription, expected_type) in subscriptions {
            let value = serde_json::to_value(subscription).expect("subscription should serialize");
            assert_eq!(value["type"], expected_type);
        }
    }

    #[test]
    fn serializes_client_messages() {
        let subscribe = serde_json::to_value(WsClientMessageWire::Subscribe {
            subscription: SubscriptionWire::AllMids { dex: None },
        })
        .expect("subscribe serializes");
        assert_eq!(
            subscribe,
            json!({
                "method": "subscribe",
                "subscription": { "type": "allMids" }
            })
        );

        let ping =
            serde_json::to_value(WsClientMessageWire::Ping(PingWire)).expect("ping serializes");
        assert_eq!(ping, json!({ "method": "ping" }));

        let action = serde_json::to_value(WsClientMessageWire::Post {
            id: 9,
            request: WsPostRequestWire::Action(ExchangeActionRequestWire {
                action: crate::exchange::ExchangeActionWire::Noop,
                common: crate::exchange::CommonExchangeFields {
                    nonce: 1710000000000,
                    signature: crate::exchange::SignatureWire {
                        r: "0x1111111111111111111111111111111111111111111111111111111111111111"
                            .to_string(),
                        s: "0x2222222222222222222222222222222222222222222222222222222222222222"
                            .to_string(),
                        v: 27,
                    },
                    vault_address: None,
                    expires_after: None,
                },
            }),
        })
        .expect("action post serializes");
        assert_eq!(
            action,
            json!({
                "method": "post",
                "id": 9,
                "request": {
                    "type": "action",
                    "payload": {
                        "action": { "type": "noop" },
                        "nonce": 1710000000000u64,
                        "signature": {
                            "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                            "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                            "v": 27
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn parses_subscription_response() {
        let message = HyperliquidWsMessageParser::parse_value(json!({
            "channel": "subscriptionResponse",
            "data": {
                "method": "subscribe",
                "subscription": { "type": "allMids" }
            }
        }))
        .expect("subscription response parses");

        match message {
            WsServerMessageWire::SubscriptionResponse(response) => {
                assert_eq!(response.method, "subscribe");
                assert_eq!(response.subscription, SubscriptionWire::AllMids { dex: None });
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }

    #[test]
    fn parses_known_channel_payloads() {
        let cases: Vec<(&str, Value)> = vec![
            ("allMids", json!({ "BTC": "60000.0" })),
            ("notification", json!({ "notification": "ok" })),
            (
                "webData2",
                json!({
                    "clearinghouseState": {
                        "assetPositions": [],
                        "marginSummary": {
                            "accountValue": "100",
                            "totalNtlPos": "0",
                            "totalRawUsd": "100",
                            "totalMarginUsed": "0"
                        }
                    },
                    "openOrders": []
                }),
            ),
            (
                "candle",
                json!([{
                    "t": 1,
                    "T": 2,
                    "s": "BTC",
                    "i": "1m",
                    "o": "1",
                    "c": "1",
                    "h": "1",
                    "l": "1",
                    "v": "1",
                    "n": 1
                }]),
            ),
            (
                "l2Book",
                json!({
                    "coin": "BTC",
                    "time": 1,
                    "levels": [
                        [{ "px": "1", "sz": "2", "n": 1 }],
                        [{ "px": "3", "sz": "4", "n": 1 }]
                    ]
                }),
            ),
            (
                "trades",
                json!([{
                    "coin": "BTC",
                    "side": "B",
                    "px": "1",
                    "sz": "2",
                    "time": 1
                }]),
            ),
            (
                "orderUpdates",
                json!([{
                    "coin": "BTC",
                    "side": "B",
                    "sz": "2",
                    "limitPx": "1",
                    "oid": 1,
                    "timestamp": 1
                }]),
            ),
            ("userEvents", json!({ "fills": [] })),
            (
                "userFills",
                json!({
                    "is_snapshot": true,
                    "fills": [{
                        "coin": "BTC",
                        "px": "1",
                        "sz": "1",
                        "side": "B",
                        "time": 1
                    }]
                }),
            ),
            ("userFundings", json!({ "fundings": [{ "coin": "BTC", "funding": "1", "time": 1 }] })),
            (
                "userNonFundingLedgerUpdates",
                json!({ "nonFundingLedgerUpdates": [{ "time": 1, "delta": { "type": "deposit" } }] }),
            ),
            ("activeAssetCtx", json!({ "coin": "BTC", "markPx": "1" })),
            ("activeAssetData", json!({ "coin": "BTC", "midPx": "1" })),
            ("userTwapSliceFills", json!({ "fills": [{ "coin": "BTC", "time": 1 }] })),
            ("userTwapHistory", json!({ "history": [{ "coin": "BTC", "time": 1 }] })),
            ("bbo", json!({ "coin": "BTC", "time": 1, "bid": { "px": "1", "sz": "1" } })),
            ("userToaster", json!({ "notification": "toast" })),
            ("userRequest", json!({ "status": "ok" })),
            ("ledgerUpdates", json!({ "nonFundingLedgerUpdates": [] })),
            ("twapStates", json!({ "states": [] })),
            ("allDexsClearinghouseState", json!({ "": { "assetPositions": [] } })),
            ("allDexsAssetCtxs", json!({ "": [{ "coin": "BTC", "markPx": "1" }] })),
        ];

        for (channel, data) in cases {
            let message = HyperliquidWsMessageParser::parse_value(json!({
                "channel": channel,
                "data": data
            }))
            .expect("channel payload should parse");
            assert!(
                matches!(message, WsServerMessageWire::Channel(_)),
                "{channel} should be a channel"
            );
        }
    }

    #[test]
    fn parses_post_success_and_error_responses() {
        let success = HyperliquidWsMessageParser::parse_value(json!({
            "channel": "post",
            "data": {
                "id": 7,
                "response": {
                    "type": "info",
                    "payload": { "ok": true }
                }
            }
        }))
        .expect("post success parses");

        match success {
            WsServerMessageWire::Post(response) => {
                assert_eq!(response.id, 7);
                assert!(matches!(
                    response.response,
                    WsPostResponsePayloadWire::Ok { type_: WsPostResponseTypeWire::Info, .. }
                ));
            }
            other => panic!("unexpected message: {other:?}"),
        }

        let error = HyperliquidWsMessageParser::parse_value(json!({
            "channel": "post",
            "data": {
                "id": 8,
                "response": {
                    "error": "bad request"
                }
            }
        }))
        .expect("post error parses");

        match error {
            WsServerMessageWire::Post(response) => {
                assert!(matches!(response.response, WsPostResponsePayloadWire::Error { .. }));
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }

    #[test]
    fn parses_pong_and_tolerates_unknown_messages() {
        let pong = HyperliquidWsMessageParser::parse_value(json!({ "channel": "pong" }))
            .expect("pong parses");
        assert!(matches!(pong, WsServerMessageWire::Pong));

        let unknown = HyperliquidWsMessageParser::parse_value(json!({
            "channel": "futureChannel",
            "data": { "x": 1 }
        }))
        .expect("unknown channel should not be fatal");
        assert!(matches!(unknown, WsServerMessageWire::Unknown { .. }));
    }

    #[test]
    fn allows_forward_compatible_extra_fields() {
        let message = HyperliquidWsMessageParser::parse_value(json!({
            "channel": "userFills",
            "data": {
                "fills": [{
                    "coin": "BTC",
                    "px": "1",
                    "sz": "2",
                    "side": "B",
                    "time": 1,
                    "newField": "kept"
                }],
                "brandNewWrapperField": 7
            }
        }))
        .expect("user fills with extra fields should parse");

        match message {
            WsServerMessageWire::Channel(WsChannelMessageWire::UserFills(payload)) => {
                assert_eq!(payload.extra.get("brandNewWrapperField"), Some(&json!(7)));
                assert_eq!(payload.fills[0].extra.get("newField"), Some(&json!("kept")));
            }
            other => panic!("unexpected message: {other:?}"),
        }
    }

    fn addr() -> String {
        "0x1111111111111111111111111111111111111111".to_string()
    }
}
