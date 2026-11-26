use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Number, Value};

use super::ExchangeV2Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExchangeRequestV2 {
    pub action: ExchangeActionV2,
    pub nonce: u64,
    pub signature: SignatureV2,
    #[serde(rename = "vaultAddress")]
    pub vault_address: Option<String>,
    #[serde(rename = "expiresAfter")]
    pub expires_after: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignatureV2 {
    r: String,
    s: String,
    v: u64,
}

impl ExchangeRequestV2 {
    pub fn validate_common(&self) -> Result<(), ExchangeV2Error> {
        if self.nonce == 0 {
            return Err(ExchangeV2Error::InvalidNonce);
        }
        if self.expires_after == Some(0) {
            return Err(ExchangeV2Error::InvalidExpiresAfter);
        }
        if !hex_with_len(&self.signature.r, 64)
            || !hex_with_len(&self.signature.s, 64)
            || self.signature.v > 255
        {
            return Err(ExchangeV2Error::InvalidSignature);
        }
        if self
            .vault_address
            .as_deref()
            .is_some_and(|value| !hex_with_len(value, 40))
        {
            return Err(ExchangeV2Error::InvalidVaultAddress);
        }
        Ok(())
    }
}

fn hex_with_len(value: &str, len: usize) -> bool {
    value
        .strip_prefix("0x")
        .is_some_and(|hex| hex.len() == len && hex.bytes().all(|byte| byte.is_ascii_hexdigit()))
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExchangeActionV2 {
    AgentEnableDexAbstraction(EmptyPayload),
    AgentSendAsset(AgentSendAssetPayload),
    AgentSetAbstraction(AbstractionPayload),
    ApproveAgent(ApproveAgentPayload),
    ApproveBuilderFee(ApproveBuilderFeePayload),
    AuthorizeAqav2Role(AuthorizeRolePayload),
    BatchModify(BatchModifyPayload),
    CDeposit(BridgePayload),
    CWithdraw(BridgePayload),
    Cancel(CancelPayload),
    CancelByCloid(CancelByCloidPayload),
    ClaimRewards(EmptyPayload),
    Hip3LiquidatorTransfer(Hip3Payload),
    Modify(ModifyPayload),
    Noop(EmptyPayload),
    Order(OrderPayload),
    ReserveRequestWeight(ReserveWeightPayload),
    ScheduleCancel(ScheduleCancelPayload),
    SendAsset(SendAssetPayload),
    SendToEvmWithData(SendToEvmPayload),
    SpotSend(SpotSendPayload),
    TokenDelegate(TokenDelegatePayload),
    TopUpIsolatedOnlyMargin(TopUpPayload),
    TwapCancel(TwapCancelPayload),
    TwapOrder(TwapOrderPayload),
    UpdateIsolatedMargin(UpdateIsolatedMarginPayload),
    UpdateLeverage(UpdateLeveragePayload),
    UsdClassTransfer(UsdClassTransferPayload),
    UsdSend(UsdSendPayload),
    UserDexAbstraction(UserDexPayload),
    UserOutcome(UserOutcomePayload),
    UserSetAbstraction(UserSetAbstractionPayload),
    ValidatorL1Stream(ValidatorPayload),
    VaultTransfer(VaultTransferPayload),
    Withdraw3(WithdrawPayload),
    Unknown { type_: String },
}

impl Serialize for ExchangeActionV2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        macro_rules! serialize_payload {
            ($payload:expr) => {
                $payload.serialize(serializer)
            };
        }
        match self {
            Self::AgentEnableDexAbstraction(value) => serialize_payload!(value),
            Self::AgentSendAsset(value) => serialize_payload!(value),
            Self::AgentSetAbstraction(value) => serialize_payload!(value),
            Self::ApproveAgent(value) => serialize_payload!(value),
            Self::ApproveBuilderFee(value) => serialize_payload!(value),
            Self::AuthorizeAqav2Role(value) => serialize_payload!(value),
            Self::BatchModify(value) => serialize_payload!(value),
            Self::CDeposit(value) => serialize_payload!(value),
            Self::CWithdraw(value) => serialize_payload!(value),
            Self::Cancel(value) => serialize_payload!(value),
            Self::CancelByCloid(value) => serialize_payload!(value),
            Self::ClaimRewards(value) => serialize_payload!(value),
            Self::Hip3LiquidatorTransfer(value) => serialize_payload!(value),
            Self::Modify(value) => serialize_payload!(value),
            Self::Noop(value) => serialize_payload!(value),
            Self::Order(value) => serialize_payload!(value),
            Self::ReserveRequestWeight(value) => serialize_payload!(value),
            Self::ScheduleCancel(value) => serialize_payload!(value),
            Self::SendAsset(value) => serialize_payload!(value),
            Self::SendToEvmWithData(value) => serialize_payload!(value),
            Self::SpotSend(value) => serialize_payload!(value),
            Self::TokenDelegate(value) => serialize_payload!(value),
            Self::TopUpIsolatedOnlyMargin(value) => serialize_payload!(value),
            Self::TwapCancel(value) => serialize_payload!(value),
            Self::TwapOrder(value) => serialize_payload!(value),
            Self::UpdateIsolatedMargin(value) => serialize_payload!(value),
            Self::UpdateLeverage(value) => serialize_payload!(value),
            Self::UsdClassTransfer(value) => serialize_payload!(value),
            Self::UsdSend(value) => serialize_payload!(value),
            Self::UserDexAbstraction(value) => serialize_payload!(value),
            Self::UserOutcome(value) => serialize_payload!(value),
            Self::UserSetAbstraction(value) => serialize_payload!(value),
            Self::ValidatorL1Stream(value) => serialize_payload!(value),
            Self::VaultTransfer(value) => serialize_payload!(value),
            Self::Withdraw3(value) => serialize_payload!(value),
            Self::Unknown { type_ } => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", type_)?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ExchangeActionV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let type_ = value
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("missing field `type`"))?;
        macro_rules! parse {
            ($variant:ident, $payload:ty) => {
                serde_json::from_value::<$payload>(value)
                    .map(Self::$variant)
                    .map_err(serde::de::Error::custom)
            };
        }
        match type_ {
            "agentEnableDexAbstraction" => parse!(AgentEnableDexAbstraction, EmptyPayload),
            "agentSendAsset" => parse!(AgentSendAsset, AgentSendAssetPayload),
            "agentSetAbstraction" => parse!(AgentSetAbstraction, AbstractionPayload),
            "approveAgent" => parse!(ApproveAgent, ApproveAgentPayload),
            "approveBuilderFee" => parse!(ApproveBuilderFee, ApproveBuilderFeePayload),
            "authorizeAqav2Role" => parse!(AuthorizeAqav2Role, AuthorizeRolePayload),
            "batchModify" => parse!(BatchModify, BatchModifyPayload),
            "cDeposit" => parse!(CDeposit, BridgePayload),
            "cWithdraw" => parse!(CWithdraw, BridgePayload),
            "cancel" => parse!(Cancel, CancelPayload),
            "cancelByCloid" => parse!(CancelByCloid, CancelByCloidPayload),
            "claimRewards" => parse!(ClaimRewards, EmptyPayload),
            "hip3LiquidatorTransfer" => parse!(Hip3LiquidatorTransfer, Hip3Payload),
            "modify" => parse!(Modify, ModifyPayload),
            "noop" => parse!(Noop, EmptyPayload),
            "order" => parse!(Order, OrderPayload),
            "reserveRequestWeight" => parse!(ReserveRequestWeight, ReserveWeightPayload),
            "scheduleCancel" => parse!(ScheduleCancel, ScheduleCancelPayload),
            "sendAsset" => parse!(SendAsset, SendAssetPayload),
            "sendToEvmWithData" => parse!(SendToEvmWithData, SendToEvmPayload),
            "spotSend" => parse!(SpotSend, SpotSendPayload),
            "tokenDelegate" => parse!(TokenDelegate, TokenDelegatePayload),
            "topUpIsolatedOnlyMargin" => parse!(TopUpIsolatedOnlyMargin, TopUpPayload),
            "twapCancel" => parse!(TwapCancel, TwapCancelPayload),
            "twapOrder" => parse!(TwapOrder, TwapOrderPayload),
            "updateIsolatedMargin" => parse!(UpdateIsolatedMargin, UpdateIsolatedMarginPayload),
            "updateLeverage" => parse!(UpdateLeverage, UpdateLeveragePayload),
            "usdClassTransfer" => parse!(UsdClassTransfer, UsdClassTransferPayload),
            "usdSend" => parse!(UsdSend, UsdSendPayload),
            "userDexAbstraction" => parse!(UserDexAbstraction, UserDexPayload),
            "userOutcome" => parse!(UserOutcome, UserOutcomePayload),
            "userSetAbstraction" => parse!(UserSetAbstraction, UserSetAbstractionPayload),
            "validatorL1Stream" => parse!(ValidatorL1Stream, ValidatorPayload),
            "vaultTransfer" => parse!(VaultTransfer, VaultTransferPayload),
            "withdraw3" => parse!(Withdraw3, WithdrawPayload),
            other => Ok(Self::Unknown {
                type_: other.to_owned(),
            }),
        }
    }
}

macro_rules! payload {
    ($name:ident { $($fields:tt)* }) => {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name { #[serde(rename = "type")] _type: String, $($fields)* }
    };
}

payload!(EmptyPayload {});
payload!(AgentSendAssetPayload {
    destination: String,
    #[serde(rename = "sourceDex")]
    source_dex: String,
    #[serde(rename = "destinationDex")]
    destination_dex: String,
    token: String,
    amount: String,
    #[serde(rename = "fromSubAccount")]
    from_sub_account: String,
    nonce: u64,
});
payload!(AbstractionPayload {
    abstraction: String,
});
payload!(ApproveAgentPayload { #[serde(rename="hyperliquidChain")] chain: String, #[serde(rename="signatureChainId")] chain_id: String, #[serde(rename="agentAddress")] agent_address: String, #[serde(rename="agentName")] agent_name: Option<String>, nonce: u64, });
payload!(ApproveBuilderFeePayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    #[serde(rename = "maxFeeRate")]
    max_fee_rate: String,
    builder: String,
    nonce: u64,
});
payload!(AuthorizeRolePayload {
    token: u64,
    role: String,
});
payload!(BridgePayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    wei: u64,
    nonce: u64,
});
payload!(Hip3Payload {
    #[serde(rename = "dexAddress")]
    dex_address: String,
    amount: String,
});
payload!(ReserveWeightPayload { weight: u64 });
payload!(ScheduleCancelPayload { time: Option<u64>, });
payload!(TopUpPayload { asset: u32 });
payload!(TwapCancelPayload { a: u32, t: u64 });
payload!(UpdateIsolatedMarginPayload {
    asset: u32,
    #[serde(rename = "isBuy")]
    is_buy: bool,
    ntli: i64,
});
payload!(UpdateLeveragePayload {
    asset: u32,
    #[serde(rename = "isCross")]
    is_cross: bool,
    leverage: u64,
});
payload!(ValidatorPayload {
    #[serde(rename = "riskFreeRate")]
    risk_free_rate: String,
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelItem {
    pub a: u32,
    pub o: u64,
}
payload!(CancelPayload { pub cancels: Vec<CancelItem>, });
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloidCancelItem {
    pub asset: u32,
    pub cloid: String,
}
payload!(CancelByCloidPayload { pub cancels: Vec<CloidCancelItem>, });

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderPayload {
    #[serde(rename = "type")]
    _type: String,
    pub orders: Vec<OrderV2>,
    grouping: String,
    builder: Option<BuilderV2>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderV2 {
    a: u32,
    b: bool,
    p: String,
    s: String,
    r: bool,
    t: OrderTypeV2,
    c: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OrderTypeV2 {
    limit: Option<LimitV2>,
    trigger: Option<TriggerV2>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct LimitV2 {
    tif: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TriggerV2 {
    #[serde(rename = "isMarket")]
    is_market: bool,
    #[serde(rename = "triggerPx")]
    trigger_px: String,
    tpsl: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuilderV2 {
    b: String,
    f: u64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModifyV2 {
    oid: Value,
    order: OrderV2,
}
payload!(ModifyPayload { oid: Value, order: OrderV2, #[serde(rename="a")] always_place: Option<bool>, });
payload!(BatchModifyPayload { pub modifies: Vec<ModifyV2>, #[serde(rename="a")] always_place: Option<bool>, });
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TwapSpecV2 {
    a: u32,
    b: bool,
    s: String,
    r: bool,
    m: u64,
    t: bool,
}
payload!(TwapOrderPayload { twap: TwapSpecV2 });

// 这些 action 当前只返回 default；仍以独立强类型 payload 固定其 wire 契约。
payload!(SendAssetPayload {
    destination: String,
    #[serde(rename = "sourceDex")]
    source_dex: String,
    #[serde(rename = "destinationDex")]
    destination_dex: String,
    token: String,
    amount: String,
    #[serde(rename = "fromSubAccount")]
    from_sub_account: String,
});
payload!(SendToEvmPayload {
    destination: String,
    data: String,
    value: String,
});
payload!(SpotSendPayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    destination: String,
    token: String,
    amount: String,
    time: u64,
});
payload!(TokenDelegatePayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    validator: String,
    wei: u64,
    #[serde(rename = "isUndelegate")]
    is_undelegate: bool,
    nonce: u64,
});
payload!(UsdClassTransferPayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    amount: String,
    #[serde(rename = "toPerp")]
    to_perp: bool,
    nonce: u64,
});
payload!(UsdSendPayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    destination: String,
    amount: String,
    time: u64,
});
payload!(UserDexPayload {
    user: String,
    dex: String,
    #[serde(rename = "enableDexAbstraction")]
    enable: bool,
});
payload!(UserSetAbstractionPayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    user: String,
    abstraction: String,
    nonce: u64,
});
payload!(VaultTransferPayload {
    #[serde(rename = "vaultAddress")]
    vault_address: String,
    #[serde(rename = "isDeposit")]
    is_deposit: bool,
    usd: Number,
});
payload!(WithdrawPayload {
    #[serde(rename = "hyperliquidChain")]
    chain: String,
    #[serde(rename = "signatureChainId")]
    chain_id: String,
    amount: String,
    time: u64,
    destination: String,
});
payload!(UserOutcomePayload { #[serde(rename="splitOutcome")] split: Option<Value>, #[serde(rename="mergeOutcome")] merge: Option<Value>, #[serde(rename="mergeQuestion")] merge_question: Option<Value>, #[serde(rename="negateOutcome")] negate: Option<Value>, });
