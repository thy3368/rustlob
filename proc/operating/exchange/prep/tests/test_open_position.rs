//! 测试 open_position 功能
//!
//! 测试永续合约开仓功能的各种场景

use prep_proc::proc::{trading_prep_order_proc::*, trading_prep_order_proc_impl::MatchingService};

#[test]
fn test_open_position_market_long_success() {
    // Given: 一个有足够余额的撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送市价做多订单
    let symbol = Symbol::new("BTCUSDT");
    let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(0.1)).with_leverage(10);

    let result = matching_service.open_position(cmd);

    // Then: 订单应该成功
    assert!(result.is_ok());
    let response = result.unwrap();

    // 验证订单状态
    assert_eq!(response.status, OrderStatus::Filled);
    assert!(response.avg_price.is_some());
    assert_eq!(response.filled_quantity.to_f64(), 0.1);
    assert!(!response.trades.is_empty());
    assert!(response.match_seq.is_some());

    println!("✓ 市价做多订单成功: order_id={}", response.order_id);
    println!("  成交均价: {}", response.avg_price.unwrap().to_f64());
    println!("  成交数量: {}", response.filled_quantity.to_f64());
    println!("  成交笔数: {}", response.trades.len());
}

#[test]
fn test_open_position_market_short_success() {
    // Given: 一个有足够余额的撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送市价做空订单
    let symbol = Symbol::new("ETHUSDT");
    let cmd = OpenPositionCommand::market_short(symbol, Quantity::from_f64(1.0)).with_leverage(5);

    let result = matching_service.open_position(cmd);

    // Then: 订单应该成功
    assert!(result.is_ok());
    let response = result.unwrap();

    assert_eq!(response.status, OrderStatus::Filled);
    assert!(response.avg_price.is_some());
    assert_eq!(response.filled_quantity.to_f64(), 1.0);

    println!("✓ 市价做空订单成功: order_id={}", response.order_id);
}

#[test]
fn test_open_position_limit_long_success() {
    // Given: 一个有足够余额的撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送限价做多订单
    let symbol = Symbol::new("BTCUSDT");
    let cmd =
        OpenPositionCommand::limit_long(symbol, Quantity::from_f64(0.2), Price::from_f64(48000.0)).with_leverage(20);

    let result = matching_service.open_position(cmd);

    // Then: 订单应该被接受（可能成交或挂单）
    assert!(result.is_ok());
    let response = result.unwrap();

    // 限价单可能立即成交或进入订单簿
    assert!(response.status == OrderStatus::Filled || response.status == OrderStatus::Submitted);

    if response.status == OrderStatus::Filled {
        println!("✓ 限价做多订单立即成交: order_id={}", response.order_id);
        assert!(response.avg_price.is_some());
        assert_eq!(response.filled_quantity.to_f64(), 0.2);
    } else {
        println!("✓ 限价做多订单已挂单: order_id={}", response.order_id);
        assert_eq!(response.filled_quantity.to_f64(), 0.0);
    }
}

#[test]
fn test_open_position_insufficient_balance() {
    // Given: 一个余额不足的撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10.0)); // 只有10 USDT

    // When: 尝试开大仓位
    let symbol = Symbol::new("BTCUSDT");
    let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(10.0)) // 需要约500,000 USDT保证金
        .with_leverage(1); // 1倍杠杆

    let result = matching_service.open_position(cmd);

    // Then: 应该返回余额不足错误
    assert!(result.is_err());
    let error = result.unwrap_err();

    match error {
        PrepCommandError::InsufficientBalance => {
            println!("✓ 正确检测到余额不足错误");
        }
        _ => panic!("期望余额不足错误，但得到: {:?}", error)
    }
}

#[test]
fn test_open_position_invalid_quantity() {
    // Given: 一个撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送数量为0的订单
    let symbol = Symbol::new("BTCUSDT");
    let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_raw(0)); // 无效数量

    let result = matching_service.open_position(cmd);

    // Then: 应该返回验证错误
    assert!(result.is_err());
    let error = result.unwrap_err();

    match error {
        PrepCommandError::ValidationError(_) => {
            println!("✓ 正确检测到数量验证错误");
        }
        _ => panic!("期望验证错误，但得到: {:?}", error)
    }
}

#[test]
fn test_open_position_invalid_leverage() {
    // Given: 一个撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送杠杆超过125倍的订单
    let symbol = Symbol::new("BTCUSDT");
    let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(0.1)).with_leverage(200); // 无效杠杆

    let result = matching_service.open_position(cmd);

    // Then: 应该返回验证错误
    assert!(result.is_err());
    let error = result.unwrap_err();

    match error {
        PrepCommandError::ValidationError(_) => {
            println!("✓ 正确检测到杠杆验证错误");
        }
        _ => panic!("期望验证错误，但得到: {:?}", error)
    }
}

#[test]
fn test_open_position_limit_missing_price() {
    // Given: 一个撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送限价单但没有指定价格
    let symbol = Symbol::new("BTCUSDT");
    let mut cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(0.1));
    cmd.order_type = OrderType::Limit;
    cmd.price = None; // 限价单缺少价格

    let result = matching_service.open_position(cmd);

    // Then: 应该返回验证错误
    assert!(result.is_err());
    let error = result.unwrap_err();

    match error {
        PrepCommandError::ValidationError(_) => {
            println!("✓ 正确检测到限价单缺少价格错误");
        }
        _ => panic!("期望验证错误，但得到: {:?}", error)
    }
}

#[test]
fn test_open_position_with_different_leverage() {
    // Given: 一个有足够余额的撮合服务
    let matching_service = MatchingService::new(Price::from_f64(100000.0));

    // Test with leverage 1x
    let symbol = Symbol::new("BTCUSDT");
    let cmd1 = OpenPositionCommand::market_long(symbol, Quantity::from_f64(0.1)).with_leverage(1);

    let result1 = matching_service.open_position(cmd1);
    assert!(result1.is_ok());
    println!("✓ 1倍杠杆开仓成功");

    // Test with leverage 10x
    let cmd2 = OpenPositionCommand::market_long(symbol, Quantity::from_f64(0.1)).with_leverage(10);

    let result2 = matching_service.open_position(cmd2);
    assert!(result2.is_ok());
    println!("✓ 10倍杠杆开仓成功");

    // Test with leverage 125x (maximum)
    let cmd3 = OpenPositionCommand::market_long(symbol, Quantity::from_f64(0.1)).with_leverage(125);

    let result3 = matching_service.open_position(cmd3);
    assert!(result3.is_ok());
    println!("✓ 125倍杠杆开仓成功");
}

#[test]
fn test_open_position_check_trades_details() {
    // Given: 一个撮合服务
    let matching_service = MatchingService::new(Price::from_f64(10000.0));

    // When: 发送市价订单
    let symbol = Symbol::new("BTCUSDT");
    let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0)).with_leverage(10);

    let result = matching_service.open_position(cmd).unwrap();

    // Then: 检查成交明细
    assert!(!result.trades.is_empty());

    for (i, trade) in result.trades.iter().enumerate() {
        println!("成交 #{}: ", i + 1);
        println!("  成交ID: {}", trade.trade_id);
        println!("  订单ID: {}", trade.order_id);
        println!("  交易对: {}", trade.symbol);
        println!("  方向: {:?}", trade.side);
        println!("  价格: {}", trade.price.to_f64());
        println!("  数量: {}", trade.quantity.to_f64());
        println!("  手续费: {} USDT", trade.fee.to_f64());
        println!("  是否Maker: {}", trade.is_maker);

        // 验证成交记录
        assert_eq!(trade.order_id, result.order_id);
        assert_eq!(trade.symbol.as_str(), "BTCUSDT");
        assert_eq!(trade.side, Side::Buy);
        assert!(trade.price.is_positive());
        assert!(trade.quantity.is_positive());
        assert!(trade.fee.raw() >= 0);
    }

    println!("✓ 成交明细验证通过");
}

#[test]
fn test_query_position_after_open() {
    // Given: 一个撮合服务
    let matching_service = MatchingService::new(Price::from_f64(100000.0));

    // When: 开仓
    let symbol = Symbol::new("BTCUSDT");
    let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0)).with_leverage(10);

    let open_result = matching_service.open_position(cmd).unwrap();
    assert_eq!(open_result.status, OrderStatus::Filled);

    // Then: 查询持仓
    let query_cmd = QueryPositionCommand::long(symbol);
    let position = matching_service.query_position(query_cmd).unwrap();

    println!("持仓信息:");
    println!("  交易对: {}", position.symbol);
    println!("  持仓方向: {:?}", position.position_side);
    println!("  持仓数量: {}", position.quantity.to_f64());
    println!("  持仓均价: {}", position.entry_price.to_f64());
    println!("  杠杆倍数: {}x", position.leverage);
    println!("  保证金: {} USDT", position.margin.to_f64());

    // 验证持仓
    assert!(position.has_position());
    assert!(position.is_long());
    assert_eq!(position.quantity.to_f64(), 1.0);
    assert_eq!(position.leverage, 10);

    println!("✓ 持仓查询验证通过");
}
