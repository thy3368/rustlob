/// OrderCommandHandler 集成测试
///
/// 测试通过 handle 方法执行各种订单命令

use lob::lob::handler::{Command, CommandResult, OrderCommandHandler};
use lob::lob::matching_service::MatchingService;
use lob::lob::repository::InMemoryOrderRepository;
use lob::lob::types::{Side, TraderId};

// ========== 辅助函数 ==========

/// 创建测试用的 MatchingService
fn create_handler() -> MatchingService<InMemoryOrderRepository> {
    let repo = InMemoryOrderRepository::new(100_000, 1000);
    MatchingService::new(repo)
}

/// 从字符串创建 TraderId
fn trader(name: &str) -> TraderId {
    TraderId::from_str(name)
}

// ========== LimitOrder 测试 ==========

#[test]
fn test_limit_order_full_match() {
    let mut handler = create_handler();

    // 添加卖单
    let sell_cmd = Command::LimitOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        price: 10000,
        quantity: 100,
    };
    handler.handle(sell_cmd);

    // 买单完全匹配
    let buy_cmd = Command::LimitOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        price: 10000,
        quantity: 100,
    };

    let result = handler.handle(buy_cmd);

    match result {
        CommandResult::LimitOrder { order_id, trades } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 100);
            assert_eq!(trades[0].price, 10000);
            assert_eq!(order_id, 0); // 完全成交，无挂单
        }
        _ => panic!("期望 LimitOrder 结果"),
    }
}

#[test]
fn test_limit_order_partial_match() {
    let mut handler = create_handler();

    // 添加小卖单
    let sell_cmd = Command::LimitOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        price: 10000,
        quantity: 50,
    };
    handler.handle(sell_cmd);

    // 大买单部分成交
    let buy_cmd = Command::LimitOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        price: 10000,
        quantity: 100,
    };

    let result = handler.handle(buy_cmd);

    match result {
        CommandResult::LimitOrder { order_id, trades } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 50);
            assert!(order_id > 0); // 有剩余50挂单
        }
        _ => panic!("期望 LimitOrder 结果"),
    }
}

#[test]
fn test_limit_order_no_match() {
    let mut handler = create_handler();

    // 添加高价卖单
    let sell_cmd = Command::LimitOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        price: 10100,
        quantity: 100,
    };
    handler.handle(sell_cmd);

    // 低价买单无法成交
    let buy_cmd = Command::LimitOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        price: 10000,
        quantity: 100,
    };

    let result = handler.handle(buy_cmd);

    match result {
        CommandResult::LimitOrder { order_id, trades } => {
            assert_eq!(trades.len(), 0);
            assert!(order_id > 0); // 全部挂单
        }
        _ => panic!("期望 LimitOrder 结果"),
    }
}

// ========== MarketOrder 测试 ==========

#[test]
fn test_market_order_buy() {
    let mut handler = create_handler();

    // 添加多个卖单
    let sell1 = Command::LimitOrder {
        trader: trader("SELLER1"),
        side: Side::Sell,
        price: 10000,
        quantity: 50,
    };
    handler.handle(sell1);

    let sell2 = Command::LimitOrder {
        trader: trader("SELLER2"),
        side: Side::Sell,
        price: 10100,
        quantity: 50,
    };
    handler.handle(sell2);

    // 市价买单吃掉所有卖单
    let market_cmd = Command::MarketOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        quantity: 100,
    };

    let result = handler.handle(market_cmd);

    match result {
        CommandResult::MarketOrder { trades } => {
            assert_eq!(trades.len(), 2);
            assert_eq!(trades[0].price, 10000); // 先成交最低价
            assert_eq!(trades[0].quantity, 50);
            assert_eq!(trades[1].price, 10100);
            assert_eq!(trades[1].quantity, 50);
        }
        _ => panic!("期望 MarketOrder 结果"),
    }
}

#[test]
fn test_market_order_sell() {
    let mut handler = create_handler();

    // 添加多个买单
    let buy1 = Command::LimitOrder {
        trader: trader("BUYER1"),
        side: Side::Buy,
        price: 10100,
        quantity: 50,
    };
    handler.handle(buy1);

    let buy2 = Command::LimitOrder {
        trader: trader("BUYER2"),
        side: Side::Buy,
        price: 10000,
        quantity: 50,
    };
    handler.handle(buy2);

    // 市价卖单吃掉所有买单
    let market_cmd = Command::MarketOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        quantity: 100,
    };

    let result = handler.handle(market_cmd);

    match result {
        CommandResult::MarketOrder { trades } => {
            assert_eq!(trades.len(), 2);
            assert_eq!(trades[0].price, 10100); // 先成交最高价
            assert_eq!(trades[0].quantity, 50);
            assert_eq!(trades[1].price, 10000);
            assert_eq!(trades[1].quantity, 50);
        }
        _ => panic!("期望 MarketOrder 结果"),
    }
}

#[test]
fn test_market_order_no_liquidity() {
    let mut handler = create_handler();

    // 空订单簿，市价单无法成交
    let market_cmd = Command::MarketOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        quantity: 100,
    };

    let result = handler.handle(market_cmd);

    match result {
        CommandResult::MarketOrder { trades } => {
            assert_eq!(trades.len(), 0);
        }
        _ => panic!("期望 MarketOrder 结果"),
    }
}

// ========== CancelOrder 测试 ==========

#[test]
fn test_cancel_order_success() {
    let mut handler = create_handler();

    // 添加限价单
    let limit_cmd = Command::LimitOrder {
        trader: trader("TRADER"),
        side: Side::Buy,
        price: 10000,
        quantity: 100,
    };

    let limit_result = handler.handle(limit_cmd);

    let order_id = match limit_result {
        CommandResult::LimitOrder { order_id, .. } => order_id,
        _ => panic!("期望 LimitOrder 结果"),
    };

    // 取消订单
    let cancel_cmd = Command::CancelOrder { order_id };
    let result = handler.handle(cancel_cmd);

    match result {
        CommandResult::CancelOrder { success } => {
            assert!(success);
        }
        _ => panic!("期望 CancelOrder 结果"),
    }
}

#[test]
fn test_cancel_order_not_found() {
    let mut handler = create_handler();

    // 取消不存在的订单
    let cancel_cmd = Command::CancelOrder { order_id: 999 };
    let result = handler.handle(cancel_cmd);

    match result {
        CommandResult::CancelOrder { success } => {
            assert!(!success);
        }
        _ => panic!("期望 CancelOrder 结果"),
    }
}

// ========== IcebergOrder 测试 ==========

#[test]
fn test_iceberg_order_no_match() {
    let mut handler = create_handler();

    // 无对手方，冰山单全部挂单
    let iceberg_cmd = Command::IcebergOrder {
        trader: trader("TRADER"),
        side: Side::Buy,
        price: 10000,
        total_quantity: 1000,
        display_quantity: 100,
    };

    let result = handler.handle(iceberg_cmd);

    match result {
        CommandResult::IcebergOrder {
            order_id,
            trades,
            remaining_total,
            current_display,
        } => {
            assert_eq!(trades.len(), 0);
            assert!(order_id > 0);
            assert_eq!(remaining_total, 900); // 总量1000 - 显示100 = 900
            assert_eq!(current_display, 100); // 显示100挂单中
        }
        _ => panic!("期望 IcebergOrder 结果"),
    }
}

#[test]
fn test_iceberg_order_partial_match() {
    let mut handler = create_handler();

    // 添加小卖单
    let sell_cmd = Command::LimitOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        price: 10000,
        quantity: 50,
    };
    handler.handle(sell_cmd);

    // 冰山买单部分成交
    let iceberg_cmd = Command::IcebergOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        price: 10000,
        total_quantity: 1000,
        display_quantity: 100,
    };

    let result = handler.handle(iceberg_cmd);

    match result {
        CommandResult::IcebergOrder {
            order_id,
            trades,
            remaining_total,
            current_display,
        } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 50);
            assert!(order_id > 0);
            // remaining_total = 总量 - 成交 - 挂单 = 1000 - 50 - 50 = 900
            assert_eq!(remaining_total, 900);
            assert_eq!(current_display, 50); // 显示剩余50挂单
        }
        _ => panic!("期望 IcebergOrder 结果"),
    }
}

#[test]
fn test_iceberg_order_display_fully_matched() {
    let mut handler = create_handler();

    // 添加卖单恰好等于显示数量
    let sell_cmd = Command::LimitOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        price: 10000,
        quantity: 100,
    };
    handler.handle(sell_cmd);

    // 冰山买单显示部分完全成交，自动补充下一批
    let iceberg_cmd = Command::IcebergOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        price: 10000,
        total_quantity: 1000,
        display_quantity: 100,
    };

    let result = handler.handle(iceberg_cmd);

    match result {
        CommandResult::IcebergOrder {
            order_id,
            trades,
            remaining_total,
            current_display,
        } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 100);
            assert!(order_id > 0);
            assert_eq!(remaining_total, 800); // 总量1000 - 成交100 - 新显示100 = 800
            assert_eq!(current_display, 100); // 自动补充新的100显示
        }
        _ => panic!("期望 IcebergOrder 结果"),
    }
}

#[test]
fn test_iceberg_order_fully_matched() {
    let mut handler = create_handler();

    // 添加大卖单
    let sell_cmd = Command::LimitOrder {
        trader: trader("SELLER"),
        side: Side::Sell,
        price: 10000,
        quantity: 200,
    };
    handler.handle(sell_cmd);

    // 小冰山单：总量100，显示50
    // 第一批50成交后，剩余50自动作为下一批显示加入订单簿
    let iceberg_cmd = Command::IcebergOrder {
        trader: trader("BUYER"),
        side: Side::Buy,
        price: 10000,
        total_quantity: 100,
        display_quantity: 50,
    };

    let result = handler.handle(iceberg_cmd);

    match result {
        CommandResult::IcebergOrder {
            order_id,
            trades,
            remaining_total,
            current_display,
        } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 50); // 第一批50完全成交
            assert!(order_id > 0); // 有订单ID（剩余50自动挂单）
            assert_eq!(remaining_total, 0); // 剩余总量0（100 - 50成交 - 50挂单）
            assert_eq!(current_display, 50); // 当前显示50挂单中
        }
        _ => panic!("期望 IcebergOrder 结果"),
    }
}

// ========== 综合场景测试 ==========

#[test]
fn test_mixed_order_types() {
    let mut handler = create_handler();

    // 场景：多种订单类型混合
    // 1. 添加限价卖单
    let sell1 = Command::LimitOrder {
        trader: trader("SELLER1"),
        side: Side::Sell,
        price: 10000,
        quantity: 100,
    };
    handler.handle(sell1);

    // 2. 添加冰山买单（部分成交）
    let iceberg = Command::IcebergOrder {
        trader: trader("BUYER1"),
        side: Side::Buy,
        price: 10000,
        total_quantity: 200,
        display_quantity: 50,
    };
    let iceberg_result = handler.handle(iceberg);

    match iceberg_result {
        CommandResult::IcebergOrder { trades, .. } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 50);
        }
        _ => panic!("期望 IcebergOrder 结果"),
    }

    // 3. 市价买单吃掉剩余卖单
    let market = Command::MarketOrder {
        trader: trader("BUYER2"),
        side: Side::Buy,
        quantity: 100,
    };
    let market_result = handler.handle(market);

    match market_result {
        CommandResult::MarketOrder { trades } => {
            assert_eq!(trades.len(), 1);
            assert_eq!(trades[0].quantity, 50); // 剩余50被吃掉
        }
        _ => panic!("期望 MarketOrder 结果"),
    }
}

#[test]
fn test_handler_name() {
    let handler = create_handler();
    assert_eq!(handler.handler_name(), "PriceTimeMatchingService");
}
