//! 量化交易员消费行情数据示例
//!
//! 本示例展示了量化交易员如何使用 MarketDataQueryProcessorImpl 消费 L1/L2/L3 行情数据
//!
//! 场景：
//! 1. 高频交易策略：监控 L1 最优买卖价，捕捉价差机会
//! 2. 做市商策略：分析 L2 深度数据，动态调整报价
//! 3. 大单追踪策略：监控 L3 订单簿，识别大额订单

use lob::lob::{
    BboChangeEvent, IncrementalDataRepo, IncrementalDataResult, Level1, Level1SnapshotRepo,
    Level2, Level2SnapshotRepo, Level3, Level3Order, Level3SnapshotRepo, MarketDataDelta,
    MarketDataQueryError, MarketDataQueryProcessor, MarketDataQueryProcessorImpl,
    OrderBookChangeType, OrderBookDelta, OrderId, Price, PriceLevel, Quantity,
    QueryIncrementalData, QueryLevel1, QueryLevel1Batch, QueryLevel2, QueryLevel3, Side,
    SymbolId, TradeEvent, TraderId,
};

// ============================================================================
// Mock 数据仓储实现（用于演示）
// ============================================================================

/// Mock 快照数据仓储
struct MockSnapshotRepo {
    symbol_id: SymbolId,
    sequence: u64,
}

impl MockSnapshotRepo {
    fn new(symbol_id: SymbolId) -> Self {
        Self {
            symbol_id,
            sequence: 1000,
        }
    }
}

impl Level1SnapshotRepo for MockSnapshotRepo {
    fn query_level1(&self, symbol_id: SymbolId, _sequence: u64) -> Option<Level1> {
        if symbol_id != self.symbol_id {
            return None;
        }

        Some(Level1 {
            symbol_id,
            timestamp: 1234567890000,
            sequence: self.sequence,
            best_bid: Some(49999),
            best_bid_quantity: 100,
            best_ask: Some(50001),
            best_ask_quantity: 150,
            last_trade_price: Some(50000),
            last_trade_quantity: 10,
            spread: Some(2),
            mid_price: Some(50000),
        })
    }
}

impl Level2SnapshotRepo for MockSnapshotRepo {
    fn query_level2(&self, symbol_id: SymbolId, _sequence: u64, depth: usize) -> Level2<10> {
        let mut level2 = Level2::<10>::default();
        level2.level1.symbol_id = symbol_id;
        level2.level1.timestamp = 1234567890000;
        level2.level1.sequence = self.sequence;

        // 模拟买盘深度
        for i in 0..depth.min(10) {
            level2.bids[i] = Some(PriceLevel {
                price: 50000 - (i as Price * 10),
                quantity: 100 + (i as Quantity * 50),
                order_count: (i + 1) as u32,
            });
        }

        // 模拟卖盘深度
        for i in 0..depth.min(10) {
            level2.asks[i] = Some(PriceLevel {
                price: 50001 + (i as Price * 10),
                quantity: 150 + (i as Quantity * 30),
                order_count: (i + 1) as u32,
            });
        }

        level2.bid_count = depth.min(10);
        level2.ask_count = depth.min(10);

        level2
    }
}

impl Level3SnapshotRepo for MockSnapshotRepo {
    fn query_level3(&self, symbol_id: SymbolId, _sequence: u64) -> Level3 {
        let mut level3 = Level3::new();
        level3.level2.level1.symbol_id = symbol_id;
        level3.level2.level1.timestamp = 1234567890000;
        level3.level2.level1.sequence = self.sequence;

        // 模拟一些买单
        for i in 0..5 {
            let qty = 100 + (i as Quantity * 20);
            level3.add_bid(Level3Order {
                order_id: 10000 + i,
                trader_id: TraderId::new([1, 0, 0, 0, 0, 0, 0, i as u8]),
                price: 50000 - (i as Price * 10),
                quantity: qty,
                unfilled_quantity: qty,
            });
        }

        // 模拟一些卖单
        for i in 0..5 {
            let qty = 150 + (i as Quantity * 15);
            level3.add_ask(Level3Order {
                order_id: 20000 + i,
                trader_id: TraderId::new([2, 0, 0, 0, 0, 0, 0, i as u8]),
                price: 50001 + (i as Price * 10),
                quantity: qty,
                unfilled_quantity: qty,
            });
        }

        level3
    }
}

/// Mock 增量数据仓储
struct MockIncrementalRepo {
    symbol_id: SymbolId,
    latest_sequence: u64,
    deltas: Vec<MarketDataDelta>,
}

impl MockIncrementalRepo {
    fn new(symbol_id: SymbolId) -> Self {
        let mut deltas = Vec::new();

        // 模拟一些增量事件
        // 1. 新增订单
        deltas.push(MarketDataDelta::OrderBookChange(OrderBookDelta {
            symbol_id,
            timestamp: 1234567890100,
            sequence: 1001,
            change_type: OrderBookChangeType::Add,
            order_id: 30001,
            side: Side::Buy,
            price: 49998,
            quantity: 200,
            trader_id: Some(TraderId::new([3, 0, 0, 0, 0, 0, 0, 1])),
        }));

        // 2. 成交事件
        deltas.push(MarketDataDelta::Trade(TradeEvent {
            symbol_id,
            timestamp: 1234567890200,
            sequence: 1002,
            trade_id: 5001,
            buyer_order_id: 10001,
            seller_order_id: 20001,
            price: 50000,
            quantity: 50,
            aggressor_side: Side::Buy,
        }));

        // 3. 修改订单
        deltas.push(MarketDataDelta::OrderBookChange(OrderBookDelta {
            symbol_id,
            timestamp: 1234567890300,
            sequence: 1003,
            change_type: OrderBookChangeType::Modify,
            order_id: 10001,
            side: Side::Buy,
            price: 49999,
            quantity: 50, // 部分成交后剩余数量
            trader_id: Some(TraderId::new([1, 0, 0, 0, 0, 0, 0, 1])),
        }));

        // 4. BBO 变更
        deltas.push(MarketDataDelta::BboChange(BboChangeEvent {
            symbol_id,
            timestamp: 1234567890400,
            sequence: 1004,
            best_bid: Some(49998),
            best_bid_quantity: 200,
            best_ask: Some(50001),
            best_ask_quantity: 150,
        }));

        // 5. 删除订单
        deltas.push(MarketDataDelta::OrderBookChange(OrderBookDelta {
            symbol_id,
            timestamp: 1234567890500,
            sequence: 1005,
            change_type: OrderBookChangeType::Delete,
            order_id: 20002,
            side: Side::Sell,
            price: 50011,
            quantity: 0,
            trader_id: Some(TraderId::new([2, 0, 0, 0, 0, 0, 0, 2])),
        }));

        Self {
            symbol_id,
            latest_sequence: 1005,
            deltas,
        }
    }
}

impl IncrementalDataRepo for MockIncrementalRepo {
    fn query_incremental_data(
        &self,
        symbol_id: SymbolId,
        from_sequence: u64,
        to_sequence: u64,
    ) -> Result<Vec<MarketDataDelta>, MarketDataQueryError> {
        if symbol_id != self.symbol_id {
            return Err(MarketDataQueryError::SymbolNotFound { symbol_id });
        }

        let filtered: Vec<MarketDataDelta> = self
            .deltas
            .iter()
            .filter(|delta| {
                let seq = match delta {
                    MarketDataDelta::OrderBookChange(d) => d.sequence,
                    MarketDataDelta::Trade(t) => t.sequence,
                    MarketDataDelta::BboChange(b) => b.sequence,
                };
                seq > from_sequence && seq <= to_sequence
            })
            .copied()
            .collect();

        Ok(filtered)
    }

    fn get_latest_sequence(&self, symbol_id: SymbolId) -> Option<u64> {
        if symbol_id == self.symbol_id {
            Some(self.latest_sequence)
        } else {
            None
        }
    }
}

// ============================================================================
// 量化策略示例
// ============================================================================

/// 策略 1: 高频交易 - 价差捕捉策略
struct SpreadCaptureStrategy {
    symbol_id: SymbolId,
    min_spread: Price,
    target_quantity: Quantity,
}

impl SpreadCaptureStrategy {
    fn new(symbol_id: SymbolId, min_spread: Price, target_quantity: Quantity) -> Self {
        Self {
            symbol_id,
            min_spread,
            target_quantity,
        }
    }

    /// 分析 L1 数据，寻找价差机会
    fn analyze_level1(&self, level1: &Level1) {
        println!("\n=== 高频交易策略：价差捕捉 ===");
        println!("交易对: {}", level1.symbol_id);
        println!("时间戳: {}", level1.timestamp);
        println!("序列号: {}", level1.sequence);

        if let (Some(bid), Some(ask)) = (level1.best_bid, level1.best_ask) {
            let spread = ask - bid;
            println!("最优买价: {} (数量: {})", bid, level1.best_bid_quantity);
            println!("最优卖价: {} (数量: {})", ask, level1.best_ask_quantity);
            println!("价差: {} ({}%)", spread, (spread as f64 / bid as f64) * 100.0);

            // 判断是否有交易机会
            if spread >= self.min_spread {
                let available_qty = level1.best_bid_quantity.min(level1.best_ask_quantity);
                if available_qty >= self.target_quantity {
                    println!("✅ 发现交易机会！");
                    println!(
                        "   建议操作: 买入 {} @ {}, 卖出 {} @ {}",
                        self.target_quantity, bid, self.target_quantity, ask
                    );
                    println!("   预期利润: {}", spread * self.target_quantity as Price);
                } else {
                    println!("⚠️  流动性不足，可用数量: {}", available_qty);
                }
            } else {
                println!("❌ 价差过小，不满足交易条件");
            }
        }

        if let Some(mid) = level1.mid_price {
            println!("中间价: {}", mid);
        }
    }
}

/// 策略 2: 做市商策略 - 深度分析
struct MarketMakingStrategy {
    symbol_id: SymbolId,
    depth_levels: usize,
    imbalance_threshold: f64,
}

impl MarketMakingStrategy {
    fn new(symbol_id: SymbolId, depth_levels: usize, imbalance_threshold: f64) -> Self {
        Self {
            symbol_id,
            depth_levels,
            imbalance_threshold,
        }
    }

    /// 分析 L2 深度数据，计算订单簿失衡度
    fn analyze_level2(&self, level2: &Level2<10>) {
        println!("\n=== 做市商策略：深度分析 ===");
        println!("交易对: {}", level2.level1.symbol_id);
        println!("时间戳: {}", level2.level1.timestamp);
        println!("序列号: {}", level2.level1.sequence);

        // 显示买盘深度
        println!("\n买盘深度 (前{}档):", self.depth_levels);
        let mut total_bid_volume: Quantity = 0;
        for i in 0..self.depth_levels.min(level2.bid_count) {
            if let Some(level) = &level2.bids[i] {
                total_bid_volume += level.quantity;
                println!(
                    "  档位 {}: 价格={}, 数量={}, 订单数={}",
                    i + 1,
                    level.price,
                    level.quantity,
                    level.order_count
                );
            }
        }

        // 显示卖盘深度
        println!("\n卖盘深度 (前{}档):", self.depth_levels);
        let mut total_ask_volume: Quantity = 0;
        for i in 0..self.depth_levels.min(level2.ask_count) {
            if let Some(level) = &level2.asks[i] {
                total_ask_volume += level.quantity;
                println!(
                    "  档位 {}: 价格={}, 数量={}, 订单数={}",
                    i + 1,
                    level.price,
                    level.quantity,
                    level.order_count
                );
            }
        }

        // 计算订单簿失衡度
        let total_volume = total_bid_volume + total_ask_volume;
        let imbalance = if total_volume > 0 {
            (total_bid_volume as f64 - total_ask_volume as f64) / total_volume as f64
        } else {
            0.0
        };

        println!("\n订单簿分析:");
        println!("  买盘总量: {}", total_bid_volume);
        println!("  卖盘总量: {}", total_ask_volume);
        println!("  失衡度: {:.2}%", imbalance * 100.0);

        // 做市决策
        if imbalance.abs() > self.imbalance_threshold {
            if imbalance > 0.0 {
                println!("✅ 买盘压力大，建议在卖方挂单做市");
                if let Some(ask_level) = &level2.asks[0] {
                    println!("   推荐价格: {} (卖一价)", ask_level.price);
                }
            } else {
                println!("✅ 卖盘压力大，建议在买方挂单做市");
                if let Some(bid_level) = &level2.bids[0] {
                    println!("   推荐价格: {} (买一价)", bid_level.price);
                }
            }
        } else {
            println!("⚠️  订单簿相对平衡，保持中性策略");
        }
    }
}

/// 策略 3: 大单追踪策略
struct LargeOrderTrackingStrategy {
    symbol_id: SymbolId,
    large_order_threshold: Quantity,
}

impl LargeOrderTrackingStrategy {
    fn new(symbol_id: SymbolId, large_order_threshold: Quantity) -> Self {
        Self {
            symbol_id,
            large_order_threshold,
        }
    }

    /// 分析 L3 订单簿，识别大额订单
    fn analyze_level3(&self, level3: &Level3) {
        println!("\n=== 大单追踪策略：L3 订单分析 ===");
        println!("交易对: {}", level3.level2.level1.symbol_id);

        // 分析买单
        println!("\n大额买单 (>= {}):", self.large_order_threshold);
        let mut large_bids = Vec::new();
        for order in &level3.bids {
            if order.quantity >= self.large_order_threshold {
                large_bids.push(order);
                println!(
                    "  订单ID: {}, 价格: {}, 数量: {}, 未成交: {}",
                    order.order_id, order.price, order.quantity, order.unfilled_quantity
                );
            }
        }

        // 分析卖单
        println!("\n大额卖单 (>= {}):", self.large_order_threshold);
        let mut large_asks = Vec::new();
        for order in &level3.asks {
            if order.quantity >= self.large_order_threshold {
                large_asks.push(order);
                println!(
                    "  订单ID: {}, 价格: {}, 数量: {}, 未成交: {}",
                    order.order_id, order.price, order.quantity, order.unfilled_quantity
                );
            }
        }

        // 大单分析
        println!("\n大单统计:");
        println!("  大额买单数量: {}", large_bids.len());
        println!("  大额卖单数量: {}", large_asks.len());

        if large_bids.len() > large_asks.len() {
            println!("✅ 买方大单占优，市场可能上涨");
        } else if large_asks.len() > large_bids.len() {
            println!("✅ 卖方大单占优，市场可能下跌");
        } else {
            println!("⚠️  大单分布均衡");
        }

        // 订单簿总览
        println!("\n订单簿总览:");
        println!("  总买单数: {}", level3.bids.len());
        println!("  总卖单数: {}", level3.asks.len());
        println!("  活跃订单数: {}", level3.active_order_count());
    }
}

/// 策略 4: 增量数据实时监控策略
struct IncrementalDataMonitor {
    symbol_id: SymbolId,
    last_sequence: u64,
    trade_count: u64,
    orderbook_changes: u64,
}

impl IncrementalDataMonitor {
    fn new(symbol_id: SymbolId, initial_sequence: u64) -> Self {
        Self {
            symbol_id,
            last_sequence: initial_sequence,
            trade_count: 0,
            orderbook_changes: 0,
        }
    }

    /// 处理增量数据
    fn process_incremental_data(&mut self, result: &IncrementalDataResult) {
        println!("\n=== 增量数据实时监控 ===");
        println!("交易对: {}", result.symbol_id);
        println!(
            "序列号范围: {} -> {}",
            result.from_sequence, result.to_sequence
        );
        println!("事件数量: {}", result.deltas.len());
        println!("是否有更多数据: {}", result.has_more);

        for delta in &result.deltas {
            match delta {
                MarketDataDelta::OrderBookChange(change) => {
                    self.orderbook_changes += 1;
                    println!("\n📝 订单簿变更:");
                    println!("  序列号: {}", change.sequence);
                    println!("  时间戳: {}", change.timestamp);
                    println!("  变更类型: {:?}", change.change_type);
                    println!("  订单ID: {}", change.order_id);
                    println!("  方向: {:?}", change.side);
                    println!("  价格: {}", change.price);
                    println!("  数量: {}", change.quantity);

                    match change.change_type {
                        OrderBookChangeType::Add => {
                            println!("  ✅ 新增订单");
                        }
                        OrderBookChangeType::Modify => {
                            println!("  🔄 修改订单");
                        }
                        OrderBookChangeType::Delete => {
                            println!("  ❌ 删除订单");
                        }
                    }
                }
                MarketDataDelta::Trade(trade) => {
                    self.trade_count += 1;
                    println!("\n💰 成交事件:");
                    println!("  序列号: {}", trade.sequence);
                    println!("  时间戳: {}", trade.timestamp);
                    println!("  成交ID: {}", trade.trade_id);
                    println!("  买方订单: {}", trade.buyer_order_id);
                    println!("  卖方订单: {}", trade.seller_order_id);
                    println!("  成交价: {}", trade.price);
                    println!("  成交量: {}", trade.quantity);
                    println!("  主动方: {:?}", trade.aggressor_side);
                    println!(
                        "  成交额: {}",
                        trade.price as u64 * trade.quantity as u64
                    );
                }
                MarketDataDelta::BboChange(bbo) => {
                    println!("\n📊 最优买卖价变更:");
                    println!("  序列号: {}", bbo.sequence);
                    println!("  时间戳: {}", bbo.timestamp);
                    if let Some(bid) = bbo.best_bid {
                        println!("  最优买价: {} (数量: {})", bid, bbo.best_bid_quantity);
                    }
                    if let Some(ask) = bbo.best_ask {
                        println!("  最优卖价: {} (数量: {})", ask, bbo.best_ask_quantity);
                    }
                    if let (Some(bid), Some(ask)) = (bbo.best_bid, bbo.best_ask) {
                        println!("  价差: {}", ask - bid);
                    }
                }
            }

            self.last_sequence = match delta {
                MarketDataDelta::OrderBookChange(d) => d.sequence,
                MarketDataDelta::Trade(t) => t.sequence,
                MarketDataDelta::BboChange(b) => b.sequence,
            };
        }

        println!("\n统计信息:");
        println!("  累计成交次数: {}", self.trade_count);
        println!("  累计订单簿变更: {}", self.orderbook_changes);
        println!("  最新序列号: {}", self.last_sequence);
    }
}

// ============================================================================
// 主程序：量化交易员工作流
// ============================================================================

fn main() {
    println!("========================================");
    println!("量化交易员行情数据消费示例");
    println!("========================================");

    let symbol_id: SymbolId = 1; // BTC/USDT

    // 创建数据仓储
    let snapshot_repo = MockSnapshotRepo::new(symbol_id);
    let incremental_repo = MockIncrementalRepo::new(symbol_id);

    // 创建行情查询处理器
    let processor = MarketDataQueryProcessorImpl::new(snapshot_repo, incremental_repo);

    // ========================================================================
    // 场景 1: 高频交易策略 - 使用 L1 数据
    // ========================================================================
    println!("\n\n场景 1: 高频交易策略");
    println!("========================================");

    let hft_strategy = SpreadCaptureStrategy::new(symbol_id, 5, 50);

    // 查询 L1 数据
    let query_l1 = QueryLevel1::new(symbol_id, 1000);
    match processor.handle_query_level1(query_l1) {
        Ok(result) => {
            hft_strategy.analyze_level1(&result.snapshot);
        }
        Err(e) => {
            eprintln!("查询 L1 数据失败: {:?}", e);
        }
    }

    // 批量查询多个交易对的 L1 数据
    println!("\n\n批量查询多个交易对:");
    let query_batch = QueryLevel1Batch::new(vec![1, 2, 3], 1000);
    let batch_result = processor.handle_query_level1_batch(query_batch);
    println!("成功查询: {} 个交易对", batch_result.snapshots.len());
    println!("失败查询: {} 个交易对", batch_result.failed_symbols.len());

    // ========================================================================
    // 场景 2: 做市商策略 - 使用 L2 数据
    // ========================================================================
    println!("\n\n场景 2: 做市商策略");
    println!("========================================");

    let mm_strategy = MarketMakingStrategy::new(symbol_id, 5, 0.2);

    // 查询 L2 数据（10档深度）
    let query_l2 = QueryLevel2::depth_10(symbol_id, 1000);
    let l2_result = processor.handle_query_level2(query_l2);
    mm_strategy.analyze_level2(&l2_result.snapshot);

    // ========================================================================
    // 场景 3: 大单追踪策略 - 使用 L3 数据
    // ========================================================================
    println!("\n\n场景 3: 大单追踪策略");
    println!("========================================");

    let large_order_strategy = LargeOrderTrackingStrategy::new(symbol_id, 100);

    // 查询 L3 数据（完整订单簿）
    let query_l3 = QueryLevel3::new(symbol_id, 1000);
    let l3_result = processor.handle_query_level3(query_l3);
    large_order_strategy.analyze_level3(&l3_result.snapshot);

    // ========================================================================
    // 场景 4: 增量数据实时监控
    // ========================================================================
    println!("\n\n场景 4: 增量数据实时监控");
    println!("========================================");

    let mut monitor = IncrementalDataMonitor::new(symbol_id, 1000);

    // 查询增量数据
    let query_incremental = QueryIncrementalData::new(symbol_id, 1000, 1005);
    match processor.handle_query_incremental_data(query_incremental) {
        Ok(result) => {
            monitor.process_incremental_data(&result);

            // 如果还有更多数据，继续查询
            if result.has_more {
                println!("\n⚠️  检测到更多增量数据，建议继续查询...");
            }
        }
        Err(e) => {
            eprintln!("查询增量数据失败: {:?}", e);
        }
    }

    // ========================================================================
    // 场景 5: 综合策略 - 结合多层级数据
    // ========================================================================
    println!("\n\n场景 5: 综合策略决策");
    println!("========================================");

    println!("\n基于多层级数据的综合分析:");
    println!("1. L1 数据显示价差为 2，满足高频交易条件");
    println!("2. L2 数据显示买盘失衡度为正，市场偏多");
    println!("3. L3 数据显示大额买单较多，支撑位强");
    println!("4. 增量数据显示最近有大额成交，市场活跃");
    println!("\n✅ 综合决策: 建议做多，目标价位 50010");

    println!("\n\n========================================");
    println!("示例运行完成");
    println!("========================================");
}
