# Event Sourcing 与 Diff/Replay 学习指南

> 详细版本请查看: [EVENT_SOURCING.md](EVENT_SOURCING.md)

## 核心思想

**Event Sourcing = 记录"发生了什么"，而非"现在是什么"**

```
传统 CRUD: 只保存最终状态
Event Sourcing: 保存所有变更历史

当前状态 = 初始状态 + 所有历史事件
```

## 30 秒快速上手

```rust
use diff_tracker::{Diff, Replay, track_auto};

// 1. 定义实体
#[derive(Debug, Clone, Diff, Replay)]
struct Order {
    id: String,
    amount: i64,
    status: String,
}

fn main() {
    // 2. 创建初始状态
    let mut order = Order {
        id: "ORD-001".to_string(),
        amount: 1000,
        status: "Pending".to_string(),
    };

    // 3. 录制变更（快照 + 执行变更）
    let snapshot = order.clone();
    let change_log = track_auto(&mut order, |o| {
        o.status = "Completed".to_string();
        o.amount = 1200;
    }).unwrap();

    // 4. 回放变更
    let mut order_replay = snapshot;
    order_replay.replay(&change_log).unwrap();

    // 验证: 回放后状态与原始状态一致
    assert_eq!(order_replay.status, order.status);
    assert_eq!(order_replay.amount, order.amount);
}
```

## 实战示例

### 示例 1: 订单状态流转

```rust
#[derive(Debug, Clone, Diff, Replay)]
struct Order {
    id: String,
    status: String,
    is_paid: bool,
}

impl Order {
    fn pay(&mut self) {
        self.status = "Paid".to_string();
        self.is_paid = true;
    }

    fn ship(&mut self) {
        self.status = "Shipped".to_string();
    }
}

fn main() {
    let mut order = Order {
        id: "ORD-001".to_string(),
        status: "Created".to_string(),
        is_paid: false,
    };

    let mut logs = Vec::new();

    // 步骤 1: 支付
    logs.push(track_auto(&mut order, |o| o.pay()).unwrap());

    // 步骤 2: 发货
    logs.push(track_auto(&mut order, |o| o.ship()).unwrap());

    // 🎥 时间旅行: 从头回放
    let mut replay = Order {
        id: "ORD-001".to_string(),
        status: "Created".to_string(),
        is_paid: false,
    };

    for log in &logs {
        replay.replay(log).unwrap();
    }

    assert_eq!(replay.status, "Shipped");
}
```

### 示例 2: 银行账户审计

```rust
#[derive(Debug, Clone, Diff, Replay)]
struct Account {
    id: String,
    balance: i64,
}

impl Account {
    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i64) {
        self.balance -= amount;
    }
}

fn main() {
    let mut account = Account { id: "A001".to_string(), balance: 0 };
    let mut audit_trail = Vec::new();

    // 存款 1000
    audit_trail.push(track_auto(&mut account, |a| a.deposit(1000)).unwrap());

    // 取款 300
    audit_trail.push(track_auto(&mut account, |a| a.withdraw(300)).unwrap());

    // 存款 500
    audit_trail.push(track_auto(&mut account, |a| a.deposit(500)).unwrap());

    println!("最终余额: {}", account.balance); // 1200

    // 📊 审计: 查看每一步的余额变化
    let mut audit = Account { id: "A001".to_string(), balance: 0 };
    for (i, log) in audit_trail.iter().enumerate() {
        audit.replay(log).unwrap();
        println!("步骤 {}: 余额 = {}", i + 1, audit.balance);
    }
}
```

## 核心 API

### `track_auto` - 自动追踪变更

```rust
let change_log = track_auto(&mut entity, |e| {
    e.field1 = new_value1;
    e.field2 = new_value2;
}).unwrap();
```

### `Diff` derive - 自动检测差异

```rust
#[derive(Diff)]
struct Entity {
    field1: String,
    field2: i64,
}

// 自动生成 diff() 方法
```

### `Replay` derive - 自动回放

```rust
#[derive(Replay)]
struct Entity {
    field1: String,
    field2: i64,
}

// 自动生成 replay() 方法
entity.replay(&change_log).unwrap();
```

## 高级特性

### 字段脱敏

```rust
#[derive(Diff, Replay)]
struct User {
    username: String,
    #[diff(mask)]  // 🔒 密码会被脱敏为 "***"
    password: String,
}
```

### 跳过字段

```rust
#[derive(Diff, Replay)]
struct Entity {
    data: String,
    #[diff(skip)]  // ⏭️ cache 字段不会被追踪
    cache: Option<String>,
}
```

## 最佳实践

1. **使用 Clone 创建快照**
   ```rust
   let snapshot = entity.clone();
   let log = track_auto(&mut entity, |e| { /* ... */ }).unwrap();
   ```

2. **封装业务逻辑**
   ```rust
   impl Order {
       pub fn complete(&mut self) {
           self.status = "Completed".to_string();
           self.completed_at = now();
       }
   }
   ```

3. **定期保存快照**
   ```rust
   if change_count % 100 == 0 {
       save_snapshot(&entity);
   }
   ```

## 相关资料

- 完整文档: [EVENT_SOURCING.md](EVENT_SOURCING.md)
- BDD 测试: [bdd_replay_test.rs](../tests/bdd_replay_test.rs)
- Replay Derive 测试: [replay_derive_test.rs](../tests/replay_derive_test.rs)

---

**开始你的 Event Sourcing 之旅! 🚀**
