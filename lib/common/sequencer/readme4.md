下面是一个简化的 Lighter 交易系统核心模块的 Rust 实现，包含排序器模块和撮合引擎的基本架构：

🏗️ 系统核心数据结构

// 订单类型和方向
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
Market,  // 市价单
Limit,   // 限价单
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
Buy,
Sell,
}

// 订单状态
#[derive(Debug, Clone)]
pub enum OrderStatus {
Pending,        // 待处理
PartiallyFilled, // 部分成交
Filled,         // 完全成交
Cancelled,      // 已取消
}

// 交易订单结构
#[derive(Debug, Clone)]
pub struct Order {
pub id: u64,
pub user_id: u64,
pub side: OrderSide,
pub order_type: OrderType,
pub price: f64,           // 限价单价格，市价单为0.0
pub quantity: u64,
pub filled_quantity: u64,
pub status: OrderStatus,
pub timestamp: i64,
pub leverage: u8,
}


⚙️ 排序器模块 (Sequencer)

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

pub struct Sequencer {
// 内存池管理订单对象
order_pool: Arc<RwLock<OrderPool>>,
// 买单队列 (价格降序排列)
buy_orders: Arc<RwLock<VecDeque<Order>>>,
// 卖单队列 (价格升序排列)
sell_orders: Arc<RwLock<VecDeque<Order>>>,
// 用户账户状态
accounts: Arc<RwLock<HashMap<u64, Account>>>,
}

impl Sequencer {
pub fn new(initial_capacity: usize) -> Self {
Self {
order_pool: Arc::new(RwLock::new(OrderPool::new(initial_capacity))),
buy_orders: Arc::new(RwLock::new(VecDeque::with_capacity(initial_capacity))),
sell_orders: Arc::new(RwLock::new(VecDeque::with_capacity(initial_capacity))),
accounts: Arc::new(RwLock::new(HashMap::new())),
}
}

    /// 提交新订单 - 核心入口函数
    pub fn submit_order(&self, order_data: OrderData) -> Result<Order, String> {
        // 1. 验证订单基本格式
        self.validate_order_format(&order_data)?;
        
        // 2. 保证金检查
        self.check_margin_requirement(&order_data)?;
        
        // 3. 从内存池获取订单对象
        let mut order = self.acquire_order_from_pool()?;
        
        // 4. 初始化订单数据
        self.initialize_order(&mut order, order_data);
        
        // 5. 根据订单类型和方向处理
        match order.order_type {
            OrderType::Market => self.handle_market_order(order),
            OrderType::Limit => self.handle_limit_order(order),
        }
    }
    
    /// 订单格式验证
    fn validate_order_format(&self, order_data: &OrderData) -> Result<(), String> {
        if order_data.quantity == 0 {
            return Err("订单数量必须大于零".to_string());
        }
        
        if order_data.leverage < 1 || order_data.leverage > 100 {
            return Err("杠杆倍数必须在1-100之间".to_string());
        }
        
        if order_data.order_type == OrderType::Limit && order_data.price <= 0.0 {
            return Err("限价单价格必须大于零".to_string());
        }
        
        Ok(())
    }
    
    /// 保证金要求检查 - 风险控制核心
    fn check_margin_requirement(&self, order_data: &OrderData) -> Result<(), String> {
        let accounts = self.accounts.read().map_err(|_| "无法读取账户信息")?;
        let account = accounts.get(&order_data.user_id)
            .ok_or("用户账户不存在")?;
        
        let required_margin = self.calculate_initial_margin(order_data);
        
        if account.available_balance < required_margin {
            return Err(format!(
                "保证金不足: 可用 {:.2}, 需要 {:.2}", 
                account.available_balance, required_margin
            ));
        }
        
        Ok(())
    }
    
    /// 计算开仓所需初始保证金
    fn calculate_initial_margin(&self, order_data: &OrderData) -> f64 {
        let order_value = order_data.price * order_data.quantity as f64;
        let margin = order_value / order_data.leverage as f64;
        
        // 添加风险准备金 (2%)
        margin * 1.02
    }
    
    /// 处理市价单
    fn handle_market_order(&self, order: Order) -> Result<Order, String> {
        // 市价单直接尝试立即撮合
        let matching_result = self.attempt_immediate_matching(order.clone());
        
        match matching_result {
            Ok(trades) => {
                if trades.iter().map(|t| t.quantity).sum::<u64>() == order.quantity {
                    // 完全成交
                    Ok(order)
                } else {
                    // 部分成交或无法完全成交
                    Err("市价单无法立即完全成交".to_string())
                }
            }
            Err(e) => Err(format!("市价单撮合失败: {}", e)),
        }
    }
    
    /// 处理限价单  
    fn handle_limit_order(&self, mut order: Order) -> Result<Order, String> {
        // 冻结保证金
        self.freeze_margin(order.user_id, order.quantity, order.price)?;
        
        // 根据买卖方向放入不同队列
        match order.side {
            OrderSide::Buy => self.add_buy_order(order),
            OrderSide::Sell => self.add_sell_order(order),
        }
    }
    
    /// 添加买单到订单簿 (按价格降序排列)
    fn add_buy_order(&self, order: Order) -> Result<Order, String> {
        let mut buys = self.buy_orders.write().map_err(|_| "无法获取买单队列写锁")?;
        
        // 找到插入位置 (价格优先，时间优先)
        let pos = buys.iter()
            .position(|o| o.price < order.price || 
                      (o.price == order.price && o.timestamp > order.timestamp))
            .unwrap_or(buys.len());
            
        buys.insert(pos, order.clone());
        Ok(order)
    }
    
    /// 添加卖单到订单簿 (按价格升序排列)
    fn add_sell_order(&self, order: Order) -> Result<Order, String> {
        let mut sells = self.sell_orders.write().map_err(|_| "无法获取卖单队列写锁")?;
        
        let pos = sells.iter()
            .position(|o| o.price > order.price || 
                      (o.price == order.price && o.timestamp > order.timestamp))
            .unwrap_or(sells.len());
            
        sells.insert(pos, order.clone());
        Ok(order)
    }
}


🔄 撮合引擎模块 (Matching Engine)

pub struct MatchingEngine {
sequencer: Arc<Sequencer>,
trade_history: Arc<RwLock<Vec<Trade>>>,
}

impl MatchingEngine {
pub fn new(sequencer: Arc<Sequencer>) -> Self {
Self {
sequencer,
trade_history: Arc::new(RwLock::new(Vec::new())),
}
}

    /// 撮合引擎主循环
    pub fn run_matching_loop(&self) {
        loop {
            if let Err(e) = self.matching_iteration() {
                eprintln!("撮合迭代错误: {}", e);
            }
            
            // 短暂休眠避免空转
            std::thread::sleep(std::time::Duration::from_micros(100));
        }
    }
    
    /// 单次撮合迭代
    fn matching_iteration(&self) -> Result<(), String> {
        let (best_bid, best_ask) = self.get_best_prices()?;
        
        // 检查是否可撮合 (买一价 >= 卖一价)
        if best_bid >= best_ask {
            self.execute_match()?;
        }
        
        Ok(())
    }
    
    /// 获取最优买卖价格
    fn get_best_prices(&self) -> Result<(f64, f64), String> {
        let buys = self.sequencer.buy_orders.read()
            .map_err(|_| "无法读取买单队列")?;
        let sells = self.sequencer.sell_orders.read()
            .map_err(|_| "无法读取卖单队列")?;
        
        let best_bid = buys.front().map(|o| o.price).unwrap_or(0.0);
        let best_ask = sells.front().map(|o| o.price).unwrap_or(f64::MAX);
        
        Ok((best_bid, best_ask))
    }
    
    /// 执行订单匹配
    fn execute_match(&self) -> Result<(), String> {
        let mut buys = self.sequencer.buy_orders.write()
            .map_err(|_| "无法获取买单队列写锁")?;
        let mut sells = self.sequencer.sell_orders.write()
            .map_err(|_| "无法获取卖单队列写锁")?;
        
        if let (Some(buy_order), Some(sell_order)) = (buys.front_mut(), sells.front_mut()) {
            // 确定成交价 (使用先进入订单簿的一方价格)
            let trade_price = if buy_order.timestamp <= sell_order.timestamp {
                buy_order.price
            } else {
                sell_order.price
            };
            
            // 计算可成交数量
            let trade_quantity = std::cmp::min(
                buy_order.quantity - buy_order.filled_quantity,
                sell_order.quantity - sell_order.filled_quantity
            );
            
            if trade_quantity > 0 {
                // 创建成交记录
                let trade = Trade {
                    id: self.generate_trade_id(),
                    buyer_order_id: buy_order.id,
                    seller_order_id: sell_order.id,
                    price: trade_price,
                    quantity: trade_quantity,
                    timestamp: chrono::Utc::now().timestamp_millis(),
                };
                
                // 更新订单状态
                self.update_order_status(&mut buy_order, trade_quantity);
                self.update_order_status(&mut sell_order, trade_quantity);
                
                // 保存成交记录
                self.record_trade(trade)?;
                
                // 移除完全成交的订单
                if buy_order.filled_quantity == buy_order.quantity {
                    buys.pop_front();
                }
                if sell_order.filled_quantity == sell_order.quantity {
                    sells.pop_front();
                }
                
                println!("成交: {} @ {:.2}", trade_quantity, trade_price);
            }
        }
        
        Ok(())
    }
    
    /// 更新订单状态
    fn update_order_status(&self, order: &mut Order, trade_quantity: u64) {
        order.filled_quantity += trade_quantity;
        
        if order.filled_quantity == order.quantity {
            order.status = OrderStatus::Filled;
        } else if order.filled_quantity > 0 {
            order.status = OrderStatus::PartiallyFilled;
        }
    }
}


💾 内存池和辅助结构

// 内存池用于高效管理订单对象
struct OrderPool {
free_objects: VecDeque<Order>,
max_size: usize,
}

impl OrderPool {
fn new(initial_capacity: usize) -> Self {
let mut pool = VecDeque::with_capacity(initial_capacity);

        // 预分配对象
        for _ in 0..initial_capacity {
            pool.push_back(Order::default());
        }
        
        Self {
            free_objects: pool,
            max_size: initial_capacity * 2, // 允许扩容
        }
    }
    
    fn acquire(&mut self) -> Option<Order> {
        self.free_objects.pop_front()
    }
    
    fn release(&mut self, mut order: Order) {
        if self.free_objects.len() < self.max_size {
            // 重置对象状态
            *order = Order::default();
            self.free_objects.push_back(order);
        }
    }
}

// 成交记录
#[derive(Debug, Clone)]
struct Trade {
pub id: u64,
pub buyer_order_id: u64,
pub seller_order_id: u64,
pub price: f64,
pub quantity: u64,
pub timestamp: i64,
}

// 用户账户
#[derive(Debug, Clone)]
struct Account {
pub user_id: u64,
pub available_balance: f64,
pub frozen_margin: f64,
}


🧪 使用示例

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_basic_matching() {
        let sequencer = Arc::new(Sequencer::new(100));
        let engine = MatchingEngine::new(sequencer.clone());
        
        // 启动撮合引擎线程
        let engine_handle = std::thread::spawn(move || {
            engine.run_matching_loop();
        });
        
        // 提交测试订单
        let buy_order = OrderData {
            user_id: 1,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            price: 100.0,
            quantity: 10,
            leverage: 10,
        };
        
        let sell_order = OrderData {
            user_id: 2, 
            side: OrderSide::Sell,
            order_type: OrderType::Limit,
            price: 99.0, // 卖单价低于买单价，可成交
            quantity: 10,
            leverage: 10,
        };
        
        sequencer.submit_order(buy_order).unwrap();
        sequencer.submit_order(sell_order).unwrap();
        
        // 等待撮合
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // 验证成交结果
        let trades = engine.trade_history.read().unwrap();
        assert!(!trades.is_empty());
        assert_eq!(trades[0].quantity, 10);
    }
}

fn main() {
println!("启动 Lighter 交易引擎...");

    let sequencer = Arc::new(Sequencer::new(1000));
    let engine = MatchingEngine::new(sequencer.clone());
    
    // 启动撮合引擎
    std::thread::spawn(move || {
        engine.run_matching_loop();
    });
    
    println!("Lighter 交易引擎运行中...");
}


💡 核心特性说明

这个实现包含了 Lighter 交易系统的关键特性：

1. 内存池管理：通过 OrderPool 减少内存分配开销
2. 价格-时间优先：严格按价格和时间排序保证公平性
3. 保证金风控：在订单入场前进行严格的保证金检查
4. 高效撮合：使用无锁或细粒度锁设计提高并发性能
5. 订单类型支持：区分处理市价单和限价单的不同逻辑

实际生产环境还需要添加持久化存储、网络接口、监控指标等组件。这个基础框架展示了 Rust 在构建高性能交易系统方面的优势。