# CacheAnalyzer Derive Macro

用于分析 Rust 结构体缓存友好性的过程宏，自动生成内存布局分析和优化建议。

## 架构

该项目由两个 crate 组成：

- **cache_analyzer_derive**: 过程宏实现
- **cache_analyzer_types**: 公共类型定义（所有分析结果使用统一的类型）

## 功能特性

- **内存布局分析**: 自动计算结构体大小、对齐、字段偏移量
- **缓存行分析**: 评估结构体占用的缓存行数量
- **填充字节检测**: 识别由于对齐产生的填充空间
- **优化建议**: 提供字段重排和结构优化建议
- **热点字段标记**: 支持 `#[hot]` 和 `#[cold]` 属性
- **统一类型**: 所有结构体共享相同的分析报告类型，避免类型爆炸

## 安装

将依赖添加到 `Cargo.toml`:

```toml
[dependencies]
cache_analyzer_derive = { path = "../cache_analyzer_derive" }
cache_analyzer_types = { path = "../cache_analyzer_types" }
```

## 基本使用

```rust
use cache_analyzer_derive::CacheAnalyzer;

#[derive(CacheAnalyzer)]
pub struct OrderBook {
    best_bid: f64,
    best_ask: f64,
    last_update_time: u64,
}

fn main() {
    // 获取详细的缓存分析报告
    let report = OrderBook::detailed_cache_analysis();

    println!("结构体: {}", report.struct_name);
    println!("总大小: {} 字节", report.total_size);
    println!("对齐: {} 字节", report.alignment);
    println!("缓存行数: {}", report.cache_lines_needed);
    println!("填充字节: {} ({:.1}%)",
             report.padding_bytes,
             report.padding_percentage);

    // 输出优化建议
    for suggestion in OrderBook::optimization_suggestions() {
        println!("建议: {}", suggestion);
    }
}
```

## 高级用法

### 标记热点字段

使用 `#[hot]` 属性标记频繁访问的字段：

```rust
#[derive(CacheAnalyzer)]
pub struct TradingData {
    #[hot]
    current_price: f64,

    #[hot]
    volume: f64,

    #[cold]
    historical_data: Vec<f64>,
}
```

### 字段分析

查看每个字段的详细信息：

```rust
let report = TradingData::detailed_cache_analysis();

for field in &report.field_analyses {
    println!("字段: {}", field.name);
    println!("  偏移: {} 字节", field.offset);
    println!("  大小: {} 字节", field.size);
    println!("  对齐: {} 字节", field.alignment);
    println!("  热点: {}", field.is_hot);
}
```

### 获取内存布局

```rust
let layout = OrderBook::memory_layout();

println!("名称: {}", layout.name);
println!("大小: {} 字节", layout.size);
println!("对齐: {} 字节", layout.alignment);
println!("是否紧凑: {}", layout.is_packed);
```

## 生成的 API

`CacheAnalyzer` 宏会为结构体生成以下方法：

### `detailed_cache_analysis() -> {StructName}DetailedCacheReport`

返回包含完整分析的详细报告：

- `struct_name`: 结构体名称
- `total_size`: 总大小（字节）
- `alignment`: 对齐要求（字节）
- `cache_line_size`: 缓存行大小（默认64字节）
- `cache_lines_needed`: 需要的缓存行数
- `field_count`: 字段数量
- `field_analyses`: 字段分析详情
- `padding_bytes`: 填充字节数
- `padding_percentage`: 填充百分比
- `optimal_field_order`: 最优字段顺序
- `is_current_order_optimal`: 当前顺序是否最优
- `suggestions`: 优化建议列表

### `optimization_suggestions() -> Vec<String>`

返回优化建议列表。

### `memory_layout() -> {StructName}LayoutInfo`

返回内存布局信息：

- `name`: 结构体名称
- `size`: 大小
- `alignment`: 对齐
- `is_packed`: 是否紧凑布局

## 优化建议示例

宏会自动生成以下类型的建议：

1. **填充空间过大**: 当填充超过 20% 时建议重排字段
2. **字段顺序非最优**: 建议按对齐和大小降序排列
3. **结构体过大**: 超过 64 字节时建议拆分
4. **多缓存行访问**: 需要多个缓存行时建议优化布局

## 示例：优化前后对比

### 优化前

```rust
#[derive(CacheAnalyzer)]
pub struct BadLayout {
    flag: bool,      // 1 字节 + 7 字节填充
    value: u64,      // 8 字节
    counter: u32,    // 4 字节 + 4 字节填充
}
// 总大小: 24 字节，填充: 11 字节 (45.8%)
```

### 优化后

```rust
#[derive(CacheAnalyzer)]
pub struct GoodLayout {
    value: u64,      // 8 字节
    counter: u32,    // 4 字节
    flag: bool,      // 1 字节 + 3 字节填充
}
// 总大小: 16 字节，填充: 3 字节 (18.75%)
```

## 性能考虑

- 分析在编译时完成，运行时零成本
- `detailed_cache_analysis()` 方法会进行运行时计算，建议仅在调试/分析时使用
- 生成的分析数据适合在测试或开发工具中使用

## 限制

- 仅支持具名字段的结构体
- 不支持元组结构体
- 不支持枚举
- 默认缓存行大小为 64 字节（符合大多数现代 CPU）

## 许可

与父项目相同的许可证。
