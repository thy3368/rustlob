# XPDL 永续合约行为建模设计文档

## 1. 概述

本文档阐述如何使用 XPDL (XML Process Definition Language) 2.2 标准对永续合约交易系统进行业务流程建模，覆盖从开仓到强平的完整生命周期。

**核心目标**：
- 将复杂的永续合约业务规则转化为可执行的流程定义
- 支持 CQRS + 事件溯源架构
- 实现超低延迟（命令执行 < 100μs）
- 符合币安 API 规范和限频要求

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

              // 发布事件
              publish_event(PositionOpenedEvent { ... }).await?;

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
   - 体现事件溯源模式（发布领域事件）

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

**事件溯源支持**：
- 所有状态变更发布领域事件
- 支持重放和审计
- 故障恢复能力

**事件定义示例**：

```xml
<!-- 持仓开仓事件 -->
<TypeDeclaration Id="PositionOpenedEvent" Name="持仓开仓事件">
  <RecordType>
    <Member Name="event_id" Type="STRING"/>
    <Description>事件唯一标识（UUID）</Description>

    <Member Name="aggregate_id" Type="PositionId"/>
    <Description>聚合根ID（持仓ID）</Description>

    <Member Name="timestamp" Type="DATETIME"/>
    <Description>事件发生时间（毫秒级时间戳）</Description>

    <Member Name="trader" Type="TraderId"/>
    <Member Name="symbol" Type="Symbol"/>
    <Member Name="position_side" Type="STRING"/>
    <Description>BOTH | LONG | SHORT</Description>

    <Member Name="entry_price" Type="Price"/>
    <Member Name="quantity" Type="Quantity"/>
    <Member Name="leverage" Type="INTEGER"/>
    <Member Name="margin_type" Type="STRING"/>
    <Description>ISOLATED | CROSSED</Description>

    <Member Name="initial_margin" Type="STRING"/>
    <Member Name="liquidation_price" Type="STRING"/>
  </RecordType>
  <Description>
    领域事件：记录持仓创建的完整快照
    用途：事件溯源、审计追踪、实时通知
  </Description>
</TypeDeclaration>

<!-- 强平触发事件 -->
<TypeDeclaration Id="LiquidationTriggeredEvent" Name="强平触发事件">
  <RecordType>
    <Member Name="event_id" Type="STRING"/>
    <Member Name="aggregate_id" Type="PositionId"/>
    <Member Name="timestamp" Type="DATETIME"/>

    <Member Name="trigger_price" Type="Price"/>
    <Description>触发强平时的标记价格</Description>

    <Member Name="liquidation_price" Type="Price"/>
    <Description>预设的强平价格</Description>

    <Member Name="margin_ratio" Type="FLOAT"/>
    <Description>当前保证金率（已低于维持保证金率）</Description>

    <Member Name="trigger_reason" Type="STRING"/>
    <Description>PRICE_BREACH | MARGIN_INSUFFICIENT</Description>
  </RecordType>
</TypeDeclaration>
```

**事件流示例**：

```
用户开仓流程的事件序列：

1. OpenPositionCommand (命令)
   ↓
2. MarginFrozenEvent (保证金冻结事件)
   ↓
3. OrderSubmittedEvent (订单提交事件)
   ↓
4. OrderMatchedEvent (订单撮合事件)
   ↓
5. PositionOpenedEvent (持仓开仓事件)
   ↓
6. RiskMonitoringRegisteredEvent (风控监控注册事件)

这些事件会：
- 存储到事件存储（Event Store）
- 更新读模型投影（Read Model Projection）
- 触发下游订阅者（如通知服务、数据分析）
```

---

## 10. 实现映射

### 10.1 从 XPDL 到 Rust 代码

**XPDL 活动**：
```xml
<Activity Id="OpenPosition" Name="开仓">
  <TaskScript>
    <Script Type="rust/composite">
      async fn execute_open_position(request: OpenPositionRequest)
        -> Result<PositionId, Error>
    </Script>
  </TaskScript>
</Activity>
```

**Rust 实现**：
```rust
// domain/usecases/open_position.rs
pub struct OpenPositionUseCase {
    order_repo: Arc<dyn OrderRepository>,
    exchange_gateway: Arc<dyn ExchangeGateway>,
}

impl OpenPositionUseCase {
    pub async fn execute(&self, request: OpenPositionRequest)
        -> Result<PositionId, Error>
    {
        // 1. 验证参数
        self.validate_params(&request)?;

        // 2. 检查保证金
        let margin = self.check_margin(&request).await?;

        // 3. 提交订单
        let order_id = self.submit_order(&request).await?;

        // 4. 发布事件
        self.publish_event(PositionOpenedEvent { ... }).await?;

        Ok(order_id)
    }
}
```

### 10.2 流程引擎执行

```rust
// infrastructure/workflow/workflow_engine.rs
pub struct XPDLWorkflowEngine {
    process_definitions: HashMap<String, WorkflowProcess>,
}

impl XPDLWorkflowEngine {
    pub async fn execute_process(
        &self,
        process_id: &str,
        data: ProcessData
    ) -> Result<ProcessResult, Error>
    {
        let process = self.process_definitions.get(process_id)?;

        // 1. 找到开始节点
        let start = process.find_start_activity()?;

        // 2. 执行流程
        let mut current = start;
        loop {
            // 执行当前活动
            let result = self.execute_activity(current, &data).await?;

            // 根据条件选择下一个活动
            current = self.select_next_activity(current, result)?;

            // 到达结束节点
            if current.is_end() {
                break;
            }
        }

        Ok(ProcessResult::Success)
    }
}
```

---

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

- **XPDL 2.2 规范**：http://www.wfmc.org/standards/docs/bpmnxpdl_40.xsd
- **币安 API 文档**：https://binance-docs.github.io/apidocs/futures/en/
- **CQRS 模式**：https://martinfowler.com/bliki/CQRS.html
- **事件溯源**：https://martinfowler.com/eaaDev/EventSourcing.html

---

**文档版本**：v1.0
**创建日期**：2025-12-14
**作者**：RustLOB Exchange Team
