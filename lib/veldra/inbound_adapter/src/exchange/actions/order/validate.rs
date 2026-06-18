use crate::exchange::actions::order::error::OrderContractError;
use crate::exchange::actions::order::wire::OrderRequestWire;
use crate::exchange::common::validate::{
    validate_cloid, validate_common_fields, validate_hex_address,
};
use crate::exchange::error::ExchangeHttpError;

pub fn validate(request: &OrderRequestWire) -> Result<(), ExchangeHttpError> {
    if request.action.type_ != "order" {
        return Err(OrderContractError::UnexpectedActionType(request.action.type_.clone()).into());
    }
    validate_common_fields(
        request.common.nonce,
        request.common.expires_after,
        &request.common.signature.r,
        &request.common.signature.s,
        request.common.signature.v,
        request.common.vault_address.as_deref(),
    )
    .map_err(ExchangeHttpError::SharedFields)?;
    if request.action.orders.is_empty() {
        return Err(OrderContractError::EmptyOrders.into());
    }
    if !matches!(request.action.grouping.as_str(), "na" | "normalTpsl" | "positionTpsl") {
        return Err(OrderContractError::InvalidGrouping.into());
    }
    if let Some(builder) = &request.action.builder {
        validate_hex_address(&builder.b).map_err(|_| {
            ExchangeHttpError::OrderContract(OrderContractError::InvalidBuilderAddress)
        })?;
    }
    for order in &request.action.orders {
        if order.p.trim().is_empty() {
            return Err(OrderContractError::InvalidPrice.into());
        }
        if order.s.trim().is_empty() {
            return Err(OrderContractError::InvalidSize.into());
        }
        if let Some(cloid) = &order.c {
            validate_cloid(cloid)
                .map_err(|_| ExchangeHttpError::OrderContract(OrderContractError::InvalidCloid))?;
        }
        match (&order.t.limit, &order.t.trigger) {
            (Some(limit), None) => {
                if !matches!(limit.tif.as_str(), "Alo" | "Ioc" | "Gtc") {
                    return Err(OrderContractError::InvalidTimeInForce.into());
                }
            }
            (None, Some(trigger)) => {
                if trigger.trigger_px.trim().is_empty() {
                    return Err(OrderContractError::InvalidTriggerPrice.into());
                }
                if !matches!(trigger.tpsl.as_str(), "tp" | "sl") {
                    return Err(OrderContractError::InvalidTriggerKind.into());
                }
            }
            _ => return Err(OrderContractError::InvalidOrderType.into()),
        }
    }
    Ok(())
}
