pub mod kyle_service;
// pub mod kyle_lob_integration;  // TODO: 等待LOB库完善后启用

// 重新导出常用类型
pub use kyle_service::{KyleModelService, KyleParameters, KyleState, KyleTradeResult};
// pub use kyle_lob_integration::{KyleMarketMaker, KyleParameterEstimator,
// SmartOrderExecutor};
