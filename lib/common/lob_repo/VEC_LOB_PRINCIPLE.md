# Rust之从0-1低时延CEX：Vec 订单薄（LOB）实现原理详解

## 目录
1. [概述](#概述)
2. [数据结构](#数据结构)
3. [核心原理](#核心原理)
4. [关键算法](#关键算法)
5. [性能分析](#性能分析)
6. [应用示例](#应用示例)

## 概述

Vec订单薄是一个基于向量（Vec）的高性能订单簿实现，采用**分层索引**和**链表聚合**的混合方法，实现O(1)的订单查找和O(k)的高效匹配。

### 设计目标
- **低延迟**：在关键路径上优化时间复杂度
- **内存效率**：预分配的Vec避免动态扩容
- **快速查找**：O(1)的订单ID查询
- **高效匹配**：避免遍历所有价格级别

---

## 数据结构

### 1. PricePoint（价格点）

```rust
struct PricePoint {
    first_order_idx: Option<usize>,  // 该价格级别第一个订单的索引
    last_order_idx: Option<usize>    // 该价格级别最后一个订单的索引
}
```

**职责**：
- 作为某个价格级别订单链表的**头尾指针**
- 支持O(1)的链表首尾访问
- 无需存储完整链表，只需两个指针

**示例**：
```
价格 100.00 的买单价格点：
PricePoint {
    first_order_idx: 5,    // 第5个订单是此价格级别的第一个
    last_order_idx: 12     // 第12个订单是此价格级别的最后一个
}

订单链表: Order[5] -> Order[7] -> Order[9] -> Order[12]
```

### 2. OrderNode（订单节点）

```rust
struct OrderNode<O: Order> {
    order: O,                    // 实际的订单对象
    next_idx: Option<usize>      // 指向同价格级别下一个订单的索引
}
```

**职责**：
- 包装`Order`对象
- 通过`next_idx`形成同价格级别的链表
- 支持时间优先原则（FIFO）

**链表结构**：
```
同一价格级别的订单链表（按时间顺序）：
┌─────────────────────┐
│ OrderNode[5]        │
│ order: Order1       │
│ next_idx: Some(7)   │ ──→ ┌─────────────────────┐
└─────────────────────┘     │ OrderNode[7]        │
                             │ order: Order2       │
                             │ next_idx: Some(9)   │ ──→ ...
                             └─────────────────────┘
```

### 3. LocalLob（主结构）

```rust
pub struct LocalLob<O: Order> {
    // 配置
    symbol: Symbol,              // 交易对（如 BTCUSDT）
    tick_size: Price,            // 最小价格变动单位

    // 核心索引结构
    bids: Vec<PricePoint>,       // 买单价格点数组（索引 = price / tick_size）
    asks: Vec<PricePoint>,       // 卖单价格点数组（索引 = price / tick_size）
    orders: Vec<Option<OrderNode<O>>>,  // 订单存储池
    order_index: HashMap<OrderId, usize>,  // 订单ID -> 存储池索引映射

    // 缓存
    bid_max: Option<Price>,      // 最佳买价缓存
    ask_min: Option<Price>,      // 最佳卖价缓存
    last_trade_price: Option<Price>,  // 最后一笔成交价

    // 管理
    next_slot: usize,            // 下一个可用的槽位索引
}
```

**数据层次关系**：
```
┌─ Tick索引 ─────────────────────┐
│  Index: price / tick_size       │
│                                  │
│  bids[i] ──→ PricePoint         │
│              - first_order_idx   │
│              - last_order_idx    │
│                     ↓             │
│  orders[idx] ──→ OrderNode      │
│                 - order          │
│                 - next_idx ──→   │
│                 OrderNode        │
└──────────────────────────────────┘
```

---

## 核心原理

### 1. 价格到索引的映射

**公式**：
```
tick_index = price.raw() / tick_size.raw()
```

**设计优势**：
- **连续映射**：价格自动映射到数组索引，无需哈希计算
- **支持多精度**：不同交易对可配置不同tick_size
  - BTC/ETH：tick_size = 0.01
  - DOGE：tick_size = 0.0001
  - SHIB：tick_size = 0.00000001

**示例**：
```
BTCUSDT (tick_size = 0.01):
  Price 50000.00 → Index: 5000000
  Price 50000.01 → Index: 5000001
  Price 50000.02 → Index: 5000002

内存布局：
bids: [PricePoint, PricePoint, ..., PricePoint(50000.00), ..., PricePoint(50000.02)]
       0         1                5000000              5000001          5000002
```

### 2. 三层索引结构

```
第一层：Tick数组
    bids[tick_idx] / asks[tick_idx]
           ↓
第二层：价格点链表头尾
    PricePoint.first_order_idx / last_order_idx
           ↓
第三层：订单存储和链接
    orders[idx] → OrderNode { order, next_idx }
           ↓
第四层：订单ID映射
    order_index[OrderId] → idx
```

**查询流程**：
```
Query: 找订单 order_id=123

Step 1: order_index.get(123) → idx=5
Step 2: orders[5] → Some(OrderNode)
Step 3: OrderNode.order → Order对象

总耗时: O(1)
```

### 3. 同价格级别订单链表

**设计**：
- 每个`PricePoint`维护该价格级别所有订单的链表
- 链表节点存储在`orders` Vec中
- 通过`next_idx`指针形成链表

**插入流程**：
```
add_order(order, price=100.00):

Step 1: 获取当前链表尾部
    price_point = get_price_point(100.00)
    last_idx = price_point.last_order_idx

Step 2: 分配新槽位
    new_idx = next_slot
    orders[new_idx] = Some(OrderNode::new(order))
    next_slot += 1

Step 3: 链接到链表
    if last_idx.is_some() {
        orders[last_idx].next_idx = Some(new_idx)
    }

Step 4: 更新价格点
    price_point.push_back(new_idx)  // 更新last_order_idx

结果：
┌─ 价格100.00 ─┐
│ first: idx1  │
│ last:  idx5  │
└──────────────┘
    ↓
orders[1] → OrderNode(order1, next=3)
    ↓
orders[3] → OrderNode(order2, next=5)
    ↓
orders[5] → OrderNode(order3, next=None)
```

---

## 关键算法

### 1. 添加订单 - add_order

**时间复杂度**：O(1)
**空间复杂度**：O(1)

```rust
fn add_order(&mut self, order: O) -> Result<(), RepoError> {
    // 1. 前置验证（不分配资源）
    if self.order_index.contains_key(&order_id) {
        return Err(RepoError::OrderAlreadyExists);  // 订单已存在
    }
    if self.get_price_point(price, side).is_none() {
        return Err(RepoError::PriceOutOfRange);     // 价格超出范围
    }

    // 2. 分配槽位
    let idx = self.next_slot;
    if idx >= self.orders.capacity() {
        return Err(RepoError::CapacityExceeded);    // 容量已满
    }

    // 3. 存储订单
    let node = OrderNode::new(order);
    if idx == self.orders.len() {
        self.orders.push(Some(node));               // 扩展Vec
    } else {
        self.orders[idx] = Some(node);              // 重用槽位
    }
    self.next_slot += 1;

    // 4. 更新索引和链表
    self.order_index.insert(order_id, idx);        // ID映射
    self.link_order_to_price_level(idx, price, side);  // 链表链接

    // 5. 更新最佳价格缓存
    self.update_best_price(price, side);

    Ok(())
}
```

**优化点**：
- 前置验证避免部分分配导致的回滚
- 槽位复用减少内存碎片
- 最佳价格缓存避免遍历

### 2. 删除订单 - remove_order

**时间复杂度**：O(1)
**空间复杂度**：O(1)

```rust
fn remove_order(&mut self, order_id: OrderId) -> bool {
    if let Some(&idx) = self.order_index.get(&order_id) {
        if self.orders.get(idx).and_then(|o| o.as_ref()).is_some() {
            self.orders[idx] = None;                 // 标记为删除
            self.order_index.remove(&order_id);      // 移除映射

            // 注意：不立即重建链表
            // 链表仍然指向此位置，但在遍历时会跳过None节点
            return true;
        }
    }
    false
}
```

**延迟链表维护**：
- 删除时只标记为None，不重建链表
- 遍历时自动跳过None节点
- 避免O(k)的链表重建成本

### 3. 查找订单 - find_order

**时间复杂度**：O(1)
**空间复杂度**：O(1)

```rust
fn find_order(&self, order_id: OrderId) -> Option<&O> {
    self.order_index                           // O(1) HashMap查询
        .get(&order_id)                        // 获取索引
        .and_then(|&idx| self.orders.get(idx))  // O(1) Vec访问
        .and_then(|opt_node| opt_node.as_ref())  // 解包Option
        .map(|node| &node.order)               // 提取订单引用
}
```

**优点**：
- 直接映射，无需遍历
- 对比红黑树查找避免O(log n)

### 4. 匹配订单 - match_orders

**时间复杂度**：O(p + k)
- p：需要遍历的价格级别数
- k：匹配的订单数
- 最坏情况：O(p + k)

**空间复杂度**：O(k)

```rust
fn match_orders(&self, side: Side, price: Price, quantity: Quantity)
    -> Option<Vec<&O>>
{
    // 预分配容量，减少重分配
    let mut matched_orders = Vec::with_capacity(16);
    let mut remaining = quantity;

    match side {
        Side::Buy => {
            // 买单从最低卖价开始向上匹配
            let ask_min_tick = self.price_to_tick_idx(self.ask_min?)?;
            let price_tick = self.price_to_tick_idx(price)?;

            for current_tick in ask_min_tick..=price_tick {
                if remaining.is_zero() { break; }

                // 遍历该价格级别的所有订单链表
                let current_price = Price::from_raw(
                    current_tick as i64 * self.tick_size.raw()
                );
                if let Some(first_idx) = self.get_first_order_at_price(
                    current_price,
                    Side::Sell
                ) {
                    let mut current_idx = Some(first_idx);

                    // 链表遍历（时间优先）
                    while !remaining.is_zero() && current_idx.is_some() {
                        let idx = current_idx.unwrap();
                        if let Some(Some(node)) = self.orders.get(idx) {
                            let order_qty = node.order.quantity();
                            if order_qty > Quantity::from_raw(0) {
                                // 计算成交量
                                let fill_qty = if remaining < order_qty {
                                    remaining
                                } else {
                                    order_qty
                                };
                                remaining = Quantity::from_raw(
                                    remaining.raw() - fill_qty.raw()
                                );
                                matched_orders.push(&node.order);
                            }
                            current_idx = node.next_idx;  // 移动到下一个
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Side::Sell => {
            // 卖单从最高买价开始向下匹配（反向遍历）
            // 实现逻辑类似，但循环方向相反
            // ...
        }
    }

    if matched_orders.is_empty() {
        None
    } else {
        Some(matched_orders)
    }
}
```

**匹配流程示例**：
```
场景：Buy 10 BTC @ 50050.00

当前订单薄：
  Asks (卖单):
    50000.00: Order[5](2 BTC) → Order[7](3 BTC) → Order[9](5 BTC)
    50000.01: Order[11](4 BTC) → Order[13](2 BTC)
    50000.02: Order[15](1 BTC)

步骤：
1. ask_min_tick = 5000000 (50000.00)
   price_tick = 5000500 (50050.00)

2. 遍历 tick 5000000 到 5000500:
   - tick=5000000 (price=50000.00):
     remaining=10, match Order[5](2), remaining=8, matched=[5]
     remaining=8, match Order[7](3), remaining=5, matched=[5,7]
     remaining=5, match Order[9](5), remaining=0, matched=[5,7,9]
     ✓ 匹配完成！

结果：matched_orders = [&Order[5], &Order[7], &Order[9]]
成交：2+3+5 = 10 BTC
```

---

## 性能分析

### 时间复杂度对比

| 操作 | Vec LOB | 红黑树 | 跳表 | 哈希表 |
|------|---------|--------|------|--------|
| 添加 | O(1) | O(log n) | O(log n) | O(1) avg |
| 删除 | O(1) | O(log n) | O(log n) | O(1) avg |
| 查找 | O(1) | O(log n) | O(log n) | O(1) avg |
| 匹配 | O(p+k) | O(log n + k) | O(log n + k) | O(k) |
| 遍历 | O(n) | O(n) | O(n) | O(n) |

**说明**：
- p = 价格级别数
- k = 匹配订单数
- n = 总订单数

### 空间复杂度

```
总内存 = Vec(tick_count) + Vec(order_count) + HashMap(order_count)

示例（BTCUSDT，tick_size=0.01，max_orders=10000）：
- bids: 30M × 24 bytes = 720 MB
- asks: 30M × 24 bytes = 720 MB
- orders: 10K × 56 bytes = 560 KB
- order_index: 10K × 24 bytes = 240 KB
────────────────────────────────────
总计: ≈ 1.44 GB (预分配，实际利用率低)

优化空间：
- 支持动态Vec扩容，初始分配较小
- 按需分配价格范围
```

### 延迟特性

**关键路径（纳秒级）**：
```
add_order: O(1)
├─ HashMap.insert: ~50ns
├─ Vec[idx] = node: ~20ns
├─ link_order_to_price_level: ~100ns
└─ update_best_price: ~30ns
总计: ~200ns

find_order: O(1)
├─ HashMap.get: ~50ns
├─ Vec.get: ~20ns
└─ 解包: ~10ns
总计: ~80ns

match_orders (单价格级别): ~k × 100ns
- 单个订单遍历: ~100ns
```

---

## 应用示例

### 示例1：添加买单

```rust
// 创建订单簿
let mut lob = LocalLob::new(Symbol::new("BTCUSDT"));

// 添加买单
let buy_order = BTreeOrder {
    order_id: 123,
    symbol: Symbol::new("BTCUSDT"),
    side: Side::Buy,
    price: Price::from_f64(50000.00),
    quantity: Quantity::from_decimal(2.5),
    timestamp: 1000,
};

lob.add_order(buy_order)?;
// 结果：在 bids[5000000] 链表中添加此订单
```

### 示例2：匹配订单

```rust
// 尝试以50050.00价格买入10个BTC
let matched = lob.match_orders(
    Side::Buy,
    Price::from_f64(50050.00),
    Quantity::from_decimal(10.0)
)?;

// 返回匹配的所有卖单订单
for order in &matched {
    println!("Matched order {} @ {} qty {}",
        order.order_id(),
        order.price(),
        order.quantity()
    );
}
```

### 示例3：删除订单

```rust
// 撤销订单ID为123的订单
if lob.remove_order(123) {
    println!("Order cancelled successfully");
} else {
    println!("Order not found");
}

// 订单被标记为None，但链表结构不变
// 下次遍历时自动跳过
```

### 示例4：快速查询

```rust
// 快速查询订单信息
if let Some(order) = lob.find_order(123) {
    println!("Order: {}/{} {} @ {}",
        order.side(),
        order.symbol(),
        order.quantity(),
        order.price()
    );
}

// 查询最佳价格
println!("Bid: {:?}", lob.best_bid());
println!("Ask: {:?}", lob.best_ask());
```

---

## 总结

### 优势
✅ **O(1)订单查找**：HashMap索引
✅ **O(1)订单添加/删除**：槽位分配和标记
✅ **O(p+k)高效匹配**：避免全表扫描
✅ **时间优先保证**：链表维护FIFO顺序
✅ **价格优先保证**：按价格级别遍历
✅ **低延迟**：所有操作常数时间或线性于匹配量

### 劣势
❌ **内存预分配**：max_ticks数量庞大时占用大量内存
❌ **不支持动态精度**：tick_size固定
❌ **链表重建开销**：大量删除后可能需要整理
❌ **缓存失效**：删除后bid_max/ask_min可能不准确

### 适用场景
- ✓ 高频交易系统（HFT）
- ✓ 交易所核心撮合引擎
- ✓ 实时市场数据处理
- ✓ 小中型交易对（价格范围有限）

### 不适用场景
- ✗ 超大价格范围的交易对
- ✗ 需要频繁修改精度的系统
- ✗ 内存受限的环境
