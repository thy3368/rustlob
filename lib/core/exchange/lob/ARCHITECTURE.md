# LOB 架构说明

## 概述

本订单簿（Limit Order Book）库采用 Clean Architecture 设计，提供高性能的订单匹配引擎。

## 核心原则

**MatchingService 是核心服务，OrderBook 是可选的便利封装。**

- ✅ **推荐**: 直接使用 `MatchingService` 获得最佳性能和控制
- ⚠️ **可选**: 使用 `OrderBook` 快速开发和简单场景

## 架构分层

### 1. 核心层（Domain Layer）

#### MatchingService - 订单匹配服务（核心）
系统的核心，实现价格-时间优先的订单匹配算法。

#### MarketDataService - 市场数据服务
提供市场数据查询功能。

### 2. 数据访问层

#### Repository 模块（完全模块化设计）
```
repository/
├── mod.rs (24行)       # 模块入口，重新导出公共 API
├── traits.rs (90行)    # 领域层接口定义
│   ├── OrderRepository trait       # 仓储接口
│   ├── RepositoryAccessor trait    # 访问器接口
│   └── RepositoryError enum        # 错误类型
└── in_memory.rs (529行) # 基础设施层实现
    ├── InMemoryOrderRepository     # 内存实现
    └── tests module                # 13 个单元测试
```

**设计原则**:
- ✅ **接口隔离**: `traits.rs` 定义抽象（领域层）
- ✅ **实现分离**: `in_memory.rs` 提供具体实现（基础设施层）
- ✅ **清晰导出**: `mod.rs` 提供简洁的公共 API
- ✅ **易于扩展**: 可添加 `redis.rs`、`postgres.rs` 等新实现
- ✅ **独立测试**: 每个实现有独立测试模块

**扩展示例**:
```rust
// 未来添加 Redis 实现
repository/
├── mod.rs
├── traits.rs
├── in_memory.rs
└── redis.rs      // 🆕 新增 Redis 实现
```

### 3. 可选便利层
- **OrderBook**: 可选的 Facade，提供简化API（已移除）

## 使用建议

### 高性能场景 → 使用 MatchingService
直接使用核心服务，零开销抽象。

### 简单场景 → 使用 OrderBook
快速开发，自动处理细节。

详细文档见模块注释和单元测试。
