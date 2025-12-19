use fixed_point::arithmetic::FixedPoint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FixedPointArithmetic 基础示例 ===\n");

    // 1. 创建价格（股票精度：0.01）
    let price1 = FixedPoint::from_f64(123.45, -2)?;
    let price2 = FixedPoint::from_f64(56.78, -2)?;

    println!("Price 1: ${}", price1.to_f64());
    println!("Price 2: ${}", price2.to_f64());
    println!();

    // 2. 加法
    let sum = price1.checked_add(price2)?;
    println!("Sum: ${}", sum.to_f64());

    // 3. 减法
    let diff = price1.checked_sub(price2)?;
    println!("Diff: ${}", diff.to_f64());

    // 4. 乘法
    let quantity = FixedPoint::from_f64(10.0, -2)?;
    let total = price1.checked_mul(quantity)?;
    println!("Total (price1 × 10): ${}", total.to_f64());

    // 5. 除法
    let half = price1.checked_div(FixedPoint::from_f64(2.0, -2)?)?;
    println!("Half: ${}", half.to_f64());
    println!();

    // 6. 序列化/反序列化（网络传输）
    let bytes = price1.to_bytes();
    println!("Serialized to {} bytes: {:?}", bytes.len(), bytes);

    let restored = FixedPoint::from_bytes(bytes);
    println!("Deserialized: ${}", restored.to_f64());
    println!();

    // 7. 批量处理
    let prices =
        vec![FixedPoint::from_f64(100.0, -2)?, FixedPoint::from_f64(200.0, -2)?, FixedPoint::from_f64(300.0, -2)?];

    let f64_prices = FixedPoint::batch_to_f64(&prices);
    println!("Batch conversion: {:?}", f64_prices);
    println!();

    // 8. Unsafe极速版本（性能关键路径）
    unsafe {
        let fast_price = FixedPoint::from_f64_unchecked(999.99, -2);
        let fast_qty = FixedPoint::from_f64_unchecked(5.0, -2);
        let fast_result = fast_price.add_unchecked(fast_qty);
        println!("Fast unchecked add: ${}", fast_result.to_f64_fast());
    }

    println!("\n✅ 所有示例运行成功!");
    Ok(())
}
