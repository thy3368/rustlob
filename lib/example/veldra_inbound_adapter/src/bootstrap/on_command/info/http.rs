use actix_web::{HttpResponse, Scope, web};

use crate::bootstrap::on_command::info::common::wire::InfoRequestTypeProbe;
use crate::bootstrap::on_command::info::error::InfoHttpError;
use crate::bootstrap::on_command::info::{InfoQueryDeps, InfoQueryReply, dispatch_info_query};

pub fn build_info_scope() -> Scope {
    web::scope("/info").route("", web::post().to(post_info))
}

async fn post_info(body: web::Bytes) -> Result<HttpResponse, InfoHttpError> {
    dispatch_info_query_from_body(&body).await
}

async fn dispatch_info_query_from_body(body: &[u8]) -> Result<HttpResponse, InfoHttpError> {
    let probe = parse_query_type_probe(body)?;
    let deps = InfoQueryDeps;
    let reply = dispatch_info_query(&probe.type_, body, &deps).await?;
    reply_to_http(reply)
}

fn parse_query_type_probe(body: &[u8]) -> Result<InfoRequestTypeProbe, InfoHttpError> {
    crate::common::parse::parse_json_request(body)
}

fn reply_to_http(reply: InfoQueryReply) -> Result<HttpResponse, InfoHttpError> {
    let value = reply.into_json_value().map_err(InfoHttpError::SerializeReply)?;
    Ok(HttpResponse::Ok().json(value))
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test as actix_test};
    use serde_json::{Value, json};

    use super::*;
    use crate::bootstrap::on_command::info::test_support::{
        query_contract_cases, valid_all_mids_request_value, valid_order_status_with_cloid_value,
    };

    #[actix_web::test]
    async fn all_mids_is_dispatched_to_info_handler() {
        let app = actix_test::init_service(App::new().service(build_info_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(valid_all_mids_request_value())
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body, json!({ "APE": "4.33245", "ARB": "1.21695" }));
    }

    #[actix_web::test]
    async fn order_status_accepts_cloid_shape() {
        let app = actix_test::init_service(App::new().service(build_info_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(valid_order_status_with_cloid_value())
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["status"], "order");
    }

    // competitor: hyperliquid; feature: exchange writes -> info projections; state: GREEN
    #[actix_web::test]
    async fn placed_v2_order_is_visible_in_frontend_and_status_but_not_v7_open_orders() {
        let user = "0x8888888888888888888888888888888888888888";
        let app = actix_test::init_service(
            App::new()
                .service(crate::bootstrap::on_command::exchange::http::build_exchange_scope())
                .service(build_info_scope()),
        )
        .await;
        let oid = 1_710_000_040_000u64;
        let place = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000, "b": true, "p": "123", "s": "4", "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }],
                    "grouping": "na"
                },
                "nonce": oid,
                "vaultAddress": user,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let place_response = actix_test::call_service(&app, place).await;
        assert_eq!(place_response.status(), StatusCode::OK);

        let open_orders = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "frontendOpenOrders", "user": user, "dex": "main" }))
            .to_request();
        let open_orders_response = actix_test::call_service(&app, open_orders).await;
        assert_eq!(open_orders_response.status(), StatusCode::OK);
        let open_orders: Value = actix_test::read_body_json(open_orders_response).await;
        assert_eq!(open_orders[0]["oid"], oid);
        assert_eq!(open_orders[0]["limitPx"], "123");
        assert_eq!(open_orders[0]["sz"], "4");
        assert_eq!(open_orders[0]["side"], "B");

        let basic_open_orders = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "openOrders", "user": user, "dex": "main" }))
            .to_request();
        let basic_open_orders_response = actix_test::call_service(&app, basic_open_orders).await;
        assert_eq!(basic_open_orders_response.status(), StatusCode::OK);
        let basic_open_orders: Value = actix_test::read_body_json(basic_open_orders_response).await;
        assert_eq!(basic_open_orders, json!([]));

        let order_status = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "orderStatus", "user": user, "oid": oid }))
            .to_request();
        let order_status_response = actix_test::call_service(&app, order_status).await;
        assert_eq!(order_status_response.status(), StatusCode::OK);
        let order_status: Value = actix_test::read_body_json(order_status_response).await;
        assert_eq!(order_status["status"], "order");
        assert_eq!(order_status["order"]["status"], "open");
        assert_eq!(order_status["order"]["order"]["oid"], oid);

        let cancel = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": { "type": "cancel", "cancels": [{ "a": 10000, "o": oid }] },
                "nonce": oid + 1,
                "vaultAddress": user,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let cancel_response = actix_test::call_service(&app, cancel).await;
        assert_eq!(cancel_response.status(), StatusCode::OK);

        let no_open_orders = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "frontendOpenOrders", "user": user, "dex": "main" }))
            .to_request();
        let no_open_orders: Value = actix_test::call_and_read_body_json(&app, no_open_orders).await;
        assert_eq!(no_open_orders, json!([]));

        let history = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "historicalOrders", "user": user }))
            .to_request();
        let history: Value = actix_test::call_and_read_body_json(&app, history).await;
        assert_eq!(history[0]["order"]["oid"], oid);
        assert_eq!(history[0]["status"], "canceled");
    }

    // competitor: hyperliquid; feature: order -> batchModify -> info; state: GREEN
    #[actix_web::test]
    async fn two_resting_orders_can_be_batch_modified_and_queried() {
        let user = "0x9999999999999999999999999999999999999999";
        let first_oid = 1_710_000_050_000u64;
        let second_oid = first_oid + 1;
        let app = actix_test::init_service(
            App::new()
                .service(crate::bootstrap::on_command::exchange::http::build_exchange_scope())
                .service(build_info_scope()),
        )
        .await;

        let place = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "order",
                    "orders": [
                        {
                            "a": 10000, "b": true, "p": "100", "s": "2", "r": false,
                            "t": { "limit": { "tif": "Gtc" } }
                        },
                        {
                            "a": 10000, "b": false, "p": "120", "s": "4", "r": false,
                            "t": { "limit": { "tif": "Gtc" } }
                        }
                    ],
                    "grouping": "na"
                },
                "nonce": first_oid,
                "vaultAddress": user,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let place_response = actix_test::call_service(&app, place).await;
        assert_eq!(place_response.status(), StatusCode::OK);
        let place_body: Value = actix_test::read_body_json(place_response).await;
        assert_eq!(place_body["response"]["data"]["statuses"][0]["resting"]["oid"], first_oid);
        assert_eq!(place_body["response"]["data"]["statuses"][1]["resting"]["oid"], second_oid);

        let batch_modify = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "batchModify",
                    "modifies": [
                        {
                            "oid": first_oid,
                            "order": {
                                "a": 10000, "b": true, "p": "105", "s": "3", "r": false,
                                "t": { "limit": { "tif": "Gtc" } }
                            }
                        },
                        {
                            "oid": second_oid,
                            "order": {
                                "a": 10000, "b": false, "p": "115", "s": "5", "r": false,
                                "t": { "limit": { "tif": "Gtc" } }
                            }
                        }
                    ]
                },
                "nonce": second_oid + 1,
                "vaultAddress": user,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let modify_response = actix_test::call_service(&app, batch_modify).await;
        assert_eq!(modify_response.status(), StatusCode::OK);
        let modify_body: Value = actix_test::read_body_json(modify_response).await;
        assert_eq!(
            modify_body["response"]["data"]["statuses"],
            json!([
                { "resting": { "oid": first_oid } },
                { "resting": { "oid": second_oid } }
            ])
        );

        let open_orders = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "frontendOpenOrders", "user": user, "dex": "main" }))
            .to_request();
        let open_orders: Value = actix_test::call_and_read_body_json(&app, open_orders).await;
        let orders = open_orders.as_array().expect("open orders 应为数组");
        assert_eq!(orders.len(), 2);
        let first =
            orders.iter().find(|order| order["oid"] == first_oid).expect("应查询到第一笔订单");
        let second =
            orders.iter().find(|order| order["oid"] == second_oid).expect("应查询到第二笔订单");
        assert_eq!((first["limitPx"].as_str(), first["sz"].as_str()), (Some("105"), Some("3")));
        assert_eq!((second["limitPx"].as_str(), second["sz"].as_str()), (Some("115"), Some("5")));

        let history = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "historicalOrders", "user": user }))
            .to_request();
        let history: Value = actix_test::call_and_read_body_json(&app, history).await;
        assert_eq!(history.as_array().map(Vec::len), Some(2));
        assert_eq!(history[0]["order"]["oid"], second_oid);
        assert_eq!(history[1]["order"]["oid"], first_oid);
    }

    // competitor: hyperliquid; feature: orderStatus by cloid; state: GREEN
    #[actix_web::test]
    async fn placed_order_can_be_queried_by_cloid_only_for_its_owner() {
        let user = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let other_user = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
        let cloid = "0x0123456789abcdef0123456789abcdef";
        let unknown_cloid = "0xfedcba9876543210fedcba9876543210";
        let oid = 1_710_000_060_000u64;
        let app = actix_test::init_service(
            App::new()
                .service(crate::bootstrap::on_command::exchange::http::build_exchange_scope())
                .service(build_info_scope()),
        )
        .await;

        let place = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000, "b": true, "p": "130", "s": "6", "r": false,
                        "t": { "limit": { "tif": "Gtc" } },
                        "c": cloid
                    }],
                    "grouping": "na"
                },
                "nonce": oid,
                "vaultAddress": user,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let place_response = actix_test::call_service(&app, place).await;
        assert_eq!(place_response.status(), StatusCode::OK);

        let by_cloid = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "orderStatus", "user": user, "oid": cloid }))
            .to_request();
        let by_cloid: Value = actix_test::call_and_read_body_json(&app, by_cloid).await;
        assert_eq!(by_cloid["status"], "order");
        assert_eq!(by_cloid["order"]["status"], "open");
        assert_eq!(by_cloid["order"]["order"]["oid"], oid);
        assert_eq!(by_cloid["order"]["order"]["cloid"], cloid);
        assert_eq!(by_cloid["order"]["order"]["limitPx"], "130");
        assert_eq!(by_cloid["order"]["order"]["sz"], "6");

        let unknown = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "orderStatus", "user": user, "oid": unknown_cloid }))
            .to_request();
        let unknown: Value = actix_test::call_and_read_body_json(&app, unknown).await;
        assert_eq!(unknown, json!({ "status": "unknownOid" }));

        let wrong_owner = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "orderStatus", "user": other_user, "oid": cloid }))
            .to_request();
        let wrong_owner: Value = actix_test::call_and_read_body_json(&app, wrong_owner).await;
        assert_ne!(wrong_owner["order"]["order"]["oid"], oid);
        assert_ne!(wrong_owner["order"]["order"]["cloid"], cloid);
    }

    // competitor: hyperliquid; feature: exchange fills -> userFills/userFillsByTime; state: GREEN
    #[actix_web::test]
    async fn matched_spot_trade_is_visible_to_taker_and_maker_fill_queries() {
        let seller = "0xcccccccccccccccccccccccccccccccccccccccc";
        let buyer = "0xdddddddddddddddddddddddddddddddddddddddd";
        let maker_oid = 1_710_000_070_000u64;
        let taker_oid = maker_oid + 1;
        let app = actix_test::init_service(
            App::new()
                .service(crate::bootstrap::on_command::exchange::http::build_exchange_scope())
                .service(build_info_scope()),
        )
        .await;

        let maker_order = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000, "b": false, "p": "100", "s": "2", "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }],
                    "grouping": "na"
                },
                "nonce": maker_oid,
                "vaultAddress": seller,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let maker_response = actix_test::call_service(&app, maker_order).await;
        assert_eq!(maker_response.status(), StatusCode::OK);

        let no_fills = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "userFills", "user": seller }))
            .to_request();
        let no_fills: Value = actix_test::call_and_read_body_json(&app, no_fills).await;
        assert_eq!(no_fills, json!([]));

        let taker_order = actix_test::TestRequest::post()
            .uri("/exchange")
            .set_json(json!({
                "action": {
                    "type": "order",
                    "orders": [{
                        "a": 10000, "b": true, "p": "100", "s": "2", "r": false,
                        "t": { "limit": { "tif": "Gtc" } }
                    }],
                    "grouping": "na"
                },
                "nonce": taker_oid,
                "vaultAddress": buyer,
                "signature": {
                    "r": "0x1111111111111111111111111111111111111111111111111111111111111111",
                    "s": "0x2222222222222222222222222222222222222222222222222222222222222222",
                    "v": 27
                }
            }))
            .to_request();
        let taker_response = actix_test::call_service(&app, taker_order).await;
        assert_eq!(taker_response.status(), StatusCode::OK);
        let taker_body: Value = actix_test::read_body_json(taker_response).await;
        assert_eq!(
            taker_body["response"]["data"]["statuses"][0]["filled"]["oid"], taker_oid,
            "跨价买单应成交: {taker_body}"
        );

        let buyer_fills = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "userFills", "user": buyer }))
            .to_request();
        let buyer_fills: Value = actix_test::call_and_read_body_json(&app, buyer_fills).await;
        assert_eq!(buyer_fills.as_array().map(Vec::len), Some(1));
        assert_eq!(buyer_fills[0]["coin"], "@0");
        assert_eq!(buyer_fills[0]["side"], "B");
        assert_eq!(buyer_fills[0]["oid"], taker_oid);
        assert_eq!(buyer_fills[0]["crossed"], true);
        assert_eq!(buyer_fills[0]["px"], "100");
        assert_eq!(buyer_fills[0]["sz"], "2");
        assert_eq!(buyer_fills[0]["feeToken"], "USDT");

        let seller_fills = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "userFills", "user": seller }))
            .to_request();
        let seller_fills: Value = actix_test::call_and_read_body_json(&app, seller_fills).await;
        assert_eq!(seller_fills.as_array().map(Vec::len), Some(1));
        assert_eq!(seller_fills[0]["side"], "A");
        assert_eq!(seller_fills[0]["oid"], maker_oid);
        assert_eq!(seller_fills[0]["crossed"], false);
        assert_eq!(seller_fills[0]["tid"], buyer_fills[0]["tid"]);

        let inclusive = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({
                "type": "userFillsByTime",
                "user": buyer,
                "startTime": taker_oid,
                "endTime": taker_oid
            }))
            .to_request();
        let inclusive: Value = actix_test::call_and_read_body_json(&app, inclusive).await;
        assert_eq!(inclusive.as_array().map(Vec::len), Some(1));

        let outside = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({
                "type": "userFillsByTime",
                "user": buyer,
                "startTime": taker_oid + 1,
                "endTime": taker_oid + 1
            }))
            .to_request();
        let outside: Value = actix_test::call_and_read_body_json(&app, outside).await;
        assert_eq!(outside, json!([]));
    }

    #[actix_web::test]
    async fn unsupported_query_returns_stable_error_shape() {
        let app = actix_test::init_service(App::new().service(build_info_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": "doesNotExist" }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(
            body,
            json!({
                "status": "err",
                "error": "Unsupported type `doesNotExist`. Supported queries: `allMids`, `openOrders`, `frontendOpenOrders`, `userFills`, `userFillsByTime`, `userRateLimit`, `orderStatus`, `l2Book`, `candleSnapshot`, `maxBuilderFee`, `historicalOrders`, `userTwapSliceFills`, `subAccounts`, `vaultDetails`, `userVaultEquities`, `userRole`, `portfolio`, `referral`, `userFees`, `delegations`, `delegatorSummary`, `delegatorHistory`, `delegatorRewards`, `userDexAbstraction`, `userAbstraction`, `alignedQuoteTokenInfo`, `borrowLendUserState`, `borrowLendReserveState`, `allBorrowLendReserveStates`, `approvedBuilders`."
            })
        );
    }

    #[actix_web::test]
    async fn malformed_json_maps_to_stable_error_shape() {
        let app = actix_test::init_service(App::new().service(build_info_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/info")
            .insert_header(("content-type", "application/json"))
            .set_payload(br#"{"type":"allMids""# as &[u8])
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body, json!({ "status": "err", "error": "Malformed JSON body." }));
    }

    #[actix_web::test]
    async fn invalid_shape_json_maps_to_stable_error_shape() {
        let app = actix_test::init_service(App::new().service(build_info_scope())).await;
        let request = actix_test::TestRequest::post()
            .uri("/info")
            .set_json(json!({ "type": 1 }))
            .to_request();

        let response = actix_test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body: Value = actix_test::read_body_json(response).await;
        assert_eq!(body["status"], "err");
        assert!(
            body["error"]
                .as_str()
                .unwrap_or_default()
                .starts_with("Missing or invalid request fields:")
        );
    }

    #[actix_web::test]
    async fn every_query_contract_has_a_route_level_happy_case() {
        let app = actix_test::init_service(App::new().service(build_info_scope())).await;

        for case in query_contract_cases() {
            let request =
                actix_test::TestRequest::post().uri("/info").set_json(case.request).to_request();
            let response = actix_test::call_service(&app, request).await;
            assert_eq!(response.status(), StatusCode::OK, "case `{}`", case.query_type);
            let body: Value = actix_test::read_body_json(response).await;
            assert_eq!(body, case.expected_response, "case `{}`", case.query_type);
        }
    }
}
