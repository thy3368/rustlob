# Hyperliquid 逆向回构文件级设计

## 目的

本文档描述 `proc/operating/dex/dex` crate 在面向 Hyperliquid 逆向回构时，建议采用的文件级演进结构。

目标不是一次性建完全部模块，而是明确：
- 现有文件各自的职责
- 后续新增文件应该落在哪一层
- 交易域与行情域如何解耦

## 设计原则

- 交易域是状态真相来源
- 行情域是交易执行结果的投影层
- `cmd_handler` 是接入层，不是完整领域层
- 新类型优先进入语义清晰的模块，而不是继续堆在 handler 中
- `CLAUDE.md` 保持短小，只做入口导航，详细分析放独立文档

---

## 一、建议目录结构

```text
proc/operating/dex/dex/
├── CLAUDE.md
├── Cargo.toml
├── docs/
│   ├── hyperliquid_reverse_usecases.md
│   └── hyperliquid_file_design.md
└── src/
    ├── lib.rs
    │
    ├── cmd_handler/
    │   ├── mod.rs
    │   ├── trading_command.rs
    │   ├── submit_trading_command_handler.rs
    │   └── execute_trading_batch_handler.rs
    │
    ├── trading/
    │   ├── mod.rs
    │   ├── order.rs
    │   ├── order_id.rs
    │   ├── order_book.rs
    │   ├── execution.rs
    │   ├── matching.rs
    │   ├── state.rs
    │   └── liquidation.rs
    │
    ├── market_data/
    │   ├── mod.rs
    │   ├── book_delta.rs
    │   ├── trade_event.rs
    │   ├── ticker.rs
    │   ├── candle.rs
    │   └── private_stream.rs
    │
    ├── events/
    │   ├── mod.rs
    │   ├── trading_event.rs
    │   ├── market_event.rs
    │   └── event_bus.rs
    │
    └── types/
        ├── mod.rs
        ├── market.rs
        ├── price.rs
        ├── quantity.rs
        ├── timestamp.rs
        └── trader.rs
```

---

## 二、现有目录职责

### 1. `src/cmd_handler/`

职责：交易命令接入层。

#### `trading_command.rs`
负责定义交易动作协议：
- `PlaceOrderCmd`
- `CancelOrderCmd`
- `AmendOrderCmd`
- `TradingCommand`
- `TradingCommandEnvelope`

这里是“系统接受什么交易动作”的入口边界。

#### `submit_trading_command_handler.rs`
负责提交阶段：
- 预校验
- 状态加载
- 变更收集
- changelog hook
- pending queue 入队

它适合处理 admission / queueing，不适合承载订单簿内部逻辑。

#### `execute_trading_batch_handler.rs`
负责批处理执行入口。

当前只做命令计数，但从职责上它应成为后续：
- 批量执行交易命令
- 调用交易域逻辑
- 收集执行结果与事件

的连接点。

---

## 三、建议新增目录职责

### 2. `src/trading/`

职责：交易用例域核心。

#### `order.rs`
定义订单实体与订单状态。

建议容纳：
- order id
- trader id
- market
- side
- price
- quantity
- remaining quantity
- status

#### `order_id.rs`
单独承载订单标识语义，避免未来所有地方都裸用 `u64`。

#### `order_book.rs`
定义订单簿结构与基础操作：
- 插入订单
- 删除订单
- 更新订单
- 查询 best bid / ask
- 管理 price level

#### `matching.rs`
封装撮合算法与成交生成逻辑。

#### `execution.rs`
统一处理：
- place
- cancel
- amend
- 后续 liquidation

把交易命令转成明确的执行结果和事件。

#### `state.rs`
定义市场级或引擎级状态聚合容器。

#### `liquidation.rs`
预留清算路径，不必一开始实现完整清算，但应留出清晰边界。

---

### 3. `src/market_data/`

职责：行情投影层。

#### `book_delta.rs`
定义盘口增量事件。

#### `trade_event.rs`
定义逐笔成交事件。

#### `ticker.rs`
定义 ticker 聚合结果。

#### `candle.rs`
定义 K 线聚合结果。

#### `private_stream.rs`
定义用户私有流事件：
- 订单更新
- 自成交/用户成交
- 仓位与余额变化（后续）

原则是：行情层只消费执行结果，不直接修改交易状态。

---

### 4. `src/events/`

职责：统一事件边界。

#### `trading_event.rs`
承载交易域输出事件，例如：
- OrderAccepted
- OrderCancelled
- OrderAmended
- TradeExecuted
- LiquidationTriggered

#### `market_event.rs`
承载行情域输出事件，例如：
- BookDelta
- PublicTrade
- TickerUpdated
- CandleClosed
- UserOrderUpdate

#### `event_bus.rs`
如果后续需要在 crate 内部组织发布/订阅，可先提供最小事件分发抽象。

注意：先定义事件类型，比先做复杂总线更重要。

---

### 5. `src/types/`

职责：共享语义类型层。

#### `market.rs`
市场标识，如 `BTC-PERP`。

#### `price.rs`
价格类型。

#### `quantity.rs`
数量类型。

#### `timestamp.rs`
统一时间戳类型。

#### `trader.rs`
交易者标识类型。

目标是逐步减少代码里无语义的裸 `u64` 和 `String`。

---

## 四、`lib.rs` 的角色

建议 `src/lib.rs` 保持很薄，只作为模块导出入口：

```rust
pub mod cmd_handler;
pub mod trading;
pub mod market_data;
pub mod events;
pub mod types;
```

不要把真正业务逻辑堆进 `lib.rs`。

---

## 五、主链路映射

建议未来把 crate 主链稳定成：

```text
TradingCommandEnvelope
-> SubmitTradingCommandHandler
-> ExecuteTradingBatchHandler
-> trading::execution
-> events::trading_event
-> market_data::*
```

这条链路的含义是：

- `cmd_handler` 负责接入
- `trading` 负责状态变化
- `events` 负责边界表达
- `market_data` 负责对外投影

---

## 六、CLAUDE.md 的引用方式

建议在 `CLAUDE.md` 中新增一节：

## Reverse-architecture references

- `docs/hyperliquid_reverse_usecases.md`：按用例域拆分 Hyperliquid 风格交易/行情分析
- `docs/hyperliquid_file_design.md`：建议的文件级演进结构

并明确说明：

- 先读 `CLAUDE.md` 了解 crate 入口与工作方式
- 需要理解 Hyperliquid 风格目标时读 `hyperliquid_reverse_usecases.md`
- 需要新增模块或重构目录时读 `hyperliquid_file_design.md`
