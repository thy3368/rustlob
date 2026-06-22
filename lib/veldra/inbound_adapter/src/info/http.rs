use actix_web::{HttpResponse, Scope, web};

use crate::info::common::wire::InfoRequestTypeProbe;
use crate::info::error::InfoHttpError;
use crate::info::{InfoQueryDeps, InfoQueryReply, dispatch_info_query};

pub fn build_info_scope() -> Scope {
    web::scope("").route("/info", web::post().to(post_info))
}

async fn post_info(body: web::Bytes) -> Result<HttpResponse, InfoHttpError> {
    dispatch_info_query_from_body(&body).await
}

async fn dispatch_info_query_from_body(body: &[u8]) -> Result<HttpResponse, InfoHttpError> {
    let probe = parse_query_type_probe(body)?;
    let deps = InfoQueryDeps::default();
    let reply = dispatch_info_query(&probe.type_, body, &deps).await?;
    Ok(reply_to_http(reply))
}

fn parse_query_type_probe(body: &[u8]) -> Result<InfoRequestTypeProbe, InfoHttpError> {
    crate::common::parse::parse_json_request(body)
}

fn reply_to_http(reply: InfoQueryReply) -> HttpResponse {
    HttpResponse::Ok().json(reply.into_json_value())
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test as actix_test};
    use serde_json::{Value, json};

    use super::*;
    use crate::info::test_support::{
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
