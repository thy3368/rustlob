#![allow(clippy::unwrap_used)]
use diff_tracker::{ChangeType, Diff, Tracked};

// ========== Tracked Derive 测试 ==========

#[derive(Clone, Debug, PartialEq, Diff, Tracked)]
struct Order {
    id: String,
    symbol: String,
    price: i64,
    quantity: u32,
    status: String,
}

impl Order {
    fn new() -> Self {
        Self {
            id: "ORD123".to_string(),
            symbol: "BTCUSDT".to_string(),
            price: 50000,
            quantity: 100,
            status: "pending".to_string(),
        }
    }
}

#[test]
fn test_tracked_derive_basic() {
    let mut order = Order::new();

    // ✨ 使用 derive 生成的 tracked_update 方法
    let entry = order.tracked_update(|o| {
        o.price = 51000;
        o.status = "confirmed".to_string();
    }).unwrap();

    // 验证变更被正确追踪
    match entry.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 2);

            // 验证 price 变更
            let price_change = changed_fields.iter()
                .find(|c| c.field_name == "price")
                .unwrap();
            assert_eq!(price_change.old_value, "50000");
            assert_eq!(price_change.new_value, "51000");

            // 验证 status 变更
            let status_change = changed_fields.iter()
                .find(|c| c.field_name == "status")
                .unwrap();
            assert_eq!(status_change.old_value, "pending");
            assert_eq!(status_change.new_value, "confirmed");
        }
        _ => panic!("Expected ChangeType::Updated"),
    }

    // 验证实体确实被更新了
    assert_eq!(order.price, 51000);
    assert_eq!(order.status, "confirmed");
}

#[test]
fn test_tracked_derive_no_changes() {
    let mut order = Order::new();

    // 不做任何修改
    let entry = order.tracked_update(|_o| {
        // 空闭包
    }).unwrap();

    // 应该没有任何变更
    match entry.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 0);
        }
        _ => panic!("Expected ChangeType::Updated"),
    }
}

#[test]
fn test_tracked_derive_multiple_updates() {
    let mut order = Order::new();

    // 第一次更新
    let entry1 = order.tracked_update(|o| {
        o.price = 51000;
    }).unwrap();

    match entry1.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 1);
            assert_eq!(changed_fields[0].field_name, "price");
        }
        _ => panic!("Expected ChangeType::Updated"),
    }

    // 第二次更新
    let entry2 = order.tracked_update(|o| {
        o.status = "completed".to_string();
        o.quantity = 200;
    }).unwrap();

    match entry2.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 2);
        }
        _ => panic!("Expected ChangeType::Updated"),
    }
}

#[test]
fn test_tracked_vs_track_auto() {
    let mut order1 = Order::new();
    let mut order2 = Order::new();

    // 方法 1: 使用 Tracked derive
    let entry1 = order1.tracked_update(|o| {
        o.price = 51000;
        o.status = "confirmed".to_string();
    }).unwrap();

    // 方法 2: 使用 track_auto 函数
    let entry2 = diff_tracker::track_auto(&mut order2, |o| {
        o.price = 51000;
        o.status = "confirmed".to_string();
    }).unwrap();

    // 两种方法应该产生相同的结果
    match (&entry1.change_type, &entry2.change_type) {
        (
            ChangeType::Updated { changed_fields: fields1 },
            ChangeType::Updated { changed_fields: fields2 }
        ) => {
            assert_eq!(fields1.len(), fields2.len());
            assert_eq!(fields1, fields2);
        }
        _ => panic!("Expected ChangeType::Updated"),
    }

    // 两个订单状态应该相同
    assert_eq!(order1, order2);
}

// ========== 带泛型的结构体测试 ==========

#[derive(Clone, Debug, PartialEq, Diff, Tracked)]
struct Container<T: Clone + PartialEq + ToString> {
    id: String,
    value: T,
}

#[test]
fn test_tracked_with_generics() {
    let mut container = Container {
        id: "C1".to_string(),
        value: 42i32,
    };

    let entry = container.tracked_update(|c| {
        c.value = 100;
    }).unwrap();

    match entry.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 1);
            assert_eq!(changed_fields[0].field_name, "value");
            assert_eq!(changed_fields[0].old_value, "42");
            assert_eq!(changed_fields[0].new_value, "100");
        }
        _ => panic!("Expected ChangeType::Updated"),
    }
}

// ========== 业务逻辑方法调用测试 ==========

#[derive(Clone, Debug, PartialEq, Diff, Tracked)]
struct Account {
    id: String,
    balance: i64,
    status: String,
}

impl Account {
    fn new(id: String, balance: i64) -> Self {
        Self {
            id,
            balance,
            status: "active".to_string(),
        }
    }

    fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i64) {
        self.balance -= amount;
    }

    fn close(&mut self) {
        self.status = "closed".to_string();
        self.balance = 0;
    }
}

#[test]
fn test_tracked_with_business_methods() {
    let mut account = Account::new("ACC001".to_string(), 1000);

    // 使用 tracked_update 追踪业务方法调用
    let entry = account.tracked_update(|acc| {
        acc.deposit(500);
        acc.withdraw(200);
    }).unwrap();

    match entry.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 1);
            let balance_change = &changed_fields[0];
            assert_eq!(balance_change.field_name, "balance");
            assert_eq!(balance_change.old_value, "1000");
            assert_eq!(balance_change.new_value, "1300");  // 1000 + 500 - 200
        }
        _ => panic!("Expected ChangeType::Updated"),
    }

    assert_eq!(account.balance, 1300);
}

#[test]
fn test_tracked_close_account() {
    let mut account = Account::new("ACC002".to_string(), 5000);

    let entry = account.tracked_update(|acc| {
        acc.close();
    }).unwrap();

    match entry.change_type {
        ChangeType::Updated { changed_fields } => {
            assert_eq!(changed_fields.len(), 2);

            // 验证 balance 和 status 都被修改
            assert!(changed_fields.iter().any(|c| c.field_name == "balance"));
            assert!(changed_fields.iter().any(|c| c.field_name == "status"));
        }
        _ => panic!("Expected ChangeType::Updated"),
    }

    assert_eq!(account.balance, 0);
    assert_eq!(account.status, "closed");
}
