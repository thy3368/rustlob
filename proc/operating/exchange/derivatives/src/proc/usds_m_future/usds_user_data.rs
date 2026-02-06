use base_types::cqrs::cqrs_types::CmdResp;

use crate::proc::usds_m_future::behavior::user_data_behavior::{
    UsdsMFutureUserDataBehavior, UsdsMFutureUserDataCmdAny, UsdsMFutureUserDataError,
    UsdsMFutureUserDataRes,
};

pub struct UsdsMFutureUserDataBehaviorImpl {}

impl UsdsMFutureUserDataBehavior for UsdsMFutureUserDataBehaviorImpl {
    fn handle(
        &mut self,
        cmd: UsdsMFutureUserDataCmdAny,
    ) -> Result<CmdResp<UsdsMFutureUserDataRes>, UsdsMFutureUserDataError> {
        match cmd {
            UsdsMFutureUserDataCmdAny::AccountInfoV3(_) => {}
            UsdsMFutureUserDataCmdAny::AccountInfoV2(_) => {}
            UsdsMFutureUserDataCmdAny::AccountBalanceV3(_) => {}
            UsdsMFutureUserDataCmdAny::AccountBalanceV2(_) => {}
            UsdsMFutureUserDataCmdAny::GetPositionMode(_) => {}
            UsdsMFutureUserDataCmdAny::ChangePositionMode(_) => {}
            UsdsMFutureUserDataCmdAny::GetMultiAssetsMode(_) => {}
            UsdsMFutureUserDataCmdAny::ChangeMultiAssetsMode(_) => {}
            UsdsMFutureUserDataCmdAny::CommissionRate(_) => {}
            UsdsMFutureUserDataCmdAny::LeverageBracket(_) => {}
            UsdsMFutureUserDataCmdAny::IncomeHistory(_) => {}
            UsdsMFutureUserDataCmdAny::AccountConfig(_) => {}
            UsdsMFutureUserDataCmdAny::TradingStatus(_) => {}
            UsdsMFutureUserDataCmdAny::UserTrades(_) => {}
            UsdsMFutureUserDataCmdAny::GetBNBBurnStatus(_) => {}
            UsdsMFutureUserDataCmdAny::SetBNBBurn(_) => {}
            UsdsMFutureUserDataCmdAny::FuturesTransfer(_) => {}
            UsdsMFutureUserDataCmdAny::TransferHistory(_) => {}
            UsdsMFutureUserDataCmdAny::GetOrderDownloadId(_) => {}
            UsdsMFutureUserDataCmdAny::GetOrderDownloadLink(_) => {}
            UsdsMFutureUserDataCmdAny::GetTradeDownloadId(_) => {}
            UsdsMFutureUserDataCmdAny::GetTradeDownloadLink(_) => {}
            UsdsMFutureUserDataCmdAny::GetIncomeDownloadId(_) => {}
            UsdsMFutureUserDataCmdAny::GetIncomeDownloadLink(_) => {}
            UsdsMFutureUserDataCmdAny::RateLimit(_) => {}
            UsdsMFutureUserDataCmdAny::SymbolConfig(_) => {}
        }
        todo!()
    }
}
