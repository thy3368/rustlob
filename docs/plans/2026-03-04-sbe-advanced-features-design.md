# SBE Derive 宏高级特性实现设计

**日期**: 2026-03-04
**目标**: 实现未验收通过的5个高级特性
**参考**: `lib/common/sbe_derive/ACCEPTANCE_REPORT.md`

## 背景

根据验收报告，以下特性需要实现：
1. **Variable-Length Data**: 代码已集成，缺验收测试
2. **Repeating Groups**: 基础设施存在（groups.rs），未集成
3. **Nested Messages**: 基础设施存在（nested.rs），未集成
4. **Decimal Types**: 属性解析存在，未集成
5. **Time Types**: 类型定义存在，未集成

## 实现策略

**方案**: 逐个实现（方案A）
- 按优先级逐个实现和测试
- 每个特性完成后独立提交
- 符合TDD原则

**实现顺序**:
1. Variable-Length Data 测试（最快）
2. Repeating Groups（最常用）
3. Nested Messages（中等复杂度）
4. Decimal Types（金融场景）
5. Time Types（时间戳场景）

## 架构设计

所有特性遵循统一的集成模式：
1. 在 `codegen.rs` 中检测特定字段类型
2. 调用对应模块的生成函数
3. 生成编码器和解码器方法
4. 添加验收测试验证功能

---

## 特性1：Variable-Length Data 测试

### 当前状态
- 代码已完全集成到 `codegen.rs`
- 位置：lines 56-67, 182-184, 392-403, 536-539
- 检测：`TypeMapper::is_var_data()` 识别 `Vec<u8>` 类型

### 实现方案

**测试结构**:
```rust
#[test]
fn test_var_data_encode_decode() {
    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 200, schema_id = 1, version = 0)]
    struct VarDataMsg {
        #[sbe(id = 0)]
        fixed_field: u64,
        #[sbe(id = 1)]
        var_data: Vec<u8>,  // 变长数据
    }

    // 测试场景：
    // 1. 空数据（length=0）
    // 2. 小数据（< 100 bytes）
    // 3. 大数据（> 1KB）
    // 4. 往返一致性验证
}
```

### 验收标准
- ✅ 空数据编解码正确
- ✅ 小数据编解码正确
- ✅ 大数据编解码正确
- ✅ 往返一致性（encode → decode 数据不变）
- ✅ 长度前缀正确编码

---

## 特性2：Repeating Groups

### 当前状态
- 基础设施：`groups.rs` 包含 `generate_group_encoder` 和 `generate_group_decoder`
- 状态：函数存在但标记为 `#[allow(dead_code)]`，未被 `codegen.rs` 调用

### 实现方案

**集成点**: `codegen.rs` 中检测 `Vec<T>` 类型（T是结构体）

**编码格式**:
```
┌─────────────────────────────────────┐
│ Fixed Fields (block)                │
├─────────────────────────────────────┤
│ Group Dimension:                    │
│   - numInGroup (u16)                │
│   - blockLength (u16)               │
├─────────────────────────────────────┤
│ Group Entry 1 (blockLength bytes)   │
├─────────────────────────────────────┤
│ Group Entry 2 (blockLength bytes)   │
├─────────────────────────────────────┤
│ ...                                 │
└─────────────────────────────────────┘
```

**代码示例**:
```rust
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 300, schema_id = 1, version = 0)]
struct OrderBook {
    #[sbe(id = 0)]
    symbol: [u8; 8],
    #[sbe(id = 1)]
    levels: Vec<PriceLevel>,  // Repeating group
}

#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 301, schema_id = 1, version = 0)]
struct PriceLevel {
    #[sbe(id = 0)]
    price: f64,
    #[sbe(id = 1)]
    quantity: i32,
}
```

**集成步骤**:
1. 在 `codegen.rs` 中检测 `Vec<T>` 类型
2. 调用 `groups::generate_group_encoder()` 生成组编码器
3. 调用 `groups::generate_group_decoder()` 生成组解码器
4. 生成 group dimension 编码逻辑（numInGroup + blockLength）
5. 生成迭代器访问每个 entry

### 验收标准
- ✅ 空组（numInGroup=0）编解码正确
- ✅ 单个entry编解码正确
- ✅ 多个entries（3-5个）编解码正确
- ✅ Group dimension 正确编码
- ✅ 往返一致性

---

## 特性3：Nested Messages

### 当前状态
- 基础设施：`nested.rs` 包含：
  - `is_nested_message()` - 检测嵌套消息类型
  - `generate_nested_encoder_call()` - 生成嵌套编码器调用
  - `generate_nested_decoder_call()` - 生成嵌套解码器调用
- 状态：函数存在但标记为 `#[allow(dead_code)]`，未被 `codegen.rs` 调用

### 实现方案

**集成点**: `codegen.rs` 中检测结构体类型字段

**编码格式**: 内联编码（inline encoding）
```
┌─────────────────────────────────────┐
│ Message Header (8 bytes)            │
├─────────────────────────────────────┤
│ Parent Fields                       │
│   - trade_id (8 bytes)              │
├─────────────────────────────────────┤
│ Nested Message Fields (inline)      │
│   - price (8 bytes)                 │
│   - quantity (4 bytes)              │
└─────────────────────────────────────┘
```

**代码示例**:
```rust
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 400, schema_id = 1, version = 0)]
struct TradeWithPrice {
    #[sbe(id = 0)]
    trade_id: u64,
    #[sbe(id = 1)]
    price_level: PriceLevel,  // 嵌套消息，内联编码
}

#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 401, schema_id = 1, version = 0)]
struct PriceLevel {
    #[sbe(id = 0)]
    price: f64,
    #[sbe(id = 1)]
    quantity: i32,
}
```

**集成步骤**:
1. 在 `codegen.rs` 中使用 `nested::is_nested_message()` 检测结构体字段
2. 调用 `nested::generate_nested_encoder_call()` 生成编码逻辑
3. 调用 `nested::generate_nested_decoder_call()` 生成解码逻辑
4. 正确计算嵌套字段的偏移量（累加父字段大小）

### 验收标准
- ✅ 单层嵌套编解码正确
- ✅ 偏移量计算正确
- ✅ 往返一致性
- ✅ Block length 包含嵌套消息大小

---

## 特性4：Decimal Types

### 当前状态
- 属性解析：`attrs.rs` 中已支持 `mantissa_type` 和 `exponent` 属性
- 类型定义：`types.rs` 中已定义 `DecimalConfig` 结构
- 枚举变体：`FieldKind::Decimal` 已存在
- 状态：未集成到 `codegen.rs` 的编码/解码生成逻辑

### 实现方案

**编码格式**: mantissa + exponent
- mantissa: int64（尾数）
- exponent: int8（指数）
- 值 = mantissa × 10^exponent

**代码示例**:
```rust
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 500, schema_id = 1, version = 0)]
struct PriceMsg {
    #[sbe(id = 0, mantissa_type = "i64", exponent = "-2")]
    price: f64,  // 100.50 编码为 mantissa=10050, exponent=-2
}
```

**集成步骤**:
1. 在 `codegen.rs` 中检测 `mantissa_type` 和 `exponent` 属性
2. 生成 mantissa 编码逻辑（将 f64 转换为 mantissa）
3. 生成 exponent 编码逻辑（常量或字段）
4. 生成解码逻辑（mantissa × 10^exponent → f64）

### 验收标准
- ✅ 精度保持（100.50 往返后仍为 100.50）
- ✅ mantissa 正确编码
- ✅ exponent 正确编码
- ✅ 往返一致性

---

## 特性5：Time Types

### 当前状态
- 类型定义：`time_types.rs` 中已定义：
  - `UTCTimestamp` - 纳秒级时间戳（int64）
  - `UTCDateOnly` - 日期（int32 天数）
  - `UTCTimeOnly` - 时间（int32 秒 + int16 纳秒）
  - `MonthYear` - 年月格式
- 状态：所有类型标记为 `#[allow(dead_code)]`，未被 `codegen.rs` 使用

### 实现方案

**编码格式**:
- `UTCTimestamp`: int64（纳秒自 Unix epoch）
- `UTCDateOnly`: int32（天数自 Unix epoch）
- `UTCTimeOnly`: int32（秒） + int16（纳秒）
- `MonthYear`: int32（年月编码）

**代码示例**:
```rust
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 600, schema_id = 1, version = 0)]
struct TimestampMsg {
    #[sbe(id = 0, time_type = "UTCTimestamp")]
    timestamp: i64,  // 纳秒级时间戳

    #[sbe(id = 1, time_type = "UTCDateOnly")]
    date: i32,  // 天数
}
```

**集成步骤**:
1. 在 `codegen.rs` 中检测 `time_type` 属性
2. 根据 time_type 生成对应的编码逻辑
3. 生成解码逻辑
4. 使用 `time_types.rs` 中的类型定义

### 验收标准
- ✅ UTCTimestamp 正确编码（int64 纳秒）
- ✅ UTCDateOnly 正确编码（int32 天数）
- ✅ UTCTimeOnly 正确编码（int32 + int16）
- ✅ 往返一致性

---

## 实现计划

### 每个特性的交付标准
1. 集成到 `codegen.rs`（如需要）
2. 添加验收测试到 `tests/acceptance_tests.rs`
3. 测试通过
4. 更新 `ACCEPTANCE_REPORT.md`
5. 独立提交

### 提交消息格式
```
feat(sbe-derive): implement [feature name]

- Integrate [feature] into codegen.rs
- Add acceptance tests
- Update acceptance report

Test results: [X/X passed]

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
```

### 验收更新
每个特性完成后，更新 `ACCEPTANCE_REPORT.md`：
- 将状态从 ⚠️ PARTIAL 改为 ✅ IMPLEMENTED
- 添加测试证据
- 更新执行摘要统计

---

## 技术约束

### 性能要求（CLAUDE.md）
- 编码/解码延迟 < 1μs
- 零拷贝设计
- 最小化内存分配

### SBE 2.0 合规性
- 遵循 FIX SBE 2.0 规范
- Little-Endian 字节序
- 正确的消息头格式

### 代码质量
- 生成代码包含 `#[inline]` 属性
- 边界检查和错误处理
- 移除 `#[allow(dead_code)]` 标记

---

## 风险和缓解

### 风险1：Repeating Groups 复杂度高
- **缓解**: 先实现简单场景（单层组），后续支持嵌套组

### 风险2：Nested Messages 偏移量计算错误
- **缓解**: 使用现有的 `OffsetCalculator`，添加详细测试

### 风险3：Decimal Types 精度损失
- **缓解**: 使用固定指数，测试边界值

### 风险4：Time Types 时区处理
- **缓解**: 使用 UTC 时间戳，避免时区转换

---

## 成功标准

### 功能完整性
- ✅ 所有5个特性实现并通过测试
- ✅ 验收报告更新为 100% 通过

### 性能达标
- ✅ 所有特性编解码 < 1μs
- ✅ 无性能退化

### 代码质量
- ✅ 无编译警告
- ✅ 所有测试通过
- ✅ 代码符合项目规范
