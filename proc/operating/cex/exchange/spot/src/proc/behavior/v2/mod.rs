pub mod spot_market_data_behavior;
pub mod spot_market_data_sse_behavior;
pub mod spot_trade_behavior_v2;
pub mod spot_user_data_behavior;
pub mod spot_user_data_sse_behavior;

pub mod spot_behavior;

// SOA (Structure of Arrays) 优化

// SOA 转换示例

// 基础类型版本（便于 SIMD 优化）
pub mod new_order_cmd_base;

// 订单到变更日志的转换算子

