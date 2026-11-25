use serde::{Deserialize, Serialize};

// ============ 顶层 Block ============
#[derive(Debug, Deserialize, Serialize)]
pub struct ReplicaCmdsBlock {
    /// 顺序 block 号（1085381001 之后才有）
    #[serde(default)]
    pub height: Option<u64>,
    #[serde(rename = "abci_block")]
    pub abci_block: AbciBlock,
    #[serde(rename = "signed_action_bundles")]
    pub signed_action_bundles: Vec<BundleEntry>,
    pub resps: Resps,
}

// ============ 共识元数据 ============
#[derive(Debug, Deserialize, Serialize)]
pub struct AbciBlock {
    pub time: String, // ISO 8601
    pub round: u64,   // ⚠️ 不是 block 号，会跳号
    #[serde(rename = "parent_round")]
    pub parent_round: u64,
    pub proposer: String, // 0x... 验证者地址
    pub hardfork: Hardfork,
    #[serde(default)]
    pub signed_action_bundles: Option<Vec<BundleEntry>>, // 某些样本里也嵌在这
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hardfork {
    pub version: u32,
    pub round: u64,
}

// ============ Bundle（元组：[hash, payload]） ============
/// 外层是 [bundle_hash, BundlePayload]
pub type BundleEntry = (String, BundlePayload);

#[derive(Debug, Deserialize, Serialize)]
pub struct BundlePayload {
    #[serde(rename = "signed_actions")]
    pub signed_actions: Vec<SignedAction>,
    pub broadcaster: String,
    #[serde(default, rename = "broadcaster_nonce")]
    pub broadcaster_nonce: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignedAction {
    pub signature: Signature,
    /// action 是 enum，见下方 ActionEnum
    #[serde(flatten)]
    pub action_wrapper: ActionWrapper,
    pub nonce: u64,
    #[serde(default, rename = "vaultAddress")]
    pub vault_address: Option<String>,
    #[serde(default, rename = "expiresAfter")]
    pub expires_after: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub r: String,
    pub s: String,
    pub v: u8, // 或 i32，看具体数据
}

/// action 字段用 flatten + enum 区分 type
#[derive(Debug, Deserialize, Serialize)]
pub struct ActionWrapper {
    #[serde(flatten)]
    pub action: ActionEnum,
}

// ============ Action 类型（核心 enum） ============
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ActionEnum {
    #[serde(rename = "order")]
    Order(BatchOrder),
    #[serde(rename = "cancel")]
    Cancel(CancelAction),
    #[serde(rename = "cancelByCloid")]
    CancelByCloid(CancelByCloidAction),
    #[serde(rename = "batchModify")]
    BatchModify(BatchModify),
    #[serde(rename = "scheduleCancel")]
    ScheduleCancel(ScheduleCancel),
    #[serde(rename = "updateLeverage")]
    UpdateLeverage(UpdateLeverage),
    #[serde(rename = "updateIsolatedMargin")]
    UpdateIsolatedMargin(UpdateIsolatedMargin),
    #[serde(rename = "usdSend")]
    UsdSend(UsdSendAction),
    #[serde(rename = "spotSend")]
    SpotSend(SpotSendAction),
    #[serde(rename = "noop")]
    Noop,
    // 其他 action 按需加
}

// ============ Order Wire 格式（紧凑字段 a,b,p,s,r,t,c） ============
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchOrder {
    #[serde(default)]
    pub grouping: Option<String>,
    pub orders: Vec<OrderWire>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderWire {
    pub a: u32,       // asset id
    pub b: bool,      // isBuy
    pub p: String,    // price
    pub s: String,    // size
    pub r: bool,      // reduceOnly
    pub t: OrderType, // limit {tif} | trigger {isMarket, triggerPx, tpsl}
    #[serde(default)]
    pub c: Option<String>, // cloid
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrderType {
    Limit { limit: LimitTif },
    Trigger { trigger: TriggerParams },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LimitTif {
    pub tif: String, // Gtc | Ioc | Alo
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TriggerParams {
    #[serde(default)]
    pub is_market: Option<bool>,
    #[serde(default, rename = "triggerPx")]
    pub trigger_px: Option<String>,
    #[serde(default)]
    pub tpsl: Option<String>,
}

// ============ Cancel ============
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelAction {
    pub cancels: Vec<CancelTarget>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelTarget {
    pub a: u32,
    pub oid: u64,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelByCloidAction {
    pub cancels: Vec<CancelByCloidTarget>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CancelByCloidTarget {
    pub a: u32,
    pub cloid: String,
}

// ============ 其他 Action（骨架） ============
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchModify {/* 按需展开 */}
#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleCancel {
    #[serde(default)]
    pub t: Option<u64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateLeverage {
    pub asset: u32,
    pub leverage: u32,
    #[serde(default)]
    pub is_cross: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub amount: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UsdSendAction {
    pub destination: String,
    pub amount: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SpotSendAction {
    pub token: String,
    pub amount: String,
    pub destination: String,
}

// ============ resps（执行结果，最关键的部分） ============
#[derive(Debug, Deserialize, Serialize)]
pub struct Resps {
    /// Full: [[bundle_hash, [ {user, res}, ... ] ], ...]
    #[serde(rename = "Full")]
    pub full: Vec<BundleRespEntry>,
}

pub type BundleRespEntry = (String, Vec<UserResp>);

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResp {
    pub user: String,
    pub res: ActionResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ActionResponse {
    pub status: String, // "ok" | "err"
    #[serde(default)]
    pub response: Option<ResponseBody>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    #[serde(rename = "type")]
    pub typ: String, // "order" | "cancel" | "default"
    #[serde(default)]
    pub data: Option<ResponseData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseData {
    /// 这就是你前面反复问的 statuses[]
    pub statuses: Vec<OrderResult>,
}

// ============ statuses[] 的三态（untagged enum） ============
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OrderResult {
    Resting { resting: RestingData },
    Filled { filled: FilledData },
    Error { error: String },
    Success(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RestingData {
    pub oid: u64,
    #[serde(default)]
    pub cloid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilledData {
    #[serde(rename = "totalSz")]
    pub total_sz: String,
    #[serde(rename = "avgPx")]
    pub avg_px: String,
    pub oid: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_order_then_cancel_by_cloid_in_the_same_bundle() {
        let bundle_hash = "0xbundle";
        let user = "0xuser";
        let asset = 10_000;
        let cloid = "0x00000000000000000000000000000001";

        let block = ReplicaCmdsBlock {
            height: Some(1_085_381_002),
            abci_block: AbciBlock {
                time: "2026-11-25T00:00:00Z".to_owned(),
                round: 42,
                parent_round: 41,
                proposer: "0xvalidator".to_owned(),
                hardfork: Hardfork {
                    version: 1,
                    round: 42,
                },
                signed_action_bundles: None,
            },
            signed_action_bundles: vec![(
                bundle_hash.to_owned(),
                BundlePayload {
                    signed_actions: vec![
                        SignedAction {
                            signature: signature(),
                            action_wrapper: ActionWrapper {
                                action: ActionEnum::Order(BatchOrder {
                                    grouping: None,
                                    orders: vec![OrderWire {
                                        a: asset,
                                        b: true,
                                        p: "12.34".to_owned(),
                                        s: "5".to_owned(),
                                        r: false,
                                        t: OrderType::Limit {
                                            limit: LimitTif {
                                                tif: "Gtc".to_owned(),
                                            },
                                        },
                                        c: Some(cloid.to_owned()),
                                    }],
                                }),
                            },
                            nonce: 1_764_028_800_000,
                            vault_address: None,
                            expires_after: None,
                        },
                        SignedAction {
                            signature: signature(),
                            action_wrapper: ActionWrapper {
                                action: ActionEnum::CancelByCloid(CancelByCloidAction {
                                    cancels: vec![CancelByCloidTarget {
                                        a: asset,
                                        cloid: cloid.to_owned(),
                                    }],
                                }),
                            },
                            nonce: 1_764_028_800_001,
                            vault_address: None,
                            expires_after: None,
                        },
                    ],
                    broadcaster: user.to_owned(),
                    broadcaster_nonce: None,
                },
            )],
            resps: Resps {
                full: vec![(
                    bundle_hash.to_owned(),
                    vec![
                        UserResp {
                            user: user.to_owned(),
                            res: successful_response(
                                "order",
                                OrderResult::Resting {
                                    resting: RestingData {
                                        oid: 123_456,
                                        cloid: Some(cloid.to_owned()),
                                    },
                                },
                            ),
                        },
                        UserResp {
                            user: user.to_owned(),
                            res: successful_response(
                                "cancel",
                                OrderResult::Success("success".to_owned()),
                            ),
                        },
                    ],
                )],
            },
        };

        let (action_bundle_hash, bundle) = &block.signed_action_bundles[0];
        let order = match &bundle.signed_actions[0].action_wrapper.action {
            ActionEnum::Order(batch) => &batch.orders[0],
            other => panic!("第一个 action 应为 Order，实际为 {other:?}"),
        };
        let cancel = match &bundle.signed_actions[1].action_wrapper.action {
            ActionEnum::CancelByCloid(action) => &action.cancels[0],
            other => panic!("第二个 action 应为 CancelByCloid，实际为 {other:?}"),
        };

        assert!(bundle.signed_actions[0].nonce < bundle.signed_actions[1].nonce);
        assert_eq!(order.a, cancel.a);
        assert_eq!(order.c.as_deref(), Some(cancel.cloid.as_str()));

        let (response_bundle_hash, responses) = &block.resps.full[0];
        assert_eq!(response_bundle_hash, action_bundle_hash);
        assert_eq!(responses.len(), bundle.signed_actions.len());
        assert!(responses.iter().all(|response| response.user == user));

        let order_response = response_body(&responses[0]);
        assert_eq!(order_response.typ, "order");
        match &order_response.data.as_ref().unwrap().statuses[0] {
            OrderResult::Resting { resting } => {
                assert_eq!(resting.oid, 123_456);
                assert_eq!(resting.cloid.as_deref(), Some(cloid));
            }
            other => panic!("下单响应应为 Resting，实际为 {other:?}"),
        }

        let cancel_response = response_body(&responses[1]);
        assert_eq!(cancel_response.typ, "cancel");
        match &cancel_response.data.as_ref().unwrap().statuses[0] {
            OrderResult::Success(status) => assert_eq!(status, "success"),
            other => panic!("取消响应应为 success，实际为 {other:?}"),
        }
    }

    fn signature() -> Signature {
        Signature {
            r: "0xr".to_owned(),
            s: "0xs".to_owned(),
            v: 27,
        }
    }

    fn successful_response(typ: &str, result: OrderResult) -> ActionResponse {
        ActionResponse {
            status: "ok".to_owned(),
            response: Some(ResponseBody {
                typ: typ.to_owned(),
                data: Some(ResponseData {
                    statuses: vec![result],
                }),
            }),
        }
    }

    fn response_body(response: &UserResp) -> &ResponseBody {
        assert_eq!(response.res.status, "ok");
        response.res.response.as_ref().unwrap()
    }
}
