# 编译时缓存友好性检查

CacheAnalyzer 提供强大的编译时检查功能,确保结构体设计符合缓存友好性原则。

## 功能概述

编译时检查可以在代码编译阶段就发现潜在的性能问题,而不是等到运行时才发现。这对于高性能低延迟系统至关重要。

## 检查属性

### 1. `#[cache(enforce_order)]` - 强制最优字段顺序

强制字段按对齐要求降序排列,最小化填充字节。

```rust
// ✅ 正确: 字段按对齐降序排列
#[derive(CacheAnalyzer)]
#[cache(enforce_order)]
pub struct OptimalStruct {
    value: u64,    // 8 字节对齐
    counter: u32,  // 4 字节对齐
    flag: bool,    // 1 字节对齐
}

// ❌ 编译错误: 字段顺序不优
#[derive(CacheAnalyzer)]
#[cache(enforce_order)]
pub struct BadStruct {
    flag: bool,    // 错误! 小字段不应该在前面
    value: u64,
    counter: u32,
}
// Error: 结构体 BadStruct 的字段顺序不是最优的
```

### 2. `#[cache(strict)]` - 严格模式

启用严格模式,强制以下规则:
- 热点字段 (`#[hot]`) 必须在结构体前部
- 冷数据字段 (`#[cold]`) 应该在后面

```rust
// ✅ 正确: 热点字段在前
#[derive(CacheAnalyzer)]
#[cache(strict)]
pub struct GoodStruct {
    #[hot]
    price: f64,
    #[hot]
    quantity: f64,
    #[cold]
    metadata: String,
}

// ❌ 编译错误: 热点字段顺序不对
#[derive(CacheAnalyzer)]
#[cache(strict)]
pub struct BadStruct {
    #[cold]
    metadata: String,
    #[hot]  // 错误! 热点字段应该在前面
    price: f64,
}
```

### 3. `#[cache(max_size = N)]` - 限制结构体大小

设置结构体最大大小（字节）。

```rust
// ✅ 正确: 符合大小限制
#[derive(CacheAnalyzer)]
#[cache(max_size = 64)]
pub struct SmallStruct {
    data1: u64,  // 8 字节
    data2: u64,  // 8 字节
    data3: u32,  // 4 字节
    // 总共 20 字节，符合 64 字节限制
}

// ❌ 编译错误: 超过大小限制
#[derive(CacheAnalyzer)]
#[cache(max_size = 32)]
pub struct OversizedStruct {
    data: [u64; 10],  // 80 字节
}
// Error: 结构体 OversizedStruct 估算大小 80 字节超过最大限制 32 字节
```

### 4. `#[cache(max_padding = N)]` - 限制填充比例

设置最大允许的填充比例（百分比）。

```rust
// ✅ 正确: 填充比例低
#[derive(CacheAnalyzer)]
#[cache(max_padding = 20.0)]
pub struct EfficientStruct {
    value: u64,
    counter: u32,
    flag: bool,
    // 填充约 12.5%
}

// ❌ 编译错误: 填充比例过高
#[derive(CacheAnalyzer)]
#[cache(max_padding = 10.0)]
pub struct WastefulStruct {
    flag1: bool,  // 1 + 7 填充
    value: u64,   // 8
    flag2: bool,  // 1 + 3 填充
    counter: u32, // 4
    // 填充约 41.7%
}
```

### 5. `#[cache(max_cache_lines = N)]` - 限制缓存行数量

限制结构体占用的缓存行数量（假设 64 字节缓存行）。

```rust
// ✅ 正确: 适合单个缓存行
#[derive(CacheAnalyzer)]
#[cache(max_cache_lines = 1)]
pub struct CompactStruct {
    data1: u64,
    data2: u64,
    data3: u32,
    // 20 字节，适合单个缓存行
}

// ❌ 编译错误: 需要多个缓存行
#[derive(CacheAnalyzer)]
#[cache(max_cache_lines = 1)]
pub struct LargeStruct {
    data: [u64; 20],  // 160 字节，需要 3 个缓存行
}
```

## 组合使用

可以组合多个属性进行全面检查:

```rust
#[derive(CacheAnalyzer)]
#[cache(
    enforce_order,
    max_size = 64,
    max_padding = 15.0,
    max_cache_lines = 1
)]
pub struct HighPerfStruct {
    value: u64,
    counter: u32,
    flag: bool,
}
```

## 编译错误示例

### 错误 1: 字段顺序不优

```
error: custom attribute panicked
  --> src/main.rs:10:10
   |
10 | #[derive(CacheAnalyzer)]
   |          ^^^^^^^^^^^^^
   |
   = help: message: 结构体 BadStruct 的字段顺序不是最优的
           字段 'value' (索引 1) 的对齐 8 大于前面字段的对齐 1

           建议的字段顺序（按对齐降序）:
           value
           counter
           flag
```

### 错误 2: 超过大小限制

```
error: custom attribute panicked
  --> src/main.rs:15:10
   |
15 | #[derive(CacheAnalyzer)]
   |          ^^^^^^^^^^^^^
   |
   = help: message: 结构体 OversizedStruct 估算大小 80 字节超过最大限制 32 字节
           建议:
           - 将大字段拆分到单独的结构体
           - 使用 Box<T> 或引用类型减小内联大小
           - 考虑使用 #[repr(C)] 并手动优化布局
```

### 错误 3: 热点字段顺序错误

```
error: custom attribute panicked
  --> src/main.rs:20:10
   |
20 | #[derive(CacheAnalyzer)]
   |          ^^^^^^^^^^^^^
   |
   = help: message: 结构体 BadHotStruct 的热点字段 'price' 不在结构体开头
           为了更好的缓存预取效果，所有 #[hot] 字段应该放在结构体前面
```

## 运行示例

```bash
# 查看正确的用法
cargo run --example compile_time_validation

# 测试编译错误（取消注释示例中的错误代码）
cargo check
```

## 最佳实践

1. **在关键路径结构上使用 `enforce_order`**
   - 交易订单、市场数据等高频访问的结构体

2. **为缓存密集型结构使用 `max_cache_lines = 1`**
   - 确保结构体适合单个缓存行

3. **为热点数据使用 `strict` 模式**
   - 确保最频繁访问的字段在缓存行开头

4. **设置合理的 `max_padding`**
   - 一般建议不超过 20%

5. **结合使用多个检查**
   - 综合检查确保最佳性能

## 与运行时分析的对比

| 特性 | 编译时检查 | 运行时分析 |
|------|-----------|-----------|
| 检查时机 | 编译期 | 运行期 |
| 性能影响 | 零 | 轻微 |
| 错误反馈 | 立即 | 需要主动调用 |
| 适用场景 | 严格要求 | 诊断优化 |

## 何时使用编译时检查

- ✅ 低延迟关键路径结构体
- ✅ 高频访问的数据结构
- ✅ 缓存敏感的算法
- ✅ 团队协作确保代码质量
- ✅ 防止性能回归

## 何时不使用

- ❌ 仅偶尔使用的结构体
- ❌ 灵活性比性能更重要
- ❌ 原型开发阶段
- ❌ 测试代码

## 总结

编译时检查是 CacheAnalyzer 的强大功能,帮助开发者在编译阶段就发现并修复缓存不友好的设计。结合运行时分析功能,可以确保代码始终保持最优的缓存性能。
