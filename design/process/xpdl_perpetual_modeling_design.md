# Rust之从0-1低时延CEX：永续合约行为建模基于XPDL

## 1. 概述

本文档阐述如何使用 XPDL (XML Process Definition Language) 2.2 标准对永续合约交易系统进行业务流程建模，覆盖从开仓到强平的完整生命周期。

**核心目标**：
- 将复杂的永续合约业务规则转化为可执行的流程定义
- 支持 CQRS 架构模式
- 实现超低延迟（命令执行 < 100μs）

---

## 2. XPDL 建模方法论

### 2.1 分层架构

```
┌─────────────────────────────────────┐
│  XPDL Package: 永续合约交易流程     │
├─────────────────────────────────────┤
│  1. TypeDeclarations (类型系统)     │
│     - 基础类型 (TraderId, Price)    │
│     - 枚举类型 (OrderSide, Status)  │
│     - 复合类型 (Position, Order)    │
├─────────────────────────────────────┤
│  2. Participants (参与者定义)       │
│     - 角色: Trader, Risk Manager    │
│     - 系统: Matching/Risk Engine    │
├─────────────────────────────────────┤
│  3. WorkflowProcesses (流程定义)    │
│     - 主流程: 永续合约交易          │
│     - 子流程: 强平、资金费率结算    │
└─────────────────────────────────────┘
```

### 2.2 流程组织原则

**主流程设计原则**：
```
永续合约交易主流程
├── Start (交易请求开始)
├── RouteAction (动作路由网关)
│   ├── OpenPosition (开仓)
│   ├── ClosePosition (平仓)
│   ├── AdjustLeverage (调整杠杆)
│   ├── AddMargin (追加保证金)
│   ├── CancelOrder (撤销订单)
│   └── ... (其他活动)
└── End (成功/失败)
```

**子流程设计原则**：
- **独立触发**：不依赖用户请求，由系统自动触发
- **并行运行**：与主流程并行执行，互不阻塞
- **专用场景**：强平（风控触发）、资金费率（定时任务）

---

## 3. 类型系统建模

### 3.1 基础数据类型

XPDL 通过 `TypeDeclaration` 定义领域特定类型：

```xml
<!-- 价格类型：使用字符串避免浮点精度问题 -->
<TypeDeclaration Id="Price" Name="价格">
  <BasicType Type="STRING"/>
  <Description>
    字符串类型，内部使用定点数表示，保持精度。
    币安API使用字符串避免浮点误差
  </Description>
</TypeDeclaration>

<!-- 杠杆倍数：整数类型，范围限制 -->
<TypeDeclaration Id="Leverage" Name="杠杆倍数">
  <BasicType Type="INTEGER"/>
  <Description>范围: 1-125</Description>
</TypeDeclaration>
```

**设计要点**：
- 遵循币安 API 数据格式（价格/数量使用字符串）
- 避免浮点数精度损失
- 包含业务约束描述

### 3.2 枚举类型

表达业务状态和选项：

```xml
<!-- 持仓模式：单向 vs 对冲 -->
<TypeDeclaration Id="PositionMode" Name="持仓模式">
  <EnumerationType>
    <EnumerationValue Name="ONE_WAY">
      单向持仓模式（默认）
    </EnumerationValue>
    <EnumerationValue Name="HEDGE">
      对冲持仓模式（可同时持有多空）
    </EnumerationValue>
  </EnumerationType>
  <Description>
    币安API: GET/POST /fapi/v1/positionSide/dual
    - ONE_WAY: 一个合约只能持有一个方向
    - HEDGE: 同一合约可同时持有多空双向仓位
  </Description>
</TypeDeclaration>
```

**建模关键**：
- 枚举值映射到币安 API 规范
- 包含使用场景说明
- 关联 API 端点文档

### 3.3 复合数据类型

表达领域实体：

```xml
<!-- 持仓实体：包含完整的持仓信息 -->
<TypeDeclaration Id="Position" Name="持仓实体">
  <RecordType>
    <!-- 基础信息 -->
    <Member Name="symbol" Type="Symbol"/>
    <Member Name="position_side" Type="STRING"/>

    <!-- 持仓数量和价格 -->
    <Member Name="position_amount" Type="STRING"/>
    <Member Name="entry_price" Type="STRING"/>
    <Member Name="liquidation_price" Type="STRING"/>

    <!-- 杠杆和保证金 -->
    <Member Name="leverage" Type="INTEGER"/>
    <Member Name="margin_type" Type="STRING"/>
    <Member Name="isolated_margin" Type="STRING"/>

    <!-- 风控 -->
    <Member Name="adl" Type="INTEGER"/>
    <Description>
      自动减仓队列等级(1-5)，数字越大优先级越高
    </Description>
  </RecordType>
  <Description>
    对应币安API: GET /fapi/v2/positionRisk
  </Description>
</TypeDeclaration>
```

**设计原则**：
- 字段映射币安 API 响应结构
- 区分外部 API 字段和内部状态字段
- 包含业务语义描述

---

## 4. 活动（Activity）建模

### 4.1 开仓活动示例

活动是流程的原子执行单元：

```xml
<Activity Id="OpenPosition" Name="开仓">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/composite">
          <![CDATA[
          // 开仓活动由多个子任务组成
          async fn execute_open_position(request: OpenPositionRequest)
            -> Result<PositionId, Error>
          {
              // 1. 验证参数
              validate_open_params(&request)?;

              // 2. 检查保证金
              let margin_check = check_margin_sufficiency(&request).await?;

              // 3. 冻结保证金
              freeze_margin(request.trader, margin_check.required_margin).await?;

              // 4. 提交订单到撮合引擎
              let order_id = submit_position_order(&request).await?;

              // 5. 等待撮合成交
              let trades = match_order(order_id).await?;

              // 6. 创建持仓记录
              let position_id = create_position_from_trades(&request, &trades).await?;

              // 7. 计算强平价格
              let liquidation_price = calculate_liquidation_price(
                  request.entry_price, request.leverage, request.side
              );

              // 8. 注册到风控引擎
              register_risk_monitoring(position_id).await?;

              Ok(position_id)
          }
          ]]>
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
  <Performer>MATCHING_ENGINE</Performer>
  <Description>
    【使用场景】
    1. 看涨行情开多仓
    2. 看跌行情开空仓
    3. 对冲套利

    【业务规则】
    - 保证金计算：名义价值 / 杠杆倍数
    - 单向模式：同一合约只能持有一个方向
    - 对冲模式：可同时持有多空双向仓位
  </Description>
</Activity>
```

**活动建模要素**：

1. **Implementation（实现）**：
   - 使用 Rust 伪代码描述执行逻辑
   - 展示完整的执行步骤和错误处理
   - 返回执行结果

2. **Performer（执行者）**：
   - 指定负责执行的参与者
   - 支持系统组件（如撮合引擎、风控引擎）

3. **Description（描述）**：
   - 使用场景
   - 前置条件
   - 业务规则

### 4.2 活动流转控制

```xml
<TransitionRestrictions>
  <TransitionRestriction>
    <Join Type="XOR"/>  <!-- 排他性汇聚 -->
    <Split Type="XOR">  <!-- 排他性分支 -->
      <TransitionRefs>
        <TransitionRef Id="ToEndSuccess"/>
        <TransitionRef Id="ToEndFailure"/>
      </TransitionRefs>
    </Split>
  </TransitionRestriction>
</TransitionRestrictions>
```

**流转类型**：
- **XOR (Exclusive)**: 排他分支，只选择一条路径
- **AND (Parallel)**: 并行分支，同时执行多条路径
- **OR (Inclusive)**: 包含分支，选择一条或多条路径

---

## 5. 流程路由建模

### 5.1 动作路由网关

使用路由活动实现动态分发：

```xml
<Activity Id="RouteAction" Name="路由交易动作">
  <Route>
    <Condition Type="CONDITION">
      根据action字段分发到对应的活动分支
    </Condition>
  </Route>
  <TransitionRestrictions>
    <TransitionRestriction>
      <Join Type="XOR"/>
      <Split Type="XOR">
        <TransitionRefs>
          <TransitionRef Id="ToOpenPosition"/>
          <TransitionRef Id="ToClosePosition"/>
          <TransitionRef Id="ToAdjustLeverage"/>
          <!-- ... 其他动作 -->
        </TransitionRefs>
      </Split>
    </TransitionRestriction>
  </TransitionRestrictions>
</Activity>
```

### 5.2 条件转换

```xml
<Transitions>
  <Transition Id="ToOpenPosition" From="RouteAction" To="OpenPosition">
    <Condition Type="CONDITION">action == "OPEN_POSITION"</Condition>
  </Transition>

  <Transition Id="ToClosePosition" From="RouteAction" To="ClosePosition">
    <Condition Type="CONDITION">action == "CLOSE_POSITION"</Condition>
  </Transition>

  <!-- 成功/失败转换 -->
  <Transition Id="FromOpenToSuccess" From="OpenPosition" To="EndSuccess">
    <Condition Type="CONDITION">execution_success == true</Condition>
  </Transition>

  <Transition Id="FromOpenToFailure" From="OpenPosition" To="EndFailure">
    <Condition Type="OTHERWISE"/>
  </Transition>
</Transitions>
```

**设计模式**：
- 单一入口点（Start）
- 动态路由到不同活动
- 统一的成功/失败出口

### 5.3 完整流程图示例

```
┌─────────────────────────────────────────────────────────┐
│                永续合约交易主流程                        │
└─────────────────────────────────────────────────────────┘
                            │
                        [Start]
                            │
                            ▼
                    [RouteAction] ────┐
                            │         │
        ┌──────────┬────────┼────────┬┴────────┬──────────┐
        │          │        │        │         │          │
        ▼          ▼        ▼        ▼         ▼          ▼
   [OpenPos]  [ClosePos] [Adjust] [AddMar] [Cancel] [...Others]
        │          │        │        │         │          │
        └──────────┴────────┴────────┴─────────┴──────────┘
                            │
                    ┌───────┴───────┐
                    ▼               ▼
              [EndSuccess]    [EndFailure]


并行运行的独立子流程：

┌────────────────────────────┐     ┌────────────────────────────┐
│    强平流程 (子流程)        │     │  资金费率结算 (子流程)      │
│  Trigger: Message          │     │  Trigger: Timer (Cron)     │
│  Priority: 0 (最高)        │     │  Priority: 2               │
├────────────────────────────┤     ├────────────────────────────┤
│  [LiquidationTriggered]    │     │  [每4小时触发]             │
│           ▼                │     │           ▼                │
│  [执行三级强平]            │     │  [结算资金费率]            │
│    1️⃣ 市场强平             │     │    - 计算费率               │
│    2️⃣ 保险基金接管         │     │    - 查询持仓               │
│    3️⃣ 自动减仓(ADL)        │     │    - 批量结算               │
│           ▼                │     │           ▼                │
│  [强平完成]                │     │  [结算完成]                │
└────────────────────────────┘     └────────────────────────────┘
        ↑                                   ↑
        │                                   │
   风控引擎实时监控                    系统定时调度器
```

**流程执行特点**：
- **主流程**：同步执行，阻塞等待结果
- **子流程**：异步执行，不阻塞主流程
- **优先级**：强平流程最高优先级（0），确保风控实时性

---

## 6. 子流程建模

### 6.1 强平流程（事件触发）

```xml
<WorkflowProcess Id="LiquidationProcess" Name="强制平仓流程">
  <ProcessHeader>
    <Description>
      独立运行的强平流程 - 由风控引擎实时监控触发
    </Description>
    <Priority>0</Priority>  <!-- 最高优先级 -->
  </ProcessHeader>

  <Activities>
    <Activity Id="Start" Name="触发强平">
      <Event>
        <StartEvent Trigger="Message">
          <TriggerResultMessage>LiquidationTriggered</TriggerResultMessage>
        </StartEvent>
      </Event>
    </Activity>

    <Activity Id="ExecuteLiquidation" Name="执行三级强平">
      <!-- 实现币安三级强平机制 -->
      <Implementation>
        <Task>
          <TaskScript>
            <Script Type="rust/composite">
              <![CDATA[
              // 1️⃣ 第一级：市场强平
              // 2️⃣ 第二级：风险保障基金接管
              // 3️⃣ 第三级：自动减仓（ADL）
              ]]>
            </Script>
          </TaskScript>
        </Task>
      </Implementation>
    </Activity>
  </Activities>
</WorkflowProcess>
```

**设计特点**：
- **消息触发**：通过 `TriggerResultMessage` 响应风控事件
- **独立运行**：不阻塞主流程
- **最高优先级**：保证风控实时性

### 6.2 资金费率结算流程（定时触发）

```xml
<WorkflowProcess Id="FundingRateSettlementProcess" Name="资金费率结算流程">
  <Activities>
    <Activity Id="Start" Name="触发结算">
      <Event>
        <StartEvent Trigger="Timer">
          <TriggerTimer>
            <TimeCycle>0 */4 * * *</TimeCycle>  <!-- Cron表达式 -->
          </TriggerTimer>
        </StartEvent>
      </Event>
      <Description>
        每4小时触发一次 (00:00, 04:00, 08:00, 12:00, 16:00, 20:00 UTC)
        实际结算根据每个合约的funding_interval_hours配置执行
      </Description>
    </Activity>

    <Activity Id="SettleFunding" Name="结算资金费率">
      <Implementation>
        <Task>
          <TaskScript>
            <Script Type="rust/composite">
              <![CDATA[
              // 支持动态间隔（4小时/8小时）
              async fn settle_funding_rates(settlement_time: DateTime)
                -> Result<(), Error>
              {
                  // 1. 计算资金费率
                  // Premium Index = (Mark Price - Index Price) / Index Price
                  // Funding Rate = Premium Index + clamp(Interest - Premium, -0.05%, 0.05%)

                  // 2. 查询所有持仓
                  // 3. 批量结算（多仓支付，空仓收取）
                  // 4. 保存历史记录
              }
              ]]>
            </Script>
          </TaskScript>
        </Task>
      </Implementation>
    </Activity>
  </Activities>
</WorkflowProcess>
```

**定时任务建模**：
- **Trigger="Timer"**：定时触发
- **TimeCycle**：Cron 表达式（每 4 小时）
- **动态间隔支持**：根据合约配置决定是否执行

---

## 7. 数据字段建模

### 7.1 流程级数据字段

```xml
<DataFields>
  <!-- 交易请求基础信息 -->
  <DataField Id="trader_id" Name="交易员ID">
    <DataType><BasicType Type="STRING"/></DataType>
  </DataField>

  <DataField Id="action" Name="交易动作">
    <DataType><BasicType Type="STRING"/></DataType>
    <Description>
      OPEN_POSITION | CLOSE_POSITION | ADJUST_LEVERAGE |
      ADD_MARGIN | CANCEL_ORDER | ...
    </Description>
  </DataField>

  <!-- 开仓参数 -->
  <DataField Id="leverage" Name="杠杆倍数">
    <DataType><BasicType Type="INTEGER"/></DataType>
    <InitialValue>10</InitialValue>  <!-- 默认值 -->
  </DataField>

  <DataField Id="position_mode" Name="持仓模式">
    <DataType><BasicType Type="STRING"/></DataType>
    <InitialValue>ONE_WAY</InitialValue>
    <Description>ONE_WAY | HEDGE（对冲模式）</Description>
  </DataField>
</DataFields>
```

**数据字段用途**：
- 存储流程执行上下文
- 在活动之间传递数据
- 支持条件判断和路由决策

---

## 8. 扩展属性建模

### 8.1 性能指标

```xml
<ExtendedAttributes>
  <ExtendedAttribute Name="Performance-SLA">
    <ExtendedAttribute Name="MainProcess-Latency" Value="100μs"/>
    <ExtendedAttribute Name="Liquidation-Latency" Value="50μs"/>
    <ExtendedAttribute Name="Throughput-TPS" Value="100000"/>
  </ExtendedAttribute>
</ExtendedAttributes>
```

### 8.2 API 限频规则

```xml
<ExtendedAttribute Name="BinanceAPILimits">
  <!-- 订单频率限制 -->
  <ExtendedAttribute Name="OrderRateLimit"
                      Value="300 orders/10s per account"/>

  <!-- 请求权重限制 -->
  <ExtendedAttribute Name="RequestWeight"
                      Value="2400 weight/1min per IP"/>

  <!-- 具体端点权重 -->
  <ExtendedAttribute Name="POST_order_weight" Value="0 or 1"/>
  <ExtendedAttribute Name="POST_order_description"
                      Value="下单权重：GTC订单=0，其他=1"/>
</ExtendedAttribute>
```

### 8.3 限频错误处理

```xml
<ExtendedAttribute Name="RateLimitErrorHandling">
  <ExtendedAttribute Name="HTTP-418"
                      Value="IP被封禁，需等待自动解封（2分钟 - 3天）"/>
  <ExtendedAttribute Name="HTTP-429"
                      Value="请求过多，应实施指数退避策略"/>
  <ExtendedAttribute Name="ExponentialBackoff"
                      Value="建议退避序列：100ms, 200ms, 400ms, 800ms, 1.6s, 3.2s"/>
  <ExtendedAttribute Name="CircuitBreaker"
                      Value="连续触发5次后启动熔断器，暂停60秒"/>
</ExtendedAttribute>
```

---

## 9. 建模最佳实践

### 9.1 活动粒度

**推荐做法**：
- **粗粒度活动**：开仓、平仓、调整杠杆（对应用户意图）
- **细粒度任务**：保证金检查、订单提交（内部实现）

**反模式**：
- ❌ 将每个 API 调用建模为独立活动
- ❌ 过度细化导致流程图难以理解

### 9.2 流程组织

**主流程 vs 子流程**：

| 特征 | 主流程 | 子流程 |
|------|--------|--------|
| 触发方式 | 用户请求 | 系统事件/定时 |
| 依赖关系 | 同步执行 | 异步独立 |
| 示例 | 开仓、平仓 | 强平、资金费率 |

### 9.3 错误处理

**分层错误处理**：

```xml
<!-- 活动级错误 -->
<Transition From="OpenPosition" To="EndFailure">
  <Condition Type="OTHERWISE"/>
</Transition>

<!-- 流程级错误捕获 -->
<Activity Id="EndFailure" Name="交易失败">
  <Event>
    <EndEvent Result="ERROR"/>
  </Event>
</Activity>
```

**错误处理策略**：
- 活动级错误：每个活动捕获自身的执行错误
- 流程级错误：统一的失败出口节点
- 错误传播：通过 Transition 条件判断错误类型
- 故障恢复：支持重试和补偿机制

---

## 10. Rust Trait 接口定义

### 10.1 命令处理器 Trait (`PerpOrderExchProc`)

永续合约订单处理器遵循 CQRS 模式的命令处理接口，用于本地订单簿（LOB）撮合：

```rust
pub trait PerpOrderExchProc: Send + Sync {
    /// 处理开仓命令（本地撮合）
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "OpenPosition"
    /// - Performer: MATCHING_ENGINE
    ///
    /// # 参数
    /// - `cmd`: OpenPositionCommand - 开仓命令
    ///
    /// # 返回
    /// - `Ok(OpenPositionResult)`: 撮合成功，返回订单结果
    /// - `Err(PrepCommandError)`: 撮合失败，返回错误信息
    ///
    /// # 执行流程（映射XPDL TaskScript）
    /// 1. 验证命令 → ValidationError
    /// 2. 风控检查（保证金充足性）→ InsufficientBalance
    /// 3. 在订单簿中撮合
    /// 4. 更新持仓状态
    /// 5. 返回撮合结果
    fn open_position(&self, cmd: OpenPositionCommand)
        -> Result<OpenPositionResult, PrepCommandError>;

    /// 处理平仓命令（本地撮合）
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "ClosePosition"
    /// - Performer: MATCHING_ENGINE
    ///
    /// # 返回
    /// - 包含已实现盈亏（realized_pnl）
    fn close_position(&self, cmd: ClosePositionCommand)
        -> Result<ClosePositionResult, PrepCommandError>;

    /// 处理取消订单命令
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "CancelOrder"
    ///
    /// # 注意
    /// - 已成交的订单无法取消，返回 `CancelOrderResult::failed`
    /// - 已取消的订单重复取消，返回成功（幂等性）
    fn cancel_order(&self, cmd: CancelOrderCommand)
        -> Result<CancelOrderResult, PrepCommandError>;

    /// 修改订单（价格和/或数量）
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "ModifyOrder"
    ///
    /// # 注意
    /// - 只能修改未成交或部分成交的订单
    /// - 修改订单会重新进入订单簿撮合队列
    fn modify_order(&self, cmd: ModifyOrderCommand)
        -> Result<ModifyOrderResult, PrepCommandError>;

    /// 批量取消订单
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "CancelAllOrders"
    ///
    /// # 取消范围（通过命令参数控制）
    /// - `symbol = None, position_side = None`: 取消所有订单
    /// - `symbol = Some(s), position_side = None`: 取消指定交易对的所有订单
    /// - `symbol = None, position_side = Some(p)`: 取消指定持仓方向的所有订单
    /// - `symbol = Some(s), position_side = Some(p)`: 取消指定交易对和方向的订单
    fn cancel_all_orders(&self, cmd: CancelAllOrdersCommand)
        -> Result<CancelAllOrdersResult, PrepCommandError>;

    // ========================================================================
    // 账户配置命令（对应XPDL AccountConfiguration活动组）
    // ========================================================================

    /// 设置杠杆倍数
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "AdjustLeverage"
    /// - DataField: "leverage" (INTEGER, 1-125)
    ///
    /// # 使用场景
    /// - 首次交易前必需设置
    /// - 调整风险水平（盈利后降低杠杆）
    /// - 优化资金利用率（提高杠杆释放保证金）
    ///
    /// # 保证金影响
    /// - 降低杠杆：锁定更多保证金，强平价格变远（更安全）
    /// - 提高杠杆：释放保证金，强平价格变近（风险增加）
    fn set_leverage(&self, cmd: SetLeverageCommand)
        -> Result<SetLeverageResult, PrepCommandError>;

    /// 设置保证金类型
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "SetMarginType"
    /// - TypeDeclaration: "MarginType" (CROSSED | ISOLATED)
    ///
    /// # 全仓 vs 逐仓
    /// - 全仓 (Cross): 所有持仓共享账户余额
    /// - 逐仓 (Isolated): 每个持仓独立保证金
    ///
    /// # 注意
    /// - 必须在无持仓时设置
    /// - 每个交易对独立设置
    fn set_margin_type(&self, cmd: SetMarginTypeCommand)
        -> Result<SetMarginTypeResult, PrepCommandError>;

    /// 设置持仓模式
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "SetPositionMode"
    /// - TypeDeclaration: "PositionMode" (ONE_WAY | HEDGE)
    ///
    /// # 单向 vs 对冲模式
    /// - 单向模式 (One-Way): 只能持有一个方向
    /// - 对冲模式 (Hedge): 可同时持有多空
    ///
    /// # ⚠️ 重要约束
    /// - 全局设置，影响所有交易对
    /// - 必须在无持仓时设置
    /// - 切换后无法撤销
    fn set_position_mode(&self, cmd: SetPositionModeCommand)
        -> Result<SetPositionModeResult, PrepCommandError>;
}
```

### 10.2 查询处理器 Trait (`PerpOrderExchQueryProc`)

永续合约查询处理器，遵循 CQRS 读模型（Read Model）设计：

```rust
pub trait PerpOrderExchQueryProc: Send + Sync {
    /// 查询订单状态（从本地订单簿）
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryOrder"
    ///
    /// # 返回
    /// - `Ok(OrderQueryResult)`: 订单详细信息
    /// - `Err(PrepCommandError::OrderNotFound)`: 订单不存在
    fn query_order(&self, cmd: QueryOrderCommand)
        -> Result<OrderQueryResult, PrepCommandError>;

    /// 查询持仓信息（从本地持仓管理器）
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryPosition"
    /// - TypeDeclaration: "Position" (复合类型)
    ///
    /// # 返回持仓信息
    /// - symbol, position_side, quantity
    /// - entry_price, liquidation_price
    /// - leverage, margin_type
    /// - unrealized_pnl
    ///
    /// # 注意
    /// - 无持仓时返回 `PositionInfo::empty()`，而不是返回错误
    /// - 可通过 `has_position()` 判断是否有持仓
    fn query_position(&self, cmd: QueryPositionCommand)
        -> Result<PositionInfo, PrepCommandError>;

    /// 查询订单簿深度
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryOrderBook"
    ///
    /// # 快照内容
    /// - 买盘档位（按价格从高到低排序）
    /// - 卖盘档位（按价格从低到高排序）
    /// - 最佳买价和最佳卖价
    /// - 买卖价差、中间价
    fn query_order_book(&self, cmd: QueryOrderBookCommand)
        -> Result<OrderBookSnapshot, PrepCommandError>;

    /// 查询成交记录
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryTrades"
    ///
    /// # 查询条件
    /// - 支持按订单ID、交易对、时间范围过滤
    /// - 支持限制返回数量（分页）
    /// - 成交记录按时间降序排列（最新的在前）
    fn query_trades(&self, cmd: QueryTradesCommand)
        -> Result<TradesQueryResult, PrepCommandError>;

    /// 查询账户余额
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryAccountBalance"
    ///
    /// # 返回信息
    /// - 总余额 (balance)
    /// - 可用余额 (available_balance)
    /// - 仓位保证金 (position_margin)
    /// - 挂单保证金 (order_margin)
    /// - 未实现盈亏 (unrealized_pnl)
    fn query_account_balance(&self, cmd: QueryAccountBalanceCommand)
        -> Result<Vec<AccountBalance>, PrepCommandError>;

    /// 查询账户完整信息
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryAccountInfo"
    ///
    /// # 包含信息
    /// - 总资产、可用余额、总盈亏
    /// - 所有持仓列表
    /// - 所有资产余额
    /// - 风险率、杠杆率计算
    fn query_account_info(&self, cmd: QueryAccountInfoCommand)
        -> Result<AccountInfo, PrepCommandError>;

    /// 查询标记价格
    ///
    /// # 对应XPDL活动
    /// - Activity Id: "QueryMarkPrice"
    /// - TypeDeclaration: "MarkPrice" (标记价格实体)
    ///
    /// # 价格类型说明
    /// - 标记价格 (Mark Price): 用于强平判断和盈亏计算
    /// - 指数价格 (Index Price): 多交易所加权平均
    /// - 最新价 (Last Price): 订单成交价
    ///
    /// # 包含资金费率信息
    /// - 当前资金费率
    /// - 下次结算时间
    /// - 预估结算价格
    fn query_mark_price(&self, cmd: QueryMarkPriceCommand)
        -> Result<Vec<MarkPriceInfo>, PrepCommandError>;

    /// 查询历史资金费率
    ///
    /// # 对应XPDL子流程
    /// - SubProcess: "FundingRateSettlementProcess"
    /// - Activity Id: "QueryFundingRateHistory"
    ///
    /// # 使用场景
    /// - 趋势分析：分析历史费率趋势，判断市场情绪
    /// - 成本预估：预估持仓期间的资金费用成本
    /// - 策略回测：回测资金费率套利策略
    fn query_funding_rate_history(&self, cmd: QueryFundingRateHistoryCommand)
        -> Result<Vec<FundingRateRecord>, PrepCommandError>;

    /// 查询资金费用收支记录
    ///
    /// # 对应XPDL子流程
    /// - SubProcess: "FundingRateSettlementProcess"
    /// - Activity Id: "QueryFundingFee"
    ///
    /// # 资金费用计算
    /// ```
    /// 资金费用 = 持仓名义价值 × 资金费率
    /// 持仓名义价值 = 标记价格 × 持仓数量
    ///
    /// 正费率时：
    /// - 多头持仓：支付费用（income为负）
    /// - 空头持仓：收取费用（income为正）
    /// ```
    fn query_funding_fee(&self, cmd: QueryFundingFeeCommand)
        -> Result<Vec<FundingFeeRecord>, PrepCommandError>;
}
```

### 10.3 从 XPDL 到 Rust 代码映射

**XPDL 活动定义**：
```xml
<Activity Id="OpenPosition" Name="开仓">
  <Implementation>
    <Task>
      <TaskScript>
        <Script Type="rust/composite">
          <![CDATA[
          async fn execute_open_position(request: OpenPositionRequest)
            -> Result<PositionId, Error>
          {
              // 1. 验证参数
              validate_open_params(&request)?;

              // 2. 检查保证金
              let margin_check = check_margin_sufficiency(&request).await?;

              // 3. 冻结保证金
              freeze_margin(request.trader, margin_check.required_margin).await?;

              // 4. 提交订单到撮合引擎
              let order_id = submit_position_order(&request).await?;

              // 5. 等待撮合成交
              let trades = match_order(order_id).await?;

              // 6. 创建持仓记录
              let position_id = create_position_from_trades(&request, &trades).await?;

              // 7. 计算强平价格
              let liquidation_price = calculate_liquidation_price(
                  request.entry_price, request.leverage, request.side
              );

              // 8. 注册到风控引擎
              register_risk_monitoring(position_id).await?;

              Ok(position_id)
          }
          ]]>
        </Script>
      </TaskScript>
    </Task>
  </Implementation>
  <Performer>MATCHING_ENGINE</Performer>
</Activity>
```

**Rust Trait 实现**：
```rust
// proc/operating/exchange/derivatives/src/proc/trading_prep_order_proc_impl.rs
pub struct LocalMatchingEngine {
    order_book: Arc<OrderBook>,
    positions: Arc<RwLock<HashMap<(Symbol, PositionSide), PositionInfo>>>,
    balance_manager: Arc<BalanceManager>,
}

impl PerpOrderExchProc for LocalMatchingEngine {
    fn open_position(&self, cmd: OpenPositionCommand)
        -> Result<OpenPositionResult, PrepCommandError>
    {
        // 1. 验证命令 - 对应XPDL validate_open_params
        cmd.validate()
            .map_err(PrepCommandError::ValidationError)?;

        // 2. 风控检查 - 对应XPDL check_margin_sufficiency
        let required_margin = self.calculate_required_margin(&cmd)?;
        if !self.balance_manager.has_sufficient_balance(required_margin) {
            return Err(PrepCommandError::InsufficientBalance);
        }

        // 3. 生成订单ID
        let order_id = OrderId::generate();

        // 4. 在订单簿中撮合 - 对应XPDL match_order
        let match_result = self.order_book.match_order(&cmd)?;

        // 5. 更新持仓 - 对应XPDL create_position_from_trades
        self.update_position(&cmd, &match_result)?;

        // 6. 注册风控监控 - 对应XPDL register_risk_monitoring
        self.register_liquidation_monitoring(&cmd, &match_result)?;

        // 7. 返回结果
        match match_result.status {
            MatchStatus::FullyFilled => {
                Ok(OpenPositionResult::filled(
                    order_id,
                    match_result.trades,
                    match_result.seq,
                ))
            }
            MatchStatus::PartiallyFilled => {
                Ok(OpenPositionResult::partially_filled(
                    order_id,
                    match_result.trades,
                    match_result.seq,
                ))
            }
            MatchStatus::NoMatch => {
                Ok(OpenPositionResult::accepted(order_id))
            }
        }
    }

    fn close_position(&self, cmd: ClosePositionCommand)
        -> Result<ClosePositionResult, PrepCommandError>
    {
        // 对应XPDL ClosePosition活动
        // ...实现细节
    }

    fn set_leverage(&self, cmd: SetLeverageCommand)
        -> Result<SetLeverageResult, PrepCommandError>
    {
        // 对应XPDL AdjustLeverage活动
        // ...实现细节
    }

    // ...其他方法实现
}
```

### 10.4 Trait 接口与 XPDL 活动映射表

| Trait 方法 | XPDL Activity Id | XPDL Performer | 主要职责 |
|-----------|------------------|----------------|---------|
| `open_position` | OpenPosition | MATCHING_ENGINE | 开仓撮合，持仓创建 |
| `close_position` | ClosePosition | MATCHING_ENGINE | 平仓撮合，盈亏计算 |
| `cancel_order` | CancelOrder | MATCHING_ENGINE | 订单取消 |
| `modify_order` | ModifyOrder | MATCHING_ENGINE | 订单修改 |
| `cancel_all_orders` | CancelAllOrders | MATCHING_ENGINE | 批量取消 |
| `set_leverage` | AdjustLeverage | ACCOUNT_MANAGER | 杠杆设置 |
| `set_margin_type` | SetMarginType | ACCOUNT_MANAGER | 保证金类型设置 |
| `set_position_mode` | SetPositionMode | ACCOUNT_MANAGER | 持仓模式设置 |
| `query_order` | QueryOrder | MATCHING_ENGINE | 订单查询 |
| `query_position` | QueryPosition | ACCOUNT_MANAGER | 持仓查询 |
| `query_order_book` | QueryOrderBook | MATCHING_ENGINE | 订单簿深度查询 |
| `query_trades` | QueryTrades | MATCHING_ENGINE | 成交记录查询 |
| `query_account_balance` | QueryAccountBalance | ACCOUNT_MANAGER | 余额查询 |
| `query_account_info` | QueryAccountInfo | ACCOUNT_MANAGER | 账户信息查询 |
| `query_mark_price` | QueryMarkPrice | MARKET_DATA | 标记价格查询 |
| `query_funding_rate_history` | QueryFundingRateHistory | MARKET_DATA | 历史费率查询 |
| `query_funding_fee` | QueryFundingFee | ACCOUNT_MANAGER | 费用记录查询 |




## 11. 总结

### 11.1 XPDL 建模的优势

✅ **标准化**：使用 XPDL 2.2 国际标准，跨工具兼容
✅ **可视化**：流程图自动生成，易于理解
✅ **可执行**：直接映射到代码实现
✅ **可维护**：业务逻辑与实现分离
✅ **可追溯**：版本控制和变更历史

### 11.2 核心设计模式

1. **活动组合模式**：复杂业务拆解为可复用活动
2. **路由网关模式**：动态分发请求到不同处理分支
3. **事件驱动模式**：子流程通过事件触发，解耦依赖
4. **分层数据模型**：基础类型 → 枚举 → 复合实体

### 11.3 建模检查清单

| 检查项 | 描述 | 优先级 | 状态 |
|--------|------|--------|------|
| **类型系统完整** | 基础类型、枚举、复合类型定义齐全 | 🔴 必须 | [ ] |
| **活动粒度适中** | 活动对应用户意图，避免过度细化 | 🔴 必须 | [ ] |
| **流程组织清晰** | 主流程 vs 子流程职责明确 | 🔴 必须 | [ ] |
| **错误处理完备** | 每个活动分支都有成功/失败出口 | 🔴 必须 | [ ] |
| **性能指标明确** | 定义延迟、吞吐量等 SLA 指标 | 🟡 重要 | [ ] |
| **API 规范遵循** | 完整映射币安 API 端点和参数 | 🔴 必须 | [ ] |
| **限频策略完善** | 请求权重、熔断器、重试机制 | 🟡 重要 | [ ] |
| **事件定义完整** | 领域事件包含完整业务快照 | 🟡 重要 | [ ] |
| **参与者明确** | Performer 指定清晰 | 🔴 必须 | [ ] |
| **文档注释充分** | 每个活动包含使用场景和业务规则 | 🟢 建议 | [ ] |

**检查要点说明**：

1. **类型系统完整**：
   - ✅ 所有 API 字段都有对应的类型定义
   - ✅ 枚举值与币安 API 完全匹配
   - ✅ 复合类型包含完整的业务属性

2. **活动粒度适中**：
   - ✅ 一个活动对应一个完整的用户操作
   - ❌ 避免：将 API 调用拆分为独立活动
   - ❌ 避免：单个活动包含过多业务逻辑

3. **流程组织清晰**：
   - ✅ 主流程：用户请求触发，同步执行
   - ✅ 子流程：系统事件/定时触发，异步执行
   - ✅ 流程间通过事件解耦

4. **API 规范遵循**：
   - ✅ TypeDeclaration 关联币安 API 端点
   - ✅ 字段名称与 API 响应一致
   - ✅ 限频规则在 ExtendedAttributes 中定义

---

## 12. 参考资料

### 12.1 外部标准和文档

- **XPDL 2.2 规范**：http://www.wfmc.org/standards/docs/bpmnxpdl_40.xsd
- **WFMC 官方网站**：http://www.wfmc.org/
- **币安 API 文档**：https://binance-docs.github.io/apidocs/futures/en/
- **CQRS 模式**：https://martinfowler.com/bliki/CQRS.html
- **事件溯源**：https://martinfowler.com/eaaDev/EventSourcing.html

