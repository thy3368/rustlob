# Use Case Group Boundary 与链路闭合

本文件定义 `use case group` 的业务边界、业务真相中心、端到端 `MI` 链路闭合和最终落定事实。

依赖顺序固定为：

1. 先读 [`moment_interval_definition.md`](./moment_interval_definition.md)，确认候选事实是否满足 `Moment-Interval` 基础定义。
2. 再读 [`mi.md`](./mi.md)，确认候选对象是 `main_mi`、`secondary_mis`、资金侧 `MI`、审计凭证型 `MI`，以及哪些事实必须 append-only。
3. 最后读本文件，判断 `group_boundary`、`business_truth_center`、`final_settled_fact` 和是否需要拆成独立 `use case group`。

本文件不重新定义 `MI`，也不提供 `MI -> entity` 命名校准规则。

## 核心概念

### `group_boundary`

`group_boundary` 是一个 `use case group` 对业务真相负责的范围声明。

它必须回答：

- 这个 group 以哪个主业务事实为中心
- 负责哪条 `MI causal chain` / 业务真相链
- 覆盖哪些合法演化
- 边界内最终落定事实在哪里
- 哪些相邻动作不属于这里，以及为什么

`group_boundary` 不是 API 范围、handler 调用范围、数据库事务范围或消息流范围。

### `business_truth_center`

`business_truth_center` 是 group 的中心业务真相上下文，通常就是该边界内的 `main_mi`。

它负责回答：

- 这一组业务动作究竟围绕谁的合法演化展开
- 谁是这条业务真相链的起点锚
- 谁定义了这一组 use case 的主要生命周期

使用非 `MI` 等价物作为 `business_truth_center` 是显式例外，不是通用 fallback；必须先能解释为什么它仍能承载该 group 的业务真相。

### `main_mi`

`main_mi` 是边界内最主要的事实主语，通常也是 `business_truth_center`。

它提供主生命周期和主要合法演化空间，但不意味着链路上所有事实都要折叠进它内部。

### `secondary_mis`

`secondary_mis` 是由 `main_mi` 推进过程中派生出的独立事实对象。

它们可以保留在当前 group 的链路内部，也可能因独立身份、生命周期、授权、失败语义或审计语义而拆成另一个 `use case group`。

### `final_settled_fact`

`final_settled_fact` 是声明的 `group_boundary` 内，业务真相链真正完成闭环后可被认定为“已经落定”的最终事实。

它不是固定名词，也不等于“最后一次数据库更新”。

## 端到端 `MI` 链设计法

设计一条业务链时，不要只看 API 顺序，也不要只看 handler 调用图。

应该按以下顺序建模：

1. 先声明 `group_boundary`
2. 找到该边界内的 `business_truth_center` / `main_mi`
3. 写出它从成立到闭环的合法推进路径
4. 标出推进过程中派生出的 `secondary_mis`
5. 标出哪些事实必须 append-only 留痕
6. 标出这条边界内的 `final_settled_fact`
7. 反查哪些对象只是步骤、字段、检查、状态值，不应冒充 `MI`

### 具体问题模板

对任意候选对象，都用下面的问题逼近：

1. 它是不是这一组业务动作真正围绕的主语？
2. 如果不是主语，它是否仍然构成必须独立留痕的次级事实？
3. 如果没有它，争议时是否无法复原完整因果链？
4. 它是一次独立凭证，还是 `main_mi` 内部的一步状态推进？
5. 它是否触发了新的授权、失败语义、审计语义、资金语义？
6. 它是否应该拆出独立 `use case group`，还是仍留在主链内部？

## 何时拆成独立 `use case group`

当一个次级对象同时满足以下大部分条件时，通常应考虑拆组：

1. 它有独立身份
2. 它有独立生命周期
3. 它有独立授权或失败语义
4. 它有独立审计意义
5. 业务会围绕它定义多种合法动作

如果它只是 `main_mi` 内部推进的一步，或只是为了完成主链闭环而产生的一次性子事实，则通常不必单独升格为 group center。

不要为了以下原因强行合组：

- 一个 API 当前同时触达多个事实对象
- 一个 workflow 当前编排多个事实对象
- 一个表或流当前存储多个事实对象
- 一个技术事务当前把多个更新放在一起

## 最终 `settled fact` 的判定法

`final_settled_fact` 是：

- 在声明的 `group_boundary` 内
- 这条业务真相链真正完成闭环后
- 对业务来说可被认定为“已经落定”的最终事实

判断方法：

1. 问“如果链条停在这里，业务会不会认为这件事已经真正办完？”
2. 问“此后还有没有必须继续推进才能成立的核心事实？”
3. 问“外部争议/对账时，是否可以把这里当作该边界内的最终结论？”

例子：

- 仅下单边界里，`Order accepted/rejected/cancelled/filled` 可能已是 `final_settled_fact`
- `Order -> Trade -> Settlement` 的完整交易履约边界里，`Trade` 不是最终落定事实，`Settlement` 才更接近 `final_settled_fact`
- 清算边界里，如果声明覆盖到“缺口被 IF 或 ADL 处理完成”，那么 `LiquidationExecution` 不是 `final_settled_fact`，`shortfall closed / adl resolved` 才更接近

不要把“技术流程暂时结束”误判成 `final_settled_fact`。

## 交易例子：`Order / Trade / Settlement`

- `Order` 常是 `main_mi` 和 `business_truth_center`
- `Trade` 常是 `secondary_mi`
- 如果边界只到撮合完成，`Trade` 可能已是该边界内的 `final_settled_fact`
- 如果边界声明覆盖完整履约，`Settlement` 才更接近 `final_settled_fact`

不要因为 `Trade` 重要，就自动让它与 `Order` 共用同一个 group center。

边界判断示例：

- 若 `group_boundary = order matching`，`Order --[可撮合谓词, caused_by/due_to]--> Trade` 可能已经闭合。
- 若 `group_boundary = full fulfillment`，`Order --[可撮合谓词, caused_by/due_to]--> Trade` 仍不完整，必须继续追到资金或资产 settlement 的落定事实。

## 清算例子：`Liquidation / Shortfall / IF / ADL`

这是一条典型的端到端 `MI` 链：

1. `LiquidationEvent` 成立
2. 派生 `LiquidationExecution / Fill`
3. 若执行价劣于破产价，派生 `BankruptcyShortfallEvent`
4. 若需基金兜底，产生 `InsuranceFundDrawdownEvent`
5. 若基金不足，产生 `ADLRound`
6. 批次内产生多个 `ADLDeleveragingRecord`
7. 必要时以 `InsuranceFundTransfer` 或 `InsuranceFundAdjustment` 收尾

这里：

- `LiquidationEvent` 常是 `business_truth_center` / `main_mi`
- `BankruptcyShortfallEvent`、`ADLRound`、`ADLDeleveragingRecord` 常是必须独立留痕的 `secondary_mis`
- `available_balance` 不是 `MI`
- `CheckRisk` 不是 `MI`
- `run_adl_step_3` 不是 `MI`

如果声明的 `group_boundary` 覆盖“缺口被彻底处理完成”，则 `final_settled_fact` 更接近：

- `shortfall fully covered`
- 或 `adl resolved / exhausted with explicit result`

而不是“刚触发强平”。

## 审计留痕自检

做完建模后，用这组问句自检：

1. 任意争议发生时，能否从 `business_truth_center` 一路追到 `final_settled_fact`？
2. 链上的关键因果事实，是否都以 append-only 凭证存在？
3. 是否有哪个对象只是状态字段，却被错误包装成 `MI`？
4. 是否有哪个必须独立留痕的资金事实，被偷懒折叠进余额快照？
5. 是否有哪个 `secondary_mi` 其实已具备独立身份与审计意义，却还被埋在主对象内部？
6. 是否有哪个相邻事实已经超出当前 `group_boundary`，应该拆成另一个 group？

如果这些问题答不稳，说明 `group_boundary` 或 `MI causal chain` 大概率还没闭合。

## 输出约束

当回答 `use case group` 边界或链路闭合问题时，结果里应显式给出：

- `group_boundary`
- `business_truth_center`
- `main_mi`
- `secondary_mis`
- `append_only_facts`
- `end_to_end_mi_chain`
- `final_settled_fact`
- `which_facts_require_independent_mi`
- `which_items_are_not_mi_and_why`

每个 `secondary_mi` 应说明前驱事实和派生原因。优先把 `end_to_end_mi_chain` 写成：

```text
predecessor_mi --[predicate, caused_by/due_to]--> successor_mi
```

不要按 API 形状、handler 形状、数据库表形状或技术步骤顺序划分 `use case group`。
