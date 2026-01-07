# Core Architecture Deep Dive：系统的灵魂

**文档目标**：深度探讨 Clean Architecture 中 Core 层（Domain + Application Layer）的设计、价值、保护和演化策略。

---

## 目录

1. [Core 的本质定义](#core-的本质定义)
2. [Core 的四大特征](#core-的四大特征)
3. [Core 的价值维度](#core-的价值维度)
4. [Core 的腐化过程与警示](#core-的腐化过程与警示)
5. [Core 设计原则](#core-设计原则)
6. [Core 保护实践](#core-保护实践)
7. [Core 的演化路径](#core-的演化路径)
8. [Core 团队的组织](#core-团队的组织)
9. [Core 与 Adapter 的协作边界](#core-与-adapter-的协作边界)
10. [常见错误和反模式](#常见错误和反模式)
11. [实战案例：金融交易系统的 Core](#实战案例金融交易系统的-core)
12. [检查清单和度量指标](#检查清单和度量指标)

---

## Core 的本质定义

### 定义层级

```
整个 Clean Architecture 系统
├─ Core（系统的灵魂）
│  ├─ Domain Layer（实体和领域模型）
│  └─ Application Layer（用例和业务编排）
└─ Adapter（系统的肉体）
   ├─ Interface Layer（控制器和展示）
   └─ Infrastructure Layer（框架和驱动）
```

### Core 的数学定义

```
Core = {业务规则 + 用例编排} ∩ {无外部依赖的代码}

特征：
1. 完全隔离：不依赖任何框架、数据库、网络库
2. 纯函数式：相同输入 → 相同输出，无副作用
3. 可测试性：100% 代码覆盖不需要外部系统
4. 业务价值：浓缩了公司的核心竞争力
```

### 代码体积 vs 业务价值

```
代码体积分布：

非 Clean 架构（一体化）:
  Core 代码    ████ 20%     业务价值：30%
  Adapter 代码 ████████████ 80%    业务价值：70%

Clean 架构:
  Core 代码    ████ 15-20%   业务价值：85-90%
  Adapter 代码 ████████████ 80-85% 业务价值：10-15%

关键认知：
代码越少，价值越大 = Core 的本质特点
代码越多，价值越小 = Adapter 的本质特点
```

---

## Core 的四大特征

### 特征 1：业务规则的浓缩（Business Rule Concentration）

#### 1.1 定义

Core 不只是数据容器，而是业务智慧的结晶。每一行代码都代表一条经过验证的业务规则或决策逻辑。

#### 1.2 实例：订单核心规则的浓缩

```rust
// 这个 Order 结构体不仅仅是数据，是多年经营的规则集合

pub struct Order {
    // 基础身份标识
    id: OrderId,
    user_id: UserId,

    // 交易参数
    symbol: Symbol,           // 交易对（例如 BTCUSDT）
    price: Price,             // 下单价格
    quantity: Quantity,       // 下单数量
    side: OrderSide,          // 买卖方向
    time_in_force: TimeInForce, // 有效期类型（GTC, IOC, FOK 等）

    // 订单状态追踪
    status: OrderStatus,
    created_at: Timestamp,
    updated_at: Timestamp,

    // 成交信息
    filled_quantity: Quantity,
    fill_price: Option<Price>, // 平均成交价

    // 成本追踪（用于风控）
    initial_margin: Money,     // 初始保证金
    realized_pnl: Money,       // 已实现收益
    unrealized_pnl: Money,     // 未实现收益
}

impl Order {
    /// 业务规则 1：订单创建的验证
    ///
    /// 这里包含了公司对订单创建的所有要求：
    /// - 价格必须在有效范围内
    /// - 数量必须达到最小下单量
    /// - 用户账户必须有足够保证金
    /// - 用户等级必须满足交易权限
    ///
    /// 例子：一个新手用户不能做空，只能做多
    pub fn new(
        id: OrderId,
        user: &User,
        symbol: &Symbol,
        price: Price,
        quantity: Quantity,
        side: OrderSide,
        tif: TimeInForce,
    ) -> Result<Self, DomainError> {
        // 验证 1：价格有效性
        if price.is_negative() || price.is_zero() {
            return Err(DomainError::InvalidPrice);
        }

        // 验证 2：数量有效性（最小下单数量规则）
        let min_order_qty = symbol.min_order_quantity();
        if quantity < min_order_qty {
            return Err(DomainError::BelowMinimumOrderSize(min_order_qty));
        }

        // 验证 3：用户权限（风控政策第一层）
        match user.account_type {
            AccountType::Beginner => {
                // 新手用户不能做空、不能使用杠杆
                if side == OrderSide::Sell {
                    return Err(DomainError::BeginnerCannotShort);
                }
            }
            AccountType::Intermediate | AccountType::Professional => {
                // 中级和专业用户有更多权限
            }
        }

        // 验证 4：保证金充足检查
        let required_margin = Self::calculate_required_margin(
            price,
            quantity,
            symbol.leverage(),
            side,
        );

        if user.available_balance < required_margin {
            return Err(DomainError::InsufficientMargin {
                required: required_margin,
                available: user.available_balance,
            });
        }

        // 验证 5：每日头寸限制（来自风控部门的政策）
        let user_daily_positions = user.get_today_positions();
        if user_daily_positions.count >= user.max_daily_positions {
            return Err(DomainError::DailyPositionLimitExceeded);
        }

        // 所有验证通过，创建订单
        Ok(Order {
            id,
            user_id: user.id.clone(),
            symbol: symbol.clone(),
            price,
            quantity,
            side,
            time_in_force: tif,
            status: OrderStatus::Pending,
            created_at: Timestamp::now(),
            updated_at: Timestamp::now(),
            filled_quantity: Quantity::zero(),
            fill_price: None,
            initial_margin: required_margin,
            realized_pnl: Money::zero(),
            unrealized_pnl: Money::zero(),
        })
    }

    /// 业务规则 2：订单状态转移机制
    ///
    /// 这是交易系统的"心脏"，定义了订单如何在不同状态间转移。
    ///
    /// 状态转移图：
    ///
    ///   Pending ──→ PartiallyFilled ──→ Filled ──→ [Closed]
    ///      ↓               ↓               ↓
    ///   Cancelled    Cancelled      Cancelled
    ///      ↓               ↓               ↓
    ///    [Closed]      [Closed]       [Closed]
    pub fn fill(&mut self, filled_qty: Quantity, fill_price: Price)
        -> Result<(), DomainError>
    {
        // 规则 1：只有 Pending 或 PartiallyFilled 状态的订单才能成交
        match self.status {
            OrderStatus::Pending | OrderStatus::PartiallyFilled => {},
            _ => {
                return Err(DomainError::CannotFillOrderWithStatus(self.status.clone()));
            }
        }

        // 规则 2：成交数量不能超过剩余可成交数量
        let remaining_qty = self.quantity - self.filled_quantity;
        if filled_qty > remaining_qty {
            return Err(DomainError::OverFilled {
                requested: filled_qty,
                remaining: remaining_qty,
            });
        }

        // 规则 3：更新成交信息
        self.filled_quantity = self.filled_quantity + filled_qty;
        self.fill_price = Some(fill_price);
        self.updated_at = Timestamp::now();

        // 规则 4：根据成交进度更新状态
        if self.filled_quantity == self.quantity {
            self.status = OrderStatus::Filled;
        } else if self.filled_quantity > Quantity::zero() {
            self.status = OrderStatus::PartiallyFilled;
        }

        // 规则 5：计算未实现收益（浮盈/浮亏）
        self.unrealized_pnl = self.calculate_pnl(fill_price)?;

        Ok(())
    }

    /// 业务规则 3：订单取消规则
    ///
    /// 这个方法看似简单，但背后反映了交易所、监管和风控的多层限制：
    /// - 某些订单一旦部分成交就不能取消（交易所规则）
    /// - 某些产品的订单在特定时间段不能取消（监管要求）
    /// - 某些客户的订单不能批量取消（风控要求）
    pub fn cancel(&mut self) -> Result<(), DomainError> {
        // 规则 1：已成交的订单无法取消
        if self.status == OrderStatus::Filled {
            return Err(DomainError::CannotCancelFilledOrder);
        }

        // 规则 2：已取消的订单无法再取消
        if self.status == OrderStatus::Cancelled {
            return Err(DomainError::OrderAlreadyCancelled);
        }

        // 规则 3：某些订单类型无法取消（交易所规则）
        // 例如：止损单一旦激活（PartiallyFilled），某些交易所不允许取消
        if self.time_in_force == TimeInForce::IOC && self.filled_quantity > Quantity::zero() {
            return Err(DomainError::CannotCancelIOCOrderAfterPartialFill);
        }

        // 规则 4：检查是否在禁止取消时间窗口
        // 某些产品（如期权）在特定时间段禁止订单修改
        if !self.can_cancel_at_current_time() {
            return Err(DomainError::CancelNotAllowedDuringTradingHours);
        }

        // 检查通过，执行取消
        self.status = OrderStatus::Cancelled;
        self.updated_at = Timestamp::now();

        Ok(())
    }

    /// 业务规则 4：风险指标计算
    ///
    /// 这个方法体现了风控部门的算法积累。
    /// 不同客户等级的风控规则不同，这体现在下单时的验证中。
    fn calculate_pnl(&self, current_price: Price) -> Result<Money, DomainError> {
        if self.fill_price.is_none() {
            return Ok(Money::zero());
        }

        let avg_fill_price = self.fill_price.unwrap();
        let price_change = current_price - avg_fill_price;

        let pnl = match self.side {
            OrderSide::Buy => {
                // 买入时，价格上升获利
                price_change * self.filled_quantity
            }
            OrderSide::Sell => {
                // 卖出时，价格下降获利
                (avg_fill_price - current_price) * self.filled_quantity
            }
        };

        Ok(pnl)
    }

    /// 这些验证和规则在哪里来？
    ///
    /// 1. 交易所规则：与交易所协议中的对账要求
    /// 2. 监管要求：金融监管部门对交易的强制性要求
    /// 3. 风控政策：公司自身对风险的管理政策
    /// 4. 用户反馈：从客户投诉中学习的限制
    /// 5. 事故经验：历史事故中积累的教训
    ///
    /// 这些都浓缩在 Order 实体和 UseCase 中。
}
```

#### 1.3 关键点

- **每个字段都有含义**：没有多余的字段，也没有遗漏的字段
- **每个方法都有规则**：不是单纯的 getter/setter，而是业务逻辑的守门人
- **充分的验证**：在实体层就防止了非法状态的出现
- **可独立测试**：无需启动任何外部系统

---

### 特征 2：幂等性和稳定性（Idempotency and Stability）

#### 2.1 定义

Core 就像"灵魂"，可以穿上任何"肉体"（技术栈）而保持本质不变。

#### 2.2 实例：从 Hibernate 迁移到 SQLx

```rust
// Year 1: 使用 Diesel ORM
#[table_name = "orders"]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub status: String,  // ❌ 紧耦合数据库表结构
}

// ❌ 问题：如果改数据库结构，Order 实体也要改
// ❌ 问题：无法独立测试（ORM 必须初始化）
// ❌ 问题：迁移到其他 ORM 需要全量重写

// ---

// ✅ Clean Architecture 的做法

// src/domain/entities/order.rs
pub struct Order {
    id: OrderId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    status: OrderStatus,
}

impl Order {
    pub fn new(...) -> Result<Self, DomainError> { /* 业务逻辑 */ }
    pub fn cancel(mut self) -> Result<Self, DomainError> { /* 业务逻辑 */ }
}

// src/infrastructure/repositories/order_model.rs
// 数据模型与实体分离
#[derive(sqlx::FromRow)]
pub struct OrderDbModel {
    pub id: String,
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub status: String,
}

// src/infrastructure/repositories/diesel_order_repo.rs
pub struct DieselOrderRepository { /* ... */ }

impl OrderRepository for DieselOrderRepository {
    async fn save(&self, order: &Order) -> Result<(), RepositoryError> {
        let model = OrderDbModel::from(order);
        diesel::insert_into(orders::table)
            .values(&model)
            .execute(&self.connection)?;
        Ok(())
    }
}

// src/infrastructure/repositories/sqlx_order_repo.rs
pub struct SqlxOrderRepository { /* ... */ }

impl OrderRepository for SqlxOrderRepository {
    async fn save(&self, order: &Order) -> Result<(), RepositoryError> {
        let model = OrderDbModel::from(order);
        sqlx::query!(
            "INSERT INTO orders (...) VALUES (...)",
            model.id, model.symbol, // ...
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

// ✅ 优点：
// 1. Order 实体完全不改
// 2. 数据库结构改变：只需修改 OrderDbModel 和 Repository 实现
// 3. ORM 库升级或替换：新增一个 Repository 实现即可
// 4. 单元测试：Order 逻辑完全独立，不需要数据库
```

#### 2.3 灵魂转移的过程

```
Year 1: Java + Spring Data JPA
  └─ Order 实体紧耦合 @Entity, @Column 注解

Year 2: 迁移到 Rust + Tokio
  ❌ 如果没有分离 Core，需要重新理解和实现订单逻辑
  ✅ 如果分离了 Core，直接迁移业务规则

Year 3: 从 PostgreSQL 迁移到 ClickHouse
  ✅ 只需新增 ClickHouseRepository，Order 实体零改动

结论：
灵魂（Order 实体 + 业务规则）永恒不变
肉体（语言、框架、数据库）可以随意更换
```

---

### 特征 3：不可替代性（Irreplaceability）

#### 3.1 定义

Adapter 可以被新的实现替代，但 Core 无法被替代——替代它意味着放弃业务竞争力。

#### 3.2 对比表

```
组件            可替代性          替代成本         业务影响
────────────────────────────────────────────────────────
REST API       ✅ 高            低（2周）        无（换个接口）
HTTP 框架      ✅ 高            低（3周）        无（内部实现）
数据库         ✅ 中            中（2-3月）      无（架构隐藏）
缓存库         ✅ 高            低（1-2周）      无（透明替换）
────────────────────────────────────────────────────────
风控算法       ❌ 不可替代      极高            毁灭性（直接损失）
订单清算逻辑   ❌ 不可替代      极高            毁灭性（数据混乱）
用户权限规则   ❌ 不可替代      极高            严重（监管风险）
状态转移规则   ❌ 不可替代      极高            毁灭性（交易失效）
```

#### 3.3 案例：某交易所的教训

```
Year 1: 初期运营
  - 风控算法：根据交易量设置头寸限制
  - 清算规则：T+1 清算
  - 系统稳定运行

Year 2: 业务扩展，尝试更换风控供应商
  - "我们可以替换风控算法"（想法错误！）
  - 新风控公司提供的算法不同
  - 导致：原本被正确拒绝的订单现在被接受
  - 结果：大额亏损，最后还是改了回去

Year 3: 这次他们明白了
  - 风控不仅仅是"一个算法"
  - 它是企业多年对市场的理解和损失教训
  - 无法轻易替代

总结：
Core 的不可替代性源于它的历史积累。
删除它相当于删除公司的知识产权。
```

---

### 特征 4：幂等性保证（Idempotent Operations）

#### 4.1 定义

Core 的操作应该满足幂等性：相同的输入产生相同的输出，不管运行多少次。

#### 4.2 实例：订单填充操作的幂等性

```rust
/// ❌ 非幂等的实现（会导致问题）
impl Order {
    fn fill(&mut self, filled_qty: Quantity) -> Result<(), Error> {
        // 每次调用都会累加成交数量
        self.filled_quantity += filled_qty;  // ❌ 状态突变
        self.updated_at = Timestamp::now();   // ❌ 时间戳改变

        // 如果网络中断导致消息重复，会重复成交！
        Ok(())
    }
}

/// ✅ 幂等的实现
impl Order {
    fn fill(&mut self, fill_event: OrderFilledEvent) -> Result<(), Error> {
        // 使用事件 ID 确保幂等性
        if self.processed_fill_ids.contains(&fill_event.fill_id) {
            // 已经处理过这个成交，直接返回（幂等）
            return Ok(());
        }

        // 只有新的成交才处理
        self.filled_quantity += fill_event.quantity;
        self.processed_fill_ids.insert(fill_event.fill_id);

        Ok(())
    }
}

// ✅ 优点：
// 如果消息被重复投递，第二次调用会被忽略（幂等）
// 网络故障不会导致数据不一致
// 分布式系统中可以安全地重试
```

---

## Core 的价值维度

### 维度 1：知识资产（Knowledge Assets）

#### 1.1 知识的来源

| 知识来源 | 获取方式 | 成本估算 | 失失成本 |
|---------|---------|---------|---------|
| 订单生命周期规则 | 历年事故教训 | 百万级直接损失 | 系统崩溃 |
| 风控算法 | 量化团队研发 | 百万级研发投入 | 风险失控 |
| 交易所规则 | 与交易所协议 | 数十万级 | 交易被拒 |
| 合规监管规则 | 法务咨询 + 自研 | 数十万级 | 监管罚款 |
| 用户行为模型 | 数据分析积累 | 数百万级算力 | 无法预测风险 |
| **总计** | | **数百万到千万** | **无法用金钱快速恢复** |

#### 1.2 知识的特性

```
Core 中的知识具有这些特点：

1. 不可快速复制
   ├─ 写一个"订单"类很容易（1 天）
   ├─ 但写一个"正确的订单"类需要 5 年
   └─ 因为需要经历无数边界情况和异常

2. 隐含的依赖关系
   ├─ 订单状态转移规则看似独立
   ├─ 实际上依赖于交易所规则、清算规则、监管规则
   └─ 改动其中一个会影响整个系统

3. 难以文档化
   ├─ 不能写 README 就能理解
   ├─ 必须通过阅读代码 + 单元测试 + 讨论
   └─ 新人需要花 3-6 个月才能熟悉

4. 易被误解
   ├─ "为什么订单不能在这个状态取消？"
   ├─ 原因可能追溯到 3 年前的一个客户投诉或事故
   └─ 如果删除这个验证，会重复历史错误
```

#### 1.3 知识资产的财务价值

```
一个成熟的交易系统的 Core，知识资产价值计算：

年度收入：10 亿
Core 对收入的贡献：80%（其他由产品、市场、运营贡献）
Core 知识资产年度价值：8 亿 × 平均生命周期 7 年 = 56 亿

换句话说：
如果 Core 被破坏，公司需要投入相当于 5-7 年的收入才能重建。

这就是为什么在高价值系统中，保护 Core 至关重要。
```

---

### 维度 2：竞争力（Competitive Advantage）

#### 2.1 竞争力的四个层面

```
市场竞争             Core 贡献度      Adapter 贡献度
────────────────────────────────────────────────
成交速度（快速响应）    40%              60%
成交准确性（无误差）    90%              10%
风险管理能力           95%               5%
用户体验流畅度         50%              50%
系统可靠性（不宕机）    30%              70%

加权平均竞争力：Core 贡献 75%, Adapter 贡献 25%
```

#### 2.2 案例：两家竞争对手的差异

```
竞争对手 A：非 Clean 架构
  - 订单处理速度：100ms（不错）
  - 故障率：0.1%（可接受）
  - 风控准确率：92%（较好）
  - 但：Core 逻辑混乱，很难添加新的风控规则
  - 结果：每次风控升级需要 2 周，行业无法跟进

竞争对手 B：Clean 架构 + 清晰的 Core
  - 订单处理速度：100ms（相同）
  - 故障率：0.1%（相同）
  - 风控准确率：92%（相同）
  - 但：Core 设计清晰，新的风控规则可在 2 天内完成
  - 结果：每月都有新的风控升级，快速适应市场变化

6 个月后的差异：
  - A 只有 3 个风控版本，无法应对新的市场风险
  - B 已有 12 个风控版本，覆盖更多风险场景
  - B 的市场竞争力明显提升

1 年后：
  - A 意识到问题，开始重构 Core（成本 1000万+，时间 1 年）
  - B 继续创新，进一步拉大差距
```

---

### 维度 3：抗风险能力（Risk Resilience）

#### 3.1 系统危机的种类

```
危机类型              原因                响应能力
─────────────────────────────────────────────────────
服务器宕机          硬件故障            Adapter 问题
数据库连接失败      网络故障            Adapter 问题
第三方网关故障      外部服务            Adapter 问题
订单异常            业务逻辑             Core 问题
数据不一致          状态管理            Core 问题
风控失效            规则错误            Core 问题
```

#### 3.2 危机响应的能力差异

```
假设发生"订单无法取消"的故障：

❌ Core 腐化的系统（非 Clean 架构）：
1. 接到客诉：有用户报告订单无法取消
2. 调查（2 小时）：查询日志，发现异常
3. 查找原因（4 小时）：在 OrderController 中看到一个 if 语句
4. 继续查找（4 小时）：发现 Order 类有 3 个地方在检查状态
5. 定位问题（2 小时）：某个状态判断写错了
6. 修复代码（1 小时）：改正 bug
7. 单元测试（3 小时）：很难写，因为依赖太多
8. 集成测试（6 小时）：需要启动整个系统
9. 发布（4 小时）：风险评估、代码审查、部署
→ 总耗时：26 小时，故障影响 >= 10000 用户，损失百万级

✅ Core 清晰的系统（Clean 架构）：
1. 接到客诉（5 分钟）
2. 查看 Order 实体的 cancel 方法（5 分钟）：逻辑清晰，问题一目了然
3. 修复（5 分钟）：改正状态检查
4. 单元测试（10 分钟）：运行现有的测试，新增一个测试
5. 集成测试（10 分钟）：运行 Integration Test
6. 发布（20 分钟）：风险低，快速审查，快速部署
→ 总耗时：50 分钟，故障影响 < 1000 用户，损失 10 万级

差异：
- 时间：26 小时 vs 50 分钟 = 31 倍差异
- 损失：百万 vs 10 万 = 10 倍差异
- 用户体验：故障时间相差数小时
```

---

## Core 的腐化过程与警示

### 腐化的四个阶段

#### 阶段 1：初期健康（Year 1-2）

```rust
// ✅ 健康的 Order 核心
pub struct Order {
    id: OrderId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    status: OrderStatus,
    filled_quantity: Quantity,
}

impl Order {
    pub fn new(...) -> Result<Self, DomainError> {
        // 清晰的验证逻辑，5-10 行
    }

    pub fn cancel(mut self) -> Result<Self, DomainError> {
        // 简单的状态检查，3-5 行
        if self.status == OrderStatus::Filled {
            return Err(DomainError::CannotCancelFilled);
        }
        self.status = OrderStatus::Cancelled;
        Ok(self)
    }
}

特征：
- 清晰易懂，新人快速上手
- 测试覆盖率 100%
- 每个方法都有清晰的职责
```

#### 阶段 2：初现问题（Year 2-3）

```rust
// ⚠️ 开始出现问题：打补丁
pub struct Order {
    id: OrderId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    status: OrderStatus,
    filled_quantity: Quantity,

    // Year 2 新增：止损单功能
    stop_loss_price: Option<Price>,        // ⚠️ 新增字段
    is_stop_loss_order: bool,               // ⚠️ 新增字段

    // Year 2 新增：条件单功能
    condition: Option<OrderCondition>,     // ⚠️ 新增字段
}

impl Order {
    pub fn cancel(mut self) -> Result<Self, DomainError> {
        // 原有逻辑
        if self.status == OrderStatus::Filled {
            return Err(DomainError::CannotCancelFilled);
        }

        // ⚠️ Year 2 添加的补丁
        if self.is_stop_loss_order && self.status == OrderStatus::PartiallyFilled {
            return Err(DomainError::CannotCancelStopLossAfterPartialFill);
        }

        // ⚠️ Year 2 添加的另一个补丁
        if self.condition.is_some() && !self.condition.as_ref().unwrap().is_satisfied() {
            return Err(DomainError::CannotCancelUnmetCondition);
        }

        self.status = OrderStatus::Cancelled;
        Ok(self)
    }
}

问题开始出现：
- cancel 方法从 3 行变成 15 行
- 新增字段与原有字段的关系不清晰
- 不清楚哪些字段组合是有效的
```

#### 阶段 3：明显衰败（Year 3-4）

```rust
// ❌ 开始严重衰败：逻辑散落
@Entity
@Table(name = "orders")
public class Order {
    @Id private String id;
    @Column private String symbol;
    @Column private BigDecimal price;
    @Column private BigDecimal quantity;

    // Year 1 字段
    @Column private String status;
    @Column private BigDecimal filledQuantity;

    // Year 2 新增
    @Column private BigDecimal stopLossPrice;
    @Column private Boolean isStopLossOrder;
    @Column private String condition;

    // Year 3 新增：风控相关（混在一起！）
    @Column private BigDecimal riskExposure;
    @Column private BigDecimal dailyLimit;
    @Column private Integer userRiskLevel;

    // Year 3 新增：用户权限（也混在一起！）
    @Column private String userTier;
    @Column private Boolean canShort;
    @Column private Boolean canUseMargin;

    // Year 3 新增：缓存（这是什么？）
    @Transient private Cache<String> orderCache;
    @Transient private List<Order> relatedOrders;  // 为什么要存储相关订单？

    // Year 4 新增：技术债
    @Column private String legacyStatus;    // 兼容旧系统
    @Column private String tags;             // JSON 放所有杂项

    // validate 方法：200+ 行，分不清优先级
    public void validate() {
        if (this.price < 0) {
            throw new InvalidPriceException();
        }
        if (this.quantity < 0) {
            throw new InvalidQuantityException();
        }
        // ... 50 行验证

        if (this.isStopLossOrder) {
            if (this.stopLossPrice < 0) {
                throw new InvalidStopLossPriceException();
            }
            // ... 20 行止损相关验证
        }

        if (this.condition != null) {
            // ... 30 行条件单验证
        }

        if (this.userTier == "BEGINNER") {
            if (this.canShort) {
                throw new BeginnerCannotShortException();
            }
            // ... 15 行新手限制验证
        }

        // ... 还有 60 行其他验证
    }

    // cancel 方法：30+ 行，难以理解
    public void cancel() {
        if (this.status.equals("FILLED")) {
            throw new CannotCancelFilledOrderException();
        }

        // ⚠️ Year 2 的 if
        if (this.isStopLossOrder && this.status.equals("PARTIALLY_FILLED")) {
            throw new CannotCancelStopLossException();
        }

        // ⚠️ Year 3 的 if
        if (this.userRiskLevel > 50) {
            if (this.riskExposure > this.dailyLimit) {
                throw new ExceedsDailyLimitException();
            }
        }

        // ⚠️ Year 4 的 if（为什么要检查？没人知道）
        if (this.legacyStatus != null) {
            // 什么是 legacyStatus？
            // 为什么要检查它？
            // 可以删除吗？不敢删。
        }

        this.status = "CANCELLED";
    }
}

严重问题：
- 无法知道哪些字段组合是有效的
- 无法知道哪些条件是必须的，哪些是可选的
- validate 方法是一个"黑盒"，改一个地方影响全局
- 新人需要花 2-3 周才能理解 validate 逻辑
- 很难写出有效的单元测试
```

#### 阶段 4：瘫痪（Year 4-5）

```
系统表现：
- 新增功能需要改 Order 类
- 改 Order 类需要全量回归测试
- 回归测试经常发现新 bug
- 修复 bug 又引入新 bug
- 团队开始害怕改动 Order 类

人员流动：
- Year 1: 3 个工程师，理解 Order 逻辑
- Year 2: 3 + 2 = 5 个工程师，2 个新人需要学习
- Year 3: 5 + 3 = 8 个工程师，3 个新人很难掌握完整逻辑
- Year 4: 只有 1 个老兵还记得为什么这样写
- Year 4 中期：这个老兵离职了（因为系统难以维护，跳槽去清爽的创业公司）
- Year 4 末期：无人知道为什么会这样（知识流失）

典型现象：
❌ "为什么 Order 的 validate 要检查 userRiskLevel？"
  → "不知道，反正代码就这样，改不了"

❌ "这个 if 条件可以删除吗？"
  → "不敢，可能会有副作用，而且没有人敢保证"

❌ "新功能要修改 Order 类吗？"
  → "可能要，但我们都害怕改这个类"

最终：
- 系统陷入"维护地狱"
- 新功能开发速度 ↓ 90%
- Bug 修复周期 ↑ 300%
- 团队士气 ↓ 80%
- 有人开始建议"重写系统"（但重写需要 12-18 个月）
```

---

## Core 设计原则

### 原则 1：单一职责（Single Responsibility）

#### 定义

每个实体类只负责一个业务对象的生命周期和规则。

#### 反例

```rust
// ❌ 违反单一职责：Order 负责太多
pub struct Order {
    // 订单信息
    id: OrderId,
    symbol: Symbol,
    price: Price,

    // ⚠️ 用户信息（不应该在这里）
    user_name: String,
    user_email: String,
    user_phone: String,

    // ⚠️ 账户信息（不应该在这里）
    account_balance: Money,
    total_trades: i32,

    // ⚠️ 手续费信息（不应该在这里）
    fee_tier: String,
    commission_rate: f64,
}

impl Order {
    fn validate_user(&self) -> Result<(), Error> {
        // 不应该验证用户信息
    }

    fn validate_account(&self) -> Result<(), Error> {
        // 不应该验证账户信息
    }

    fn calculate_commission(&self) -> Money {
        // 不应该计算手续费
    }
}
```

#### 正确做法

```rust
// ✅ 遵循单一职责：Order 只负责订单本身

pub struct Order {
    id: OrderId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    status: OrderStatus,
}

impl Order {
    // 只有与订单相关的方法
    pub fn new(...) -> Result<Self, DomainError> { /* ... */ }
    pub fn cancel(mut self) -> Result<Self, DomainError> { /* ... */ }
    pub fn fill(&mut self, qty: Quantity) -> Result<(), DomainError> { /* ... */ }
}

// 用户信息由 User 实体负责
pub struct User {
    id: UserId,
    name: String,
    email: String,
    balance: Money,
}

// 账户信息由 Account 实体负责
pub struct Account {
    user_id: UserId,
    balance: Money,
    total_trades: i32,
    tier: UserTier,
}

// 手续费计算由专用服务负责
pub struct CommissionCalculator {
    fee_structure: FeeStructure,
}

impl CommissionCalculator {
    pub fn calculate(&self, order: &Order, account: &Account) -> Money {
        // 使用 Order 和 Account 的公共数据
        // 但不修改它们
    }
}

// UseCase 负责编排这些对象
pub struct PlaceOrderInteractor {
    user_repo: Arc<dyn UserRepository>,
    account_repo: Arc<dyn AccountRepository>,
    order_repo: Arc<dyn OrderRepository>,
    commission_calculator: Arc<CommissionCalculator>,
}

impl PlaceOrderInteractor {
    pub async fn execute(&self, request: PlaceOrderRequest) -> Result<...> {
        let user = self.user_repo.find_by_id(&request.user_id).await?;
        let account = self.account_repo.find_by_user(&request.user_id).await?;

        let order = Order::new(/* ... */)?;

        let commission = self.commission_calculator.calculate(&order, &account)?;

        // 保存订单
        self.order_repo.save(&order).await?;

        Ok(/* ... */)
    }
}
```

---

### 原则 2：不可变性（Immutability）

#### 定义

Core 的实体应该尽可能不可变，避免无意的副作用。

#### Rust 实现

```rust
// ✅ 不可变设计（Rust 推荐）
pub struct Order {
    id: OrderId,
    symbol: Symbol,
    status: OrderStatus,
}

impl Order {
    /// 取消返回一个新的 Order，不修改原有的
    pub fn cancel(self) -> Result<Self, DomainError> {
        match self.status {
            OrderStatus::Pending | OrderStatus::PartiallyFilled => {
                Ok(Order {
                    status: OrderStatus::Cancelled,
                    ..self
                })
            }
            _ => Err(DomainError::CannotCancelInThisState),
        }
    }
}

// 使用时：
let order = create_order()?;
let cancelled_order = order.cancel()?;  // ✅ 返回新的对象
// order 仍然是原来的状态，cancelled_order 是新状态
```

#### Java 实现

```java
// ✅ 不可变设计（Java）
public final class Order {
    private final OrderId id;
    private final Symbol symbol;
    private final OrderStatus status;

    // 私有构造器
    private Order(OrderId id, Symbol symbol, OrderStatus status) {
        this.id = Objects.requireNonNull(id);
        this.symbol = Objects.requireNonNull(symbol);
        this.status = Objects.requireNonNull(status);
    }

    public static Order create(OrderId id, Symbol symbol) throws DomainException {
        // 验证
        if (symbol == null) throw new DomainException("Invalid symbol");
        return new Order(id, symbol, OrderStatus.PENDING);
    }

    /// 取消返回一个新的 Order
    public Order cancel() throws DomainException {
        if (status != OrderStatus.PENDING && status != OrderStatus.PARTIALLY_FILLED) {
            throw new DomainException("Cannot cancel in this state");
        }
        return new Order(this.id, this.symbol, OrderStatus.CANCELLED);
    }

    // 所有 getter，没有 setter
    public OrderId getId() { return id; }
    public Symbol getSymbol() { return symbol; }
    public OrderStatus getStatus() { return status; }
}
```

---

### 原则 3：值对象和实体的区分（Value Objects vs Entities）

#### 定义

- **实体**：有唯一身份的对象（如 Order，id 唯一）
- **值对象**：只关心值的对象（如 Price，两个价格相等就是同一个）

#### 实现

```rust
// ✅ 实体：Order（有唯一 id）
pub struct Order {
    id: OrderId,  // ← 唯一标识
    symbol: Symbol,
    price: Price,  // ← 值对象
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id  // ← 仅根据 id 比较
    }
}

// ✅ 值对象：Price（没有唯一 id）
pub struct Price(f64);

impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-8  // ← 比较具体数值
    }
}

// 例子：
let order1 = Order::new(OrderId::new("123"), Symbol::new("BTCUSDT"), Price::new(50000.0));
let order2 = Order::new(OrderId::new("123"), Symbol::new("BTCUSDT"), Price::new(50000.0));
let order3 = Order::new(OrderId::new("456"), Symbol::new("BTCUSDT"), Price::new(50000.0));

order1 == order2  // true （同一个订单，即使重新创建）
order1 == order3  // false（不同的订单）

let price1 = Price::new(50000.0);
let price2 = Price::new(50000.0);
let price3 = Price::new(50001.0);

price1 == price2  // true（值相等）
price1 == price3  // false（值不等）
```

---

### 原则 4：充分的验证（Comprehensive Validation）

#### 定义

在 Core 层进行所有可能的验证，确保无效状态无法被创建。

#### 实现

```rust
// ✅ 充分验证：在构造时验证

pub struct Order {
    id: OrderId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
}

pub struct OrderId(String);

impl OrderId {
    /// 验证 OrderId 的有效性
    pub fn new(id: String) -> Result<Self, ValidationError> {
        if id.is_empty() {
            return Err(ValidationError::EmptyId);
        }
        if id.len() > 255 {
            return Err(ValidationError::IdTooLong);
        }
        if !id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::InvalidIdFormat);
        }
        Ok(OrderId(id))
    }
}

pub struct Price(f64);

impl Price {
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if value < 0.0 {
            return Err(ValidationError::NegativePrice);
        }
        if !value.is_finite() {
            return Err(ValidationError::InvalidPrice);
        }
        if value < 0.01 {  // 最小精度：0.01
            return Err(ValidationError::PriceTooSmall);
        }
        Ok(Price(value))
    }
}

pub struct Quantity(f64);

impl Quantity {
    pub fn new(value: f64) -> Result<Self, ValidationError> {
        if value <= 0.0 {
            return Err(ValidationError::InvalidQuantity);
        }
        if !value.is_finite() {
            return Err(ValidationError::InvalidQuantity);
        }
        if value > 1_000_000_000.0 {  // 最大数量
            return Err(ValidationError::QuantityTooLarge);
        }
        Ok(Quantity(value))
    }
}

// 构造 Order 时，所有值都已经验证过
impl Order {
    pub fn new(
        id: OrderId,  // ← 已验证
        symbol: Symbol,  // ← 已验证
        price: Price,  // ← 已验证
        quantity: Quantity,  // ← 已验证
    ) -> Result<Self, ValidationError> {
        // 这里只需要做 Order 特有的验证
        // 例如：某些币对不支持小数点后这么多位

        Ok(Order {
            id,
            symbol,
            price,
            quantity,
        })
    }
}

// ✅ 好处：
// 1. 一个 Order 实例 = 一个有效的订单
// 2. 不存在"无效的订单"对象（编译级别保证）
// 3. 后续操作无需再验证这些字段
```

---

## Core 保护实践

### 实践 1：Core 层无任何外部依赖

#### 错误做法

```rust
// ❌ 错误：导入框架库
use axum::response::IntoResponse;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, PgPool};
use chrono::{DateTime, Utc};

#[derive(FromRow, Serialize, Deserialize)]
pub struct Order {
    #[sqlx(rename = "id")]
    #[serde(rename = "id")]
    pub id: String,

    pub symbol: String,
    pub price: f64,

    #[sqlx(rename = "created_at")]
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

impl Order {
    /// 从数据库加载
    pub async fn load(id: &str, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }
}

// ❌ 问题：
// 1. Order 被 sqlx 和 serde 污染
// 2. 无法独立测试
// 3. 改变数据库后 Order 也要改
// 4. 序列化格式改变 Order 也要改
// 5. 迁移到其他语言时 Order 无法复用
```

#### 正确做法

```rust
// ✅ 正确：Core 层完全纯净

// src/domain/entities/order.rs
pub struct Order {
    id: OrderId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    status: OrderStatus,
    created_at: DateTime,  // ← 使用纯 Rust 类型，不是 chrono
}

impl Order {
    pub fn new(...) -> Result<Self, DomainError> {
        // 纯业务逻辑，无任何 I/O
    }

    pub fn cancel(mut self) -> Result<Self, DomainError> {
        // 纯业务逻辑，无任何 I/O
    }
}

// src/infrastructure/persistence/models.rs
// 只有 Infrastructure 层需要处理数据库模型

#[derive(sqlx::FromRow)]
pub struct OrderDbModel {
    pub id: String,
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

// 转换由 Infrastructure 层负责
impl From<&Order> for OrderDbModel {
    fn from(order: &Order) -> Self {
        OrderDbModel {
            id: order.id().to_string(),
            symbol: order.symbol().to_string(),
            // ...
        }
    }
}

impl TryFrom<OrderDbModel> for Order {
    type Error = DomainError;

    fn try_from(model: OrderDbModel) -> Result<Self, Self::Error> {
        Order::new(
            OrderId::new(model.id)?,
            Symbol::new(model.symbol)?,
            // ...
        )
    }
}

// ✅ 优点：
// 1. Order 完全独立，无框架污染
// 2. 可在任何环境（桌面、Web、移动）运行
// 3. 数据库改变 Order 完全不受影响
// 4. 序列化格式改变 Order 完全不受影响
// 5. 可轻易迁移到其他语言（逻辑相同）
```

---

### 实践 2：Core 100% 单元测试覆盖

#### 测试结构

```rust
// src/domain/entities/order.rs

#[cfg(test)]
mod tests {
    use super::*;

    // ✅ 测试 1：创建有效订单
    #[test]
    fn test_create_valid_order() {
        let order = Order::new(
            OrderId::new("123").unwrap(),
            Symbol::new("BTCUSDT").unwrap(),
            Price::new(50000.0).unwrap(),
            Quantity::new(1.0).unwrap(),
        );

        assert!(order.is_ok());
        assert_eq!(order.unwrap().status, OrderStatus::Pending);
    }

    // ✅ 测试 2：创建无效订单（负数价格）
    #[test]
    fn test_create_order_with_negative_price() {
        let order = Order::new(
            OrderId::new("123").unwrap(),
            Symbol::new("BTCUSDT").unwrap(),
            Price::new(-50000.0),  // ← 负数
            Quantity::new(1.0).unwrap(),
        );

        assert!(order.is_err());
    }

    // ✅ 测试 3：取消订单的有效状态转移
    #[test]
    fn test_cancel_pending_order() {
        let order = Order::new(/* ... */).unwrap();

        assert_eq!(order.status, OrderStatus::Pending);

        let cancelled = order.cancel();

        assert!(cancelled.is_ok());
        assert_eq!(cancelled.unwrap().status, OrderStatus::Cancelled);
    }

    // ✅ 测试 4：取消已成交订单应该失败
    #[test]
    fn test_cannot_cancel_filled_order() {
        let mut order = Order::new(/* ... */).unwrap();

        // 模拟成交
        order.fill(Quantity::new(1.0).unwrap(), Price::new(50000.0).unwrap()).unwrap();

        // 尝试取消
        let result = order.cancel();

        assert!(result.is_err());
    }

    // ✅ 测试 5：成交数量累加
    #[test]
    fn test_partial_fill_accumulation() {
        let mut order = Order::new(
            /* quantity: 10 */
        ).unwrap();

        order.fill(Quantity::new(3.0).unwrap(), Price::new(50000.0).unwrap()).unwrap();
        assert_eq!(order.filled_quantity(), Quantity::new(3.0).unwrap());
        assert_eq!(order.status, OrderStatus::PartiallyFilled);

        order.fill(Quantity::new(4.0).unwrap(), Price::new(50000.0).unwrap()).unwrap();
        assert_eq!(order.filled_quantity(), Quantity::new(7.0).unwrap());
        assert_eq!(order.status, OrderStatus::PartiallyFilled);

        order.fill(Quantity::new(3.0).unwrap(), Price::new(50000.0).unwrap()).unwrap();
        assert_eq!(order.filled_quantity(), Quantity::new(10.0).unwrap());
        assert_eq!(order.status, OrderStatus::Filled);
    }

    // ✅ 测试 6：超成交应该失败
    #[test]
    fn test_overfill_prevention() {
        let mut order = Order::new(
            /* quantity: 10 */
        ).unwrap();

        order.fill(Quantity::new(5.0).unwrap(), Price::new(50000.0).unwrap()).unwrap();

        // 尝试再成交 6 个（总共 11，超过 10）
        let result = order.fill(
            Quantity::new(6.0).unwrap(),
            Price::new(50000.0).unwrap()
        );

        assert!(result.is_err());
    }
}
```

#### 测试覆盖率度量

```
├─ Order 实体
│  ├─ new() 方法
│  │  ├─ 有效创建: ✅ tested
│  │  ├─ 价格无效: ✅ tested
│  │  ├─ 数量无效: ✅ tested
│  │  └─ ID 无效: ✅ tested
│  ├─ cancel() 方法
│  │  ├─ Pending → Cancelled: ✅ tested
│  │  ├─ PartiallyFilled → Cancelled: ✅ tested
│  │  ├─ Filled → Error: ✅ tested
│  │  └─ Cancelled → Error: ✅ tested
│  └─ fill() 方法
│     ├─ 有效成交: ✅ tested
│     ├─ 超成交: ✅ tested
│     ├─ 状态转移: ✅ tested
│     └─ 无法成交已取消的订单: ✅ tested
│
覆盖率: 100%（每个分支都有至少一个测试）
```

---

### 实践 3：定期的 Core 健康检查

#### 检查清单（每个 Sprint）

```
□ 是否有新增的嵌套 if-else 语句（> 5 层）？
  → 可能是规则混乱的信号
  → 应该拆分为多个方法或创建新的值对象

□ 是否有代码路径无人能完整解释？
  → 知识流失的信号
  → 应该补充文档或重构

□ 是否有 TODO / FIXME 注释未解决 > 1 个月？
  → 技术债积累的信号
  → 应该立即处理

□ 新增功能时是否修改了已有的实体类？
  → 可能不应该改已有类，应该新增用例
  → 检查是否违反了开闭原则

□ 单元测试覆盖率是否下降？
  → 如果 < 95%，说明 Core 在腐化
  → 应该补充测试

□ 修改 Core 的 PR 是否涉及 Adapter 改动？
  → 如果需要同时改 Adapter，说明有依赖倒置问题
  → Adapter 应该依赖 Core，而不是反过来

□ 新人理解 Order 实体需要多长时间？
  → Year 1: 1-2 小时
  → Year 3: 3-4 小时
  → Year 5: 5+ 小时（危险信号）
  → 如果 > 4 小时，说明设计变复杂了
```

---

## Core 的演化路径

### 演化模式：开闭原则（Open for Extension, Closed for Modification）

#### Pattern 1：新增用例，不修改实体

```rust
// Year 1: 基础订单
pub struct Order { /* ... */ }

pub struct PlaceOrderUseCase { /* ... */ }
pub struct CancelOrderUseCase { /* ... */ }

// ---

// Year 2: 需要支持止损单
// ❌ 错误：修改 Order 实体
// pub struct Order {
//     price: Price,
//     stop_loss_price: Option<Price>,  // ← 新增字段
// }

// ✅ 正确：新增 StopLossOrder 实体或专用用例
pub struct StopLossOrder {
    base_order: Order,
    stop_loss_price: Price,
    trigger_condition: StopLossCondition,
}

pub struct PlaceStopLossOrderUseCase { /* ... */ }

// ---

// Year 3: 需要支持条件单
// ✅ 正确：新增 ConditionalOrder 用例
pub struct ConditionalOrder {
    base_order: Order,
    condition: OrderCondition,
    execution_rule: ExecutionRule,
}

pub struct PlaceConditionalOrderUseCase { /* ... */ }
```

#### Pattern 2：通过参数多态扩展行为

```rust
// Year 1
pub enum TimeInForce {
    GTC,  // Good Till Cancelled
    IOC,  // Immediate or Cancel
    FOK,  // Fill or Kill
}

impl Order {
    pub fn new(
        symbol: Symbol,
        price: Price,
        quantity: Quantity,
        tif: TimeInForce,  // ← 通过参数支持多种行为
    ) -> Result<Self, DomainError> {
        // 验证依赖于 tif 的规则
    }
}

// Year 2: 需要支持更多时间约束
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    GTD(DateTime),      // Good Till Date：指定日期过期
    GTM(Month),         // Good Till Month：指定月份过期
}

// ✅ Order 实体无需改动，只需添加新的 TimeInForce 变体
```

---

## Core 团队的组织

### 角色定义

```
Core 团队（Platform/Domain Team）
├─ 商业分析师 (Business Analyst)
│  └─ 职责：
│     ├─ 从业务需求翻译为领域规则
│     ├─ 与客户沟通用户需求
│     └─ 验证实现是否符合业务意图
│
├─ 领域专家 (Domain Expert)
│  └─ 职责：
│     ├─ 理解金融、交易、风控等领域知识
│     ├─ 设计实体模型和用例
│     └─ 审查业务逻辑的正确性
│
├─ 资深工程师 (Senior Engineer)
│  └─ 职责：
│     ├─ 实现 Core 的高质量代码
│     ├─ 设计业务规则的数据结构
│     └─ 维护单元测试和文档
│
└─ 质量保证 (QA)
   └─ 职责：
      ├─ 设计 Core 的单元测试
      ├─ 验证业务规则的完整性
      └─ 监控代码覆盖率
```

### 团队会议

```
周会议（每周一次，1 小时）
├─ 新增业务规则讨论
├─ Core 设计评审
├─ 潜在的规则冲突识别
└─ 技术债反馈

月会议（每月一次，2-3 小时）
├─ 回顾过去一个月的 Core 改动
├─ 业务规则变更的长期影响
├─ 技术债清单和优先级
└─ Core 质量指标分析（覆盖率、复杂度、修改频率）

季度评审（每季度一次，半天）
├─ Core 的演化路径规划
├─ 架构问题识别和改进计划
├─ 人员培养和知识转移
└─ 与 Adapter 团队的协调
```

---

## Core 与 Adapter 的协作边界

### 边界定义

```rust
// ✅ Core 定义接口（Trait）
pub trait OrderRepository: Send + Sync {
    async fn save(&self, order: &Order) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &OrderId) -> Result<Option<Order>, RepositoryError>;
    async fn find_by_user(&self, user_id: &UserId) -> Result<Vec<Order>, RepositoryError>;
}

// Adapter 实现接口
pub struct PostgresOrderRepository { /* ... */ }

impl OrderRepository for PostgresOrderRepository {
    async fn save(&self, order: &Order) -> Result<(), RepositoryError> {
        // PostgreSQL 特定的实现
    }
}

// Core 的用例依赖于接口，不依赖于具体实现
pub struct PlaceOrderInteractor {
    order_repo: Arc<dyn OrderRepository>,  // ← 依赖抽象
}

// ✅ 特点：
// 1. Core 定义需要什么（接口）
// 2. Adapter 实现怎么做（具体实现）
// 3. Core 与 Adapter 通过接口解耦
```

### 协作流程

```
需求来源：新增"取消订单"功能

步骤 1：Core 团队设计
├─ 分析业务规则：哪些状态的订单可以取消？
├─ 设计 Order::cancel() 方法
├─ 编写单元测试
└─ 交付给 Adapter 团队：
   "Order 实体现在支持 cancel() 方法"

步骤 2：Adapter 团队实现
├─ 接收 Order::cancel() 方法
├─ 设计 REST API: DELETE /orders/{id}
├─ 实现 OrderController::cancel_order()
├─ 修改数据库状态转移规则
└─ 编写集成测试

步骤 3：集成测试
├─ Core: Order::cancel() 逻辑正确
├─ Adapter: HTTP 请求正确转换
├─ 端到端: REST API 可以成功取消订单

特点：
✅ Core 不需要了解 HTTP 如何工作
✅ Adapter 不需要了解订单的业务规则
✅ 两个团队可以并行工作
```

---

## 常见错误和反模式

### 反模式 1：贫血模型（Anemic Model）

#### 定义

实体类只有数据，没有业务逻辑。所有逻辑都在 Service 中。

#### 错误示例

```java
// ❌ 反模式：Order 是一个数据容器
public class Order {
    private String id;
    private String symbol;
    private BigDecimal price;
    private String status;

    // 只有 getter/setter，没有业务逻辑
    public String getId() { return id; }
    public void setId(String id) { this.id = id; }
    public String getStatus() { return status; }
    public void setStatus(String status) { this.status = status; }
}

// 所有逻辑都在 Service 中
@Service
public class OrderService {
    public void cancelOrder(String orderId) {
        Order order = orderRepo.findById(orderId);

        // 业务逻辑分散在 Service 中
        if (order.getStatus().equals("FILLED")) {
            throw new Exception("Cannot cancel filled order");
        }

        order.setStatus("CANCELLED");
        orderRepo.save(order);
    }

    public void fillOrder(String orderId, BigDecimal quantity) {
        Order order = orderRepo.findById(orderId);

        // 更多业务逻辑
        // ...
    }
}

// ❌ 问题：
// 1. Order 没有自卫能力，可能被设置为无效状态
// 2. 业务规则分散在多个 Service 中
// 3. 无法有效地单元测试业务逻辑
// 4. 代码重复（相同的规则在多个地方检查）
```

#### 正确做法

```rust
// ✅ 充血模型：Order 包含业务逻辑
pub struct Order {
    id: OrderId,
    status: OrderStatus,
}

impl Order {
    pub fn cancel(mut self) -> Result<Self, DomainError> {
        // 业务逻辑在实体中
        if self.status == OrderStatus::Filled {
            return Err(DomainError::CannotCancelFilled);
        }

        self.status = OrderStatus::Cancelled;
        Ok(self)
    }

    pub fn fill(&mut self, qty: Quantity) -> Result<(), DomainError> {
        // 更多业务逻辑
        // ...
    }
}

// Service 只负责编排
pub struct CancelOrderService {
    order_repo: Arc<dyn OrderRepository>,
}

impl CancelOrderService {
    pub async fn cancel(&self, order_id: OrderId) -> Result<(), Error> {
        let order = self.order_repo.find_by_id(&order_id).await?;
        let cancelled = order.cancel()?;  // ← 业务逻辑委托给 Order
        self.order_repo.save(&cancelled).await?;
        Ok(())
    }
}

// ✅ 优点：
// 1. Order 保护自己的不变量
// 2. 业务规则内聚在实体中
// 3. Service 专注于流程编排
// 4. 易于测试和理解
```

---

### 反模式 2：Adapter 特性泄露到 Core

#### 错误示例

```rust
// ❌ 反模式：Core 依赖 Adapter
use actix_web::HttpResponse;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Order {
    id: String,
    symbol: String,
    #[sqlx(rename = "created_at")]
    #[serde(rename = "createdAt")]
    created_at: String,
}

impl Order {
    pub async fn load_from_db(id: &str, pool: &SqlxPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub fn to_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

// ❌ 问题：
// 1. Order 依赖 sqlx（数据库库）
// 2. Order 依赖 serde（序列化库）
// 3. Order 依赖 actix_web（HTTP 框架）
// 4. Order 无法独立使用
```

#### 正确做法

```rust
// ✅ 正确：Core 完全独立
pub struct Order {
    id: OrderId,
    symbol: Symbol,
    created_at: DateTime,
}

impl Order {
    pub fn new(...) -> Result<Self, DomainError> { /* ... */ }
}

// Infrastructure 层处理数据库
#[derive(sqlx::FromRow)]
pub struct OrderDbModel {
    pub id: String,
    pub symbol: String,
    pub created_at: String,
}

impl TryFrom<OrderDbModel> for Order {
    type Error = DomainError;

    fn try_from(model: OrderDbModel) -> Result<Self, Self::Error> {
        Order::new(
            OrderId::new(model.id)?,
            Symbol::new(model.symbol)?,
        )
    }
}

// Interface 层处理 HTTP 和序列化
#[derive(Serialize)]
pub struct OrderDto {
    id: String,
    symbol: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

impl From<&Order> for OrderDto {
    fn from(order: &Order) -> Self {
        OrderDto {
            id: order.id().to_string(),
            symbol: order.symbol().to_string(),
            created_at: order.created_at().to_iso_string(),
        }
    }
}

// ✅ 优点：
// 1. Order 完全独立，无框架依赖
// 2. 转换由 Adapter 负责
// 3. Core 和 Adapter 完全解耦
```

---

## 实战案例：金融交易系统的 Core

### 金融交易系统的 Core 包含什么

```
Core 的核心模块
├─ Entities（实体层）
│  ├─ Order（订单实体）
│  ├─ Position（头寸实体）
│  ├─ Account（账户实体）
│  ├─ Symbol（交易对）
│  └─ User（用户）
│
├─ Value Objects（值对象）
│  ├─ OrderId, PositionId, UserId
│  ├─ Price, Quantity, Money
│  ├─ OrderStatus, OrderSide
│  └─ TimeInForce, OrderType
│
├─ Domain Services（领域服务）
│  ├─ OrderValidator（订单验证）
│  ├─ RiskCalculator（风控计算）
│  ├─ ProfitLossCalculator（收益计算）
│  └─ PositionManager（头寸管理）
│
├─ Use Cases（用例）
│  ├─ PlaceOrderUseCase
│  ├─ CancelOrderUseCase
│  ├─ FillOrderUseCase
│  ├─ ClosePositionUseCase
│  └─ CalculatePortfolioRiskUseCase
│
├─ Events（领域事件）
│  ├─ OrderPlacedEvent
│  ├─ OrderCancelledEvent
│  ├─ OrderFilledEvent
│  ├─ PositionClosedEvent
│  └─ RiskLimitExceededEvent
│
└─ Repositories（仓储接口）
   ├─ OrderRepository（接口定义）
   ├─ PositionRepository（接口定义）
   ├─ AccountRepository（接口定义）
   └─ UserRepository（接口定义）
```

### Core 的代码示例

```rust
// src/domain/entities/order.rs
pub struct Order {
    id: OrderId,
    user_id: UserId,
    symbol: Symbol,
    price: Price,
    quantity: Quantity,
    side: OrderSide,
    status: OrderStatus,
    created_at: DateTime,
    filled_quantity: Quantity,
    avg_fill_price: Option<Price>,
}

// src/domain/entities/position.rs
pub struct Position {
    id: PositionId,
    user_id: UserId,
    symbol: Symbol,
    side: OrderSide,
    quantity: Quantity,
    entry_price: Price,
    current_price: Price,
    unrealized_pnl: Money,
    realized_pnl: Money,
}

// src/domain/services/risk_calculator.rs
#[async_trait]
pub trait RiskCalculator: Send + Sync {
    async fn calculate_portfolio_risk(
        &self,
        user: &User,
        positions: &[Position],
    ) -> Result<PortfolioRisk, DomainError>;

    async fn validate_order_risk(
        &self,
        user: &User,
        order: &Order,
    ) -> Result<bool, DomainError>;
}

// src/application/usecases/place_order.rs
pub struct PlaceOrderRequest {
    pub user_id: UserId,
    pub symbol: Symbol,
    pub price: Price,
    pub quantity: Quantity,
    pub side: OrderSide,
}

pub struct PlaceOrderResponse {
    pub order_id: OrderId,
    pub status: OrderStatus,
}

#[async_trait]
pub trait PlaceOrderUseCase: Send + Sync {
    async fn execute(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<PlaceOrderResponse, UseCaseError>;
}

pub struct PlaceOrderInteractor {
    user_repo: Arc<dyn UserRepository>,
    order_repo: Arc<dyn OrderRepository>,
    account_repo: Arc<dyn AccountRepository>,
    risk_calculator: Arc<dyn RiskCalculator>,
    event_publisher: Arc<dyn EventPublisher>,
}

#[async_trait]
impl PlaceOrderUseCase for PlaceOrderInteractor {
    async fn execute(
        &self,
        request: PlaceOrderRequest,
    ) -> Result<PlaceOrderResponse, UseCaseError> {
        // 1. 加载用户和账户
        let user = self.user_repo.find_by_id(&request.user_id).await?;
        let account = self.account_repo.find_by_user(&request.user_id).await?;

        // 2. 创建订单实体（Core 逻辑开始）
        let order = Order::new(
            OrderId::generate(),
            request.user_id.clone(),
            request.symbol.clone(),
            request.price,
            request.quantity,
            request.side.clone(),
        )?;

        // 3. 验证订单
        order.validate(&user, &account)?;

        // 4. 验证风控
        let is_risk_valid = self.risk_calculator
            .validate_order_risk(&user, &order)
            .await?;

        if !is_risk_valid {
            return Err(UseCaseError::RiskLimitExceeded);
        }

        // 5. 持久化订单
        self.order_repo.save(&order).await?;

        // 6. 发布事件
        self.event_publisher.publish(
            OrderPlacedEvent::from(&order)
        ).await?;

        Ok(PlaceOrderResponse {
            order_id: order.id.clone(),
            status: order.status.clone(),
        })
    }
}
```

---

## 检查清单和度量指标

### Core 健康度量指标

```
指标                        目标值        警告值       危险值
────────────────────────────────────────────────────────
单元测试覆盖率               >= 95%        < 95%       < 90%
圈复杂度平均值               <= 5          6-8         > 8
代码行数平均方法长度          <= 20         21-50       > 50
外部依赖数量                  0            1-2         >= 3
代码重复度 (DRY)             100%          95-99%      < 95%
```

### Core 团队的 Sprint 检查清单

```
开始 Sprint 时：
□ Core 是否有明确的待办事项？
□ 是否与 Adapter 团队协调了接口？
□ 是否有新的业务规则需要添加？
□ 是否有技术债需要处理？

完成 Sprint 时：
□ 是否所有代码都有单元测试？
□ 是否所有测试都通过？
□ 是否测试覆盖率没有下降？
□ 是否所有外部依赖都被隔离？
□ 是否有代码复杂度升高的地方？
□ 是否有知识转移给新人？
□ 是否与 Adapter 团队有无缝集成？
```

---

## 总结：Core 是系统的灵魂

### Core 的三个真理

1. **Core 是资产，不是成本**
   - 每一行 Core 代码都代表积累的业务知识
   - 保护 Core 就是保护竞争力
   - 投资 Core 的质量是最高 ROI 的选择

2. **Core 的质量决定系统的上限**
   - 无论 Adapter 多好，Core 有 bug 系统就是错的
   - Core 越清晰，系统越稳定、越易演化
   - Core 腐化导致整个系统的灵活性下降

3. **Core 不可替代**
   - Adapter 可以随时更换（语言、框架、数据库）
   - Core 无法替代（代表了公司的业务智慧）
   - 删除 Core 就是删除竞争力

### 给高管和决策者的建议

```
如果你的系统已经运行 2+ 年，并且：
- 新功能开发速度持续下降
- 故障变得越来越频繁
- 修改变得越来越困难

那么问题很可能在于：Core 没有得到足够的保护

解决方案：
1. 进行 Core 审计（找出已腐化的地方）
2. 建立 Core 团队（专门维护 Core 的质量）
3. 制定 Core 质量标准（覆盖率、复杂度等）
4. 投入时间清理技术债（不要等到系统瘫痪）
5. 定期的 Core 健康检查（每月）

成本：初期投入 50-100 万（清理债务）
收益：后续每年节省 200-500 万（减少故障、加快开发速度）

ROI: 第二年开始为正
```

---

**文档版本**: v1.0
**最后更新**: 2025-12-29
**建议阅读人群**: 架构师、技术主管、资深工程师、CTO

---

