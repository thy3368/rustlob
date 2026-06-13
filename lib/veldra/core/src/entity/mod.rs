mod block;
mod product;
mod spot;
mod support;

pub use block::NewBlock;
pub use product::{
    PendingRequest, ProductContext, ProductPlugin, ProductPluginError, ProductPluginRegistry,
    RequestExecutionResult, build_new_block,
};
pub use spot::{
    SpotBalanceSnapshot, SpotMarketRules, SpotOrder, SpotOrderSide, SpotPlaceOrderPayload,
    SpotPlaceOrderResult, SpotProductContext, SpotProductPlugin,
};
pub(crate) use support::{
    FIELD_TYPE_BOOL, int_field, stable_hash_hex, stable_positive_i64, string_field,
    updated_int_field,
};
