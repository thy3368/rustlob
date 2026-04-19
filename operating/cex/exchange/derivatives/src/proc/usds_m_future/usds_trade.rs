use base_types::Timestamp;
use base_types::cqrs::cqrs_types::{CmdResp, ResMetadata};
use cmd_handler::CmdHandlerForUpdate3;
use db_repo::adapter::v2::memdb_repo::MemdbRepo;
use db_repo::core::event_publish::{EventPublisher2, PublishError};
use serde::Serialize;

use crate::proc::usds_m_future::behavior::trade_behavior::{
    CountdownResponse, LeverageResponse, MarginResponse, MarginTypeResponse, OrderResponse,
    SuccessResponse, UsdsMFutureTradeBehavior, UsdsMFutureTradeCmd, UsdsMFutureTradeCmdError,
    UsdsMFutureTradeCmdOrQuery, UsdsMFutureTradeQuery, UsdsMFutureTradeRes,
};
use crate::proc::usds_m_future::handler::trade_handler::{
    AutoCancelAllOpenOrdersCmdHandler, CancelAllOpenOrdersCmdHandler,
    CancelMultipleOrdersCmdHandler, CancelOrderCmdHandler, ChangeInitialLeverageCmdHandler,
    ChangeMarginTypeCmdHandler, ChangeMultiAssetsModeCmdHandler, ChangePositionModeCmdHandler,
    ModifyIsolatedPositionMarginCmdHandler, ModifyMultipleOrdersCmdHandler,
    ModifyOrderCmdHandler, NewOrderCmdHandler, NewOrderTestCmdHandler,
    PlaceMultipleOrdersCmdHandler,
};

#[derive(Clone, Copy, Debug, Default)]
struct NoopEventPublisher;

impl EventPublisher2 for NoopEventPublisher {
    fn publish<E: Serialize>(
        &self,
        _event: &diff::diff_types::DomainEvent<E>,
    ) -> Result<(), PublishError> {
        Ok(())
    }

    fn publish_batch<E: Serialize>(
        &self,
        _events: &[diff::diff_types::DomainEvent<E>],
    ) -> Result<(), PublishError> {
        Ok(())
    }
}

pub struct UsdsMFutureTradeBehaviorImpl {}

impl UsdsMFutureTradeBehaviorImpl {
    fn response(result: UsdsMFutureTradeRes) -> CmdResp<UsdsMFutureTradeRes> {
        CmdResp::new(ResMetadata::new(0, false, Timestamp(0)), result)
    }

    fn todo_order_response() -> OrderResponse {
        OrderResponse {
            order_id: 0,
            symbol: String::new(),
            status: crate::proc::usds_m_future::behavior::trade_behavior::OrderStatus::NEW,
            client_order_id: String::new(),
            price: String::new(),
            avg_price: String::new(),
            orig_qty: String::new(),
            executed_qty: String::new(),
            cum_qty: String::new(),
            cum_quote: String::new(),
            time_in_force: base_types::exchange::spot::spot_types::TimeInForce::GTC,
            order_type: crate::proc::usds_m_future::behavior::trade_behavior::OrderType::LIMIT,
            reduce_only: false,
            close_position: false,
            side: base_types::OrderSide::Buy,
            position_side: crate::proc::usds_m_future::behavior::trade_behavior::PositionSide::BOTH,
            stop_price: None,
            working_type: crate::proc::usds_m_future::behavior::trade_behavior::WorkingType::MARK_PRICE,
            price_protect: false,
            orig_type: crate::proc::usds_m_future::behavior::trade_behavior::OrderType::LIMIT,
            price_match: None,
            self_trade_prevention_mode: None,
            good_till_date: 0,
            update_time: 0,
            activate_price: None,
            price_rate: None,
        }
    }

    fn todo_success_response() -> SuccessResponse {
        SuccessResponse { code: 0, msg: "todo".to_string() }
    }
}

impl UsdsMFutureTradeBehavior for UsdsMFutureTradeBehaviorImpl {
    fn handle(
        &mut self,
        cmd: UsdsMFutureTradeCmdOrQuery,
    ) -> Result<CmdResp<UsdsMFutureTradeRes>, UsdsMFutureTradeCmdError> {
        let repo = MemdbRepo::default();
        let publisher = NoopEventPublisher;

        match cmd {
            UsdsMFutureTradeCmdOrQuery::Cmd(cmd) => match cmd {
                UsdsMFutureTradeCmd::NewOrder(cmd) => {
                    NewOrderCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Order(Self::todo_order_response())))
                }
                UsdsMFutureTradeCmd::NewOrderTest(cmd) => {
                    NewOrderTestCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Order(Self::todo_order_response())))
                }
                UsdsMFutureTradeCmd::PlaceMultipleOrders(cmd) => {
                    PlaceMultipleOrdersCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::BatchOrders(vec![])))
                }
                UsdsMFutureTradeCmd::CancelOrder(cmd) => {
                    CancelOrderCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Order(Self::todo_order_response())))
                }
                UsdsMFutureTradeCmd::ModifyOrder(cmd) => {
                    ModifyOrderCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Order(Self::todo_order_response())))
                }
                UsdsMFutureTradeCmd::CancelMultipleOrders(cmd) => {
                    CancelMultipleOrdersCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::BatchOrders(vec![])))
                }
                UsdsMFutureTradeCmd::ModifyMultipleOrders(cmd) => {
                    ModifyMultipleOrdersCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::BatchOrders(vec![])))
                }
                UsdsMFutureTradeCmd::CancelAllOpenOrders(cmd) => {
                    CancelAllOpenOrdersCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Success(Self::todo_success_response())))
                }
                UsdsMFutureTradeCmd::AutoCancelAllOpenOrders(cmd) => {
                    AutoCancelAllOpenOrdersCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::CountdownResponse(CountdownResponse {
                        symbol: String::new(),
                        countdown_time: 0,
                    })))
                }
                UsdsMFutureTradeCmd::ChangeInitialLeverage(cmd) => {
                    ChangeInitialLeverageCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Leverage(LeverageResponse {
                        leverage: 0,
                        max_notional_value: String::new(),
                        symbol: String::new(),
                    })))
                }
                UsdsMFutureTradeCmd::ChangeMarginType(cmd) => {
                    ChangeMarginTypeCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::MarginType(MarginTypeResponse {
                        code: 0,
                        msg: "todo".to_string(),
                    })))
                }
                UsdsMFutureTradeCmd::ModifyIsolatedPositionMargin(cmd) => {
                    ModifyIsolatedPositionMarginCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Margin(MarginResponse {
                        amount: String::new(),
                        code: 0,
                        msg: "todo".to_string(),
                        margin_type: 0,
                    })))
                }
                UsdsMFutureTradeCmd::ChangePositionMode(cmd) => {
                    ChangePositionModeCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Success(Self::todo_success_response())))
                }
                UsdsMFutureTradeCmd::ChangeMultiAssetsMode(cmd) => {
                    ChangeMultiAssetsModeCmdHandler::new(repo.clone(), publisher)
                        .cmd_handle(cmd, repo.clone(), publisher)?;
                    Ok(Self::response(UsdsMFutureTradeRes::Success(Self::todo_success_response())))
                }
            },
            UsdsMFutureTradeCmdOrQuery::Query(query) => match query {
                UsdsMFutureTradeQuery::QueryOrder(_) => todo!(),
                UsdsMFutureTradeQuery::CurrentAllOpenOrders(_) => todo!(),
                UsdsMFutureTradeQuery::QueryCurrentOpenOrder(_) => todo!(),
                UsdsMFutureTradeQuery::AllOrders(_) => todo!(),
                UsdsMFutureTradeQuery::AccountTradeList(_) => todo!(),
                UsdsMFutureTradeQuery::GetOrderModifyHistory(_) => todo!(),
                UsdsMFutureTradeQuery::GetPositionMarginChangeHistory(_) => todo!(),
                UsdsMFutureTradeQuery::PositionInformationV2(_) => todo!(),
                UsdsMFutureTradeQuery::PositionInformationV3(_) => todo!(),
                UsdsMFutureTradeQuery::UsersForceOrders(_) => todo!(),
                UsdsMFutureTradeQuery::PositionADLQuantileEstimation(_) => todo!(),
            },
        }
    }
}
