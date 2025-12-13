# 永续合约交易系统使用示例

本文档展示如何使用新实现的工作流、清算、资金费率和事件系统。

## 1. 工作流编排器使用

### 1.1 开仓交易

```rust
use rustlob::workflow::*;
use rustlob::proc::trading_prep_order_proc::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建处理器实例
    let command_processor = Arc::new(MyPerpOrderExchProc::new());
    let query_processor = Arc::new(MyPerpOrderExchQueryProc::new());
    let margin_processor = Arc::new(MyMarginManagementProc::new());

    // 创建工作流
    let workflow = PerpetualTradingWorkflow::new(
        command_processor,
        query_processor,
        margin_processor,
    );

    // 构造开仓命令
    let open_cmd = OpenPositionCommand {
        trader_id: "trader123".to_string(),
        symbol: Symbol::new("BTCUSDT"),
        side: Side::Buy,
        position_side: PositionSide::Long,
        order_type: OrderType::Market,
        quantity: Quantity::from_f64(1.0),
        price: None,
        leverage: 10,
        margin_mode: "ISOLATED".to_string(),
    };

    // 执行开仓
    let action = TradeAction::OpenPosition(open_cmd);
    let result = workflow.execute(action).await?;

    match result {
        WorkflowResult::PositionOpened(res) => {
            println!("开仓成功！持仓ID: {}", res.position_id);
            println!("开仓均价: {}", res.entry_price);
            println!("强平价格: {:?}", res.liquidation_price);
        }
        _ => unreachable!(),
    }

    Ok(())
}
```

### 1.2 追加保证金（避免强平）

```rust
// 当价格接近强平价时，追加保证金
let add_margin_req = AddMarginRequest::new(
    "trader123".to_string(),
    Symbol::new("BTCUSDT"),
    PositionSide::Long,
    Price::from_f64(500.0), // 追加500 USDT
);

let action = TradeAction::AddMargin(add_margin_req);
let result = workflow.execute(action).await?;

match result {
    WorkflowResult::MarginAdded(res) => {
        println!("追加保证金成功！");
        println!("新的总保证金: {}", res.new_margin);
        println!("新的强平价格: {:?}", res.new_liquidation_price);
    }
    _ => unreachable!(),
}
```

### 1.3 调整杠杆

```rust
// 从10倍降低到5倍，降低风险
let leverage_cmd = SetLeverageCommand {
    trader_id: "trader123".to_string(),
    symbol: Symbol::new("BTCUSDT"),
    leverage: 5,
};

let action = TradeAction::AdjustLeverage(leverage_cmd);
let result = workflow.execute(action).await?;

match result {
    WorkflowResult::LeverageAdjusted(res) => {
        println!("杠杆调整成功！新杠杆: {}", res.new_leverage);
    }
    _ => unreachable!(),
}
```

## 2. 强制平仓流程

### 2.1 实现清算处理器

```rust
use rustlob::liquidation::*;

struct MyLiquidationProc {
    // 订单簿、撮合引擎等依赖
}

#[async_trait::async_trait]
impl LiquidationProc for MyLiquidationProc {
    async fn execute_liquidation(
        &self,
        position: PositionInfo,
    ) -> Result<LiquidationResult, PrepCommandError> {
        // 使用工作流自动执行三级强平
        let workflow = LiquidationWorkflow::new(Arc::new(self.clone()));
        workflow.execute(position).await
    }

    async fn try_market_liquidation(
        &self,
        position: &PositionInfo,
    ) -> Result<Option<Vec<Trade>>, PrepCommandError> {
        // 1. 提交紧急市价单
        let liquidation_side = match position.side {
            PositionSide::Long => Side::Sell,
            PositionSide::Short => Side::Buy,
            _ => return Err(PrepCommandError::ValidationError("Invalid position side")),
        };

        let order_id = self.submit_urgent_market_order(
            position.symbol,
            liquidation_side,
            position.quantity,
        ).await?;

        // 2. 等待撮合（5秒超时）
        match tokio::time::timeout(
            Duration::from_secs(5),
            self.wait_for_order_filled(order_id),
        ).await {
            Ok(Ok(trades)) => {
                // 市场强平成功
                Ok(Some(trades))
            }
            _ => {
                // 超时或失败，进入下一级
                Ok(None)
            }
        }
    }

    async fn try_insurance_fund_takeover(
        &self,
        position: &PositionInfo,
    ) -> Result<Option<InsuranceFundTakeover>, PrepCommandError> {
        // 1. 检查保险基金容量
        let capacity = self.get_insurance_fund_capacity().await?;
        let required_loss = self.calculate_potential_loss(position)?;

        if capacity.can_cover(required_loss) {
            // 2. 保险基金接管
            let takeover = InsuranceFundTakeover {
                total_loss: required_loss,
                timestamp: current_timestamp_ms(),
            };

            // 3. 扣除保险基金
            self.deduct_insurance_fund(required_loss).await?;

            Ok(Some(takeover))
        } else {
            // 容量不足，进入ADL
            Ok(None)
        }
    }

    async fn trigger_auto_deleveraging(
        &self,
        position: &PositionInfo,
    ) -> Result<ADLExecutionResult, PrepCommandError> {
        // 1. 获取对手方盈利持仓（按ADL优先级排序）
        let counterparty_side = match position.side {
            PositionSide::Long => PositionSide::Short,
            PositionSide::Short => PositionSide::Long,
            _ => return Err(PrepCommandError::ValidationError("Invalid position side")),
        };

        let mut candidates = self.get_adl_candidates(
            position.symbol,
            counterparty_side,
        ).await?;

        // 2. 按优先级排序（Level5优先）
        candidates.sort_by(|a, b| {
            b.priority_score().partial_cmp(&a.priority_score()).unwrap()
        });

        // 3. 依次强平对手方，直到覆盖损失
        let mut affected_positions = Vec::new();
        let mut total_quantity = Quantity::from_raw(0);
        let mut total_notional = 0.0;
        let mut remaining_qty = position.quantity;

        for candidate in candidates {
            if remaining_qty.raw() == 0 {
                break;
            }

            let close_qty = remaining_qty.min(candidate.quantity);

            // 强制平仓对手方持仓
            self.force_close_position(
                candidate.position_id,
                close_qty,
            ).await?;

            affected_positions.push(candidate.position_id);
            total_quantity = Quantity::from_raw(total_quantity.raw() + close_qty.raw());
            total_notional += close_qty.to_f64() * position.mark_price.to_f64();
            remaining_qty = Quantity::from_raw(remaining_qty.raw() - close_qty.raw());

            // 通知被ADL的用户
            self.notify_adl_counterparty(candidate.trader_id).await?;
        }

        let avg_price = Price::from_f64(total_notional / total_quantity.to_f64());

        Ok(ADLExecutionResult {
            affected_positions,
            total_quantity,
            avg_price,
            timestamp: current_timestamp_ms(),
        })
    }

    // ... 其他方法实现
}
```

### 2.2 触发强平

```rust
// 风控引擎检测到触发强平
async fn risk_monitor_loop(liquidation_proc: Arc<MyLiquidationProc>) {
    loop {
        // 1. 获取所有持仓
        let positions = get_all_positions().await;

        for position in positions {
            // 2. 检查是否触发强平
            if should_liquidate(&position) {
                log::warn!("触发强平: Position {}", position.position_id);

                // 3. 执行强平流程
                match liquidation_proc.execute_liquidation(position.clone()).await {
                    Ok(result) => {
                        log::info!(
                            "强平完成: {} 类型={} 损失={}",
                            position.position_id,
                            result.liquidation_type.as_str(),
                            result.margin_loss
                        );

                        // 发送通知
                        notify_user_liquidation(&position.trader_id, &result).await;
                    }
                    Err(e) => {
                        log::error!("强平失败: {:?}", e);
                    }
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn should_liquidate(position: &PositionInfo) -> bool {
    // 标记价格触达或超过强平价格
    if let Some(liq_price) = position.liquidation_price {
        match position.side {
            PositionSide::Long => position.mark_price.raw() <= liq_price.raw(),
            PositionSide::Short => position.mark_price.raw() >= liq_price.raw(),
            _ => false,
        }
    } else {
        false
    }
}
```

## 3. 资金费率结算

### 3.1 实现资金费率处理器

```rust
use rustlob::funding_rate::*;

struct MyFundingRateProc {
    calculator: FundingRateCalculator,
}

#[async_trait::async_trait]
impl FundingRateProc for MyFundingRateProc {
    async fn settle_funding_rates(
        &self,
        settlement_time: SystemTime,
    ) -> Result<FundingRateSettlementSummary, PrepCommandError> {
        // 使用工作流自动执行结算
        let workflow = FundingRateSettlementWorkflow::new(self.clone());
        workflow.execute(settlement_time).await
    }

    fn calculate_funding_rate(
        &self,
        symbol: Symbol,
        mark_price: Price,
        index_price: Price,
        funding_interval_hours: u32,
    ) -> FundingRateRecord {
        let premium_index = self.calculator.calculate_premium_index(
            mark_price,
            index_price,
        );

        let funding_rate = self.calculator.calculate_funding_rate(
            premium_index,
            funding_interval_hours,
        );

        FundingRateRecord::new(
            symbol,
            funding_rate,
            premium_index,
            funding_interval_hours,
        )
    }

    async fn settle_position_funding_fee(
        &self,
        position: &PositionInfo,
        funding_rate: f64,
    ) -> Result<FundingFeeDetail, PrepCommandError> {
        // 计算持仓价值
        let position_value = Price::from_f64(
            position.mark_price.to_f64() * position.quantity.to_f64()
        );

        // 创建费用明细
        let fee_detail = FundingFeeDetail::new(
            position.position_id,
            position.trader_id.clone(),
            position.symbol,
            position.side,
            position.quantity,
            position_value,
            funding_rate,
        );

        // 更新账户余额
        if fee_detail.is_payment() {
            // 多仓支付
            self.deduct_balance(
                &position.trader_id,
                fee_detail.abs_amount(),
            ).await?;
        } else {
            // 空仓收取
            self.credit_balance(
                &position.trader_id,
                fee_detail.abs_amount(),
            ).await?;
        }

        Ok(fee_detail)
    }

    // ... 其他方法实现
}
```

### 3.2 定时触发结算

```rust
use tokio::time::{interval, Duration};

async fn funding_rate_scheduler(funding_proc: Arc<MyFundingRateProc>) {
    // 每4小时触发一次（检查是否需要结算）
    let mut timer = interval(Duration::from_secs(4 * 3600));

    loop {
        timer.tick().await;

        log::info!("资金费率定时任务触发");

        let settlement_time = SystemTime::now();

        match funding_proc.settle_funding_rates(settlement_time).await {
            Ok(summary) => {
                log::info!(
                    "资金费率结算完成: {} 个交易对, {} 个持仓",
                    summary.symbols_count,
                    summary.positions_count
                );
                log::info!("净资金流动: {}", summary.net_flow);
            }
            Err(e) => {
                log::error!("资金费率结算失败: {:?}", e);
            }
        }
    }
}
```

## 4. 事件系统集成

### 4.1 实现事件发布器

```rust
use rustlob::events::*;
use kafka::producer::{Producer, Record};

pub struct KafkaEventPublisher {
    producer: Producer,
    topic: String,
}

#[async_trait::async_trait]
impl EventPublisher for KafkaEventPublisher {
    async fn publish<E: DomainEvent>(&self, event: E) -> Result<(), String> {
        let event_type = event.event_type();
        let json = event.to_json().map_err(|e| e.to_string())?;

        // 发送到Kafka
        let record = Record::from_value(&self.topic, json.as_bytes());
        self.producer
            .send(&record)
            .map_err(|e| format!("Kafka发送失败: {:?}", e))?;

        log::debug!("事件已发布: {}", event_type);

        Ok(())
    }

    async fn publish_batch(&self, events: Vec<Box<dyn DomainEvent>>) -> Result<(), String> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }
}
```

### 4.2 在业务流程中发布事件

```rust
// 在开仓成功后发布事件
async fn open_position_with_events(
    processor: &MyPerpOrderExchProc,
    event_publisher: &KafkaEventPublisher,
    cmd: OpenPositionCommand,
) -> Result<OpenPositionResult, PrepCommandError> {
    // 1. 执行开仓
    let result = processor.open_position(cmd.clone()).await?;

    // 2. 发布事件
    let event = PositionOpenedEvent::new(
        result.position_id,
        cmd.trader_id,
        cmd.symbol,
        cmd.position_side,
        cmd.quantity,
        result.entry_price,
        cmd.leverage,
        cmd.margin_mode,
        result.initial_margin,
        result.liquidation_price,
    );

    event_publisher.publish(event).await.ok();

    Ok(result)
}

// 在强平完成后发布事件
async fn liquidate_with_events(
    liquidation_proc: &MyLiquidationProc,
    event_publisher: &KafkaEventPublisher,
    position: PositionInfo,
) -> Result<LiquidationResult, PrepCommandError> {
    // 1. 触发强平事件
    let trigger_event = LiquidationTriggeredEvent {
        position_id: position.position_id,
        trader_id: position.trader_id.clone(),
        symbol: position.symbol,
        trigger_price: position.mark_price,
        liquidation_price: position.liquidation_price.unwrap(),
        quantity: position.quantity,
        side: position.side,
        timestamp: current_timestamp_ms(),
    };
    event_publisher.publish(trigger_event).await.ok();

    // 2. 执行强平
    let result = liquidation_proc.execute_liquidation(position.clone()).await?;

    // 3. 强平完成事件
    let completed_event = LiquidationCompletedEvent {
        position_id: result.position_id,
        trader_id: position.trader_id,
        symbol: position.symbol,
        liquidation_type: result.liquidation_type.as_str().to_string(),
        liquidation_price: result.liquidation_price,
        liquidated_quantity: result.liquidated_quantity,
        margin_loss: result.margin_loss,
        insurance_fund_loss: result.insurance_fund_loss,
        order_status: result.order_status.to_string(),
        timestamp: result.timestamp,
    };
    event_publisher.publish(completed_event).await.ok();

    Ok(result)
}
```

## 5. 完整示例：开仓到平仓的完整流程

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化组件
    let command_processor = Arc::new(MyPerpOrderExchProc::new());
    let query_processor = Arc::new(MyPerpOrderExchQueryProc::new());
    let margin_processor = Arc::new(MyMarginManagementProc::new());
    let liquidation_proc = Arc::new(MyLiquidationProc::new());
    let funding_proc = Arc::new(MyFundingRateProc::new());
    let event_publisher = Arc::new(KafkaEventPublisher::new());

    // 创建工作流
    let workflow = PerpetualTradingWorkflow::new(
        command_processor.clone(),
        query_processor.clone(),
        margin_processor.clone(),
    );

    // 1. 开仓
    println!("===== 步骤1: 开仓 =====");
    let open_cmd = OpenPositionCommand::market_long(
        Symbol::new("BTCUSDT"),
        Quantity::from_f64(1.0),
    );
    let open_result = workflow.execute(TradeAction::OpenPosition(open_cmd)).await?;

    if let WorkflowResult::PositionOpened(res) = open_result {
        println!("开仓成功: Position {}", res.position_id);
        println!("开仓价: {}, 强平价: {:?}", res.entry_price, res.liquidation_price);
    }

    // 2. 模拟价格波动，接近强平价
    println!("\n===== 步骤2: 价格下跌，接近强平 =====");
    println!("当前价格接近强平价，追加保证金...");

    let add_margin = AddMarginRequest::new(
        "trader123".to_string(),
        Symbol::new("BTCUSDT"),
        PositionSide::Long,
        Price::from_f64(1000.0),
    );
    let margin_result = workflow.execute(TradeAction::AddMargin(add_margin)).await?;

    if let WorkflowResult::MarginAdded(res) = margin_result {
        println!("追加保证金成功，新强平价: {:?}", res.new_liquidation_price);
    }

    // 3. 资金费率结算
    println!("\n===== 步骤3: 资金费率结算 =====");
    let funding_summary = funding_proc.settle_funding_rates(SystemTime::now()).await?;
    println!("结算 {} 个交易对", funding_summary.symbols_count);
    println!("净资金流动: {}", funding_summary.net_flow);

    // 4. 平仓
    println!("\n===== 步骤4: 平仓 =====");
    let close_cmd = ClosePositionCommand {
        trader_id: "trader123".to_string(),
        symbol: Symbol::new("BTCUSDT"),
        position_side: PositionSide::Long,
        order_type: OrderType::Market,
        quantity: Some(Quantity::from_f64(1.0)),
        price: None,
    };
    let close_result = workflow.execute(TradeAction::ClosePosition(close_cmd)).await?;

    if let WorkflowResult::PositionClosed(res) = close_result {
        println!("平仓成功，已实现盈亏: {}", res.realized_pnl);
    }

    Ok(())
}
```

## 总结

新实现的组件完整覆盖了XPDL流程定义：

1. **工作流编排器** - 统一路由所有交易动作
2. **三级强平机制** - 市场强平 → 保险基金 → ADL
3. **资金费率结算** - 支持4h/8h动态间隔
4. **事件系统** - 完整的领域事件发布
5. **保证金管理** - 追加/减少/自动追加

所有组件都使用中文注释，符合低时延优化要求。
