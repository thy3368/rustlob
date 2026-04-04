# Hyperliquid 逆向回构用例分析

## 目的

本文档用于给 `proc/operating/dex/dex` crate 提供一个面向 Hyperliquid 的逆向回构视角。
重点不是声称当前 crate 已经实现这些能力，而是明确：

- 当前代码已经处于哪一层
- 后续应优先补哪些用例
- 交易域与行情域应该如何拆分

当前 `dex` crate 还只是一个“交易命令接入 + 批处理执行骨架”，并不是完整撮合引擎或完整 DEX。

## 分析范围

本文档只拆两个核心用例域：

- 交易用例域
- 行情用例域

暂时不展开以下主题的完整设计，只在需要时作为上下文提及：

- 保证金与账户域
- 结算域
- 共识/排序域
- HyperEVM 域

## 已验证的 Hyperliquid 背景

附近文档能确认的高层信息包括：

- `../hyperliquid/README.md` 说明 HyperCore 包含 fully onchain 的现货和永续订单簿。
- 同一文档明确把 `order / cancel / trade / liquidation` 视为核心状态跃迁。
- `../hyperliquid/trading.md` 展示了交易相关主题范围，包括 order book、order types、margining、liquidations、funding、market making 等。

本文档只把这些信息作为逆向目标，不假设 Hyperliquid 的未公开内部实现。

## 用例建模原则

本 crate 采用“用例 -> command/query -> 测试”的建模方式：

- 会改变状态的用例，建模为 Command
- 只读取状态的用例，建模为 Query
- 验证某个用例是否成立，优先通过测试对应的 Command / Query 完成

换句话说，每个业务用例都应落到一个明确的 command 或 query 接口上，并由对应测试作为验收标准。

### 当前与建议的 command/query 映射

**当前已存在的 Command：**
- `PlaceOrderCmd`
- `CancelOrderCmd`
- `AmendOrderCmd`

**后续建议补充的 Command：**
- `ExecuteTradingBatchCmd` 或等价批处理执行入口
- `LiquidatePositionCmd`（未来清算路径明确后再引入）

**后续建议补充的 Query：**
- `GetOrderBookQuery`
- `GetRecentTradesQuery`
- `GetTickerQuery`
- `GetUserOpenOrdersQuery`
- `GetUserFillsQuery`

---

## 一、交易用例域

### 核心问题

交易域回答的是：

> 用户或系统如何改变交易状态？

它是一个命令驱动的领域。起点是一个明确动作，终点是状态变化和一组事件输出。

### 核心角色

- 交易用户
- 批处理执行器 / 撮合执行器
- 风控/清算触发器
- 未来的账户与保证金子系统

### 主要用例

#### 1. 发单（Place Order）

**目标：**
提交买单或卖单，并进入订单簿或立即成交。

**当前 crate 中的映射：**
- `src/cmd_handler/trading_command.rs` 中的 `PlaceOrderCmd`
- `TradingCommand::PlaceOrder`
- `TradingCommandEnvelope`

**后续期望能力：**
- 校验市场、方向、价格、数量
- 接入排队
- 进入订单簿或触发撮合
- 产出订单更新与成交事件

#### 2. 撤单（Cancel Order）

**目标：**
撤销一个仍可取消的订单。

**当前 crate 中的映射：**
- `CancelOrderCmd`
- `TradingCommand::CancelOrder`

**后续期望能力：**
- 定位订单
- 校验订单所有权和可撤销状态
- 从订单簿中移除剩余挂单量
- 产出撤单事件与簿变化事件

#### 3. 改单（Amend Order）

**目标：**
修改已有订单的价格和/或数量。

**当前 crate 中的映射：**
- `AmendOrderCmd`
- `TradingCommand::AmendOrder`

**后续期望能力：**
- 校验改单合法性
- 明确改单语义是原地修改还是 cancel-replace
- 必要时重排优先级
- 产出改单事件与簿变化事件

#### 4. 成交（Trade Execution）

**目标：**
让可撮合的订单形成成交。

**当前 crate 中的映射：**
- `src/cmd_handler/execute_trading_batch_handler.rs` 目前只做命令计数
- 还没有真实撮合逻辑

**后续期望能力：**
- 执行撮合
- 计算成交价、成交量、maker/taker
- 更新订单状态
- 产生成交事件与下游行情事件

#### 5. 强平/清算（Liquidation）

**目标：**
在风险条件触发时强制平仓或接管头寸。

**当前 crate 中的映射：**
- 尚未建模为命令或专门处理器

**后续期望能力：**
- 接入未来风险/保证金子系统
- 触发强平执行路径
- 产出清算相关交易事件和行情事件

### 交易域主链路

建议按以下主链理解当前 crate 的未来方向：

```text
TradingCommandEnvelope
-> 提交校验 / 入队
-> 批处理执行
-> 交易状态变化
-> 交易事件
```

### 当前边界

当前 `dex` 实际只清晰覆盖了前两层：

- 命令建模
- 命令提交/排队骨架

尚未覆盖：

- 订单簿状态
- 订单生命周期状态机
- 撮合逻辑
- 成交生成
- 清算路径

---

## 二、行情用例域

### 核心问题

行情域回答的是：

> 交易状态变化后，如何向外投影为公共或私有数据流？

行情域不应成为交易状态的来源，而应是交易执行结果的投影层。

### 核心角色

- 公共行情订阅者
- 用户私有流订阅者
- 前端界面
- 做市或量化策略

### 主要用例

#### 1. 订单簿深度（Order Book / L2）

**目标：**
发布市场盘口状态，支持快照和增量。

**来源：**
- 发单
- 撤单
- 改单
- 成交后的剩余量变化
- 清算导致的订单簿变化

**期望输出：**
- snapshot
- delta
- sequence / timestamp

#### 2. 逐笔成交（Trade Tape）

**目标：**
发布每一笔真实成交。

**来源：**
- 交易域撮合结果

**期望输出：**
- market
- price
- quantity
- aggressor side
- timestamp

#### 3. Ticker

**目标：**
发布市场摘要行情。

**来源：**
- 最优买卖盘
- 成交流
- 滚动聚合统计

**期望输出：**
- last price
- bid / ask
- mid
- rolling volume
- rolling change

#### 4. Kline / Candle

**目标：**
把成交流按时间窗口聚合成 OHLCV。

**来源：**
- trade events

**期望输出：**
- open / high / low / close / volume
- interval bucket

#### 5. 用户私有流（Private Stream）

**目标：**
发布只属于某个用户的状态变化。

**来源：**
- 订单状态变化
- 用户自己的成交
- 未来的仓位 / 余额变化
- 未来的强平通知

**期望输出：**
- order update
- own trade
- account / position delta

### 行情域主链路

```text
交易状态变化
-> 交易事件
-> 行情投影
-> 公共流 / 私有流输出
```

### 当前边界

当前 crate 还没有独立的 `market_data` 模块，这是正常的。
重要的是后续要坚持这个原则：

- 行情是交易结果的投影
- 不要把行情逻辑直接塞进命令处理器里
- 不要让行情层反向主导交易状态

---

## 三、当前代码映射

### `src/cmd_handler/trading_command.rs`

这是当前交易动作协议边界。
任何新的交易动作，应该先明确是否真的属于交易命令，再决定是否扩展这里。

### `src/cmd_handler/submit_trading_command_handler.rs`

这是命令接入层，负责：

- pre-check
- state load
- change collection
- changelog hooks
- pending queue 入队

这里适合放提交阶段逻辑，不适合直接放完整撮合逻辑。

### `src/cmd_handler/execute_trading_batch_handler.rs`

这是当前最自然的“执行阶段边界”。
后续应在这里接入真正的 trading execution，而不是继续停留在计数逻辑。

### `src/types/mod.rs`

当前几乎为空。
后续如果引入语义更强的市场、价格、数量、时间戳类型，这里是自然落点。

---

## 四、后续演进建议

如果要把当前 `dex` 向 Hyperliquid 风格回构推进，建议优先顺序是：

1. 先补交易域闭环  
   从“能收命令”进化到“能产生真实执行结果”

2. 再补事件层  
   明确 trading event / market event

3. 再补行情投影层  
   从交易事件派生 book delta / trade event / private stream

4. 最后补高阶能力  
   如 liquidation、margining、funding 等
