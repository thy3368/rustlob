# PrepOrder 状态机文档

## 文档概述

本文档详细说明 `PrepOrder` 的状态定义、状态转换规则、以及各状态下的行为特征。`PrepOrder` 是永续合约交易系统中的核心订单类型，负责管理开仓、平仓、修改和取消等订单生命周期。

**文档版本**: 1.0.0
**最后更新**: 2025-12-23
**相关代码**:
- `PrepOrder` 定义: `src/proc/prep_types.rs:7-21`
- `OrderStatus` 枚举: `src/proc/trading_prep_order_proc.rs:412-443`

---

## 1. 订单状态定义

### 1.1 状态枚举

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderStatus {
    /// 等待提交
    Pending = 1,
    /// 已提交
    Submitted = 2,
    /// 部分成交
    PartiallyFilled = 3,
    /// 完全成交
    Filled = 4,
    /// 已取消
    Cancelled = 5,
    /// 已拒绝
    Rejected = 6
}
```

### 1.2 状态分类

#### **初始状态**
- **Pending(1)**: 订单刚创建，等待提交到订单簿

#### **中间状态（活跃状态）**
- **Submitted(2)**: 订单已提交到 LOB（限价单处于待撮合状态）
- **PartiallyFilled(3)**: 订单部分成交，仍有未成交部分在订单簿中

#### **最终状态（终止状态）**
- **Filled(4)**: 订单已完全成交，不再处理
- **Cancelled(5)**: 订单已被用户或系统取消，不再处理
- **Rejected(6)**: 订单被风控或验证拒绝，不再处理

### 1.3 状态判断

```rust
impl OrderStatus {
    /// 判断是否为最终状态
    pub const fn is_final(self) -> bool {
        matches!(self, OrderStatus::Filled | OrderStatus::Cancelled | OrderStatus::Rejected)
    }
}
```

**使用场景**:
- 检查订单是否已完成处理
- 确定是否需要持续监听订单事件
- 判断是否可以进行后续操作（如修改、取消）

---

## 2. PrepOrder 数据结构

### 2.1 字段说明

```rust
pub struct PrepOrder {
    /// 订单唯一标识符
    pub order_id: OrderId,

    /// 交易对（如 BTC/USDT）
    pub trading_pair: TradingPair,

    /// 订单方向（Buy=做多, Sell=做空）
    pub side: Side,

    /// 订单类型（Market=市价, Limit=限价）
    pub order_type: OrderType,

    /// 订单数量
    pub quantity: Quantity,

    /// 限价单价格（市价单为 None）
    pub price: Option<Price>,

    /// 已成交数量（0 ≤ filled_quantity ≤ quantity）
    pub filled_quantity: Quantity,

    /// 当前订单状态
    pub status: OrderStatus,

    /// 订单创建时间戳（毫秒）
    pub created_at: u64,

    /// 冻结的保证金金额（订单取消时归还）
    pub frozen_margin: Price,
}
```

### 2.2 关键字段解析

#### **frozen_margin（冻结保证金）**

该字段在订单生命周期中的变化规律：

| 阶段 | 值 | 说明 |
|------|-----|------|
| **创建时** | 0 | 订单初始化，保证金未冻结 |
| **验证通过** | > 0 | 根据订单数量和杠杆计算的保证金被冻结 |
| **完全成交** | 0 | 订单成交，冻结的保证金转为持仓保证金，字段清零 |
| **部分成交** | > 0（减少）| 保留未成交部分需要的保证金 |
| **已取消** | 0 | 冻结的保证金归还给账户余额 |
| **已拒绝** | 0 | 冻结的保证金归还给账户余额 |

#### **filled_quantity（已成交数量）**

变化规律：

```
Pending → 0
Submitted → 0
PartiallyFilled → 0 < filled_quantity < quantity
Filled → filled_quantity = quantity
```

---

## 3. 状态转换规则

### 3.1 完整状态转换图

```
                    ┌─────────────────────┐
                    │  订单创建            │
                    │  Pending(1)         │
                    └──────────┬──────────┘
                               │
                    ┌──────────┴──────────┐
                    │                     │
                    ▼                     ▼
            ┌──────────────┐      ┌──────────────┐
            │ 命令验证失败  │      │ 风控验证     │
            │              │      │              │
            └──────┬───────┘      └────────┬─────┘
                   │                      │
                   │ 违反规则              │ 余额不足/杠杆过高
                   │                      │
                   ▼                      ▼
            ┌──────────────────────────────────┐
            │    Rejected(6) ✓ [最终态]        │
            │ 归还冻结保证金                    │
            └──────────────────────────────────┘


            ┌──────────────────┐
            │ 验证通过          │
            │ 冻结保证金        │
            └────────┬─────────┘
                     │
                     ▼
            ┌────────────────────┐
            │ 提交到订单簿        │
            │ Submitted(2)       │
            └────────┬───────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
   无成交      部分成交      完全成交
        │            │            │
   继续等待    ▼            ▼
        │  ┌──────────────┐  ┌──────────────┐
        │  │PartiallyFill│  │   Filled(4)  │
        │  │ed(3)        │  │  ✓[最终态]   │
        │  └────────┬─────┘  └──────────────┘
        │           │
        │      ┌────┴─────┐
        │      │           │
    继续撮合  用户取消
        │      │
        ▼      ▼
   PartiallyFilled → Cancelled(5) ✓ [最终态]
                  或 Filled(4) ✓ [最终态]

(任何活跃状态下可取消)
Submitted(2) ──┐
               ├──→ Cancelled(5) ✓ [最终态]
PartiallyFilled(3) ┘
```

### 3.2 按状态的转换规则

#### **Pending → 其他状态**

| 目标状态 | 触发条件 | 冻结保证金 | 备注 |
|---------|---------|----------|------|
| **Rejected** | 命令验证失败或风控拒绝 | 0（不冻结） | 最终态 |
| **Submitted** | 验证通过，无立即成交 | > 0 | 限价单或市价单无可成交 |
| **Filled** | 验证通过，全部成交 | 0 | 市价单或限价单全部成交 |
| **PartiallyFilled** | 验证通过，部分成交 | > 0 | 成交部分的保证金已转为持仓 |

#### **Submitted → 其他状态**

| 目标状态 | 触发条件 | 保证金处理 | 备注 |
|---------|---------|----------|------|
| **Filled** | 后续成交导致全部成交 | 释放冻结保证金 | 最终态 |
| **PartiallyFilled** | 部分成交 | 更新冻结金额 | 只保留未成交部分 |
| **Cancelled** | 用户取消订单 | 归还冻结保证金 | 最终态 |

#### **PartiallyFilled → 其他状态**

| 目标状态 | 触发条件 | 保证金处理 | 备注 |
|---------|---------|----------|------|
| **Filled** | 继续撮合致全部成交 | 释放冻结保证金 | 最终态 |
| **PartiallyFilled** | 继续部分成交 | 更新冻结金额 | 保留未成交部分 |
| **Cancelled** | 用户取消订单 | 归还冻结保证金 | 最终态 |

#### **已达最终态**

| 状态 | 进一步转换 | 说明 |
|------|----------|------|
| **Filled** | ❌ 无 | 订单完全成交，不可转换 |
| **Cancelled** | ❌ 无 | 订单已取消，不可转换 |
| **Rejected** | ❌ 无 | 订单已拒绝，不可转换 |

---

## 4. 订单类型与状态转换

### 4.1 限价单（Limit Order）

限价单按指定价格或更优价格成交，可能产生多种状态转换：

```
Limit Order 状态转换流程：

Pending
   ↓
验证（风控、余额）
   ↓
   ├─ 失败 → Rejected ✓
   │
   └─ 通过：冻结保证金
      ↓
      ├─ 完全成交（LOB 中有匹配） → Filled ✓
      │
      ├─ 无成交（LOB 中无匹配） → Submitted
      │                          ↓
      │                    等待后续撮合
      │                          ↓
      │                    ├─ 部分成交 → PartiallyFilled
      │                    │           ↓
      │                    │    ├─ 继续成交 → Filled ✓
      │                    │    │
      │                    │    ├─ 用户取消 → Cancelled ✓
      │                    │    │
      │                    │    └─ 继续部分成交 → PartiallyFilled
      │                    │
      │                    ├─ 全部成交 → Filled ✓
      │                    │
      │                    └─ 用户取消 → Cancelled ✓
      │
      └─ 部分成交（LOB 中部分匹配）→ PartiallyFilled
                                  ↓
                          同上（继续等待）
```

**特点**：
- 可能在 Submitted 或 PartiallyFilled 状态停留（等待撮合）
- 用户可以在活跃状态（Submitted/PartiallyFilled）取消
- 保证金根据未成交数量动态调整

### 4.2 市价单（Market Order）

市价单以最优价格立即成交或被拒绝：

```
Market Order 状态转换流程：

Pending
   ↓
验证（风控、余额、流动性）
   ↓
   ├─ 验证失败 → Rejected ✓
   │
   └─ 验证通过：冻结保证金
      ↓
      尝试在 LOB 中快速成交
      ↓
      ├─ 完全成交 → Filled ✓
      │           (frozen_margin = 0)
      │
      ├─ 部分成交 → PartiallyFilled
      │ (剩余部分挂单)  ↓
      │           可被用户取消 → Cancelled ✓
      │
      └─ 无可成交量 → Submitted
                 (作为限价单挂单)
                 ↓
            同限价单流程
```

**特点**：
- 优先尝试立即成交
- 通常不在 Submitted 状态停留（会立即成交或转为 PartiallyFilled）
- 成交速度快，但保证金冻结可能更高

---

## 5. 关键场景的状态转换

### 5.1 开仓订单（OpenPosition）

#### 场景 1: 限价做多订单 - 立即成交

```
事件序列：
1. 发送命令 → Pending
2. 验证通过，冻结保证金
3. LOB 有卖单匹配 → 成交
4. frozen_margin 清零 → Filled ✓

准备金变化：
- 开始: available = 1000 USDT
- 冻结: available = 1000 - 保证金 = 950 USDT
- 成交: 保证金转为持仓占用，available = 1000
```

#### 场景 2: 限价做多订单 - 部分成交后继续成交

```
事件序列：
1. 发送命令 → Pending
2. 验证通过，冻结保证金（假设 100 USDT）
3. LOB 部分成交（50%）
   状态: PartiallyFilled
   filled_quantity = 50
   frozen_margin = 50 USDT（保留未成交部分）
4. 后续成交继续
5. 全部成交 → Filled ✓
   frozen_margin = 0

保证金变化：
- 初始冻结: 100 USDT
- 部分成交后: 保留 50 USDT（未成交部分）
- 全部成交: 释放所有冻结，转为持仓占用
```

#### 场景 3: 市价做空订单

```
事件序列：
1. 发送命令 → Pending
2. 验证通过，冻结保证金
3. 市场快速匹配
   - 情况 A: 完全成交 → Filled ✓
   - 情况 B: 部分成交 → PartiallyFilled
             → 可继续成交或被取消

杠杆影响:
- 10倍杠杆: frozen_margin = 订单金额 / 10
- 1倍杠杆: frozen_margin = 订单金额
```

### 5.2 取消订单（CancelOrder）

#### 场景 1: 取消 Submitted 状态订单

```
事件序列：
1. 订单状态: Submitted
   - frozen_margin = 100 USDT
   - filled_quantity = 0
2. 用户发起取消
3. 从 LOB 移除订单
4. 状态转换: Cancelled ✓
5. 归还冻结保证金: available + 100 USDT
```

#### 场景 2: 取消 PartiallyFilled 状态订单

```
事件序列：
1. 订单状态: PartiallyFilled
   - quantity = 100
   - filled_quantity = 60
   - frozen_margin = 40 USDT（未成交部分）
2. 用户发起取消
3. 从 LOB 移除未成交部分
4. 状态转换: Cancelled ✓
5. 已成交部分变为持仓（不可撤销）
6. 归还冻结保证金: available + 40 USDT
```

#### 场景 3: 尝试取消已成交订单

```
事件序列：
1. 订单状态: Filled ✓（最终态）
2. 用户发起取消
3. 系统检查: is_final() = true
4. 拒绝操作，返回错误信息

保证金状态：无变化（已转为持仓占用）
```

### 5.3 强平流程中的 PrepOrder

#### 场景: 强平时的订单状态转换

```
强平流程（Liquidation）:

1. 触发强平条件（风险率过高）
2. 生成强平订单 → Pending
3. 验证通过 → Submitted
4. 强平市价单快速成交 → Filled ✓

特点：
- 强平订单优先级最高
- 通常以市价立即成交
- frozen_margin 直接清零
- 不允许用户取消
- 最终转换为 Filled（保证减仓）
```

---

## 6. 状态转换的约束条件

### 6.1 转换前置条件

#### **从 Pending 转换**
- 必须完成命令验证（参数合法性）
- 必须通过风控检查（余额、杠杆、头寸）
- 必须通过交易对验证

#### **从 Submitted 转换**
- 订单必须在 LOB 中存在
- 必须有匹配的对手方订单（成交）或用户取消指令

#### **从 PartiallyFilled 转换**
- 订单未成交部分必须在 LOB 中
- 后续成交必须基于 LOB 撮合结果

#### **禁止转换**
```rust
if status.is_final() {
    return Err(OrderError::FinalStateNoTransition);
}
```

### 6.2 状态转换的原子性

状态转换必须与以下操作原子执行：

| 操作 | 相关状态 | 说明 |
|------|---------|------|
| **冻结保证金** | Pending → Submitted/Filled/PartiallyFilled | 防止超额冻结 |
| **归还保证金** | Submitted/PartiallyFilled → Cancelled | 防止资金丢失 |
| **更新持仓** | → Filled/PartiallyFilled | 保持一致性 |
| **记录事件** | 所有转换 | 事件溯源记录 |

---

## 7. 事件溯源与状态持久化

### 7.1 状态变更事件

每个状态转换都会生成对应的事件：

```rust
pub enum OrderStatusChangeEvent {
    /// 订单创建事件
    OrderCreated {
        order_id: OrderId,
        trading_pair: TradingPair,
        side: Side,
        quantity: Quantity,
        price: Option<Price>,
        frozen_margin: Price,
        timestamp: u64,
    },

    /// 状态变更事件
    StatusChanged {
        order_id: OrderId,
        from_status: OrderStatus,
        to_status: OrderStatus,
        filled_quantity: Quantity,
        frozen_margin: Price,
        timestamp: u64,
    },

    /// 成交事件
    OrderFilled {
        order_id: OrderId,
        status: OrderStatus,
        filled_quantity: Quantity,
        avg_price: Price,
        timestamp: u64,
    },

    /// 取消事件
    OrderCancelled {
        order_id: OrderId,
        refunded_margin: Price,
        timestamp: u64,
    },

    /// 拒绝事件
    OrderRejected {
        order_id: OrderId,
        reason: String,
        timestamp: u64,
    },
}
```

### 7.2 事件持久化流程

```
┌──────────────────┐
│ 状态转换触发      │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ 验证新状态合法    │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ 生成状态变更事件  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ 更新内存中的状态  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ 事件写入事件日志  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ 返回成功响应      │
└──────────────────┘
```

---

## 8. 状态机的实现要点

### 8.1 状态转换的安全性检查

```rust
pub fn validate_transition(
    current: OrderStatus,
    target: OrderStatus,
) -> Result<(), OrderError> {
    // 检查是否为最终状态
    if current.is_final() {
        return Err(OrderError::FinalStateNoTransition);
    }

    // 检查转换是否合法
    match (current, target) {
        (OrderStatus::Pending, OrderStatus::Rejected) => Ok(()),
        (OrderStatus::Pending, OrderStatus::Submitted) => Ok(()),
        (OrderStatus::Pending, OrderStatus::Filled) => Ok(()),
        (OrderStatus::Pending, OrderStatus::PartiallyFilled) => Ok(()),

        (OrderStatus::Submitted, OrderStatus::Filled) => Ok(()),
        (OrderStatus::Submitted, OrderStatus::PartiallyFilled) => Ok(()),
        (OrderStatus::Submitted, OrderStatus::Cancelled) => Ok(()),

        (OrderStatus::PartiallyFilled, OrderStatus::Filled) => Ok(()),
        (OrderStatus::PartiallyFilled, OrderStatus::PartiallyFilled) => Ok(()),
        (OrderStatus::PartiallyFilled, OrderStatus::Cancelled) => Ok(()),

        _ => Err(OrderError::InvalidStateTransition),
    }
}
```

### 8.2 保证金管理与状态同步

```rust
fn handle_state_transition(
    mut order: PrepOrder,
    new_status: OrderStatus,
    filled_qty: Quantity,
) -> Result<PrepOrder, OrderError> {
    // 更新已成交数量
    order.filled_quantity = filled_qty;

    // 根据新状态调整冻结保证金
    match new_status {
        OrderStatus::Filled => {
            // 完全成交，释放所有冻结保证金
            order.frozen_margin = Price::from_raw(0);
        }
        OrderStatus::PartiallyFilled => {
            // 部分成交，保留未成交部分的保证金
            let unfilled_ratio =
                (order.quantity - filled_qty) / order.quantity;
            order.frozen_margin = order.frozen_margin * unfilled_ratio;
        }
        OrderStatus::Cancelled | OrderStatus::Rejected => {
            // 取消或拒绝，保证金在外层处理（归还）
            // 这里仅标记为已处理
            order.frozen_margin = Price::from_raw(0);
        }
        _ => {}
    }

    // 转换状态
    order.status = new_status;

    Ok(order)
}
```

### 8.3 并发安全性

```rust
// 使用锁确保状态转换原子性
pub async fn transition_order_state(
    order_id: OrderId,
    new_status: OrderStatus,
) -> Result<PrepOrder, OrderError> {
    // 获取锁（防止并发修改）
    let mut order = self.order_lock.write().await;

    // 验证转换合法性
    validate_transition(order.status, new_status)?;

    // 执行转换
    let updated_order = handle_state_transition(
        order.clone(),
        new_status,
        order.filled_quantity,
    )?;

    // 持久化
    self.repo.save(&updated_order).await?;

    // 发布事件
    self.event_bus.publish(StatusChangeEvent {
        order_id,
        from_status: order.status,
        to_status: new_status,
    })?;

    *order = updated_order;
    Ok(order)
}
```

---

## 9. 常见状态转换错误

### 9.1 错误场景

| 错误 | 场景 | 原因 |
|------|------|------|
| **FinalStateTransition** | 尝试修改已成交订单 | 最终态不可转换 |
| **InvalidStatusTransition** | Submitted → Pending | 状态只能向后转换 |
| **InsufficientMargin** | 冻结保证金失败 | 账户余额不足 |
| **OrderNotFound** | 取消不存在的订单 | 订单 ID 无效 |
| **DoubleCancel** | 重复取消同一订单 | 订单已处于最终态 |

### 9.2 错误处理

```rust
pub fn cancel_order(order_id: OrderId) -> Result<(), OrderError> {
    let order = self.repo.find_by_id(order_id)?;

    // 检查订单是否存在
    if order.is_none() {
        return Err(OrderError::OrderNotFound);
    }

    let order = order.unwrap();

    // 检查是否为最终态
    if order.status.is_final() {
        return Err(OrderError::CannotCancelFinalOrder(order.status));
    }

    // 执行取消
    let cancelled_order = order.with_status(OrderStatus::Cancelled);
    self.repo.save(&cancelled_order)?;

    // 归还保证金
    self.balance_repo.refund_margin(
        order.frozen_margin
    )?;

    Ok(())
}
```

---

## 10. 状态机测试用例

### 10.1 基础状态转换测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pending_to_filled() {
        let mut order = create_pending_order();
        assert_eq!(order.status, OrderStatus::Pending);

        order.status = OrderStatus::Filled;
        order.filled_quantity = order.quantity;
        order.frozen_margin = Price::from_raw(0);

        assert_eq!(order.status, OrderStatus::Filled);
        assert!(order.status.is_final());
    }

    #[test]
    fn test_submitted_to_partially_filled() {
        let mut order = create_submitted_order();
        assert_eq!(order.status, OrderStatus::Submitted);

        order.status = OrderStatus::PartiallyFilled;
        order.filled_quantity = Quantity::from_f64(50.0);

        assert!(order.filled_quantity > Quantity::from_raw(0));
        assert!(!order.status.is_final());
    }

    #[test]
    fn test_partially_filled_to_cancelled() {
        let mut order = create_partially_filled_order();
        let original_margin = order.frozen_margin;

        order.status = OrderStatus::Cancelled;
        order.frozen_margin = Price::from_raw(0);

        assert!(order.status.is_final());
        assert_eq!(original_margin > Price::from_raw(0), true);
    }

    #[test]
    fn test_invalid_transition() {
        let order = create_filled_order();
        assert!(order.status.is_final());
        // 无法从 Filled 转换到其他状态
    }

    #[test]
    fn test_frozen_margin_calculation() {
        let mut order = create_pending_order();
        let initial_margin = Price::from_f64(100.0);
        order.frozen_margin = initial_margin;

        // 部分成交 50%
        order.filled_quantity = order.quantity / Quantity::from_raw(2);
        order.frozen_margin = initial_margin / Price::from_raw(2);

        assert_eq!(order.frozen_margin, Price::from_f64(50.0));
    }
}
```

### 10.2 集成测试

```rust
#[cfg(test)]
mod integration_tests {
    #[tokio::test]
    async fn test_complete_order_lifecycle() {
        // 1. 创建订单
        let order = create_order();
        assert_eq!(order.status, OrderStatus::Pending);

        // 2. 提交到 LOB
        let order = submit_to_lob(&order).await;
        assert_eq!(order.status, OrderStatus::Submitted);

        // 3. 部分成交
        let order = process_partial_fill(&order, 50).await;
        assert_eq!(order.status, OrderStatus::PartiallyFilled);

        // 4. 继续成交
        let order = process_partial_fill(&order, 50).await;
        assert_eq!(order.status, OrderStatus::Filled);
        assert!(order.status.is_final());
    }

    #[tokio::test]
    async fn test_order_cancellation_flow() {
        let mut order = create_and_submit_order().await;
        assert_eq!(order.status, OrderStatus::Submitted);

        let original_margin = order.frozen_margin;

        // 取消订单
        order.status = OrderStatus::Cancelled;
        order.frozen_margin = Price::from_raw(0);

        // 验证最终状态
        assert!(order.status.is_final());
        assert!(original_margin > Price::from_raw(0));
    }
}
```

---

## 11. 相关代码索引

| 概念 | 文件位置 | 行号 |
|------|---------|------|
| PrepOrder 结构 | `src/proc/prep_types.rs` | 7-21 |
| OrderStatus 枚举 | `src/proc/trading_prep_order_proc.rs` | 412-425 |
| 状态判断方法 | `src/proc/trading_prep_order_proc.rs` | 439-443 |
| 开仓命令 | `src/proc/trading_prep_order_proc.rs` | 77-97 |
| 开仓结果 | `src/proc/trading_prep_order_proc.rs` | 450-460 |
| 状态转换实现 | `src/proc/trading_prep_order_proc_impl.rs` | - |
| 强平流程 | `src/proc/liquidation_proc.rs` | - |

---

## 12. 状态机设计原则

### 12.1 核心设计原则

1. **单向转换**: 订单状态只能向前转换（从创建到最终），不能回溯
2. **最终态不可逆**: Filled/Cancelled/Rejected 是终止状态
3. **保证金严格管理**: 冻结和归还必须原子执行
4. **事件溯源**: 所有转换都记录事件用于审计
5. **并发安全**: 状态转换需要锁保护

### 12.2 设计优势

| 优势 | 说明 |
|------|------|
| **清晰性** | 有限的状态集合，易于理解和维护 |
| **可靠性** | 严格的转换规则防止非法状态 |
| **可追溯性** | 事件记录提供完整审计线索 |
| **并发安全** | 原子操作保证数据一致性 |
| **扩展性** | 易于添加新状态或转换规则 |

---

## 13. 常见问题 (FAQ)

### Q1: 为什么有 Submitted 和 PartiallyFilled 两种中间状态？

**A**:
- **Submitted**: 限价订单已挂入 LOB，完全未成交
- **PartiallyFilled**: 订单已有部分成交，剩余部分在 LOB 中
- 区分两者方便统计和查询（如计算部分成交的平均价格）

### Q2: frozen_margin 何时释放？

**A**:
- **成交时**: 成交数量对应的保证金转为持仓占用，不再冻结
- **取消时**: 所有冻结保证金立即归还（事务性）
- **拒绝时**: 冻结保证金不再产生（从未实际冻结）

### Q3: 为什么订单可以从 Pending 直接转为 Filled？

**A**:
- 市价单或有充足流动性的限价单可能立即成交
- 无需经过 Submitted 状态（无需在 LOB 中停留）

### Q4: PartiallyFilled 订单能修改吗？

**A**:
- **不能修改数量或价格**（已部分成交）
- **可以取消**（未成交部分）
- 修改会影响保证金计算，增加复杂性

### Q5: 能否实现"部分成交后自动取消"的逻辑？

**A**:
```rust
// 可以通过 IOC (Immediate Or Cancel) 实现
pub enum TimeInForce {
    IOC = 2,  // 立即成交或取消
    FOK = 3,  // 全部成交或取消
}
```
- IOC: 成交部分后自动取消未成交部分
- FOK: 全部成交或全部取消（不允许 PartiallyFilled）

---

## 附录：状态机快速参考

### 状态代码表

```
1 = Pending（待提交）
2 = Submitted（已提交）
3 = PartiallyFilled（部分成交）
4 = Filled（完全成交） ✓ 最终态
5 = Cancelled（已取消） ✓ 最终态
6 = Rejected（已拒绝） ✓ 最终态
```

### 合法转换矩阵

```
     1  2  3  4  5  6
1    ✗  ✓  ✓  ✓  ✗  ✓
2    ✗  ✗  ✓  ✓  ✓  ✗
3    ✗  ✗  ✓  ✓  ✓  ✗
4    ✗  ✗  ✗  ✗  ✗  ✗
5    ✗  ✗  ✗  ✗  ✗  ✗
6    ✗  ✗  ✗  ✗  ✗  ✗

行: 当前状态  列: 目标状态
✓ = 合法转换  ✗ = 非法转换
```

### 状态特征总结

```
┌─────────────────┬─────────┬──────────────┬──────────┐
│ 状态            │ 代码    │ 类别         │ filled_qty│
├─────────────────┼─────────┼──────────────┼──────────┤
│ Pending         │ 1       │ 初始态       │ = 0      │
│ Submitted       │ 2       │ 活跃态       │ = 0      │
│ PartiallyFilled │ 3       │ 活跃态       │ ∈(0,qty) │
│ Filled          │ 4       │ 最终态 ✓     │ = qty    │
│ Cancelled       │ 5       │ 最终态 ✓     │ ≤ qty    │
│ Rejected        │ 6       │ 最终态 ✓     │ = 0      │
└─────────────────┴─────────┴──────────────┴──────────┘
```

---

**文档版本**: 1.0.0
**维护者**: 交易系统团队
**最后更新**: 2025-12-23
**下次审查**: 2026-03-23
