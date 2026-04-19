use std::time::{Instant, SystemTime, UNIX_EPOCH};

use rand::Rng;

use crate::k_line::aggregator::k_line_aggregator::KLineAggregator;
use crate::k_line::k_line_types::{KLineAgg, TimeWindow};

#[test]
fn main2() {
    println!("=== 高性能 K 线聚合器演示 ===\n");

    // 创建聚合器
    let aggregator = KLineAggregator::new();

    // 订阅 K 线更新事件
    aggregator.subscribe(|event| {
        let window_name = match event.window {
            TimeWindow::Second => "1秒",
            TimeWindow::Minute => "1分钟",
            TimeWindow::FifteenMin => "15分钟",
            TimeWindow::Hour => "1小时",
        };

        if event.is_new_window {
            println!(
                "[{}] 新窗口 K 线生成: 时间={}, 开盘={:.4}, 最高={:.4}, 最低={:.4}, 收盘={:.4}, 成交量={:.2}",
                window_name, event.ohlc.timestamp, event.ohlc.open, event.ohlc.high,
                event.ohlc.low, event.ohlc.close, event.ohlc.volume
            );
        }
    });

    // 模拟实时交易流
    let mut timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let mut price = 100.0;
    let mut rng = rand::thread_rng();

    println!("开始处理模拟交易...");

    let start = Instant::now();
    let mut trade_count = 0;

    // 模拟 100 万笔交易
    for _ in 0..1_000_000 {
        // 生成随机交易
        let change = rng.gen_range(-0.5..0.5);
        let new_price = price + change;
        price = if new_price > 1.0 { new_price } else { 1.0 };
        let volume = rng.gen_range(1.0..100.0);

        // 处理交易
        aggregator.process_trade(timestamp, price, volume).unwrap();

        trade_count += 1;
        timestamp += rng.gen_range(0..2);

        // 每 10 万笔显示进度
        if trade_count % 100_000 == 0 {
            let elapsed = start.elapsed();
            let speed = trade_count as f64 / elapsed.as_secs_f64();
            println!("已处理: {} 笔, 速度: {:.0} 笔/秒", trade_count, speed);
        }
    }

    let total_time = start.elapsed();
    println!("\n处理完成!");
    println!("总耗时: {:?}", total_time);
    println!("平均速度: {:.0} 笔/秒", 1_000_000.0 / total_time.as_secs_f64());

    // 显示结果
    let (total_trades, total_volume) = aggregator.get_total_stats();
    println!("\n统计信息:");
    println!("总交易数: {}", total_trades);
    println!("总交易量: {:.2}", total_volume as f64);

    // 显示当前 K 线
    for window in
        &[TimeWindow::Second, TimeWindow::Minute, TimeWindow::FifteenMin, TimeWindow::Hour]
    {
        if let Some(ohlc) = aggregator.get_current_ohlc(*window) {
            let window_name = match window {
                TimeWindow::Second => "1秒",
                TimeWindow::Minute => "1分钟",
                TimeWindow::FifteenMin => "15分钟",
                TimeWindow::Hour => "1小时",
            };

            println!("\n{} K线:", window_name);
            println!("  时间: {}", ohlc.timestamp);
            println!("  开盘: {:.4}", ohlc.open);
            println!("  最高: {:.4}", ohlc.high);
            println!("  最低: {:.4}", ohlc.low);
            println!("  收盘: {:.4}", ohlc.close);
            println!("  成交量: {:.2}", ohlc.volume);
            println!("  交易次数: {}", ohlc.count);
        }
    }

    // 显示滑动窗口统计
    println!("\n滑动窗口统计:");
    for window in
        &[TimeWindow::Second, TimeWindow::Minute, TimeWindow::FifteenMin, TimeWindow::Hour]
    {
        let (open, high, low, close, volume) = aggregator.get_sliding_stats(*window, 10);

        let window_name = match window {
            TimeWindow::Second => "最近10秒",
            TimeWindow::Minute => "最近10分钟",
            TimeWindow::FifteenMin => "最近10个15分钟",
            TimeWindow::Hour => "最近10小时",
        };

        println!("\n{}:", window_name);
        println!("  开盘: {:.4}", open);
        println!("  最高: {:.4}", high);
        println!("  最低: {:.4}", low);
        println!("  收盘: {:.4}", close);
        println!("  总成交量: {:.2}", volume);
    }
}
