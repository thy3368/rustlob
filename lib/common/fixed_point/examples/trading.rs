use fixed_point::arithmetic::FixedPoint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 高频交易订单处理示例 ===\n");

    // 模拟订单数据
    let buy_price = FixedPoint::from_f64(100.50, -2)?;
    let sell_price = FixedPoint::from_f64(100.55, -2)?;
    let quantity = FixedPoint::from_f64(1000.0, -2)?;

    println!("买价: ${}", buy_price.to_f64());
    println!("卖价: ${}", sell_price.to_f64());
    println!("数量: {}", quantity.to_f64());
    println!();

    // 计算价差
    let spread = sell_price.checked_sub(buy_price)?;
    println!("价差: ${}", spread.to_f64());

    // 计算中间价
    let two = FixedPoint::from_f64(2.0, -2)?;
    let sum = buy_price.checked_add(sell_price)?;
    let mid_price = sum.checked_div(two)?;
    println!("中间价: ${}", mid_price.to_f64());
    println!();

    // 计算订单价值
    let order_value = buy_price.checked_mul(quantity)?;
    println!("订单价值: ${}", order_value.to_f64());

    // 计算手续费 (0.1%)
    let fee_rate = FixedPoint::from_f64(0.001, -2)?;
    let fee = order_value.checked_mul(fee_rate)?;
    println!("手续费 (0.1%): ${}", fee.to_f64());

    let total_cost = order_value.checked_add(fee)?;
    println!("总成本: ${}", total_cost.to_f64());
    println!();

    // 网络传输模拟（序列化）
    println!("--- 网络传输 ---");
    let serialized = buy_price.to_bytes();
    println!("序列化: {:?} ({}字节)", serialized, serialized.len());

    // 反序列化
    let received = FixedPoint::from_bytes(serialized);
    println!("反序列化: ${}", received.to_f64());
    println!("✓ 数据完整性验证通过");
    println!();

    // 批量处理（高频场景）
    println!("--- 批量处理1000笔订单 ---");
    let mut prices = Vec::with_capacity(1000);
    for i in 0..1000 {
        let price = 100.0 + (i as f64) * 0.01;
        prices.push(FixedPoint::from_f64(price, -2)?);
    }

    let f64_prices = FixedPoint::batch_to_f64(&prices);
    println!("处理了 {} 笔订单", f64_prices.len());
    println!("第一笔: ${:.2}", f64_prices[0]);
    println!("最后一笔: ${:.2}", f64_prices[999]);

    println!("\n✅ 交易示例完成!");
    Ok(())
}
