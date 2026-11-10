use common_entity::{ComputeView, QueryExecutionError, QueryExecutor};
use example_core_use_case::{
    OpenSpotOrdersQuery, OpenSpotOrdersQueryUseCase, OrderSide, SpotOrderV7,
};
use example_outbound_adapter::state_source::ScheduleCancelSpotOrderSource;

use crate::bootstrap::on_command::hyperliquid_ws::WsBasicOrder;
use crate::bootstrap::on_command::info::common::validate::ensure_type;
use crate::bootstrap::on_command::info::error::InfoHttpError;
use crate::bootstrap::on_command::info::queries::InfoQueryDeps;

pub mod reply {
    pub type ResponseWire = Vec<crate::bootstrap::on_command::hyperliquid_ws::WsBasicOrder>;
}

#[derive(Debug, serde::Deserialize)]
pub struct RequestWire {
    #[serde(rename = "type")]
    type_: String,
    user: String,
    dex: Option<String>,
}

#[derive(Debug, Clone, Copy)]
struct OpenSpotOrdersView;

impl ComputeView<Vec<SpotOrderV7>> for OpenSpotOrdersView {
    type View = reply::ResponseWire;

    fn compute_view(&self, orders: Vec<SpotOrderV7>) -> Self::View {
        orders
            .into_iter()
            .map(|order| {
                let base = order.order_base();
                WsBasicOrder {
                    coin: base.symbol.base.to_string(),
                    side: match base.side {
                        OrderSide::Buy => "B",
                        OrderSide::Sell => "A",
                    }
                    .to_string(),
                    sz: base.size.to_string(),
                    limit_px: Some(base.limit_px.to_string()),
                    oid: base.order_id,
                    timestamp: base.created_at,
                    extra: Default::default(),
                }
            })
            .collect()
    }
}

pub async fn handle(
    body: &[u8],
    _deps: &InfoQueryDeps,
) -> Result<reply::ResponseWire, InfoHttpError> {
    let request: RequestWire = crate::common::parse::parse_json_request(body)?;
    ensure_type(&request.type_, "openOrders")?;
    let query = OpenSpotOrdersQuery { user: request.user, dex: request.dex };
    let source =
        ScheduleCancelSpotOrderSource::new(crate::bootstrap::runtime::schedule_cancel_outbound());

    let orders =
        QueryExecutor.execute(&query, &source, &OpenSpotOrdersQueryUseCase).map_err(|error| {
            match error {
                QueryExecutionError::Business(error) => {
                    InfoHttpError::validation(error.to_string())
                }
                QueryExecutionError::LoadReadModel(error) => {
                    InfoHttpError::validation(format!("spot order store unavailable: {error}"))
                }
            }
        })?;

    Ok(OpenSpotOrdersView.compute_view(orders))
}

#[cfg(test)]
mod tests {
    use common_entity::{QueryUseCase, ReadModelSource};
    use example_core_use_case::account::coin::Coin;
    use example_core_use_case::account::symbol::Symbol;
    use example_core_use_case::common::common_order::OrderBase;
    use example_core_use_case::{SpotOrderStatus, SpotOrderV7ReadModel, UserId};
    use rust_decimal::Decimal;

    use super::*;

    const USER: &str = "0x1111111111111111111111111111111111111111";
    const OTHER_USER: &str = "0x2222222222222222222222222222222222222222";

    fn query(user: &str, dex: Option<&str>) -> OpenSpotOrdersQuery {
        OpenSpotOrdersQuery { user: user.to_string(), dex: dex.map(str::to_string) }
    }

    fn order(
        user: &str,
        order_id: u64,
        status: SpotOrderStatus,
        side: OrderSide,
        created_at: u64,
    ) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(common_entity::stable_string_entity_id(user)),
            order_id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px: Decimal::new(29_792, 1),
            size: Decimal::new(15, 1),
            orig_size: Decimal::from(2),
            status,
            created_at,
            updated_at: created_at,
            ..OrderBase::default()
        })
        .expect("测试订单应有效")
    }

    #[test]
    fn validates_user_and_optional_dex() {
        let use_case = OpenSpotOrdersQueryUseCase;

        assert!(use_case.validate_query(&query(USER, None)).is_ok());
        assert!(use_case.validate_query(&query(USER, Some("main"))).is_ok());
        assert!(use_case.validate_query(&query("", None)).is_err());
        assert!(use_case.validate_query(&query("   ", None)).is_err());
        assert!(use_case.validate_query(&query(USER, Some("   "))).is_err());
    }

    #[test]
    fn query_use_case_returns_only_users_open_v7_orders_in_stable_order() {
        let statuses = [
            SpotOrderStatus::Pending,
            SpotOrderStatus::Open,
            SpotOrderStatus::PartiallyFilled,
            SpotOrderStatus::Filled,
            SpotOrderStatus::Canceled,
            SpotOrderStatus::Rejected,
        ];
        let mut orders = statuses
            .into_iter()
            .enumerate()
            .map(|(index, status)| {
                order(USER, 100 + index as u64, status, OrderSide::Buy, 20 + index as u64)
            })
            .collect::<Vec<_>>();
        orders.push(order(OTHER_USER, 1, SpotOrderStatus::Open, OrderSide::Sell, 1));
        orders.push(order(USER, 99, SpotOrderStatus::Open, OrderSide::Sell, 10));
        orders.push(order(USER, 98, SpotOrderStatus::Open, OrderSide::Buy, 10));

        let result = OpenSpotOrdersQueryUseCase
            .compute_query(&query(USER, Some("main")), SpotOrderV7ReadModel(orders))
            .expect("应计算开放订单结果");

        assert_eq!(
            result.iter().map(|order| order.order_base().order_id).collect::<Vec<_>>(),
            [98, 99, 100, 101, 102]
        );
    }

    #[test]
    fn compute_view_maps_accepted_orders_to_wire_fields() {
        let orders = vec![
            order(USER, 98, SpotOrderStatus::Open, OrderSide::Buy, 10),
            order(USER, 99, SpotOrderStatus::Open, OrderSide::Sell, 10),
        ];

        let result = OpenSpotOrdersView.compute_view(orders);

        assert_eq!(result[0].coin, "BTC");
        assert_eq!(result[0].side, "B");
        assert_eq!(result[0].sz, "1.5");
        assert_eq!(result[0].limit_px.as_deref(), Some("2979.2"));
        assert_eq!(result[0].timestamp, 10);
        assert_eq!(result[1].side, "A");
    }

    #[test]
    fn executor_result_is_projected_to_response_wire_outside_executor() {
        struct Source;

        impl ReadModelSource<OpenSpotOrdersQuery> for Source {
            type ReadModel = SpotOrderV7ReadModel;
            type Error = std::convert::Infallible;

            fn load_read_model(
                &self,
                _query: &OpenSpotOrdersQuery,
            ) -> Result<Self::ReadModel, Self::Error> {
                Ok(SpotOrderV7ReadModel(vec![order(
                    USER,
                    7,
                    SpotOrderStatus::Open,
                    OrderSide::Buy,
                    10,
                )]))
            }
        }

        let query = query(USER, None);
        let output = QueryExecutor
            .execute(&query, &Source, &OpenSpotOrdersQueryUseCase)
            .expect("查询应成功");
        let response = OpenSpotOrdersView.compute_view(output);

        assert_eq!(response.len(), 1);
        assert_eq!(response[0].oid, 7);
    }

    #[test]
    fn query_executor_maps_business_and_source_errors() {
        #[derive(Debug, thiserror::Error)]
        #[error("load failed")]
        struct LoadError;

        struct FailingSource;

        impl ReadModelSource<OpenSpotOrdersQuery> for FailingSource {
            type ReadModel = SpotOrderV7ReadModel;
            type Error = LoadError;

            fn load_read_model(
                &self,
                _query: &OpenSpotOrdersQuery,
            ) -> Result<Self::ReadModel, Self::Error> {
                Err(LoadError)
            }
        }

        assert!(matches!(
            QueryExecutor.execute(&query("", None), &FailingSource, &OpenSpotOrdersQueryUseCase),
            Err(QueryExecutionError::Business(_))
        ));
        assert!(matches!(
            QueryExecutor.execute(&query(USER, None), &FailingSource, &OpenSpotOrdersQueryUseCase),
            Err(QueryExecutionError::LoadReadModel(LoadError))
        ));
    }
}
