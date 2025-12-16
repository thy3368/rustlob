# LocalLob 实现方案对比

本项目实现了两种 LocalLob（本地限价订单簿）方案，以支持不同精度和价格范围的加密货币交易对。

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

**适用场景**:
- 低价币交易（SHIB, PEPE, DOGE）
- 内存受限的环境
- 价格波动范围大的币种
- 订单分布稀疏的交易对

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

| 操作 | LocalLob (Vec) | LocalLobHashMap |
|------|----------------|-----------------|
| 添加订单 | O(1) 确定 | O(1) 期望 |
| 查找订单 | O(1) | O(1) 期望 |
| 匹配订单 | O(k) | O(k + log n)* |
| 最佳价格 | O(1) | O(1) |
| 内存使用 | 预分配固定 | 按需动态 |

\* k = 匹配订单数，n = 价格级别数（HashMap 需要排序）

---

## 选择建议

### 使用 LocalLob (Vec 方案) 当：
1. **高频交易** - 需要纳秒级延迟
2. **主流币种** - BTC, ETH, BNB（价格范围可预测）
3. **订单密集** - 大部分价格级别都有订单
4. **内存充足** - 可以预分配 240 MB+ 内存

### 使用 LocalLobHashMap (HashMap 方案) 当：
1. **低价币种** - SHIB, PEPE, DOGE（需要高精度）
2. **内存受限** - 移动端或嵌入式设备
3. **订单稀疏** - 只有少数价格级别有订单
4. **价格范围大** - 价格波动超过 Vec 支持范围

---

## 测试覆盖

### LocalLob 测试
文件: `tests/local_lob_tests.rs`
- ✅ BTC 真实价格测试（50000.00 USDT）
- ✅ 订单匹配测试
- ✅ 最佳价格缓存
- ✅ 订单增删改查

### LocalLobHashMap 测试
文件: `tests/local_lob_hashmap_tests.rs`
- ✅ BTC 高价币测试（tick = 0.01）
- ✅ SHIB 低价币测试（tick = 0.00000001）
- ✅ PEPE 超低价币测试（tick = 0.00000001）
- ✅ DOGE 中价币测试（tick = 0.0001）
- ✅ 稀疏订单簿内存效率测试

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

---

## 总结

两种方案各有优劣，根据实际需求选择：

- **追求极致性能** → LocalLob (Vec)
- **追求灵活性和内存效率** → LocalLobHashMap (HashMap)

对于多币种交易所，可以混合使用：
- 主流币（BTC/ETH）使用 Vec 方案
- 低价币（SHIB/PEPE）使用 HashMap 方案
