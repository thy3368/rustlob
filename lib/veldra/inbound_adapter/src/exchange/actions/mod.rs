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
