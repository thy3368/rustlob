# 提议者/验证者区块全流程设计文档

**文档版本**: v1.1
**创建日期**: 2026-03-23
**状态**: 设计阶段

---

## 1. 概述

本文档描述基于 `hotstuff_rs` 的 `App` trait，实现提议者（Proposer）打区块、广播区块、验证者（Validator）验证区块，两个角色最终持久化的完整流程。

### 核心决策

| 决策项 | 选择 | 理由 |
|--------|------|------|
| 持久化方式 | `AppStateUpdates` → KVStore | 框架保证原子性，commit 时自动写入 |
| 查询层 | 内存状态（乐观更新）+ KVStore 备份 | 接近零延迟查询，类 Hyperliquid 架构 |
| 内存更新时机 | 乐观（produce/validate 后立即更新） | 最低查询延迟，HotStuff 安全窗口极小 |
| 区块数据格式 | `SpotTransaction`（高层结构） | 包含签名验证所需字段，语义完整 |

---

## 2. 整体流程

```
┌─────────────────────────────────────────────────────────────────┐
│                        提议者 (Proposer)                         │
│                                                                  │
│  mempool → 取 SpotTransaction → 执行撮合（读 AppBlockTreeView）   │
│         → 生成 AppStateUpdates → 序列化 Data → 计算 data_hash    │
│         → 更新内存状态（乐观）                                    │
│         → 返回 ProduceBlockResponse → hotstuff_rs 广播 Proposal  │
└─────────────────────────────────────────────────────────────────┘
                              ↓ 网络广播
┌─────────────────────────────────────────────────────────────────┐
│                        验证者 (Validator)                         │
│                                                                  │
│  收到 Proposal → 反序列化 Data → 验证 data_hash                   │
│               → 重新执行撮合（读 AppBlockTreeView，确定性）        │
│               → 生成 AppStateUpdates → 验证与提议者一致           │
│               → 更新内存状态（乐观）                              │
│               → 返回 Valid → hotstuff_rs 发送 PhaseVote          │
└─────────────────────────────────────────────────────────────────┘
                              ↓ 3轮投票 (Prepare→PreCommit→Commit)
┌─────────────────────────────────────────────────────────────────┐
│                        持久化 (Commit)                            │
│                                                                  │
│  hotstuff_rs 收集 2f+1 PhaseVote → 区块 commit                   │
│  → 自动将 AppStateUpdates 原子写入 KVStore (RocksDB)              │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. 提议者流程（produce_block）

### 3.1 调用时机

hotstuff_rs 在当前节点成为 Leader 时调用 `produce_block`。

### 3.2 实现

```rust
impl App<RocksDB> for SpotApp {
    fn produce_block(&mut self, request: ProduceBlockRequest<RocksDB>) -> ProduceBlockResponse {
        // 1. 从 mempool 取交易（最多 MAX_TXS_PER_BLOCK 笔）
        let transactions = self.mempool.drain_batch(MAX_TXS_PER_BLOCK);

        // 2. 执行撮合，读 AppBlockTreeView（确定性来源）
        let (app_state_updates, executed_txs) =
            self.execute_transactions(&transactions, request.block_tree());

        // 3. 序列化已执行的交易到 Data
        // 序列化失败时降级为空区块，同时丢弃 app_state_updates，保证提议者与验证者状态一致
        let (data, app_state_updates) = if executed_txs.is_empty() {
            (Data::new(vec![Datum::new(vec![])]), app_state_updates)
        } else {
            match borsh::to_vec(&executed_txs) {
                Ok(bytes) => (Data::new(vec![Datum::new(bytes)]), app_state_updates),
                // 序列化失败：降级为空区块 + 空 updates，避免 KVStore 分叉
                Err(_) => (Data::new(vec![Datum::new(vec![])]), AppStateUpdates::new()),
            }
        };

        // 4. 计算 data_hash（SHA-256 over raw Datum bytes）
        let data_hash = compute_data_hash(&data);

        // 5. 乐观更新内存状态（立即可查询）
        self.memory.apply_updates(&app_state_updates);

        ProduceBlockResponse {
            data_hash,
            data,
            app_state_updates: if app_state_updates.is_empty() {
                None
            } else {
                Some(app_state_updates)
            },
            validator_set_updates: None,
        }
    }
}
```

### 3.3 execute_transactions 内部逻辑

**关键约束**：`execute_transactions` 必须只从 `AppBlockTreeView` 读取已提交状态，不得读取 `self.memory`，以保证确定性。

```rust
fn execute_transactions(
    &self,
    transactions: &[SpotTransaction],
    block_tree: &AppBlockTreeView<RocksDB>,
) -> (AppStateUpdates, Vec<SpotTransaction>) {
    let mut updates = AppStateUpdates::new();
    let mut executed = Vec::new();

    for tx in transactions {
        // 签名验证
        if !tx.verify_signature() { continue; }

        // Nonce 去重：从 AppBlockTreeView 读取已提交状态
        let nonce_key = nonce_key(&tx.sender, tx.nonce);
        if block_tree.app_state(&nonce_key).is_some() { continue; }
        // 同时检查本区块内已用 nonce（updates 中）
        if updates.contains_insert(&nonce_key) { continue; }

        // 执行（委托 → 撮合 → 清算），读余额/订单也从 block_tree 读
        if let Ok(tx_updates) = self.execute_one(tx, block_tree, &updates) {
            updates.merge(tx_updates);
            executed.push(tx.clone());
        }
    }

    (updates, executed)
}
```

### 3.4 AppStateUpdates 的 key 设计

所有 key 使用**长度前缀二进制编码**，避免字段含分隔符时的碰撞：

```
key = [namespace_byte] ++ [len_u16_be] ++ field1_bytes ++ [len_u16_be] ++ field2_bytes ...
```

| namespace | 格式 | 示例 |
|-----------|------|------|
| `0x01` balance | `0x01 ++ len(user_id) ++ user_id ++ len(asset) ++ asset` | 余额 |
| `0x02` order | `0x02 ++ len(order_id) ++ order_id` | 订单（deleted = 已成交/撤销） |
| `0x03` nonce | `0x03 ++ len(user_id) ++ user_id ++ nonce_u64_be` | 防重放 |
| `0x04` trade | `0x04 ++ trade_id_u64_be` | 成交记录 |

value 均为 borsh 序列化的对应结构体，deleted 表示从 KVStore 删除该 key。

---

## 4. 验证者流程（validate_block）

### 4.1 调用时机

hotstuff_rs 收到 Proposal 后调用 `validate_block`，验证者需要在 `max_view_time` 内返回。

`validate_block` 可包含最小时长 sleep（用于控制出块速率）；`validate_block_for_sync` 不 sleep，快速完成。

目标：`produce_block` + `validate_block` 各 < 20ms（1000笔交易/区块），sleep 时间计入 `max_view_time` 预算。

### 4.2 实现

```rust
fn validate_block(&mut self, request: ValidateBlockRequest<RocksDB>) -> ValidateBlockResponse {
    // 可在此加最小时长 sleep 控制出块速率
    self.validate_block_inner(request)
}

fn validate_block_for_sync(&mut self, request: ValidateBlockRequest<RocksDB>) -> ValidateBlockResponse {
    // 不 sleep，快速完成
    self.validate_block_inner(request)
}

fn validate_block_inner(
    &mut self,
    request: ValidateBlockRequest<RocksDB>,
) -> ValidateBlockResponse {
    let block = request.proposed_block();

    // 1. 验证 data_hash
    let expected_hash = compute_data_hash(&block.data);
    if block.data_hash != expected_hash {
        return ValidateBlockResponse::Invalid;
    }

    // 2. 反序列化交易（空 Datum = 空区块，合法）
    // 防御性检查：恶意提案可能包含零个 Datum
    let datums = block.data.vec();
    if datums.is_empty() {
        return ValidateBlockResponse::Invalid;
    }
    let raw = datums[0].bytes();
    let transactions: Vec<SpotTransaction> = if raw.is_empty() {
        vec![]
    } else {
        match borsh::from_slice(raw) {
            Ok(txs) => txs,
            Err(_) => return ValidateBlockResponse::Invalid,
        }
    };

    // 3. 重新执行（确定性，与提议者相同逻辑，读 AppBlockTreeView）
    let (app_state_updates, _) =
        self.execute_transactions(&transactions, request.block_tree());

    // 4. 验证 AppStateUpdates 与提议者一致
    //    通过对比 updates 的 borsh 序列化哈希
    //    （hotstuff_rs 本身不传递提议者的 updates，此处通过 data_hash 间接保证：
    //     data_hash 一致 + 确定性执行 → updates 必然一致）
    //    如果执行结果与 data 不一致（如交易数量不符），已在步骤 1 被 data_hash 拦截。

    // 5. 乐观更新内存状态
    self.memory.apply_updates(&app_state_updates);

    ValidateBlockResponse::Valid {
        app_state_updates: if app_state_updates.is_empty() {
            None
        } else {
            Some(app_state_updates)
        },
        validator_set_updates: None,
    }
}
```

**关于 AppStateUpdates 一致性验证**：hotstuff_rs 的安全模型依赖确定性执行。`data_hash` 覆盖了序列化后的交易列表，只要：
1. `data_hash` 验证通过（交易内容一致）
2. `execute_transactions` 是纯确定性函数（只读 `AppBlockTreeView`）

则所有诚实节点的 `app_state_updates` 必然相同，无需额外比对。

---

## 5. 持久化流程（Commit）

### 5.1 hotstuff_rs 自动处理

区块经过 3 轮投票（Prepare → PreCommit → Commit）后，hotstuff_rs 自动将 `AppStateUpdates` 原子写入 KVStore：

```
区块 commit → KVStore.write(app_state_updates)
```

无需应用层干预，框架保证原子性和顺序性。

### 5.2 KVStore 实现

使用 RocksDB 实现 `KVStore` trait：

```rust
pub struct RocksKVStore(Arc<DB>);

impl KVStore for RocksKVStore {
    type WriteBatch = RocksWriteBatch;
    type Snapshot<'a> = RocksSnapshot<'a>;

    fn write(&mut self, wb: Self::WriteBatch) {
        self.0.write(wb.inner).unwrap();
    }

    fn snapshot(&self) -> RocksSnapshot<'_> {
        RocksSnapshot(self.0.snapshot())
    }
}
```

---

## 6. 内存状态与查询

### 6.1 InMemoryState 结构

```rust
pub struct InMemoryState {
    /// 余额表
    pub balances: HashMap<BalanceKey, Balance>,
    /// 活跃订单
    pub orders: HashMap<OrderId, SpotOrder>,
    /// 订单簿（由 orders 派生，不单独持久化）
    pub order_books: HashMap<TradingPair, OrderBook>,
    /// 已用 Nonce（由 KVStore nonce key 派生）
    pub used_nonces: HashSet<(UserId, u64)>,
}
```

**注意**：`order_books` 是 `orders` 的派生视图，不写入 `AppStateUpdates`。恢复时从 `order:*` key 重建。

### 6.2 乐观更新

`produce_block` 和 `validate_block` 执行后立即调用：

```rust
impl InMemoryState {
    pub fn apply_updates(&mut self, updates: &AppStateUpdates) {
        for (key, value) in updates.inserts() {
            self.apply_kv(key, value);
        }
        for key in updates.deletes() {
            self.delete_kv(key);
        }
    }
}
```

### 6.3 乐观更新的 view change 语义

乐观更新存在 1-2 个区块的"超前"窗口。当 view change 导致某个已 validate 的区块被放弃时：

- **内存状态不回滚**：hotstuff_rs 不提供回滚 hook，内存中会保留"幻影"状态
- **安全性不受影响**：KVStore 只写入已 commit 的区块，BFT 安全性由共识层保证
- **查询语义**：内存状态是"乐观/pending"视图，可能包含最终未 commit 的变更
- **实际影响极小**：HotStuff 在正常网络下 view change 极少发生，且幻影状态会在后续区块执行中被覆盖（余额/订单的 KVStore 值最终会追上）

对于需要强一致性的查询（如结算、提现），应读 KVStore 的已提交状态，而非内存状态。

### 6.4 查询路径

```
查询请求
    ↓
内存状态（纳秒级，包含未 commit 区块，乐观视图）
    ↓ 节点重启后内存为空
KVStore（毫秒级，只含已 commit 状态，强一致）
```

### 6.5 节点重启恢复

重启时从 KVStore 重建内存状态，通过前缀扫描所有 namespace `0x01`（balance）、`0x02`（order）、`0x03`（nonce）的 key（二进制格式见 §3.4）：

```rust
impl SpotApp {
    pub fn restore_from_kv(&mut self, kv: &RocksKVStore) {
        // 直接扫描 KVStore，不使用 BlockTreeSnapshot（App 方法外调用）
        self.memory = InMemoryState::from_kv_prefix_scan(kv);
        // order_books 从 orders 重建
        self.memory.rebuild_order_books();
    }
}
```

此方法在 `Replica::start()` 之前调用，不在 `App` trait 方法内，因此可以直接访问 KVStore。

---

## 7. data_hash 计算

提议者和验证者必须使用完全相同的算法：

```rust
fn compute_data_hash(data: &Data) -> CryptoHash {
    let mut hasher = CryptoHasher::new();
    // 对每个 Datum 的原始字节做哈希（与 kv_consensus.rs 示例一致）
    for datum in data.vec() {
        hasher.update(datum.bytes());
    }
    CryptoHash::new(hasher.finalize().into())
}
```

---

## 8. 时序图

```
提议者                    网络                    验证者
   │                       │                       │
   │ produce_block()        │                       │
   │ ─读 AppBlockTreeView──│                       │
   │ ─执行撮合──────────────│                       │
   │ ─生成AppStateUpdates──│                       │
   │ ─更新内存状态（乐观）──│                       │
   │                       │                       │
   │ ──── Proposal ────────>│──── Proposal ────────>│
   │                       │                       │ validate_block()
   │                       │                       │ ─验证data_hash──
   │                       │                       │ ─读 AppBlockTreeView
   │                       │                       │ ─重新执行撮合───
   │                       │                       │ ─更新内存状态───
   │                       │                       │
   │                       │<──── PhaseVote ────────│
   │                       │  (Prepare/PreCommit/Commit x 3轮)
   │                       │                       │
   │ ◄── commit ───────────│                       │
   │ KVStore.write()        │         KVStore.write()│
   │ (hotstuff_rs自动)      │         (hotstuff_rs自动)
```

---

## 9. 关键约束

### 9.1 确定性要求

`execute_transactions` 必须是纯确定性函数：
- **只从 `AppBlockTreeView` 读取状态**，不读 `self.memory`
- 不读系统时间（使用区块 view number 作为时间源）
- 不使用随机数
- 交易排序固定（按 tx 在 Data 中的顺序）
- 浮点运算使用定点数（`Decimal128`）

### 9.2 时序约束

```
4 * EWNL + produce_block_duration + validate_block_duration < max_view_time
```

目标：`produce_block` + `validate_block`（不含 sleep）各 < 20ms（1000笔交易/区块）。

### 9.3 空区块处理

`mempool.drain_batch()` 返回空时，仍产生合法区块：
- `data = Data::new(vec![Datum::new(vec![])])`
- `app_state_updates = None`
- `data_hash` 为空 Datum 的哈希

---

## 10. 文件结构

```
lib/common/spot_consensus/
├── src/
│   ├── lib.rs
│   ├── app.rs              # SpotApp: impl App<RocksKVStore>
│   ├── executor.rs         # execute_transactions（确定性撮合执行，读 AppBlockTreeView）
│   ├── memory.rs           # InMemoryState + 乐观更新 + 重启恢复
│   ├── mempool.rs          # 交易队列
│   ├── kv_store.rs         # RocksDB KVStore 实现
│   └── types.rs            # SpotTransaction, key 编码函数
└── Cargo.toml
```
