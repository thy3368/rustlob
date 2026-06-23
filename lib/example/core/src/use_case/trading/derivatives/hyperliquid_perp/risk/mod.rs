//! `risk` 组围绕 `HyperliquidPerpLiquidation` 这一仓位级强平处置会话展开，
//! 不是“风险查询工具箱”，也不是“条件单变体”。
//!
//! - `business_truth_center`: `HyperliquidPerpLiquidation`
//! - `four_color_archetype`: `Moment-Interval`
//! - `meaning`: 某个 perp 仓位被系统正式认定进入强平流程，并在该流程中被推进直到结束或升级的业务事实
//! - `lifecycle_or_state_machine`: `Started -> OrderPlaced -> Resolved | Escalated`
//! - `group_boundary`: 只负责强平会话事实的创建与推进，不负责纯风险查询、成交清算、资金费、ADL 执行、逐仓保证金调整
//!
//! 这个分组最重要的主语不是订单，而是一次强平处置会话。
//! 强平在触发方式上看起来像条件单，但业务主语并不是“某张订单被触发”，而是
//! “某个仓位进入并推进一次风险处置流程”。
//!
//! 同时要区分 `HyperliquidPerpLiquidation` 与强平过程中发出的订单：
//! 强平单本身仍然是 `HyperliquidPerpOrder`，不应新增
//! `HyperliquidPerpLiquidationOrder` 这类专属 entity；
//! `HyperliquidPerpLiquidation` 只记录为什么、何时、以及如何推进这次处置。
//! `HyperliquidPerpPosition` 则主要是输入与校验对象，不是本组的中心主语。
//!
//! 从业务闭环看，这组 mutation use case 应收敛为：
//!
//! - `StartHyperliquidPerpLiquidation`：创建强平会话事实
//! - `PlaceHyperliquidPerpLiquidationOrder`：为强平会话发出订单并推进会话
//! - `ResolveHyperliquidPerpLiquidation`：正常关闭强平会话
//! - `EscalateHyperliquidPerpLiquidation`：升级到更高风险处置路径
//!
//! 当前这四个 mutation 已统一按 `CommandUseCase4` 建模：
//! `Changes` 是业务第一真相，事件只从 `Changes` 投影。
//!
//! 与风险查询也要明确分组边界：爆仓候选扫描只返回 view，
//! 不创建也不推进 `HyperliquidPerpLiquidation`，因此它本质上应归 `query`。
//! 当前目录中的 `scan_liquidation_candidates` 更适合视为相邻查询能力，
//! 而不是这个 `risk` mutation 组的中心 use case。
//!
//! 审查这一组时，应重点确认：
//!
//! - `risk` 目录中是否混入了 query-only use case
//! - 是否把“条件触发”误建模成订单主语
//! - 是否错误新增了“强平单专属 entity”
//! - `HyperliquidPerpLiquidation` 的每个状态是否都有对应的 mutation use case
pub mod escalate_liquidation;
pub mod place_liquidation_order;
pub mod resolve_liquidation;
pub mod scan_liquidation_candidates;
pub mod start_liquidation;

pub use escalate_liquidation::{
    EscalateHyperliquidPerpLiquidationChanges, EscalateHyperliquidPerpLiquidationCmd,
    EscalateHyperliquidPerpLiquidationError, EscalateHyperliquidPerpLiquidationState,
    EscalateHyperliquidPerpLiquidationUseCase,
};
pub use place_liquidation_order::{
    PlaceHyperliquidPerpLiquidationOrderChanges, PlaceHyperliquidPerpLiquidationOrderCmd,
    PlaceHyperliquidPerpLiquidationOrderError, PlaceHyperliquidPerpLiquidationOrderState,
    PlaceHyperliquidPerpLiquidationOrderUseCase,
};
pub use resolve_liquidation::{
    ResolveHyperliquidPerpLiquidationChanges, ResolveHyperliquidPerpLiquidationCmd,
    ResolveHyperliquidPerpLiquidationError, ResolveHyperliquidPerpLiquidationState,
    ResolveHyperliquidPerpLiquidationUseCase,
};
pub use scan_liquidation_candidates::{
    HyperliquidPerpLiquidationCandidate, HyperliquidPerpRiskSnapshot,
    QueryHyperliquidPerpLiquidationCandidates, QueryHyperliquidPerpLiquidationCandidatesError,
    QueryHyperliquidPerpLiquidationCandidatesReadModel,
    QueryHyperliquidPerpLiquidationCandidatesUseCase,
};
pub use start_liquidation::{
    StartHyperliquidPerpLiquidationChanges, StartHyperliquidPerpLiquidationCmd,
    StartHyperliquidPerpLiquidationError, StartHyperliquidPerpLiquidationState,
    StartHyperliquidPerpLiquidationUseCase,
};
