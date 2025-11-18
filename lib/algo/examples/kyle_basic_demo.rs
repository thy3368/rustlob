//! Kyle 模型入门示例
//!
//! 通过实际场景帮助理解 Kyle 模型的核心概念
//!
//! 运行方式：
//! ```bash
//! cd /Users/hongyaotang/src/rustlob/lib/algo
//! cargo run --example kyle_basic_demo
//! ```

use algo::{KyleModelService, KyleParameters};

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║          Kyle 模型入门教程 - 通俗易懂版                ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    // ============================================================
    // 场景说明：你是一个基金经理，发现了一只被低估的股票
    // ============================================================
    println!("【背景故事】");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("你是某基金的投资经理，通过深入研究发现：");
    println!("  • 某股票当前市场价格：100 元");
    println!("  • 你的分析认为真实价值：110 元（被低估了10元！）");
    println!("  • 你想买入赚取差价\n");

    println!("【问题】");
    println!("如果你直接下单买 10,000 股会怎样？");
    println!("  ❌ 你的大单会推高价格（市场冲击）");
    println!("  ❌ 做市商会察觉有内幕，立即涨价");
    println!("  ❌ 最后你可能以 108 元成交，利润大幅缩水\n");

    println!("【Kyle 模型的解决方案】");
    println!("分批买入，隐藏在噪音交易者中，降低市场冲击！\n");

    pause();

    // ============================================================
    // 示例 1：理解价格影响系数 λ（lambda）
    // ============================================================
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  示例 1：理解价格影响系数 λ (lambda)                   ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("【什么是 λ？】");
    println!("λ 衡量「订单对价格的影响力」");
    println!("  公式：价格变化 = λ × 订单量\n");

    // 创建不同流动性的市场
    let markets = vec![
        ("高流动性市场（大盘股）", 10.0, 100.0), // σ_v=10, σ_u=100
        ("中等流动性市场", 10.0, 50.0),
        ("低流动性市场（小盘股）", 10.0, 20.0),
    ];

    println!("让我们看看在不同市场买入 1000 股的影响：\n");
    println!("市场类型              λ值      买1000股→价格涨幅");
    println!("─────────────────────────────────────────────");

    for (name, sigma_v, sigma_u) in markets {
        let params = KyleParameters::new(sigma_v, sigma_u, 100.0, 1);
        let lambda = params.price_impact();
        let price_change = lambda * 1000.0;

        println!(
            "{:20} {:6.4}    {:+.2} 元",
            name, lambda, price_change
        );
    }

    println!("\n【结论】");
    println!("  ✓ 流动性好的市场 → λ 小 → 大单冲击小");
    println!("  ✓ 流动性差的市场 → λ 大 → 大单冲击大");
    println!("  ✓ 做市商会根据 λ 来设置买卖价差\n");

    pause();

    // ============================================================
    // 示例 2：知情交易者的最优策略
    // ============================================================
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  示例 2：知情交易者如何分批买入                        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("【场景设定】");
    println!("  • 当前价格：100 元");
    println!("  • 你认为真实价值：115 元");
    println!("  • 总共想买：5000 股");
    println!("  • 分 5 轮执行\n");

    // 创建 Kyle 模型（多期）
    let params = KyleParameters::new(
        20.0,  // σ_v: 价值不确定性
        10.0,  // σ_u: 噪音交易波动
        100.0, // 初始价格
        5,     // 5轮交易
    );

    let mut service = KyleModelService::new(params);

    println!("【Kyle 模型参数】");
    println!("  λ (价格影响) = {:.4}", service.lambda());
    println!("  β (交易强度) = {:.4}", service.beta());
    println!("  含义：每轮买入 = β × (真实价值 - 当前价格)\n");

    // 模拟噪音交易（随机买卖）
    let noise_orders = vec![50.0, -30.0, 40.0, -20.0, 30.0];
    let true_value = 115.0;

    println!("【执行过程】");
    println!("轮次  知情买入  噪音交易  总订单  执行价格  价格涨幅  累积利润");
    println!("─────────────────────────────────────────────────────────");

    let mut total_profit = 0.0;
    let mut total_shares = 0.0;

    for (round, &noise) in noise_orders.iter().enumerate() {
        let result = service.execute_round(true_value, noise);
        total_profit += result.informed_profit;
        total_shares += result.informed_order;

        println!(
            "  {}    {:6.1}    {:6.1}    {:6.1}   {:6.2}   {:+5.2}    {:7.2}",
            round + 1,
            result.informed_order,
            noise,
            result.total_order_flow,
            result.execution_price,
            result.price_impact,
            total_profit
        );
    }

    println!("\n【结果分析】");
    println!("  ✓ 总买入股数：{:.0} 股", total_shares);
    println!("  ✓ 最终价格：{:.2} 元", service.state().current_price);
    println!(
        "  ✓ 价格上涨：{:.2} 元",
        service.state().price_change()
    );
    println!("  ✓ 总利润：{:.2} 元", total_profit);
    println!(
        "  ✓ 平均成本：{:.2} 元/股\n",
        (service.state().current_price + 100.0) / 2.0
    );

    println!("【如果一次性买入会怎样？】");
    let naive_impact = service.lambda() * total_shares;
    let naive_cost = 100.0 + naive_impact / 2.0;
    println!("  ❌ 价格会直接涨 {:.2} 元", naive_impact);
    println!("  ❌ 平均成本约 {:.2} 元/股", naive_cost);
    println!("  ❌ 利润损失巨大！\n");

    pause();

    // ============================================================
    // 示例 3：做市商如何学习和定价
    // ============================================================
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  示例 3：做市商的学习过程                              ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("【做市商的视角】");
    println!("做市商不知道真实价值，只能观察订单流来猜测：\n");

    let params_mm = KyleParameters::new(15.0, 8.0, 100.0, 10);
    let mut service_mm = KyleModelService::new(params_mm);

    let true_value_mm = 118.0;
    let noise_orders_mm = vec![5.0, -2.0, 8.0, 3.0, -1.0, 6.0, 2.0, -3.0, 4.0, 5.0];

    println!("【做市商逐步学习】");
    println!("轮次  累积订单流  做市商估值  估值误差  当前价格");
    println!("──────────────────────────────────────────────");

    for (round, &noise) in noise_orders_mm.iter().enumerate() {
        service_mm.execute_round(true_value_mm, noise);

        let estimated = service_mm.estimate_true_value();
        let error = (estimated - true_value_mm).abs();
        let cumulative_flow = service_mm.state().cumulative_order_flow;
        let current_price = service_mm.state().current_price;

        println!(
            "  {}     {:+7.1}     {:6.2}     {:6.2}    {:6.2}",
            round + 1,
            cumulative_flow,
            estimated,
            error,
            current_price
        );
    }

    println!("\n【结论】");
    println!("  ✓ 做市商通过累积订单流逐步接近真实价值");
    println!("  ✓ 持续的单边买入 → 做市商察觉有内幕 → 涨价");
    println!("  ✓ 知情交易者需要在噪音中隐藏自己的意图\n");

    pause();

    // ============================================================
    // 示例 4：不同策略的盈亏对比
    // ============================================================
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  示例 4：不同交易策略的盈亏对比                        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("【场景】想买 10,000 股，真实价值 120 元，当前价 100 元\n");

    // 策略 1：一次性买入
    println!("【策略 1：新手做法 - 一次性全买】");
    let params_naive = KyleParameters::new(20.0, 10.0, 100.0, 1);
    let lambda = params_naive.price_impact();
    let total_shares = 10000.0;
    let price_impact_naive = lambda * total_shares;
    let avg_price_naive = 100.0 + price_impact_naive / 2.0;
    let profit_naive = total_shares * (120.0 - avg_price_naive);

    println!("  • 价格冲击：{:.2} 元", price_impact_naive);
    println!("  • 平均成本：{:.2} 元/股", avg_price_naive);
    println!("  • 总利润：{:.2} 元", profit_naive);
    println!(
        "  • 每股利润：{:.2} 元\n",
        profit_naive / total_shares
    );

    // 策略 2：Kyle 最优策略（分10轮）
    println!("【策略 2：Kyle 最优策略 - 分 10 轮执行】");
    let params_kyle = KyleParameters::new(20.0, 10.0, 100.0, 10);
    let mut service_kyle = KyleModelService::new(params_kyle);

    // 模拟噪音
    let noise_kyle: Vec<f64> = (0..10).map(|i| (i as f64 - 5.0) * 50.0).collect();

    service_kyle.simulate(120.0, &noise_kyle);

    let total_shares_kyle = service_kyle.state().informed_position;
    let avg_price_kyle =
        (100.0 + service_kyle.state().current_price) / 2.0;
    let profit_kyle = total_shares_kyle * (120.0 - avg_price_kyle);

    println!("  • 实际买入：{:.0} 股", total_shares_kyle);
    println!("  • 平均成本：{:.2} 元/股", avg_price_kyle);
    println!("  • 总利润：{:.2} 元", profit_kyle);
    println!("  • 每股利润：{:.2} 元\n", profit_kyle / total_shares_kyle);

    println!("【对比总结】");
    let profit_diff = profit_kyle - profit_naive;
    let improvement = (profit_diff / profit_naive * 100.0).abs();

    if profit_kyle > profit_naive {
        println!("  ✅ Kyle 策略比新手策略多赚：{:.2} 元", profit_diff);
        println!("  ✅ 利润提升：{:.1}%", improvement);
    } else {
        println!("  ⚠️  本次模拟中新手策略更优（噪音交易影响）");
    }
    println!("  💡 实际交易中，Kyle 策略能稳定降低市场冲击成本\n");

    pause();

    // ============================================================
    // 实用建议
    // ============================================================
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║  实际应用建议                                           ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");

    println!("【Kyle 模型的实际用途】\n");

    println!("1️⃣  【大单执行优化】");
    println!("   适合：机构投资者、基金经理");
    println!("   应用：将大单分拆，降低市场冲击成本");
    println!("   工具：VWAP、TWAP 算法都基于类似原理\n");

    println!("2️⃣  【做市商定价】");
    println!("   适合：做市商、高频交易商");
    println!("   应用：根据订单流动态调整买卖价差");
    println!("   指标：λ 越大 → 逆向选择风险越高 → 价差越宽\n");

    println!("3️⃣  【交易成本估算】");
    println!("   适合：交易前分析");
    println!("   应用：预估大单的市场冲击成本");
    println!("   公式：冲击成本 ≈ λ × 订单量 / 2\n");

    println!("4️⃣  【内幕交易检测】");
    println!("   适合：监管机构、合规部门");
    println!("   应用：检测异常订单流模式");
    println!("   信号：持续单边订单流 + 价格持续偏离\n");

    println!("5️⃣  【流动性分析】");
    println!("   适合：量化研究");
    println!("   应用：评估市场深度和流动性");
    println!("   指标：市场深度 = 1/λ（越大越好）\n");

    println!("【参数如何选择？】\n");
    println!("  • σ_v（价值波动）：用历史价格标准差估算");
    println!("  • σ_u（噪音波动）：用历史订单流标准差估算");
    println!("  • λ (价格影响)：用订单流对价格的回归系数");
    println!("  • 交易轮数 T：根据执行时间窗口确定\n");

    println!("【注意事项】\n");
    println!("  ⚠️  Kyle 模型假设市场有效，实际可能有延迟");
    println!("  ⚠️  极端行情下，价格影响可能非线性");
    println!("  ⚠️  需要结合实际市场微观结构调整参数");
    println!("  ⚠️  监管禁止利用内幕信息交易\n");

    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║             示例演示完成！                              ║");
    println!("║  现在你应该理解 Kyle 模型如何帮助优化交易执行了        ║");
    println!("╚══════════════════════════════════════════════════════════╝\n");
}

/// 暂停等待用户按回车继续
fn pause() {
    use std::io::{self, BufRead};
    println!("─────────────────────────────────────────────");
    println!("按 Enter 继续...");
    println!("─────────────────────────────────────────────\n");
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
}
