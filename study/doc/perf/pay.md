# 跨境支付系统架构设计

> **文档类型**: 首席架构师技术方案
> **版本**: v2.0
> **创建日期**: 2025-12-03
> **最后更新**: 2025-12-03
> **机密级别**: 内部机密

---

## 1. 执行摘要

### 1.1 业务愿景

构建新一代跨境支付基础设施，实现：
- **T+0 实时到账** (目标 95% 交易)
- **低成本**: 费率 < 1% (vs 传统 3-5%)
- **多通道**: 支持 50+ 国家/地区
- **合规优先**: 满足全球监管要求

### 1.2 核心竞争力

```
┌─────────────────────────────────────────────────────────────────┐
│                      竞争力矩阵                                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   速度 ●────────────────────────────────────○ 传统银行 (T+3)    │
│        ↑ 我们 (秒级)                                            │
│                                                                 │
│   成本 ●────────────────────────────────────○ SWIFT (高)       │
│        ↑ 我们 (低)                                              │
│                                                                 │
│   覆盖 ●────────────────────────────────────● 全球网络          │
│                                                                 │
│   合规 ●────────────────────────────────────● 多牌照            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. 业务架构

### 2.1 业务能力地图

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           跨境支付业务能力                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │  B2B 贸易   │  │  B2C 电商   │  │  C2C 汇款   │  │  供应链金融  │   │
│  │  付款       │  │  收单       │  │  转账       │  │  融资       │   │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │
│         │                │                │                │          │
│         └────────────────┴────────────────┴────────────────┘          │
│                                  │                                     │
│  ┌───────────────────────────────┴───────────────────────────────┐    │
│  │                     核心支付能力层                              │    │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ │    │
│  │  │ 收款    │ │ 付款    │ │ 换汇    │ │ 清算    │ │ 对账    │ │    │
│  │  │ Gateway │ │ Payout  │ │ FX      │ │ Settle  │ │ Recon   │ │    │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                  │                                     │
│  ┌───────────────────────────────┴───────────────────────────────┐    │
│  │                     基础能力层                                  │    │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ │    │
│  │  │ 账户    │ │ 风控    │ │ 合规    │ │ 路由    │ │ 通道    │ │    │
│  │  │ Account │ │ Risk    │ │ Compli. │ │ Router  │ │ Channel │ │    │
│  │  └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘ │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 2.2 核心业务流程

#### 2.2.1 跨境汇款流程 (B2C/C2C)

```
发起方                    我们平台                              收款方
  │                          │                                    │
  │  1. 创建汇款订单         │                                    │
  │─────────────────────────►│                                    │
  │                          │                                    │
  │  2. 返回报价 (汇率+费用)  │                                    │
  │◄─────────────────────────│                                    │
  │                          │                                    │
  │  3. 确认并支付           │                                    │
  │─────────────────────────►│                                    │
  │                          │  ┌─────────────────────────────┐   │
  │                          │  │ 内部处理:                    │   │
  │                          │  │ - KYC/AML 检查              │   │
  │                          │  │ - 制裁名单筛查              │   │
  │                          │  │ - 风险评分                  │   │
  │                          │  │ - 换汇处理                  │   │
  │                          │  │ - 通道路由选择              │   │
  │                          │  └─────────────────────────────┘   │
  │                          │                                    │
  │                          │  4. 通过最优通道出款              │
  │                          │───────────────────────────────────►│
  │                          │                                    │
  │  5. 状态通知 (实时)       │                                    │
  │◄─────────────────────────│                                    │
  │                          │                                    │
  │                          │  6. 收款确认                       │
  │                          │◄───────────────────────────────────│
  │                          │                                    │
  │  7. 完成通知             │                                    │
  │◄─────────────────────────│                                    │
  │                          │                                    │

时间线:
┌────────────────────────────────────────────────────────────────────┐
│ 0s        10s       30s       1min      5min      30min     T+1   │
│ │          │         │         │         │          │        │    │
│ ●──────────●─────────●─────────●─────────●──────────●────────●    │
│ 下单    风控通过   换汇完成  通道提交  银行处理  到账通知  最终确认 │
└────────────────────────────────────────────────────────────────────┘
```

#### 2.2.2 B2B 贸易付款流程

```
买方企业                   我们平台                      卖方企业
   │                          │                            │
   │  1. 提交付款申请         │                            │
   │  (发票+合同)             │                            │
   │─────────────────────────►│                            │
   │                          │                            │
   │                          │  ┌──────────────────────┐  │
   │                          │  │ 贸易背景审核:         │  │
   │                          │  │ - 发票真实性验证      │  │
   │                          │  │ - 合同匹配            │  │
   │                          │  │ - 关联交易检查        │  │
   │                          │  │ - 反洗钱审查          │  │
   │                          │  └──────────────────────┘  │
   │                          │                            │
   │  2. 审批结果             │                            │
   │◄─────────────────────────│                            │
   │                          │                            │
   │  3. 确认付款             │                            │
   │─────────────────────────►│                            │
   │                          │                            │
   │                          │  ┌──────────────────────┐  │
   │                          │  │ 资金处理:             │  │
   │                          │  │ - 买方账户扣款        │  │
   │                          │  │ - 换汇 (如需)         │  │
   │                          │  │ - 通道路由            │  │
   │                          │  │ - 出款执行            │  │
   │                          │  └──────────────────────┘  │
   │                          │                            │
   │                          │  4. 付款至卖方            │
   │                          │───────────────────────────►│
   │                          │                            │
   │  5. 水单/凭证            │                            │
   │◄─────────────────────────│───────────────────────────►│
   │                          │                            │
```

### 2.3 产品矩阵

| 产品线 | 目标客户 | 核心功能 | 费率区间 | 到账时效 |
|--------|----------|----------|----------|----------|
| **GlobalPay** | 中小企业 | 贸易付款、供应商管理 | 0.5-1.0% | T+0 ~ T+1 |
| **RemitNow** | 个人用户 | 跨境汇款、留学缴费 | 0.3-0.8% | 实时 ~ 1小时 |
| **MerchantHub** | 电商卖家 | 全球收款、多币种 | 0.8-1.5% | T+1 ~ T+3 |
| **TreasuryCloud** | 大型企业 | 资金归集、FX 对冲 | 协议定价 | 实时 |
| **PayrollGlobal** | HR/企业 | 全球发薪、合规代缴 | 固定+比例 | 当地薪资日 |

---

## 3. 应用架构

### 3.1 系统全景图

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              接入层 (Access Layer)                           │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │ Web Portal│  │Mobile App│  │ Open API │  │ Partner  │  │ Internal │      │
│  │ (商户后台)│  │ (C端APP) │  │ (开放接口)│  │ (合作方) │  │ (运营后台)│      │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘      │
│       └─────────────┴─────────────┴─────────────┴─────────────┘            │
│                                   │                                         │
│                          ┌────────┴────────┐                               │
│                          │   API Gateway   │                               │
│                          │ (Kong/APISIX)   │                               │
│                          └────────┬────────┘                               │
└──────────────────────────────────┼──────────────────────────────────────────┘
                                   │
┌──────────────────────────────────┼──────────────────────────────────────────┐
│                          业务服务层 (BFF)                                    │
├──────────────────────────────────┼──────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──┴───────┐  ┌──────────┐  ┌──────────┐      │
│  │ 商户 BFF │  │ C端 BFF  │  │ 开放API  │  │ 合作方   │  │ 运营 BFF │      │
│  │          │  │          │  │ BFF      │  │ BFF      │  │          │      │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘      │
└───────┴─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘
                                   │
┌──────────────────────────────────┼──────────────────────────────────────────┐
│                           核心服务层 (Core Services)                         │
├──────────────────────────────────┴──────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        交易域 (Transaction Domain)                    │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ 订单服务 │  │ 支付服务 │  │ 收款服务 │  │ 退款服务 │            │   │
│  │  │ Order    │  │ Payment  │  │ Collection│  │ Refund   │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        资金域 (Fund Domain)                           │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ 账户服务 │  │ 余额服务 │  │ 清算服务 │  │ 结算服务 │            │   │
│  │  │ Account  │  │ Balance  │  │ Clearing │  │ Settle   │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        换汇域 (FX Domain)                             │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ 报价服务 │  │ 换汇服务 │  │ 头寸服务 │  │ 对冲服务 │            │   │
│  │  │ Quote    │  │ Exchange │  │ Position │  │ Hedge    │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        风控域 (Risk Domain)                           │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ 反欺诈   │  │ 反洗钱   │  │ 制裁筛查 │  │ 额度管控 │            │   │
│  │  │ Fraud    │  │ AML      │  │ Sanction │  │ Limit    │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                        合规域 (Compliance Domain)                     │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │   │
│  │  │ KYC/KYB  │  │ 贸易审核 │  │ 监管报送 │  │ 牌照管理 │            │   │
│  │  │ Identity │  │ Trade    │  │ Report   │  │ License  │            │   │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
                                   │
┌──────────────────────────────────┼──────────────────────────────────────────┐
│                           通道层 (Channel Layer)                             │
├──────────────────────────────────┴──────────────────────────────────────────┤
│                                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐      │
│  │ 通道路由 │  │ 通道适配 │  │ 通道监控 │  │ 容灾切换 │  │ 成本计算 │      │
│  │ Router   │  │ Adapter  │  │ Monitor  │  │ Failover │  │ Costing  │      │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘      │
│       └─────────────┴─────────────┴─────────────┴─────────────┘            │
│                                   │                                         │
│  ┌────────────────────────────────┴────────────────────────────────────┐   │
│  │                         外部通道接口                                  │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ │   │
│  │  │ SWIFT  │ │ 银行直 │ │ 本地清 │ │ 卡组织 │ │ 数字钱 │ │ 区块链 │ │   │
│  │  │ GPI    │ │ 联网关 │ │ 算网络 │ │ VISA等 │ │ 包     │ │ 网络   │ │   │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ └────────┘ │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### 3.2 核心服务详细设计

#### 3.2.1 订单服务 (Order Service)

```rust
// 订单领域模型
pub mod order {
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;

    #[derive(Debug, Clone)]
    pub struct PaymentOrder {
        pub order_id: OrderId,
        pub merchant_id: MerchantId,
        pub order_type: OrderType,
        pub source: PaymentSource,
        pub destination: PaymentDestination,
        pub amount: Money,
        pub fee: Fee,
        pub fx_quote: Option<FxQuote>,
        pub status: OrderStatus,
        pub compliance_status: ComplianceStatus,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Debug, Clone)]
    pub enum OrderType {
        B2BPayment,      // 企业付款
        B2CRemittance,   // 个人汇款
        C2CTransfer,     // 个人转账
        MerchantPayout,  // 商户提现
        PayrollPayment,  // 薪资发放
    }

    #[derive(Debug, Clone)]
    pub struct PaymentSource {
        pub country: CountryCode,
        pub currency: CurrencyCode,
        pub account: AccountInfo,
        pub payer: PartyInfo,
    }

    #[derive(Debug, Clone)]
    pub struct PaymentDestination {
        pub country: CountryCode,
        pub currency: CurrencyCode,
        pub account: AccountInfo,
        pub payee: PartyInfo,
        pub purpose: PaymentPurpose,
    }

    #[derive(Debug, Clone)]
    pub enum OrderStatus {
        Created,           // 已创建
        PendingPayment,    // 待付款
        PaymentReceived,   // 已收款
        ComplianceCheck,   // 合规审核中
        FxProcessing,      // 换汇处理中
        ChannelSubmitted,  // 已提交通道
        ChannelProcessing, // 通道处理中
        Completed,         // 已完成
        Failed,            // 失败
        Cancelled,         // 已取消
        Refunding,         // 退款中
        Refunded,          // 已退款
    }

    impl PaymentOrder {
        pub fn create(request: CreateOrderRequest) -> Result<Self, OrderError> {
            // 1. 参数校验
            request.validate()?;

            // 2. 生成订单ID
            let order_id = OrderId::generate();

            // 3. 计算费用
            let fee = FeeCalculator::calculate(&request)?;

            Ok(Self {
                order_id,
                status: OrderStatus::Created,
                // ... 其他字段
            })
        }

        pub fn submit_to_channel(&mut self, channel: &Channel) -> Result<(), OrderError> {
            // 状态校验
            self.ensure_status(OrderStatus::FxProcessing)?;

            // 提交通道
            self.status = OrderStatus::ChannelSubmitted;
            self.updated_at = Utc::now();

            Ok(())
        }
    }
}
```

#### 3.2.2 账户服务 (Account Service)

```rust
// 账户领域模型
pub mod account {
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use std::collections::HashMap;

    /// 多币种账户
    #[derive(Debug, Clone)]
    pub struct MultiCurrencyAccount {
        pub account_id: AccountId,
        pub merchant_id: MerchantId,
        pub account_type: AccountType,
        pub wallets: HashMap<CurrencyCode, Wallet>,
        pub status: AccountStatus,
        pub created_at: DateTime<Utc>,
    }

    /// 单币种钱包
    #[derive(Debug, Clone)]
    pub struct Wallet {
        pub currency: CurrencyCode,
        pub available_balance: Decimal,  // 可用余额
        pub frozen_balance: Decimal,     // 冻结余额
        pub pending_in: Decimal,         // 待入账
        pub pending_out: Decimal,        // 待出账
    }

    impl Wallet {
        pub fn total_balance(&self) -> Decimal {
            self.available_balance + self.frozen_balance
        }

        pub fn freeze(&mut self, amount: Decimal) -> Result<(), AccountError> {
            if self.available_balance < amount {
                return Err(AccountError::InsufficientBalance);
            }
            self.available_balance -= amount;
            self.frozen_balance += amount;
            Ok(())
        }

        pub fn unfreeze(&mut self, amount: Decimal) -> Result<(), AccountError> {
            if self.frozen_balance < amount {
                return Err(AccountError::InsufficientFrozen);
            }
            self.frozen_balance -= amount;
            self.available_balance += amount;
            Ok(())
        }

        pub fn debit(&mut self, amount: Decimal) -> Result<(), AccountError> {
            if self.frozen_balance < amount {
                return Err(AccountError::InsufficientFrozen);
            }
            self.frozen_balance -= amount;
            Ok(())
        }

        pub fn credit(&mut self, amount: Decimal) {
            self.available_balance += amount;
        }
    }

    /// 账务流水
    #[derive(Debug, Clone)]
    pub struct AccountEntry {
        pub entry_id: EntryId,
        pub account_id: AccountId,
        pub currency: CurrencyCode,
        pub direction: EntryDirection,
        pub amount: Decimal,
        pub balance_after: Decimal,
        pub entry_type: EntryType,
        pub reference_id: String,
        pub created_at: DateTime<Utc>,
    }

    #[derive(Debug, Clone)]
    pub enum EntryType {
        Deposit,           // 充值
        Withdrawal,        // 提现
        PaymentOut,        // 付款出
        PaymentIn,         // 收款入
        FxConversion,      // 换汇
        Fee,               // 手续费
        Refund,            // 退款
        Adjustment,        // 调账
    }
}
```

#### 3.2.3 通道路由服务 (Channel Router)

```rust
// 通道路由策略
pub mod router {
    use rust_decimal::Decimal;
    use std::collections::HashMap;

    /// 路由决策引擎
    pub struct ChannelRouter {
        channels: Vec<Channel>,
        rules: Vec<RoutingRule>,
        strategies: HashMap<String, Box<dyn RoutingStrategy>>,
    }

    impl ChannelRouter {
        /// 选择最优通道
        pub fn select_channel(&self, request: &RouteRequest) -> Result<RoutingDecision, RouterError> {
            // 1. 筛选可用通道
            let available = self.filter_available_channels(request)?;

            if available.is_empty() {
                return Err(RouterError::NoAvailableChannel);
            }

            // 2. 应用路由规则
            let filtered = self.apply_routing_rules(&available, request)?;

            // 3. 评分排序
            let scored = self.score_channels(&filtered, request);

            // 4. 选择最优
            let selected = scored.into_iter()
                .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
                .ok_or(RouterError::ScoringFailed)?;

            Ok(RoutingDecision {
                primary_channel: selected.channel,
                backup_channels: self.select_backups(&filtered, &selected),
                estimated_time: selected.estimated_time,
                estimated_cost: selected.estimated_cost,
            })
        }

        fn filter_available_channels(&self, request: &RouteRequest) -> Result<Vec<&Channel>, RouterError> {
            let available: Vec<_> = self.channels.iter()
                .filter(|c| c.supports_corridor(&request.source_country, &request.dest_country))
                .filter(|c| c.supports_currency(&request.source_currency, &request.dest_currency))
                .filter(|c| c.is_healthy())
                .filter(|c| c.within_limits(&request.amount))
                .collect();

            Ok(available)
        }

        fn score_channels(&self, channels: &[&Channel], request: &RouteRequest) -> Vec<ScoredChannel> {
            channels.iter().map(|channel| {
                let cost_score = self.calculate_cost_score(channel, request);
                let speed_score = self.calculate_speed_score(channel, request);
                let reliability_score = channel.success_rate * 100.0;
                let capacity_score = self.calculate_capacity_score(channel);

                // 加权评分
                let score = cost_score * 0.3
                    + speed_score * 0.25
                    + reliability_score * 0.30
                    + capacity_score * 0.15;

                ScoredChannel {
                    channel: (*channel).clone(),
                    score,
                    estimated_time: channel.avg_processing_time,
                    estimated_cost: channel.calculate_cost(request),
                }
            }).collect()
        }
    }

    /// 通道信息
    #[derive(Debug, Clone)]
    pub struct Channel {
        pub channel_id: String,
        pub channel_name: String,
        pub channel_type: ChannelType,
        pub corridors: Vec<Corridor>,
        pub currencies: Vec<CurrencyPair>,
        pub fee_structure: FeeStructure,
        pub limits: ChannelLimits,
        pub success_rate: f64,
        pub avg_processing_time: Duration,
        pub status: ChannelStatus,
        pub daily_volume: Decimal,
        pub daily_limit: Decimal,
    }

    #[derive(Debug, Clone)]
    pub enum ChannelType {
        SwiftGpi,           // SWIFT GPI
        LocalClearing,      // 本地清算 (ACH, SEPA, FPS等)
        BankDirect,         // 银行直连
        CardNetwork,        // 卡组织
        DigitalWallet,      // 数字钱包
        Cryptocurrency,     // 加密货币
        MobilePayment,      // 移动支付
    }

    /// 费率结构
    #[derive(Debug, Clone)]
    pub struct FeeStructure {
        pub fixed_fee: Decimal,
        pub percentage_fee: Decimal,
        pub min_fee: Decimal,
        pub max_fee: Option<Decimal>,
        pub fx_markup: Decimal,  // 换汇加点
    }

    impl FeeStructure {
        pub fn calculate(&self, amount: Decimal) -> Decimal {
            let calculated = self.fixed_fee + (amount * self.percentage_fee / Decimal::from(100));
            let fee = calculated.max(self.min_fee);
            match self.max_fee {
                Some(max) => fee.min(max),
                None => fee,
            }
        }
    }
}
```

### 3.3 状态机设计

```
订单状态机:

                                    ┌─────────┐
                                    │ Created │
                                    └────┬────┘
                                         │
                              ┌──────────┴──────────┐
                              ▼                     ▼
                     ┌────────────────┐      ┌───────────┐
                     │ PendingPayment │      │ Cancelled │
                     └───────┬────────┘      └───────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │PaymentReceived │
                    └───────┬────────┘
                            │
                            ▼
                   ┌─────────────────┐
                   │ComplianceCheck  │────────────┐
                   └───────┬─────────┘            │
                           │                      ▼
              ┌────────────┴────────────┐  ┌───────────┐
              ▼                         ▼  │  Failed   │
     ┌────────────────┐        ┌──────────┐└───────────┘
     │  FxProcessing  │        │ Rejected │
     └───────┬────────┘        └──────────┘
             │
             ▼
    ┌─────────────────┐
    │ChannelSubmitted │
    └───────┬─────────┘
            │
            ▼
   ┌─────────────────┐
   │ChannelProcessing│────────────┐
   └───────┬─────────┘            │
           │                      ▼
           │               ┌───────────┐
           ▼               │  Failed   │───────┐
    ┌───────────┐          └───────────┘       │
    │ Completed │                              ▼
    └───────────┘                       ┌───────────┐
                                        │ Refunding │
                                        └─────┬─────┘
                                              │
                                              ▼
                                        ┌───────────┐
                                        │ Refunded  │
                                        └───────────┘
```

---

## 4. 技术架构

### 4.1 技术栈选型

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              技术栈全景                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  前端层:                                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐                        │
│  │ React   │  │ React   │  │ Flutter │  │ Ant     │                        │
│  │ (Web)   │  │ Native  │  │ (跨端)  │  │ Design  │                        │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘                        │
│                                                                             │
│  网关层:                                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐                                     │
│  │ Kong/   │  │ Envoy   │  │ Nginx   │                                     │
│  │ APISIX  │  │ (Mesh)  │  │ (CDN)   │                                     │
│  └─────────┘  └─────────┘  └─────────┘                                     │
│                                                                             │
│  服务层:                                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │ Rust    │  │ Go      │  │ Java    │  │ Python  │  │ Node.js │          │
│  │ (核心)  │  │ (通道)  │  │ (业务)  │  │ (AI/ML) │  │ (BFF)   │          │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘          │
│                                                                             │
│  中间件:                                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │ Kafka   │  │ Redis   │  │ Elastic │  │ Consul  │  │ Vault   │          │
│  │ (消息)  │  │ (缓存)  │  │ (搜索)  │  │ (注册)  │  │ (密钥)  │          │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘          │
│                                                                             │
│  数据层:                                                                     │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │PostgreSQL│ │TiDB/    │  │ClickHouse│ │ MinIO   │  │ HBase   │          │
│  │ (主库)  │  │CockroachDB│ (分析)  │  │ (对象)  │  │ (大表)  │          │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘          │
│                                                                             │
│  基础设施:                                                                   │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐          │
│  │Kubernetes│ │ Docker  │  │Terraform│  │ Istio   │  │Prometheus│          │
│  │ (编排)  │  │ (容器)  │  │ (IaC)   │  │ (Mesh)  │  │ (监控)  │          │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘  └─────────┘          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 4.2 数据架构

#### 4.2.1 数据模型

```sql
-- 核心表设计

-- 商户表
CREATE TABLE merchants (
    merchant_id         UUID PRIMARY KEY,
    merchant_name       VARCHAR(255) NOT NULL,
    business_type       VARCHAR(50) NOT NULL,
    registration_country VARCHAR(2) NOT NULL,
    kyb_status          VARCHAR(20) NOT NULL,
    risk_level          VARCHAR(10) NOT NULL,
    status              VARCHAR(20) NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 账户表
CREATE TABLE accounts (
    account_id          UUID PRIMARY KEY,
    merchant_id         UUID NOT NULL REFERENCES merchants(merchant_id),
    account_type        VARCHAR(20) NOT NULL,
    status              VARCHAR(20) NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(merchant_id, account_type)
);

-- 钱包表 (多币种)
CREATE TABLE wallets (
    wallet_id           UUID PRIMARY KEY,
    account_id          UUID NOT NULL REFERENCES accounts(account_id),
    currency            CHAR(3) NOT NULL,
    available_balance   DECIMAL(20, 8) NOT NULL DEFAULT 0,
    frozen_balance      DECIMAL(20, 8) NOT NULL DEFAULT 0,
    version             BIGINT NOT NULL DEFAULT 0,  -- 乐观锁
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(account_id, currency),
    CHECK (available_balance >= 0),
    CHECK (frozen_balance >= 0)
);

-- 订单表
CREATE TABLE payment_orders (
    order_id            UUID PRIMARY KEY,
    merchant_id         UUID NOT NULL REFERENCES merchants(merchant_id),
    order_type          VARCHAR(30) NOT NULL,

    -- 来源信息
    source_country      CHAR(2) NOT NULL,
    source_currency     CHAR(3) NOT NULL,
    source_amount       DECIMAL(20, 8) NOT NULL,
    payer_name          VARCHAR(255) NOT NULL,
    payer_account       VARCHAR(100),

    -- 目的信息
    dest_country        CHAR(2) NOT NULL,
    dest_currency       CHAR(3) NOT NULL,
    dest_amount         DECIMAL(20, 8),
    payee_name          VARCHAR(255) NOT NULL,
    payee_account       VARCHAR(100) NOT NULL,
    payee_bank_code     VARCHAR(20),

    -- 换汇信息
    fx_rate             DECIMAL(20, 10),
    fx_quote_id         UUID,

    -- 费用信息
    fee_amount          DECIMAL(20, 8) NOT NULL DEFAULT 0,
    fee_currency        CHAR(3) NOT NULL,

    -- 通道信息
    channel_id          VARCHAR(50),
    channel_ref         VARCHAR(100),

    -- 状态
    status              VARCHAR(30) NOT NULL,
    compliance_status   VARCHAR(30) NOT NULL DEFAULT 'PENDING',

    -- 时间
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at        TIMESTAMPTZ,

    -- 索引优化
    INDEX idx_merchant_created (merchant_id, created_at DESC),
    INDEX idx_status_created (status, created_at DESC),
    INDEX idx_channel_ref (channel_id, channel_ref)
);

-- 账务流水表 (分区)
CREATE TABLE account_entries (
    entry_id            UUID PRIMARY KEY,
    account_id          UUID NOT NULL,
    wallet_id           UUID NOT NULL,
    currency            CHAR(3) NOT NULL,
    direction           CHAR(1) NOT NULL,  -- D: 借, C: 贷
    amount              DECIMAL(20, 8) NOT NULL,
    balance_after       DECIMAL(20, 8) NOT NULL,
    entry_type          VARCHAR(30) NOT NULL,
    reference_type      VARCHAR(30) NOT NULL,
    reference_id        UUID NOT NULL,
    description         TEXT,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    INDEX idx_account_created (account_id, created_at DESC),
    INDEX idx_reference (reference_type, reference_id)
) PARTITION BY RANGE (created_at);

-- 按月分区
CREATE TABLE account_entries_2025_01 PARTITION OF account_entries
    FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');
```

#### 4.2.2 数据分层

```
数据分层架构:

┌─────────────────────────────────────────────────────────────────────────┐
│                           ODS (操作数据层)                               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ 订单实时 │  │ 账务实时 │  │ 通道日志 │  │ 风控事件 │               │
│  │ (Kafka)  │  │ (Kafka)  │  │ (Kafka)  │  │ (Kafka)  │               │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘               │
│       └─────────────┴─────────────┴─────────────┘                      │
│                              │                                          │
│                    ┌─────────▼─────────┐                               │
│                    │   Flink/Spark     │                               │
│                    │   (实时ETL)       │                               │
│                    └─────────┬─────────┘                               │
└──────────────────────────────┼──────────────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────────────┐
│                           DWD (明细数据层)                               │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                        ClickHouse / Doris                         │  │
│  ├──────────┬──────────┬──────────┬──────────┬──────────────────────┤  │
│  │ 订单明细 │ 账务明细 │ 换汇明细 │ 通道明细 │ 风控明细             │  │
│  └──────────┴──────────┴──────────┴──────────┴──────────────────────┘  │
└──────────────────────────────┬──────────────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────────────┐
│                           DWS (汇总数据层)                               │
│  ┌──────────────────────────────────────────────────────────────────┐  │
│  │                        ClickHouse / Doris                         │  │
│  ├──────────┬──────────┬──────────┬──────────┬──────────────────────┤  │
│  │ 商户汇总 │ 通道汇总 │ 日报汇总 │ 币种汇总 │ 国家/地区汇总        │  │
│  └──────────┴──────────┴──────────┴──────────┴──────────────────────┘  │
└──────────────────────────────┬──────────────────────────────────────────┘
                               │
┌──────────────────────────────┼──────────────────────────────────────────┐
│                           ADS (应用数据层)                               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ BI 报表  │  │ 监管报送 │  │ 商户报表 │  │ 风险监控 │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
└─────────────────────────────────────────────────────────────────────────┘
```

### 4.3 部署架构

```
多区域部署架构:

                          ┌─────────────────────┐
                          │    Global DNS       │
                          │   (Route53/CF)      │
                          └──────────┬──────────┘
                                     │
              ┌──────────────────────┼──────────────────────┐
              │                      │                      │
              ▼                      ▼                      ▼
    ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
    │   Region: APAC  │    │  Region: EMEA   │    │ Region: AMER   │
    │  (Singapore)    │    │   (Frankfurt)   │    │  (Virginia)    │
    ├─────────────────┤    ├─────────────────┤    ├─────────────────┤
    │                 │    │                 │    │                 │
    │ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
    │ │ K8s Cluster │ │    │ │ K8s Cluster │ │    │ │ K8s Cluster │ │
    │ │ (Primary)   │ │    │ │ (Primary)   │ │    │ │ (Primary)   │ │
    │ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
    │                 │    │                 │    │                 │
    │ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
    │ │ PostgreSQL  │ │    │ │ PostgreSQL  │ │    │ │ PostgreSQL  │ │
    │ │ (Primary)   │◄┼────┼─│ (Replica)   │◄┼────┼─│ (Replica)   │ │
    │ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
    │                 │    │                 │    │                 │
    │ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
    │ │ Redis       │ │    │ │ Redis       │ │    │ │ Redis       │ │
    │ │ (Cluster)   │ │    │ │ (Cluster)   │ │    │ │ (Cluster)   │ │
    │ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
    │                 │    │                 │    │                 │
    │ 本地通道接入:    │    │ 本地通道接入:    │    │ 本地通道接入:    │
    │ - PayNow (SG)  │    │ - SEPA         │    │ - ACH          │
    │ - FPS (HK)     │    │ - FPS (UK)     │    │ - Fedwire      │
    │ - UPI (IN)     │    │ - TIPS         │    │ - RTP          │
    │ - 人民币跨境    │    │                 │    │                 │
    │                 │    │                 │    │                 │
    └────────┬────────┘    └────────┬────────┘    └────────┬────────┘
             │                      │                      │
             └──────────────────────┼──────────────────────┘
                                    │
                          ┌─────────▼─────────┐
                          │  跨区域网络      │
                          │  (专线/VPN)      │
                          └───────────────────┘

数据同步策略:
┌────────────────────────────────────────────────────────────────────────┐
│ 数据类型         │ 同步方式          │ 延迟      │ 一致性              │
├────────────────────────────────────────────────────────────────────────┤
│ 账户余额         │ 同步复制          │ < 10ms    │ 强一致性            │
│ 订单数据         │ 异步复制          │ < 100ms   │ 最终一致性          │
│ 配置数据         │ 全量同步          │ < 1s      │ 最终一致性          │
│ 日志/统计        │ 异步批量          │ < 1min    │ 最终一致性          │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 5. 风控与合规架构

### 5.1 风控体系

```
风控三道防线:

┌─────────────────────────────────────────────────────────────────────────┐
│                          第一道防线: 业务风控                            │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │   交易风控   │  │   额度管控   │  │   行为分析   │                  │
│  │              │  │              │  │              │                  │
│  │ - 金额限制   │  │ - 单笔限额   │  │ - 设备指纹   │                  │
│  │ - 频率限制   │  │ - 日累计     │  │ - 行为特征   │                  │
│  │ - 时间规则   │  │ - 月累计     │  │ - 异常检测   │                  │
│  │ - 地理规则   │  │ - 动态调整   │  │ - 关联分析   │                  │
│  └──────────────┘  └──────────────┘  └──────────────┘                  │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          第二道防线: 合规风控                            │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │    AML/CFT   │  │   制裁筛查   │  │   PEP 筛查   │                  │
│  │              │  │              │  │              │                  │
│  │ - 交易监控   │  │ - OFAC       │  │ - 政治人物   │                  │
│  │ - 可疑报告   │  │ - UN         │  │ - 关联人     │                  │
│  │ - 规则引擎   │  │ - EU         │  │ - 高风险人   │                  │
│  │ - ML 模型    │  │ - 本地名单   │  │              │                  │
│  └──────────────┘  └──────────────┘  └──────────────┘                  │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          第三道防线: 审计监督                            │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │   内部审计   │  │   监管报送   │  │   外部审计   │                  │
│  │              │  │              │  │              │                  │
│  │ - 操作审计   │  │ - STR/SAR    │  │ - 年度审计   │                  │
│  │ - 合规审计   │  │ - CTR        │  │ - 专项检查   │                  │
│  │ - 系统审计   │  │ - 定期报告   │  │ - 认证审核   │                  │
│  └──────────────┘  └──────────────┘  └──────────────┘                  │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.2 反洗钱 (AML) 系统

```rust
// AML 检测引擎
pub mod aml {
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;

    /// AML 检测引擎
    pub struct AmlEngine {
        rules: Vec<Box<dyn AmlRule>>,
        ml_model: MlModel,
        sanction_checker: SanctionChecker,
        case_manager: CaseManager,
    }

    impl AmlEngine {
        /// 交易检测
        pub async fn screen_transaction(&self, tx: &Transaction) -> AmlResult {
            let mut alerts = Vec::new();

            // 1. 规则引擎检测
            for rule in &self.rules {
                if let Some(alert) = rule.evaluate(tx).await {
                    alerts.push(alert);
                }
            }

            // 2. 制裁名单筛查
            let sanction_hits = self.sanction_checker.check(tx).await?;
            if !sanction_hits.is_empty() {
                alerts.push(Alert::SanctionHit(sanction_hits));
            }

            // 3. ML 模型评分
            let risk_score = self.ml_model.score(tx).await?;

            // 4. 综合决策
            let decision = self.make_decision(&alerts, risk_score);

            // 5. 创建案件 (如需要)
            if decision == AmlDecision::Review || decision == AmlDecision::Block {
                self.case_manager.create_case(tx, &alerts, risk_score).await?;
            }

            Ok(AmlResult {
                transaction_id: tx.id.clone(),
                decision,
                risk_score,
                alerts,
            })
        }

        fn make_decision(&self, alerts: &[Alert], risk_score: f64) -> AmlDecision {
            // 制裁命中直接阻止
            if alerts.iter().any(|a| matches!(a, Alert::SanctionHit(_))) {
                return AmlDecision::Block;
            }

            // 高风险交易人工审核
            if risk_score > 80.0 || alerts.len() > 3 {
                return AmlDecision::Review;
            }

            // 中风险增强监控
            if risk_score > 50.0 || !alerts.is_empty() {
                return AmlDecision::EnhancedMonitoring;
            }

            AmlDecision::Pass
        }
    }

    /// 结构化拆分检测
    pub struct StructuringRule {
        threshold: Decimal,        // 报告阈值 (如 $10,000)
        window: Duration,          // 检测窗口
        tolerance: Decimal,        // 容忍度
    }

    /// 快进快出检测
    pub struct RapidMovementRule {
        inflow_window: Duration,   // 入金窗口
        outflow_window: Duration,  // 出金窗口
        ratio_threshold: f64,      // 出入比阈值
    }
}
```

### 5.3 KYC/KYB 流程

```
KYC/KYB 分级验证:

┌─────────────────────────────────────────────────────────────────────────┐
│                           Level 1 - 基础验证                             │
├─────────────────────────────────────────────────────────────────────────┤
│  个人 (KYC):                      企业 (KYB):                           │
│  - 姓名                           - 公司名称                            │
│  - 身份证号/护照号                 - 注册号                             │
│  - 手机验证                       - 联系人信息                          │
│  - 邮箱验证                       - 邮箱验证                            │
│                                                                         │
│  限额: $500/笔, $2,000/月         限额: $10,000/笔, $50,000/月          │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           Level 2 - 标准验证                             │
├─────────────────────────────────────────────────────────────────────────┤
│  个人 (KYC):                      企业 (KYB):                           │
│  - 证件照片上传                   - 营业执照                            │
│  - 人脸识别                       - 公司章程                            │
│  - 地址证明                       - 股权结构                            │
│  - 职业信息                       - UBO 信息                            │
│                                   - 授权代表证明                        │
│                                                                         │
│  限额: $5,000/笔, $20,000/月      限额: $100,000/笔, $500,000/月        │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           Level 3 - 增强验证                             │
├─────────────────────────────────────────────────────────────────────────┤
│  个人 (KYC):                      企业 (KYB):                           │
│  - 收入证明                       - 财务报表                            │
│  - 资金来源说明                   - 银行流水                            │
│  - 银行账单                       - 贸易背景资料                        │
│  - 视频面签                       - 尽职调查报告                        │
│                                   - 现场检查 (可选)                     │
│                                                                         │
│  限额: $50,000/笔, 无月限额       限额: 协议限额                        │
└─────────────────────────────────────────────────────────────────────────┘
```

### 5.4 全球牌照与合规要求

```
主要市场牌照要求:

┌─────────────────────────────────────────────────────────────────────────┐
│ 地区        │ 牌照类型              │ 监管机构       │ 主要要求         │
├─────────────────────────────────────────────────────────────────────────┤
│ 美国        │ MTL (各州)            │ 各州金融局     │ 保证金、审计     │
│             │ FinCEN MSB            │ FinCEN         │ AML/BSA 合规     │
│─────────────────────────────────────────────────────────────────────────│
│ 欧盟        │ EMI (电子货币)        │ 各国央行       │ 资本金 35万欧    │
│             │ PI (支付机构)         │               │ 资本金 12.5万欧  │
│             │ PSD2 合规             │ EBA            │ 开放银行         │
│─────────────────────────────────────────────────────────────────────────│
│ 英国        │ EMI/PI                │ FCA            │ 资本金、客户资金 │
│─────────────────────────────────────────────────────────────────────────│
│ 新加坡      │ MPI/SPI               │ MAS            │ 资本金、审计     │
│             │ (支付服务法)          │               │                  │
│─────────────────────────────────────────────────────────────────────────│
│ 香港        │ MSO (汇款)            │ 海关           │ AML 合规         │
│             │ SVF (储值工具)        │ 金管局         │ 资本金、托管     │
│─────────────────────────────────────────────────────────────────────────│
│ 中国大陆    │ 支付业务许可          │ 人民银行       │ 资本金、实缴     │
│             │ 跨境支付资质          │ 外管局         │ 合规报备         │
│─────────────────────────────────────────────────────────────────────────│
│ 日本        │ 资金移动业务          │ FSA            │ 资本金、保证金   │
│─────────────────────────────────────────────────────────────────────────│
│ 澳大利亚    │ AFSL                  │ ASIC           │ 资本金、审计     │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 6. 通道网络架构

### 6.1 通道类型矩阵

```
通道类型与适用场景:

┌─────────────────────────────────────────────────────────────────────────┐
│ 通道类型          │ 到账时效    │ 费用      │ 适用场景                 │
├─────────────────────────────────────────────────────────────────────────┤
│ SWIFT GPI         │ 24-48h     │ $20-50    │ 大额 B2B、全球覆盖        │
│                   │ (优先服务)  │ + 中转费  │                          │
├─────────────────────────────────────────────────────────────────────────┤
│ 本地清算网络:                                                           │
│ - ACH (美国)      │ T+1        │ $0.2-1    │ 美国境内付款              │
│ - SEPA (欧洲)     │ T+0/T+1    │ €0.2-0.5  │ 欧元区付款                │
│ - FPS (英国)      │ 实时       │ 免费-£0.5 │ 英国境内付款              │
│ - TIPS (欧洲)     │ 实时       │ €0.002    │ 欧元区实时                │
│ - FedNow (美国)   │ 实时       │ $0.01-0.5 │ 美国实时付款              │
│ - 人民币跨境      │ T+0        │ 协议      │ 人民币结算                │
├─────────────────────────────────────────────────────────────────────────┤
│ 银行直连          │ T+0/T+1    │ 协议      │ 大客户、VIP               │
├─────────────────────────────────────────────────────────────────────────┤
│ 卡组织:                                                                 │
│ - Visa Direct     │ 实时-30min │ 0.5-1%    │ 卡到卡转账                │
│ - Mastercard Send │ 实时-30min │ 0.5-1%    │ 卡到卡转账                │
├─────────────────────────────────────────────────────────────────────────┤
│ 数字钱包:                                                               │
│ - PayPal          │ 实时       │ 2-5%      │ 个人汇款                  │
│ - Alipay          │ 实时       │ 1-3%      │ 中国相关                  │
│ - GrabPay         │ 实时       │ 1-2%      │ 东南亚                    │
├─────────────────────────────────────────────────────────────────────────┤
│ 移动支付:                                                               │
│ - M-Pesa          │ 实时       │ 1-3%      │ 非洲汇款                  │
│ - UPI             │ 实时       │ 免费-0.3% │ 印度支付                  │
│ - PromptPay       │ 实时       │ 低        │ 泰国支付                  │
├─────────────────────────────────────────────────────────────────────────┤
│ 区块链网络:                                                             │
│ - Stablecoin      │ 分钟级     │ Gas费     │ 跨境结算                  │
│ - RippleNet       │ 秒级       │ 低        │ 银行间结算                │
└─────────────────────────────────────────────────────────────────────────┘
```

### 6.2 全球通道网络拓扑

```
全球通道网络:

                           ┌───────────────────┐
                           │    我们的平台      │
                           │   (核心路由)       │
                           └─────────┬─────────┘
                                     │
         ┌───────────────────────────┼───────────────────────────┐
         │                           │                           │
         ▼                           ▼                           ▼
┌─────────────────┐        ┌─────────────────┐        ┌─────────────────┐
│   北美区域       │        │   欧洲区域       │        │   亚太区域       │
├─────────────────┤        ├─────────────────┤        ├─────────────────┤
│                 │        │                 │        │                 │
│ ┌─────────────┐ │        │ ┌─────────────┐ │        │ ┌─────────────┐ │
│ │ 本地银行网络 │ │        │ │ 本地银行网络 │ │        │ │ 本地银行网络 │ │
│ │ - BOA       │ │        │ │ - Barclays  │ │        │ │ - DBS       │ │
│ │ - JPMorgan  │ │        │ │ - Deutsche  │ │        │ │ - HSBC      │ │
│ │ - Wells     │ │        │ │ - BNP       │ │        │ │ - OCBC      │ │
│ └─────────────┘ │        │ └─────────────┘ │        │ └─────────────┘ │
│                 │        │                 │        │                 │
│ ┌─────────────┐ │        │ ┌─────────────┐ │        │ ┌─────────────┐ │
│ │ 清算网络     │ │        │ │ 清算网络     │ │        │ │ 清算网络     │ │
│ │ - ACH       │ │        │ │ - SEPA      │ │        │ │ - PayNow    │ │
│ │ - Fedwire   │ │        │ │ - TIPS      │ │        │ │ - FPS(HK)   │ │
│ │ - RTP       │ │        │ │ - CHAPS     │ │        │ │ - UPI       │ │
│ │ - FedNow    │ │        │ │ - TARGET2   │ │        │ │ - CIPS      │ │
│ └─────────────┘ │        │ └─────────────┘ │        │ └─────────────┘ │
│                 │        │                 │        │                 │
└─────────────────┘        └─────────────────┘        └─────────────────┘
         │                           │                           │
         └───────────────────────────┼───────────────────────────┘
                                     │
                           ┌─────────▼─────────┐
                           │    SWIFT 网络     │
                           │   (全球后备)       │
                           └───────────────────┘
```

### 6.3 汇率与定价引擎

```rust
// FX 定价引擎
pub mod fx {
    use rust_decimal::Decimal;

    /// FX 报价引擎
    pub struct FxQuoteEngine {
        liquidity_providers: Vec<Box<dyn LiquidityProvider>>,
        pricing_rules: Vec<PricingRule>,
        cache: QuoteCache,
    }

    impl FxQuoteEngine {
        /// 获取报价
        pub async fn get_quote(&self, request: &QuoteRequest) -> Result<FxQuote, FxError> {
            // 1. 获取银行间汇率 (多源聚合)
            let interbank_rate = self.get_interbank_rate(
                &request.source_currency,
                &request.dest_currency,
            ).await?;

            // 2. 计算 Markup
            let markup = self.calculate_markup(request);

            // 3. 计算客户汇率
            let customer_rate = match request.direction {
                FxDirection::Buy => interbank_rate * (Decimal::ONE + markup),
                FxDirection::Sell => interbank_rate * (Decimal::ONE - markup),
            };

            // 4. 生成报价 (30秒有效期)
            let quote = FxQuote {
                quote_id: QuoteId::generate(),
                source_currency: request.source_currency.clone(),
                dest_currency: request.dest_currency.clone(),
                interbank_rate,
                customer_rate,
                markup,
                source_amount: request.source_amount,
                dest_amount: self.calculate_dest_amount(request, customer_rate),
                valid_until: Utc::now() + Duration::seconds(30),
                created_at: Utc::now(),
            };

            Ok(quote)
        }
    }

    /// 阶梯定价
    #[derive(Debug, Clone)]
    pub struct TierMarkup {
        pub min_amount: Decimal,
        pub max_amount: Option<Decimal>,
        pub markup: Decimal,
    }
}
```

---

## 7. 安全架构

### 7.1 安全防护体系

```
安全防护层次:

┌─────────────────────────────────────────────────────────────────────────┐
│                           边界安全层                                     │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ WAF      │  │ DDoS防护 │  │ CDN      │  │ 入侵检测 │               │
│  │ (OWASP)  │  │          │  │ (边缘)   │  │ IDS/IPS  │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           接入安全层                                     │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ API 认证 │  │ 速率限制 │  │ IP 白名单│  │ 设备绑定 │               │
│  │ (OAuth)  │  │          │  │          │  │          │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           应用安全层                                     │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ 输入验证 │  │ 授权控制 │  │ 会话管理 │  │ 安全审计 │               │
│  │          │  │ (RBAC)   │  │          │  │          │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
└─────────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           数据安全层                                     │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │ 传输加密 │  │ 存储加密 │  │ 密钥管理 │  │ 数据脱敏 │               │
│  │ (TLS1.3) │  │ (AES256) │  │ (HSM)    │  │          │               │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘               │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.2 密钥管理架构

```
密钥管理体系:

┌─────────────────────────────────────────────────────────────────────────┐
│                           密钥层次结构                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  Level 0: 根密钥 (Master Key)                                           │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    HSM (硬件安全模块)                            │   │
│  │  ┌──────────────────────────────────────────────────────────┐   │   │
│  │  │                   Master Key (KEK)                        │   │   │
│  │  │                   永不导出                                │   │   │
│  │  └──────────────────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                   │                                     │
│                                   ▼                                     │
│  Level 1: 区域密钥 (Zone Key)                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │ APAC Zone    │  │ EMEA Zone    │  │ AMER Zone    │                  │
│  │ Key          │  │ Key          │  │ Key          │                  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘                  │
│         │                 │                 │                          │
│         ▼                 ▼                 ▼                          │
│  Level 2: 应用密钥 (Application Key)                                    │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐     │
│  │ 支付签名 │ │ 数据加密 │ │ 通道密钥 │ │ 会话密钥 │ │ API密钥  │     │
│  │ Key      │ │ Key      │ │ Key      │ │ Key      │ │ Key      │     │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘     │
│                                                                         │
│  密钥轮换策略:                                                           │
│  - Master Key: 永不轮换 (需要仪式)                                       │
│  - Zone Key: 年度轮换                                                    │
│  - Application Key: 季度轮换                                             │
│  - Session Key: 每次会话                                                 │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 7.3 安全合规认证

| 认证类型 | 适用范围 | 优先级 | 计划时间 |
|----------|----------|--------|----------|
| PCI DSS Level 1 | 卡支付处理 | P0 | 第1年 |
| SOC 2 Type II | 全平台安全 | P0 | 第1年 |
| ISO 27001 | 信息安全管理 | P1 | 第1-2年 |
| ISO 22301 | 业务连续性 | P1 | 第2年 |
| GDPR 合规 | 欧洲数据保护 | P0 | 第1年 |
| CSA STAR | 云安全 | P2 | 第2年 |

---

## 8. 运维与监控

### 8.1 可观测性体系

```
可观测性三支柱:

┌─────────────────────────────────────────────────────────────────────────┐
│                              Metrics (指标)                              │
│  Prometheus + Grafana                                                   │
│  业务指标: 交易量、成功率、平均处理时间、通道成功率                        │
│  技术指标: QPS、延迟 (P50/P95/P99)、错误率、资源使用                      │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                              Logs (日志)                                 │
│  ELK Stack / Loki                                                       │
│  交易日志、审计日志、错误日志、访问日志                                   │
│  JSON 结构化、统一 trace_id、敏感信息脱敏                                │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                              Traces (链路)                               │
│  Jaeger / Tempo                                                         │
│  API GW → Order → Risk → FX → Router → Channel                          │
│  端到端追踪、性能分析                                                     │
└─────────────────────────────────────────────────────────────────────────┘
```

### 8.2 SLA 指标

| 指标 | 目标 | 说明 |
|------|------|------|
| **可用性** | 99.95% | 年度停机 < 4.38 小时 |
| **API P99 延迟** | < 200ms | 创建订单等核心操作 |
| **交易成功率** | > 99% | 排除用户取消和风控拦截 |
| **T+0 到账率** | > 95% | 通过本地清算通道 |
| **平均处理时间** | < 30min | 从下单到到账 |

### 8.3 容灾与高可用设计

```
RTO/RPO 目标:

┌─────────────────────────────────────────────────────────────────────────┐
│ 系统层级              │ RTO         │ RPO        │ 容灾策略              │
├─────────────────────────────────────────────────────────────────────────┤
│ 核心交易系统          │ < 1 分钟    │ 0 (零丢失) │ 同城双活 + 异地热备    │
│ 账户余额系统          │ < 1 分钟    │ 0 (零丢失) │ 同步复制、强一致性      │
│ 风控合规系统          │ < 5 分钟    │ < 1 分钟   │ 主备切换               │
│ 通道路由系统          │ < 30 秒     │ < 1 分钟   │ 多活、自动降级          │
│ 报表分析系统          │ < 1 小时    │ < 5 分钟   │ 异步复制               │
│ 运营管理系统          │ < 30 分钟   │ < 10 分钟  │ 异步复制               │
└─────────────────────────────────────────────────────────────────────────┘

容灾架构:

                    ┌─────────────────────┐
                    │   Global DNS (GTM)  │
                    │   健康检查 + 权重    │
                    └──────────┬──────────┘
                               │
             ┌─────────────────┼─────────────────┐
             │                 │                 │
             ▼                 ▼                 ▼
    ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
    │ 主数据中心 (A)   │ │ 同城容灾 (B)     │ │ 异地容灾 (C)    │
    │ 新加坡 - 1       │ │ 新加坡 - 2       │ │ 香港             │
    │                 │ │                 │ │                 │
    │ 活跃状态: Active │ │ 活跃状态: Active │ │ 活跃状态: Standby│
    │ 写入: 是        │ │ 写入: 是 (同步)  │ │ 写入: 否 (异步)  │
    │ 读取: 是        │ │ 读取: 是        │ │ 读取: 只读       │
    │                 │ │                 │ │                 │
    │ ┌─────────────┐ │ │ ┌─────────────┐ │ │ ┌─────────────┐ │
    │ │ PostgreSQL  │◄├─┼─│ PostgreSQL  │ │ │ │ PostgreSQL  │ │
    │ │ (Primary)   │ │ │ │ (Sync Rep)  │ │ │ │ (Async Rep) │ │
    │ └─────────────┘ │ │ └─────────────┘ │ │ └─────────────┘ │
    │ ┌─────────────┐ │ │ ┌─────────────┐ │ │ ┌─────────────┐ │
    │ │ Redis       │◄├─┼─│ Redis       │ │ │ │ Redis       │ │
    │ │ (Master)    │ │ │ │ (Slave)     │ │ │ │ (Slave)     │ │
    │ └─────────────┘ │ │ └─────────────┘ │ │ └─────────────┘ │
    └─────────────────┘ └─────────────────┘ └─────────────────┘

故障切换流程:
┌────────────────────────────────────────────────────────────────────────┐
│ 时间      │ 动作                              │ 责任方                 │
├────────────────────────────────────────────────────────────────────────┤
│ T+0s     │ 检测到故障 (健康检查连续3次失败)     │ 监控系统自动           │
│ T+5s     │ 触发告警 (PagerDuty)               │ 值班 SRE               │
│ T+10s    │ DNS 权重调整 (流量切走)             │ GTM 自动               │
│ T+15s    │ 确认切换完成                       │ 值班 SRE               │
│ T+30s    │ 新主库提升 (如需要)                 │ DBA                    │
│ T+5min   │ 故障根因分析启动                   │ 故障响应团队            │
│ T+15min  │ 初步报告                          │ 值班 SRE               │
│ T+4h     │ 详细故障报告                       │ 技术负责人             │
└────────────────────────────────────────────────────────────────────────┘
```

### 8.4 通道容错与熔断机制

```rust
// 通道熔断器实现
pub mod circuit_breaker {
    use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum CircuitState {
        Closed,      // 正常状态
        Open,        // 熔断状态 (拒绝请求)
        HalfOpen,    // 半开状态 (探测恢复)
    }

    pub struct CircuitBreaker {
        state: AtomicUsize,
        failure_count: AtomicU64,
        success_count: AtomicU64,
        last_failure_time: AtomicU64,
        config: CircuitBreakerConfig,
    }

    #[derive(Debug, Clone)]
    pub struct CircuitBreakerConfig {
        pub failure_threshold: u64,       // 失败阈值
        pub success_threshold: u64,       // 半开成功阈值
        pub timeout: Duration,            // 熔断超时
        pub half_open_requests: u64,      // 半开允许请求数
    }

    impl CircuitBreaker {
        pub fn can_execute(&self) -> bool {
            match self.get_state() {
                CircuitState::Closed => true,
                CircuitState::Open => {
                    // 检查是否应该转为半开
                    if self.should_try_reset() {
                        self.transition_to_half_open();
                        true
                    } else {
                        false
                    }
                }
                CircuitState::HalfOpen => {
                    // 限制半开状态的请求数
                    self.success_count.load(Ordering::Relaxed)
                        < self.config.half_open_requests
                }
            }
        }

        pub fn record_success(&self) {
            match self.get_state() {
                CircuitState::HalfOpen => {
                    let count = self.success_count.fetch_add(1, Ordering::Relaxed) + 1;
                    if count >= self.config.success_threshold {
                        self.transition_to_closed();
                    }
                }
                CircuitState::Closed => {
                    self.failure_count.store(0, Ordering::Relaxed);
                }
                _ => {}
            }
        }

        pub fn record_failure(&self) {
            self.last_failure_time.store(
                Instant::now().elapsed().as_secs(),
                Ordering::Relaxed
            );

            match self.get_state() {
                CircuitState::Closed => {
                    let count = self.failure_count.fetch_add(1, Ordering::Relaxed) + 1;
                    if count >= self.config.failure_threshold {
                        self.transition_to_open();
                    }
                }
                CircuitState::HalfOpen => {
                    // 半开状态任何失败直接熔断
                    self.transition_to_open();
                }
                _ => {}
            }
        }
    }
}

// 通道降级策略
pub struct ChannelDegradationStrategy {
    primary_channels: Vec<ChannelId>,
    fallback_channels: Vec<ChannelId>,
    degradation_rules: Vec<DegradationRule>,
}

impl ChannelDegradationStrategy {
    /// 获取可用通道 (按优先级)
    pub fn get_available_channels(&self, request: &RouteRequest) -> Vec<ChannelId> {
        let mut available = Vec::new();

        // 1. 尝试主通道
        for channel_id in &self.primary_channels {
            if self.is_channel_available(channel_id, request) {
                available.push(channel_id.clone());
            }
        }

        // 2. 如果主通道不可用，启用降级
        if available.is_empty() {
            for channel_id in &self.fallback_channels {
                if self.is_channel_available(channel_id, request) {
                    available.push(channel_id.clone());
                }
            }
            // 记录降级事件
            self.emit_degradation_event();
        }

        available
    }
}
```

### 8.5 API 设计规范

```yaml
# API 设计规范

## 版本控制
版本策略: URL Path 版本
格式: /api/v{major}/resource
示例: /api/v1/orders, /api/v2/orders

## 幂等性设计
幂等性头: X-Idempotency-Key
有效期: 24小时
存储: Redis (分布式锁)

幂等请求示例:
POST /api/v1/orders
Headers:
  X-Idempotency-Key: "uuid-xxxx-xxxx"
  Content-Type: application/json

## 标准响应格式
成功响应:
{
  "code": 0,
  "message": "success",
  "data": { ... },
  "trace_id": "abc123",
  "timestamp": "2025-12-03T10:00:00Z"
}

错误响应:
{
  "code": 40001,
  "message": "Invalid amount",
  "error_type": "VALIDATION_ERROR",
  "details": [
    {"field": "amount", "reason": "must be positive"}
  ],
  "trace_id": "abc123",
  "timestamp": "2025-12-03T10:00:00Z"
}

## 错误码规范
1xxxx: 认证授权错误 (10001: Token过期, 10002: 权限不足)
2xxxx: 参数验证错误 (20001: 参数缺失, 20002: 格式错误)
3xxxx: 业务逻辑错误 (30001: 余额不足, 30002: 通道不可用)
4xxxx: 外部服务错误 (40001: 银行接口超时)
5xxxx: 系统内部错误 (50001: 数据库错误)

## 限流策略
| 接口类型 | 限流策略 | 限制 |
|----------|----------|------|
| 公开接口 | IP限流 | 100次/分钟 |
| 商户接口 | 商户ID限流 | 1000次/分钟 |
| 核心交易 | 令牌桶 | 500 TPS/商户 |
| 批量接口 | 滑动窗口 | 10次/分钟 |
```

---

## 9. 换汇头寸与风险管理

### 9.1 FX 头寸管理

```rust
// FX 头寸管理
pub mod fx_position {
    use rust_decimal::Decimal;
    use std::collections::HashMap;

    /// 币种头寸
    #[derive(Debug, Clone)]
    pub struct CurrencyPosition {
        pub currency: CurrencyCode,
        pub long_position: Decimal,    // 多头头寸
        pub short_position: Decimal,   // 空头头寸
        pub net_position: Decimal,     // 净头寸
        pub limit: PositionLimit,      // 头寸限额
        pub updated_at: DateTime<Utc>,
    }

    /// 头寸限额
    #[derive(Debug, Clone)]
    pub struct PositionLimit {
        pub max_net_long: Decimal,     // 最大净多头
        pub max_net_short: Decimal,    // 最大净空头
        pub max_gross: Decimal,        // 最大总头寸
        pub warning_threshold: f64,    // 预警阈值 (%)
    }

    impl CurrencyPosition {
        /// 检查是否需要对冲
        pub fn needs_hedging(&self) -> bool {
            let utilization = self.position_utilization();
            utilization > self.limit.warning_threshold
        }

        /// 计算头寸使用率
        pub fn position_utilization(&self) -> f64 {
            let max_limit = if self.net_position > Decimal::ZERO {
                self.limit.max_net_long
            } else {
                self.limit.max_net_short
            };

            (self.net_position.abs() / max_limit).to_f64().unwrap_or(0.0)
        }
    }

    /// 头寸管理器
    pub struct PositionManager {
        positions: HashMap<CurrencyCode, CurrencyPosition>,
        hedging_engine: Box<dyn HedgingEngine>,
        alert_manager: AlertManager,
    }

    impl PositionManager {
        /// 更新头寸 (交易后)
        pub async fn update_position(
            &mut self,
            currency: &CurrencyCode,
            amount: Decimal,
            direction: PositionDirection,
        ) -> Result<(), PositionError> {
            let position = self.positions.get_mut(currency)
                .ok_or(PositionError::CurrencyNotFound)?;

            // 更新头寸
            match direction {
                PositionDirection::Long => {
                    position.long_position += amount;
                    position.net_position += amount;
                }
                PositionDirection::Short => {
                    position.short_position += amount;
                    position.net_position -= amount;
                }
            }

            // 检查限额
            if position.needs_hedging() {
                // 触发自动对冲
                self.trigger_auto_hedge(currency, position).await?;
            }

            // 检查是否超限
            if self.is_limit_breached(position) {
                self.alert_manager.send_critical_alert(
                    &format!("FX Position limit breached for {}", currency)
                ).await;
                return Err(PositionError::LimitBreached);
            }

            Ok(())
        }

        /// 触发自动对冲
        async fn trigger_auto_hedge(
            &self,
            currency: &CurrencyCode,
            position: &CurrencyPosition,
        ) -> Result<(), PositionError> {
            // 计算对冲金额
            let hedge_amount = self.calculate_hedge_amount(position);

            // 执行对冲
            self.hedging_engine.execute_hedge(
                currency,
                hedge_amount,
                HedgeType::Auto,
            ).await?;

            Ok(())
        }
    }
}
```

### 9.2 FX 风险限额矩阵

```
FX 风险限额:

┌─────────────────────────────────────────────────────────────────────────┐
│ 币种对        │ 单日限额      │ 单笔限额     │ 净头寸限额    │ 预警阈值   │
├─────────────────────────────────────────────────────────────────────────┤
│ USD/SGD      │ $50M         │ $5M         │ $10M        │ 70%       │
│ USD/HKD      │ $50M         │ $5M         │ $10M        │ 70%       │
│ USD/CNH      │ $30M         │ $3M         │ $5M         │ 60%       │
│ EUR/USD      │ $40M         │ $4M         │ $8M         │ 70%       │
│ GBP/USD      │ $30M         │ $3M         │ $6M         │ 70%       │
│ USD/JPY      │ $40M         │ $4M         │ $8M         │ 70%       │
│ 新兴市场货币    │ $10M         │ $1M         │ $2M         │ 50%       │
└─────────────────────────────────────────────────────────────────────────┘

VaR (风险价值) 限额:
- 1日 VaR (99%): < $500K
- 10日 VaR (99%): < $1.5M
- 压力测试损失: < $3M
```

---

## 10. 成本核算模型

### 10.1 成本结构

```
成本结构分解:

┌─────────────────────────────────────────────────────────────────────────┐
│                           交易成本结构                                    │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    通道成本 (变动成本)                           │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 通道费用 │ │ 换汇成本 │ │ 中转费用 │ │ 退款成本 │          │   │
│  │  │ 0.1-1%  │ │ 5-50bps │ │ $10-30  │ │ 0.2%    │          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    运营成本 (半固定成本)                          │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 合规成本 │ │ 风控成本 │ │ 客服成本 │ │ 对账成本 │          │   │
│  │  │ $0.5/笔 │ │ $0.2/笔 │ │ $0.1/笔 │ │ $0.05/笔│          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    基础设施成本 (固定成本)                        │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐          │   │
│  │  │ 云服务  │ │ 牌照费用 │ │ 安全合规 │ │ 人力成本 │          │   │
│  │  │ $50K/月 │ │ $20K/月 │ │ $10K/月 │ │ $200K/月│          │   │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### 10.2 定价模型

```rust
// 定价引擎
pub mod pricing {
    use rust_decimal::Decimal;

    /// 定价引擎
    pub struct PricingEngine {
        cost_calculator: CostCalculator,
        margin_rules: Vec<MarginRule>,
        tier_config: TierConfig,
    }

    impl PricingEngine {
        /// 计算最终报价
        pub fn calculate_price(&self, request: &PriceRequest) -> PriceQuote {
            // 1. 计算基础成本
            let base_cost = self.cost_calculator.calculate(request);

            // 2. 计算目标利润率
            let target_margin = self.get_target_margin(request);

            // 3. 应用阶梯定价
            let tier_adjustment = self.apply_tier_pricing(request);

            // 4. 计算最终价格
            let final_price = base_cost * (Decimal::ONE + target_margin + tier_adjustment);

            PriceQuote {
                base_cost,
                margin: target_margin,
                tier_adjustment,
                final_fee: final_price,
                fx_markup: self.calculate_fx_markup(request),
            }
        }
    }

    /// 阶梯定价配置
    #[derive(Debug, Clone)]
    pub struct TierConfig {
        pub tiers: Vec<PricingTier>,
    }

    #[derive(Debug, Clone)]
    pub struct PricingTier {
        pub min_volume: Decimal,       // 月交易量下限
        pub max_volume: Option<Decimal>, // 月交易量上限
        pub fee_rate: Decimal,          // 费率
        pub fx_markup: Decimal,         // 汇率加点
    }

    // 阶梯定价示例:
    // Tier 1: $0 - $100K/月,     费率 1.0%, 加点 50bps
    // Tier 2: $100K - $1M/月,    费率 0.8%, 加点 40bps
    // Tier 3: $1M - $10M/月,     费率 0.5%, 加点 25bps
    // Tier 4: $10M+/月,          费率 0.3%, 加点 15bps
}
```

### 10.3 盈利模型分析

```
单位经济模型 (Unit Economics):

┌─────────────────────────────────────────────────────────────────────────┐
│ 假设条件:                                                                │
│ - 平均交易金额: $5,000                                                   │
│ - 平均费率: 0.8%                                                         │
│ - 平均换汇加点: 30bps                                                    │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│ 收入端:                                                                  │
│ ┌────────────────────────────────────────────────────────┐              │
│ │ 交易手续费        $5,000 × 0.8%  = $40.00              │              │
│ │ 换汇收益         $5,000 × 0.30% = $15.00              │              │
│ │ 单笔总收入                       = $55.00              │              │
│ └────────────────────────────────────────────────────────┘              │
│                                                                         │
│ 成本端:                                                                  │
│ ┌────────────────────────────────────────────────────────┐              │
│ │ 通道成本 (0.3%)   $5,000 × 0.3%  = $15.00              │              │
│ │ 换汇成本 (10bps)  $5,000 × 0.10% = $5.00               │              │
│ │ 运营成本 (固定)                   = $0.85               │              │
│ │ 单笔总成本                       = $20.85              │              │
│ └────────────────────────────────────────────────────────┘              │
│                                                                         │
│ 盈利:                                                                    │
│ ┌────────────────────────────────────────────────────────┐              │
│ │ 单笔毛利         $55.00 - $20.85 = $34.15              │              │
│ │ 毛利率           $34.15 / $55.00 = 62%                 │              │
│ │ 盈亏平衡交易量   $280K固定成本 / $34.15 = 8,200笔/月   │              │
│ └────────────────────────────────────────────────────────┘              │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘

规模效应分析:
┌────────────────────────────────────────────────────────────────────────┐
│ 月交易量      │ 月收入      │ 变动成本    │ 固定成本    │ 净利润      │
├────────────────────────────────────────────────────────────────────────┤
│ $10M (2K笔)  │ $110K      │ $42K       │ $280K      │ -$212K     │
│ $50M (10K笔) │ $550K      │ $209K      │ $280K      │ +$61K      │
│ $100M(20K笔) │ $1.1M      │ $417K      │ $300K      │ +$383K     │
│ $500M(100K笔)│ $4.4M      │ $1.7M      │ $400K      │ +$2.3M     │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 11. 实施路线图

```
实施路线图:

Phase 1: MVP (Month 1-6)
┌─────────────────────────────────────────────────────────────────────────┐
│ 目标: 单一市场上线 (新加坡)                                              │
├─────────────────────────────────────────────────────────────────────────┤
│ M1-M2: 核心系统 (订单、账户、用户)                                       │
│ M3-M4: 风控合规 (KYC/KYB、AML、制裁筛查)                                 │
│ M5: 通道接入 (本地银行、PayNow、SWIFT)                                   │
│ M6: 试运营 (Beta 测试、种子客户)                                         │
│                                                                         │
│ 里程碑: 获得 MAS 原则性批准, 完成首笔交易                                 │
└─────────────────────────────────────────────────────────────────────────┘

Phase 2: 区域扩展 (Month 7-12)
┌─────────────────────────────────────────────────────────────────────────┐
│ 目标: 亚太区域扩展                                                       │
├─────────────────────────────────────────────────────────────────────────┤
│ M7-M8: 香港市场 (MSO申请、FPS接入)                                       │
│ M9-M10: 大陆通道 (人民币跨境、微信/支付宝)                                │
│ M11: 日韩通道                                                            │
│ M12: 产品完善 (B2B产品、API完善)                                         │
│                                                                         │
│ 里程碑: 覆盖 10+ 亚太国家, 月交易量 $10M+                                 │
└─────────────────────────────────────────────────────────────────────────┘

Phase 3: 全球化 (Year 2)
┌─────────────────────────────────────────────────────────────────────────┐
│ 目标: 欧美市场扩展                                                       │
├─────────────────────────────────────────────────────────────────────────┤
│ Q1: 英国/欧洲 (EMI牌照、SEPA、FPS UK)                                    │
│ Q2: 美国 (MTL分州申请、ACH/Fedwire)                                      │
│ Q3-Q4: 全球扩展 (新兴市场、平台化)                                       │
│                                                                         │
│ 里程碑: 覆盖 50+ 国家, 月交易量 $100M+                                    │
└─────────────────────────────────────────────────────────────────────────┘
```

### 11.4 灾备演练计划

```
灾备演练年度计划:

┌─────────────────────────────────────────────────────────────────────────┐
│ 演练类型          │ 频率      │ 范围              │ 目标                 │
├─────────────────────────────────────────────────────────────────────────┤
│ 桌面演练          │ 月度      │ 核心团队          │ 流程熟悉、角色确认     │
│ 组件故障演练      │ 季度      │ 单组件            │ 自动恢复验证          │
│ 区域切换演练      │ 半年度    │ 单区域            │ RTO 验证 < 1min       │
│ 全站容灾演练      │ 年度      │ 全站              │ 业务连续性验证         │
│ 红蓝对抗          │ 年度      │ 安全团队          │ 安全响应验证          │
└─────────────────────────────────────────────────────────────────────────┘

演练检查清单:
□ 演练通知发送 (提前7天)
□ 值班人员确认
□ 监控告警静默配置
□ 客户通知 (如需要)
□ 演练脚本准备
□ 回滚方案准备
□ 演练执行
□ 恢复确认
□ 演练报告
□ 改进项跟踪
```

---

## 12. 附录

### 12.1 术语表

| 术语 | 全称 | 说明 |
|------|------|------|
| AML | Anti-Money Laundering | 反洗钱 |
| CFT | Countering the Financing of Terrorism | 反恐融资 |
| KYC | Know Your Customer | 了解你的客户 |
| KYB | Know Your Business | 了解你的企业客户 |
| PEP | Politically Exposed Person | 政治公众人物 |
| UBO | Ultimate Beneficial Owner | 最终受益人 |
| STR | Suspicious Transaction Report | 可疑交易报告 |
| MTL | Money Transmitter License | 货币转移牌照 (美国) |
| EMI | Electronic Money Institution | 电子货币机构 (欧盟) |
| MPI | Major Payment Institution | 大型支付机构 (新加坡) |
| SWIFT | Society for Worldwide Interbank Financial Telecommunication | 环球银行金融电信协会 |
| GPI | Global Payments Innovation | SWIFT 全球支付创新 |
| ACH | Automated Clearing House | 自动清算所 (美国) |
| SEPA | Single Euro Payments Area | 单一欧元支付区 |
| FPS | Faster Payments Service | 快速支付服务 |
| FX | Foreign Exchange | 外汇 |
| T+0/T+1 | Trade Date + 0/1 Day | 交易日 + 0/1 天结算 |

### 12.2 参考资源

**监管指南**:
- [MAS Payment Services Act](https://www.mas.gov.sg/regulation/acts/payment-services-act)
- [FCA Payment Services Regulations](https://www.fca.org.uk/firms/payment-services-regulations)
- [FinCEN MSB Requirements](https://www.fincen.gov/money-services-business-msb-registration)
- [PSD2 Directive](https://ec.europa.eu/info/law/payment-services-psd-2-directive-eu-2015-2366_en)

**技术标准**:
- [ISO 20022 Financial Messaging](https://www.iso20022.org/)
- [OpenAPI Banking Standards](https://www.openbanking.org.uk/standards/)
- [PCI DSS Standards](https://www.pcisecuritystandards.org/)

**行业参考**:
- [SWIFT gpi](https://www.swift.com/our-solutions/swift-gpi)
- [Wise Architecture](https://wise.com/)
- [Stripe Atlas](https://stripe.com/atlas)

---

**文档版本**: v2.0
**创建日期**: 2025-12-03
**最后更新**: 2025-12-03
**作者**: 首席架构师
**审阅状态**: 已完成首轮Review

**变更记录**:
- v2.0: 增加容灾高可用设计、通道熔断机制、API规范、FX头寸管理、成本模型、灾备演练
- v1.0: 初始版本

---

*本文档为公司内部机密文件，未经授权不得对外披露。*
