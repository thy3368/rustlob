# HotStuff BFT 共识算法设计文档

## 项目概述

本项目使用纯 Rust 实现了 HotStuff 拜占庭容错共识算法，不依赖第三方共识框架，仅使用 Rust 标准库。

## 设计目标

1. **纯 Rust 实现**：不依赖外部共识框架
2. **Clean Architecture**：遵循整洁架构原则，领域逻辑独立
3. **低延迟**：遵循 CLAUDE.md 中的性能标准
4. **可测试性**：完整的单元测试和集成测试

## 架构设计

### 分层结构

```
┌─────────────────────────────────────┐
│        应用层 (Examples)            │
│    basic_consensus.rs               │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│        节点层 (Node)                │
│    消息处理、角色管理               │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│      共识层 (Consensus)             │
│    三阶段协议、QC 形成              │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│      实体层 (Entities)              │
│    Block, Vote, QC                  │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│     密码层 (Crypto)                 │
│    Hash, Signature (简化)           │
└─────────────────────────────────────┘
```

## HotStuff 协议流程

### 三阶段共识

```
┌──────────┐          ┌──────────┐
│  Leader  │          │ Replicas │
└────┬─────┘          └────┬─────┘
     │                     │
     │  1. Proposal        │
     │─────────────────────>
     │                     │
     │  2. Prepare Votes   │
     <─────────────────────┤
     │                     │
     │  3. Prepare QC      │
     │─────────────────────>
     │                     │
     │  4. Pre-commit Votes│
     <─────────────────────┤
     │                     │
     │  5. Pre-commit QC   │
     │─────────────────────>
     │                     │
     │  6. Commit Votes    │
     <─────────────────────┤
     │                     │
     │  7. Commit QC       │
     │─────────────────────>
     │                     │
     ▼                     ▼
```

### 提交规则

当存在三个连续的 QC 链时提交：

```
Block B0 --> Block B1 --> Block B2 --> Block B3
   ^          ^            ^
   |          |            |
Prepare QC  PreCommit QC  Commit QC

→ Block B0 被提交
```

## 核心数据结构

### Block (区块)

```rust
pub struct Block {
    hash: Hash,
    parent_hash: Hash,
    view: ViewNumber,
    height: Height,
    proposer: PublicKey,
    justify: QuorumCertificate,
    commands: Vec<Vec<u8>>,
}
```

### Vote (投票)

```rust
pub struct Vote {
    block_hash: Hash,
    view: ViewNumber,
    phase: Phase,
    voter: PublicKey,
    signature: Signature,
}
```

### QuorumCertificate (QC)

```rust
pub struct QuorumCertificate {
    block_hash: Hash,
    view: ViewNumber,
    phase: Phase,
    votes: HashMap<PublicKey, Vote>,
}
```

## 安全性保证

1. **投票规则**：节点只对满足以下条件的区块投票
   - 视图匹配
   - 父区块存在
   - QC 有效（除创世 QC）
   - 不与 locked_qc 冲突

2. **锁定机制**：Pre-commit QC 形成时锁定，防止分叉

3. **提交规则**：三个连续 QC 链确保最终性

## 性能优化

### 内存管理
- 使用 `HashMap` 高效存储区块和投票
- 缓存行对齐（关键数据结构）
- 最小化内存分配

### 编译优化
- LTO（链接时优化）
- 目标 CPU 优化
- 单个代码生成单元

## 测试覆盖

### 单元测试
- ✅ 密码学原语
- ✅ 实体创建和验证
- ✅ 共识逻辑
- ✅ 节点行为

### 集成测试
- ✅ 单区块共识
- ✅ 多区块共识
- ✅ 视图切换
- ✅ Leader 轮换

## 运行示例

```bash
# 运行基本共识演示
cargo run --example basic_consensus

# 运行所有测试
cargo test

# 运行详细测试
cargo test -- --nocapture
```

## 限制和未来改进

### 当前限制
1. 简化的密码学实现（生产环境需真实加密库）
2. 同步网络模型
3. 无持久化存储
4. 简化的 Leader 选举

### 未来改进
- [ ] 集成真实密码学库（ed25519-dalek, sha2）
- [ ] 实现网络通信层
- [ ] 添加持久化存储
- [ ] 实现 BLS 门限签名
- [ ] 支持动态验证者集合
- [ ] 性能基准测试

## 参考文献

- [HotStuff: BFT Consensus in the Lens of Blockchain](https://arxiv.org/abs/1803.05069)
- [LibraBFT 技术报告](https://developers.diem.com/papers/diem-consensus-state-machine-replication-in-the-diem-blockchain/2019-06-28.pdf)

---

**实现状态**: ✅ 完成
**测试状态**: ✅ 所有测试通过
**最后更新**: 2025-01-23
