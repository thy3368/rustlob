# SBE Derive 宏验收测试设计

**日期**: 2026-03-04
**目标**: 完善 sbe-derive 用例测试并进行全面验收
**参考计划**: `.sisyphus/plans/completed/sbe-derive-macro.md`

## 验收范围

基于计划文档中的 32 个任务和 Must Have 要求，本验收测试采用**增量验收策略**，重点补充缺失的关键测试用例。

## 当前测试覆盖分析

### 已覆盖特性（✅ 通过）

**基础功能**（5个测试通过）：
- 基本类型编解码：u8/u16/u32/u64/i8/i16/i32/i64/f32/f64
- 布尔和字符类型：bool, char
- 可选字段：Option<T> with nullValue
- 定长数组：[u8; N]
- 常量字段：presence=constant
- 枚举支持：SbeEnum derive
- 版本字段：since_version (v0/v1/v2)

**性能测试**：
- 编码性能：< 1000ns/op
- 解码性能：< 1000ns/op
- 往返性能：< 2000ns/op
- Serde 对比：SBE 优于 JSON 和 Bincode

**示例覆盖**（14个示例）：
- simple_trade.rs
- comprehensive.rs
- enum_support.rs
- version_fields.rs
- advanced_features.rs
- repeating_groups.rs（概念示例）
- nested_messages.rs（概念示例）
- decimal_types.rs
- time_types.rs
- value_validation.rs
- xml_schema_gen.rs
- trade_codec.rs

### 缺失测试用例（❌ 需补充）

**高优先级**：
1. **Repeating Groups 验收测试** - 只有概念示例，无实际编解码测试
2. **变长数据（var-data）验收测试** - 未找到测试用例
3. **嵌套消息验收测试** - 只有概念示例，无实际测试

**中优先级**：
4. **时间类型验收测试** - 有示例但无验收测试
5. **十进制类型验收测试** - 有示例但无验收测试
6. **消息头验收测试** - 需要验证 8 字节头格式

**低优先级**：
7. **XML Schema 生成验收** - 有示例但无验收测试
8. **端到端集成验收** - 完整流程测试

## 验收测试架构

### 测试目录结构

```
lib/common/sbe_derive/
├── tests/
│   ├── basic_test.rs              # ✅ 已有
│   ├── comprehensive_test.rs      # ✅ 已有
│   ├── performance_test.rs        # ✅ 已有
│   ├── serde_comparison.rs        # ✅ 已有
│   └── acceptance/                # ❌ 新增
│       ├── mod.rs
│       ├── repeating_groups_acceptance.rs
│       ├── var_data_acceptance.rs
│       ├── nested_messages_acceptance.rs
│       ├── time_types_acceptance.rs
│       ├── decimal_types_acceptance.rs
│       ├── message_header_acceptance.rs
│       └── integration_acceptance.rs
└── acceptance_report.md           # ❌ 新增验收报告
```

### 验收标准

每个验收测试必须验证：

1. **编码正确性**：数据正确写入 buffer，字节序正确
2. **解码正确性**：数据正确读取，类型转换正确
3. **往返一致性**：encode → decode 数据完全一致
4. **性能要求**：< 1μs（符合 CLAUDE.md 低延迟标准）
5. **SBE 2.0 合规性**：符合 FIX SBE 2.0 规范

### 验收测试用例设计

#### 1. Repeating Groups 验收测试

**测试目标**：验证 Task C1 的 Acceptance Criteria
- Repeating Groups 可以正确编解码
- Group dimension 编码正确（numInGroup + blockLength）

**测试用例**：
```rust
#[test]
fn test_repeating_groups_encode_decode() {
    // 1. 创建包含 3 个订单的消息
    // 2. 编码 group dimension（numInGroup=3, blockLength=20）
    // 3. 编码每个 group entry
    // 4. 解码并验证 group dimension
    // 5. 迭代解码每个 entry
    // 6. 验证数据一致性
}

#[test]
fn test_empty_repeating_group() {
    // 验证 numInGroup=0 的情况
}

#[test]
fn test_nested_repeating_groups() {
    // 验证嵌套组（如果支持）
}
```

#### 2. 变长数据验收测试

**测试目标**：验证 Task 6 的 Acceptance Criteria
- 变长数据字段可以正确编解码

**测试用例**：
```rust
#[test]
fn test_var_data_encode_decode() {
    // 1. 创建包含变长数据的消息
    // 2. 编码长度前缀 + 数据
    // 3. 解码并验证长度和数据
}

#[test]
fn test_var_data_empty() {
    // 验证空变长数据（length=0）
}

#[test]
fn test_var_data_large() {
    // 验证大数据（> 1KB）
}
```

#### 3. 嵌套消息验收测试

**测试目标**：验证 Task 7 的 Acceptance Criteria
- 嵌套消息可以正确编解码

**测试用例**：
```rust
#[test]
fn test_nested_message_encode_decode() {
    // 1. 创建包含嵌套消息的父消息
    // 2. 编码父字段 + 嵌套字段（inline）
    // 3. 解码并验证偏移量计算
    // 4. 验证嵌套数据一致性
}
```

#### 4. 时间类型验收测试

**测试目标**：验证 Task C6 的 Acceptance Criteria
- 时间类型正确编解码
- 时区处理正确

**测试用例**：
```rust
#[test]
fn test_utc_timestamp() {
    // 验证 UTCTimestamp（int64 + int16）
}

#[test]
fn test_utc_date_only() {
    // 验证 UTCDateOnly（int32 days）
}

#[test]
fn test_utc_time_only() {
    // 验证 UTCTimeOnly（int32 + int16）
}
```

#### 5. 十进制类型验收测试

**测试目标**：验证 Task C3 的 Acceptance Criteria
- decimal 类型正确编解码
- 精度保持正确

**测试用例**：
```rust
#[test]
fn test_decimal_encode_decode() {
    // 验证 mantissa + exponent 编码
    // 验证精度保持（如 100.50 = 10050 * 10^-2）
}
```

#### 6. 消息头验收测试

**测试目标**：验证 Task 9 的 Acceptance Criteria
- 消息头可以正确生成
- 8 字节格式：blockLength(2) + templateId(2) + schemaId(2) + version(2)

**测试用例**：
```rust
#[test]
fn test_message_header_format() {
    // 验证消息头 8 字节布局
    // 验证字段顺序和字节序
}
```

#### 7. 端到端集成验收测试

**测试目标**：验证完整流程
- 创建消息 → 编码 → 传输 → 解码 → 验证

**测试用例**：
```rust
#[test]
fn test_end_to_end_trade_flow() {
    // 模拟完整的交易消息流程
    // 验证所有特性组合使用
}
```

## 验收报告格式

### 报告结构

```markdown
# SBE Derive 宏验收报告

生成时间：2026-03-04
执行环境：macOS Darwin 25.3.0

## 执行摘要

- 总测试数：X
- 通过：Y
- 失败：Z
- 跳过：W
- 覆盖率：P%

## 按计划任务验收

### Wave 1: 基础架构（Task 1-4, A1-A2）
- [✅] Task 1: 创建 sbe-derive crate - 通过
- [✅] Task 2: 字段属性解析 - 通过
- [✅] Task 3: 基本类型编码 - 通过
- [✅] Task 4: 基本类型解码 - 通过
- [✅] Task A1: 类型映射与偏移量计算 - 通过
- [✅] Task A2: 共享代码生成基础设施 - 通过

### Wave 2: SBE 标准特性（Task 5-10, C1-C4c）
- [✅] Task 5: 定长数组支持 - 通过
- [❌] Task 6: 变长数据支持 - 需补充测试
- [❌] Task 7: 嵌套消息 - 需补充测试
- [✅] Task 8: 枚举支持 - 通过
- [✅] Task 9: 消息头生成 - 通过
- [✅] Task 10: 版本字段 - 通过
- [❌] Task C1: Repeating Groups - 需补充测试
- [✅] Task C2: 可选字段与 nullValue - 通过
- [⚠️] Task C3: 十进制类型 - 有示例，需验收测试
- [✅] Task C4: 布尔类型 - 通过
- [⚠️] Task C4b: 定长数据类型 - 需验证
- [✅] Task C4c: char 字符类型 - 通过

### Wave 3: 高级特性（Task C5-C8）
- [✅] Task C5: 常量字段 - 通过
- [⚠️] Task C6: 时间类型 - 有示例，需验收测试
- [⚠️] Task C7: 值范围验证 - 有示例，需验证
- [⚠️] Task C8: MonthYear、Length、semanticType - 需验证

### Wave 4: 集成测试（Task 11-14, B1, D1-D2）
- [✅] Task 11: 示例代码 - 14个示例通过
- [✅] Task 12: 完整测试 - 5个测试通过
- [⚠️] Task 13: 替换现有实现 - 需验证
- [✅] Task B1: 性能基准测试 - 通过（< 1000ns）
- [✅] Task D1: Serde 性能对比 - 通过（SBE 优于 JSON/Bincode）
- [⚠️] Task D2: XML Schema 生成 - 有示例，需验证

## 性能验收

### 基准测试结果

| 操作 | 实测性能 | 目标 | 状态 |
|------|---------|------|------|
| 编码 | X ns/op | < 1000ns | ✅/❌ |
| 解码 | Y ns/op | < 1000ns | ✅/❌ |
| 往返 | Z ns/op | < 2000ns | ✅/❌ |

### Serde 对比

| 格式 | 编码倍数 | 解码倍数 | 大小倍数 |
|------|---------|---------|---------|
| vs JSON | Xx faster | Yx faster | Zx smaller |
| vs Bincode | Xx faster | Yx faster | Zx smaller |

## Must Have 合规性检查

- [✅] 消息头：8 bytes (blockLength + templateId + schemaId + version)
- [✅] 整数类型：int8/16/32/64, uint8/16/32/64
- [✅] 浮点类型：float (f32), double (f64)
- [✅] 字符类型：char (单字节)
- [⚠️] 字符串：定长数组 + 变长字符串
- [⚠️] 数据类型：定长 data + 变长 data
- [✅] 枚举：隐式枚举
- [✅] 可选字段：presence=optional + nullValue
- [✅] 常量字段：presence=constant
- [❌] Repeating Groups：重复组支持
- [✅] 版本控制：sinceVersion/deprecatedVersion
- [⚠️] 十进制类型：decimal (mantissa + exponent)
- [✅] 布尔类型：boolean encoding
- [⚠️] 时间类型：UTCTimestamp, UTCDateOnly, UTCTimeOnly
- [⚠️] 值范围：minValue, maxValue 验证
- [✅] 字节序：Little-Endian
- [✅] 生成代码：#[inline] 属性
- [✅] Block Length：静态计算
- [✅] 零拷贝：flyweight pattern
- [✅] 错误处理：边界检查

## 验收结论

### 通过项（✅）
- 基础类型编解码
- 枚举支持
- 可选字段
- 常量字段
- 版本控制
- 性能基准
- Serde 对比

### 需补充测试项（❌）
- Repeating Groups 实际测试
- 变长数据实际测试
- 嵌套消息实际测试

### 需验证项（⚠️）
- 时间类型验收
- 十进制类型验收
- XML Schema 生成验收
- 值范围验证
- 端到端集成测试

### 总体评估

**核心功能**：✅ 通过
**高级特性**：⚠️ 部分通过
**性能要求**：✅ 通过
**SBE 2.0 合规性**：⚠️ 大部分合规

### 建议

1. **高优先级**：补充 Repeating Groups、变长数据、嵌套消息的实际编解码测试
2. **中优先级**：为时间类型、十进制类型添加验收测试
3. **低优先级**：验证 XML Schema 生成、值范围验证

## 附录

### 测试命令

```bash
# 运行所有测试
cargo test --package sbe_derive

# 运行验收测试
cargo test --package sbe_derive --test acceptance

# 运行性能测试
cargo test --package sbe_derive --test performance_test -- --nocapture

# 运行 Serde 对比测试
cargo test --package sbe_derive --test serde_comparison -- --nocapture

# 运行基准测试
cargo bench --package sbe_derive
```

### 参考文档

- 计划文档：`.sisyphus/plans/completed/sbe-derive-macro.md`
- SBE 2.0 规范：FIX SBE 2.0 (Release Candidate 2, 2019-08-13)
- 低延迟标准：`CLAUDE.md` (目标 < 1μs)
```
