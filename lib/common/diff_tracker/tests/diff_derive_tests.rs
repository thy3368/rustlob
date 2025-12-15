use diff::{ChangeType, Diff};
use diff_tracker::tracker::tracker::track_auto;

// ========== Diff Derive 宏测试 ==========

// 🎉 使用 Diff derive 宏的测试实体
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct User {
    id: String,
    name: String,
    age: i32,
}

#[test]
fn test_diff_derive_basic() {
    println!("\n=== Diff Derive 宏测试 - 基础功能 ===\n");

    let user1 = User {
        id: "user_001".to_string(),
        name: "Alice".to_string(),
        age: 30,
    };

    let user2 = User {
        id: "user_001".to_string(),
        name: "Alice Smith".to_string(),
        age: 31,
    };

    // 使用自动生成的 diff 方法
    let changes = user1.diff(&user2);

    assert_eq!(changes.len(), 2);
    println!("✓ 检测到 {} 个字段变更", changes.len());

    // 验证 name 变更
    let name_change = changes.iter().find(|c| c.field_name == "name").unwrap();
    assert_eq!(name_change.old_value, "Alice");
    assert_eq!(name_change.new_value, "Alice Smith");
    println!("  ✓ name: {} → {}", name_change.old_value, name_change.new_value);

    // 验证 age 变更
    let age_change = changes.iter().find(|c| c.field_name == "age").unwrap();
    assert_eq!(age_change.old_value, "30");
    assert_eq!(age_change.new_value, "31");
    println!("  ✓ age: {} → {}", age_change.old_value, age_change.new_value);

    println!("\n=== Diff Derive 基础测试通过! ===\n");
}

#[test]
fn test_diff_derive_no_changes() {
    let user1 = User {
        id: "user_002".to_string(),
        name: "Bob".to_string(),
        age: 25,
    };

    let user2 = user1.clone();

    let changes = user1.diff(&user2);

    assert_eq!(changes.len(), 0);
    println!("✓ 正确检测到 0 个字段变更");
}

// 测试 skip 属性
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct Account {
    id: String,
    balance: i64,
    #[diff(skip)]
    internal_cache: Option<String>,
}

#[test]
fn test_diff_skip_attribute() {
    println!("\n=== Diff Derive - skip 属性测试 ===\n");

    let acc1 = Account {
        id: "acc_001".to_string(),
        balance: 1000,
        internal_cache: Some("cache1".to_string()),
    };

    let acc2 = Account {
        id: "acc_001".to_string(),
        balance: 2000,
        internal_cache: Some("cache2".to_string()),  // 这个字段会被跳过
    };

    let changes = acc1.diff(&acc2);

    // 只应该检测到 balance 的变更，internal_cache 被跳过
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].field_name, "balance");
    assert_eq!(changes[0].old_value, "1000");
    assert_eq!(changes[0].new_value, "2000");

    println!("✓ skip 属性生效：只检测到 1 个字段变更 (balance)");
    println!("✓ internal_cache 被正确跳过");

    println!("\n=== skip 属性测试通过! ===\n");
}

// 测试 mask 属性
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct SecureUser {
    id: String,
    username: String,
    #[diff(mask)]
    password: String,
}

#[test]
fn test_diff_mask_attribute() {
    println!("\n=== Diff Derive - mask 属性测试 ===\n");

    let user1 = SecureUser {
        id: "user_003".to_string(),
        username: "alice".to_string(),
        password: "old_password".to_string(),
    };

    let user2 = SecureUser {
        id: "user_003".to_string(),
        username: "alice_updated".to_string(),
        password: "new_password".to_string(),
    };

    let changes = user1.diff(&user2);

    assert_eq!(changes.len(), 2);

    // 验证 username 变更
    let username_change = changes.iter().find(|c| c.field_name == "username").unwrap();
    assert_eq!(username_change.old_value, "alice");
    assert_eq!(username_change.new_value, "alice_updated");
    println!("✓ username: {} → {}", username_change.old_value, username_change.new_value);

    // 验证 password 被脱敏
    let password_change = changes.iter().find(|c| c.field_name == "password").unwrap();
    assert_eq!(password_change.old_value, "***");
    assert_eq!(password_change.new_value, "***");
    println!("✓ password 被脱敏为: ***");

    println!("\n=== mask 属性测试通过! ===\n");
}

// 测试 Diff derive 与 track_auto 集成
#[derive(Debug, Clone, PartialEq, diff_tracker::Diff)]
struct Order {
    id: String,
    amount: i64,
    status: String,
}

#[test]
fn test_diff_derive_with_track_auto() {
    println!("\n=== Diff Derive + track_auto 集成测试 ===\n");

    let mut order = Order {
        id: "ord_001".to_string(),
        amount: 100,
        status: "pending".to_string(),
    };

    // 使用 track_auto，自动使用 derive 生成的 Diff 实现
    let entry = track_auto(&mut order, |o| {
        o.amount = 200;
        o.status = "completed".to_string();
    }).unwrap();

    if let ChangeType::Updated { changed_fields } = &entry.change_type {
        assert_eq!(changed_fields.len(), 2);
        println!("✓ track_auto + Diff derive: 检测到 {} 个字段变更", changed_fields.len());

        for change in changed_fields {
            println!("  - {}: {} → {}", change.field_name, change.old_value, change.new_value);
        }
    }

    println!("\n=== 集成测试通过! ===\n");
}
