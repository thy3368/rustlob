# 交易员使用指南：如何使用 MatchingService 进行交易

## 概述

本指南展示了交易员如何使用 `MatchingService` 进行各种交易操作。`MatchingService` 是订单匹配引擎的核心服务，负责处理订单的提交、撮合和取消。

## 核心概念

### 1. 订单类型

#### 限价单（Limit Order）
- **定义**：指定价格和数量的订单
- **特点**：只在指定价格或更优价格成交
- **适用场景**：对价格敏感，愿意等待成交

#### 市价单（Market Order）
- **定义**：以当前市场最优价格立即成交的订单
- **特点**：优先保证成交，不保证价格
- **适用场景**：需要快速建仓或平仓

### 2. TimeInForce（订单有效期）

| 类型 | 全称 | 说明 | 适用场景 |
|------|------|------|----------|
| **GTC** | Good Till Cancel | 撤单前一直有效 | 长期挂单，等待理想价格 |
| **IOC** | Immediate Or Cancel | 立即成交，未成交部分取消 | 快速执行，不想挂单 |
| **FOK** | Fill Or Kill | 全部成交或全部拒绝 | 大单执行，避免部分成交 |
| **PostOnly** | Post Only | 只做 Maker，不吃单 | 做市商，赚取手续费返佣 |
| **GTD** | Good Till Date | 有效至指定时间 | 定时策略 |

### 3. 订单状态

```
Initial → Pending → PartiallyFilled → Filled
                 ↘ Cancelled
                 ↘ Rejected
                 ↘ Expired
```

- **Initial**: 初始状态
- **Pending**: 已挂单，等待成交
- **PartiallyFilled**: 部分成交
- **Filled**: 完全成交
- **Cancelled**: 已取消
- **Rejected**: 被拒绝（如 PostOnly 会立即成交）
- **Expired**: 已过期（GTD）

## 交易场景示例

### 场景1：限价买单（GTC）- 长期挂单

**需求**：交易员想以 50000 USDT 的价格买入 1.5 BTC，如果无法立即成交，愿意等待。

```rust
let mut trader = Trader::new(trader_id);

let result = trader.place_limit_buy_gtc(
    &mut matching_service,
    Symbol::from("BTCUSDT"),
    50000,  // 价格: 50000 USDT
    1500,   // 数量: 1.5 BTC
);

match result {
    Ok(response) => {
        if let SpotCommandResult::LimitOrder { order_id, status, .. } = response.result {
            match status {
                OrderStatus::Filled => println!("✅ 订单已完全成交"),
                OrderStatus::PartiallyFilled => println!("⚠️ 订单部分成交，剩余挂单中"),
                OrderStatus::Pending => println!("⏳ 订单已挂单，等待成交"),
                _ => {}
            }
        }
    }
    Err(e) => println!("❌ 下单失败: {}", e),
}
```

**执行流程**：
1. 系统检查账户余额（需要冻结 50000 * 1.5 = 75000 USDT）
2. 尝试与订单簿中的卖单匹配
3. 如果有匹配：部分或全部成交
4. 如果有剩余：挂单到订单簿，等待后续成交

### 场景2：限价卖单（PostOnly）- 做市商策略

**需求**：做市商想以 50100 USDT 的价格卖出 2.0 BTC，但只想做流动性提供者（Maker），不想吃单（Taker）。

```rust
let result = trader.place_limit_sell_post_only(
    &mut matching_service,
    Symbol::from("BTCUSDT"),
    50100,  // 价格: 50100 USDT
    2000,   // 数量: 2.0 BTC
);

match result {
    Ok(response) => {
        if let SpotCommandResult::LimitOrder { status, .. } = response.result {
            match status {
                OrderStatus::Pending => println!("✅ 订单已挂单，等待成交"),
                OrderStatus::Rejected => println!("❌ 订单被拒绝（会立即成交）"),
                _ => {}
            }
        }
    }
    Err(e) => println!("❌ 下单失败: {}", e),
}
```

**PostOnly 特点**：
- ✅ 如果订单不会立即成交：挂单成功
- ❌ 如果订单会立即成交：拒绝订单（避免支付 Taker 手续费）

**适用场景**：
- 做市商赚取手续费返佣
- 高频交易策略
- 避免支付 Taker 手续费

### 场景3：限价买单（IOC）- 快速执行

**需求**：交易员想立即买入，但不想挂单等待。

```rust
let result = trader.place_limit_buy_ioc(
    &mut matching_service,
    Symbol::from("BTCUSDT"),
    50050,  // 价格: 50050 USDT
    500,    // 数量: 0.5 BTC
);

match result {
    Ok(response) => {
        if let SpotCommandResult::LimitOrder {
            status,
            filled_quantity,
            remaining_quantity,
            ..
        } = response.result {
            match status {
                OrderStatus::Filled => {
                    println!("✅ 订单已完全成交: {} BTC", filled_quantity);
                }
                OrderStatus::Cancelled => {
                    println!("⚠️ 部分成交: {} BTC，剩余 {} BTC 已取消",
                        filled_quantity, remaining_quantity);
                }
                _ => {}
            }
        }
    }
    Err(e) => println!("❌ 下单失败: {}", e),
}
```

**IOC 特点**：
- 立即尝试成交
- 能成交多少就成交多少
- 未成交部分自动取消（不挂单）

**适用场景**：
- 快速建仓
- 不想挂单暴露意图
- 测试市场流动性

### 场景4：限价买单（FOK）- 全部成交或拒绝

**需求**：交易员想买入 3.0 BTC，但必须全部成交，否则不买。

```rust
let result = trader.place_limit_buy_fok(
    &mut matching_service,
    Symbol::from("BTCUSDT"),
    50200,  // 价格: 50200 USDT
    3000,   // 数量: 3.0 BTC
);

match result {
    Ok(response) => {
        if let SpotCommandResult::LimitOrder { status, .. } = response.result {
            match status {
                OrderStatus::Filled => {
                    println!("✅ 订单已全部成交");
                }
                OrderStatus::Rejected => {
                    println!("❌ 订单被拒绝（无法全部成交）");
                }
                _ => {}
            }
        }
    }
    Err(e) => println!("❌ 下单失败: {}", e),
}
```

**FOK 特点**：
- 全部成交 → 成功
- 无法全部成交 → 拒绝（不会部分成交）

**适用场景**：
- 大单执行
- 避免部分成交的风险
- 套利交易（需要精确数量）

### 场景5：市价买单 - 快速建仓

**需求**：交易员想立即买入 1.0 BTC，不关心具体价格，但设置价格保护。

```rust
let result = trader.place_market_buy(
    &mut matching_service,
    Symbol::from("BTCUSDT"),
    1000,           // 数量: 1.0 BTC
    Some(51000),    // 价格保护: 最高 51000 USDT
);

match result {
    Ok(response) => {
        if let SpotCommandResult::MarketOrder {
            status,
            filled_quantity,
            trades,
            ..
        } = response.result {
            println!("✅ 市价单成交: {} BTC，成交笔数: {}",
                filled_quantity, trades.len());
        }
    }
    Err(e) => println!("❌ 下单失败: {}", e),
}
```

**市价单特点**：
- 立即以市场最优价格成交
- 不保证价格，保证成交
- 可设置价格保护（price_limit）

**价格保护机制**：
- **买单**：`price_limit` = 最高愿意支付的价格
- **卖单**：`price_limit` = 最低愿意接受的价格
- 超过限制的订单不会成交

**适用场景**：
- 紧急建仓
- 止损平仓
- 快速响应市场变化

### 场景6：市价卖单（FOK）- 全部卖出或不卖

**需求**：交易员想立即卖出 0.5 BTC，但必须全部卖出。

```rust
let result = trader.place_market_sell_fok(
    &mut matching_service,
    Symbol::from("BTCUSDT"),
    500,            // 数量: 0.5 BTC
    Some(49000),    // 价格保护: 最低 49000 USDT
);

match result {
    Ok(response) => {
        if let SpotCommandResult::MarketOrder { status, .. } = response.result {
            match status {
                OrderStatus::Filled => {
                    println!("✅ 市价单已全部成交");
                }
                OrderStatus::Rejected => {
                    println!("❌ 市价单被拒绝（无法全部成交或价格超限）");
                }
                _ => {}
            }
        }
    }
    Err(e) => println!("❌ 下单失败: {}", e),
}
```

**市价单 + FOK 特点**：
- 必须全部成交
- 如果市场深度不足 → 拒绝
- 如果价格超过保护限制 → 拒绝

### 场景7：取消订单

**需求**：交易员想取消之前挂的订单。

```rust
let result = trader.cancel_order(&mut matching_service, order_id);

match result {
    Ok(response) => {
        if let SpotCommandResult::CancelOrder { status, .. } = response.result {
            println!("✅ 订单已成功取消");
        }
    }
    Err(SpotCommandError::Common(CommonError::OrderNotFound { .. })) => {
        println!("❌ 订单不存在（可能已成交或已取消）");
    }
    Err(e) => println!("❌ 取消失败: {}", e),
}
```

**取消订单注意事项**：
- 只能取消 `Pending` 或 `PartiallyFilled` 状态的订单
- 已成交（`Filled`）的订单无法取消
- 取消成功后，冻结的资金会解冻

## 错误处理

### 常见错误类型

#### 1. 余额不足

```rust
Err(SpotCommandError::Common(CommonError::InsufficientBalance {
    required,
    available
})) => {
    println!("❌ 余额不足: 需要 {}, 可用 {}", required, available);
}
```

**解决方案**：
- 减少订单数量
- 充值账户
- 取消其他挂单释放冻结资金

#### 2. PostOnly 订单会立即成交

```rust
Ok(response) => {
    if let SpotCommandResult::LimitOrder { status, .. } = response.result {
        if status == OrderStatus::Rejected {
            println!("❌ PostOnly 订单被拒绝（会立即成交）");
        }
    }
}
```

**解决方案**：
- 调整价格（买单降价，卖单提价）
- 使用 GTC 代替 PostOnly

#### 3. FOK 订单无法全部成交

```rust
Ok(response) => {
    if let SpotCommandResult::LimitOrder { status, .. } = response.result {
        if status == OrderStatus::Rejected {
            println!("❌ FOK 订单被拒绝（无法全部成交）");
        }
    }
}
```

**解决方案**：
- 提高价格（买单提价，卖单降价）
- 减少订单数量
- 使用 IOC 代替 FOK

#### 4. 订单不存在

```rust
Err(SpotCommandError::Common(CommonError::OrderNotFound { order_id })) => {
    println!("❌ 订单不存在: ", order_id);
}
```

**原因**：
- 订单已成交
- 订单已取消
- 订单ID错误

## 幂等性保证

### Nonce 机制

每个命令都需要一个唯一的 `nonce`（Number used ONCE），确保命令只执行一次。

```rust
pub struct Trader {
    trader_id: TraderId,
    nonce_counter: u64,  // 递增的 nonce 计数器
}

impl Trader {
    fn next_nonce(&mut self) -> u64 {
        self.nonce_counter += 1;
        self.nonce_counter
    }
}
```

**幂等性保证**：
- 相同 `nonce` 的命令只执行一次
- 重复提交返回缓存的结果
- 防止网络重传导致重复下单

### 命令包装

```rust
let command = SpotCommand::LimitOrder { /* ... */ };
let idempotent_cmd = Command::new(self.next_nonce(), command);
let result = matching_service.handle(idempotent_cmd)?;
```

## 账户余额管理

### 冻结机制

下单时，系统会自动冻结所需资金：

**买单**：
```
冻结金额 = 价格 × 数量
冻结资产 = quote_asset (如 USDT)
```

**卖单**：
```
冻结金额 = 数量
冻结资产 = base_asset (如 BTC)
```

### 解冻时机

- **订单成交**：冻结资金转为已用
- **订单取消**：冻结资金解冻
- **部分成交**：未成交部分解冻

## 性能优化建议

### 1. 批量操作

如果需要下多个订单，使用批量接口（如果支持）：

```rust
// 不推荐：逐个下单
for order in orders {
    matching_service.handle(order)?;
}

// 推荐：批量下单（如果支持）
matching_service.handle_batch(orders)?;
```

### 2. 异步处理

对于非关键路径，使用异步处理：

```rust
// 同步等待结果
let result = matching_service.handle(cmd)?;

// 异步提交（如果支持）
let future = matching_service.handle_async(cmd);
// 继续其他操作...
let result = future.await?;
```

### 3. 连接复用

保持长连接，避免频繁创建连接：

```rust
// 不推荐：每次创建新连接
fn place_order() {
    let matching_service = create_service();
    matching_service.handle(cmd)?;
}

// 推荐：复用连接
struct TradingSession {
    matching_service: MatchingService,
}

impl TradingSession {
    fn place_order(&mut self, cmd: Command) {
        self.matching_service.handle(cmd)?;
    }
}
```

## 最佳实践

### 1. 价格保护

市价单务必设置价格保护：

```rust
// ❌ 危险：无价格保护
trader.place_market_buy(
    &mut matching_service,
    symbol,
    1000,
    None,  // 可能以极高价格成交
);

// ✅ 安全：设置价格保护
trader.place_market_buy(
    &mut matching_service,
    symbol,
    1000,
    Some(51000),  // 最高 51000 USDT
);
```

### 2. 订单状态检查

下单后检查订单状态：

```rust
let result = trader.place_limit_buy_gtc(/* ... */)?;

if let SpotCommandResult::LimitOrder { order_id, status, .. } = result.result {
    match status {
        OrderStatus::Filled => {
            // 已完全成交，无需后续操作
        }
        OrderStatus::PartiallyFilled | OrderStatus::Pending => {
            // 部分成交或挂单中，可能需要监控或取消
            monitor_order(order_id);
        }
        OrderStatus::Rejected => {
            // 被拒绝，需要调整策略
            adjust_strategy();
        }
        _ => {}
    }
}
```

### 3. 错误重试

网络错误可以重试，业务错误不应重试：

```rust
fn place_order_with_retry(
    trader: &mut Trader,
    matching_service: &mut MatchingService,
    max_retries: u32,
) -> Result<CommandResponse, SpotCommandError> {
    let mut retries = 0;

    loop {
        match trader.place_limit_buy_gtc(/* ... */) {
            Ok(response) => return Ok(response),
            Err(SpotCommandError::Common(CommonError::Internal { .. })) => {
                // 内部错误，可以重试
                retries += 1;
                if retries >= max_retries {
                    return Err(/* ... */);
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                // 业务错误（如余额不足），不应重试
                return Err(e);
            }
        }
    }
}
```

### 4. 日志记录

记录所有交易操作：

```rust
fn place_order_with_logging(/* ... */) -> Result<CommandResponse, SpotCommandError> {
    log::info!("下单请求: trader={:?}, symbol={:?}, side={:?}, price={}, quantity={}",
        trader_id, symbol, side, price, quantity);

    let result = matching_service.handle(cmd);

    match &result {
        Ok(response) => {
            log::info!("下单成功: nonce={}, result={:?}", response.nonce, response.result);
        }
        Err(e) => {
            log::error!("下单失败: error={}", e);
        }
    }

    result
}
```

## 运行示例

```bash
# 运行完整的交易流程示例
cargo test test_trader_workflow -- --nocapture

# 运行错误处理示例
cargo test test_error_handling -- --nocapture

# 运行单个场景
cargo run --example trader_example
```

## 总结

### TimeInForce 选择指南

| 场景 | 推荐 TimeInForce | 原因 |
|------|------------------|------|
| 长期挂单 | GTC | 等待理想价格 |
| 快速执行 | IOC | 不想挂单 |
| 大单执行 | FOK | 避免部分成交 |
| 做市商 | PostOnly | 赚取手续费返佣 |
| 紧急平仓 | Market + IOC | 优先保证成交 |

### 订单类型选择指南

| 场景 | 推荐订单类型 | 原因 |
|------|--------------|------|
| 对价格敏感 | 限价单 | 控制成交价格 |
| 需要快速成交 | 市价单 | 优先保证成交 |
| 测试流动性 | 限价单 + IOC | 不挂单，快速反馈 |
| 套利交易 | 限价单 + FOK | 精确数量，避免风险 |

### 风险提示

1. **市价单风险**：可能以极端价格成交，务必设置价格保护
2. **PostOnly 风险**：可能被拒绝，需要调整价格
3. **FOK 风险**：可能被拒绝，需要评估市场深度
4. **余额管理**：注意冻结资金，避免余额不足
5. **幂等性**：确保 nonce 唯一，避免重复下单

---

**文档版本**: v1.0
**创建日期**: 2025-12-10
**适用项目**: RustLOB - 低延迟订单簿系统
