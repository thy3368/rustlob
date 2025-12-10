//! 交易员使用 MatchingService 进行交易的完整示例
//!
//! 本示例展示了一个交易员如何使用订单匹配服务进行各种交易操作：
//! - 限价单交易（买入/卖出）
//! - 市价单交易
//! - 不同的 TimeInForce 策略
//! - 订单取消
//! - 错误处理

use lob::lob::{
    Command, CommandResponse, MemoryOrderRepo, SpotMatchingService, OrderId, OrderRepo,
    OrderStatus, Price, Quantity, Side, SpotCommand, SpotCommandError, SpotCommandResult,
    SpotOrderProc, Symbol, TimeInForce, TraderId,
};

// 模拟的账户服务（实际使用时需要真实实现）
use account::AccountService;

/// 交易员结构体
///
/// 封装交易员的身份和交易操作
pub struct Trader {
    trader_id: TraderId,
    nonce_counter: u64, // 用于生成唯一的 nonce
}

impl Trader {
    /// 创建新的交易员
    pub fn new(trader_id: TraderId) -> Self {
        Self {
            trader_id,
            nonce_counter: 0,
        }
    }

    /// 生成下一个 nonce（幂等性保证）
    fn next_nonce(&mut self) -> u64 {
        self.nonce_counter += 1;
        self.nonce_counter
    }

    /// 场景1：限价买单 - GTC（Good Till Cancel）
    ///
    /// 交易员想以 50000 USDT 的价格买入 1.5 BTC
    /// 如果无法立即成交，订单会挂在订单簿上等待
    pub fn place_limit_buy_gtc<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景1: 限价买单 (GTC) ===");
        println!("交易员: {:?}", self.trader_id);
        println!("交易对: {:?}", symbol);
        println!("方向: 买入 (Buy)");
        println!("价格: {} USDT", price);
        println!("数量: {} BTC", quantity);
        println!("有效期: GTC (撤单前一直有效)");

        let command = SpotCommand::LimitOrder {
            trader: self.trader_id,
            symbol,
            side: Side::Buy,
            price,
            quantity,
            time_in_force: TimeInForce::GoodTillCancel,
            client_order_id: Some(format!("CLIENT-BUY-{}", self.nonce_counter)),
        };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 场景2：限价卖单 - PostOnly（只做 Maker）
    ///
    /// 交易员想以 50100 USDT 的价格卖出 2.0 BTC
    /// PostOnly 确保订单不会立即成交（只做流动性提供者）
    pub fn place_limit_sell_post_only<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景2: 限价卖单 (PostOnly) ===");
        println!("交易员: {:?}", self.trader_id);
        println!("交易对: {:?}", symbol);
        println!("方向: 卖出 (Sell)");
        println!("价格: {} USDT", price);
        println!("数量: {} BTC", quantity);
        println!("有效期: PostOnly (只做 Maker，不吃单)");

        let command = SpotCommand::LimitOrder {
            trader: self.trader_id,
            symbol,
            side: Side::Sell,
            price,
            quantity,
            time_in_force: TimeInForce::PostOnly,
            client_order_id: Some(format!("CLIENT-SELL-{}", self.nonce_counter)),
        };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 场景3：限价买单 - IOC（Immediate Or Cancel）
    ///
    /// 交易员想立即买入，未成交部分自动取消
    /// 适合需要快速执行但不想挂单的场景
    pub fn place_limit_buy_ioc<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景3: 限价买单 (IOC) ===");
        println!("交易员: {:?}", self.trader_id);
        println!("交易对: {:?}", symbol);
        println!("方向: 买入 (Buy)");
        println!("价格: {} USDT", price);
        println!("数量: {} BTC", quantity);
        println!("有效期: IOC (立即成交，未成交部分取消)");

        let command = SpotCommand::LimitOrder {
            trader: self.trader_id,
            symbol,
            side: Side::Buy,
            price,
            quantity,
            time_in_force: TimeInForce::ImmediateOrCancel,
            client_order_id: Some(format!("CLIENT-IOC-{}", self.nonce_counter)),
        };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 场景4：限价买单 - FOK（Fill Or Kill）
    ///
    /// 交易员要求全部成交，否则全部拒绝
    /// 适合大单执行，避免部分成交的风险
    pub fn place_limit_buy_fok<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景4: 限价买单 (FOK) ===");
        println!("交易员: {:?}", self.trader_id);
        println!("交易对: {:?}", symbol);
        println!("方向: 买入 (Buy)");
        println!("价格: {} USDT", price);
        println!("数量: {} BTC", quantity);
        println!("有效期: FOK (全部成交或全部拒绝)");

        let command = SpotCommand::LimitOrder {
            trader: self.trader_id,
            symbol,
            side: Side::Buy,
            price,
            quantity,
            time_in_force: TimeInForce::FillOrKill,
            client_order_id: Some(format!("CLIENT-FOK-{}", self.nonce_counter)),
        };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 场景5：市价买单
    ///
    /// 交易员想立即以市场价买入，不关心具体价格
    /// 适合需要快速建仓的场景
    pub fn place_market_buy<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        symbol: Symbol,
        quantity: Quantity,
        price_limit: Option<Price>, // 价格保护：最高愿意支付的价格
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景5: 市价买单 ===");
        println!("交易员: {:?}", self.trader_id);
        println!("交易对: {:?}", symbol);
        println!("方向: 买入 (Buy)");
        println!("数量: {} BTC", quantity);
        if let Some(limit) = price_limit {
            println!("价格保护: 最高 {} USDT", limit);
        } else {
            println!("价格保护: 无限制（风险较高）");
        }

        let command = SpotCommand::MarketOrder {
            trader: self.trader_id,
            symbol,
            side: Side::Buy,
            quantity,
            price_limit,
            time_in_force: None, // 默认 IOC
            client_order_id: Some(format!("CLIENT-MARKET-{}", self.nonce_counter)),
        };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 场景6：市价卖单 - FOK
    ///
    /// 交易员想立即卖出全部数量，否则不卖
    pub fn place_market_sell_fok<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        symbol: Symbol,
        quantity: Quantity,
        price_limit: Option<Price>, // 价格保护：最低愿意接受的价格
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景6: 市价卖单 (FOK) ===");
        println!("交易员: {:?}", self.trader_id);
        println!("交易对: {:?}", symbol);
        println!("方向: 卖出 (Sell)");
        println!("数量: {} BTC", quantity);
        if let Some(limit) = price_limit {
            println!("价格保护: 最低 {} USDT", limit);
        }
        println!("有效期: FOK (全部成交或全部拒绝)");

        let command = SpotCommand::MarketOrder {
            trader: self.trader_id,
            symbol,
            side: Side::Sell,
            quantity,
            price_limit,
            time_in_force: Some(TimeInForce::FillOrKill),
            client_order_id: Some(format!("CLIENT-MARKET-FOK-{}", self.nonce_counter)),
        };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 场景7：取消订单
    ///
    /// 交易员想取消之前挂的订单
    pub fn cancel_order<R, A>(
        &mut self,
        matching_service: &mut SpotMatchingService<R, A>,
        order_id: OrderId,
    ) -> Result<CommandResponse<SpotCommandResult>, SpotCommandError>
    where
        R: OrderRepo + Send + Sync,
        A: AccountService,
    {
        println!("\n=== 场景7: 取消订单 ===");
        println!("交易员: {:?}", self.trader_id);
        println!("订单ID: {}", order_id);

        let command = SpotCommand::CancelOrder { order_id };

        let idempotent_cmd = Command::new(self.next_nonce(), command);
        let result = matching_service.handle(idempotent_cmd)?;

        self.print_result(&result);
        Ok(result)
    }

    /// 打印命令执行结果
    fn print_result(&self, response: &CommandResponse<SpotCommandResult>) {
        println!("\n--- 执行结果 ---");
        println!("Nonce: {}", response.metadata.nonce);

        match &response.result {
            SpotCommandResult::LimitOrder {
                order_id,
                status,
                filled_quantity,
                remaining_quantity,
                trades,
            } => {
                println!("订单类型: 限价单");
                println!("订单ID: {}", order_id);
                println!("状态: {:?}", status);
                println!("已成交数量: {}", filled_quantity);
                println!("剩余数量: {}", remaining_quantity);
                println!("成交笔数: {}", trades.len());

                match status {
                    OrderStatus::Filled => {
                        println!("✅ 订单已完全成交");
                    }
                    OrderStatus::PartiallyFilled => {
                        println!("⚠️  订单部分成交，剩余 {} 挂单中", remaining_quantity);
                    }
                    OrderStatus::Pending => {
                        println!("⏳ 订单已挂单，等待成交");
                    }
                    OrderStatus::Rejected => {
                        println!("❌ 订单被拒绝（可能是 PostOnly 会立即成交，或 FOK 无法全部成交）");
                    }
                    OrderStatus::Cancelled => {
                        println!("🚫 订单已取消（IOC 未成交部分）");
                    }
                    _ => {}
                }
            }
            SpotCommandResult::MarketOrder {
                status,
                filled_quantity,
                trades,
            } => {
                println!("订单类型: 市价单");
                println!("状态: {:?}", status);
                println!("已成交数量: {}", filled_quantity);
                println!("成交笔数: {}", trades.len());

                match status {
                    OrderStatus::Filled => {
                        println!("✅ 市价单已完全成交");
                    }
                    OrderStatus::Cancelled => {
                        println!("⚠️  市价单部分成交，剩余已取消");
                    }
                    OrderStatus::Rejected => {
                        println!("❌ 市价单被拒绝（FOK 无法全部成交）");
                    }
                    _ => {}
                }
            }
            SpotCommandResult::CancelOrder { order_id, status } => {
                println!("操作类型: 取消订单");
                println!("订单ID: {}", order_id);
                println!("状态: {:?}", status);
                println!("✅ 订单已成功取消");
            }
            SpotCommandResult::CancelAllOrders { cancelled_count, order_ids } => {
                println!("操作类型: 批量取消订单");
                println!("取消数量: {}", cancelled_count);
                println!("订单IDs: {:?}", order_ids);
                println!("✅ 已成功取消 {} 个订单", cancelled_count);
            }
        }
    }
}

/// 完整的交易场景示例
///
/// 展示一个交易员的完整交易流程
#[cfg(test)]
mod tests {
    use super::*;
    use account::TradingPair;

    #[test]
    fn test_trader_workflow() {
        // 1. 初始化交易系统
        let order_repo = MemoryOrderRepo::new(100000, 10000);
        let account_service = mocks::MockAccountService::new();
        let trading_pair = TradingPair {
            base_asset: account::AssetId(1), // BTC
            quote_asset: account::AssetId(2), // USDT
        };

        let mut matching_service = SpotMatchingService::new(order_repo, account_service, trading_pair);

        // 2. 创建交易员
        let trader_id = TraderId::new([1u8; 8]);
        let mut trader = Trader::new(trader_id);

        let symbol = Symbol::from_str("BTCUSDT");

        // 3. 场景1: 挂限价买单（GTC）
        println!("\n========================================");
        println!("交易员开始交易");
        println!("========================================");

        let result = trader.place_limit_buy_gtc(
            &mut matching_service,
            symbol,
            50000, // 价格: 50000 USDT
            1500,  // 数量: 1.5 BTC (使用整数表示，实际需要除以精度)
        );

        match result {
            Ok(response) => {
                if let SpotCommandResult::LimitOrder { order_id, .. } = response.result {
                    println!("\n✅ 限价买单已挂单，订单ID: {}", order_id);

                    // 4. 场景2: 挂限价卖单（PostOnly）
                    let _ = trader.place_limit_sell_post_only(
                        &mut matching_service,
                        symbol,
                        50100, // 价格: 50100 USDT
                        2000,  // 数量: 2.0 BTC
                    );

                    // 5. 场景3: 立即买入（IOC）
                    let _ = trader.place_limit_buy_ioc(
                        &mut matching_service,
                        symbol,
                        50050, // 价格: 50050 USDT
                        500,   // 数量: 0.5 BTC
                    );

                    // 6. 场景4: 全部成交或拒绝（FOK）
                    let _ = trader.place_limit_buy_fok(
                        &mut matching_service,
                        symbol,
                        50200, // 价格: 50200 USDT
                        3000,  // 数量: 3.0 BTC
                    );

                    // 7. 场景5: 市价买入
                    let _ = trader.place_market_buy(
                        &mut matching_service,
                        symbol,
                        1000,           // 数量: 1.0 BTC
                        Some(51000),    // 价格保护: 最高 51000 USDT
                    );

                    // 8. 场景6: 市价卖出（FOK）
                    let _ = trader.place_market_sell_fok(
                        &mut matching_service,
                        symbol,
                        500,            // 数量: 0.5 BTC
                        Some(49000),    // 价格保护: 最低 49000 USDT
                    );

                    // 9. 场景7: 取消之前的订单
                    let _ = trader.cancel_order(&mut matching_service, order_id);
                }
            }
            Err(e) => {
                println!("\n❌ 交易失败: {}", e);
            }
        }

        println!("\n========================================");
        println!("交易流程结束");
        println!("========================================");
    }

    #[test]
    fn test_error_handling() {
        println!("\n========================================");
        println!("错误处理示例");
        println!("========================================");

        // 模拟各种错误场景
        let order_repo = MemoryOrderRepo::new(100000, 10000);
        let account_service = mocks::MockAccountService::new();
        let trading_pair = TradingPair {
            base_asset: account::AssetId(1), // BTC
            quote_asset: account::AssetId(2), // USDT
        };

        let mut matching_service = SpotMatchingService::new(order_repo, account_service, trading_pair);
        let trader_id = TraderId::new([2u8; 8]);
        let mut trader = Trader::new(trader_id);
        let symbol = Symbol::from_str("BTCUSDT");

        // 错误1: 余额不足
        println!("\n--- 错误场景1: 余额不足 ---");
        let result = trader.place_limit_buy_gtc(
            &mut matching_service,
            symbol,
            50000,
            1000000, // 尝试买入 10000 BTC（余额不足）
        );

        if let Err(e) = result {
            println!("❌ 预期错误: {}", e);
        }

        // 错误2: PostOnly 订单会立即成交
        println!("\n--- 错误场景2: PostOnly 订单会立即成交 ---");
        // 假设当前最优卖价是 50000
        let result = trader.place_limit_sell_post_only(
            &mut matching_service,
            symbol,
            49900, // 价格低于最优买价，会立即成交
            1000,
        );

        if let Ok(response) = result {
            if let SpotCommandResult::LimitOrder { status, .. } = response.result {
                if status == OrderStatus::Rejected {
                    println!("✅ PostOnly 订单正确被拒绝");
                }
            }
        }

        // 错误3: FOK 订单无法全部成交
        println!("\n--- 错误场景3: FOK 订单无法全部成交 ---");
        let result = trader.place_limit_buy_fok(
            &mut matching_service,
            symbol,
            50000,
            100000, // 尝试买入 1000 BTC（市场深度不足）
        );

        if let Ok(response) = result {
            if let SpotCommandResult::LimitOrder { status, .. } = response.result {
                if status == OrderStatus::Rejected {
                    println!("✅ FOK 订单因无法全部成交被正确拒绝");
                }
            }
        }

        // 错误4: 取消不存在的订单
        println!("\n--- 错误场景4: 取消不存在的订单 ---");
        let result = trader.cancel_order(&mut matching_service, 999999);

        if let Err(e) = result {
            println!("❌ 预期错误: {}", e);
        }
    }
}

// Mock 实现（实际使用时需要真实实现）
#[cfg(test)]
mod mocks {
    use super::*;
    use account::{AccountCommand, AccountCommandResult, AccountId, AccountService};

    pub struct MockAccountService;

    impl MockAccountService {
        pub fn new() -> Self {
            Self
        }
    }

    impl AccountService for MockAccountService {
        fn execute(&mut self, _cmd: AccountCommand) -> AccountCommandResult {
            // 简单模拟：总是返回冻结成功
            AccountCommandResult::Frozen {
                reference_id: 1,
                asset_id: account::AssetId(2), // USDT
                amount: 50000,
                new_available: 100000,
                new_frozen: 50000,
            }
        }

        fn execute_batch(
            &mut self,
            cmds: Vec<AccountCommand>,
        ) -> Result<Vec<AccountCommandResult>, account::BalanceError> {
            // 简单模拟：批量执行
            Ok(cmds.into_iter().map(|cmd| self.execute(cmd)).collect())
        }
    }
}

fn main() {
    println!("请运行 `cargo test` 查看交易示例");
}
