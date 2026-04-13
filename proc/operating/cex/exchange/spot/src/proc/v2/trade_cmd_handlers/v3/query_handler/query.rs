use base_types::exchange::spot::spot_types::SpotOrder;
use cmd_handler::handler_query::QueryHandler;
use db_repo::core::db_repo2::{QueryRepo2, RepoError};
use db_repo::core::db_repo::QueryRepo;

use crate::proc::behavior::v2::spot_trade_error::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_user_data_behavior::{OrderInfo, QueryOrderCmd};

pub struct QueryOrderHandler<R: QueryRepo> {
    repo: R,
}

impl<R: QueryRepo2> QueryOrderHandler<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: QueryRepo2> QueryHandler for QueryOrderHandler<R> {
    type Query = QueryOrderCmd;
    type Reply = Result<Option<OrderInfo>, SpotCmdErrorAny>;
    type StateSet = Result<Option<SpotOrder>, RepoError>;
    type Error = SpotCmdErrorAny;

    fn pre_check_command(&self, cmd: &Self::Query) -> Result<(), Self::Error> {
        todo!()
    }

    fn load_state_set(&self, cmd: &Self::Query) -> Result<Self::StateSet, Self::Error> {
        todo!()
    }

    fn state_set_to_reply(&self, state_set: Self::StateSet) -> Self::Reply {
        todo!()
    }
}
