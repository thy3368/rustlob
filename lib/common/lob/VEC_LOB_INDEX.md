# Vec订单簿 RTO=0 RPO=0 文档索引

**完整文档导航和使用指南**

## 📚 文档系列概览

本系列共包含4份文档，循序渐进地解释Vec订单簿的快照和回放原理，以及如何实现RTO=0 RPO=0。

### 文档关系图

```
┌─────────────────────────────────────────────┐
│  快速参考指南 (QUICK_REFERENCE)             │
│  └─ 一页纸速查，日常开发必备                  │
└────────────────┬────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────┐
│  核心原理文档 (SNAPSHOT_AND_REPLAY)         │
│  └─ 详细讲解快照和回放的原理、优化、故障处理  │
└────────────────┬────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────┐
│  实现指南 (IMPLEMENTATION_GUIDE)            │
│  └─ 完整的代码示例和实现细节                  │
└────────────────┬────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────┐
│  本文档 (INDEX)                            │
│  └─ 导航和问题速查                          │
└─────────────────────────────────────────────┘
```

## 🎯 按场景快速选择

### 场景1: 我是新手，第一次接触这个系统

**推荐路径**:
1. 阅读 [快速参考指南](#快速参考指南) - 5分钟了解核心概念
2. 浏览 [核心原理文档](#核心原理文档) 的概述部分
3. 查看 [实现指南](#实现指南) 中的系统架构章节

### 场景2: 我要实现快照和恢复功能

**推荐路径**:
1. 先读 [核心原理文档](#核心原理文档) 的**数据结构**部分
2. 再看 [实现指南](#实现指南) 的**持久化层实现**章节
3. 参考源代码: `local_lob_impl.rs:78-100` (RepoSnapshot实现)

### 场景3: 系统故障了，我需要快速恢复

**推荐路径**:
1. 查看 [快速参考指南](#快速参考指南) 的**故障恢复决策树**
2. 按 [核心原理文档](#核心原理文档) 的**恢复流程**执行
3. 参考 [实现指南](#实现指南) 的**恢复流程实现**了解代码

### 场景4: 我要优化性能

**推荐路径**:
1. 查看 [核心原理文档](#核心原理文档) 的**性能优化**章节
2. 参考 [快速参考指南](#快速参考指南) 的**性能基准速查**
3. 实现 [实现指南](#实现指南) 中的并行化和批处理

### 场景5: 我要设置监控告警

**推荐路径**:
1. 看 [实现指南](#实现指南) 的**监控和告警**部分
2. 参考 [快速参考指南](#快速参考指南) 的**监控指标速查**
3. 实现 Prometheus 指标收集

## 📖 文档详情

### 快速参考指南
**文件**: `VEC_LOB_QUICK_REFERENCE.md`
**适合**: 日常开发、临时查询
**内容**:
- ✓ 核心概念速览 (1分钟)
- ✓ 文件位置速查
- ✓ 日常操作代码片段
- ✓ 数据结构简表
- ✓ 性能基准
- ✓ 常用模式
- ✓ 故障决策树
- ✓ 错误处理

**何时查看**:
- 需要快速查找某个方法
- 想看代码示例
- 遇到问题需要决策

### 核心原理文档
**文件**: `VEC_LOB_SNAPSHOT_AND_REPLAY.md`
**适合**: 深入学习、架构设计
**内容**:
- ✓ 概述: RTO/RPO 定义
- ✓ 数据结构详解 (5分钟)
- ✓ 快照机制原理 (10分钟)
- ✓ 回放机制原理 (10分钟)
- ✓ RTO=0 RPO=0 实现方案 (10分钟)
- ✓ 完整恢复流程 (15分钟)
- ✓ 故障场景分析
- ✓ 性能优化方案

**何时查看**:
- 第一次学习系统
- 需要理解设计原理
- 要做架构决策
- 遇到复杂问题需要原理支撑

### 实现指南
**文件**: `VEC_LOB_IMPLEMENTATION_GUIDE.md`
**适合**: 实现开发、代码阅读
**内容**:
- ✓ 系统架构详解
- ✓ WAL Writer 完整实现 (100+ 行代码)
- ✓ Snapshot Manager 完整实现 (150+ 行代码)
- ✓ Recovery Manager 完整实现 (150+ 行代码)
- ✓ 应用启动恢复流程
- ✓ 监控和告警实现
- ✓ 单元测试示例
- ✓ 集成测试示例
- ✓ 压力测试示例
- ✓ 常见问题解答

**何时查看**:
- 要实现新功能
- 阅读源代码时需要参考
- 要写测试
- 需要调试

## 🔍 按主题查询

### 主题1: 理解快照机制

**问题**: 快照是什么？为什么需要快照？

**查询路径**:
1. [快速参考](VEC_LOB_QUICK_REFERENCE.md) → 核心概念速览
2. [核心原理](VEC_LOB_SNAPSHOT_AND_REPLAY.md) → 快照机制
3. [源代码](./src/adapter/local_lob_impl.rs) → RepoSnapshot impl (L78-100)

**核心概念**: 快照是整个LOB状态在某个时刻的完整副本，用于快速恢复。

---

### 主题2: 理解事件回放机制

**问题**: 如何从事件重建状态？

**查询路径**:
1. [快速参考](VEC_LOB_QUICK_REFERENCE.md) → 回放事件操作
2. [核心原理](VEC_LOB_SNAPSHOT_AND_REPLAY.md) → 回放机制
3. [源代码](./src/adapter/local_lob_impl.rs) → EventReplay impl (L102-166)

**核心概念**: 事件回放通过顺序应用变更事件来重建状态，支持增量恢复。

---

### 主题3: 实现快照功能

**问题**: 我要在我的系统中实现快照，怎么做？

**查询路径**:
1. [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) → Snapshot Manager 实现
2. [源代码](./src/adapter/local_lob_impl.rs) → RepoSnapshot impl
3. [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) → 测试策略

**步骤**:
- 实现 RepoSnapshot trait
- 创建 SnapshotManager 结构
- 添加序列化/反序列化逻辑
- 编写快照测试

---

### 主题4: 故障恢复流程

**问题**: 系统故障后怎样快速恢复？

**查询路径**:
1. [快速参考](VEC_LOB_QUICK_REFERENCE.md) → 故障恢复决策树
2. [核心原理](VEC_LOB_SNAPSHOT_AND_REPLAY.md) → 恢复流程
3. [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) → Recovery Manager 实现

**恢复步骤**:
1. 加载快照 (~100ms)
2. 回放增量事件 (~1s)
3. 验证一致性 (~50ms)

---

### 主题5: 性能优化

**问题**: 如何让快照和恢复更快？

**查询路径**:
1. [核心原理](VEC_LOB_SNAPSHOT_AND_REPLAY.md) → 性能优化章节
2. [快速参考](VEC_LOB_QUICK_REFERENCE.md) → 性能基准速查
3. [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) → 性能优化代码

**优化技巧**:
- 增量快照
- 并行快照
- 压缩存储
- 事件批处理
- 事件去重

---

### 主题6: 监控和告警

**问题**: 怎样监控RTO和RPO？

**查询路径**:
1. [快速参考](VEC_LOB_QUICK_REFERENCE.md) → 监控指标速查
2. [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) → 监控和告警

**关键指标**:
- RTO (恢复时间)
- RPO (数据丢失)
- 快照大小
- 事件缓冲区大小

---

### 主题7: 测试策略

**问题**: 怎样测试快照和恢复功能？

**查询路径**:
1. [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) → 测试策略章节
2. 查看单元测试、集成测试、压力测试代码

**测试覆盖**:
- 单元测试: 快照创建/恢复
- 集成测试: 完整恢复流程
- 压力测试: 性能基准

---

## ❓ 常见问题速查

### Q1: 快照多久创建一次最好？

**A**: 查看 [核心原理 → 快照时间点选择](VEC_LOB_SNAPSHOT_AND_REPLAY.md#快照的时间点选择)

**简答**: 每5秒或1000个事件，取决于系统的RTO/RPO目标。

---

### Q2: 系统故障时数据会丢失吗？

**A**: 查看 [核心原理 → RTO=0 RPO=0](VEC_LOB_SNAPSHOT_AND_REPLAY.md#rto0-rpo0-实现方案)

**简答**: 不会，所有事件都被WAL持久化，RPO=0。

---

### Q3: 恢复需要多长时间？

**A**: 查看 [快速参考 → 性能基准](VEC_LOB_QUICK_REFERENCE.md#性能基准速查)

**简答**: 1-2秒（快照100ms + 增量回放<1s）。

---

### Q4: 如何处理快照文件损坏？

**A**: 查看 [核心原理 → 故障场景分析 → 场景2](VEC_LOB_SNAPSHOT_AND_REPLAY.md#场景2-快照损坏)

**简答**: 从WAL全量恢复（5-10秒，但RPO=0）。

---

### Q5: 需要备份吗？

**A**: 查看 [核心原理 → 故障场景分析](VEC_LOB_SNAPSHOT_AND_REPLAY.md#故障场景分析)

**简答**: 建议双写快照（本地+远程）和定期备份。

---

### Q6: 单机还是分布式部署？

**A**: 查看 [核心原理 → 整体架构](VEC_LOB_SNAPSHOT_AND_REPLAY.md#整体架构)

**简答**:
- 单机: 使用本地快照 + WAL
- 分布式: 快照复制到多副本 + 分布式一致性

---

### Q7: 怎样实现RTO<1秒？

**A**: 查看 [核心原理 → 性能优化](VEC_LOB_SNAPSHOT_AND_REPLAY.md#性能优化)

**简答**: 使用增量快照、压缩、并行化和事件批处理。

---

### Q8: 可以跳过某些事件吗？

**A**: **否**。查看 [快速参考 → 序列号管理](VEC_LOB_QUICK_REFERENCE.md#序列号管理速查)

**简答**: 事件序列号必须连续，不能跳过。

---

## 📌 核心指标一览表

| 指标 | 目标值 | 当前值 | 查询位置 |
|-----|------|------|--------|
| RTO | <2s | ~1-2s | [基准](VEC_LOB_QUICK_REFERENCE.md#性能基准速查) |
| RPO | 0 | 0 | [保证](VEC_LOB_SNAPSHOT_AND_REPLAY.md#rto0-rpo0-实现方案) |
| 快照创建 | <100ms | 50-100ms | [基准](VEC_LOB_SNAPSHOT_AND_REPLAY.md#快照性能优化) |
| 事件回放 | <100ms per 1000 | 50-100ms | [基准](VEC_LOB_SNAPSHOT_AND_REPLAY.md#事件回放性能优化) |
| 快照大小 | <50MB | 20-40MB | [优化](VEC_LOB_SNAPSHOT_AND_REPLAY.md#快照压缩) |

## 🔗 文件导航

```
lib/common/lob/
├─ VEC_LOB_QUICK_REFERENCE.md           ← 快速参考（推荐首先阅读）
├─ VEC_LOB_SNAPSHOT_AND_REPLAY.md       ← 详细原理
├─ VEC_LOB_IMPLEMENTATION_GUIDE.md      ← 实现代码
├─ VEC_LOB_INDEX.md                     ← 本文档
│
├─ src/
│  ├─ adapter/
│  │  └─ local_lob_impl.rs              ← LocalLob实现
│  └─ core/
│     └─ repo_snapshot_support.rs       ← Trait定义
│
├─ tests/
│  ├─ local_lob_tests.rs                ← 单元测试
│  ├─ snapshot_tests.rs                 ← 快照测试
│  └─ recovery_tests.rs                 ← 恢复测试
```

## 🚀 快速开始清单

- [ ] 阅读 [快速参考指南](VEC_LOB_QUICK_REFERENCE.md) (5min)
- [ ] 理解 [核心原理](VEC_LOB_SNAPSHOT_AND_REPLAY.md#核心数据结构) (15min)
- [ ] 查看 [源代码实现](./src/adapter/local_lob_impl.rs) (20min)
- [ ] 学习 [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) (30min)
- [ ] 运行测试验证理解 (10min)

**总计**: ~80分钟掌握完整系统

## 📞 获取帮助

| 问题类型 | 推荐文档 |
|---------|--------|
| 快速查询 | [快速参考](VEC_LOB_QUICK_REFERENCE.md) |
| 理解原理 | [核心原理](VEC_LOB_SNAPSHOT_AND_REPLAY.md) |
| 代码实现 | [实现指南](VEC_LOB_IMPLEMENTATION_GUIDE.md) |
| 源代码 | [local_lob_impl.rs](./src/adapter/local_lob_impl.rs) |
| 接口定义 | [repo_snapshot_support.rs](./src/core/repo_snapshot_support.rs) |
| 测试示例 | [实现指南 → 测试策略](VEC_LOB_IMPLEMENTATION_GUIDE.md#测试策略) |

---

**索引版本**: v1.0
**最后更新**: 2025-12-18
**文档集**: Vec订单簿 RTO=0 RPO=0 完整文档库
