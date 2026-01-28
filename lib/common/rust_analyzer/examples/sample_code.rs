// 示例：演示各种性能问题的代码

use std::collections::HashMap;

// ❌ 问题1: 未对齐的结构体
pub struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
    last_price: f64,
}

// ❌ 问题2: 热路径中的堆分配
pub fn process_orders(orders: &[Order]) -> Vec<Order> {
    let mut results = Vec::new();  // 每次调用都分配

    for order in orders {
        let processed = order.clone();  // ❌ 不必要的克隆
        results.push(processed);
    }

    results
}

// ❌ 问题3: 字符串拼接在循环中
pub fn generate_report(items: &[String]) -> String {
    let mut report = String::new();

    for item in items {
        report = report + item + "\n";  // ❌ 每次都重新分配
    }

    report
}

// ❌ 问题4: 小函数未内联
pub fn calculate_fee(amount: f64) -> f64 {
    amount * 0.001
}

// ❌ 问题5: 递归函数
pub fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

// ❌ 问题6: 过度使用 unwrap
pub fn get_price(prices: &HashMap<String, f64>, symbol: &str) -> f64 {
    prices.get(symbol).unwrap().clone()
}

// ❌ 问题7: 可向量化但未优化的循环
pub fn multiply_arrays(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(a.len());

    for i in 0..a.len() {
        result.push(a[i] * b[i]);  // 可以向量化
    }

    result
}

#[derive(Clone)]
pub struct Order {
    pub id: u64,
    pub price: f64,
    pub quantity: f64,
}

// ✅ 优化后的示例

// ✅ 正确: 缓存行对齐
#[repr(align(64))]
pub struct OptimizedOrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
    last_price: f64,
}

// ✅ 正确: 预分配和重用
pub fn process_orders_optimized(orders: &[Order], results: &mut Vec<Order>) {
    results.clear();
    results.reserve(orders.len());

    for order in orders {
        results.push(order.clone());
    }
}

// ✅ 正确: 使用 StringBuilder
pub fn generate_report_optimized(items: &[String]) -> String {
    let capacity = items.iter().map(|s| s.len() + 1).sum();
    let mut report = String::with_capacity(capacity);

    for item in items {
        report.push_str(item);
        report.push('\n');
    }

    report
}

// ✅ 正确: 添加内联提示
#[inline(always)]
pub fn calculate_fee_optimized(amount: f64) -> f64 {
    amount * 0.001
}

// ✅ 正确: 迭代版本
pub fn fibonacci_optimized(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

// ✅ 正确: 使用 ? 操作符
pub fn get_price_optimized(prices: &HashMap<String, f64>, symbol: &str) -> Option<f64> {
    prices.get(symbol).copied()
}

// ✅ 正确: SIMD 向量化提示
#[inline(always)]
pub fn multiply_arrays_optimized(a: &[f64], b: &[f64]) -> Vec<f64> {
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .collect()
}

fn main() {}