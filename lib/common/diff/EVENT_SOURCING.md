# Rust之从0-1低时延CEX：Event Sourcing的基石 Diff/Replay 原理

## 📚 目录

- [Event Sourcing 核心思想](#event-sourcing-核心思想)
- [Diff/Replay 原理](#diffreplay-原理)
- [快速上手](#快速上手)
- [核心概念](#核心概念)
- [实战示例](#实战示例)
- [最佳实践](#最佳实践)
- [常见问题](#常见问题)

---

## Event Sourcing 核心思想

### 传统 CRUD vs Event Sourcing

#### 传统 CRUD 模式
```
订单状态: Pending → Processing → Completed
         ↓           ↓            ↓
数据库:  UPDATE    UPDATE       UPDATE
结果:    只保存最终状态 (Completed)
问题:    ❌ 无法知道何时变更
        ❌ 无法知道谁变更的
        ❌ 无法回滚到历史状态
        ❌ 无法审计变更历史
```

#### Event Sourcing 模式
```
订单状态: Pending → Processing → Completed
         ↓           ↓            ↓
事件流:  Event1     Event2       Event3
        (已创建)    (已支付)     (已完成)

结果: 保存所有事件，当前状态 = 重放所有事件
优势: ✅ 完整的历史记录
     ✅ 可审计
     ✅ 可回放
     ✅ 可重建任意时刻的状态
```

### Event Sourcing 三大支柱

1. **事件 (Events)** - 记录"发生了什么"
2. **聚合 (Aggregates)** - 通过重放事件重建状态
3. **投影 (Projections)** - 从事件流生成查询视图

---

## Diff/Replay 原理

### 核心思想

**Diff/Replay 是 Event Sourcing 的简化实现**，专注于字段级变更追踪：

```
┌─────────────────────────────────────────────────────────┐
│                  Event Sourcing                          │
│                                                          │
│  State₀ + Event₁ + Event₂ + ... + Eventₙ = Stateₙ       │
│                                                          │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                    Diff/Replay                           │
│                                                          │
│  State₀ + Diff₁ + Diff₂ + ... + Diffₙ = Stateₙ          │
│                                                          │
│  Diff = { field_name, old_value, new_value }            │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 工作流程

```
┌─────────────┐
│  初始状态    │
│  State₀     │
└──────┬──────┘
       │
       ↓
┌─────────────┐      ┌──────────────┐
│  执行变更    │ ──→  │  Diff Derive │ ──→ 生成变更日志
│  State₀→₁   │      │  自动检测差异 │      (ChangeLogEntry)
└─────────────┘      └──────────────┘
       │
       ↓
┌─────────────┐
│  持久化日志  │ ──→ 数据库/文件/消息队列
└─────────────┘
       │
       ↓
┌─────────────┐      ┌──────────────┐
│  数据回放    │ ──→  │ Replay Derive│ ──→ 重建状态
│  State₀→ₙ   │      │  应用变更日志 │      (任意时刻)
└─────────────┘      └──────────────┘
```

---

## 快速上手

### 1. 定义实体

```rust
use diff_tracker::{Diff, Replay};

/// 订单实体 - 使用 Diff 和 Replay derive 宏
#[derive(Debug, Clone, Diff, Replay)]
struct Order {
    id: String,
    amount: i64,
    status: String,
    is_paid: bool,
}
```

### 2. 录制变更

```rust
use diff_tracker::track_auto;

let mut order = Order {
    id: "ORD-001".to_string(),
    amount: 1000,
    status: "Pending".to_string(),
    is_paid: false,
};

// 🎬 录制变更
let change_log = track_auto(&mut order, |o| {
    o.status = "Completed".to_string();
    o.is_paid = true;
}).unwrap();

// change_log 包含:
// - amount: 1000 → 1000 (未变更，不记录)
// - status: "Pending" → "Completed"
// - is_paid: false → true
```

### 3. 回放变更

```rust
// 从初始状态开始
let mut order_replay = Order {
    id: "ORD-001".to_string(),
    amount: 1000,
    status: "Pending".to_string(),
    is_paid: false,
};

// 🎥 回放变更
order_replay.replay(&change_log).unwrap();

// 结果: order_replay 状态与 order 一致
assert_eq!(order_replay.status, "Completed");
assert_eq!(order_replay.is_paid, true);
```

---

## 核心概念

### 1. FieldChange - 字段变更

```rust
pub struct FieldChange {
    pub field_name: String,   // 字段名
    pub old_value: String,    // 旧值
    pub new_value: String,    // 新值
}
```

**示例**:
```rust
FieldChange {
    field_name: "status",
    old_value: "Pending",
    new_value: "Completed",
}
```

### 2. ChangeType - 变更类型

```rust
pub enum ChangeType {
    Created,                              // 创建
    Updated { changed_fields: Vec<FieldChange> },  // 更新
    Deleted,                              // 删除
}
```

### 3. ChangeLogEntry - 变更日志条目

```rust
pub struct ChangeLogEntry {
    pub entity_id: String,      // 实体ID
    pub entity_type: String,    // 实体类型
    pub change_type: ChangeType, // 变更类型
    pub timestamp: u64,         // 时间戳
}
```

### 4. Diff Trait - 差异检测

```rust
pub trait Diff {
    /// 比较 self(旧) 和 other(新)，返回字段变更列表
    fn diff(&self, other: &Self) -> Vec<FieldChange>;
}
```

**自动实现**:
```rust
#[derive(Diff)]
struct User {
    name: String,
    age: i32,
}

// 自动生成 diff() 方法
```

### 5. Replay Trait - 数据回放

```rust
pub trait Replay {
    /// 从变更日志回放数据，更新 self 的字段
    fn replay(&mut self, entry: &ChangeLogEntry) -> Result<(), String>;
}
```

**自动实现**:
```rust
#[derive(Replay)]
struct User {
    name: String,
    age: i32,
}

// 自动生成 replay() 方法
```

---

## 实战示例

### 示例 1: 电商订单状态机

```rust
use diff_tracker::{Diff, Replay, track_auto};

#[derive(Debug, Clone, Diff, Replay)]
struct Order {
    id: String,
    amount: i64,
    status: String,
    payment_method: String,
    is_paid: bool,
}

impl Order {
    fn new(id: String, amount: i64) -> Self {
        Self {
            id,
            amount,
            status: "Created".to_string(),
            payment_method: "".to_string(),
            is_paid: false,
        }
    }

    // 业务方法：支付订单
    fn pay(&mut self, method: String) {
        self.status = "Paid".to_string();
        self.payment_method = method;
        self.is_paid = true;
    }

    // 业务方法：发货
    fn ship(&mut self) {
        self.status = "Shipped".to_string();
    }

    // 业务方法：完成
    fn complete(&mut self) {
        self.status = "Completed".to_string();
    }
}

fn main() {
    let mut order = Order::new("ORD-001".to_string(), 5000);
    let mut change_logs = Vec::new();

    // 📝 步骤 1: 支付
    let log1 = track_auto(&mut order, |o| {
        o.pay("Alipay".to_string());
    }).unwrap();
    change_logs.push(log1);

    // 📝 步骤 2: 发货
    let log2 = track_auto(&mut order, |o| {
        o.ship();
    }).unwrap();
    change_logs.push(log2);

    // 📝 步骤 3: 完成
    let log3 = track_auto(&mut order, |o| {
        o.complete();
    }).unwrap();
    change_logs.push(log3);

    // 🎥 回放: 从头重建订单状态
    let mut order_replay = Order::new("ORD-001".to_string(), 5000);
    for log in &change_logs {
        order_replay.replay(log).unwrap();
    }

    assert_eq!(order_replay.status, "Completed");
    assert_eq!(order_replay.is_paid, true);
    assert_eq!(order_replay.payment_method, "Alipay");
}
```

**输出的变更日志**:
```
Log 1:
  - status: "Created" → "Paid"
  - payment_method: "" → "Alipay"
  - is_paid: false → true

Log 2:
  - status: "Paid" → "Shipped"

Log 3:
  - status: "Shipped" → "Completed"
```

### 示例 2: 银行账户余额变更

```rust
use diff_tracker::{Diff, Replay, track_auto};

#[derive(Debug, Clone, Diff, Replay)]
struct BankAccount {
    account_id: String,
    balance: i64,
    last_transaction: String,
}

impl BankAccount {
    fn new(account_id: String) -> Self {
        Self {
            account_id,
            balance: 0,
            last_transaction: "Initial".to_string(),
        }
    }

    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
        self.last_transaction = format!("Deposit: {}", amount);
    }

    fn withdraw(&mut self, amount: i64) -> Result<(), String> {
        if self.balance < amount {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        self.last_transaction = format!("Withdraw: {}", amount);
        Ok(())
    }
}

fn main() {
    let mut account = BankAccount::new("ACC-001".to_string());
    let mut change_logs = Vec::new();

    // 存款 1000
    let log1 = track_auto(&mut account, |a| {
        a.deposit(1000);
    }).unwrap();
    change_logs.push(log1);

    // 取款 300
    let log2 = track_auto(&mut account, |a| {
        a.withdraw(300).unwrap();
    }).unwrap();
    change_logs.push(log2);

    // 存款 500
    let log3 = track_auto(&mut account, |a| {
        a.deposit(500);
    }).unwrap();
    change_logs.push(log3);

    println!("最终余额: {}", account.balance); // 1200

    // 🎥 回放: 审计账户历史
    let mut account_replay = BankAccount::new("ACC-001".to_string());
    for (i, log) in change_logs.iter().enumerate() {
        account_replay.replay(log).unwrap();
        println!("步骤 {}: 余额 = {}", i + 1, account_replay.balance);
    }
}
```

**输出**:
```
步骤 1: 余额 = 1000
步骤 2: 余额 = 700
步骤 3: 余额 = 1200
```

### 示例 3: 时间旅行调试

```rust
use diff_tracker::{Diff, Replay, track_auto};

#[derive(Debug, Clone, Diff, Replay)]
struct GameState {
    player_hp: i32,
    player_mp: i32,
    level: u32,
    position_x: i32,
    position_y: i32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_hp: 100,
            player_mp: 50,
            level: 1,
            position_x: 0,
            position_y: 0,
        }
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.position_x = x;
        self.position_y = y;
    }

    fn take_damage(&mut self, damage: i32) {
        self.player_hp -= damage;
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.player_hp = 100;
        self.player_mp = 50;
    }
}

fn main() {
    let mut game = GameState::new();
    let mut snapshots = Vec::new();

    // 🎬 记录游戏进度
    snapshots.push((game.clone(), track_auto(&mut game, |g| {
        g.move_to(10, 20);
    }).unwrap()));

    snapshots.push((game.clone(), track_auto(&mut game, |g| {
        g.take_damage(30);
    }).unwrap()));

    snapshots.push((game.clone(), track_auto(&mut game, |g| {
        g.level_up();
    }).unwrap()));

    // 🔙 时间旅行: 回到第 2 步
    println!("回到第 2 步之前的状态:");
    let (snapshot, _) = &snapshots[1];
    println!("HP: {}, Level: {}", snapshot.player_hp, snapshot.level);

    // 🎥 从某个快照重放后续操作
    let mut replay_game = snapshots[1].0.clone();
    replay_game.replay(&snapshots[2].1).unwrap();
    println!("重放后: HP: {}, Level: {}", replay_game.player_hp, replay_game.level);
}
```

### 示例 4: 数据脱敏

```rust
use diff_tracker::{Diff, Replay};

#[derive(Debug, Clone, Diff, Replay)]
struct User {
    id: String,
    username: String,
    email: String,
    #[diff(mask)]  // 🔒 脱敏字段
    password: String,
}

fn main() {
    let user1 = User {
        id: "user_001".to_string(),
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        password: "secret123".to_string(),
    };

    let user2 = User {
        id: "user_001".to_string(),
        username: "alice_updated".to_string(),
        email: "alice@newdomain.com".to_string(),
        password: "newsecret456".to_string(),
    };

    let changes = user1.diff(&user2);

    for change in changes {
        println!("{}: {} → {}",
            change.field_name,
            change.old_value,
            change.new_value);
    }
}
```

**输出**:
```
username: alice → alice_updated
email: alice@example.com → alice@newdomain.com
password: *** → ***  ← 密码被脱敏
```

### 示例 5: 跳过不重要的字段

```rust
#[derive(Debug, Clone, Diff, Replay)]
struct CachedEntity {
    id: String,
    data: String,
    #[diff(skip)]  // ⏭️ 跳过缓存字段
    cache: Option<String>,
}

fn main() {
    let entity1 = CachedEntity {
        id: "E001".to_string(),
        data: "Important".to_string(),
        cache: Some("cached_value".to_string()),
    };

    let entity2 = CachedEntity {
        id: "E001".to_string(),
        data: "Updated".to_string(),
        cache: Some("new_cached_value".to_string()),  // cache 变化不会被记录
    };

    let changes = entity1.diff(&entity2);

    // 只记录了 data 的变更，cache 被跳过
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].field_name, "data");
}
```

---

## 最佳实践

### 1. 何时使用 Diff/Replay

✅ **适用场景**:
- 需要审计日志的业务系统
- 需要撤销/重做功能
- 需要时间旅行调试
- 需要数据回放验证
- 需要变更历史追踪

❌ **不适用场景**:
- 高频写入场景（可能产生大量日志）
- 不需要历史记录的临时数据
- 性能极度敏感的场景

### 2. 设计原则

#### 原则 1: 实体不可变性
```rust
// ✅ 好的设计
#[derive(Clone, Diff, Replay)]
struct Order {
    id: String,  // 不可变ID
    amount: i64,
    status: String,
}

// ❌ 避免
struct Order {
    // ID 不应该变更
}
```

#### 原则 2: 业务方法封装
```rust
impl Order {
    // ✅ 封装业务逻辑
    pub fn pay(&mut self, method: String) {
        self.status = "Paid".to_string();
        self.payment_method = method;
        self.is_paid = true;
    }

    // ❌ 避免直接暴露字段修改
    // pub status: String
}
```

#### 原则 3: 快照 + 变更日志
```rust
// ✅ 保存快照以便回放
let snapshot = entity.clone();
let change_log = track_auto(&mut entity, |e| {
    e.update();
}).unwrap();

// 可以从快照回放
let mut replay = snapshot;
replay.replay(&change_log).unwrap();
```

### 3. 持久化策略

#### 策略 1: 追加式存储（Append-Only）
```rust
// 变更日志表结构
CREATE TABLE change_logs (
    id BIGSERIAL PRIMARY KEY,
    entity_id VARCHAR(50),
    entity_type VARCHAR(50),
    change_type VARCHAR(20),
    changed_fields JSONB,
    timestamp BIGINT,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_entity ON change_logs(entity_id, entity_type);
CREATE INDEX idx_timestamp ON change_logs(timestamp);
```

#### 策略 2: 快照 + 增量
```rust
// 每 N 个变更保存一次快照
if change_count % 100 == 0 {
    save_snapshot(&entity);
}
save_change_log(&change_log);
```

#### 策略 3: 事件溯源架构
```
┌─────────────┐
│  命令层      │  ← 执行业务命令
└──────┬──────┘
       ↓
┌─────────────┐
│  事件存储    │  ← 保存变更日志
└──────┬──────┘
       ↓
┌─────────────┐
│  投影层      │  ← 构建查询视图
└─────────────┘
```

### 4. 性能优化

#### 优化 1: 批量回放
```rust
// ❌ 慢：逐个回放
for log in logs {
    entity.replay(&log).unwrap();
}

// ✅ 快：批量回放
let batch_log = merge_logs(&logs);
entity.replay(&batch_log).unwrap();
```

#### 优化 2: 增量快照
```rust
// 定期保存快照，减少回放次数
let snapshot = load_latest_snapshot(&entity_id);
let logs = load_logs_since(&snapshot.timestamp);
let mut entity = snapshot.state;
for log in logs {
    entity.replay(&log).unwrap();
}
```

#### 优化 3: 异步持久化
```rust
// 异步保存日志，不阻塞主流程
tokio::spawn(async move {
    save_change_log_async(&change_log).await;
});
```

### 5. 错误处理

```rust
// ✅ 完善的错误处理
match order.replay(&change_log) {
    Ok(_) => println!("回放成功"),
    Err(e) if e.contains("Failed to parse") => {
        // 数据格式错误，可能需要数据迁移
        log::error!("数据格式不兼容: {}", e);
    }
    Err(e) if e.contains("not an Update") => {
        // 变更类型错误
        log::error!("无法回放非更新类型的变更: {}", e);
    }
    Err(e) => {
        log::error!("回放失败: {}", e);
    }
}
```

---

## 常见问题

### Q1: Diff/Replay 与 Event Sourcing 的区别？

**Diff/Replay**:
- 字段级变更追踪
- 自动检测差异
- 适合简单场景
- 学习曲线低

**Event Sourcing**:
- 业务事件建模
- 手动定义事件
- 适合复杂领域
- 学习曲线高

### Q2: 如何处理大对象的变更？

```rust
// 方案 1: 分解为多个小对象
#[derive(Diff, Replay)]
struct OrderHeader { /* 订单头 */ }

#[derive(Diff, Replay)]
struct OrderLine { /* 订单行 */ }

// 方案 2: 跳过大字段
#[derive(Diff, Replay)]
struct Document {
    id: String,
    metadata: String,
    #[diff(skip)]
    large_content: String,  // 跳过大字段
}
```

### Q3: 如何处理复杂类型（枚举、嵌套结构）？

```rust
// 枚举需要实现 Display 和 FromStr
#[derive(Debug, Clone, PartialEq)]
enum OrderStatus {
    Pending,
    Completed,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Completed => write!(f, "Completed"),
        }
    }
}

impl std::str::FromStr for OrderStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(OrderStatus::Pending),
            "Completed" => Ok(OrderStatus::Completed),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

// 然后可以在结构体中使用
#[derive(Diff, Replay)]
struct Order {
    status: OrderStatus,  // ✅ 自动支持
}
```

### Q4: 如何保证数据一致性？

```rust
// 方案 1: 使用事务
db.transaction(|tx| {
    // 1. 保存实体状态
    tx.save_entity(&entity)?;

    // 2. 保存变更日志
    tx.save_change_log(&change_log)?;

    Ok(())
})?;

// 方案 2: 先保存日志，再更新实体（Event Sourcing 方式）
save_change_log(&change_log)?;
update_entity_from_log(&change_log)?;
```

### Q5: 变更日志会无限增长吗？

**解决方案**:

1. **定期归档**: 将旧日志归档到冷存储
2. **快照压缩**: 每 N 个变更生成快照，删除中间日志
3. **TTL 策略**: 设置日志过期时间
4. **业务规则**: 根据业务需求保留必要日志

```rust
// 示例: 快照压缩
fn compact_logs(entity_id: &str) {
    // 1. 回放所有日志生成快照
    let logs = load_all_logs(entity_id);
    let mut entity = Entity::default();
    for log in logs {
        entity.replay(&log).unwrap();
    }

    // 2. 保存快照
    save_snapshot(&entity);

    // 3. 删除旧日志
    delete_logs_before(entity_id, snapshot.timestamp);
}
```

---

## 总结

### Diff/Replay 的价值

1. **业务价值**
   - ✅ 完整的审计追踪
   - ✅ 数据变更可追溯
   - ✅ 支持撤销/重做
   - ✅ 合规性保证

2. **技术价值**
   - ✅ 调试友好（时间旅行）
   - ✅ 测试友好（确定性回放）
   - ✅ 数据恢复能力
   - ✅ 事件驱动架构基础

3. **开发价值**
   - ✅ 自动生成代码（derive 宏）
   - ✅ 类型安全
   - ✅ 简单易用
   - ✅ 零学习成本

### 下一步

- 查看 [BDD 测试示例](../diff_tracker/tests/bdd_replay_test.rs)
- 查看 [Replay Derive 测试](../diff_tracker/tests/replay_derive_test.rs)
- 查看 [API 文档](../diff_tracker/src/lib.rs)

---

**Happy Event Sourcing! 🎉**
