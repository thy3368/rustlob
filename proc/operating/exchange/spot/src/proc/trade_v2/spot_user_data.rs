use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_user_data_behavior::{SpotUserDataBehavior, SpotUserDataCmdAny, SpotUserDataRes};

pub struct SpotUserDataImpl {}

impl SpotUserDataBehavior for SpotUserDataImpl {
    fn handle(&mut self, cmd: SpotUserDataCmdAny) -> Result<CmdResp<SpotUserDataRes>, SpotCmdError> {
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
