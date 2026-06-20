pub mod agent_enable_dex_abstraction;
pub mod agent_send_asset;
pub mod agent_set_abstraction;
pub mod approve_agent;
pub mod approve_builder_fee;
pub mod authorize_aqav2_role;
pub mod batch_modify;
pub mod c_deposit;
pub mod c_withdraw;
pub mod cancel;
pub mod cancel_by_cloid;
pub mod claim_rewards;
pub mod hip3_liquidator_transfer;
pub mod modify;
pub mod noop;
pub mod order;
pub mod reserve_request_weight;
pub mod schedule_cancel;
pub mod send_asset;
pub mod send_to_evm_with_data;
pub mod spot_send;
pub mod token_delegate;
pub mod top_up_isolated_only_margin;
pub mod twap_cancel;
pub mod twap_order;
pub mod update_isolated_margin;
pub mod update_leverage;
pub mod usd_class_transfer;
pub mod usd_send;
pub mod user_dex_abstraction;
pub mod user_outcome;
pub mod user_set_abstraction;
pub mod validator_l1_stream;
pub mod vault_transfer;
pub mod withdraw3;

use serde::{Serialize, Serializer};
use serde_json::Value as JsonValue;

use crate::exchange::common::wire::{ExchangeRequestEnvelopeWire, JsonObjectWire};
use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, Default, Clone, Copy)]
pub struct ExchangeActionDeps;

pub const SUPPORTED_ACTION_TYPES: &[&str] = &[
    "agentEnableDexAbstraction",
    "agentSendAsset",
    "agentSetAbstraction",
    "approveAgent",
    "approveBuilderFee",
    "authorizeAqav2Role",
    "batchModify",
    "cDeposit",
    "cWithdraw",
    "cancel",
    "cancelByCloid",
    "claimRewards",
    "hip3LiquidatorTransfer",
    "modify",
    "noop",
    "order",
    "reserveRequestWeight",
    "scheduleCancel",
    "sendAsset",
    "sendToEvmWithData",
    "spotSend",
    "tokenDelegate",
    "topUpIsolatedOnlyMargin",
    "twapCancel",
    "twapOrder",
    "updateIsolatedMargin",
    "updateLeverage",
    "usdClassTransfer",
    "usdSend",
    "userDexAbstraction",
    "userOutcome",
    "userSetAbstraction",
    "validatorL1Stream",
    "vaultTransfer",
    "withdraw3",
];

pub type ExchangeActionRequestWire = ExchangeRequestEnvelopeWire<ExchangeActionWire>;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeOrderWire {
    pub a: u32,
    pub b: bool,
    pub p: String,
    pub s: String,
    pub r: bool,
    pub t: ExchangeOrderTypeWire,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeOrderTypeWire {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ExchangeLimitOrderTypeWire>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<ExchangeTriggerOrderTypeWire>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeLimitOrderTypeWire {
    pub tif: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeTriggerOrderTypeWire {
    #[serde(rename = "isMarket")]
    pub is_market: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    pub tpsl: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeBuilderWire {
    pub b: String,
    pub f: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeCancelWire {
    pub a: u32,
    pub o: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeCancelByCloidWire {
    pub asset: u32,
    pub cloid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeModifyWire {
    pub oid: JsonValue,
    pub order: ExchangeOrderWire,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeTwapSpecWire {
    pub a: u32,
    pub b: bool,
    pub s: String,
    pub r: bool,
    pub m: u64,
    pub t: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ExchangeActionWire {
    #[serde(rename = "agentEnableDexAbstraction")]
    AgentEnableDexAbstraction {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "agentSendAsset")]
    AgentSendAsset {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "agentSetAbstraction")]
    AgentSetAbstraction {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "approveAgent")]
    ApproveAgent {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "approveBuilderFee")]
    ApproveBuilderFee {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "authorizeAqav2Role")]
    AuthorizeAqav2Role {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "batchModify")]
    BatchModify {
        modifies: Vec<ExchangeModifyWire>,
        #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
        always_place: Option<bool>,
    },
    #[serde(rename = "cDeposit")]
    CDeposit {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "cWithdraw")]
    CWithdraw {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "cancel")]
    Cancel {
        cancels: Vec<ExchangeCancelWire>,
        #[serde(skip_serializing_if = "Option::is_none")]
        f: Option<bool>,
    },
    #[serde(rename = "cancelByCloid")]
    CancelByCloid {
        cancels: Vec<ExchangeCancelByCloidWire>,
        #[serde(skip_serializing_if = "Option::is_none")]
        f: Option<bool>,
    },
    #[serde(rename = "claimRewards")]
    ClaimRewards {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "hip3LiquidatorTransfer")]
    Hip3LiquidatorTransfer {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "modify")]
    Modify {
        oid: JsonValue,
        order: ExchangeOrderWire,
        #[serde(rename = "a", skip_serializing_if = "Option::is_none")]
        always_place: Option<bool>,
    },
    #[serde(rename = "noop")]
    Noop,
    #[serde(rename = "order")]
    Order {
        orders: Vec<ExchangeOrderWire>,
        grouping: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        builder: Option<ExchangeBuilderWire>,
    },
    #[serde(rename = "reserveRequestWeight")]
    ReserveRequestWeight {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "scheduleCancel")]
    ScheduleCancel {
        #[serde(skip_serializing_if = "Option::is_none")]
        time: Option<u64>,
    },
    #[serde(rename = "sendAsset")]
    SendAsset {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "sendToEvmWithData")]
    SendToEvmWithData {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "spotSend")]
    SpotSend {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "tokenDelegate")]
    TokenDelegate {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "topUpIsolatedOnlyMargin")]
    TopUpIsolatedOnlyMargin {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "twapCancel")]
    TwapCancel { a: u32, t: u64 },
    #[serde(rename = "twapOrder")]
    TwapOrder { twap: ExchangeTwapSpecWire },
    #[serde(rename = "updateIsolatedMargin")]
    UpdateIsolatedMargin {
        asset: u32,
        #[serde(rename = "isBuy")]
        is_buy: bool,
        ntli: i64,
    },
    #[serde(rename = "updateLeverage")]
    UpdateLeverage {
        asset: u32,
        #[serde(rename = "isCross")]
        is_cross: bool,
        leverage: u64,
    },
    #[serde(rename = "usdClassTransfer")]
    UsdClassTransfer {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "usdSend")]
    UsdSend {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "userDexAbstraction")]
    UserDexAbstraction {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "userOutcome")]
    UserOutcome {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "userSetAbstraction")]
    UserSetAbstraction {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "validatorL1Stream")]
    ValidatorL1Stream {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "vaultTransfer")]
    VaultTransfer {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
    #[serde(rename = "withdraw3")]
    Withdraw3 {
        #[serde(flatten)]
        payload: JsonObjectWire,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn typed_trading_actions_serialize_to_expected_shapes() {
        let order = serde_json::to_value(ExchangeActionWire::Order {
            orders: vec![ExchangeOrderWire {
                a: 10000,
                b: true,
                p: "1891.4".to_string(),
                s: "0.02".to_string(),
                r: false,
                t: ExchangeOrderTypeWire {
                    limit: Some(ExchangeLimitOrderTypeWire { tif: "Gtc".to_string() }),
                    trigger: None,
                },
                c: None,
            }],
            grouping: "na".to_string(),
            builder: None,
        })
        .expect("order action serializes");
        assert_eq!(
            order,
            json!({
                "type": "order",
                "orders": [{
                    "a": 10000,
                    "b": true,
                    "p": "1891.4",
                    "s": "0.02",
                    "r": false,
                    "t": { "limit": { "tif": "Gtc" } }
                }],
                "grouping": "na"
            })
        );

        let modify = serde_json::to_value(ExchangeActionWire::Modify {
            oid: json!(77738308u64),
            order: ExchangeOrderWire {
                a: 10000,
                b: true,
                p: "1891.4".to_string(),
                s: "0.02".to_string(),
                r: false,
                t: ExchangeOrderTypeWire {
                    limit: Some(ExchangeLimitOrderTypeWire { tif: "Gtc".to_string() }),
                    trigger: None,
                },
                c: Some("0x1234567890abcdef1234567890abcdef".to_string()),
            },
            always_place: Some(true),
        })
        .expect("modify action serializes");
        assert_eq!(
            modify,
            json!({
                "type": "modify",
                "oid": 77738308u64,
                "order": {
                    "a": 10000,
                    "b": true,
                    "p": "1891.4",
                    "s": "0.02",
                    "r": false,
                    "t": { "limit": { "tif": "Gtc" } },
                    "c": "0x1234567890abcdef1234567890abcdef"
                },
                "a": true
            })
        );

        let twap = serde_json::to_value(ExchangeActionWire::TwapOrder {
            twap: ExchangeTwapSpecWire {
                a: 7,
                b: true,
                s: "1.25".to_string(),
                r: false,
                m: 15,
                t: true,
            },
        })
        .expect("twap action serializes");
        assert_eq!(
            twap,
            json!({
                "type": "twapOrder",
                "twap": {
                    "a": 7,
                    "b": true,
                    "s": "1.25",
                    "r": false,
                    "m": 15,
                    "t": true
                }
            })
        );
    }
}

pub enum ExchangeActionReply {
    AgentEnableDexAbstraction(
        agent_enable_dex_abstraction::reply::AgentEnableDexAbstractionResponseWire,
    ),
    AgentSendAsset(agent_send_asset::reply::AgentSendAssetResponseWire),
    AgentSetAbstraction(agent_set_abstraction::reply::AgentSetAbstractionResponseWire),
    ApproveAgent(approve_agent::reply::ApproveAgentResponseWire),
    ApproveBuilderFee(approve_builder_fee::reply::ApproveBuilderFeeResponseWire),
    AuthorizeAqav2Role(authorize_aqav2_role::reply::AuthorizeAqav2RoleResponseWire),
    BatchModify(batch_modify::reply::BatchModifyResponseWire),
    Cancel(cancel::reply::CancelResponseWire),
    CancelByCloid(cancel_by_cloid::reply::CancelByCloidResponseWire),
    CDeposit(c_deposit::reply::CDepositResponseWire),
    CWithdraw(c_withdraw::reply::CWithdrawResponseWire),
    ClaimRewards(claim_rewards::reply::ClaimRewardsResponseWire),
    Hip3LiquidatorTransfer(hip3_liquidator_transfer::reply::Hip3LiquidatorTransferResponseWire),
    Modify(modify::reply::ModifyResponseWire),
    Noop(noop::reply::NoopResponseWire),
    Order(order::reply::OrderResponseWire),
    ReserveRequestWeight(reserve_request_weight::reply::ReserveRequestWeightResponseWire),
    ScheduleCancel(schedule_cancel::reply::ScheduleCancelResponseWire),
    SendAsset(send_asset::reply::SendAssetResponseWire),
    SendToEvmWithData(send_to_evm_with_data::reply::SendToEvmWithDataResponseWire),
    SpotSend(spot_send::reply::SpotSendResponseWire),
    TopUpIsolatedOnlyMargin(
        top_up_isolated_only_margin::reply::TopUpIsolatedOnlyMarginResponseWire,
    ),
    TwapCancel(twap_cancel::reply::TwapCancelResponseWire),
    TwapOrder(twap_order::reply::TwapOrderResponseWire),
    TokenDelegate(token_delegate::reply::TokenDelegateResponseWire),
    UpdateIsolatedMargin(update_isolated_margin::reply::UpdateIsolatedMarginResponseWire),
    UpdateLeverage(update_leverage::reply::UpdateLeverageResponseWire),
    UsdClassTransfer(usd_class_transfer::reply::UsdClassTransferResponseWire),
    UsdSend(usd_send::reply::UsdSendResponseWire),
    UserOutcome(user_outcome::reply::UserOutcomeResponseWire),
    UserDexAbstraction(user_dex_abstraction::reply::UserDexAbstractionResponseWire),
    UserSetAbstraction(user_set_abstraction::reply::UserSetAbstractionResponseWire),
    ValidatorL1Stream(validator_l1_stream::reply::ValidatorL1StreamResponseWire),
    VaultTransfer(vault_transfer::reply::VaultTransferResponseWire),
    Withdraw3(withdraw3::reply::Withdraw3ResponseWire),
}

impl Serialize for ExchangeActionReply {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AgentEnableDexAbstraction(response) => response.serialize(serializer),
            Self::AgentSendAsset(response) => response.serialize(serializer),
            Self::AgentSetAbstraction(response) => response.serialize(serializer),
            Self::ApproveAgent(response) => response.serialize(serializer),
            Self::ApproveBuilderFee(response) => response.serialize(serializer),
            Self::AuthorizeAqav2Role(response) => response.serialize(serializer),
            Self::BatchModify(response) => response.serialize(serializer),
            Self::Cancel(response) => response.serialize(serializer),
            Self::CancelByCloid(response) => response.serialize(serializer),
            Self::CDeposit(response) => response.serialize(serializer),
            Self::CWithdraw(response) => response.serialize(serializer),
            Self::ClaimRewards(response) => response.serialize(serializer),
            Self::Hip3LiquidatorTransfer(response) => response.serialize(serializer),
            Self::Modify(response) => response.serialize(serializer),
            Self::Noop(response) => response.serialize(serializer),
            Self::Order(response) => response.serialize(serializer),
            Self::ReserveRequestWeight(response) => response.serialize(serializer),
            Self::ScheduleCancel(response) => response.serialize(serializer),
            Self::SendAsset(response) => response.serialize(serializer),
            Self::SendToEvmWithData(response) => response.serialize(serializer),
            Self::SpotSend(response) => response.serialize(serializer),
            Self::TopUpIsolatedOnlyMargin(response) => response.serialize(serializer),
            Self::TwapCancel(response) => response.serialize(serializer),
            Self::TwapOrder(response) => response.serialize(serializer),
            Self::TokenDelegate(response) => response.serialize(serializer),
            Self::UpdateIsolatedMargin(response) => response.serialize(serializer),
            Self::UpdateLeverage(response) => response.serialize(serializer),
            Self::UsdClassTransfer(response) => response.serialize(serializer),
            Self::UsdSend(response) => response.serialize(serializer),
            Self::UserOutcome(response) => response.serialize(serializer),
            Self::UserDexAbstraction(response) => response.serialize(serializer),
            Self::UserSetAbstraction(response) => response.serialize(serializer),
            Self::ValidatorL1Stream(response) => response.serialize(serializer),
            Self::VaultTransfer(response) => response.serialize(serializer),
            Self::Withdraw3(response) => response.serialize(serializer),
        }
    }
}

pub async fn dispatch_exchange_action(
    action_type: &str,
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<ExchangeActionReply, ExchangeHttpError> {
    match action_type {
        "agentEnableDexAbstraction" => agent_enable_dex_abstraction::handle(body, deps)
            .await
            .map(ExchangeActionReply::AgentEnableDexAbstraction),
        "agentSendAsset" => {
            agent_send_asset::handle(body, deps).await.map(ExchangeActionReply::AgentSendAsset)
        }
        "agentSetAbstraction" => agent_set_abstraction::handle(body, deps)
            .await
            .map(ExchangeActionReply::AgentSetAbstraction),
        "approveAgent" => {
            approve_agent::handle(body, deps).await.map(ExchangeActionReply::ApproveAgent)
        }
        "approveBuilderFee" => approve_builder_fee::handle(body, deps)
            .await
            .map(ExchangeActionReply::ApproveBuilderFee),
        "authorizeAqav2Role" => authorize_aqav2_role::handle(body, deps)
            .await
            .map(ExchangeActionReply::AuthorizeAqav2Role),
        "batchModify" => {
            batch_modify::handle(body, deps).await.map(ExchangeActionReply::BatchModify)
        }
        "cancel" => cancel::handle(body, deps).await.map(ExchangeActionReply::Cancel),
        "cancelByCloid" => {
            cancel_by_cloid::handle(body, deps).await.map(ExchangeActionReply::CancelByCloid)
        }
        "cDeposit" => c_deposit::handle(body, deps).await.map(ExchangeActionReply::CDeposit),
        "cWithdraw" => c_withdraw::handle(body, deps).await.map(ExchangeActionReply::CWithdraw),
        "claimRewards" => {
            claim_rewards::handle(body, deps).await.map(ExchangeActionReply::ClaimRewards)
        }
        "hip3LiquidatorTransfer" => hip3_liquidator_transfer::handle(body, deps)
            .await
            .map(ExchangeActionReply::Hip3LiquidatorTransfer),
        "modify" => modify::handle(body, deps).await.map(ExchangeActionReply::Modify),
        "noop" => noop::handle(body, deps).await.map(ExchangeActionReply::Noop),
        "order" => order::handle(body, deps).await.map(ExchangeActionReply::Order),
        "reserveRequestWeight" => reserve_request_weight::handle(body, deps)
            .await
            .map(ExchangeActionReply::ReserveRequestWeight),
        "scheduleCancel" => {
            schedule_cancel::handle(body, deps).await.map(ExchangeActionReply::ScheduleCancel)
        }
        "sendAsset" => send_asset::handle(body, deps).await.map(ExchangeActionReply::SendAsset),
        "sendToEvmWithData" => send_to_evm_with_data::handle(body, deps)
            .await
            .map(ExchangeActionReply::SendToEvmWithData),
        "spotSend" => spot_send::handle(body, deps).await.map(ExchangeActionReply::SpotSend),
        "topUpIsolatedOnlyMargin" => top_up_isolated_only_margin::handle(body, deps)
            .await
            .map(ExchangeActionReply::TopUpIsolatedOnlyMargin),
        "twapCancel" => twap_cancel::handle(body, deps).await.map(ExchangeActionReply::TwapCancel),
        "twapOrder" => twap_order::handle(body, deps).await.map(ExchangeActionReply::TwapOrder),
        "tokenDelegate" => {
            token_delegate::handle(body, deps).await.map(ExchangeActionReply::TokenDelegate)
        }
        "updateIsolatedMargin" => update_isolated_margin::handle(body, deps)
            .await
            .map(ExchangeActionReply::UpdateIsolatedMargin),
        "updateLeverage" => {
            update_leverage::handle(body, deps).await.map(ExchangeActionReply::UpdateLeverage)
        }
        "usdClassTransfer" => {
            usd_class_transfer::handle(body, deps).await.map(ExchangeActionReply::UsdClassTransfer)
        }
        "usdSend" => usd_send::handle(body, deps).await.map(ExchangeActionReply::UsdSend),
        "userOutcome" => {
            user_outcome::handle(body, deps).await.map(ExchangeActionReply::UserOutcome)
        }
        "userDexAbstraction" => user_dex_abstraction::handle(body, deps)
            .await
            .map(ExchangeActionReply::UserDexAbstraction),
        "userSetAbstraction" => user_set_abstraction::handle(body, deps)
            .await
            .map(ExchangeActionReply::UserSetAbstraction),
        "validatorL1Stream" => validator_l1_stream::handle(body, deps)
            .await
            .map(ExchangeActionReply::ValidatorL1Stream),
        "vaultTransfer" => {
            vault_transfer::handle(body, deps).await.map(ExchangeActionReply::VaultTransfer)
        }
        "withdraw3" => withdraw3::handle(body, deps).await.map(ExchangeActionReply::Withdraw3),
        other => Err(ExchangeHttpError::UnsupportedActionType(other.to_string())),
    }
}
