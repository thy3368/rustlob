//! 测试 MatchingService 与 BalanceRepo 集成
//!
//! 验证依赖注入的 BalanceRepo 正确工作

use account::{
    adaptor::MemoryBalanceRepo,
    domain::entity::{AccountId, AssetId}
};
use prep_proc::proc::{
    trading_prep_order_proc::{
        OpenPositionCommand, OrderType, PositionSide, Price, Quantity, Side, TradingPair, TimeInForce
    },
    trading_prep_order_proc_impl::PrepMatchingService
};

#[test]
fn test_matching_service_with_balance_repo() {
    // 创建余额仓储
    let mut balance_repo = MemoryBalanceRepo::with_default_timestamp();

    // 设置初始余额：10000 USDT
    let account_id = AccountId(1);
    let asset_id = AssetId(1); // USDT
    balance_repo.set_balance(account_id, asset_id, 10_000_000_000_000); // 10000 USDT (假设 8 位小数)

    // 创建撮合服务（依赖注入 BalanceRepo）
    let service = PrepMatchingService::new(balance_repo, account_id, asset_id);

    // 测试开仓命令
    let cmd = OpenPositionCommand {
        symbol: TradingPair::new("BTCUSDT"),
        side: Side::Buy,
        order_type: OrderType::Market,
        quantity: Quantity::from_f64(0.1),
        price: None,
        position_side: PositionSide::Long,
        time_in_force: TimeInForce::GTC,
        leverage: 10
    };

    // 执行开仓
    let result = service.open_position(cmd);

    // 验证结果
    assert!(result.is_ok(), "Open position should succeed");
    let open_result = result.unwrap();
    assert!(open_result.order_id.as_str().len() > 0, "Should have order ID");

    println!("✅ MatchingService with BalanceRepo integration test passed!");
}
