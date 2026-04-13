use cmd_handler::handler_query::QueryHandler;
use db_repo::core::db_repo2::QueryRepo2;
use crate::proc::behavior::v2::spot_trade_api_error::{
    ApiError, SpotErrorAny, TraderError,
};
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
    type StateSet = Option<OrderInfo>;
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
            let order_id = order_id as i64;

            // 在repo2 trait上怎么抽象
            match self.repo.find_by_id(&order_id) {
                Ok(Some(order)) => Ok(Some(order)),
                Ok(None) => Err(TraderError::OrderNotFound {
                    order_id: order_id.to_string(),
                    action: "请检查订单号是否正确",
                }
                .into()),
                Err(_) => {
                    Err(ApiError::ServiceUnavailable { service: "database", code: -3001 }.into())
                }
            }
        } else if let Some(ref orig_client_order_id) = cmd.orig_client_order_id {
            // 在repo2 trait上怎么抽象
            match self.repo.find_order_by_orig_client_order_id(orig_client_order_id) {
                Ok(Some(order)) => Ok(Some(order)),
                Ok(None) => Err(TraderError::OrderNotFound {
                    order_id: orig_client_order_id.clone(),
                    action: "请检查订单号是否正确",
                }
                .into()),
                Err(_) => {
                    Err(ApiError::ServiceUnavailable { service: "database", code: -3001 }.into())
                }
            }
        } else {
            Ok(None)
        }
    }

    fn state_set_to_reply(&self, state_set: Self::StateSet) -> Self::Reply {
        Ok(state_set)
    }
}
