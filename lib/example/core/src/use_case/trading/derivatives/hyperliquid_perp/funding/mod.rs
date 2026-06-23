//! Hyperliquid perp `funding` use case 组定义。
//!
//! ## Use Case Group
//! - `business_truth_center`: `HyperliquidPerpFundingSettlement`
//! - `four_color_archetype`: `Moment-Interval`
//! - `lifecycle_or_state_machine`: `unsettled batch scope -> settled funding facts recorded`
//! - `group_boundary`: 围绕“资金费结算事实的生成与记账”展开，负责把一批符合条件的
//!   `HyperliquidPerpPosition` 在给定 `funding_rate_e8 + oracle_price + margin_mode`
//!   下转换为可审计的 `HyperliquidPerpFundingSettlement`，并同步产出受影响账户
//!   `Balance` 的合法变化；不负责订单提交、撮合、成交结算、强平处置、查询投影。
//! - `use_cases`: `SettleHyperliquidPerpFunding`
//!
//! ## Boundary Decisions
//! - `HyperliquidPerpPosition` 是输入对象，不是本组中心；本次动作真正新建并被系统记住
//!   的业务真相是 `HyperliquidPerpFundingSettlement`。
//! - `Balance` 承载资金费结算的副作用，是相邻被推进的实体，不是本组主语。
//! - `Trade` / `Order` / `Liquidation` 分别属于执行、风险或订单生命周期边界，即使影响
//!   funding 前置状态，也不属于本组。
//!
//! ## Use Case
//! - `SettleHyperliquidPerpFunding`
//!   - `command`: 对一个 funding batch 发起资金费结算，请求中指定
//!     `funding_batch_id / funding_time / asset / symbol / oracle_price /
//!     funding_rate_e8 / position_ids`
//!   - `given_state`: 本批次涉及的 `positions`、对应保证金币种 `margin_balances`、
//!     `margin_asset_id`、已结算去重集合 `settled_position_ids`、`margin_mode`
//!   - `changes`: 新建 `HyperliquidPerpFundingSettlement` 列表，并产出受影响
//!     `Balance` 的 `UpdatedEntityPair`
//!   - `entity`: `HyperliquidPerpFundingSettlement`、`HyperliquidPerpPosition`、`Balance`
//!
//! ## Candidate Future Use Cases
//! 只有当业务语义已经形成独立真相时，才应该新增：
//! - `SettleHyperliquidPerpFundingForIsolatedMargin`
//! - `ReverseHyperliquidPerpFundingSettlement`
//! - `FinalizeHyperliquidPerpFundingBatch`
//!
//! 当前不要为了流程拆分而提前新增这些 use case。
pub mod settle_perp_funding;

pub use settle_perp_funding::{
    SettleHyperliquidPerpFundingChanges, SettleHyperliquidPerpFundingCmd,
    SettleHyperliquidPerpFundingError, SettleHyperliquidPerpFundingState,
    SettleHyperliquidPerpFundingUseCase,
};
