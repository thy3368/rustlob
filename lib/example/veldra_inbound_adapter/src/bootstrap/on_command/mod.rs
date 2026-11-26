pub mod exchange_v2;
pub mod hyperliquid_ws;
pub mod info;
pub mod info_v2;

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::{App, test};
    use serde_json::json;

    use super::info::http::build_info_scope;
    use super::info_v2::build_info_v2_scope;

    #[actix_web::test]
    async fn info_and_info_v2_scopes_coexist() {
        let app = test::init_service(
            App::new()
                .service(build_info_scope())
                .service(build_info_v2_scope()),
        )
        .await;

        for uri in ["/info", "/info-v2"] {
            let request = test::TestRequest::post()
                .uri(uri)
                .set_json(json!({ "type": "allMids" }))
                .to_request();
            assert_eq!(
                test::call_service(&app, request).await.status(),
                StatusCode::OK
            );
        }
    }
}
