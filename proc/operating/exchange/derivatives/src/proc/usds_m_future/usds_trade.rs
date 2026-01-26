use base_types::cqrs::cqrs_types::CmdResp;

use crate::proc::usds_m_future::behavior::trade_behavior::{
    UsdsMFutureTradeBehavior, UsdsMFutureTradeCmdAny, UsdsMFutureTradeCmdError, UsdsMFutureTradeRes
};

pub struct UsdsMFutureTradeBehaviorImpl {}

impl UsdsMFutureTradeBehavior for UsdsMFutureTradeBehaviorImpl {
    fn handle(
        &mut self, cmd: UsdsMFutureTradeCmdAny
    ) -> Result<CmdResp<UsdsMFutureTradeRes>, UsdsMFutureTradeCmdError> {
        match cmd {
            UsdsMFutureTradeCmdAny::NewOrder(_) => {}
            UsdsMFutureTradeCmdAny::NewOrderTest(_) => {}
            UsdsMFutureTradeCmdAny::PlaceMultipleOrders(_) => {}
            UsdsMFutureTradeCmdAny::QueryOrder(_) => {}
            UsdsMFutureTradeCmdAny::CancelOrder(_) => {}
            UsdsMFutureTradeCmdAny::ModifyOrder(_) => {}
            UsdsMFutureTradeCmdAny::CancelMultipleOrders(_) => {}
            UsdsMFutureTradeCmdAny::ModifyMultipleOrders(_) => {}
            UsdsMFutureTradeCmdAny::CancelAllOpenOrders(_) => {}
            UsdsMFutureTradeCmdAny::AutoCancelAllOpenOrders(_) => {}
            UsdsMFutureTradeCmdAny::CurrentAllOpenOrders(_) => {}
            UsdsMFutureTradeCmdAny::QueryCurrentOpenOrder(_) => {}
            UsdsMFutureTradeCmdAny::AllOrders(_) => {}
            UsdsMFutureTradeCmdAny::AccountTradeList(_) => {}
            UsdsMFutureTradeCmdAny::GetOrderModifyHistory(_) => {}
            UsdsMFutureTradeCmdAny::ChangeInitialLeverage(_) => {}
            UsdsMFutureTradeCmdAny::ChangeMarginType(_) => {}
            UsdsMFutureTradeCmdAny::ModifyIsolatedPositionMargin(_) => {}
            UsdsMFutureTradeCmdAny::GetPositionMarginChangeHistory(_) => {}
            UsdsMFutureTradeCmdAny::PositionInformationV2(_) => {}
            UsdsMFutureTradeCmdAny::PositionInformationV3(_) => {}
            UsdsMFutureTradeCmdAny::UsersForceOrders(_) => {}
            UsdsMFutureTradeCmdAny::PositionADLQuantileEstimation(_) => {}
            UsdsMFutureTradeCmdAny::ChangePositionMode(_) => {}
            UsdsMFutureTradeCmdAny::ChangeMultiAssetsMode(_) => {}
        }
        todo!()
    }
}
