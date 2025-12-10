# OrderDelta 结构体优化方案

## 当前状态分析

### 内存布局

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrderDelta {
    pub symbol_id: SymbolId,              // u32: 4 字节
    pub timestamp: u64,                    // u64: 8 字节
    pub sequence: u64,                     // u64: 8 字节
    pub change_type: OrderChangeType,      // enum: 1 字节
    pub order_id: OrderId,                 // u64: 8 字节
    pub side: Side,                        // enum: 1 字节
    pub price: Price,                      // u32: 4 字节
    pub quantity: Quantity,                // u32: 4 字节
    pub trader_id: Option<TraderId>,       // Option<[u8;8]>: 16 字节！
}
```

### 关键发现

- **总大小**: 56 字节
- **理论最小**: 47 字节
- **Padding 浪费**: 9 字节 (16.1%)
- **最大问题**: `Option<TraderId>` 占用 16 字节（8 字节数据 + 8 字节判别符）

### 性能影响

- 每个缓存行（64字节）只能容纳 **1 个** OrderDelta
- 100 个实例占用 **88 个缓存行**
- 内存带宽浪费严重

---

## 优化方案

### 方案 1: 字段重排（最简单，效果有限）

**目标**: 减少 padding，但无法解决 Option<TraderId> 的问题

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV1 {
    // 8 字节对齐字段（放在前面）
    pub timestamp: u64,                    // 偏移 0
    pub sequence: u64,                     // 偏移 8
    pub order_id: u64,                     // 偏移 16
    pub trader_id: Option<TraderId>,       // 偏移 24 (16 字节)

    // 4 字节对齐字段
    pub symbol_id: u32,                    // 偏移 40
    pub price: u32,                        // 偏移 44
    pub quantity: u32,                     // 偏移 48

    // 1 字节字段（放在最后）
    pub change_type: OrderChangeType,      // 偏移 52
    pub side: Side,                        // 偏移 53
    // padding: 2 字节到 56
}
```

**效果**:
- 大小: 56 字节（无变化）
- 优点: 字段访问可能更快（热字段在前）
- 缺点: 没有实质性减少内存占用

---

### 方案 2: 使用 NonZeroU64 优化 Option（推荐）

**核心思想**: 利用 `NonZeroU64` 的空值优化（niche optimization）

```rust
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV2 {
    pub timestamp: u64,
    pub sequence: u64,
    pub order_id: u64,
    pub trader_id: Option<NonZeroU64>,     // 只占 8 字节！

    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    pub change_type: OrderChangeType,
    pub side: Side,
    // padding: 2 字节
}
```

**效果**:
- 大小: **48 字节**（减少 8 字节，14.3%）
- `Option<NonZeroU64>` 只占 8 字节（利用 0 作为 None）
- 100 个实例: 4,800 字节（节省 800 字节）

**实现**:
```rust
impl OrderDeltaV2 {
    pub fn new(/* ... */, trader_id: Option<u64>) -> Self {
        Self {
            // ...
            trader_id: trader_id.and_then(NonZeroU64::new),
        }
    }

    pub fn trader_id_u64(&self) -> Option<u64> {
        self.trader_id.map(|id| id.get())
    }
}
```

---

### 方案 3: 位域压缩（极致优化）

**核心思想**: 将小枚举压缩到一个字节

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV3 {
    pub timestamp: u64,
    pub sequence: u64,
    pub order_id: u64,
    pub trader_id: Option<NonZeroU64>,     // 8 字节

    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    // 位域压缩: change_type (2 bits) + side (1 bit) = 3 bits
    pub flags: u8,                         // 1 字节
    // padding: 3 字节
}

impl OrderDeltaV3 {
    const SIDE_MASK: u8 = 0b0000_0001;
    const CHANGE_TYPE_MASK: u8 = 0b0000_0110;

    #[inline]
    pub fn side(&self) -> Side {
        if self.flags & Self::SIDE_MASK == 0 {
            Side::Buy
        } else {
            Side::Sell
        }
    }

    #[inline]
    pub fn change_type(&self) -> OrderChangeType {
        match (self.flags & Self::CHANGE_TYPE_MASK) >> 1 {
            0 => OrderChangeType::Add,
            1 => OrderChangeType::Modify,
            2 => OrderChangeType::Delete,
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn set_side(&mut self, side: Side) {
        match side {
            Side::Buy => self.flags &= !Self::SIDE_MASK,
            Side::Sell => self.flags |= Self::SIDE_MASK,
        }
    }

    #[inline]
    pub fn set_change_type(&mut self, change_type: OrderChangeType) {
        self.flags &= !Self::CHANGE_TYPE_MASK;
        let bits = match change_type {
            OrderChangeType::Add => 0,
            OrderChangeType::Modify => 1,
            OrderChangeType::Delete => 2,
        };
        self.flags |= bits << 1;
    }
}
```

**效果**:
- 大小: **48 字节**（与方案2相同）
- 优点: 节省 1 字节，减少 padding
- 缺点: 访问需要位操作（可能稍慢）

---

### 方案 4: 分离可选字段（最激进）

**核心思想**: 将 `trader_id` 移到单独的存储

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV4 {
    pub timestamp: u64,
    pub sequence: u64,
    pub order_id: u64,

    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    pub change_type: OrderChangeType,
    pub side: Side,
    // padding: 2 字节
}

// 单独存储 trader_id
pub struct OrderDeltaStore {
    deltas: Vec<OrderDeltaV4>,
    trader_ids: HashMap<OrderId, TraderId>,
}
```

**效果**:
- OrderDeltaV4 大小: **40 字节**（减少 16 字节，28.6%）
- 100 个实例: 4,000 字节（节省 1,600 字节）
- 如果只有 10% 的订单有 trader_id，总内存节省更多

**权衡**:
- ✅ 大幅减少内存占用
- ✅ 缓存友好（每个缓存行可容纳 1.6 个实例）
- ❌ 访问 trader_id 需要额外查找
- ❌ 代码复杂度增加

---

### 方案 5: 使用更紧凑的类型（需要业务评估）

**核心思想**: 如果业务允许，使用更小的类型

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrderDeltaV5 {
    pub timestamp_ms: u32,                 // 相对时间戳（毫秒）
    pub sequence: u32,                     // 如果范围足够
    pub order_id: u64,
    pub trader_id: Option<NonZeroU64>,

    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    pub change_type: OrderChangeType,
    pub side: Side,
    // padding: 2 字节
}
```

**效果**:
- 大小: **40 字节**（减少 16 字节）
- 需要业务确认:
  - timestamp 是否可以用相对时间（相对于某个基准时间）
  - sequence 是否会超过 u32::MAX (4,294,967,295)

---

## 性能对比

| 方案 | 大小 | 节省 | 缓存行利用率 | 复杂度 | 推荐度 |
|------|------|------|-------------|--------|--------|
| 当前 | 56 字节 | - | 1.14 个/行 | 低 | - |
| 方案1 (重排) | 56 字节 | 0% | 1.14 个/行 | 低 | ⭐ |
| 方案2 (NonZero) | 48 字节 | 14.3% | 1.33 个/行 | 低 | ⭐⭐⭐⭐⭐ |
| 方案3 (位域) | 48 字节 | 14.3% | 1.33 个/行 | 中 | ⭐⭐⭐ |
| 方案4 (分离) | 40 字节 | 28.6% | 1.60 个/行 | 高 | ⭐⭐⭐⭐ |
| 方案5 (紧凑) | 40 字节 | 28.6% | 1.60 个/行 | 中 | ⭐⭐⭐ |

---

## 推荐实施路径

### 阶段 1: 立即实施（方案2）

**使用 NonZeroU64 优化 Option**

```rust
#[derive(Debug, Clone, Copy)]
pub struct OrderDelta {
    // 8 字节对齐字段
    pub timestamp: u64,
    pub sequence: u64,
    pub order_id: u64,
    pub trader_id: Option<NonZeroU64>,     // 改这里！

    // 4 字节对齐字段
    pub symbol_id: u32,
    pub price: u32,
    pub quantity: u32,

    // 1 字节字段
    pub change_type: OrderChangeType,
    pub side: Side,
}
```

**优点**:
- 简单，改动最小
- 立即节省 14.3% 内存
- 无性能损失
- 向后兼容（只需修改构造和访问方法）

### 阶段 2: 评估业务需求（方案4或5）

如果内存压力仍然很大，考虑:
- **方案4**: 如果 trader_id 使用率低（< 20%）
- **方案5**: 如果业务允许使用更小的类型

### 阶段 3: 极致优化（方案3）

如果需要榨取最后一点性能:
- 结合方案2和方案3
- 使用位域压缩枚举
- 可能节省额外的 padding

---

## 基准测试计划

创建以下基准测试来验证优化效果:

```rust
// 1. 内存占用测试
bench_memory_size_v1()
bench_memory_size_v2()
bench_memory_size_v4()

// 2. 分配性能测试
bench_allocation_100_v1()
bench_allocation_100_v2()
bench_allocation_100_v4()

// 3. 访问性能测试
bench_field_access_v1()
bench_field_access_v2()
bench_field_access_v3()  // 测试位域访问开销

// 4. 缓存性能测试
bench_sequential_access_v1()
bench_sequential_access_v2()
bench_sequential_access_v4()
```

---

## 实施检查清单

- [ ] 实现 OrderDeltaV2 (NonZeroU64)
- [ ] 创建基准测试对比
- [ ] 验证内存大小减少到 48 字节
- [ ] 测试分配性能（应该更快）
- [ ] 测试访问性能（应该相同或更快）
- [ ] 更新文档和示例
- [ ] 迁移现有代码
- [ ] 性能回归测试

---

## 预期收益

对于 100 个 OrderDelta 实例:

| 指标 | 当前 | 优化后 (方案2) | 改进 |
|------|------|---------------|------|
| 内存占用 | 5,600 字节 | 4,800 字节 | -14.3% |
| 缓存行占用 | 88 行 | 75 行 | -14.8% |
| 分配时间 | 179 ns | ~150 ns (预期) | -16% |
| L1 缓存命中率 | 基准 | 更高 | +10-15% |

对于 1,000 个实例:
- 节省内存: **8,000 字节** (8 KB)
- 减少缓存行: **125 行**

---

## 结论

**立即实施方案2（NonZeroU64）**，这是最佳的性价比选择:
- ✅ 简单易实施
- ✅ 显著的内存节省（14.3%）
- ✅ 性能提升（更好的缓存利用率）
- ✅ 无运行时开销
- ✅ 向后兼容

后续可根据实际需求考虑方案4或方案5进行进一步优化。
