# HotStuff BFT Consensus Implementation

纯 Rust 实现的 HotStuff 拜占庭容错共识协议，不依赖第三方共识框架。

## 概述

HotStuff 是一个高效的拜占庭容错（BFT）共识协议，具有以下特点：

- **线性通信复杂度**：每个共识阶段只需 O(n) 次消息传递
- **三阶段共识**：Prepare → Pre-commit → Commit
- **视图切换机制**：支持 Leader 轮换
- **安全性保证**：在 n ≥ 3f + 1 的网络中容忍 f 个拜占庭节点

## 架构设计

本实现遵循 **Clean Architecture** 原则：

```
src/
├── crypto/              # 密码学原语层
│   └── mod.rs          # Hash, Signature, PublicKey, PrivateKey
├── domain/             # 领域层（核心业务逻辑）
│   ├── entities.rs     # 领域实体：Block, Vote, QC
│   ├── consensus.rs    # 共识引擎
│   └── node.rs         # 节点实现
├── tests.rs            # 集成测试
└── lib.rs              # 库入口
```

### 分层说明

1. **Crypto 层**：密码学原语（简化实现，生产环境应使用标准库）
2. **Domain 层**：核心共识逻辑，独立于外部依赖
3. **Tests 层**：集成测试和网络模拟

## 核心概念

### 区块 (Block)

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

### 投票 (Vote)

```rust
pub struct Vote {
    block_hash: Hash,
    view: ViewNumber,
    phase: Phase,
    voter: PublicKey,
    signature: Signature,
}
```

### 仲裁证书 (Quorum Certificate)

QC 是 2f+1 个节点投票的聚合证明，证明区块已通过某个阶段。

```rust
pub struct QuorumCertificate {
    block_hash: Hash,
    view: ViewNumber,
    phase: Phase,
    votes: HashMap<PublicKey, Vote>,
}
```

## 共识流程

### 三阶段协议

```
Leader                     Replicas
  |                           |
  |------ Proposal ---------->|
  |                           |
  |<------ Votes -------------|
  |                           |
  |--- Prepare QC ----------->|  Phase 1: Prepare
  |                           |
  |<------ Votes -------------|
  |                           |
  |-- Pre-commit QC --------->|  Phase 2: Pre-commit (Lock)
  |                           |
  |<------ Votes -------------|
  |                           |
  |--- Commit QC ------------>|  Phase 3: Commit (Decide)
  |                           |
```

### 提交规则

当存在三个连续的 QC 链时，第一个 QC 对应的区块被提交：

```
Block B0 --> Block B1 --> Block B2 --> Block B3
   ^          ^            ^
   |          |            |
Prepare QC  PreCommit QC  Commit QC

→ Block B0 可以提交
```

## 使用示例

### 基本用法

```rust
use hotstuff::{Node, crypto::PrivateKey};

// 创建验证者集合
let validators: Vec<_> = (0..4)
    .map(|i| PrivateKey::from_u64(i).public_key())
    .collect();

// 创建节点
let private_key = PrivateKey::from_u64(0);
let node = Node::new(0, private_key, validators, true);
```

### 运行示例

```bash
# 运行基本共识演示
cargo run --example basic_consensus

# 运行测试
cargo test

# 运行详细测试
cargo test -- --nocapture
```

## 测试

### 单元测试

```bash
# 测试密码学模块
cargo test crypto::tests

# 测试实体模块
cargo test domain::entities::tests

# 测试共识逻辑
cargo test domain::consensus::tests

# 测试节点
cargo test domain::node::tests
```

### 集成测试

```bash
# 单区块共识测试
cargo test test_single_block_consensus -- --nocapture

# 多区块共识测试
cargo test test_multi_block_consensus -- --nocapture

# 视图切换测试
cargo test test_view_change -- --nocapture
```

## 性能特性

根据 CLAUDE.md 中的低延迟开发标准，本实现：

### 内存优化
- ✅ 缓存行对齐的数据结构（关键路径）
- ✅ 最小化内存分配（使用 `Vec` 预分配）
- ✅ 零拷贝设计（使用引用传递）

### 编译优化

```toml
[profile.release]
opt-level = 3           # 最高优化级别
lto = "fat"            # 链接时优化
codegen-units = 1      # 单个代码生成单元
```

### 无锁设计
- 使用纯函数式数据结构
- 避免共享可变状态
- 消息传递模式（未来可扩展为异步）

## 限制和改进方向

### 当前限制

1. **简化的密码学**：使用模拟的签名和哈希（生产环境需真实实现）
2. **同步网络模型**：未实现真实的网络通信层
3. **无持久化**：区块仅存储在内存中
4. **简化的 Leader 选举**：使用 round-robin 方式

### 未来改进

- [ ] 集成真实的密码学库（ed25519-dalek, sha2）
- [ ] 实现网络通信层（gRPC 或自定义协议）
- [ ] 添加持久化存储
- [ ] 实现更复杂的 Leader 选举算法
- [ ] 支持动态验证者集合
- [ ] 实现视图同步优化
- [ ] 添加性能基准测试
- [ ] 支持并发处理

## 参考文献

- [HotStuff: BFT Consensus in the Lens of Blockchain](https://arxiv.org/abs/1803.05069)
- [HotStuff 论文解读](https://decert.me/tutorial/hotstuff)
- [LibraBFT: HotStuff 的实际应用](https://developers.diem.com/papers/diem-consensus-state-machine-replication-in-the-diem-blockchain/2019-06-28.pdf)

## 许可

本项目采用与 workspace 相同的许可证。

## 贡献

欢迎提交 Issue 和 Pull Request！
