# clean architecture 之：Trace 追行为调用链，Event 追实体溯源心法

很多团队会把 `trace` 和 `event` 混在一起谈，最后导致两类能力都做得不对：

- 一类是把业务事件当成链路追踪来用，结果只能看到“发生了什么”，看不到“是谁调了谁”。
- 另一类是把调用日志当成实体历史来存，结果只能看到“调用经过了哪些模块”，看不到“实体为什么变成现在这个状态”。

如果用 clean architecture 的语言讲，这两个概念应该分开：

> `trace` 可观测，是 `inbound adapter -> use_case -> outbound adapter` 的调用链追溯，也是行为追溯。  
> `event` 溯源，是 `entity` 的状态演进记录，也是实体溯源。

这篇文章只讲一件事：为什么 `trace` 和 `event` 不是一回事，以及系统里应该分别把它们放在哪里。

## 一句话先讲清

`trace` 关注的是一次请求的执行路径和执行行为。

- 请求从哪里进来
- 经过了哪个 `use_case`
- 调用了哪些 `outbound adapter`
- 卡在了哪里
- 哪一跳慢了
- 哪个外部依赖失败了

`event` 关注的是一个实体的状态历史。

- 这个实体创建于何时
- 被谁修改过
- 发生了哪些业务动作
- 为什么从状态 A 变成状态 B
- 当前状态是否可以由历史回放重建

所以：

- `trace` 回答“这次调用怎么走的，这个行为是怎么执行的”
- `event` 回答“这个实体怎么变成这样的”

## 用 clean architecture 重新摆正位置

先把角色摆清楚。

## Core

### use_case

`use_case` 是业务动作的编排者。它接收 `command/query`，执行业务规则，决定要不要调用外部端口。

### entity

`entity` 是核心业务规则和不变量的承载者。它不关心 HTTP、Kafka、DB、OpenTelemetry，也不关心 controller/service/repository 这些实现命名。

## Adapter

### inbound

`inbound adapter` 负责把外部输入翻译成 `use_case` 能理解的输入，比如：

- HTTP 请求
- WebSocket 消息
- MQ 消费消息
- CLI 命令

### outbound

`outbound adapter` 负责实现 `use_case` 定义的 port，把核心动作送到外部机制，比如：

- 数据库持久化
- 外部交易所 API
- 消息发布
- 缓存
- 推送

## Infra

`infra` 是框架、SDK、驱动、runtime、数据库、OpenTelemetry、消息中间件这类外部机制。

## 三种视图必须分开看

很多混乱不是因为代码复杂，而是因为把三种视图混成了一种。

### 1. role view

```text
core
  use_case
  entity

adapter
  inbound
  outbound

infra
  http framework / db driver / mq sdk / tracing sdk
```

### 2. source dependency view

```text
inbound -> use_case -> entity
outbound -> port <- use_case
outbound -> infra
```

### 3. call flow view

```text
inbound -> use_case -> outbound -> infra
```

`trace` 属于第三种视图，也就是 `call flow view`。  
`event` 属于 `entity` 的状态历史，不属于调用链本身。

这句话非常关键。

## Trace 到底是什么

`trace` 不是业务事实，它是一次执行过程的观测数据。

它通常长这样：

- `trace_id`
- `span_id`
- parent span
- service/module name
- method/use case name
- start time
- duration
- status
- error
- tags / attributes

它的核心目标不是“保存业务真相”，而是“让你能追请求、追行为”。

比如一个下单请求：

```text
HTTP POST /orders
  -> PlaceOrderInboundAdapter
  -> PlaceOrderUseCase
  -> AccountRepoPort
  -> OrderRepoPort
  -> MatchingGatewayPort
  -> EventPublisherPort
```

这条链里，`trace` 关心的是：

- 请求是否进入了 `PlaceOrderUseCase`
- `AccountRepoPort` 是否超时
- `MatchingGatewayPort` 是否失败
- 失败发生在第几个 span
- 整体耗时是不是超了目标延迟

也就是说，`trace` 追的是 call 和 behavior，不是 state。

这里的 `behavior` 不是领域实体内部的不变量本身，而是一次业务动作在系统分层中的执行经过，比如：

- 请求进入了哪个 `inbound adapter`
- 命中了哪个 `use_case`
- 调用了哪些 `outbound adapter`
- 哪一步返回了错误
- 哪个外部依赖拖慢了整体响应

所以在 clean architecture 里，`trace` 最适合挂在这些位置：

- `inbound adapter` 建立入口 span
- `use_case` 建立核心业务 span
- `outbound adapter` 建立外部调用 span

不应该让 `entity` 依赖 tracing SDK。  
因为 `entity` 不是为可观测框架服务的，它是为业务规则服务的。

## Event 到底是什么

`event` 不是为了看调用链，而是为了表达业务事实。

例如订单实体可能有这些事件：

- `OrderCreated`
- `OrderAccepted`
- `OrderPartiallyFilled`
- `OrderFilled`
- `OrderCanceled`

账户实体可能有这些事件：

- `BalanceDebited`
- `BalanceCredited`
- `MarginLocked`
- `MarginReleased`

这些事件的意义不是“某个函数被调用了”，而是：

- 某个实体确实发生了业务状态变化
- 这个变化可以被记录
- 这个变化可以被回放
- 这个变化可以解释当前状态

所以 `event` 回答的是：

- 订单为什么变成 `Filled`
- 账户为什么少了 100 USDT
- 某次撮合之后仓位为什么变了

也就是说，`event` 追的是 entity state transition，不是 call transition。

## 一个下单例子，把两者彻底分开

假设用户提交一笔限价买单。

### 从 trace 视角看

你想知道的是：

1. HTTP 请求有没有进系统
2. `PlaceOrderUseCase` 有没有被执行
3. 风控检查花了多久
4. 持久化有没有成功
5. 撮合网关有没有调用
6. MQ 发布有没有失败

这是调用链问题。

### 从 event 视角看

你想知道的是：

1. 订单何时创建
2. 是否通过校验
3. 是否冻结了余额
4. 是否进入订单簿
5. 是否发生部分成交
6. 是否最终成交或撤销

这是实体历史问题。

同一个“下单动作”，会同时产生两种数据：

- 一份 `trace`，用于看这次请求怎么执行、这个行为经过了哪些调用链
- 一组 `event`，用于看订单实体怎么演化

它们相关，但不是同一个东西。

## 为什么很多系统会把它们做混

常见有四种混法。

### 1. 用 event 当 trace

做法是每调用一个模块就发一个“事件”。

问题是这些东西不是实体状态变化，只是函数经过点。它会让事件流变成技术流水账，污染领域语义。

### 2. 用 trace 当审计历史

做法是把 span/log 存下来，想靠它恢复订单或账户状态。

问题是 trace 天生不是稳定的业务事实。采样、丢日志、字段变化、重试折叠，都会让它不适合做实体真相来源。

### 3. 把 tracing 放进 entity

做法是在实体方法里直接打链路 span、埋监控 SDK。

问题是外部机制污染了 `core.entity`，破坏依赖方向。`entity` 不应该知道 OpenTelemetry、Jaeger、Zipkin、Sentry。

### 4. 把 event 放在 adapter 层才定义

做法是消息格式先有了，领域事件后补。

问题是这样得到的往往是“集成事件”或“传输事件”，不是实体事件。结果领域模型退化成 DTO 流转。

## 正确的边界应该是什么

可以用一句很硬的判断标准：

如果数据是为了定位一次调用经过了谁、慢在了哪、错在了哪，以及这个行为是如何执行的，它是 `trace`。  
如果数据是为了说明一个实体发生了什么变化、为什么变、能否回放重建，它是 `event`。

进一步落到分层上：

- `trace` 的重点落在 `inbound adapter`、`use_case`、`outbound adapter`
- `event` 的重点落在 `entity`
- `use_case` 负责驱动实体产生事件，并决定是否通过 port 发布
- `outbound adapter` 可以把事件投递到 MQ / EventStore / CDC / Push 通道
- `infra` 提供 tracing SDK、消息系统、存储系统

## 最小实现建议

如果你现在要在系统里同时做好 `trace` 和 `event`，不需要一上来搞很重。

先做最小闭环：

### 对 trace

1. 在每个 `inbound adapter` 生成或接入 `trace_id`
2. 进入 `use_case` 时建立核心 span
3. 调每个 `outbound adapter` 时建立子 span
4. 统一记录耗时、错误、外部依赖名、业务主键

业务主键建议带上：

- `user_id`
- `account_id`
- `order_id`
- `symbol`

但这些是 trace attribute，不等于 event 本身。

### 对 event

1. 让 `entity` 在状态变化时产出领域事件
2. 让 `use_case` 收集本次执行产生的事件
3. 先持久化实体状态，或持久化事件流
4. 再由 `outbound adapter` 把事件发布出去

如果系统以后要做：

- 审计
- 对账
- 状态回放
- 异步投影
- CQRS
- 订阅推送

那么事件这条线会非常值钱。

## 一张图收尾

```text
外部请求
  -> inbound adapter
  -> use_case
  -> entity
  -> outbound adapter
  -> infra

trace: 追这次请求跨层调用怎么走，追业务行为如何执行
event: 追 entity 状态如何演进
```

再压缩成一句话：

> `trace` 是 `inbound adapter`、`use_case`、`outbound adapter` 的调用链追溯，是可观测，也是行为追溯。  
> `event` 是 `entity` 的状态演进记录，是实体溯源。

如果这两个概念不分开，最后你既做不好可观测，也做不好领域建模。  
一旦分开，系统的诊断能力和业务表达能力都会明显上一个台阶。
