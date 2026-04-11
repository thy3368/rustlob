---
name: define_bdd_case_4_use_case
description: "Use when defining BDD acceptance test cases for use cases. Keywords: bdd, use case, given when then, acceptance test, scenario, handler, domain event, 测试用例, 验收测试, 场景"
user-invocable: true
---

# BDD Use Case Acceptance Tests

> **Purpose**: Define BDD test cases that verify use case handlers work correctly

## Rule Mapping

| BDD Element | Rust Handler Element |
|-------------|---------------------|
| `scenario` | `CmdHandlerForUpdate2` handler name |
| `given` | `GivenStateSet` - preconditions state |
| `when` | Specific `Command` to execute |
| `then` | `ThenStateSet` - expected domain events/state changes |
| `bdd_case` | Cartesian product of given × when穷举 |

## Use Case Handler Trait

Based on `CmdHandlerForUpdate2` from `base_types/src/handler/handler_update2.rs`:

```rust
pub trait CmdHandlerForUpdate2: ApplyCommandChanges2 + Send + Sync {
    type Command;
    type Reply;
    type GivenStateSet;
    type ThenStateSet: DomainEventSet;
    type Error;
}
```

## BDD Case Definition Template

```rust
#[bdd_test(
    feature = "<业务领域>",
    scenario = "<handler_name>",        // 对应 handler 实现名称
    given(<given_state_1>),             // GivenStateSet 状态
    when = "<specific_command>",        // 具体执行的 Command
    then(<expected_event_1>, <expected_event_2>),  // ThenStateSet 期望的 domain events
    tags(<domain>, <handler>),
    priority = "N"
)]
fn test_<scenario>_<given>_<when>() {
    // Given: 构建 GivenStateSet
    let given_state = build_given_state(...);
    
    // When: 执行 command
    let result = handler.apply_command_and_collect_changes(&cmd, given_state);
    
    // Then: 验证 ThenStateSet
    assert!(result.is_ok());
    let changes = result.unwrap();
    verify_domain_events(changes, &[expected_event_1, expected_event_2]);
}
```

## Example: Place Order Handler

```rust
// Handler 定义
struct PlaceOrderHandler;

impl CmdHandlerForUpdate2 for PlaceOrderHandler {
    type Command = PlaceOrderCmd;
    type Reply = PlaceOrderReply;
    type GivenStateSet = AccountState;      // 账户状态
    type ThenStateSet = OrderPlacedEvent;   // 订单成交事件
    type Error = OrderError;
}

// BDD Case: 正常下单
#[bdd_test(
    feature = "订单管理",
    scenario = "place_order_handler",
    given(账户有足够余额, 账户未冻结),
    when = "提交限价买单",
    then(订单已创建, 余额已冻结, 成交事件已发布),
    tags(order, place, happy_path),
    priority = "5"
)]
fn test_place_order_with_sufficient_balance() {
    // Given: 构建账户状态
    let account = AccountBuilder::new()
        .balance(Decimal::from(10000))
        .status(Active)
        .build();
    
    // When: 提交买单
    let cmd = PlaceOrderCmd::new(
        TraderId::from("trader_001"),
        Symbol::from("BTCUSDT"),
        Side::Buy,
        Price::from(50000),
        Quantity::from(0.1),
    );
    
    // Then: 验证结果
    let result = handler.apply_command_and_collect_changes(&cmd, account);
    assert!(result.is_ok());
}

// BDD Case: 余额不足
#[bdd_test(
    feature = "订单管理",
    scenario = "place_order_handler",
    given(账户余额不足),
    when = "提交限价买单",
    then(订单创建失败, 余额未变化, 错误事件已发布),
    tags(order, place, insufficient_balance),
    priority = "4"
)]
fn test_place_order_insufficient_balance() {
    // Given: 余额不足的账户
    let account = AccountBuilder::new()
        .balance(Decimal::from(100))  // 不足
        .build();
    
    // When
    let cmd = PlaceOrderCmd::new(...);
    
    // Then: 验证失败
    let result = handler.apply_command_and_collect_changes(&cmd, account);
    assert!(result.is_err());
}
```

## BDD Case Coverage Rule

```
given 穷举 × when 穷举 = bdd_case 集合
```

| Given States | When Commands | BDD Cases |
|--------------|---------------|-----------|
| [余额足够, 余额不足, 账户冻结] | [限价买单, 市价买单, 取消订单] | 9 个测试 |

## Trigger Phrases

- "定义 bdd 用例"
- "定义验收测试"
- "given when then 测试"
- "use case bdd test"
- "测试 handler 场景"

## Related Files

| File | Purpose |
|------|---------|
| `lib/common/bdd/src/lib.rs` | BDD proc-macro 实现 |
| `lib/common/bdd_core/src/lib.rs` | BDD 核心类型 |
| `lib/common/base_types/src/handler/handler_update2.rs` | CmdHandlerForUpdate2 trait |