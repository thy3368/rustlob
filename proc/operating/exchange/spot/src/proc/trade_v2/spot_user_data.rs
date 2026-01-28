use base_types::handler::handler::Handler;

use crate::proc::behavior::{
    spot_trade_behavior::{CmdResp, SpotCmdErrorAny},
    v2::spot_user_data_behavior::{SpotUserDataCmdAny, SpotUserDataResAny}
};

pub struct SpotUserDataImpl {}

impl SpotUserDataImpl {
    pub fn new() -> Self { Self {} }
}


impl Handler<SpotUserDataCmdAny, SpotUserDataResAny, SpotCmdErrorAny> for SpotUserDataImpl {
    async fn handle(&self, cmd: SpotUserDataCmdAny) -> Result<CmdResp<SpotUserDataResAny>, SpotCmdErrorAny> {
        match cmd {
            SpotUserDataCmdAny::Account(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryOrder(_) => {
                todo!()
            }
            SpotUserDataCmdAny::CurrentOpenOrders(_) => {
                todo!()
            }
            SpotUserDataCmdAny::AllOrders(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryOrderList(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryAllOrderList(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryOpenOrderList(_) => {
                todo!()
            }
            SpotUserDataCmdAny::MyTrades(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryUnfilledOrderCount(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryPreventedMatches(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryAllocations(_) => {
                todo!()
            }
            SpotUserDataCmdAny::QueryCommissionRates(_) => {
                todo!()
            }
        }
    }
}
