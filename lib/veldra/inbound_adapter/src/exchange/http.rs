use actix_web::{HttpResponse, Scope, web};
use futures_util::future::{FutureExt, LocalBoxFuture};

use crate::exchange::actions::{self, SUPPORTED_ACTION_TYPES};
use crate::exchange::common::parse::parse_json_request;
use crate::exchange::common::runner::run_exchange_action_http;
use crate::exchange::common::wire::ExchangeActionTypeProbe;
use crate::exchange::error::ExchangeHttpError;

type ExchangeActionDispatch =
    for<'a> fn(&'a [u8]) -> LocalBoxFuture<'a, Result<HttpResponse, ExchangeHttpError>>;

struct ExchangeActionRegistration {
    action_type: &'static str,
    dispatch: ExchangeActionDispatch,
}

macro_rules! action_registration {
    ($index:expr, $action:path) => {
        ExchangeActionRegistration {
            action_type: SUPPORTED_ACTION_TYPES[$index],
            dispatch: |body| run_exchange_action_http::<$action>(body).boxed_local(),
        }
    };
}

static ACTION_REGISTRY: &[ExchangeActionRegistration] = &[
    action_registration!(0, actions::agent_enable_dex_abstraction::AgentEnableDexAbstractionAction),
    action_registration!(1, actions::agent_send_asset::AgentSendAssetAction),
    action_registration!(2, actions::agent_set_abstraction::AgentSetAbstractionAction),
    action_registration!(3, actions::approve_agent::ApproveAgentAction),
    action_registration!(4, actions::approve_builder_fee::ApproveBuilderFeeAction),
    action_registration!(5, actions::authorize_aqav2_role::AuthorizeAqav2RoleAction),
    action_registration!(6, actions::batch_modify::BatchModifyAction),
    action_registration!(7, actions::c_deposit::CDepositAction),
    action_registration!(8, actions::c_withdraw::CWithdrawAction),
    action_registration!(9, actions::cancel::CancelAction),
    action_registration!(10, actions::cancel_by_cloid::CancelByCloidAction),
    action_registration!(11, actions::claim_rewards::ClaimRewardsAction),
    action_registration!(12, actions::hip3_liquidator_transfer::Hip3LiquidatorTransferAction),
    action_registration!(13, actions::modify::ModifyAction),
    action_registration!(14, actions::noop::NoopAction),
    action_registration!(15, actions::order::OrderAction),
    action_registration!(16, actions::reserve_request_weight::ReserveRequestWeightAction),
    action_registration!(17, actions::schedule_cancel::ScheduleCancelAction),
    action_registration!(18, actions::send_asset::SendAssetAction),
    action_registration!(19, actions::send_to_evm_with_data::SendToEvmWithDataAction),
    action_registration!(20, actions::spot_send::SpotSendAction),
    action_registration!(21, actions::token_delegate::TokenDelegateAction),
    action_registration!(22, actions::top_up_isolated_only_margin::TopUpIsolatedOnlyMarginAction),
    action_registration!(23, actions::twap_cancel::TwapCancelAction),
    action_registration!(24, actions::twap_order::TwapOrderAction),
    action_registration!(25, actions::update_isolated_margin::UpdateIsolatedMarginAction),
    action_registration!(26, actions::update_leverage::UpdateLeverageAction),
    action_registration!(27, actions::usd_class_transfer::UsdClassTransferAction),
    action_registration!(28, actions::usd_send::UsdSendAction),
    action_registration!(29, actions::user_dex_abstraction::UserDexAbstractionAction),
    action_registration!(30, actions::user_outcome::UserOutcomeAction),
    action_registration!(31, actions::user_set_abstraction::UserSetAbstractionAction),
    action_registration!(32, actions::validator_l1_stream::ValidatorL1StreamAction),
    action_registration!(33, actions::vault_transfer::VaultTransferAction),
    action_registration!(34, actions::withdraw3::Withdraw3Action),
];

pub fn build_exchange_scope() -> Scope {
    web::scope("").route("/exchange", web::post().to(post_exchange))
}

async fn post_exchange(body: web::Bytes) -> Result<HttpResponse, ExchangeHttpError> {
    dispatch_exchange_action_from_body(&body).await
}

async fn dispatch_exchange_action_from_body(
    body: &[u8],
) -> Result<HttpResponse, ExchangeHttpError> {
    let probe = parse_action_type_probe(body)?;
    let registration = ACTION_REGISTRY
        .iter()
        .find(|registration| registration.action_type == probe.action.type_)
        .ok_or_else(|| ExchangeHttpError::UnsupportedActionType(probe.action.type_.clone()))?;
    (registration.dispatch)(body).await
}

fn parse_action_type_probe(body: &[u8]) -> Result<ExchangeActionTypeProbe, ExchangeHttpError> {
    parse_json_request(body)
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test as actix_test};
    use serde_json::{Value, json};

    use super::*;
    use crate::exchange::test_support::valid_order_request_value;

    #[test]
    fn registry_action_types_match_supported_action_types() {
        let registry_action_types: Vec<_> =
            ACTION_REGISTRY.iter().map(|registration| registration.action_type).collect();
        assert_eq!(registry_action_types, SUPPORTED_ACTION_TYPES);
    }

    #[actix_web::test]
    async fn order_action_is_dispatched_to_order_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(valid_order_request_value())
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["status"], "ok");
        assert_eq!(body["response"]["type"], "order");
        assert_eq!(body["response"]["data"]["statuses"][0]["resting"]["oid"], 77738308u64);
    }

    #[actix_web::test]
    async fn unsupported_action_returns_stable_error_shape() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "doesNotExist"
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(
            body,
            json!({
                "status": "err",
                "error": "Unsupported action.type `doesNotExist`. Supported actions: `agentEnableDexAbstraction`, `agentSendAsset`, `agentSetAbstraction`, `approveAgent`, `approveBuilderFee`, `authorizeAqav2Role`, `batchModify`, `cDeposit`, `cWithdraw`, `cancel`, `cancelByCloid`, `claimRewards`, `hip3LiquidatorTransfer`, `modify`, `noop`, `order`, `reserveRequestWeight`, `scheduleCancel`, `sendAsset`, `sendToEvmWithData`, `spotSend`, `tokenDelegate`, `topUpIsolatedOnlyMargin`, `twapCancel`, `twapOrder`, `updateIsolatedMargin`, `updateLeverage`, `usdClassTransfer`, `usdSend`, `userDexAbstraction`, `userOutcome`, `userSetAbstraction`, `validatorL1Stream`, `vaultTransfer`, `withdraw3`."
            })
        );
    }

    #[actix_web::test]
    async fn cancel_action_is_dispatched_to_cancel_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "cancel",
                    "cancels": [{ "a": 10000, "o": 77738308 }]
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["response"]["type"], "cancel");
        assert_eq!(body["response"]["data"]["statuses"][0], "success");
    }

    #[actix_web::test]
    async fn noop_action_is_dispatched_to_default_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": { "type": "noop" },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body, json!({ "status": "ok", "response": { "type": "default" } }));
    }

    #[actix_web::test]
    async fn usd_send_action_is_dispatched_to_default_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "usdSend",
                    "hyperliquidChain": "Mainnet",
                    "signatureChainId": "0xa4b1",
                    "destination": "0x5555555555555555555555555555555555555555",
                    "amount": "1",
                    "time": 1710000000000u64
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body, json!({ "status": "ok", "response": { "type": "default" } }));
    }

    #[actix_web::test]
    async fn schedule_cancel_action_is_dispatched_to_default_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "scheduleCancel",
                    "time": 1710000006000u64
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body, json!({ "status": "ok", "response": { "type": "default" } }));
    }

    #[actix_web::test]
    async fn cancel_by_cloid_action_is_dispatched_to_cancel_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "cancelByCloid",
                    "cancels": [{
                        "asset": 10000,
                        "cloid": "0x1234567890abcdef1234567890abcdef"
                    }]
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["response"]["type"], "cancel");
        assert_eq!(body["response"]["data"]["statuses"][0], "success");
    }

    #[actix_web::test]
    async fn modify_action_is_dispatched_to_modify_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "modify",
                    "oid": 77738308u64,
                    "order": {
                        "a": 10000,
                        "b": true,
                        "p": "1891.4",
                        "s": "0.02",
                        "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["response"]["type"], "order");
        assert_eq!(body["response"]["data"]["statuses"][0]["resting"]["oid"], 77738309u64);
    }

    #[actix_web::test]
    async fn batch_modify_action_is_dispatched_to_batch_modify_handler() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "batchModify",
                    "modifies": [
                        {
                            "oid": 77738308u64,
                            "order": {
                                "a": 10000,
                                "b": true,
                                "p": "1891.4",
                                "s": "0.02",
                                "r": false,
                                "t": { "limit": { "tif": "Gtc" } }
                            }
                        },
                        {
                            "oid": "0x1234567890abcdef1234567890abcdef",
                            "order": {
                                "a": 10001,
                                "b": false,
                                "p": "1890.0",
                                "s": "0.04",
                                "r": false,
                                "t": { "limit": { "tif": "Ioc" } }
                            }
                        }
                    ]
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["response"]["type"], "order");
        assert_eq!(body["response"]["data"]["statuses"][0]["resting"]["oid"], 77738400u64);
        assert_eq!(body["response"]["data"]["statuses"][1]["resting"]["oid"], 77738401u64);
    }

    #[actix_web::test]
    async fn malformed_json_returns_stable_error_shape() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_payload(r#"{"action":{"type":"order""#)
            .insert_header(("content-type", "application/json"))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(
            body,
            json!({
                "status": "err",
                "error": "Malformed JSON body."
            })
        );
    }

    #[actix_web::test]
    async fn statuses_count_matches_orders_count() {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "order",
                    "orders": [
                        {
                            "a": 10000,
                            "b": true,
                            "p": "1891.4",
                            "s": "0.02",
                            "r": false,
                            "t": { "limit": { "tif": "Gtc" } }
                        },
                        {
                            "a": 10001,
                            "b": false,
                            "p": "1890.0",
                            "s": "0.04",
                            "r": false,
                            "t": { "limit": { "tif": "Ioc" } }
                        }
                    ],
                    "grouping": "na"
                },
                "nonce": 1710000000000u64,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["response"]["data"]["statuses"].as_array().map(Vec::len), Some(2));
    }
}
