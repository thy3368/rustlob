use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrderContractError {
    #[error("Unexpected `action.type` for order handler: `{0}`.")]
    UnexpectedActionType(String),
    #[error("`action.orders` must contain at least one order.")]
    EmptyOrders,
    #[error("Invalid `action.grouping`. Expected one of `na`, `normalTpsl`, `positionTpsl`.")]
    InvalidGrouping,
    #[error("Invalid `action.builder.b`. Expected a 42-character hexadecimal address.")]
    InvalidBuilderAddress,
    #[error("Invalid `action.orders[].c`. Expected a 128-bit hexadecimal cloid.")]
    InvalidCloid,
    #[error("Invalid `action.orders[].t`. Expected exactly one of `limit` or `trigger`.")]
    InvalidOrderType,
    #[error("Invalid `action.orders[].t.limit.tif`. Expected one of `Alo`, `Ioc`, `Gtc`.")]
    InvalidTimeInForce,
    #[error("Invalid `action.orders[].t.trigger.tpsl`. Expected `tp` or `sl`.")]
    InvalidTriggerKind,
    #[error("Invalid `action.orders[].p`. Expected a non-empty decimal string.")]
    InvalidPrice,
    #[error("Invalid `action.orders[].s`. Expected a non-empty decimal string.")]
    InvalidSize,
    #[error("Invalid `action.orders[].t.trigger.triggerPx`. Expected a non-empty decimal string.")]
    InvalidTriggerPrice,
}
