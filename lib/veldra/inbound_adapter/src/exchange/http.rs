use actix_web::{HttpResponse, Scope, web};

use crate::exchange::actions::{ExchangeActionDeps, ExchangeActionReply, dispatch_exchange_action};
use crate::exchange::common::parse::parse_json_request;
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
    let deps = ExchangeActionDeps::default();
    let reply = dispatch_exchange_action(&probe.action.type_, body, &deps).await?;
    Ok(action_reply_to_http(reply))
}

fn parse_action_type_probe(body: &[u8]) -> Result<ExchangeActionTypeProbe, ExchangeHttpError> {
    parse_json_request(body)
}

fn action_reply_to_http(reply: ExchangeActionReply) -> HttpResponse {
    HttpResponse::Ok().json(reply)
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test as actix_test};
    use serde_json::{Value, json};

    use super::*;
    use crate::exchange::test_support::valid_order_request_value;

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
