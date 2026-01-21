use crate::proc::behavior::spot_trade_behavior::{CmdResp, SpotCmdError};
use crate::proc::behavior::v2::spot_user_data_behavior::{SpotUserDataBehavior, SpotUserDataCmdAny, SpotUserDataRes};

pub struct SpotUserDataImpl {}

impl SpotUserDataBehavior for SpotUserDataImpl {
    fn handle(&mut self, cmd: SpotUserDataCmdAny) -> Result<CmdResp<SpotUserDataRes>, SpotCmdError> {
        match cmd {
            SpotUserDataCmdAny::Account(_) => {}
            SpotUserDataCmdAny::QueryOrder(_) => {}
            SpotUserDataCmdAny::CurrentOpenOrders(_) => {}
            SpotUserDataCmdAny::AllOrders(_) => {}
            SpotUserDataCmdAny::QueryOrderList(_) => {}
            SpotUserDataCmdAny::QueryAllOrderList(_) => {}
            SpotUserDataCmdAny::QueryOpenOrderList(_) => {}
            SpotUserDataCmdAny::MyTrades(_) => {}
            SpotUserDataCmdAny::QueryUnfilledOrderCount(_) => {}
            SpotUserDataCmdAny::QueryPreventedMatches(_) => {}
            SpotUserDataCmdAny::QueryAllocations(_) => {}
            SpotUserDataCmdAny::QueryCommissionRates(_) => {}
        }
        todo!()
    }
}
