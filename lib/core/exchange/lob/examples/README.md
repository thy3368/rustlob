# 量化交易行情数据消费示例

## 概述

本示例展示了量化交易员如何使用 `MarketDataQueryProcessorImpl` 消费 L1/L2/L3 行情数据，实现真实的量化交易策略。

## 运行示例

```bash
cargo run --example quant_market_data_consumer
```

## 示例场景

### 场景 1: 高频交易策略 - 价差捕捉

**策略目标**: 监控 L1 最优买卖价，捕捉价差套利机会

**使用的数据层级**: Level 1 (BBO - Best Bid/Offer)

**策略逻辑**:
1. 查询 L1 数据获取最优买卖价
2. 计算价差 (Spread)
3. 判断价差是否满足最小阈值
4. 检查流动性是否充足
5. 生成交易信号

**代码示例**:
```rust
let hft_strategy = SpreadCaptureStrategy::new(symbol_id, 5, 50);
let query_l1 = QueryLevel1::new(symbol_id, 1000);
let result = processor.handle_query_level1(query_l1)?;
hft_strategy.analyze_level1(&result.snapshot);
```

**输出示例**:
```
=== 高频交易策略：价差捕捉 ===
交易对: 1
最优买价: 49999 (数量: 100)
最优卖价: 50001 (数量: 150)
价差: 2 (0.004%)
✅ 发现交易机会！
   建议操作: 买入 50 @ 49999, 卖出 50 @ 50001
   预期利润: 100
```

### 场景 2: 做市商策略 - 深度分析

**策略目标**: 分析 L2 深度数据，动态调整做市报价

**使用的数据层级**: Level 2 (Market Depth)

**策略逻辑**:
1. 查询 L2 数据获取多档深度
2. 计算买卖盘总量
3. 计算订单簿失衡度 (Imbalance)
4. 根据失衡度决定做市方向
5. 推荐最优报价位置

**代码示例**:
```rust
let mm_strategy = MarketMakingStrategy::new(symbol_id, 5, 0.2);
let query_l2 = QueryLevel2::depth_10(symbol_id, 1000);
let l2_result = processor.handle_query_level2(query_l2);
mm_strategy.analyze_level2(&l2_result.snapshot);
```

**输出示例**:
```
=== 做市商策略：深度分析 ===
买盘深度 (前5档):
  档位 1: 价格=50000, 数量=100, 订单数=1
  档位 2: 价格=49990, 数量=150, 订单数=2
  ...

卖盘深度 (前5档):
  档位 1: 价格=50001, 数量=150, 订单数=1
  ...

订单簿分析:
  买盘总量: 1000
  卖盘总量: 1050
  失衡度: -2.44%
✅ 卖盘压力大，建议在买方挂单做市
   推荐价格: 50000 (买一价)
```

### 场景 3: 大单追踪策略 - L3 订单分析

**策略目标**: 监控 L3 订单簿，识别大额订单和机构行为

**使用的数据层级**: Level 3 (Full Order Book)

**策略逻辑**:
1. 查询 L3 数据获取完整订单簿
2. 识别大额订单 (超过阈值)
3. 分析大单分布
4. 判断市场方向
5. 生成跟单信号

**代码示例**:
```rust
let large_order_strategy = LargeOrderTrackingStrategy::new(symbol_id, 100);
let query_l3 = QueryLevel3::new(symbol_id, 1000);
let l3_result = processor.handle_query_level3(query_l3);
large_order_strategy.analyze_level3(&l3_result.snapshot);
```

**输出示例**:
```
=== 大单追踪策略：L3 订单分析 ===
大额买单 (>= 100):
  订单ID: 10000, 价格: 50000, 数量: 100
  订单ID: 10001, 价格: 49990, 数量: 120
  ...

大额卖单 (>= 100):
  订单ID: 20000, 价格: 50001, 数量: 150
  ...

大单统计:
  大额买单数量: 5
  大额卖单数量: 3
✅ 买方大单占优，市场可能上涨
```

### 场景 4: 增量数据实时监控

**策略目标**: 实时监控订单簿变更、成交和价格变动

**使用的数据类型**: 增量数据 (Incremental Data)

**策略逻辑**:
1. 查询增量数据 (指定序列号范围)
2. 处理订单簿变更事件 (Add/Modify/Delete)
3. 处理成交事件
4. 处理最优买卖价变更事件
5. 统计市场活跃度

**代码示例**:
```rust
let mut monitor = IncrementalDataMonitor::new(symbol_id, 1000);
let query_incremental = QueryIncrementalData::new(symbol_id, 1000, 1005);
let result = processor.handle_query_incremental_data(query_incremental)?;
monitor.process_incremental_data(&result);
```

**输出示例**:
```
=== 增量数据实时监控 ===
序列号范围: 1000 -> 1005
事件数量: 5

📝 订单簿变更:
  序列号: 1001
  变更类型: Add
  订单ID: 30001
  价格: 49998
  数量: 200
  ✅ 新增订单

💰 成交事件:
  序列号: 1002
  成交ID: 5001
  成交价: 50000
  成交量: 50
  成交额: 2500000

📊 最优买卖价变更:
  序列号: 1004
  最优买价: 49998 (数量: 200)
  最优卖价: 50001 (数量: 150)
  价差: 3

统计信息:
  累计成交次数: 1
  累计订单簿变更: 3
```

### 场景 5: 综合策略决策

**策略目标**: 结合多层级数据做出综合交易决策

**使用的数据层级**: L1 + L2 + L3 + 增量数据

**策略逻辑**:
1. L1 数据: 判断价差是否满足条件
2. L2 数据: 分析订单簿失衡度
3. L3 数据: 识别大单支撑/压力位
4. 增量数据: 评估市场活跃度
5. 综合分析生成最终决策

## 核心组件

### 1. MarketDataQueryProcessorImpl

行情查询处理器，负责处理所有行情查询请求。

**初始化**:
```rust
let processor = MarketDataQueryProcessorImpl::new(
    snapshot_repo,      // 快照数据仓储
    incremental_repo,   // 增量数据仓储
);
```

### 2. 数据仓储接口

**快照数据仓储**:
- `Level1SnapshotRepo`: 提供 L1 快照
- `Level2SnapshotRepo`: 提供 L2 快照
- `Level3SnapshotRepo`: 提供 L3 快照

**增量数据仓储**:
- `IncrementalDataRepo`: 提供增量数据查询

### 3. 查询命令

**快照查询**:
- `QueryLevel1`: 查询单个交易对的 L1 数据
- `QueryLevel1Batch`: 批量查询多个交易对的 L1 数据
- `QueryLevel2`: 查询指定深度的 L2 数据
- `QueryLevel3`: 查询完整的 L3 订单簿

**增量查询**:
- `QueryIncrementalData`: 查询指定序列号范围的增量数据

### 4. 查询结果

**快照结果**:
- `Level1QueryResult`: L1 查询结果
- `Level2QueryResult`: L2 查询结果
- `Level3QueryResult`: L3 查询结果

**增量结果**:
- `IncrementalDataResult`: 增量数据查询结果
  - `deltas`: 增量事件列表
  - `has_more`: 是否还有更多数据

### 5. 增量事件类型

**订单簿变更**:
```rust
OrderBookDelta {
    change_type: OrderBookChangeType,  // Add/Modify/Delete
    order_id: OrderId,
    price: Price,
    quantity: Quantity,
    ...
}
```

**成交事件**:
```rust
TradeEvent {
    trade_id: u64,
    buyer_order_id: OrderId,
    seller_order_id: OrderId,
    price: Price,
    quantity: Quantity,
    aggressor_side: Side,
    ...
}
```

**最优买卖价变更**:
```rust
BboChangeEvent {
    best_bid: Option<Price>,
    best_bid_quantity: Quantity,
    best_ask: Option<Price>,
    best_ask_quantity: Quantity,
    ...
}
```

## 实际应用场景

### 1. 高频交易 (HFT)
- 使用 L1 数据进行毫秒级决策
- 捕捉微小价差套利机会
- 要求极低延迟

### 2. 做市商 (Market Making)
- 使用 L2 数据分析市场深度
- 动态调整买卖报价
- 管理库存风险

### 3. 算法交易 (Algo Trading)
- 使用 L3 数据识别大单
- TWAP/VWAP 执行算法
- 冰山订单检测

### 4. 量化研究 (Quant Research)
- 使用增量数据进行回测
- 分析订单流
- 构建预测模型

## 性能优化建议

### 1. 数据层级选择
- **L1**: 最快，适合高频策略
- **L2**: 中等，适合做市策略
- **L3**: 最慢，适合深度分析

### 2. 批量查询
```rust
// 批量查询多个交易对，减少网络往返
let query_batch = QueryLevel1Batch::new(vec![1, 2, 3], sequence);
let result = processor.handle_query_level1_batch(query_batch);
```

### 3. 增量数据分页
```rust
// 分页查询增量数据，避免一次加载过多
let query = QueryIncrementalData::new(symbol_id, from_seq, to_seq);
let result = processor.handle_query_incremental_data(query)?;

if result.has_more {
    // 继续查询下一页
}
```

### 4. 缓存策略
- 缓存 L1 数据用于快速访问
- 定期刷新 L2/L3 数据
- 增量数据实时更新

## 扩展示例

### 自定义策略实现

```rust
struct MyCustomStrategy {
    // 策略参数
}

impl MyCustomStrategy {
    fn analyze(&self, l1: &Level1, l2: &Level2<10>, l3: &Level3) {
        // 1. 分析 L1 数据
        let spread = l1.best_ask.unwrap() - l1.best_bid.unwrap();

        // 2. 分析 L2 深度
        let bid_volume = calculate_volume(&l2.bids);
        let ask_volume = calculate_volume(&l2.asks);

        // 3. 分析 L3 大单
        let large_orders = find_large_orders(&l3);

        // 4. 生成交易信号
        if spread < threshold && bid_volume > ask_volume {
            println!("✅ 买入信号");
        }
    }
}
```

## 注意事项

1. **序列号管理**: 确保增量数据查询的序列号连续性
2. **错误处理**: 处理查询失败和数据缺失情况
3. **性能监控**: 监控查询延迟和吞吐量
4. **内存管理**: 及时释放不再使用的数据

## 相关文档

- [行情数据接口设计](../src/lob/domain/service/trading_market_data_bp.rs)
- [查询处理器实现](../src/lob/domain/service/trading_market_data_bp_impl.rs)
- [Level 1-3 数据类型](../src/lob/domain/entity/level_types.rs)
