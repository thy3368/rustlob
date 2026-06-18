pub mod approve_agent;
pub mod cancel;
pub mod noop;
pub mod order;
pub mod twap_cancel;
pub mod twap_order;
pub mod update_isolated_margin;
pub mod update_leverage;
pub mod user_set_abstraction;

use crate::exchange::error::ExchangeHttpError;

#[derive(Debug, Default, Clone, Copy)]
pub struct ExchangeActionDeps;

pub enum ExchangeActionReply {
    ApproveAgent(approve_agent::reply::ApproveAgentResponseWire),
    Cancel(cancel::reply::CancelResponseWire),
    Noop(noop::reply::NoopResponseWire),
    Order(order::reply::OrderResponseWire),
    TwapCancel(twap_cancel::reply::TwapCancelResponseWire),
    TwapOrder(twap_order::reply::TwapOrderResponseWire),
    UpdateIsolatedMargin(update_isolated_margin::reply::UpdateIsolatedMarginResponseWire),
    UpdateLeverage(update_leverage::reply::UpdateLeverageResponseWire),
    UserSetAbstraction(user_set_abstraction::reply::UserSetAbstractionResponseWire),
}

pub async fn dispatch_exchange_action(
    action_type: &str,
    body: &[u8],
    deps: &ExchangeActionDeps,
) -> Result<ExchangeActionReply, ExchangeHttpError> {
    match action_type {
        "approveAgent" => {
            approve_agent::handle(body, deps).await.map(ExchangeActionReply::ApproveAgent)
        }
        "cancel" => cancel::handle(body, deps).await.map(ExchangeActionReply::Cancel),
        "noop" => noop::handle(body, deps).await.map(ExchangeActionReply::Noop),
        "order" => order::handle(body, deps).await.map(ExchangeActionReply::Order),
        "twapCancel" => twap_cancel::handle(body, deps).await.map(ExchangeActionReply::TwapCancel),
        "twapOrder" => twap_order::handle(body, deps).await.map(ExchangeActionReply::TwapOrder),
        "updateIsolatedMargin" => update_isolated_margin::handle(body, deps)
            .await
            .map(ExchangeActionReply::UpdateIsolatedMargin),
        "updateLeverage" => {
            update_leverage::handle(body, deps).await.map(ExchangeActionReply::UpdateLeverage)
        }
        "userSetAbstraction" => user_set_abstraction::handle(body, deps)
            .await
            .map(ExchangeActionReply::UserSetAbstraction),
        other => Err(ExchangeHttpError::UnsupportedActionType(other.to_string())),
    }
}
