//! CmdHandler 职责规范模板。
//!
//! 这个文件不是具体业务实现，而是 CmdHandler 的职责约束与实现模板。
//! 适用于 CQRS 写侧，用来统一 command -> state set -> changeset -> changelog persistence -> replay -> publish 的处理链路。
//!
//! ## 1. 定位
//! - CmdHandler 属于写侧（write model）入口。
//! - 负责把命令转成业务状态变更与后续变更输出。
//! - 负责协调取数、校验、状态更新并产出 changelog、持久化 changelog、回放 changelog 更新 state、发消息。
//! - 不负责 QueryHandler/read model 的查询拼装。
//!
//! ## 2. 标准处理链路
//! 典型处理顺序：
//! 1. 接收命令与 metadata
//! 2. 读取参与计算的 state set
//! 3. 做参数、权限、幂等、时效校验
//! 4. 执行业务规则并完成状态更新，同时产出 changelog / domain event
//! 5. 持久化 changelog
//! 6. 回放 changelog 更新 state
//! 7. 发布消息、事件或下游通知
//! 8. 返回结果与错误
//!
//! ## 3. 输入约束
//! CmdHandler 的输入建议至少包含以下语义：
//! - command payload: 表达业务意图
//! - user_id / actor: 标识命令发起方
//! - nonce / command_id: 用于幂等判断
//! - timestamp_ms / recv_window: 用于时效性控制
//! - aggregate_id / reference_id: 用于定位主实体或聚合根
//!
//! 如果命令对象本身能提供 account_id、asset_id、reference_id 等辅助定位能力，
//! 应优先在命令模型中封装，而不是在 handler 中散落解析逻辑。
//!
//! ## 4. 前置校验职责
//! CmdHandler 应明确以下前置校验是否由自己负责：
//! - 参数合法性校验
//! - 权限校验
//! - 幂等校验
//! - 时间窗口 / 过期校验
//! - 聚合边界校验
//!
//! 校验失败时，应明确：
//! - 是否直接返回错误
//! - 是否记录 rejected changelog
//! - 是否需要保留审计轨迹
//!
//! ## 5. 状态读取职责
//! CmdHandler 应明确：
//! - 需要读取哪些 state set
//! - 是否依赖 snapshot / current state / changelog replay
//! - 缺失状态如何处理
//! - 并发更新时的一致性假设是什么
//!
//! 取数只读取本次命令真正需要的数据，避免无关查询污染热路径。
//!
//! ### 5.0 锁策略三分法
//! - Create = no lock
//! - Single-thread update = no lock
//! - Concurrent update = lock
//!
//! ### 5.1 create / update / single-thread 的取数规则
//! - create new state set: 一般不需要加锁
//! - update existing state set: 需要按更新语义加锁读取
//! - single-thread state set processing: 不需要加锁
//!
//! 原则：
//! - 新增场景通常没有“基于旧状态并发修改”的问题，因此默认无锁
//! - 更新 existing state set 需要基于当前状态做判断和修改，因此默认加锁
//! - 单线程处理场景下，不存在并发竞争，因此也不需要加锁
//!
//! ### 5.2 update 场景的一锁二判三更新
//! update state set 推荐遵循：
//! - 零预判：加锁前可先做一次快速预检查，尽早失败，减少锁持有时间
//! - 一锁：先锁定参与本次写入的关键 state set
//! - 二判：在锁内重新判断状态、版本、余额、约束是否仍然成立
//! - 三更新：完成状态更新并同步产出 changelog，随后持久化与回放
//!
//! 锁外判断只能做快速预检查，真正影响写入正确性的条件应在锁内再次判断。
//!
//! ## 6. 业务执行职责
//! CmdHandler 负责协调业务执行，但不要把所有领域逻辑硬编码在 handler 内。
//! 更推荐：
//! - handler 做流程编排
//! - entity / domain object 承载核心业务规则与状态迁移
//! - repository / publisher 通过抽象接口注入
//!
//! 对于撮合、清算、批量成交、批量更新等场景，不应假设“一次命令只依赖一个状态、只影响一个实体、只产出一个 changelog”。
//! 更合理的抽象是：一次命令读取一个 state set，计算后产出一个 changeset，其中包含多个待写入变更和多个 changelog。
//!
//! CmdHandler 需要定义清楚：
//! - 成功语义
//! - 拒绝语义
//! - 是否允许部分成功
//! - 状态迁移是否必须可回放
//!
//! ## 7. ChangeLog / Event 职责
//! CmdHandler 应明确本次命令会产生什么变更输出：
//! - create / update / delete 哪种 changelog
//! - 是否产生一个或多个 changelog
//! - event 字段最小集合
//! - 因果顺序和先后关系
//!
//! changelog/event 必须来源于真实的状态变更，不能脱离状态变化单独构造。
//!
//! ## 8. 持久化与回放职责
//! CmdHandler 应明确：
//! - 是否先持久化 changelog，再回放更新 state
//! - state 是否完全由 changelog replay 驱动
//! - 回放是否在当前命令内同步完成
//! - 回放失败时如何处理重试与补偿
//! - 持久化边界是否要求原子性
//!
//! 如果系统采用 event sourcing / changelog 驱动设计，
//! 则推荐将“落 changelog”和“回放更新 state”明确拆成两个阶段。
//!
//! ## 9. 发布职责
//! CmdHandler 应明确：
//! - 发布什么消息
//! - 发布到哪里
//! - 是同步发布、异步发布还是批量发布
//! - 发布失败是否影响主事务结果
//! - 是否需要 outbox / retry / 补偿机制
//!
//! 默认建议先明确 changelog 持久化与回放边界，再定义 publish 行为，避免副作用顺序不清。
//!
//! ## 10. 返回值职责
//! 返回值应明确表达：
//! - 业务执行结果
//! - 是否 duplicate
//! - 命令处理元信息
//! - 错误分类
//!
//! 如果系统采用统一命令响应结构，应优先与 CmdResp / ResMetadata 语义保持一致。
//!
//! ## 11. 非职责边界
//! 以下内容不应混入 CmdHandler 热路径：
//! - read model 拼装
//! - QueryHandler 职责
//! - HTTP / gRPC / DTO 转换
//! - UI 展示逻辑
//! - 与本命令无关的额外查询
//!
//! ## 12. 低时延约束
//! 针对本项目低时延要求，CmdHandler 应遵守：
//! - 热路径避免不必要分配
//! - 避免重复取数与重复序列化
//! - 尽量在锁外完成快速预检查，缩短锁持有时间
//! - 避免把读侧拼装逻辑塞进写侧
//! - 明确 changelog persist / replay / publish 顺序，减少不确定性
//! - 尽量让失败语义、重试语义、幂等语义可预测
//! - 仅保留本次命令真正必要的步骤
//!
//! ## 13. Clean Architecture 边界
//! 从整洁架构角度，CmdHandler 应满足：
//! - 作为应用层/用例层编排者
//! - 依赖抽象接口，不直接耦合具体基础设施实现
//! - 不把框架细节、协议细节带入领域规则
//! - 让领域对象承担可复用业务规则
//!
//! ## 14. 实现自检清单
//! 在落地具体 CmdHandler 前，至少确认：
//! - [ ] 输入 metadata 是否完整
//! - [ ] 幂等语义是否明确
//! - [ ] create 路径是否避免了不必要加锁
//! - [ ] single-thread 路径是否明确为无锁处理
//! - [ ] update 路径是否采用“零预判 + 一锁二判三更新”
//! - [ ] 锁外检查是否只做快速预判，不承载最终正确性
//! - [ ] 是否支持多个状态参与计算
//! - [ ] 是否支持一次命令影响多个实体
//! - [ ] 是否支持一次命令产出多个 changelog
//! - [ ] 是否明确拆分了 persist changelog 与 replay changelog to state
//! - [ ] changelog/event 是否来自真实状态变更
//! - [ ] replay 顺序是否明确
//! - [ ] publish 顺序是否明确
//! - [ ] 错误语义与重试语义是否明确
//! - [ ] 没有混入 QueryHandler/read model 逻辑
//! - [ ] 热路径没有无意义分配或冗余步骤

/// CmdHandler 职责规范占位类型。
pub struct CmdTemplateSpec;

/// 一次命令执行产生的变更集合。
///
/// 适用于撮合、清算、批量更新等“一次命令依赖多个状态、影响多个实体、产出多个 changelog”的场景。
pub struct ChangeSet<W, L> {
    pub writes: W,
    pub changelogs: Vec<L>,
}

/// 撮合场景的示例状态集合。
///
/// 一次撮合命令在计算前，可能需要同时读取：
/// - taker order
/// - maker orders
/// - balances
/// - positions
/// - orderbook snapshot / level view
pub struct MatchStateSet<O, B, P, OB> {
    pub taker_order: O,
    pub maker_orders: Vec<O>,
    pub balances: Vec<B>,
    pub positions: Vec<P>,
    pub orderbook: OB,
}

/// 撮合场景的示例写集结构。
///
/// 一次撮合命令可能同时影响：
/// - taker / maker orders
/// - trades
/// - balances
/// - positions
pub struct MatchWrites<O, T, B, P> {
    pub orders: Vec<O>,
    pub trades: Vec<T>,
    pub balances: Vec<B>,
    pub positions: Vec<P>,
}

/// 示例依赖：create 场景，构建初始 state set，一般不需要加锁。
pub trait BuildInitialStateSet<C, S, E> {
    fn build_initial_state_set(&self, cmd: &C) -> Result<S, E>;
}

/// 示例依赖：update 场景，锁外快速预检查。
pub trait PreCheckCommand<C, E> {
    fn pre_check_command(&self, cmd: &C) -> Result<(), E>;
}

/// 示例依赖：update 场景，加锁读取参与更新的 state set。
pub trait LoadStateSetForUpdate<C, S, E> {
    fn load_state_set_for_update(&self, cmd: &C) -> Result<S, E>;
}

/// 示例依赖：single-thread 场景，无锁读取 state set。
pub trait LoadStateSetInSingleThread<C, S, E> {
    fn load_state_set_in_single_thread(&self, cmd: &C) -> Result<S, E>;
}

/// 示例依赖：create 或通用场景的命令校验。
pub trait ValidateCommand<C, S, E> {
    fn validate_command(&self, cmd: &C, state_set: &S) -> Result<(), E>;
}

/// 示例依赖：update 场景的锁内二判。
pub trait ValidateCommandInLock<C, S, E> {
    fn validate_command_in_lock(&self, cmd: &C, state_set: &S) -> Result<(), E>;
}

/// 示例依赖：将业务状态更新与 changelog 收集合并为一个步骤。
///
/// 注意：这里输入的是 state set，返回的是 ChangeSet，
/// 以支持一次命令依赖多个状态并产出多实体写入、多条 changelog 的情况。
///
/// 例如撮合场景里：
/// - S 可以是 `MatchStateSet<OrderState, BalanceState, PositionState, OrderBookView>`
/// - W 可以是 `MatchWrites<OrderWrite, TradeWrite, BalanceWrite, PositionWrite>`。
pub trait ApplyCommandAndCollectChanges<C, S, W, L, E> {
    fn apply_command_and_collect_changes(
        &self,
        cmd: &C,
        state_set: S,
    ) -> Result<ChangeSet<W, L>, E>;
}

/// 示例依赖：先持久化 changelog。
pub trait PersistChangeLogs<L, E> {
    fn persist_changelogs(&self, changelogs: &[L]) -> Result<(), E>;
}

/// 示例依赖：通过回放 changelog 更新 state。
pub trait ReplayChangeLogsToState<L, E> {
    fn replay_changelogs_to_state(&self, changelogs: &[L]) -> Result<(), E>;
}

/// 示例依赖：发布 changelog / event / message。
pub trait PublishChangeLog<L, E> {
    fn publish_changelog(&self, changelogs: &[L]) -> Result<(), E>;
}

/// create new state set 的模板示例。
///
/// 主链路：build initial state set -> validate -> apply and collect changes -> persist changelog -> replay state -> publish -> return result。
pub struct CreateStateSetCmdTemplate<Builder, Validator, Applier, LogPersister, Replayer, Publisher> {
    pub builder: Builder,
    pub validator: Validator,
    pub applier: Applier,
    pub log_persister: LogPersister,
    pub replayer: Replayer,
    pub publisher: Publisher,
}

impl<Builder, Validator, Applier, LogPersister, Replayer, Publisher>
    CreateStateSetCmdTemplate<Builder, Validator, Applier, LogPersister, Replayer, Publisher>
{
    pub fn handle<C, S, W, L, R, E, F>(&self, cmd: C, result_mapper: F) -> Result<R, E>
    where
        Builder: BuildInitialStateSet<C, S, E>,
        Validator: ValidateCommand<C, S, E>,
        Applier: ApplyCommandAndCollectChanges<C, S, W, L, E>,
        LogPersister: PersistChangeLogs<L, E>,
        Replayer: ReplayChangeLogsToState<L, E>,
        Publisher: PublishChangeLog<L, E>,
        F: FnOnce(&W, &[L]) -> R,
    {
        let state_set = self.builder.build_initial_state_set(&cmd)?;
        self.validator.validate_command(&cmd, &state_set)?;

        let changes = self
            .applier
            .apply_command_and_collect_changes(&cmd, state_set)?;

        self.log_persister
            .persist_changelogs(&changes.changelogs)?;
        self.replayer
            .replay_changelogs_to_state(&changes.changelogs)?;
        self.publisher.publish_changelog(&changes.changelogs)?;

        Ok(result_mapper(&changes.writes, &changes.changelogs))
    }
}

/// single-thread state set 的模板示例。
///
/// 主链路：load state set in single thread -> validate -> apply and collect changes -> persist changelog -> replay state -> publish -> return result。
pub struct SingleThreadStateSetCmdTemplate<Loader, Validator, Applier, LogPersister, Replayer, Publisher> {
    pub loader: Loader,
    pub validator: Validator,
    pub applier: Applier,
    pub log_persister: LogPersister,
    pub replayer: Replayer,
    pub publisher: Publisher,
}

impl<Loader, Validator, Applier, LogPersister, Replayer, Publisher>
    SingleThreadStateSetCmdTemplate<Loader, Validator, Applier, LogPersister, Replayer, Publisher>
{
    pub fn handle<C, S, W, L, R, E, F>(&self, cmd: C, result_mapper: F) -> Result<R, E>
    where
        Loader: LoadStateSetInSingleThread<C, S, E>,
        Validator: ValidateCommand<C, S, E>,
        Applier: ApplyCommandAndCollectChanges<C, S, W, L, E>,
        LogPersister: PersistChangeLogs<L, E>,
        Replayer: ReplayChangeLogsToState<L, E>,
        Publisher: PublishChangeLog<L, E>,
        F: FnOnce(&W, &[L]) -> R,
    {
        let state_set = self.loader.load_state_set_in_single_thread(&cmd)?;
        self.validator.validate_command(&cmd, &state_set)?;

        let changes = self
            .applier
            .apply_command_and_collect_changes(&cmd, state_set)?;

        self.log_persister
            .persist_changelogs(&changes.changelogs)?;
        self.replayer
            .replay_changelogs_to_state(&changes.changelogs)?;
        self.publisher.publish_changelog(&changes.changelogs)?;

        Ok(result_mapper(&changes.writes, &changes.changelogs))
    }
}

/// update existing state set 的模板示例。
///
/// 主链路：零预判 -> 一锁 -> 二判 -> 三更新并产出 changeset -> persist changelog -> replay state -> publish -> return result。
pub struct UpdateStateSetCmdTemplate<PreChecker, Loader, InLockValidator, Applier, LogPersister, Replayer, Publisher> {
    pub pre_checker: PreChecker,
    pub loader: Loader,
    pub in_lock_validator: InLockValidator,
    pub applier: Applier,
    pub log_persister: LogPersister,
    pub replayer: Replayer,
    pub publisher: Publisher,
}

impl<PreChecker, Loader, InLockValidator, Applier, LogPersister, Replayer, Publisher>
    UpdateStateSetCmdTemplate<PreChecker, Loader, InLockValidator, Applier, LogPersister, Replayer, Publisher>
{
    pub fn handle<C, S, W, L, R, E, F>(&self, cmd: C, result_mapper: F) -> Result<R, E>
    where
        PreChecker: PreCheckCommand<C, E>,
        Loader: LoadStateSetForUpdate<C, S, E>,
        InLockValidator: ValidateCommandInLock<C, S, E>,
        Applier: ApplyCommandAndCollectChanges<C, S, W, L, E>,
        LogPersister: PersistChangeLogs<L, E>,
        Replayer: ReplayChangeLogsToState<L, E>,
        Publisher: PublishChangeLog<L, E>,
        F: FnOnce(&W, &[L]) -> R,
    {
        // 零预判：锁外快速失败，减少锁持有时间
        self.pre_checker.pre_check_command(&cmd)?;

        // 一锁：先锁定参与本次更新的关键 state set
        let state_set = self.loader.load_state_set_for_update(&cmd)?;

        // 二判：在锁内重新判断当前状态是否仍可更新
        self.in_lock_validator
            .validate_command_in_lock(&cmd, &state_set)?;

        // 三更新：执行更新逻辑，并同步产出 changeset
        let changes = self
            .applier
            .apply_command_and_collect_changes(&cmd, state_set)?;

        self.log_persister
            .persist_changelogs(&changes.changelogs)?;
        self.replayer
            .replay_changelogs_to_state(&changes.changelogs)?;
        self.publisher.publish_changelog(&changes.changelogs)?;

        Ok(result_mapper(&changes.writes, &changes.changelogs))
    }
}

/// 完整撮合场景模板示例。
///
/// 这个例子展示：
/// - 输入是 `MatchStateSet`
/// - 输出是 `ChangeSet<MatchWrites<...>, L>`
/// - 流程遵循并发更新场景的“零预判 + 一锁二判三更新”
/// - 落库阶段拆成“persist changelog”与“replay changelog to state”
pub struct MatchCmdTemplateExample<PreChecker, Loader, InLockValidator, Applier, LogPersister, Replayer, Publisher> {
    pub pre_checker: PreChecker,
    pub loader: Loader,
    pub in_lock_validator: InLockValidator,
    pub applier: Applier,
    pub log_persister: LogPersister,
    pub replayer: Replayer,
    pub publisher: Publisher,
}

impl<PreChecker, Loader, InLockValidator, Applier, LogPersister, Replayer, Publisher>
    MatchCmdTemplateExample<PreChecker, Loader, InLockValidator, Applier, LogPersister, Replayer, Publisher>
{
    pub fn handle<C, O, T, B, P, OB, L, R, E, F>(
        &self,
        cmd: C,
        result_mapper: F,
    ) -> Result<R, E>
    where
        PreChecker: PreCheckCommand<C, E>,
        Loader: LoadStateSetForUpdate<C, MatchStateSet<O, B, P, OB>, E>,
        InLockValidator: ValidateCommandInLock<C, MatchStateSet<O, B, P, OB>, E>,
        Applier: ApplyCommandAndCollectChanges<
            C,
            MatchStateSet<O, B, P, OB>,
            MatchWrites<O, T, B, P>,
            L,
            E,
        >,
        LogPersister: PersistChangeLogs<L, E>,
        Replayer: ReplayChangeLogsToState<L, E>,
        Publisher: PublishChangeLog<L, E>,
        F: FnOnce(&MatchWrites<O, T, B, P>, &[L]) -> R,
    {
        // 零预判：锁外快速失败，例如基本参数、时间窗、路由合法性检查
        self.pre_checker.pre_check_command(&cmd)?;

        // 一锁：读取并锁定本次撮合需要参与计算的状态集合
        let state_set = self.loader.load_state_set_for_update(&cmd)?;

        // 二判：锁内再次确认订单、余额、仓位、簿状态仍满足撮合条件
        self.in_lock_validator
            .validate_command_in_lock(&cmd, &state_set)?;

        // 三更新：执行撮合计算，同时产出订单、成交、余额、仓位等写集及 changelog
        let changes = self
            .applier
            .apply_command_and_collect_changes(&cmd, state_set)?;

        // 先持久化 changelog
        self.log_persister
            .persist_changelogs(&changes.changelogs)?;

        // 再回放 changelog，实现 state 更新
        self.replayer
            .replay_changelogs_to_state(&changes.changelogs)?;

        // 最后发布 changelog / event
        self.publisher.publish_changelog(&changes.changelogs)?;

        Ok(result_mapper(&changes.writes, &changes.changelogs))
    }
}
