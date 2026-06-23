# MI 领域建模总纲

这是 RustLOB skill 体系中关于 `Moment-Interval` / `MI` 的 canonical reference。

它回答的不是“某个案例怎么实现”，而是以下建模判定问题：

- 什么对象值得升格为 `MI`
- 一条业务链上哪些事实必须 append-only 留痕
- 哪些只是命令、字段、余额、步骤，不应冒充 `MI`
- 何时应拆成独立 `use case group`
- 何时只是主 `MI` 内部推进，不应升格

当任务涉及 `MI 识别`、`主次 MI`、`审计链`、`settled fact`、`业务事实留痕`、`最终事实闭环` 时，应先读本文件，再继续做 use case group、entity、aggregate 或流程边界判断。

## 一句话定义

`MI` 是“业务上值得被单独记住、可被追溯、必须可审计，并沿时间发生合法演化的事实性对象”。

如果一个对象不值得被独立留痕、不能形成事实链、也不承载独立审计意义，它通常不是 `MI`。

## MI 判定标准

把一个对象判断为 `MI`，通常应同时满足大部分以下条件：

1. 它不是一次技术动作，而是一条业务事实。
2. 它的存在本身值得被记录、查询、审计、追责或复盘。
3. 它有自己的业务身份，不能只靠别的对象“顺手带出”。
4. 它会沿时间发生合法演化，或至少会成为后续事实链的稳定锚点。
5. 业务会围绕它定义状态、阶段、结果、失效、撤销、完成、拒绝、关闭等语义。
6. 多个业务动作之所以存在，是因为这个对象会沿不同合法路径推进。
7. 即使底层实现方式变化，业务仍会认为“这个东西”真实存在。
8. 当出现争议、仲裁、追责、对账时，必须能把它单独拿出来复原真相。

### 四问速判

先问四个问题：

1. 业务会不会说“这件事本身发生过”？
2. 这件事值不值得单独记账、审计、查询？
3. 它后续会不会继续演化，或被后续事实反复引用？
4. 围绕它是否天然存在一组独立业务动作？

如果大多数答案是“会”，它大概率是 `MI`。

### 反证法

再问：

1. 去掉它，业务真相是否仍然完整？
2. 它是否只是为了支持别的对象而临时存在？
3. 它是否只是命令、字段、余额数值、检查步骤、技术流程节点的另一种说法？

如果这些反证大多成立，它大概率不是 `MI`。

## MI 与 command / state / entity / aggregate / description 的边界

## 与 `command` 的边界

`command` 是“有人请求系统做什么”。

`MI` 是“业务认可为已经成立、值得被记住、会继续演化或被持续引用的事实对象”。

不要把“请求”误当成“事实”。

典型关系只有三种：

1. 一个 `command` 创建一个新的 `MI`
2. 一个 `command` 推进一个已有 `MI`
3. 一个 `command` 推进主 `MI`，同时派生一个次级 `MI`

如果某个东西只在命令入口有意义，进入业务事实层后并不作为独立真相存在，它就不是 `MI`。

## 与 `state` 的边界

`state` 是某个业务对象在当前时刻的状态表达。

`MI` 是承载业务事实及其合法演化的对象本身。

不要把“当前状态值”当成 `MI`。

例如：

- `Order` 可能是 `MI`
- `OrderStatus = Working` 不是 `MI`
- `available_balance = 100` 不是 `MI`
- `FundHold` 可能是 `MI`
- `frozen_amount = 100` 只是某个状态字段

如果一个对象只能表达“现在是多少”，而不能表达“什么事实成立过、如何推进到这里”，它通常不是 `MI`。

## 与 `entity` 的边界

`entity` 是代码/领域建模里的对象形态。

`MI` 是业务真相层面的对象类型判断。

二者经常重叠，但不等价：

- 一个 `MI` 往往会落成 `entity` 或聚合内核心对象
- 不是所有 `entity` 都是 `MI`
- 一些 `Description`、policy carrier、snapshot object 也可能是 `entity`，但不是 `MI`

先判断“它是不是值得独立记住的业务事实”，再决定它在代码里怎么落。

## 与 `aggregate` 的边界

`aggregate` 解决的是一致性边界。

`MI` 解决的是业务事实边界与审计边界。

不要因为某个对象需要事务一致性，就自动把它当成 `MI`；
也不要因为某个对象是 `MI`，就默认它必须单独成为一个聚合。

判断顺序应是：

1. 先识别 `MI`
2. 再判断哪些 `MI` 或状态对象应落在同一聚合内保持一致
3. 最后再判断哪些跨聚合协调必须留在 `use case`

## 与 `description` 的边界

`description` 是规则、分类、能力、约束、阈值、模板、市场配置等描述性对象。

它解释业务如何运行，但它本身通常不是“发生过的事实”。

例如：

- 风控参数表
- 杠杆档位配置
- 费用规则
- liquidation policy
- ADL 排序规则

这些都可能非常重要，但通常是 `Description`，不是 `MI`。

## 主 MI / 次级 MI / 资金侧 MI / 审计凭证型 MI

## 主 MI

`主 MI` 是某个 `use case group` 的 `business_truth_center`。

它负责回答：

- 这一组业务动作究竟围绕谁的合法演化展开
- 谁是这条业务真相链的起点锚
- 谁定义了这一组 use case 的主要生命周期

例子：

- 交易下单链中，`Order` 常是主 `MI`
- 清算链中，`LiquidationEvent` 常是主 `MI`
- 资金保留链中，`FundHold` 常是主 `MI`

## 次级 MI

`次级 MI` 是由主 `MI` 推进过程中派生出来、但自身也具备独立业务事实意义的对象。

它通常：

- 被主 `MI` 引发
- 有独立身份
- 有独立审计价值
- 不一定与主 `MI` 属于同一个 group

例子：

- `Order` 推进派生 `Trade`
- `LiquidationEvent` 推进派生 `BankruptcyShortfallEvent`
- `ADLRound` 推进派生多个 `ADLDeleveragingRecord`

## 资金侧 MI

`资金侧 MI` 是专门承载资金占用、释放、划拨、结算、扣减、回充等事实的 `MI`。

判断重点不是“它是不是动了余额”，而是：

- 是否是一笔独立可审计的资金事实
- 是否需要回答“为何产生、何时成立、因谁而起、何时关闭”
- 是否需要 append-only 历史，而不是只看余额快照

典型例子：

- `FundHold`
- `InsuranceFundDrawdownEvent`
- `InsuranceFundTx(SURPLUS | REPLENISH)`
- `SettlementTransfer`

`available_balance`、`frozen_amount` 这类数值本身不是资金侧 `MI`。

## 审计凭证型 MI

`审计凭证型 MI` 指那些即使生命周期不长，也必须以“独立凭证”形式存在的事实对象。

它们往往用于：

- 争议处理
- 外部仲裁
- 财务核对
- 合规留痕
- 回放与责任归因

它们的核心价值不在“活得久”，而在“不能丢、不能改、必须能被引用”。

典型例子：

- `LiquidationExecution`
- `BankruptcyShortfallEvent`
- `InsuranceFundDrawdownEvent`
- `ADLDeleveragingRecord`

## 端到端 MI 链设计法

设计一条业务链时，不要只看 API 顺序，也不要只看 handler 调用图。

应该按以下顺序建模：

1. 先声明 `group_boundary`
2. 找到该边界内的 `main_mi`
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
4. 它是一次独立凭证，还是主 `MI` 内部的一步状态推进？
5. 它是否触发了新的授权、失败语义、审计语义、资金语义？
6. 它是否应该拆出独立 `use case group`，还是仍留在主链内部？

### 何时拆成独立 use case group

当一个次级对象同时满足以下大部分条件时，通常应考虑拆组：

1. 它有独立身份
2. 它有独立生命周期
3. 它有独立授权或失败语义
4. 它有独立审计意义
5. 业务会围绕它定义多种合法动作

如果它只是主 `MI` 内部推进的一步，或只是为了完成主链闭环而产生的一次性子事实，则通常不必单独升格为 group center。

## 最终 settled fact 的判定法

`final_settled_fact` 不是固定名词，也不等于“最后一次数据库更新”。

它是：

- 在声明的 `group_boundary` 内
- 这条业务真相链真正完成闭环后
- 对业务来说可被认定为“已经落定”的最终事实

判断方法：

1. 问“如果链条停在这里，业务会不会认为这件事已经真正办完？”
2. 问“此后还有没有必须继续推进才能成立的核心事实？”
3. 问“外部争议/对账时，是否可以把这里当作该边界内的最终结论？”

例子：

- 仅下单边界里，`Order accepted/rejected/cancelled/filled` 可能已是 settled fact
- `Order -> Trade -> Settlement` 的完整交易履约边界里，`Trade` 不是最终 settled fact，`Settlement` 才更接近
- 清算边界里，如果声明覆盖到“缺口被 IF 或 ADL 处理完成”，那么 `LiquidationExecution` 不是最终 settled fact，`shortfall closed / adl resolved` 才是

不要把“技术流程暂时结束”误判成 settled fact。

## 常见误判清单

以下对象通常不是 `MI`，除非领域明确赋予其独立业务真相语义：

- `CheckRisk`
- `ValidateInput`
- `PersistOrder`
- `LoadState`
- `PublishEvent`
- `MapReply`
- `matching_step`
- `db_row`
- `available_balance`
- `frozen_amount`（仅为数值字段时）
- `price`
- `quantity`
- `status enum value`

常见误判模式：

1. 把命令名当成 `MI`
2. 把状态字段当成 `MI`
3. 把技术步骤当成 `MI`
4. 把余额快照当成 `MI`
5. 把“内部可复用子步骤”误升格成独立 use case
6. 把“需要事务处理”误当成“必须是独立 MI”
7. 把“某张表存在”误当成“业务真相存在”

## 短例子

## 交易：`Order / Trade / Settlement`

- `Order` 常是主 `MI`
- `Trade` 常是次级 `MI`
- 如果边界只到撮合完成，`Trade` 可能已是该边界内 settled fact
- 如果边界声明覆盖完整履约，`Settlement` 才更接近最终 settled fact

不要因为 `Trade` 重要，就自动让它与 `Order` 共用同一个 group center。

## 清算：`Liquidation / Shortfall / IF / ADL`

这是一条典型的端到端 `MI` 链：

1. `LiquidationEvent` 成立
2. 派生 `LiquidationExecution / Fill`
3. 若执行价劣于破产价，派生 `BankruptcyShortfallEvent`
4. 若需基金兜底，产生 `InsuranceFundDrawdownEvent`
5. 若基金不足，产生 `ADLRound`
6. 批次内产生多个 `ADLDeleveragingRecord`
7. 必要时以 `InsuranceFundTx(REPLENISH)` 收尾

这里：

- `LiquidationEvent` 常是主 `MI`
- `Shortfall`、`ADLRound`、`ADLDeleveragingRecord` 常是必须独立留痕的次级或凭证型 `MI`
- `available_balance changed` 不是 `MI`
- `run_adl_step_3` 不是 `MI`

如果声明的 group boundary 覆盖“缺口被彻底处理完成”，则最终 settled fact 更接近：

- `shortfall fully covered`
- 或 `adl resolved / exhausted with explicit result`

而不是“刚触发强平”。

## 资金冻结：`Freeze / FundHold / Balance`

`Freeze` 这个词不能直接判定。

如果它只是订单提交流程里的一个内部动作，例如“下单时先冻结资产”，它常只是主链内部推进。

但若业务明确存在：

- 冻结编号
- 独立释放/消耗/过期
- 可争议、可审计、可查询
- 可被多个业务动作引用

那么更合理的建模常是把它提升为 `FundHold` 之类的资金侧 `MI`，而不是停留在“freeze 这个动作”。

`Balance` 或 `available_balance` 通常只是状态快照或派生值，不是 `MI`。

## 审计留痕校验问句

做完建模后，用这组问句自检：

1. 任意争议发生时，能否从主 `MI` 一路追到最终 settled fact？
2. 链上的关键因果事实，是否都以 append-only 凭证存在？
3. 是否有哪个对象只是状态字段，却被错误包装成 `MI`？
4. 是否有哪个必须独立留痕的资金事实，被偷懒折叠进余额快照？
5. 是否有哪个次级事实其实已具备独立身份与审计意义，却还被埋在主对象内部？

如果这些问题答不稳，说明 `MI` 边界大概率还没建对。

## 校准案例：强平链

现有强平案例可以作为本文件的校准样本，而不是主结构本身。

它帮助回答三件事：

1. 一条业务链不只会有一个 `MI`
2. 资金侧事实与业务事实需要并列建模
3. 最终 settled fact 必须落到“缺口是否被真正处理完成”，而不是停在中途状态

用一句话概括这条链：

`LiquidationEvent -> LiquidationExecution -> BankruptcyShortfallEvent -> InsuranceFundDrawdownEvent -> ADLRound -> ADLDeleveragingRecord -> final shortfall resolution`

这条链里的对象并不一定都属于同一个 `use case group`，但它们共同定义了一个完整的审计事实闭环。
