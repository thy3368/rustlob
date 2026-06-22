use actix_web::HttpResponse;
use futures_util::future::{FutureExt, LocalBoxFuture};

use crate::exchange::actions;
use crate::exchange::common::runner::run_exchange_action_http;
use crate::exchange::error::ExchangeHttpError;

pub(crate) type ExchangeActionDispatch =
    for<'a> fn(&'a [u8]) -> LocalBoxFuture<'a, Result<HttpResponse, ExchangeHttpError>>;

pub(crate) struct ExchangeActionRegistration {
    pub(crate) action_type: &'static str,
    pub(crate) dispatch: ExchangeActionDispatch,
}

macro_rules! define_exchange_action_registry {
    (($first_action_type:literal, $first_action:path), $(($action_type:literal, $action:path)),+ $(,)?) => {
        #[cfg_attr(not(test), allow(dead_code))]
        pub(crate) const SUPPORTED_ACTION_TYPES: &[&str] = &[
            $first_action_type,
            $($action_type),+
        ];

        pub(crate) const SUPPORTED_ACTION_TYPES_DISPLAY: &str = concat!(
            "`", $first_action_type, "`",
            $(", `", $action_type, "`"),+,
            "."
        );

        pub(crate) static ACTION_REGISTRY: &[ExchangeActionRegistration] = &[
            ExchangeActionRegistration {
                action_type: $first_action_type,
                dispatch: |body| run_exchange_action_http::<$first_action>(body).boxed_local(),
            },
            $(
                ExchangeActionRegistration {
                    action_type: $action_type,
                    dispatch: |body| run_exchange_action_http::<$action>(body).boxed_local(),
                }
            ),+
        ];
    };
}

define_exchange_action_registry!(
    (
        "agentEnableDexAbstraction",
        actions::agent_enable_dex_abstraction::AgentEnableDexAbstractionAction
    ),
    ("agentSendAsset", actions::agent_send_asset::AgentSendAssetAction),
    ("agentSetAbstraction", actions::agent_set_abstraction::AgentSetAbstractionAction),
    ("approveAgent", actions::approve_agent::ApproveAgentAction),
    ("approveBuilderFee", actions::approve_builder_fee::ApproveBuilderFeeAction),
    ("authorizeAqav2Role", actions::authorize_aqav2_role::AuthorizeAqav2RoleAction),
    ("batchModify", actions::batch_modify::BatchModifyAction),
    ("cDeposit", actions::c_deposit::CDepositAction),
    ("cWithdraw", actions::c_withdraw::CWithdrawAction),
    ("cancel", actions::cancel::CancelAction),
    ("cancelByCloid", actions::cancel_by_cloid::CancelByCloidAction),
    ("claimRewards", actions::claim_rewards::ClaimRewardsAction),
    ("hip3LiquidatorTransfer", actions::hip3_liquidator_transfer::Hip3LiquidatorTransferAction),
    ("modify", actions::modify::ModifyAction),
    ("noop", actions::noop::NoopAction),
    ("order", actions::order::OrderAction),
    ("reserveRequestWeight", actions::reserve_request_weight::ReserveRequestWeightAction),
    ("scheduleCancel", actions::schedule_cancel::ScheduleCancelAction),
    ("sendAsset", actions::send_asset::SendAssetAction),
    ("sendToEvmWithData", actions::send_to_evm_with_data::SendToEvmWithDataAction),
    ("spotSend", actions::spot_send::SpotSendAction),
    ("tokenDelegate", actions::token_delegate::TokenDelegateAction),
    (
        "topUpIsolatedOnlyMargin",
        actions::top_up_isolated_only_margin::TopUpIsolatedOnlyMarginAction
    ),
    ("twapCancel", actions::twap_cancel::TwapCancelAction),
    ("twapOrder", actions::twap_order::TwapOrderAction),
    ("updateIsolatedMargin", actions::update_isolated_margin::UpdateIsolatedMarginAction),
    ("updateLeverage", actions::update_leverage::UpdateLeverageAction),
    ("usdClassTransfer", actions::usd_class_transfer::UsdClassTransferAction),
    ("usdSend", actions::usd_send::UsdSendAction),
    ("userDexAbstraction", actions::user_dex_abstraction::UserDexAbstractionAction),
    ("userOutcome", actions::user_outcome::UserOutcomeAction),
    ("userSetAbstraction", actions::user_set_abstraction::UserSetAbstractionAction),
    ("validatorL1Stream", actions::validator_l1_stream::ValidatorL1StreamAction),
    ("vaultTransfer", actions::vault_transfer::VaultTransferAction),
    ("withdraw3", actions::withdraw3::Withdraw3Action),
);
