#![allow(clippy::unwrap_used)]
#![allow(clippy::indexing_slicing)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::assign_op_pattern)]
use diff::{ChangeLogEntry, ChangeType};
use diff_tracker::tracker::track_auto;
use diff_tracker::Replay;  // 导入 Replay trait

// ========== BDD: 数据录制与回放 ==========

/// 订单实体 - 使用 Diff 和 Replay derive 宏自动生成 diff() 和 replay() 方法
#[derive(Debug, Clone, diff_tracker::Diff, diff_tracker::Replay)]
struct Order {
    id: String,
    amount: i64,
    quantity: u32,
    price: f64,
    status: String,  // 简化：使用 String 而非枚举
    is_paid: bool,
}

impl Order {
    fn new(id: String, amount: i64, quantity: u32) -> Self {
        Self {
            id,
            amount,
            quantity,
            price: amount as f64 / quantity as f64,
            status: "Pending".to_string(),
            is_paid: false,
        }
    }

    /// 业务逻辑：处理订单
    fn process(&mut self) {
        self.status = "Processing".to_string();
        self.amount = self.amount + 100;  // 加税费
        self.price = self.amount as f64 / self.quantity as f64;
    }

    /// 业务逻辑：完成订单
    fn complete(&mut self) {
        self.status = "Completed".to_string();
        self.is_paid = true;
    }
}

#[test]
fn test_bdd_scenario_1_basic_record_and_replay() {
    println!("
========================================");
    println!("场景 1: 基础录制与回放");
    println!("========================================
");

    // Given: 初始订单
    println!("Given: 创建一个初始订单");
    let mut order = Order::new("ORD-001".to_string(), 1000, 10);
    println!("  订单ID: {}", order.id);
    println!("  金额: {}", order.amount);
    println!("  数量: {}", order.quantity);
    println!("  价格: {:.2}", order.price);
    println!("  状态: {}", order.status);
    println!("  已支付: {}", order.is_paid);

    // When: 处理订单并录制变更
    println!("
When: 处理订单（添加税费）");
    let log_entry = track_auto(&mut order, |o| {
        o.process();
    }).unwrap();

    println!("  录制的变更:");
    if let ChangeType::Updated { changed_fields } = &log_entry.change_type {
        for change in changed_fields {
            println!("    - {}: {} → {}",
                change.field_name,
                change.old_value,
                change.new_value);
        }
    }

    // Then: 验证录制的变更
    println!("
Then: 验证变更被正确录制");
    assert_eq!(order.amount, 1100);
    assert_eq!(order.status, "Processing");
    println!("  ✓ 订单状态已更新");

    // Given: 从初始状态开始
    println!("
----------------------------------------");
    println!("Given: 重置到初始状态");
    let mut order_replay = Order::new("ORD-001".to_string(), 1000, 10);
    println!("  金额: {} (初始)", order_replay.amount);
    println!("  状态: {} (初始)", order_replay.status);

    // When: 回放变更日志
    println!("
When: 回放变更日志");
    order_replay.replay(&log_entry).unwrap();  // 使用 Replay derive 生成的方法

    // Then: 验证回放结果
    println!("
Then: 验证回放后的状态");
    assert_eq!(order_replay.amount, 1100);
    assert_eq!(order_replay.status, "Processing");
    assert_eq!(order_replay.price, 110.0);
    println!("  ✓ 金额: {} (回放后)", order_replay.amount);
    println!("  ✓ 状态: {} (回放后)", order_replay.status);
    println!("  ✓ 价格: {:.2} (回放后)", order_replay.price);

    println!("
✅ 场景 1 通过：基础录制与回放成功
");
}

#[test]
fn test_bdd_scenario_2_multi_step_replay() {
    println!("
========================================");
    println!("场景 2: 多步骤变更录制与回放");
    println!("========================================
");

    // Given: 初始订单
    println!("Given: 创建一个新订单");
    let mut order = Order::new("ORD-002".to_string(), 2000, 5);
    println!("  初始状态: amount={}, status={}, is_paid={}",
        order.amount, order.status, order.is_paid);

    // When: 执行多步业务操作
    println!("
When: 执行多步业务操作");
    let mut change_logs = Vec::new();

    println!("  步骤 1: 处理订单");
    let log1 = track_auto(&mut order, |o| {
        o.process();
    }).unwrap();
    change_logs.push(log1);

    println!("  步骤 2: 完成订单");
    let log2 = track_auto(&mut order, |o| {
        o.complete();
    }).unwrap();
    change_logs.push(log2);

    println!("  最终状态: amount={}, status={}, is_paid={}",
        order.amount, order.status, order.is_paid);

    // Then: 按顺序回放所有变更
    println!("
Then: 从初始状态回放所有变更");
    let mut order_replay = Order::new("ORD-002".to_string(), 2000, 5);
    println!("  回放前: amount={}, status={}, is_paid={}",
        order_replay.amount, order_replay.status, order_replay.is_paid);

    for (i, log) in change_logs.iter().enumerate() {
        println!("
  回放步骤 {}:", i + 1);
        order_replay.replay(log).unwrap();  // 使用 Replay derive 生成的方法

        if let ChangeType::Updated { changed_fields } = &log.change_type {
            for change in changed_fields {
                println!("    - {}: {} → {}",
                    change.field_name,
                    change.old_value,
                    change.new_value);
            }
        }
    }

    println!("
  回放后: amount={}, status={}, is_paid={}",
        order_replay.amount, order_replay.status, order_replay.is_paid);

    // 验证最终状态
    assert_eq!(order_replay.amount, order.amount);
    assert_eq!(order_replay.status, order.status);
    assert_eq!(order_replay.is_paid, order.is_paid);

    println!("
✅ 场景 2 通过：多步骤回放成功
");
}

#[test]
fn test_bdd_scenario_3_type_safety_issue() {
    println!("
========================================");
    println!("场景 3: 值类型问题分析");
    println!("========================================
");

    // Given: 创建订单
    println!("Given: 创建一个订单");
    let mut order = Order::new("ORD-003".to_string(), 999, 3);

    // When: 录制变更
    println!("
When: 修改订单金额");
    let log = track_auto(&mut order, |o| {
        o.amount = 1500;
    }).unwrap();

    // Then: 分析 FieldChange 的值类型
    println!("
Then: 分析 FieldChange 中的值类型");
    if let ChangeType::Updated { changed_fields } = &log.change_type {
        for change in changed_fields {
            println!("  字段: {}", change.field_name);
            println!("  旧值: {} (String 类型)", change.old_value);
            println!("  新值: {} (String 类型)", change.new_value);

            // 问题 1: 类型信息丢失
            println!("
  ⚠️  问题 1: 类型信息丢失");
            println!("    - 存储时: i64(999) → String(\"999\")");
            println!("    - 回放时需要: String(\"999\") → i64(999)");
            println!("    - 需要知道字段的原始类型才能正确解析");

            // 问题 2: 解析可能失败
            println!("
  ⚠️  问题 2: 解析可能失败");
            println!("    - 如果 String 不是有效的数字，parse() 会失败");
            println!("    - 需要错误处理");

            // 问题 3: 浮点精度
            if change.field_name == "price" {
                println!("
  ⚠️  问题 3: 浮点数精度问题");
                println!("    - f64(333.0) → String(\"333\") → f64(333.0)");
                println!("    - 字符串转换可能丢失精度");
            }
        }
    }

    println!("
✅ 场景 3 通过：已识别值类型的潜在问题
");
}

#[test]
fn test_bdd_scenario_4_type_parsing_errors() {
    println!("
========================================");
    println!("场景 4: 类型解析错误处理");
    println!("========================================
");

    // Given: 创建一个订单和一个损坏的变更日志
    println!("Given: 创建一个正常订单");
    let mut order = Order::new("ORD-004".to_string(), 1000, 10);

    // When: 创建一个包含无效数据的变更日志
    println!("
When: 尝试回放包含无效类型的变更日志");
    use diff::FieldChange;
    let corrupted_log = ChangeLogEntry {
        entity_id: "ORD-004".to_string(),
        entity_type: "Order".to_string(),
        change_type: ChangeType::Updated {
            changed_fields: vec![
                FieldChange {
                    field_name: "amount".to_string(),
                    old_value: "1000".to_string(),
                    new_value: "not_a_number".to_string(),  // 无效的数字
                }
            ]
        },
        timestamp: 0,
    };

    // Then: 回放应该失败并返回错误
    println!("
Then: 回放失败并返回解析错误");
    let result = order.replay(&corrupted_log);  // 使用 Replay derive 生成的方法

    assert!(result.is_err());
    if let Err(e) = result {
        println!("  ✓ 捕获到错误: {}", e);
        assert!(e.contains("Failed to parse field 'amount'"));  // derive 宏生成的错误消息格式
    }

    println!("
✅ 场景 4 通过：类型解析错误被正确处理
");
}

// ========== 总结：FieldChange 是否需要考虑值类型？ ==========

#[test]
fn test_analysis_should_we_add_types() {
    println!("
========================================");
    println!("分析：FieldChange 是否需要值类型？");
    println!("========================================
");

    println!("当前设计 (String-based):");
    println!("  pub struct FieldChange {{");
    println!("      pub field_name: String,");
    println!("      pub old_value: String,  // ← 所有类型都存为 String");
    println!("      pub new_value: String,  // ← 所有类型都存为 String");
    println!("  }}");
    println!();

    println!("✅ 优点:");
    println!("  1. 简单：所有类型都统一为 String");
    println!("  2. 序列化友好：易于存储到数据库或 JSON");
    println!("  3. 调试友好：可以直接打印查看");
    println!("  4. 通用性强：适用于任何类型");
    println!();

    println!("❌ 缺点:");
    println!("  1. 类型信息丢失：回放时需要知道原始类型");
    println!("  2. 解析开销：每次回放都需要 parse()");
    println!("  3. 解析失败：无效的 String 会导致回放失败");
    println!("  4. 精度问题：浮点数转 String 可能丢失精度");
    println!("  5. 复杂类型：枚举、结构体需要自定义序列化");
    println!();

    println!("🤔 改进方案选项:");
    println!();
    println!("方案 1: 添加类型标记");
    println!("  pub struct FieldChange {{");
    println!("      pub field_name: String,");
    println!("      pub field_type: String,      // ← 新增：\"i64\", \"f64\", \"bool\"");
    println!("      pub old_value: String,");
    println!("      pub new_value: String,");
    println!("  }}");
    println!("  优点：保持 String 的优点，增加类型提示");
    println!("  缺点：仍需要 parse()，只是有了类型提示");
    println!();

    println!("方案 2: 使用泛型值");
    println!("  pub struct FieldChange {{");
    println!("      pub field_name: String,");
    println!("      pub old_value: serde_json::Value,  // ← JSON Value");
    println!("      pub new_value: serde_json::Value,");
    println!("  }}");
    println!("  优点：保留类型信息，支持复杂类型");
    println!("  缺点：依赖 serde_json，增加复杂度");
    println!();

    println!("方案 3: 类型化枚举");
    println!("  pub enum FieldValue {{");
    println!("      Int64(i64),");
    println!("      Float64(f64),");
    println!("      Bool(bool),");
    println!("      String(String),");
    println!("  }}");
    println!("  pub struct FieldChange {{");
    println!("      pub field_name: String,");
    println!("      pub old_value: FieldValue,  // ← 类型化枚举");
    println!("      pub new_value: FieldValue,");
    println!("  }}");
    println!("  优点：类型安全，无需 parse()，性能好");
    println!("  缺点：枚举有限，不支持任意类型");
    println!();

    println!("📋 建议:");
    println!("  - 如果只用于日志和审计 → 保持 String（当前方案）");
    println!("  - 如果需要数据回放 → 方案 1 或 2");
    println!("  - 如果性能敏感 → 方案 3");
    println!();

    println!("✅ 本测试演示了当前 String 方案的优缺点
");
}
