use actix_web::{HttpResponse, Scope, web};

use crate::common::parse::parse_json_request;
use crate::exchange::action_registry::ACTION_REGISTRY;
use crate::exchange::common::wire::ExchangeActionTypeProbe;
use crate::exchange::error::ExchangeHttpError;

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
    use crate::exchange::action_registry::{
        SUPPORTED_ACTION_TYPES, SUPPORTED_ACTION_TYPES_DISPLAY,
    };
    use crate::exchange::test_support::valid_order_request_value;

    async fn post_exchange_json(payload: Value) -> Value {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request =
            actix_test::TestRequest::post().uri("/exchange").set_json(payload).to_request();
        let response = actix_test::call_service(&app, request).await;
        actix_test::read_body_json(response).await
    }

    async fn post_exchange_status_and_json(payload: Value) -> (StatusCode, Value) {
        let app = actix_test::init_service(App::new().service(build_exchange_scope())).await;
        let request =
            actix_test::TestRequest::post().uri("/exchange").set_json(payload).to_request();
        let response = actix_test::call_service(&app, request).await;
        let status = response.status();
        let body = actix_test::read_body_json(response).await;
        (status, body)
    }

    #[test]
    fn registry_action_types_match_supported_action_types() {
        let registry_action_types: Vec<_> =
            ACTION_REGISTRY.iter().map(|registration| registration.action_type).collect();
        assert_eq!(registry_action_types, SUPPORTED_ACTION_TYPES);
    }

    #[test]
    fn supported_action_types_display_stays_stable() {
        assert_eq!(
            SUPPORTED_ACTION_TYPES_DISPLAY,
            "`agentEnableDexAbstraction`, `agentSendAsset`, `agentSetAbstraction`, `approveAgent`, `approveBuilderFee`, `authorizeAqav2Role`, `batchModify`, `cDeposit`, `cWithdraw`, `cancel`, `cancelByCloid`, `claimRewards`, `hip3LiquidatorTransfer`, `modify`, `noop`, `order`, `reserveRequestWeight`, `scheduleCancel`, `sendAsset`, `sendToEvmWithData`, `spotSend`, `tokenDelegate`, `topUpIsolatedOnlyMargin`, `twapCancel`, `twapOrder`, `updateIsolatedMargin`, `updateLeverage`, `usdClassTransfer`, `usdSend`, `userDexAbstraction`, `userOutcome`, `userSetAbstraction`, `validatorL1Stream`, `vaultTransfer`, `withdraw3`."
        );
    }

    #[actix_web::test]
    async fn order_action_is_dispatched_to_order_handler() {
        let (status, body) = post_exchange_status_and_json(valid_order_request_value()).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["status"], "ok");
        assert_eq!(body["response"]["type"], "order");
        assert_eq!(
            body["response"]["data"]["statuses"][0]["error"],
            "load_state failed: spot order v2 place state is not wired for default HTTP path"
        );
    }

    #[actix_web::test]
    async fn unsupported_action_returns_stable_error_shape() {
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
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
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["response"]["type"], "cancel");
        assert_eq!(
            body["response"]["data"]["statuses"][0]["error"],
            "load_state failed: spot order v2 cancel state is not wired for default HTTP path"
        );
    }

    #[actix_web::test]
    async fn noop_action_is_dispatched_to_default_handler() {
        let (status, body) = post_exchange_status_and_json(json!({
            "action": { "type": "noop" },
            "nonce": 1710000000000u64,
            "signature": {
                "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                "v": 27
            }
        }))
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({ "status": "ok", "response": { "type": "default" } }));
    }

    #[actix_web::test]
    async fn usd_send_action_is_dispatched_to_default_handler() {
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({ "status": "ok", "response": { "type": "default" } }));
    }

    #[actix_web::test]
    async fn schedule_cancel_action_is_dispatched_to_default_handler() {
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({ "status": "ok", "response": { "type": "default" } }));
    }

    #[actix_web::test]
    async fn cancel_by_cloid_action_is_dispatched_to_cancel_handler() {
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["response"]["type"], "cancel");
        assert_eq!(
            body["response"]["data"]["statuses"][0]["error"],
            "load_state failed: spot order v2 cancel state is not wired for default HTTP path"
        );
    }

    #[actix_web::test]
    async fn modify_action_is_dispatched_to_modify_handler() {
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["response"]["type"], "order");
        assert_eq!(body["response"]["data"]["statuses"][0]["resting"]["oid"], 77738309u64);
    }

    #[actix_web::test]
    async fn batch_modify_action_is_dispatched_to_batch_modify_handler() {
        let (status, body) = post_exchange_status_and_json(json!({
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
        .await;
        assert_eq!(status, StatusCode::OK);
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
        let body = post_exchange_json(json!({
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
        .await;
        assert_eq!(body["response"]["data"]["statuses"].as_array().map(Vec::len), Some(2));
    }
}
