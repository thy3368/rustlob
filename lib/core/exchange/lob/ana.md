# 领域

- Command,CommandResult,Handler,CommandRepo,HandlerRepo
- Order,Trade,EntityEvent,EntityEventRepo

## 行情数据

## 性能优化 (2025-11-15)

### 核心优化
1. **bid_max / ask_min 缓存**: O(100,000) → O(1)
2. **订单簿不变式**: debug_assert 验证 `bid_max <= ask_min`
3. **早期退出**: 匹配前检查价格范围，避免无效遍历

### 性能提升
- 最佳价格查询: ~10,000x 加速
- 无效匹配退出: ~100-100,000x 加速
- 符合 CLAUDE.md Rust < 50ns 标准

### 测试
- 59 个测试 100% 通过
- 新增 15 个测试验证优化
- 执行时间: ~0.05秒

详见: `PERFORMANCE_OPTIMIZATION.md`

## 模块重构 (2025-11-15)

### Repository 完全模块化

**第一阶段**：将 `repository.rs` 重构为目录结构
```
repository.rs  →  repository/
                  ├── mod.rs
                  └── in_memory.rs
```

**第二阶段**：进一步拆分为独立职责模块
```
repository/
├── mod.rs          # 24 行 - 模块入口，重新导出
├── traits.rs       # 90 行 - Trait 定义和错误类型
│                     └── OrderRepository trait
│                     └── RepositoryAccessor trait
│                     └── RepositoryError enum
└── in_memory.rs    # 529 行 - InMemoryOrderRepository 实现 + 测试
```

### 架构优势
- ✅ **单一职责**: 每个文件职责明确
  - `traits.rs`: 接口定义 + 错误类型（领域层）
  - `in_memory.rs`: 具体实现 + 测试（基础设施层）
  - `mod.rs`: 模块导出（清晰的公共 API）
- ✅ **高内聚低耦合**: 接口、错误、实现完全分离
- ✅ **易于扩展**: 可轻松添加新实现（Redis、PostgreSQL）
- ✅ **清晰结构**: 符合 Clean Architecture
- ✅ **独立测试**: 每个实现有独立测试模块
- ✅ **零回归**: 60 个测试 100% 通过（新增 1 个文档测试）