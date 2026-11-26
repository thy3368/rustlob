use std::fmt::Debug;

use actix_web::{HttpResponse, Scope, web};

use super::{
    CancelStatusWire, ExchangeActionV2, ExchangeRequestV2, ExchangeResponseWire, ExchangeV2Error,
    OrderStatusWire, TwapCancelStatusWire, TwapOrderStatusWire,
};

pub fn build_exchange_v2_scope() -> Scope {
    web::scope("/exchange-v2").route("", web::post().to(post_exchange_v2))
}

async fn post_exchange_v2(body: web::Bytes) -> Result<HttpResponse, ExchangeV2Error> {
    let request: ExchangeRequestV2 =
        serde_json::from_slice(&body).map_err(ExchangeV2Error::json)?;
    request.validate_common()?;

    let response = match request.action {
        ExchangeActionV2::Unknown { type_ } => {
            return Err(ExchangeV2Error::UnsupportedActionType(type_));
        }
        ExchangeActionV2::AgentEnableDexAbstraction(action) => log_default(action),
        ExchangeActionV2::AgentSendAsset(action) => log_default(action),
        ExchangeActionV2::AgentSetAbstraction(action) => log_default(action),
        ExchangeActionV2::ApproveAgent(action) => log_default(action),
        ExchangeActionV2::ApproveBuilderFee(action) => log_default(action),
        ExchangeActionV2::AuthorizeAqav2Role(action) => log_default(action),
        ExchangeActionV2::BatchModify(action) => {
            println!("{action:?}");
            batch_modify_errors(
                action.modifies.len(),
                "load_state failed: spot order not found",
            )
        }
        ExchangeActionV2::CDeposit(action) => log_default(action),
        ExchangeActionV2::CWithdraw(action) => log_default(action),
        ExchangeActionV2::Cancel(action) => {
            println!("{action:?}");
            cancel_errors(action.cancels.len())
        }
        ExchangeActionV2::CancelByCloid(action) => {
            println!("{action:?}");
            cancel_by_cloid_errors(action.cancels.len())
        }
        ExchangeActionV2::ClaimRewards(action) => log_default(action),
        ExchangeActionV2::Hip3LiquidatorTransfer(action) => log_default(action),
        ExchangeActionV2::Modify(action) => {
            println!("{action:?}");
            modify_errors(1, "load_state failed: spot order not found")
        }
        ExchangeActionV2::Noop(action) => log_default(action),
        ExchangeActionV2::Order(action) => {
            println!("{action:?}");
            order_errors(
                action.orders.len(),
                "load_state failed: spot order book not found",
            )
        }
        ExchangeActionV2::ReserveRequestWeight(action) => log_default(action),
        ExchangeActionV2::ScheduleCancel(action) => log_default(action),
        ExchangeActionV2::SendAsset(action) => log_default(action),
        ExchangeActionV2::SendToEvmWithData(action) => log_default(action),
        ExchangeActionV2::SpotSend(action) => log_default(action),
        ExchangeActionV2::TokenDelegate(action) => log_default(action),
        ExchangeActionV2::TopUpIsolatedOnlyMargin(action) => log_default(action),
        ExchangeActionV2::TwapCancel(action) => {
            println!("{action:?}");
            ExchangeResponseWire::twap_cancel(TwapCancelStatusWire::Error {
                error: "TWAP was never placed, already canceled, or filled.".to_owned(),
            })
        }
        ExchangeActionV2::TwapOrder(action) => {
            println!("{action:?}");
            ExchangeResponseWire::twap_order(TwapOrderStatusWire::Error {
                error: "Invalid TWAP duration: 1 min(s)".to_owned(),
            })
        }
        ExchangeActionV2::UpdateIsolatedMargin(action) => log_default(action),
        ExchangeActionV2::UpdateLeverage(action) => log_default(action),
        ExchangeActionV2::UsdClassTransfer(action) => log_default(action),
        ExchangeActionV2::UsdSend(action) => log_default(action),
        ExchangeActionV2::UserDexAbstraction(action) => log_default(action),
        ExchangeActionV2::UserOutcome(action) => log_default(action),
        ExchangeActionV2::UserSetAbstraction(action) => log_default(action),
        ExchangeActionV2::ValidatorL1Stream(action) => log_default(action),
        ExchangeActionV2::VaultTransfer(action) => log_default(action),
        ExchangeActionV2::Withdraw3(action) => log_default(action),
    };
    Ok(HttpResponse::Ok().json(response))
}

fn log_default(action: impl Debug) -> ExchangeResponseWire {
    println!("{action:?}");
    ExchangeResponseWire::default()
}

fn order_errors(count: usize, error: &'static str) -> ExchangeResponseWire {
    ExchangeResponseWire::order(order_error_statuses(count, error))
}

fn modify_errors(count: usize, error: &'static str) -> ExchangeResponseWire {
    ExchangeResponseWire::order(order_error_statuses(count, error))
}

fn batch_modify_errors(count: usize, error: &'static str) -> ExchangeResponseWire {
    ExchangeResponseWire::order(order_error_statuses(count, error))
}

fn order_error_statuses(count: usize, error: &'static str) -> Vec<OrderStatusWire> {
    (0..count)
        .map(|_| OrderStatusWire::Error {
            error: error.to_owned(),
        })
        .collect()
}

fn cancel_errors(count: usize) -> ExchangeResponseWire {
    ExchangeResponseWire::cancel(
        (0..count)
            .map(|_| CancelStatusWire::Error {
                error: "load_state failed: spot order not found".to_owned(),
            })
            .collect(),
    )
}

fn cancel_by_cloid_errors(count: usize) -> ExchangeResponseWire {
    ExchangeResponseWire::cancel(
        (0..count)
            .map(|_| CancelStatusWire::Error {
                error: "load_state failed: spot order not found".to_owned(),
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test};
    use serde_json::{Value, json};

    use super::*;

    fn envelope(action: Value) -> Value {
        json!({
            "action": action,
            "nonce": 1710000000000_u64,
            "signature": {
                "r": format!("0x{}", "11".repeat(32)),
                "s": format!("0x{}", "22".repeat(32)),
                "v": 27
            }
        })
    }

    fn expected_response(action: &Value) -> Value {
        let type_ = action["type"].as_str().expect("fixture action type");
        let repeated_errors = |response_type: &str, count: usize, error: &str| {
            json!({
                "status": "ok",
                "response": {
                    "type": response_type,
                    "data": {"statuses": (0..count).map(|_| json!({"error": error})).collect::<Vec<_>>()}
                }
            })
        };

        match type_ {
            "order" => repeated_errors(
                "order",
                action["orders"].as_array().expect("orders array").len(),
                "load_state failed: spot order book not found",
            ),
            "cancel" | "cancelByCloid" => repeated_errors(
                "cancel",
                action["cancels"].as_array().expect("cancels array").len(),
                "load_state failed: spot order not found",
            ),
            "modify" => repeated_errors("order", 1, "load_state failed: spot order not found"),
            "batchModify" => repeated_errors(
                "order",
                action["modifies"].as_array().expect("modifies array").len(),
                "load_state failed: spot order not found",
            ),
            "twapCancel" => json!({
                "status": "ok",
                "response": {"type": "twapCancel", "data": {"status": {
                    "error": "TWAP was never placed, already canceled, or filled."
                }}}
            }),
            "twapOrder" => json!({
                "status": "ok",
                "response": {"type": "twapOrder", "data": {"status": {
                    "error": "Invalid TWAP duration: 1 min(s)"
                }}}
            }),
            _ => serde_json::to_value(ExchangeResponseWire::default())
                .expect("default response serializes"),
        }
    }

    fn order(oid: u64) -> Value {
        json!({
            "oid": oid,
            "order": {
                "a": 10000,
                "b": true,
                "p": "1",
                "s": "1",
                "r": false,
                "t": {"limit": {"tif": "Gtc"}, "trigger": null},
                "c": null
            }
        })
    }

    #[actix_web::test]
    async fn all_35_registered_actions_reach_a_v2_match_branch() {
        let fixtures = [
            json!({"type":"agentEnableDexAbstraction"}),
            json!({"type":"agentSendAsset","destination":"0x1111111111111111111111111111111111111111","sourceDex":"a","destinationDex":"b","token":"USDC","amount":"1","fromSubAccount":"","nonce":1710000000000_u64}),
            json!({"type":"agentSetAbstraction","abstraction":"u"}),
            json!({"type":"approveAgent","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","agentAddress":"0x1111111111111111111111111111111111111111","agentName":null,"nonce":1710000000000_u64}),
            json!({"type":"approveBuilderFee","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","maxFeeRate":"0.1%","builder":"0x1111111111111111111111111111111111111111","nonce":1710000000000_u64}),
            json!({"type":"authorizeAqav2Role","token":1,"role":"agent"}),
            json!({"type":"batchModify","modifies":[],"a":true}),
            json!({"type":"cDeposit","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","wei":1,"nonce":1710000000000_u64}),
            json!({"type":"cWithdraw","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","wei":1,"nonce":1710000000000_u64}),
            json!({"type":"cancel","cancels":[{"a":10000,"o":1}]}),
            json!({"type":"cancelByCloid","cancels":[{"asset":10000,"cloid":"0x1234567890abcdef1234567890abcdef"}]}),
            json!({"type":"claimRewards"}),
            json!({"type":"hip3LiquidatorTransfer","dexAddress":"0x1111111111111111111111111111111111111111","amount":"1"}),
            json!({"type":"modify","oid":1,"order":{"a":10000,"b":true,"p":"1","s":"1","r":false,"t":{"limit":{"tif":"Gtc"},"trigger":null},"c":null},"a":true}),
            json!({"type":"noop"}),
            json!({"type":"order","orders":[],"grouping":"na","builder":null}),
            json!({"type":"reserveRequestWeight","weight":1}),
            json!({"type":"scheduleCancel","time":null}),
            json!({"type":"sendAsset","destination":"0x1111111111111111111111111111111111111111","sourceDex":"a","destinationDex":"b","token":"USDC","amount":"1","fromSubAccount":""}),
            json!({"type":"sendToEvmWithData","destination":"0x1111111111111111111111111111111111111111","data":"0x","value":"0"}),
            json!({"type":"spotSend","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","destination":"0x1111111111111111111111111111111111111111","token":"USDC","amount":"1","time":1710000000000_u64}),
            json!({"type":"tokenDelegate","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","validator":"0x1111111111111111111111111111111111111111","wei":1,"isUndelegate":false,"nonce":1710000000000_u64}),
            json!({"type":"topUpIsolatedOnlyMargin","asset":1}),
            json!({"type":"twapCancel","a":1,"t":1}),
            json!({"type":"twapOrder","twap":{"a":1,"b":true,"s":"1","r":false,"m":5,"t":false}}),
            json!({"type":"updateIsolatedMargin","asset":1,"isBuy":true,"ntli":1}),
            json!({"type":"updateLeverage","asset":1,"isCross":true,"leverage":2}),
            json!({"type":"usdClassTransfer","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","amount":"1","toPerp":true,"nonce":1710000000000_u64}),
            json!({"type":"usdSend","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","destination":"0x1111111111111111111111111111111111111111","amount":"1","time":1710000000000_u64}),
            json!({"type":"userDexAbstraction","user":"0x1111111111111111111111111111111111111111","dex":"test","enableDexAbstraction":true}),
            json!({"type":"userOutcome","splitOutcome":null,"mergeOutcome":null,"mergeQuestion":null,"negateOutcome":null}),
            json!({"type":"userSetAbstraction","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","user":"0x1111111111111111111111111111111111111111","abstraction":"disabled","nonce":1710000000000_u64}),
            json!({"type":"validatorL1Stream","riskFreeRate":"0.04"}),
            json!({"type":"vaultTransfer","vaultAddress":"0x1111111111111111111111111111111111111111","isDeposit":true,"usd":1}),
            json!({"type":"withdraw3","hyperliquidChain":"Mainnet","signatureChainId":"0xa4b1","amount":"1","time":1710000000000_u64,"destination":"0x1111111111111111111111111111111111111111"}),
        ];
        assert_eq!(fixtures.len(), 35);
        for action in fixtures {
            let expected = expected_response(&action);
            let app = test::init_service(App::new().service(build_exchange_v2_scope())).await;
            let request = test::TestRequest::post()
                .uri("/exchange-v2")
                .set_json(envelope(action))
                .to_request();
            let response = test::call_service(&app, request).await;
            assert_eq!(response.status(), StatusCode::OK);
            let response: Value = test::read_body_json(response).await;
            assert_eq!(response, expected, "unexpected response for action fixture");
        }
    }

    #[actix_web::test]
    async fn repeated_actions_return_one_error_per_entry() {
        let actions = [
            json!({
                "type": "order",
                "orders": [order(1)["order"].clone(), order(2)["order"].clone()],
                "grouping": "na",
                "builder": null
            }),
            json!({"type": "cancel", "cancels": [{"a": 10000, "o": 1}, {"a": 10000, "o": 2}]}),
            json!({"type": "batchModify", "modifies": [order(1), order(2)], "a": true}),
        ];

        for action in actions {
            let expected = expected_response(&action);
            let app = test::init_service(App::new().service(build_exchange_v2_scope())).await;
            let request = test::TestRequest::post()
                .uri("/exchange-v2")
                .set_json(envelope(action))
                .to_request();
            let response: Value = test::call_and_read_body_json(&app, request).await;
            assert_eq!(response, expected);
            assert_eq!(
                response["response"]["data"]["statuses"]
                    .as_array()
                    .unwrap()
                    .len(),
                2
            );
        }
    }

    #[actix_web::test]
    async fn unknown_action_returns_bad_request() {
        let app = test::init_service(App::new().service(build_exchange_v2_scope())).await;
        let request = test::TestRequest::post()
            .uri("/exchange-v2")
            .set_json(envelope(json!({"type": "doesNotExist"})))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body: Value = test::read_body_json(response).await;
        assert!(
            body["error"]
                .as_str()
                .unwrap()
                .starts_with("Unsupported action.type")
        );
    }

    #[actix_web::test]
    async fn rejects_unknown_payload_fields() {
        let app = test::init_service(App::new().service(build_exchange_v2_scope())).await;
        let request = test::TestRequest::post()
            .uri("/exchange-v2")
            .set_json(envelope(json!({"type":"noop","extra":true})))
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::BAD_REQUEST
        );
    }

    #[actix_web::test]
    async fn envelope_error_precedes_unknown_action_error() {
        let app = test::init_service(App::new().service(build_exchange_v2_scope())).await;
        let mut body = envelope(json!({"type":"doesNotExist"}));
        body["nonce"] = json!(0);
        let request = test::TestRequest::post()
            .uri("/exchange-v2")
            .set_json(body)
            .to_request();
        let response: Value = test::call_and_read_body_json(&app, request).await;
        assert_eq!(
            response["error"],
            "Invalid `nonce`. Expected a positive millisecond timestamp."
        );
    }

    #[actix_web::test]
    async fn only_exchange_v2_route_is_mounted() {
        let app = test::init_service(App::new().service(build_exchange_v2_scope())).await;

        let v2_request = test::TestRequest::post()
            .uri("/exchange-v2")
            .set_json(envelope(json!({"type": "noop"})))
            .to_request();
        assert_eq!(
            test::call_service(&app, v2_request).await.status(),
            StatusCode::OK
        );

        let old_request = test::TestRequest::post()
            .uri("/exchange")
            .set_json(envelope(json!({"type": "noop"})))
            .to_request();
        assert_eq!(
            test::call_service(&app, old_request).await.status(),
            StatusCode::NOT_FOUND
        );
    }
}
