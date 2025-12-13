//! 工作流测试 - 无锁版本

use prep_proc::proc::workflow::*;
use prep_proc::proc::trading_prep_order_proc::*;
use std::sync::Arc;

struct MockProc;

impl PerpOrderExchProc for MockProc {
    fn open_position(&self, cmd: OpenPositionCommand) -> Result<OpenPositionResult, PrepCommandError> {
        Ok(OpenPositionResult {
            order_id: OrderId::from_u64(1),
            status: OrderStatus::Filled,
            avg_price: Some(Price::from_f64(50000.0)),
            filled_quantity: cmd.quantity,
            trades: vec![],
            match_seq: 1,
            timestamp_ns: 0,
        })
    }

    fn close_position(&self, _cmd: ClosePositionCommand) -> Result<ClosePositionResult, PrepCommandError> {
        Ok(ClosePositionResult {
            order_id: OrderId::from_u64(1),
            status: OrderStatus::Filled,
            avg_price: Some(Price::from_f64(51000.0)),
            filled_quantity: Quantity::from_f64(1.0),
            trades: vec![],
            match_seq: 2,
            timestamp_ns: 0,
        })
    }

    fn set_leverage(&self, cmd: SetLeverageCommand) -> Result<SetLeverageResult, PrepCommandError> {
        Ok(SetLeverageResult {
            symbol: cmd.symbol,
            new_leverage: cmd.leverage,
            old_leverage: 10,
            timestamp: 0,
        })
    }

    fn set_position_mode(&self, _cmd: SetPositionModeCommand) -> Result<SetPositionModeResult, PrepCommandError> {
        Ok(SetPositionModeResult {
            dual_side_position: true,
            timestamp: 0,
        })
    }

    fn set_margin_type(&self, cmd: SetMarginTypeCommand) -> Result<SetMarginTypeResult, PrepCommandError> {
        Ok(SetMarginTypeResult {
            symbol: cmd.symbol,
            margin_type: cmd.margin_type,
            timestamp: 0,
        })
    }

    fn cancel_order(&self, _cmd: CancelOrderCommand) -> Result<CancelOrderResult, PrepCommandError> {
        Ok(CancelOrderResult {
            order_id: OrderId::from_u64(1),
            client_order_id: None,
            symbol: Some(Symbol::new("BTCUSDT")),
            status: OrderStatus::Cancelled,
            timestamp: 0,
        })
    }

    fn cancel_all_orders(&self, _cmd: CancelAllOrdersCommand) -> Result<CancelAllOrdersResult, PrepCommandError> {
        Ok(CancelAllOrdersResult {
            symbol: Symbol::new("BTCUSDT"),
            cancelled_count: 3,
            timestamp: 0,
        })
    }

    fn modify_order(&self, _cmd: ModifyOrderCommand) -> Result<ModifyOrderResult, PrepCommandError> {
        Ok(ModifyOrderResult {
            order_id: OrderId::from_u64(1),
            new_price: Some(Price::from_f64(50500.0)),
            new_quantity: Some(Quantity::from_f64(2.0)),
            timestamp: 0,
        })
    }
}

struct MockMarginProc;

impl MarginManagementProc for MockMarginProc {
    fn add_margin(&self, request: AddMarginRequest) -> Result<MarginOperationResult, PrepCommandError> {
        Ok(MarginOperationResult::new(
            request.symbol,
            request.position_side,
            request.amount,
            Price::from_f64(6000.0),
            Some(Price::from_f64(43000.0)),
        ))
    }

    fn reduce_margin(&self, request: ReduceMarginRequest) -> Result<MarginOperationResult, PrepCommandError> {
        Ok(MarginOperationResult::new(
            request.symbol,
            request.position_side,
            request.amount,
            Price::from_f64(4500.0),
            Some(Price::from_f64(46000.0)),
        ))
    }

    fn set_auto_add_margin(&self, _request: SetAutoAddMarginRequest) -> Result<(), PrepCommandError> {
        Ok(())
    }
}

#[test]
fn scenario_trader_opens_long_position_on_btc() {
    println!("\n🔄 场景1: 交易员开仓做多BTC");
    
    // Given: 交易员想要以10倍杠杆做多1个BTC
    let workflow = PerpetualTradingWorkflow::new(
        Arc::new(MockProc),
        Arc::new(MockProc),
        Arc::new(MockMarginProc),
    );

    let open_cmd = OpenPositionCommand {
        symbol: Some(Symbol::new("BTCUSDT")),
        side: Side::Buy,
        position_side: PositionSide::Long,
        order_type: OrderType::Market,
        quantity: Quantity::from_f64(1.0),
        price: None,
        time_in_force: TimeInForce::GoodTillCancel,
    };

    // When: 交易员提交开仓请求
    let action = TradeAction::OpenPosition(open_cmd);
    let result = workflow.execute(action);

    // Then: 开仓成功
    assert!(result.is_ok(), "开仓应该成功");

    if let Ok(WorkflowResult::PositionOpened(pos_result)) = result {
        assert_eq!(pos_result.status, OrderStatus::Filled);
        assert!(pos_result.avg_price.is_some());
        
        println!("   ✅ 开仓成功");
        println!("   订单ID: {}", pos_result.order_id);
        println!("   成交均价: {:?}", pos_result.avg_price);
        println!("   成交数量: {}", pos_result.filled_quantity.to_f64());
    }
}
