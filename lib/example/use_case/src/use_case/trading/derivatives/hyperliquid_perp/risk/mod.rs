//! `risk` 组围绕 `HyperliquidPerpLiquidation` 这一仓位级强平处置会话展开，
//! 不是“风险查询工具箱”，也不是“条件单变体”。
//!
//! - `business_truth_center`: `HyperliquidPerpLiquidation`
//! - `four_color_archetype`: `Moment-Interval`
//! - `meaning`: 某个 perp 仓位被系统正式认定进入强平流程，并沿着缺口、保险基金、ADL 链路推进直到结束
//! - `lifecycle_or_state_machine`:
//!   `Started -> Executing -> ShortfallAssessed -> FundCovering | AdlCovering -> Closed | Exhausted`
//! - `group_boundary`: 当前示例覆盖主强平会话与次级 MI 事实，不负责纯风险查询、成交清算、逐仓保证金调整或余额出入账
//!
//! 这个分组最重要的主语不是订单，而是一次强平处置会话。
//! 强平在触发方式上看起来像条件单，但业务主语并不是“某张订单被触发”，而是
//! “某个仓位进入并推进一次风险处置流程”。
//!
//! 同时要区分 `HyperliquidPerpLiquidation` 与链路中的次级事实：
//! `HyperliquidPerpLiquidationFill`、`HyperliquidPerpShortfall`、
//! `HyperliquidPerpInsuranceFundAllocation`、`HyperliquidPerpAdlBatch`、
//! `HyperliquidPerpAdlExecution`、`HyperliquidPerpAdlDeleveragingRecord`
//! 都是围绕主强平会话的独立审计事实，
//! 但不会替代 `HyperliquidPerpLiquidation` 作为顶层生命周期主语。
//!
//! 从业务闭环看，这组 mutation use case 应收敛为：
//!
//! - `StartHyperliquidPerpLiquidation`：创建强平会话事实
//! - `ApplyHyperliquidPerpLiquidationFill`：记录强平成交并推进会话
//! - `ConfirmHyperliquidPerpShortfall`：确认是否存在缺口
//! - `AllocateHyperliquidPerpInsuranceFund`：记录保险基金覆盖
//! - `StartHyperliquidPerpAdlBatch`：启动 ADL 覆盖批次
//! - `StartHyperliquidPerpAdlExecution`：启动一次 ADL 执行尝试
//! - `CompleteHyperliquidPerpAdlExecution`：完成执行并落结果凭证，或把 ADL 路径推进到穷尽
//! - `CloseHyperliquidPerpLiquidation`：把强平会话显式推进到 `Closed | Exhausted`
//!
//! 当前这组 mutation 已统一按 `CommandUseCase4` 建模：
//! `Changes` 是业务第一真相，事件只从 `Changes` 投影。
pub mod allocate_insurance_fund;
pub mod apply_liquidation_fill;
pub mod close_liquidation;
pub mod complete_adl_execution;
pub mod confirm_shortfall;
pub mod start_adl_batch;
pub mod start_adl_execution;
pub mod start_liquidation;

pub use allocate_insurance_fund::{
    AllocateHyperliquidPerpInsuranceFundChanges, AllocateHyperliquidPerpInsuranceFundCmd,
    AllocateHyperliquidPerpInsuranceFundError, AllocateHyperliquidPerpInsuranceFundState,
    AllocateHyperliquidPerpInsuranceFundUseCase,
};
pub use apply_liquidation_fill::{
    ApplyHyperliquidPerpLiquidationFillChanges, ApplyHyperliquidPerpLiquidationFillCmd,
    ApplyHyperliquidPerpLiquidationFillError, ApplyHyperliquidPerpLiquidationFillState,
    ApplyHyperliquidPerpLiquidationFillUseCase,
};
pub use close_liquidation::{
    CloseHyperliquidPerpLiquidationChanges, CloseHyperliquidPerpLiquidationCmd,
    CloseHyperliquidPerpLiquidationError, CloseHyperliquidPerpLiquidationState,
    CloseHyperliquidPerpLiquidationUseCase, HyperliquidPerpLiquidationCloseAs,
};
pub use complete_adl_execution::{
    CompleteHyperliquidPerpAdlExecutionChanges, CompleteHyperliquidPerpAdlExecutionCmd,
    CompleteHyperliquidPerpAdlExecutionError, CompleteHyperliquidPerpAdlExecutionState,
    CompleteHyperliquidPerpAdlExecutionUseCase,
};
pub use confirm_shortfall::{
    ConfirmHyperliquidPerpShortfallChanges, ConfirmHyperliquidPerpShortfallCmd,
    ConfirmHyperliquidPerpShortfallError, ConfirmHyperliquidPerpShortfallState,
    ConfirmHyperliquidPerpShortfallUseCase,
};
pub use start_adl_batch::{
    StartHyperliquidPerpAdlBatchChanges, StartHyperliquidPerpAdlBatchCmd,
    StartHyperliquidPerpAdlBatchError, StartHyperliquidPerpAdlBatchState,
    StartHyperliquidPerpAdlBatchUseCase,
};
pub use start_adl_execution::{
    StartHyperliquidPerpAdlExecutionChanges, StartHyperliquidPerpAdlExecutionCmd,
    StartHyperliquidPerpAdlExecutionError, StartHyperliquidPerpAdlExecutionState,
    StartHyperliquidPerpAdlExecutionUseCase,
};
pub use start_liquidation::{
    StartHyperliquidPerpLiquidationChanges, StartHyperliquidPerpLiquidationCmd,
    StartHyperliquidPerpLiquidationError, StartHyperliquidPerpLiquidationState,
    StartHyperliquidPerpLiquidationUseCase,
};
