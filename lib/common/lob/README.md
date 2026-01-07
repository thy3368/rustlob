# Rust之从0-1低时延CEX：LocalLob 定制多搓合算法方案对比

本项目实现了**三种** LocalLob（本地限价订单簿）方案，以支持不同精度和价格范围的加密货币交易对。

## 方案概述

### 方案 1: LocalLob (Vec + Tick Size)
**文件**: `src/adapter/local_lob_impl.rs`

**核心特性**:
- 使用 **Vec 数组** 存储价格点
- 通过 **Tick Size** 将价格映射到数组索引
- 数组索引 = `price.raw() / tick_size.raw()`

**优点**:
- ✅ **O(1) 确定性能** - 数组访问速度极快
- ✅ **CPU 缓存友好** - 连续内存访问
- ✅ **无哈希开销** - 直接索引访问

**缺点**:
- ❌ **预分配内存** - 需要预先分配大数组（默认 30,000,000 元素）
- ❌ **内存浪费** - 稀疏订单簿会浪费大量空间
- ❌ **价格范围受限** - 最大支持 `max_ticks * tick_size`

**适用场景**:
- 高频交易（HFT）系统
- 价格范围可预测的币种（BTC, ETH）
- 订单分布密集的交易对
- 对延迟敏感的应用

---

### 方案 2: LocalLobHashMap (HashMap + Tick Size)
**文件**: `src/adapter/local_lob_hashmap_impl.rs`

**核心特性**:
- 使用 **HashMap** 存储价格点
- Key = Tick 数量，Value = PricePoint
- 只存储实际有订单的价格级别

**优点**:
- ✅ **内存高效** - 只存储有订单的价格点
- ✅ **无价格范围限制** - 支持任意价格范围
- ✅ **灵活精度** - 适合低价币（SHIB, PEPE）
- ✅ **稀疏友好** - 订单稀疏时节省内存

**缺点**:
- ❌ **哈希开销** - HashMap 查找有哈希计算成本
- ❌ **O(1) 期望** - 不是确定性 O(1)，存在碰撞风险
- ❌ **缓存不友好** - 非连续内存访问
- ❌ **需要排序** - 匹配时需要收集并排序价格点

**适用场景**:
- 低价币交易（SHIB, PEPE, DOGE）
- 内存受限的环境
- 价格波动范围大的币种
- 订单分布稀疏的交易对

---

### 方案 3: LocalLobBTreeMap (BTreeMap + Tick Size) 🌟 推荐
**文件**: `src/adapter/local_lob_btreemap_impl.rs`

**核心特性**:
- 使用 **BTreeMap** 存储价格点
- Key = Tick 数量（自动排序），Value = PricePoint
- 利用 BTreeMap 的有序性和范围查询

**优点**:
- ✅ **自动排序** - Key 自动按升序排列，无需额外排序
- ✅ **高效范围查询** - `range()` 方法直接返回有序迭代器
- ✅ **内存高效** - 只存储有订单的价格点
- ✅ **O(log n) 确定性能** - 比 HashMap 的 O(1) 期望更可预测
- ✅ **无价格范围限制** - 支持任意价格范围
- ✅ **市场深度查询** - 天然支持按序遍历

**缺点**:
- ❌ **查找稍慢** - O(log n) vs Vec 的 O(1)
- ❌ **插入开销** - 需要维护树结构

**适用场景**:
- **推荐用于生产环境** - 性能和灵活性的最佳平衡
- 订单匹配引擎（价格优先原则）
- 市场深度查询（Depth of Market）
- 需要频繁遍历价格级别的场景
- 对确定性性能有要求的系统

---

## Price 精度配置

### 当前配置
```rust
impl Price {
    const DECIMALS: i64 = 100_000_000; // 8 位小数
}
```

**8 位小数精度**：
- 符合比特币 Satoshi 标准（0.00000001 BTC）
- 支持最小单位 0.00000001 USDT
- 可表示范围：-92,233,720,368.54775808 ~ 92,233,720,368.54775807

---

## Tick Size 配置示例

### 高价币（BTC, ETH）
```rust
let btc_lob = LocalLob::new_with_tick(
    Symbol::new("BTCUSDT"),
    Price::from_f64(0.01)  // tick = 0.01 USDT
);
```
- **价格步进**: 50000.00, 50000.01, 50000.02 ...
- **内存占用**: 30M * 8 bytes = 240 MB (Vec 方案)
- **支持范围**: 0.00 ~ 300,000.00 USDT

### 中价币（DOGE）
```rust
let doge_lob = LocalLobHashMap::new_with_tick(
    Symbol::new("DOGEUSDT"),
    Price::from_f64(0.0001)  // tick = 0.0001 USDT
);
```
- **价格步进**: 0.0800, 0.0801, 0.0802 ...
- **内存占用**: 按需分配（HashMap 方案）

### 低价币（SHIB, PEPE）
```rust
let shib_lob = LocalLobHashMap::new_with_tick(
    Symbol::new("SHIBUSDT"),
    Price::from_f64(0.00000001)  // tick = 0.00000001 USDT
);
```
- **价格步进**: 0.00001234, 0.00001235, 0.00001236 ...
- **内存占用**: 极少（只存储有订单的价格点）

---

## 性能对比

| 操作 | LocalLob (Vec) | LocalLobHashMap | LocalLobBTreeMap |
|------|----------------|-----------------|------------------|
| 添加订单 | O(1) 确定 | O(1) 期望 | O(log n) 确定 |
| 查找订单 | O(1) | O(1) 期望 | O(log n) |
| 匹配订单 | O(k) | O(k + n log n)* | O(k + log n)** |
| 最佳价格 | O(1) | O(1) | O(1) |
| 市场深度 | O(m) | O(n + n log n) | O(m + log n)*** |
| 内存使用 | 预分配固定 | 按需动态 | 按需动态 |
| 性能稳定性 | 确定 | 期望（碰撞风险） | 确定 |

\* k = 匹配订单数，n = 价格级别数（HashMap 需要收集并排序）
\** BTreeMap 使用 range() 直接返回有序迭代器，无需排序
\*** m = 深度档位数，BTreeMap 直接按序遍历

---

## 选择建议

### 使用 LocalLob (Vec 方案) 当：
1. **极致性能** - 需要纳秒级延迟（如高频交易）
2. **主流币种** - BTC, ETH, BNB（价格范围可预测）
3. **订单密集** - 大部分价格级别都有订单
4. **内存充足** - 可以预分配 240 MB+ 内存

### 使用 LocalLobHashMap (HashMap 方案) 当：
1. **极低价币** - SHIB, PEPE（需要最高精度）
2. **内存受限** - 移动端或嵌入式设备
3. **极稀疏订单簿** - 价格点非常分散
4. **不需要排序** - 只做单点查询，不遍历价格

### 使用 LocalLobBTreeMap (BTreeMap 方案) 当：🌟 **推荐**
1. **生产环境** - 性能和灵活性的最佳平衡
2. **订单匹配引擎** - 需要频繁按价格顺序遍历
3. **市场深度查询** - 需要显示 LOB 深度数据
4. **确定性性能** - 相比 HashMap 更可预测
5. **通用场景** - 适合大多数交易对

---

## 测试覆盖

### LocalLob 测试 (7 个)
文件: `tests/local_lob_tests.rs`
- ✅ BTC 真实价格测试（50000.00 USDT）
- ✅ 订单匹配测试
- ✅ 最佳价格缓存
- ✅ 订单增删改查

### LocalLobHashMap 测试 (8 个)
文件: `tests/local_lob_hashmap_tests.rs`
- ✅ BTC 高价币测试（tick = 0.01）
- ✅ SHIB 低价币测试（tick = 0.00000001）
- ✅ PEPE 超低价币测试（tick = 0.00000001）
- ✅ DOGE 中价币测试（tick = 0.0001）
- ✅ 稀疏订单簿内存效率测试

### LocalLobBTreeMap 测试 (9 个) ⭐ 新增
文件: `tests/local_lob_btreemap_tests.rs`
- ✅ 基础订单操作测试
- ✅ **有序匹配测试** - 验证价格优先原则
- ✅ **市场深度查询测试** - 独有功能
- ✅ **范围查询测试** - BTreeMap 优势
- ✅ SHIB 低价币支持
- ✅ 同价格时间优先（FIFO）
- ✅ 买卖双向匹配测试

---

## 示例代码

### 使用 Vec 方案（高性能）
```rust
use lob_repo::adapter::local_lob_impl::LocalLob;

// BTC 订单簿
let mut btc_lob = LocalLob::new_with_tick(
    Symbol::new("BTCUSDT"),
    Price::from_f64(0.01)
);

// 添加订单
let order = BtcOrder {
    id: 1,
    price: Price::from_f64(50000.0),  // 50000.00 USDT
    quantity: Quantity::from_f64(1.5),  // 1.5 BTC
    side: Side::Buy,
};
btc_lob.add_order(order)?;
```

### 使用 HashMap 方案（灵活）
```rust
use lob_repo::adapter::local_lob_hashmap_impl::LocalLobHashMap;

// SHIB 订单簿
let mut shib_lob = LocalLobHashMap::new_with_tick(
    Symbol::new("SHIBUSDT"),
    Price::from_f64(0.00000001)
);

// 添加订单
let order = ShibOrder {
    id: 1,
    price: Price::from_f64(0.00001234),  // 0.00001234 USDT
    quantity: Quantity::from_f64(1000000.0),  // 100万 SHIB
    side: Side::Buy,
};
shib_lob.add_order(order)?;
```

### 使用 BTreeMap 方案（推荐）🌟
```rust
use lob_repo::adapter::local_lob_btreemap_impl::LocalLobBTreeMap;

// 通用订单簿（适合大多数币种）
let mut lob = LocalLobBTreeMap::new_with_tick(
    Symbol::new("ETHUSDT"),
    Price::from_f64(0.01)
);

// 添加订单
let order = Order {
    id: 1,
    price: Price::from_f64(3000.0),  // 3000.00 USDT
    quantity: Quantity::from_f64(10.0),  // 10 ETH
    side: Side::Buy,
};
lob.add_order(order)?;

// 市场深度查询（BTreeMap 独有优势）
let (bids, asks) = lob.market_depth(5);
// bids: [(3000.0, 10.0), (2999.0, 5.0), ...]
// asks: [(3001.0, 8.0), (3002.0, 12.0), ...]
```

---

## 总结

三种方案各有优劣，根据实际需求选择：

- **追求极致性能** → LocalLob (Vec)
- **追求灵活性** → LocalLobHashMap (HashMap)
- **生产环境推荐** → LocalLobBTreeMap (BTreeMap) 🌟

### 推荐配置

**多币种交易所最佳实践**:
1. **主流币**（BTC/ETH/BNB）→ **BTreeMap**（平衡性能和功能）
2. **极低价币**（SHIB/PEPE）→ **BTreeMap** 或 **HashMap**
3. **高频交易专用**（延迟敏感）→ **Vec**

**单一场景推荐**:
- 如果不确定选哪个 → **直接用 BTreeMap**
- BTreeMap 提供了性能、功能、灵活性的最佳平衡
