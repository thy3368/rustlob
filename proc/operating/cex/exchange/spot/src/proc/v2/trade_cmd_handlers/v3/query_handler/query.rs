use base_types::exchange::spot::spot_types::{OrderStatus, SpotOrder};
use cmd_handler::handler_query::QueryHandler;
use db_repo::core::db_repo2::QueryRepo2;

use crate::proc::behavior::v2::spot_trade_api_error::{ApiError, SpotErrorAny, TraderError};
use crate::proc::behavior::v2::spot_user_data_behavior::{OrderInfo, QueryOrderCmd};

pub struct QueryOrderHandler<R: QueryRepo2> {
    repo: R,
}

impl<R: QueryRepo2> QueryOrderHandler<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: QueryRepo2> QueryHandler for QueryOrderHandler<R> {
    type Query = QueryOrderCmd;
    type Reply = Result<Option<OrderInfo>, SpotErrorAny>;
    type StateSet = Option<SpotOrder>;
    type Error = SpotErrorAny;

    fn pre_check_command(&self, cmd: &Self::Query) -> Result<(), Self::Error> {
        if cmd.order_id.is_none() && cmd.orig_client_order_id.is_none() {
            return Err(ApiError::InvalidField {
                field: "order_id or orig_client_order_id",
                reason: "one is required",
                code: -1001,
                doc: "https://docs.rustlob.com/errors/-1001",
            }
            .into());
        }
        if cmd.symbol.is_empty() {
            return Err(ApiError::InvalidField {
                field: "symbol",
                reason: "cannot be empty",
                code: -1001,
                doc: "https://docs.rustlob.com/errors/-1001",
            }
            .into());
        }
        Ok(())
    }

    fn load_state_set(&self, cmd: &Self::Query) -> Result<Self::StateSet, Self::Error> {
        if let Some(order_id) = cmd.order_id {
            let entity_id = order_id.to_string();
            match self.repo.find_by_id::<SpotOrder>(&entity_id) {
                Ok(Some(order)) => Ok(Some(order)),
                Ok(None) => Err(TraderError::OrderNotFound {
                    order_id: entity_id.clone(),
                    action: "请检查订单号是否正确",
                }
                .into()),
                Err(_) => {
                    Err(ApiError::ServiceUnavailable { service: "database", code: -3001 }.into())
                }
            }
        } else if cmd.orig_client_order_id.is_some() {
            //todo 通过client_order_id查
            Err(ApiError::NotFound {
                resource: "orig_client_order_id query - use order_id instead",
                code: -1002,
            }
            .into())
        } else {
            Ok(None)
        }
    }

    fn state_set_to_reply(&self, state_set: Self::StateSet) -> Self::Reply {
        match state_set {
            Some(order) => {
                let info = OrderInfo {
                    symbol: format!("{:?}", order.trading_pair),
                    order_id: order.order_id as i64,
                    order_list_id: -1,
                    client_order_id: order.client_order_id.unwrap_or_default(),
                    price: order
                        .price
                        .map(|p| format!("{:?}", p))
                        .unwrap_or_else(|| "0".to_string()),
                    orig_qty: format!("{:?}", order.total_base_qty),
                    executed_qty: format!("{:?}", order.state.filled_base_qty),
                    cummulative_quote_qty: format!("{:?}", order.state.cumulative_quote_qty),
                    status: format!("{:?}", order.state.status),
                    time_in_force: format!("{:?}", order.time_in_force),
                    order_type: format!("{:?}", order.execution_method),
                    side: format!("{:?}", order.side),
                    stop_price: order.stop_price.map(|p| format!("{:?}", p)),
                    iceberg_qty: order.iceberg_qty.map(|q| format!("{:?}", q)),
                    time: order.timestamp.0 as i64,
                    update_time: order.state.last_updated.0 as i64,
                    is_working: matches!(
                        order.state.status,
                        OrderStatus::Pending | OrderStatus::PartiallyFilled
                    ),
                    working_time: 0,
                    orig_quote_order_qty: format!("{:?}", order.total_quote_qty),
                    self_trade_prevention_mode: format!("{:?}", order.self_trade_prevention),
                };
                Ok(Some(info))
            }
            None => Ok(None),
        }
    }
}
