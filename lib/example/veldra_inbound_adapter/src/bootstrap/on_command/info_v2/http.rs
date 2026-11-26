use actix_web::{HttpResponse, Scope, web};

use super::{InfoRequestV2, InfoResponseV2, InfoV2Error};

pub fn build_info_v2_scope() -> Scope {
    web::scope("/info-v2").route("", web::post().to(post_info_v2))
}

async fn post_info_v2(body: web::Bytes) -> Result<HttpResponse, InfoV2Error> {
    let request: InfoRequestV2 = serde_json::from_slice(&body).map_err(InfoV2Error::json)?;
    request.validate()?;
    Ok(HttpResponse::Ok().json(InfoResponseV2::for_request(&request)))
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test};
    use serde_json::{Value, json};

    use super::*;

    const USER: &str = "0x0000000000000000000000000000000000000000";

    fn requests() -> Vec<Value> {
        vec![
            json!({"type":"allMids"}),
            json!({"type":"openOrders","user":USER}),
            json!({"type":"frontendOpenOrders","user":USER}),
            json!({"type":"userFills","user":USER}),
            json!({"type":"userFillsByTime","user":USER,"startTime":1,"endTime":2}),
            json!({"type":"userRateLimit","user":USER}),
            json!({"type":"orderStatus","user":USER,"oid":1}),
            json!({"type":"l2Book","coin":"BTC"}),
            json!({"type":"candleSnapshot","req":{"coin":"BTC","interval":"15m","startTime":1,"endTime":2}}),
            json!({"type":"maxBuilderFee","user":USER,"builder":USER}),
            json!({"type":"historicalOrders","user":USER}),
            json!({"type":"userTwapSliceFills","user":USER}),
            json!({"type":"subAccounts","user":USER}),
            json!({"type":"vaultDetails","vaultAddress":USER}),
            json!({"type":"userVaultEquities","user":USER}),
            json!({"type":"userRole","user":USER}),
            json!({"type":"portfolio","user":USER}),
            json!({"type":"referral","user":USER}),
            json!({"type":"userFees","user":USER}),
            json!({"type":"delegations","user":USER}),
            json!({"type":"delegatorSummary","user":USER}),
            json!({"type":"delegatorHistory","user":USER}),
            json!({"type":"delegatorRewards","user":USER}),
            json!({"type":"userDexAbstraction","user":USER}),
            json!({"type":"userAbstraction","user":USER}),
            json!({"type":"alignedQuoteTokenInfo","token":0}),
            json!({"type":"borrowLendUserState","user":USER}),
            json!({"type":"borrowLendReserveState","token":0}),
            json!({"type":"allBorrowLendReserveStates"}),
            json!({"type":"approvedBuilders","user":USER}),
        ]
    }

    #[actix_web::test]
    async fn all_documented_requests_are_accepted() {
        let app = test::init_service(App::new().service(build_info_v2_scope())).await;
        for body in requests() {
            let request = test::TestRequest::post()
                .uri("/info-v2")
                .set_json(body)
                .to_request();
            assert_eq!(
                test::call_service(&app, request).await.status(),
                StatusCode::OK
            );
        }
    }

    #[actix_web::test]
    async fn every_request_variant_returns_a_nonempty_or_explicit_primitive_fixture() {
        let app = test::init_service(App::new().service(build_info_v2_scope())).await;
        let types = [
            "allMids",
            "openOrders",
            "frontendOpenOrders",
            "userFills",
            "userFillsByTime",
            "userRateLimit",
            "orderStatus",
            "l2Book",
            "candleSnapshot",
            "maxBuilderFee",
            "historicalOrders",
            "userTwapSliceFills",
            "subAccounts",
            "vaultDetails",
            "userVaultEquities",
            "userRole",
            "portfolio",
            "referral",
            "userFees",
            "delegations",
            "delegatorSummary",
            "delegatorHistory",
            "delegatorRewards",
            "userDexAbstraction",
            "userAbstraction",
            "alignedQuoteTokenInfo",
            "borrowLendUserState",
            "borrowLendReserveState",
            "allBorrowLendReserveStates",
            "approvedBuilders",
        ];
        for (body, type_) in requests().into_iter().zip(types) {
            let request = test::TestRequest::post()
                .uri("/info-v2")
                .set_json(body)
                .to_request();
            let response = test::call_service(&app, request).await;
            assert_eq!(response.status(), StatusCode::OK, "{type_}");
            let value: Value = test::read_body_json(response).await;
            assert_ne!(value, json!([]), "{type_} returned an empty placeholder");
            if type_ == "userFills" || type_ == "userFillsByTime" {
                assert!(value[0].get("startPosition").is_some(), "{type_}: {value}");
                assert!(value[0].get("start_position").is_none(), "{type_}: {value}");
            }
            if type_ == "orderStatus" {
                assert_eq!(value, json!({"status":"unknownOid"}));
            }
            if type_ == "maxBuilderFee"
                || type_ == "userDexAbstraction"
                || type_ == "userAbstraction"
            {
                assert!(!value.is_array() && !value.is_object(), "{type_}: {value}");
            }
        }
    }

    #[actix_web::test]
    async fn rejects_invalid_input_classes() {
        let app = test::init_service(App::new().service(build_info_v2_scope())).await;
        let cases = [
            "{".to_owned(),
            json!({"type":"missing"}).to_string(),
            json!({"type":"openOrders"}).to_string(),
            json!({"type":"openOrders","user":USER,"extra":true}).to_string(),
            json!({"type":"openOrders","user":"0x00"}).to_string(),
            json!({"type":"userFillsByTime","user":USER,"startTime":2,"endTime":1}).to_string(),
            json!({"type":"orderStatus","user":USER,"oid":"0x01"}).to_string(),
            json!({"type":"l2Book","coin":"BTC","nSigFigs":4,"mantissa":2}).to_string(),
        ];
        for body in cases {
            let request = test::TestRequest::post()
                .uri("/info-v2")
                .insert_header(("content-type", "application/json"))
                .set_payload(body)
                .to_request();
            let response = test::call_service(&app, request).await;
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            let value: Value = test::read_body_json(response).await;
            assert_eq!(value["status"], "err");
        }
    }
}
