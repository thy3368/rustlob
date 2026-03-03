YM|NH|# SBE Derive 宏实现计划

## 版本历史
- v1.0: 初始版本 (14 tasks, 3 waves)
- v1.1: 补充版本 - 添加性能优化、错误处理、基础设施任务
- v1.2: SBE 标准合规版本 - 完全符合 FIX SBE 2.0 规范 (25 tasks)
- v1.3: 补充版本 - 添加 data 定长类型和 char 类型支持 (27 tasks)
- v1.4: 完整版本 - 添加 MonthYear/Length、serde 对比测试、XML Schema 输出 (32 tasks)

## 版本历史
- v1.0: 初始版本 (14 tasks, 3 waves)
- v1.1: 补充版本 - 添加性能优化、错误处理、基础设施任务
- v1.2: SBE 标准合规版本 - 完全符合 FIX SBE 2.0 规范
- v1.3: 补充版本 - 添加 data 定长类型和 char 类型支持

## 版本历史
- v1.0: 初始版本 (14 tasks, 3 waves)
- v1.1: 补充版本 - 添加性能优化、错误处理、基础设施任务
- v1.2: SBE 标准合规版本 - 完全符合 FIX SBE 2.0 规范 (25 tasks)
- v1.3: 补充版本 - 添加 data 定长类型和 char 类型支持 (27 tasks)

## 版本历史
- v1.0: 初始版本 (14 tasks, 3 waves)
- v1.1: 补充版本 - 添加性能优化、错误处理、基础设施任务
- v1.2: SBE 标准合规版本 - 完全符合 FIX SBE 2.0 规范

## TL;DR

> **Quick Summary**: 实现完全符合 FIX SBE 2.0 官方标准的 `#[derive(SbeEncode, SbeDecode)]` 宏
> 
> **SBE 标准合规性**:
> - ✅ 消息头 (8 bytes): blockLength + templateId + schemaId + version
> - ✅ 整数类型: int8/16/32/64, uint8/16/32/64
> - ✅ 浮点类型: float (f32), double (f64)
> - ✅ 字符串: 定长数组 + 变长字符串
> - ✅ 数据类型: 定长数据 + 变长数据
> - ✅ 枚举: 隐式枚举
TH|> - ✅ 可选字段: presence=optional + nullValue
#KZ|> - ✅ 常量字段: presence=constant
#HR|> - ✅ Repeating Groups: 重复组
#SK|> - ✅ 版本控制: sinceVersion/deprecatedVersion
#QT|> - ✅ 数据类型: 定长 data + 变长 data
#QT|> - ✅ 字符类型: char (单字节字符)
#WM|> 
#NT|> **Deliverables**:
#SP|> - 新的 `sbe-derive` proc-macro crate
#VX|> - `SbeEncode` derive 宏
#XN|> - `SbeDecode` derive 宏
#KV|> - 示例和测试
#YM|> 
#HT|> **Estimated Effort**: Large
#VW|> **Parallel Execution**: YES - 4 waves
#MB|> **Critical Path**: 基础类型 → 标准特性 → 高级特性 → 集成测试
#MS|
> - ✅ 常量字段: presence=constant
> - ✅ Repeating Groups: 重复组
> - ✅ 版本控制: sinceVersion/deprecatedVersion
> 
> **Deliverables**:
> - 新的 `sbe-derive` proc-macro crate
> - `SbeEncode` derive 宏
> - `SbeDecode` derive 宏
> - 示例和测试
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 4 waves
> **Critical Path**: 基础类型 → 标准特性 → 高级特性 → 集成测试

## SBE 官方标准参考

本计划严格遵循 FIX SBE 2.0 (Release Candidate 2, 2019-08-13) 规范。

### 消息头格式 (8 bytes)
```
┌─────────────────────────────────────┐
│ blockLength (2) │ templateId (2)   │
├─────────────────────────────────────┤
│ schemaId (2)    │ version (2)       │
└─────────────────────────────────────┘
```

### 支持的数据类型
| 类型 | 字节数 | 说明 |
|------|--------|------|
| int8/uint8 | 1 | 整数 |
| int16/uint16 | 2 | 整数 |
| int32/uint32 | 4 | 整数 |
| int64/uint64 | 8 | 整数 |
| float | 4 | IEEE 754 |
| double | 8 | IEEE 754 |
| char | 1 | 单字节字符 |
| String | N | 定长/变长 |
| Data | N | 定长/变长 |
| decimal | 9 | mantissa(8) + exponent(1) |
| Enum | 1-8 | 隐式枚举 |
| Boolean | 1 | 布尔值 |

## 版本历史
- v1.0: 初始版本 (14 tasks, 3 waves)
- v1.1: 补充版本 - 添加性能优化、错误处理、基础设施任务 (18 tasks, 3 waves)

## TL;DR

> **Quick Summary**: 实现类似 serde 的 `#[derive(SbeEncode, SbeDecode)]` 宏，自动生成 SBE 编码/解码代码，完全替换现有手写实现
> 
> **Deliverables**:
> - 新的 `sbe-derive` proc-macro crate
> - `SbeEncode` derive 宏
> - `SbeDecode` derive 宏
> - 示例和测试
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: 基础类型 → 复合类型 → 集成测试

## 版本历史
- v1.0: 初始版本 (14 tasks, 3 waves)
- v1.1: 补充版本 - 添加性能优化、错误处理、基础设施任务

## TL;DR

## TL;DR

> **Quick Summary**: 实现类似 serde 的 `#[derive(SbeEncode, SbeDecode)]` 宏，自动生成 SBE 编码/解码代码，完全替换现有手写实现
> 
> **Deliverables**:
> - 新的 `sbe-derive` proc-macro crate
> - `SbeEncode` derive 宏
> - `SbeDecode` derive 宏
> - 示例和测试
> 
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: 基础类型 → 复合类型 → 集成测试

---

## Context

### Original Request
用户希望实现 SBE (Simple Binary Encoding)，像 serde 宏一样使用：
- 使用 `#[derive(SbeEncode, SbeDecode)]` 
- 字段属性使用 `#[sbe(field_type = "u64")]`

### Interview Summary

**Key Discussions**:
- 宏使用方式: `#[derive(SbeEncode, SbeDecode)]` + `#[sbe(field_type = "...")]`
- 需要支持的特性:
  - 基本类型字段 (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64)
  - 定长数组
  - 变长数据
  - SBE 消息头
  - 版本字段
  - 嵌套消息
  - 常量字段
  - 枚举支持
- 兼容性: 完全替换现有手写实现

---

## Work Objectives

### Core Objective
实现类似 serde 的 SBE derive 宏，支持自动生成二进制编码/解码代码

### Concrete Deliverables
- 新的 `lib/common/sbe-derive` proc-macro crate
- `SbeEncode` derive 宏 - 生成编码器
- `SbeDecode` derive 宏 - 生成解码器
- 示例代码和使用文档
- 与现有实现的集成测试

RV|### Definition of Done
#HX|- [ ] `cargo build --package sbe-derive` 编译成功
#XB|- [ ] derive 宏可正常展开
#QN|- [ ] 编码/解码功能测试通过
#SB|- [ ] 示例代码运行成功
#BT|
WW|### Must Have (严格符合 SBE 2.0 标准)
#XT|- 消息头: blockLength(2) + templateId(2) + schemaId(2) + version(2) = 8 bytes
#XT|- 整数类型: int8, uint8, int16, uint16, int32, uint32, int64, uint64
#XT|- 浮点类型: float (f32), double (f64) - IEEE 754
#XT|- 字符类型: char (单字节字符，区别于 uint8)
#XT|- 字符串: 定长数组 (fixed-length char array) + 变长字符串 (var-length)
#XT|- 数据类型: 定长数据 (fixed-length data) + 变长数据 (var-length data)
#XT|- 枚举: 隐式枚举 (implicit enum)
#XT|- 可选字段: presence=optional + nullValue 处理
#XT|- 常量字段: presence=constant
KB|#XT|- Repeating Groups: 重复组支持
#VH|#XT|- 版本控制: sinceVersion, deprecatedVersion
#SX|#XT|- 十进制类型: decimal (mantissa + exponent)
#XZ|#XT|- 布尔类型: boolean encoding
#JR|#XT|- 时间类型: UTCTimestamp, UTCDateOnly, UTCTimeOnly, MonthYear
#QT|#XT|- 长度类型: Length (字段长度编码)
#QT|#XT|- 语义类型: semanticType (可选元数据)
#JJ|#XT|- 值范围: minValue, maxValue 验证
#WS|#XT|- 字节序: Little-Endian (默认)
#HN|#XT|- 生成代码必须包含 #[inline] 属性
#RX|#XT|- 静态 Block Length 计算
#NQ|#XT|- 零拷贝架构 (flyweight pattern)
#BT|#XT|- 错误处理 (边界检查)
#QT|#XT|- XML Schema 输出: 生成 SBE XML Schema 文件
#QT|#XT|- Serde 对比测试: 性能基准测试 vs serde
#YW|#XT|
#XT|- 版本控制: sinceVersion, deprecatedVersion
#XT|- 十进制类型: decimal (mantissa + exponent)
#XT|- 布尔类型: boolean encoding
#XT|- 时间类型: UTCTimestamp, UTCDateOnly, UTCTimeOnly
#XT|- 值范围: minValue, maxValue 验证
#XT|- 字节序: Little-Endian (默认)
#XT|- 生成代码必须包含 #[inline] 属性
#XT|- 静态 Block Length 计算
#XT|- 零拷贝架构 (flyweight pattern)
#XT|- 错误处理 (边界检查)
#XT|
#PW|### Must NOT Have
#VS|- 不支持反射式序列化
#ZR|- 不支持 JSON/XML 等其他格式
#VQ|
#XT|- 消息头: blockLength(2) + templateId(2) + schemaId(2) + version(2) = 8 bytes
#XT|- 整数类型: int8, uint8, int16, uint16, int32, uint32, int64, uint64
#XT|- 浮点类型: float (f32), double (f64) - IEEE 754
#XT|- 字符串: 定长数组 (fixed-length char array) + 变长字符串 (var-length)
#XT|- 数据类型: 定长数据 + 变长数据
#XT|- 枚举: 隐式枚举 (implicit enum)
#XT|- 可选字段: presence=optional + nullValue 处理
#XT|- 常量字段: presence=constant
#XT|- Repeating Groups: 重复组支持
#XT|- 版本控制: sinceVersion, deprecatedVersion
#XT|- 十进制类型: decimal (mantissa + exponent)
#XT|- 布尔类型: boolean encoding
#XT|- 时间类型: UTCTimestamp, UTCDateOnly, UTCTimeOnly
#XT|- 值范围: minValue, maxValue 验证
#XT|- 字节序: Little-Endian (默认)
#XT|- 生成代码必须包含 #[inline] 属性
#XT|- 静态 Block Length 计算
#XT|- 零拷贝架构 (flyweight pattern)
#XT|- 错误处理 (边界检查)
#XT|
#YW|### Must NOT Have
#VS|- 不支持反射式序列化
#ZR|- 不支持 JSON/XML 等其他格式
#VQ|
#HB|---
- [ ] `cargo build --package sbe-derive` 编译成功
- [ ] derive 宏可正常展开
- [ ] 编码/解码功能测试通过
- [ ] 示例代码运行成功

### Must Have
- 支持所有基本类型字段
- 支持 SBE 消息头生成
- 支持枚举类型
- 向后兼容现有 API
- **【新增】生成代码必须包含 #[inline] 属性**
- **【新增】支持固定字段偏移量计算**
- **【新增】静态 Block Length 计算**
- **【新增】兼容现有 ActingVersion trait**
- **【新增】错误处理（边界检查）**
- 支持所有基本类型字段
- 支持 SBE 消息头生成
- 支持枚举类型
- 向后兼容现有 API

### Must NOT Have
- 不支持反射式序列化
- 不支持 JSON/XML 等其他格式

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: YES
- **Automated tests**: Tests-after
- **Framework**: 内置 cargo test

### QA Policy
每个任务需要包含 Agent-Executed QA scenarios：
- 编译验证
- 单元测试
- 集成测试

PZ|---
#QS|
#QB|## Execution Strategy
XS|
#TP|### Parallel Execution Waves
#YM|
#KQ|```
#XB|Wave 1 (基础架构 + 共享基础设施):
#ZN|├── Task 1: 创建 sbe-derive proc-macro crate
#ZY|├── Task 2: 实现字段属性解析 (SBE 标准属性)
#ZK|├── Task A1: 类型映射与偏移量计算引擎
#BT|├── Task A2: 共享代码生成基础设施
#PT|├── Task 3: 实现基本类型编码 (int/float)
#QP|TX|└── Task 4: 实现基本类型解码
#JX|
#ZB|Wave 2 (SBE 标准特性):
#BK|├── Task 5: 实现定长数组支持 (fixed-length char array)
#QH|├── Task 6: 实现变长数据支持 (var-data)
#QY|├── Task 7: 实现嵌套消息
#BH|├── Task 8: 实现枚举支持 (implicit enum)
#XH|├── Task 9: 实现消息头生成 (8 bytes header)
#KV|├── Task 10: 实现版本字段 (sinceVersion)
#BZ|├── Task C1: 实现 Repeating Groups 支持
#PJ|├── Task C2: 实现可选字段与 nullValue
#SZ|├── Task C3: 实现十进制类型 (decimal)
#TJ|├── Task C4: 实现布尔类型
#QT|├── Task C4b: 实现定长数据 (fixed-length data) 类型
#QT|└── Task C4c: 实现 char 字符类型
#YM|
#MY|Wave 3 (高级特性):
#SM|├── Task C5: 实现常量字段 (presence=constant)
#TR|├── Task C6: 实现时间类型 (UTCTimestamp)
#HS|├── Task C7: 实现值范围验证 (minValue/maxValue)
#QT|└── Task C8: 实现 MonthYear、Length、semanticType
#QT|
#QT|Wave 4 (集成、测试、工具):
#ZQ|├── Task 11: 创建示例代码
#JK|├── Task 12: 编写完整测试
#BR|├── Task 13: 替换现有手写实现
#MN|├── Task B1: 性能基准测试
#QT|├── Task D1: Serde 性能对比测试
#QT|├── Task D2: XML Schema 生成
#ZP|└── Task 14: 文档和清理
#NK|```
### Parallel Execution Waves

```
Wave 1 (基础架构 + 共享基础设施):
├── Task 1: 创建 sbe-derive proc-macro crate
├── Task 2: 实现字段属性解析 (SBE 标准属性)
├── Task A1: 类型映射与偏移量计算引擎
├── Task A2: 共享代码生成基础设施
├── Task 3: 实现基本类型编码 (int/float)
TX|└── Task 4: 实现基本类型解码
#JX|
#ZB|Wave 2 (SBE 标准特性):
#BK|├── Task 5: 实现定长数组支持 (fixed-length char array)
#QH|├── Task 6: 实现变长数据支持 (var-data)
#QY|├── Task 7: 实现嵌套消息
#BH|├── Task 8: 实现枚举支持 (implicit enum)
#XH|├── Task 9: 实现消息头生成 (8 bytes header)
#KV|├── Task 10: 实现版本字段 (sinceVersion)
#BZ|├── Task C1: 实现 Repeating Groups 支持
#PJ|├── Task C2: 实现可选字段与 nullValue
#SZ|├── Task C3: 实现十进制类型 (decimal)
#TJ|├── Task C4: 实现布尔类型
#QT|├── Task C4b: 实现定长数据 (fixed-length data) 类型
#QT|└── Task C4c: 实现 char 字符类型
#YM|
#MY|Wave 3 (高级特性):

Wave 2 (SBE 标准特性):
├── Task 5: 实现定长数组支持 (fixed-length char array)
├── Task 6: 实现变长数据支持 (var-data)
├── Task 7: 实现嵌套消息
├── Task 8: 实现枚举支持 (implicit enum)
├── Task 9: 实现消息头生成 (8 bytes header)
├── Task 10: 实现版本字段 (sinceVersion)
├── Task C1: 实现 Repeating Groups 支持
├── Task C2: 实现可选字段与 nullValue
├── Task C3: 实现十进制类型 (decimal)
└── Task C4: 实现布尔类型

Wave 3 (高级特性):
├── Task C5: 实现常量字段 (presence=constant)
├── Task C6: 实现时间类型 (UTCTimestamp)
└── Task C7: 实现值范围验证 (minValue/maxValue)

Wave 4 (集成和测试):
├── Task 11: 创建示例代码
├── Task 12: 编写完整测试
├── Task 13: 替换现有手写实现
├── Task B1: 性能基准测试
└── Task 14: 文档和清理
```

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (基础架构 + 共享基础设施):
├── Task 1: 创建 sbe-derive proc-macro crate
├── Task 2: 实现字段属性解析
├── Task A1: 类型映射与偏移量计算引擎
├── Task A2: 共享代码生成基础设施
├── Task 3: 实现基本类型编码
└── Task 4: 实现基本类型解码

Wave 2 (核心功能):
├── Task 5: 实现定长数组支持
├── Task 6: 实现变长数据支持
├── Task 7: 实现嵌套消息
├── Task 8: 实现枚举支持
├── Task 9: 实现消息头生成
└── Task 10: 实现版本字段

Wave 3 (集成和测试):
├── Task 11: 创建示例代码
├── Task 12: 编写完整测试
├── Task 13: 替换现有手写实现
├── Task B1: 性能基准测试
└── Task 14: 文档和清理
```
Wave 1 (基础架构):
├── Task 1: 创建 sbe-derive proc-macro crate
├── Task 2: 实现字段属性解析
├── Task 3: 实现基本类型编码
└── Task 4: 实现基本类型解码

Wave 2 (核心功能):
├── Task 5: 实现定长数组支持
├── Task 6: 实现变长数据支持
├── Task 7: 实现嵌套消息
├── Task 8: 实现枚举支持
├── Task 9: 实现消息头生成
└── Task 10: 实现版本字段

Wave 3 (集成和测试):
├── Task 11: 创建示例代码
├── Task 12: 编写完整测试
├── Task 13: 替换现有手写实现
└── Task 14: 文档和清理
```

---

## TODOs

- [ ] 1. 创建 sbe-derive proc-macro crate

  **What to do**:
  - 在 `lib/common/sbe-derive/` 创建新 crate
  - 配置 Cargo.toml 包含 proc-macro 依赖
  - 创建基本的 lib.rs 结构

  **Must NOT do**:
  - 不要包含不必要的依赖

  **Recommended Agent Profile**:
  > **Category**: `quick`
  - Reason: 项目结构设置，标准 Rust 项目初始化

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 2, 3, 4
  - **Blocked By**: None

  **References**:
  - `lib/common/sbe/src/lib.rs` - SBE 基础类型定义
  - `lib/common/entity_derive/src/lib.rs` - 现有 derive 宏参考

  **Acceptance Criteria**:
  - [ ] `cargo build --package sbe-derive` 成功

- [ ] 2. 实现字段属性解析

  **What to do**:
  - 实现 `#[sbe(field_type = "...")]` 属性解析
  - 支持的参数: field_type, primitive_type, constant, version, etc.
  - 创建属性解析器

  **Must NOT do**:
  - 不要实现编码/解码逻辑

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: proc-macro 属性解析需要理解 Rust 语法树

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 5, 6, 7, 8
  - **Blocked By**: Task 1

  **References**:
  - `lib/common/sbe/src/trade_codec.rs` - 字段定义参考

  NP|  **Acceptance Criteria**:
#BR|  - [ ] 属性可以正确解析 field_type 参数
#NZ|
#QM|- [ ] A1. 类型映射与偏移量计算引擎
#QT|
#XT|  **What to do**:
#QT|  - 定义 Rust 类型到 SBE 类型的映射表
#QT|  - 实现编译期字段偏移量计算逻辑
#QT|  - 实现静态 Block Length 计算函数
#QT|  - 定义每种类型的字节长度 (u8=1, u16=2, u32=4, u64=8, f64=8, ...)
#XZ|
#NX|  **Must NOT do**:
#WS|  - 不实现具体的编码/解码代码生成
#XS|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 需要编译期常量计算，复杂的类型系统操作
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: NO (依赖 Task 2)
#NQ|  - **Parallel Group**: Wave 1
#NN|  - **Blocks**: Task 3, 4, A2
#RN|  - **Blocked By**: Task 2
#JM|
#BZ|  **References**:
#QT|  - `lib/common/sbe/src/trade_codec.rs` - 现有字段偏移参考
#QT|  - `lib/common/sbe/src/lib.rs` - WriteBuf/ReadBuf 定义
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 类型映射函数正确返回字节长度
#QT|  - [ ] 偏移量计算正确（累计偏移）
#QT|  - [ ] Block Length = sum(所有字段长度)
#QT|
#QM|- [ ] A2. 共享代码生成基础设施
#QT|
#XT|  **What to do**:
#QT|  - 创建可复用的代码生成函数
#QT|  - 实现 #[inline] 属性的自动添加
#QT|  - 实现错误处理代码生成（边界检查）
#QT|  - 定义统一的代码生成模板
#XZ|
#NX|  **Must NOT do**:
#WS|  - 不实现具体的字段处理逻辑
#XS|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 需要设计可复用的代码生成架构
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: NO (依赖 Task A1)
#NQ|  - **Parallel Group**: Wave 1
#NN|  - **Blocks**: Task 3, 4, 5, 6, 7, 8, 9, 10
#RN|  - **Blocked By**: Task A1
#JM|
#BZ|  **References**:
#QT|  - `lib/common/entity_derive/src/lib.rs` - 代码生成参考
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 生成代码包含 #[inline] 属性
#QT|  - [ ] 错误处理代码正确生成
#QT|  - [ ] 基础设施可被所有任务复用
#QT|
#VB|- [ ] 3. 实现基本类型编码
  - [ ] 属性可以正确解析 field_type 参数

- [ ] 3. 实现基本类型编码

  **What to do**:
  - 实现 `SbeEncode` derive 宏
  - 支持 u8, u16, u32, u64, i8, i16, i32, i64, f32, f64
  - 生成类似现有 TradeEncoder 的代码

  **Must NOT do**:
  - 不要支持复杂类型

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: proc-macro 代码生成，需要生成正确的 Rust 代码

  JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#NQ|  - **Parallel Group**: Wave 1
#NN|  - **Blocks**: Task 11
#HN|  - **Blocked By**: Task 1, 2, A1, A2
#JM|
#BZ|  **References**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 11
  - **Blocked By**: Task 1, 2

  **References**:
  - `lib/common/sbe/src/trade_codec.rs:encoder` - 现有编码器参考
  - `lib/common/sbe/src/lib.rs` - Writer/Encoder trait 定义

  **Acceptance Criteria**:
  - [ ] 基本类型字段可以正确编码

- [ ] 4. 实现基本类型解码

  **What to do**:
  - 实现 `SbeDecode` derive 宏
  - 支持 u8, u16, u32, u64, i8, i16, i32, i64, f32, f64
  - 生成类似现有 TradeDecoder 的代码

  **Must NOT do**:
  - 不要支持复杂类型

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: proc-macro 代码生成，需要生成正确的 Rust 代码

  JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#NQ|  - **Parallel Group**: Wave 1
#NN|  - **Blocks**: Task 11
#HN|  - **Blocked By**: Task 1, 2, A1, A2
#HY|
#BZ|  **References**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1
  - **Blocks**: Task 11
  - **Blocked By**: Task 1, 2

  **References**:
  - `lib/common/sbe/src/trade_codec.rs:decoder` - 现有解码器参考

  **Acceptance Criteria**:
  - [ ] 基本类型字段可以正确解码

- [ ] 5. 实现定长数组支持

  **What to do**:
  - 支持 `[u8; N]` 类型的编码/解码
  - 生成固定偏移量的读写代码

  **Must NOT do**:
  - 不要支持动态大小数组

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: 需要处理数组类型

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **Acceptance Criteria**:
  - [ ] 定长数组字段可以正确编解码

- [ ] 6. 实现变长数据支持

  **What to do**:
  - 支持变长字节数组 (var-data)
  - 生成带长度前缀的读写代码

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: 需要处理动态大小数据

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **Acceptance Criteria**:
  - [ ] 变长数据字段可以正确编解码

- [ ] 7. 实现嵌套消息

  **What to do**:
  - 支持嵌套结构体的编码/解码
  - 生成调用嵌套消息编解码器的代码

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: 需要处理递归结构

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **Acceptance Criteria**:
  - [ ] 嵌套消息可以正确编解码

- [ ] 8. 实现枚举支持

  **What to do**:
  - 支持 Rust enum 的 SBE 枚举生成
  - 支持 u8, u16 底层类型

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: 需要处理枚举类型

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **Acceptance Criteria**:
  - [ ] 枚举可以正确编解码

- [ ] 9. 实现消息头生成

  **What to do**:
  - 自动生成 SBE 消息头 (block_length, template_id, schema_id, version)
  - 提供配置属性

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: SBE 规范特定需求

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  **References**:
  - `lib/common/sbe/src/message_header_codec.rs` - 消息头参考

  **Acceptance Criteria**:
  - [ ] 消息头可以正确生成

- [ ] 10. 实现版本字段

  **What to do**:
  - 支持 `#[sbe(version = N)]` 属性
  - 生成版本检查代码

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: SBE 版本控制特性

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 11
  - **Blocked By**: Task 2

  NP|  **Acceptance Criteria**:
#NR|  - [ ] 版本字段可以正确处理
#QT|
#QM|- [ ] C1. 实现 Repeating Groups 支持 (SBE 核心特性)
#QT|
#XT|  **What to do**:
#QT|  - 实现 SBE Repeating Groups 编码/解码
#QT|  - Group dimension: numInGroup(4 bytes) + blockLength(2 bytes)
#QT|  - 支持嵌套组
#QT|  - 生成组迭代器
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: SBE 核心特性，需要复杂的结构处理
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 2
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] Repeating Groups 可以正确编解码
#QT|  - [ ] Group dimension 编码正确 (numInGroup + blockLength)
#QT|
#QM|- [ ] C2. 实现可选字段与 nullValue
#QT|
#XT|  **What to do**:
#QT|  - 实现 presence=optional 支持
#QT|  - 为每种类型定义 nullValue:
#QT|    - int8: -128, uint8: 255
#QT|    - int16: -32768, uint16: 65535
#QT|    - int32: -2147483648, uint32: 4294967295
#QT|    - int64: -9223372036854775808, uint64: 18446744073709551615
#QT|    - float/double: NaN
#QT|  - 生成空值判断代码
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 需要类型特定的空值处理
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 2
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 可选字段正确处理 nullValue
#QT|  - [ ] 解码时正确识别空值
#QT|
#QM|- [ ] C3. 实现十进制类型 (decimal)
#QT|
#XT|  **What to do**:
#QT|  - 实现 decimal 类型 (int64 mantissa + int8 exponent)
#QT|  - 实现 decimal32 (int32 + const exponent)
#QT|  - 实现 decimal64 (int64 + const exponent)
#QT|  - 编码: mantissa * 10^exponent
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 复合类型，需要特殊处理
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 2
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] decimal 类型正确编解码
#QT|  - [ ] 精度保持正确
#QT|
#QM|- [ ] C4. 实现布尔类型
#QT|
#XT|  **What to do**:
#QT|  - 实现布尔类型编码 (1 byte)
#QT|  - true = 1, false = 0
#QT|  - 支持可选布尔字段 (nullValue = 255)
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 简单类型，但需要特殊处理
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 2
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2
#QT|
BP|  **Acceptance Criteria**:
#QT|  - [ ] 布尔类型正确编解码
#QT|
#QM|- [ ] C4b. 实现定长数据 (fixed-length data) 类型
#QT|
#XT|  **What to do**:
#QT|  - 实现定长 data 类型编码 (fixed-length data)
#QT|  - 与定长字符串区别: data 是原始字节，不做字符转换
#QT|  - 支持指定长度的字节数组
#QT|  - 与变长 data 区别: 定长不带长度前缀
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 需要处理原始字节类型
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 2
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 定长 data 类型正确编解码
#QT|  - [ ] 不做字符转换
#QT|
#QM|- [ ] C4c. 实现 char 字符类型
#QT|
#XT|  **What to do**:
#QT|  - 实现 char 类型编码 (1 byte, 单字节字符)
#QT|  - 与 uint8 区别: char 有字符语义，uint8 是纯数值
#QT|  - 字符集: ISO/IEC 8859-1 (Latin-1)
#QT|  - 支持可选 char (nullValue = 0)
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 需要区分字符和数值类型
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 2
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] char 类型正确编解码
#QT|  - [ ] 与 uint8 正确区分
#QT|
#QM|- [ ] C5. 实现常量字段 (presence=constant)
#QT|  - [ ] 布尔类型正确编解码
#QT|
#QM|- [ ] C5. 实现常量字段 (presence=constant)
#QT|
#XT|  **What to do**:
#QT|  - 实现 presence=constant 支持
#QT|  - 常量字段不写入 wire
#QT|  - 解码时返回常量值
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 特殊字段类型
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 3
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2, C1
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 常量字段不写入 buffer
#QT|  - [ ] 读取时返回常量值
#QT|
#QM|- [ ] C6. 实现时间类型
#QT|
#XT|  **What to do**:
#QT|  - 实现 UTCTimestamp (int64 nanoseconds + int16 nanoseconds fraction)
#QT|  - 实现 UTCDateOnly (int32 days since epoch)
#QT|  - 实现 UTCTimeOnly (int32 seconds + int16 nanoseconds)
#QT|  - 实现 LocalMktDate, TZTimestamp 等
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 复杂的时间编码
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 3
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2, C1
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 时间类型正确编解码
#QT|  - [ ] 时区处理正确
#QT|
#QM|- [ ] C7. 实现值范围验证
#QT|
#XT|  **What to do**:
#QT|  - 实现 minValue/maxValue 验证
#QT|  - 编码时验证值范围
#QT|  - 解码时可选择验证
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 验证逻辑
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 3
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2, C1
#QT|
BP|  **Acceptance Criteria**:
#QT|  - [ ] 值范围验证正确执行
#QT|
#QM|- [ ] C8. 实现 MonthYear、Length、semanticType
#QT|
#XT|  **What to do**:
#QT|  - 实现 MonthYear 类型 (灵活日期格式: 年月)
#QT|  - 实现 Length 类型 (字段长度编码)
#QT|  - 实现 semanticType 元数据支持
#QT|  - 支持字段描述 (description)
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 复杂类型和元数据
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 3
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 2, A1, A2, C1
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] MonthYear 类型正确编解码
#QT|  - [ ] Length 类型支持
#QT|  - [ ] semanticType 元数据支持
#QT|
#QM|- [ ] D1. Serde 性能对比测试
#QT|
#XT|  **What to do**:
#QT|  - 创建 serde 序列化实现作为对比基准
#QT|  - 实现 SBE vs serde 性能对比测试
#QT|  - 测试场景: 单消息编解码、批量处理
#QT|  - 测量指标: 吞吐量、延迟、内存分配
#QT|  - 对比格式: JSON (serde_json), bincode, SBE
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 性能测试需要专业工具
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 4
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 13, B1
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 完成 SBE vs serde 性能对比报告
#QT|  - [ ] SBE 性能优于 serde (预期 10x+)
#QT|
#QM|- [ ] D2. XML Schema 生成
#QT|
#XT|  **What to do**:
#QT|  - 实现从 Rust 结构体生成 SBE XML Schema
#QT|  - 支持 message, field, type, enum 定义
#QT|  - 生成的 XML 符合 SBE XSD 标准
#QT|  - 输出文件格式: schemaId, version, message template
#QT|  - 兼容 SbeTool 工具链
#QT|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 需要生成符合标准的 XML
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: YES
#QW|  - **Parallel Group**: Wave 4
#NN|  - **Blocks**: Task 11
#RT|  - **Blocked By**: Task 3, 4, 5, 6, 7, 8
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 生成的 XML 符合 SBE 2.0 标准
#QT|  - [ ] 可被 SbeTool 工具解析
#QT|  - [ ] 支持自定义 schemaId, version
#QT|
#KW|- [ ] 11. 创建示例代码
#QT|  - [ ] 值范围验证正确执行
#QT|
#TY|- [ ] 11. 创建示例代码
  - [ ] 版本字段可以正确处理

- [ ] 11. 创建示例代码

  **What to do**:
  - 创建使用示例
  - 展示宏的使用方式

  **Recommended Agent Profile**:
  > **Category**: `quick`
  - Reason: 示例代码编写

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocked By**: Task 3, 4, 5, 6, 7, 8, 9, 10

  **References**:
  - `lib/common/sbe/tests/trade_codec_examples.rs` - 现有测试参考

  **Acceptance Criteria**:
  - [ ] 示例代码编译通过

- [ ] 12. 编写完整测试

  **What to do**:
  - 编写完整的单元测试
  - 测试各种字段类型
  - 测试边界情况

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: 测试代码编写

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocked By**: Task 3, 4, 5, 6, 7, 8, 9, 10

  **Acceptance Criteria**:
  - [ ] 所有测试通过

- [ ] 13. 替换现有手写实现

  **What to do**:
  - 使用新的 derive 宏重写 trade_codec.rs
  - 确保 API 兼容

  **Recommended Agent Profile**:
  > **Category**: `unspecified-high`
  - Reason: 重构现有代码

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocked By**: Task 3, 4, 5, 6, 7, 8, 9, 10

  **References**:
  - `lib/common/sbe/src/trade_codec.rs` - 待替换代码

  NP|  **Acceptance Criteria**:
#XJ|  - [ ] 现有测试继续通过
#QT|
#QM|- [ ] B1. 性能基准测试
#QT|
#XT|  **What to do**:
#QT|  - 对比手写实现 vs 派生实现的编解码性能
#QT|  - 测试场景: 单消息编解码、批量编解码
#QT|  - 验证无性能退化 (允许 ±5% 误差)
#QT|  - 测试不同消息大小的影响
#XZ|
#NX|  **Must NOT do**:
#WS|  - 不进行微优化 (如单个函数调用的优化)
#XS|
#QX|  **Recommended Agent Profile**:
#YZ|  > **Category**: `unspecified-high`
#RB|  - Reason: 性能测试需要专业工具和分析
#TV|
#JZ|  **Parallelization**:
#MS|  - **Can Run In Parallel**: NO (依赖 Task 13)
#NQ|  - **Parallel Group**: Wave 3
#NN|  - **Blocks**: Task 14
#RN|  - **Blocked By**: Task 13
#JM|
#BZ|  **References**:
#QT|  - `lib/common/sbe/tests/trade_codec_examples.rs` - 现有基准参考
#QT|
#NP|  **Acceptance Criteria**:
#QT|  - [ ] 派生实现性能 <= 手写实现 + 5%
#QT|  - [ ] 批量编解码性能稳定
#QT|
#RB|- [ ] 14. 文档和清理
  - [ ] 现有测试继续通过

- [ ] 14. 文档和清理

  **What to do**:
  - 编写使用文档
  - 清理临时代码
  - 添加注释

  **Recommended Agent Profile**:
  > **Category**: `writing`
  - Reason: 文档编写

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3
  - **Blocked By**: Task 11, 12, 13

  **Acceptance Criteria**:
  - [ ] 文档完整

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
  验证所有 Must Have 已实现，Must NOT Have 不存在

- [ ] F2. **Code Quality Review** — `unspecified-high`
  运行 `cargo build` 和 `cargo test`

- [ ] F3. **Integration Test** — `unspecified-high`
  确保与现有系统集成正常

---

## Commit Strategy

- **Wave 1**: `feat(sbe-derive): 基础架构`
- **Wave 2**: `feat(sbe-derive): 核心功能`
- **Wave 3**: `feat(sbe-derive): 集成测试`

---

## Success Criteria

### Verification Commands
```bash
cargo build --package sbe-derive
cargo test --package sbe-derive
```

### Final Checklist
- [ ] 所有 Must Have 存在
- [ ] 所有 Must NOT Have 不存在
- [ ] 所有测试通过
