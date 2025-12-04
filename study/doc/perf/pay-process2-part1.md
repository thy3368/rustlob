# 跨境支付五级流程模型（上篇：概述与全景图）

> **文档类型**: 业务流程架构
> **版本**: v4.6
> **创建日期**: 2025-12-03
> **最后更新**: 2025-12-03
> **建模标准**: 银行业五级流程模型 (APQC/eTOM) + CQRS Command模式
> **本篇内容**: 五级模型概述 + L1-L3 流程全景图
> **下篇链接**: [L4任务与Command映射](pay-process2-part2.md)

---

## 1. 五级流程模型概述

```
银行业五级流程模型层次:

┌─────────────────────────────────────────────────────────────────────────────┐
│ Level 1: 价值链 (Value Chain)                                               │
│ 定义: 企业核心价值活动领域                                                    │
│ 示例: 跨境支付服务                                                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ Level 2: 流程域 (Process Area)                                              │
│ 定义: 价值链下的主要业务流程领域                                              │
│ 示例: 汇出汇款、汇入汇款、换汇、清结算                                        │
├─────────────────────────────────────────────────────────────────────────────┤
│ Level 3: 流程组 (Process Group)                                             │
│ 定义: 流程域内的端到端业务流程                                                │
│ 示例: B2B付款流程、C2C汇款流程                                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ Level 4: 业务任务 (Task)                                                    │
│ 定义: 可独立执行的原子业务活动                                                │
│ 示例: 创建订单、风控审核、换汇执行                                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ Level 5: 步骤与规则 (Step & Rule)                                           │
│ 定义: 任务执行的具体步骤和业务规则                                            │
│ 示例: 校验金额 > 0、查询实时汇率、计算手续费                                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. 跨境支付流程全景图

### 2.1 L1 价值链

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        L1: 跨境支付服务价值链                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         核心业务流程                                  │   │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐           │   │
│  │  │ 汇出汇款  │→│ 换汇处理  │→│ 通道路由  │→│ 清算结算  │           │   │
│  │  │ Outward   │ │ FX        │ │ Routing   │ │ Settlement│           │   │
│  │  └───────────┘ └───────────┘ └───────────┘ └───────────┘           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                         支撑业务流程                                  │   │
│  │  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────┐           │   │
│  │  │ 客户管理  │ │ 风险管控  │ │ 合规管理  │ │ 账户管理  │           │   │
│  │  │ Customer  │ │ Risk      │ │ Compliance│ │ Account   │           │   │
│  │  └───────────┘ └───────────┘ └───────────┘ └───────────┘           │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 L2 流程域

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                    L2: 流程域分解                                                │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                  核心业务流程域                                           │   │
│  ├─────────────────────────────────────────────────────────────────────────────────────────┤   │
│  │                                                                                         │   │
│  │  L2.1 汇出汇款域          L2.2 汇入汇款域          L2.3 换汇域          L2.4 清结算域   │   │
│  │  ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐   ┌─────────────┐ │   │
│  │  │ • B2B 贸易付款  │     │ • 收款入账      │     │ • 实时换汇      │   │ • 通道清算  │ │   │
│  │  │ • B2C 个人汇款  │     │ • 收款通知      │     │ • 远期换汇      │   │ • 内部结算  │ │   │
│  │  │ • C2C 转账      │     │ • 退汇处理      │     │ • 批量换汇      │   │ • 对账核销  │ │   │
│  │  │ • 批量付款      │     │ • 追索处理      │     │ • 头寸管理      │   │ • 差错处理  │ │   │
│  │  │ • 定期付款      │     │ • 代收代付      │     │ • 汇率管理      │   │ • 资金归集  │ │   │
│  │  └─────────────────┘     └─────────────────┘     └─────────────────┘   └─────────────┘ │   │
│  │                                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                  风控合规流程域                                           │   │
│  ├─────────────────────────────────────────────────────────────────────────────────────────┤   │
│  │                                                                                         │   │
│  │  L2.5 风控合规域          L2.6 争议处理域                                               │   │
│  │  ┌─────────────────┐     ┌─────────────────┐                                           │   │
│  │  │ • 交易风控      │     │ • 拒付处理      │                                           │   │
│  │  │ • AML/CFT       │     │ • 争议仲裁      │                                           │   │
│  │  │ • 制裁筛查      │     │ • 赔付管理      │                                           │   │
│  │  │ • 合规报送      │     │ • 举证调单      │                                           │   │
│  │  │ • 黑名单管理    │     │ • 申诉处理      │                                           │   │
│  │  └─────────────────┘     └─────────────────┘                                           │   │
│  │                                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                  客户商户流程域                                           │   │
│  ├─────────────────────────────────────────────────────────────────────────────────────────┤   │
│  │                                                                                         │   │
│  │  L2.7 商户管理域          L2.8 账户管理域                                               │   │
│  │  ┌─────────────────┐     ┌─────────────────┐                                           │   │
│  │  │ • 商户入驻      │     │ • 客户注册      │                                           │   │
│  │  │ • 合同管理      │     │ • KYC/KYB       │                                           │   │
│  │  │ • 商户分级      │     │ • 账户开立      │                                           │   │
│  │  │ • API接入       │     │ • 钱包管理      │                                           │   │
│  │  │ • 商户结算      │     │ • 余额管理      │                                           │   │
│  │  └─────────────────┘     └─────────────────┘                                           │   │
│  │                                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
│  ┌─────────────────────────────────────────────────────────────────────────────────────────┐   │
│  │                                  运营支撑流程域                                           │   │
│  ├─────────────────────────────────────────────────────────────────────────────────────────┤   │
│  │                                                                                         │   │
│  │  L2.9 通道管理域          L2.10 费用管理域        L2.11 资金管理域       L2.12 报表域   │   │
│  │  ┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐   ┌─────────────┐ │   │
│  │  │ • 通道接入      │     │ • 费率配置      │     │ • 备付金监控    │   │ • 交易报表  │ │   │
│  │  │ • 通道监控      │     │ • 阶梯定价      │     │ • 资金调拨      │   │ • 财务报表  │ │   │
│  │  │ • 通道路由      │     │ • 分润结算      │     │ • 流动性预警    │   │ • 监管报表  │ │   │
│  │  │ • 限额管理      │     │ • 成本核算      │     │ • 头寸管理      │   │ • 经营分析  │ │   │
│  │  │ • 通道切换      │     │ • 账单生成      │     │ • 银行对接      │   │ • 实时大屏  │ │   │
│  │  └─────────────────┘     └─────────────────┘     └─────────────────┘   └─────────────┘ │   │
│  │                                                                                         │   │
│  └─────────────────────────────────────────────────────────────────────────────────────────┘   │
│                                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

**L2 流程域说明：**

| 分类 | L2 编号 | 流程域名称 | 核心职责 |
|------|---------|-----------|---------|
| 核心业务 | L2.1 | 汇出汇款域 | 跨境付款全流程，含B2B/B2C/C2C/批量 |
| 核心业务 | L2.2 | 汇入汇款域 | 跨境收款、入账、退汇、追索 |
| 核心业务 | L2.3 | 换汇域 | 汇率报价、换汇成交、头寸对冲 |
| 核心业务 | L2.4 | 清结算域 | 通道清算、对账、差错、资金归集 |
| 风控合规 | L2.5 | 风控合规域 | 交易风控、AML、制裁筛查、合规报送 |
| 风控合规 | L2.6 | 争议处理域 | 拒付、争议、赔付、举证 |
| 客户商户 | L2.7 | 商户管理域 | 商户入驻、合同、分级、API接入 |
| 客户商户 | L2.8 | 账户管理域 | 客户注册、KYC/KYB、账户钱包 |
| 运营支撑 | L2.9 | 通道管理域 | 通道接入、监控、路由、限额 |
| 运营支撑 | L2.10 | 费用管理域 | 费率、定价、分润、成本 |
| 运营支撑 | L2.11 | 资金管理域 | 备付金、流动性、资金调拨 |
| 运营支撑 | L2.12 | 报表分析域 | 交易/财务/监管报表、经营分析 |

### 2.3 L3 流程组

#### 2.3.1 L2.1 汇出汇款域 → L3 流程组

**重要说明**：
- **L3层定位**: 展示客户/运营可主动调用的API端点,每个Command对应一次联机调用
- **系统自动化**: 风控/路由/出款等自动化处理是Command内部的L5步骤,不是独立触发点
- **Command vs Query**: Command改变状态(POST/PUT/DELETE),Query只读(GET)
- **事件发布**: Command执行成功后可发布领域事件,供后台任务订阅处理

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.1.1 B2B贸易付款流程(API调用视图)                                         │
│                                                                             │
│ 说明: L3展示客户/运营可主动调用的API,每个Command=一次API调用               │
│       系统内部自动化(风控/路由/出款)是Command内部的L5步骤                   │
│                                                                             │
│ [API 1] 客户调用 POST /api/orders                                           │
│                   ├→ CreatePaymentOrderCommand ✅                           │
│                   │  L5内部步骤: 验证→风控扫描→制裁筛查→路由→冻结资金       │
│                   │  返回: {order_id, status, quote_id}                     │
│                   │  状态: PENDING_PAYMENT / RISK_REJECTED                  │
│                                                                             │
│ [API 2] 运营调用 POST /api/orders/{id}/docs (可选,高风险订单)               │
│                   ├→ SupplementTradeDocsCommand ✅                          │
│                   │  L5内部步骤: 补充单证→更新风控状态                       │
│                   │  状态: DOCS_SUBMITTED                                   │
│                                                                             │
│ [API 3] 客户调用 GET /api/fx/quote                                          │
│                   ├→ RequestFxQuoteCommand ✅                               │
│                   │  L5内部步骤: 聚合报价→计算加点→生成报价                 │
│                   │  返回: {quote_id, rate, expiry}                         │
│                                                                             │
│ [API 4] 客户调用 POST /api/fx/lock                                          │
│                   ├→ LockFxRateCommand ✅                                   │
│                   │  L5内部步骤: 锁定报价30秒→预占头寸                       │
│                   │  返回: {lock_id, locked_rate, expires_at}               │
│                                                                             │
│ [API 5] 客户调用 POST /api/orders/{id}/confirm                              │
│                   ├→ ConfirmFxDealCommand ✅                                │
│                   │  L5内部步骤: 换汇成交→更新头寸→提交支付→通知收款人       │
│                   │  返回: {order_id, status, tracking_number}              │
│                   │  状态: PAYMENT_SUBMITTED                                │
│                                                                             │
│ [Query] 客户查询 GET /api/orders/{id}                                       │
│                   └→ QueryPaymentStatusQuery (非Command,只读查询)           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.2 B2C个人汇款流程(API调用视图)                                         │
│                                                                             │
│ [API 1] 客户调用 POST /api/personal-remittance                              │
│                   ├→ CreatePaymentOrderCommand ✅                           │
│                   │  L5内部步骤: 验证→KYC检查→AML规则检查→生成订单          │
│                   │  返回: {order_id, status, kyc_required}                 │
│                   │  状态: PENDING_VERIFICATION / READY_TO_PAY              │
│                                                                             │
│ [API 2] 客户调用 POST /api/contact/verify (如需身份验证)                    │
│                   ├→ VerifyContactCommand ✅                                │
│                   │  L5内部步骤: 发送验证码→验证→更新状态                   │
│                   │  返回: {verified: true}                                 │
│                                                                             │
│ [API 3] 客户调用 POST /api/orders/{id}/confirm-fx                           │
│                   ├→ ConfirmFxQuoteCommand ✅                               │
│                   │  L5内部步骤: 获取报价→风控扫描→换汇成交→提交支付→通知   │
│                   │  返回: {order_id, status, tracking_number}              │
│                   │  状态: PAYMENT_SUBMITTED                                │
│                                                                             │
│ [Query] 客户查询 GET /api/orders/{id}/status                                │
│                   └→ QueryPaymentStatusQuery                                │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.3 C2C转账流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/c2c-transfer                                     │
│                   ├→ CreateC2CTransferCommand ❌ **缺失**                   │
│                   │  L5内部步骤: 验证双方账户→AML检查→换汇→即时入账→双向通知│
│                   │  返回: {transfer_id, status, arrival_time}              │
│                   │  状态: TRANSFER_COMPLETED / AML_HOLD                    │
│                                                                             │
│ [Query] 客户查询 GET /api/transfers/{id}                                    │
│                   └→ QueryTransferStatusQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.4 批量付款流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/batch-payment                                    │
│                   ├→ UploadBatchFileCommand ✅                              │
│                   │  L5内部步骤: 解析文件→批量风控→批量换汇→批量路由→批量提交│
│                   │  返回: {batch_id, total_count, status}                  │
│                   │  状态: BATCH_PROCESSING / PARTIAL_SUCCESS / ALL_SUCCESS │
│                                                                             │
│ [Query] 客户查询 GET /api/batch-payment/{id}/report                         │
│                   └→ QueryBatchPaymentReportQuery                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.5 定期付款流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/recurring-payment                                │
│                   ├→ CreateRecurringPaymentCommand ❌ **缺失**              │
│                   │  L5内部步骤: 创建计划→设置调度→保存模板                 │
│                   │  返回: {plan_id, next_execution_time}                   │
│                   │  状态: PLAN_ACTIVE                                      │
│                                                                             │
│ [API 2] 客户调用 PUT /api/recurring-payment/{id}                            │
│                   ├→ UpdateRecurringPaymentCommand ❌ **缺失**              │
│                   │  L5内部步骤: 更新参数→重新调度                          │
│                                                                             │
│ [API 3] 客户调用 DELETE /api/recurring-payment/{id}                         │
│                   ├→ CancelRecurringPaymentCommand ❌ **缺失**              │
│                   │  L5内部步骤: 取消调度→更新状态                          │
│                                                                             │
│ 注: 定期触发执行由系统Cron Job完成,不是客户API调用                          │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.6 付款取消流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/orders/{id}/cancel                               │
│                   ├→ CancelPaymentCommand ✅                                │
│                   │  L5内部步骤: 状态校验→资金解冻→通道撤销→取消通知         │
│                   │  返回: {order_id, status, refund_amount}                │
│                   │  状态: CANCELLED / CANCEL_FAILED                        │
│                                                                             │
│ [Query] 客户查询 GET /api/orders/{id}                                       │
│                   └→ QueryPaymentStatusQuery                                │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.7 付款退款流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/orders/{id}/refund                               │
│                   ├→ RequestRefundCommand ✅                                │
│                   │  L5内部步骤: 原单核验→风控检查→生成退款单                │
│                   │  返回: {refund_id, status}                              │
│                   │  状态: REFUND_PENDING_APPROVAL                          │
│                                                                             │
│ [API 2] 运营调用 POST /api/refund/{id}/approve (大额退款需审批)             │
│                   ├→ ApproveRefundCommand ❌ **缺失**                       │
│                   │  L5内部步骤: 审批通过→资金退回→账务冲正→退款通知         │
│                   │  返回: {refund_id, status}                              │
│                   │  状态: REFUND_COMPLETED                                 │
│                                                                             │
│ [Query] 客户查询 GET /api/refund/{id}                                       │
│                   └→ QueryRefundStatusQuery                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.1.8 汇款入金流程(API调用视图) - 充值场景                                  │
│                                                                             │
│ [API 1] 客户调用 POST /api/deposit                                          │
│                   ├→ InitiateDepositCommand ✅                              │
│                   │  L5内部步骤: 验证→生成充值订单→返回支付链接              │
│                   │  返回: {deposit_id, payment_url, qr_code}               │
│                   │  状态: PENDING_PAYMENT                                  │
│                                                                             │
│ [API 2] 客户调用 POST /api/deposit/{id}/payment-method                      │
│                   ├→ SelectPaymentMethodCommand ✅                          │
│                   │  L5内部步骤: 调用收单通道→生成支付表单                   │
│                   │  返回: {payment_form, redirect_url}                     │
│                                                                             │
│ [Callback] 收单通道回调 POST /webhook/deposit-callback                      │
│                   ├→ CollectionSuccessCallbackCommand ✅                    │
│                   │  L5内部步骤: 验签→更新余额→入账→推送通知                 │
│                   │  状态: DEPOSIT_COMPLETED                                │
│                                                                             │
│ [Query] 客户查询 GET /api/deposit/{id}                                      │
│                   └→ QueryDepositStatusQuery                                │
└─────────────────────────────────────────────────────────────────────────────┘

图例说明:
  [API N] = 客户/运营主动调用的HTTP API端点,每个Command对应一次联机调用
  [Query] = 只读查询操作(GET请求),不改变系统状态
  [Callback] = 外部系统(通道/银行)异步回调
  L5内部步骤 = Command执行时的内部处理逻辑,包含风控/路由/通知等自动化
  ✅ = Command已在part2定义  ❌ = Command缺失需补充  ⚠️ = 需功能增强
```

#### 2.3.2 L2.2 汇入汇款域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.2.1 收款入账流程(API调用视图)                                             │
│                                                                             │
│ [Callback] 银行/SWIFT通知 POST /webhook/inbound-payment                     │
│                   ├→ InboundPaymentNotifyCommand ✅                         │
│                   │  L5内部步骤: 报文解析→制裁筛查→换汇→账户匹配→入账→通知   │
│                   │  返回: {payment_id, status, credited_amount}            │
│                   │  状态: CREDITED / SANCTION_HOLD / MATCHING_FAILED       │
│                                                                             │
│ [Query] 客户查询 GET /api/inbound-payments/{id}                             │
│                   └→ QueryInboundPaymentQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.2.2 收款通知流程(API调用视图)                                             │
│                                                                             │
│ 说明: 此流程由L3.2.1入账成功后自动触发,属于L5内部步骤,无独立API             │
│       通知操作已整合在InboundPaymentNotifyCommand的L5步骤中                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.2.3 退汇处理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/inbound-payments/{id}/return                     │
│                   ├→ RequestReturnCommand ❌ **缺失**                       │
│                   │  L5内部步骤: 原因分析→账务冲正→资金退回→通道通知         │
│                   │  返回: {return_id, status, return_amount}               │
│                   │  状态: RETURN_PROCESSING / RETURN_COMPLETED             │
│                                                                             │
│ [Callback] 银行退汇确认 POST /webhook/return-confirmed                      │
│                   ├→ BankReturnNotifyCommand ✅                             │
│                   │  L5内部步骤: 验签→更新状态→记录日志                      │
│                                                                             │
│ [Query] 运营查询 GET /api/returns/{id}                                      │
│                   └→ QueryReturnStatusQuery                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.2.4 追索处理流程(API调用视图) - SWIFT gpi追索                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/payments/{id}/recall                             │
│                   ├→ RequestRecallCommand ❌ **缺失**                       │
│                   │  L5内部步骤: 原单查询→证据收集→发起GPI追索→生成追索单    │
│                   │  返回: {recall_id, status, uetr}                        │
│                   │  状态: RECALL_INITIATED                                 │
│                                                                             │
│ [Callback] SWIFT追索结果 POST /webhook/recall-result                        │
│                   ├→ RecallRequestReceivedCommand ✅                        │
│                   │  L5内部步骤: 解析结果→账务调整→通知客户                  │
│                   │  状态: RECALL_SUCCESS / RECALL_REJECTED                 │
│                                                                             │
│ [API 2] 运营调用 POST /api/recalls/{id}/adjust                              │
│                   ├→ AdjustAccountCommand ✅                                │
│                   │  L5内部步骤: 账务调整→审批→执行→记录                     │
│                                                                             │
│ [Query] 客户查询 GET /api/recalls/{id}                                      │
│                   └→ QueryRecallStatusQuery                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.2.5 代收代付流程(API调用视图) - 商户代收场景                              │
│                                                                             │
│ [API 1] 商户调用 POST /api/collection-orders                                │
│                   ├→ CreateCollectionOrderCommand ❌ **缺失**               │
│                   │  L5内部步骤: 授权验证→风控审查→扣款执行→资金归集→代付分发│
│                   │  返回: {collection_id, status, disbursement_ids}        │
│                   │  状态: COLLECTION_COMPLETED / PARTIAL_SUCCESS           │
│                                                                             │
│ [Query] 商户查询 GET /api/collection-orders/{id}                            │
│                   └→ QueryCollectionStatusQuery                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.2.6 收款出金流程(API调用视图) - 提现场景                                  │
│                                                                             │
│ [API 1] 客户调用 POST /api/withdraw                                         │
│                   ├→ InitiateWithdrawCommand ✅                             │
│                   │  L5内部步骤: 风控校验→余额检查→生成提现单                │
│                   │  返回: {withdraw_id, status, amount}                    │
│                   │  状态: PENDING_APPROVAL / APPROVED                      │
│                                                                             │
│ [API 2] 客户调用 POST /api/withdraw/{id}/select-bank                        │
│                   ├→ SelectWithdrawBankCommand ✅                           │
│                   │  L5内部步骤: 选择银行卡→调用代付通道→扣减余额            │
│                   │  返回: {withdraw_id, status, tracking_number}           │
│                   │  状态: WITHDRAW_PROCESSING                              │
│                                                                             │
│ [Callback] 代付到账回调 POST /webhook/withdraw-callback                     │
│                   ├→ WithdrawArrivalCallbackCommand ✅                      │
│                   │  L5内部步骤: 验签→更新状态→通知客户                      │
│                   │  状态: WITHDRAW_COMPLETED                               │
│                                                                             │
│ [Query] 客户查询 GET /api/withdraw/{id}                                     │
│                   └→ QueryWithdrawStatusQuery                               │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.3 L2.3 换汇域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.3.1 实时换汇流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 GET /api/fx/quote                                          │
│                   ├→ RequestFxQuoteCommand ✅                               │
│                   │  L5内部步骤: 聚合做市商报价→计算加点→生成报价            │
│                   │  返回: {quote_id, rate, spread, expiry}                 │
│                   │  有效期: 30秒                                           │
│                                                                             │
│ [API 2] 客户调用 POST /api/fx/lock                                          │
│                   ├→ LockFxRateCommand ✅                                   │
│                   │  L5内部步骤: 锁定汇率→预占头寸→启动过期定时器            │
│                   │  返回: {lock_id, locked_rate, expires_at}               │
│                                                                             │
│ [API 3] 客户调用 POST /api/fx/confirm                                       │
│                   ├→ ConfirmFxDealCommand ✅                                │
│                   │  L5内部步骤: 换汇成交→更新头寸→记录交易                  │
│                   │  返回: {deal_id, executed_rate, amount}                 │
│                   │  状态: FX_COMPLETED                                     │
│                                                                             │
│ [Query] 客户查询 GET /api/fx/deals/{id}                                     │
│                   └→ QueryFxDealQuery                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.3.2 远期换汇流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 GET /api/fx/forward-quote                                  │
│                   ├→ RequestForwardFxQuoteCommand ❌ **缺失**               │
│                   │  L5内部步骤: 期限定价→掉期点计算→保证金计算→生成报价     │
│                   │  返回: {quote_id, forward_rate, margin_required}        │
│                                                                             │
│ [API 2] 客户调用 POST /api/fx/forward-contract                              │
│                   ├→ CreateForwardContractCommand ✅                        │
│                   │  L5内部步骤: 签订合约→冻结保证金→设置到期提醒            │
│                   │  返回: {contract_id, value_date, locked_rate}           │
│                   │  状态: CONTRACT_ACTIVE                                  │
│                                                                             │
│ [API 3] 客户调用 DELETE /api/fx/forward-contract/{id}                       │
│                   ├→ CancelForwardContractCommand ✅                        │
│                   │  L5内部步骤: 取消合约→释放保证金→记录                    │
│                   │  状态: CONTRACT_CANCELLED                               │
│                                                                             │
│ 注: 合约到期自动交割由系统Cron Job执行,调用SettleForwardContractCommand     │
│                                                                             │
│ [Query] 客户查询 GET /api/fx/forward-contracts/{id}                         │
│                   └→ QueryForwardContractQuery                              │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.3.3 批量换汇流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/fx/batch-request                                 │
│                   ├→ BatchFxRequestCommand ❌ **缺失**                      │
│                   │  L5内部步骤: 汇总金额→统一询价→拆单执行→批量成交→报告    │
│                   │  返回: {batch_id, total_deals, avg_rate}                │
│                   │  状态: BATCH_COMPLETED / PARTIAL_SUCCESS                │
│                                                                             │
│ [Query] 客户查询 GET /api/fx/batch/{id}/report                              │
│                   └→ QueryBatchFxReportQuery                                │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.3.4 头寸对冲流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/fx/hedge (手动触发对冲)                          │
│                   ├→ TriggerHedgeCommand ❌ **缺失**                        │
│                   │  L5内部步骤: 头寸检查→对冲决策→询价下单→执行成交         │
│                   │  返回: {hedge_id, hedge_amount, executed_rate}          │
│                   │  状态: HEDGE_COMPLETED                                  │
│                                                                             │
│ [API 2] 运营调用 PUT /api/fx/hedge-threshold                                │
│                   ├→ ConfigureHedgeThresholdCommand ❌ **缺失**             │
│                   │  L5内部步骤: 更新阈值配置→启用监控                       │
│                   │  返回: {threshold_config}                               │
│                                                                             │
│ 注: 自动对冲由系统监控任务触发,达到阈值自动调用AutoHedgePositionCommand     │
│                                                                             │
│ [Query] 运营查询 GET /api/fx/position                                       │
│                   └→ QueryFxPositionQuery                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.3.5 日终轧平流程(API调用视图)                                             │
│                                                                             │
│ 说明: 此流程由系统Cron Job在日终自动执行,无客户/运营API调用                 │
│       日终任务内部步骤: 汇总头寸→轧平决策→执行交易→计算盈亏→生成报表         │
│                                                                             │
│ [Query] 运营查询 GET /api/fx/eod-report                                     │
│                   └→ QueryEodFxReportQuery                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.3.6 汇率管理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/fx/rate-feed-subscription                        │
│                   ├→ SubscribeFxRateFeedCommand ❌ **缺失**                 │
│                   │  L5内部步骤: 配置汇率源→订阅推送→启用监控                │
│                   │  返回: {subscription_id, provider, pairs}               │
│                                                                             │
│ [API 2] 运营调用 POST /api/fx/manual-rate-adjustment                        │
│                   ├→ ManualAdjustFxRateCommand ✅                           │
│                   │  L5内部步骤: 手动调整→审批→生效→通知                     │
│                   │  返回: {adjustment_id, new_rate, effective_time}        │
│                                                                             │
│ [Callback] 汇率提供商推送 POST /webhook/fx-rate-feed                        │
│                   ├→ FxRateFeedCommand ✅                                   │
│                   │  L5内部步骤: 验签→异常检测→更新汇率→发送变动通知         │
│                   │  状态: RATE_UPDATED                                     │
│                                                                             │
│ [Query] 客户查询 GET /api/fx/rates                                          │
│                   └→ QueryFxRatesQuery                                      │
└─────────────────────────────────────────────────────────────────────────────┘
```
│ ┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐   ┌─────┐                           │
│ │汇率 │──→│异常 │──→│人工 │──→│汇率 │──→│生效 │                           │
│ │订阅 │   │监控 │   │干预 │   │更新 │   │通知 │                           │
│ └─────┘   └─────┘   └─────┘   └─────┘   └─────┘                           │
│ [O]❌     [S]❌     [O]✅     [S]✅     [S]❌                                 │
│ Subscribe MonitorF  ManualAd  UpdateFx  NotifyFx                            │
│ FxRateFe  xAnomaly  justFxRa  RateComm  RateChan                            │
│ edCmd     Cmd       teCmd     and       geCmd                               │
│ **缺失**  **缺失**            [E]✅:FxR **缺失**                             │
│                               ateFeedCm                                      │
│                               d                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.4 L2.4 清结算域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.4.1 日终清算流程(API调用视图)                                             │
│                                                                             │
│ 说明: 此流程主要由系统Cron Job在日切时自动执行,部分环节支持手动触发          │
│                                                                             │
│ [API 1] 运营调用 POST /api/settlement/daily/trigger (可选手动触发)          │
│                   ├→ TriggerDailySettlementCommand ❌ **缺失**              │
│                   │  L5内部步骤: 日切检查→交易汇总→轧差计算→生成清算文件      │
│                   │              →发送清算指令→等待确认→更新状态            │
│                   │  返回: {settlement_id, status, total_amount}            │
│                   │  状态: SETTLEMENT_PROCESSING / SETTLEMENT_COMPLETED     │
│                                                                             │
│ [Callback] 清算机构回调 POST /webhook/settlement-result                     │
│                   ├→ SettlementResultCallbackCommand ❌ **缺失**            │
│                   │  L5内部步骤: 验签→解析结果→更新状态→生成报告→通知       │
│                   │  状态: SETTLEMENT_SUCCESS / SETTLEMENT_FAILED           │
│                                                                             │
│ [Query] 运营查询 GET /api/settlement/daily/{date}/report                    │
│                   └→ QueryDailySettlementReportQuery                        │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.4.2 实时清算流程(API调用视图)                                             │
│                                                                             │
│ 说明: 单笔交易完成后自动触发实时清算,属于L5内部步骤                          │
│       运营可查询清算状态和手动重试失败的清算                                 │
│                                                                             │
│ [API 1] 运营调用 POST /api/settlement/realtime/retry (重试失败清算)         │
│                   ├→ RetryRealtimeSettlementCommand ❌ **缺失**             │
│                   │  L5内部步骤: 校验状态→封装清算报文→发送清算→等待确认     │
│                   │  返回: {settlement_id, status}                          │
│                   │  状态: RETRY_PROCESSING / RETRY_SUCCESS                 │
│                                                                             │
│ [Query] 运营查询 GET /api/settlement/realtime/{id}                          │
│                   └→ QueryRealtimeSettlementQuery                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.4.3 对账流程(API调用视图)                                                 │
│                                                                             │
│ [API 1] 运营调用 POST /api/reconciliation/initiate                          │
│                   ├→ InitiateReconciliationCommand ❌ **缺失**              │
│                   │  L5内部步骤: 获取对账单→解析文件→加载本地数据            │
│                   │              →自动匹配→标记差异→生成报告                │
│                   │  返回: {recon_id, total_count, matched_count,           │
│                   │          diff_count, report_url}                        │
│                   │  状态: RECON_COMPLETED                                  │
│                                                                             │
│ [Callback] 通道对账文件推送 POST /webhook/channel-statement                 │
│                   ├→ ChannelStatementReceivedCommand ❌ **缺失**            │
│                   │  L5内部步骤: 验签→保存文件→触发对账任务                  │
│                   │  状态: FILE_RECEIVED                                    │
│                                                                             │
│ [Query] 运营查询 GET /api/reconciliation/{id}/report                        │
│                   └→ QueryReconciliationReportQuery                         │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.4.4 差错处理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/discrepancy/create                               │
│                   ├→ CreateDiscrepancyCommand ❌ **缺失**                   │
│                   │  L5内部步骤: 差异分析→原因判定→生成工单→分配处理人       │
│                   │  返回: {discrepancy_id, type, amount, status}           │
│                   │  状态: PENDING_RESOLUTION                               │
│                                                                             │
│ [API 2] 运营调用 POST /api/discrepancy/{id}/resolve                         │
│                   ├→ ResolveDiscrepancyCommand ❌ **缺失**                  │
│                   │  L5内部步骤: 处理方案选择→调账执行→审批确认→结案归档     │
│                   │  返回: {discrepancy_id, resolution, status}             │
│                   │  状态: RESOLVED                                         │
│                                                                             │
│ [API 3] 运营调用 POST /api/account/adjust (需审批)                          │
│                   ├→ AdjustAccountCommand ✅                                │
│                   │  L5内部步骤: 调账验证→生成凭证→账务更新→通知            │
│                   │  返回: {adjustment_id, amount, status}                  │
│                                                                             │
│ [Query] 运营查询 GET /api/discrepancy/list                                  │
│                   └→ QueryDiscrepancyListQuery                              │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.4.5 资金归集流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/fund-pooling/initiate                            │
│                   ├→ InitiateFundPoolingCommand ❌ **缺失**                 │
│                   │  L5内部步骤: 余额监控→归集决策→生成归集指令              │
│                   │              →发起划拨→等待到账→余额更新                │
│                   │  返回: {pooling_id, from_accounts, to_account,          │
│                   │          total_amount, status}                          │
│                   │  状态: POOLING_PROCESSING / POOLING_COMPLETED           │
│                                                                             │
│ [API 2] 运营调用 POST /api/fund-pooling/auto-config                         │
│                   ├→ ConfigureAutoPoolingCommand ❌ **缺失**                │
│                   │  L5内部步骤: 配置阈值→设置规则→启用自动归集              │
│                   │  返回: {config_id, threshold, schedule}                 │
│                                                                             │
│ [Callback] 银行到账通知 POST /webhook/fund-arrival                          │
│                   ├→ FundArrivalConfirmCommand ❌ **缺失**                  │
│                   │  L5内部步骤: 验签→匹配归集单→更新状态→余额确认           │
│                   │  状态: ARRIVAL_CONFIRMED                                │
│                                                                             │
│ [Query] 运营查询 GET /api/fund-pooling/history                              │
│                   └→ QueryFundPoolingHistoryQuery                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.5 L2.5 风控合规域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.5.1 交易风控流程(API调用视图)                                             │
│                                                                             │
│ 说明: 实时风控扫描属于L5内部步骤,运营主要进行规则配置和人工复核              │
│                                                                             │
│ [API 1] 运营调用 POST /api/risk/rules/configure                             │
│                   ├→ ConfigureRiskRuleCommand ✅                            │
│                   │  L5内部步骤: 规则验证→版本管理→生效部署→通知            │
│                   │  返回: {rule_id, version, status}                       │
│                   │  状态: RULE_ACTIVE                                      │
│                                                                             │
│ [API 2] 运营调用 POST /api/risk/manual-review                               │
│                   ├→ ManualRiskReviewCommand ✅                             │
│                   │  L5内部步骤: 加载案件→证据分析→决策记录→结果通知        │
│                   │  返回: {review_id, decision, reason}                    │
│                   │  决策: APPROVED / REJECTED / PENDING                    │
│                                                                             │
│ [Query] 运营查询 GET /api/risk/alerts                                       │
│                   └→ QueryRiskAlertsQuery                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.5.2 AML检测流程(API调用视图)                                              │
│                                                                             │
│ 说明: AML规则检测属于L5内部步骤,运营进行可疑交易调查和SAR报告提交            │
│                                                                             │
│ [API 1] 运营调用 POST /api/aml/investigation/create                         │
│                   ├→ CreateAmlInvestigationCommand ❌ **缺失**              │
│                   │  L5内部步骤: 创建案件→收集证据→分配调查员→设置截止日期   │
│                   │  返回: {investigation_id, assigned_to, deadline}        │
│                   │  状态: INVESTIGATION_OPEN                               │
│                                                                             │
│ [API 2] 运营调用 POST /api/aml/investigation/{id}/conclude                  │
│                   ├→ ConcludeAmlInvestigationCommand ❌ **缺失**            │
│                   │  L5内部步骤: 结论记录→案件归档→生成报告                  │
│                   │  返回: {investigation_id, conclusion, report_url}       │
│                   │  结论: SUSPICIOUS / NORMAL                              │
│                                                                             │
│ [API 3] 运营调用 POST /api/aml/sar/submit (可疑活动报告)                    │
│                   ├→ SubmitSarReportCommand ✅                              │
│                   │  L5内部步骤: 报告生成→合规审核→加密签名→监管提交        │
│                   │  返回: {sar_id, submission_time, status}                │
│                   │  状态: SAR_SUBMITTED                                    │
│                                                                             │
│ [Query] 运营查询 GET /api/aml/suspicious-patterns                           │
│                   └→ QuerySuspiciousPatternsQuery                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.5.3 制裁筛查流程(API调用视图)                                             │
│                                                                             │
│ 说明: 自动制裁筛查属于L5内部步骤,运营处理命中案件和白名单管理                │
│                                                                             │
│ [API 1] 运营调用 POST /api/sanction/hits/{id}/review                        │
│                   ├→ ReviewSanctionHitCommand ❌ **缺失**                   │
│                   │  L5内部步骤: 加载命中记录→人工复核→误报判定→处置决策     │
│                   │  返回: {hit_id, decision, action}                       │
│                   │  决策: FALSE_POSITIVE / TRUE_HIT / UNCERTAIN            │
│                                                                             │
│ [API 2] 运营调用 POST /api/sanction/whitelist/add                           │
│                   ├→ AddToSanctionWhitelistCommand ❌ **缺失**              │
│                   │  L5内部步骤: 白名单审批→添加→生效→通知                   │
│                   │  返回: {whitelist_id, entity, status}                   │
│                                                                             │
│ [API 3] 运营调用 POST /api/sanction/list/update (更新制裁名单)              │
│                   ├→ UpdateSanctionListCommand ✅                           │
│                   │  L5内部步骤: 下载更新→增量比对→数据加载→生效通知        │
│                   │  返回: {update_id, source, records_added}               │
│                                                                             │
│ [Query] 运营查询 GET /api/sanction/hits/pending                             │
│                   └→ QueryPendingSanctionHitsQuery                          │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.5.4 合规报送流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/compliance/report/generate                       │
│                   ├→ GenerateComplianceReportCommand ❌ **缺失**            │
│                   │  L5内部步骤: 数据抽取→报表生成→格式转换→预览            │
│                   │  返回: {report_id, type, preview_url}                   │
│                   │  状态: REPORT_GENERATED                                 │
│                                                                             │
│ [API 2] 运营调用 POST /api/compliance/report/{id}/submit                    │
│                   ├→ SubmitComplianceReportCommand ❌ **缺失**              │
│                   │  L5内部步骤: 合规审核→加密签名→监管提交→确认回执        │
│                   │  返回: {report_id, submission_time, receipt}            │
│                   │  状态: SUBMITTED                                        │
│                                                                             │
│ [Query] 运营查询 GET /api/compliance/report/history                         │
│                   └→ QueryComplianceReportHistoryQuery                      │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.5.5 人工审核流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/review/task/assign                               │
│                   ├→ AssignReviewTaskCommand ❌ **缺失**                    │
│                   │  L5内部步骤: 任务分配→通知审核员→设置优先级              │
│                   │  返回: {task_id, assigned_to, priority}                 │
│                   │  状态: TASK_ASSIGNED                                    │
│                                                                             │
│ [API 2] 运营调用 POST /api/review/task/{id}/complete                        │
│                   ├→ CompleteReviewTaskCommand ❌ **缺失**                  │
│                   │  L5内部步骤: 审核决策→证据附件→结果记录→流转下一步      │
│                   │  返回: {task_id, decision, next_action}                 │
│                   │  决策: APPROVED / REJECTED / ESCALATED                  │
│                                                                             │
│ [Query] 运营查询 GET /api/review/task/queue                                 │
│                   └→ QueryReviewTaskQueueQuery                              │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.5.6 黑名单管理流程(API调用视图)                                           │
│                                                                             │
│ [API 1] 运营调用 POST /api/blacklist/add                                    │
│                   ├→ AddToBlacklistCommand ✅                               │
│                   │  L5内部步骤: 审核确认→添加名单→生效同步→通知下发        │
│                   │  返回: {blacklist_id, entity_type, entity_id}           │
│                   │  状态: BLACKLIST_ACTIVE                                 │
│                                                                             │
│ [API 2] 运营调用 DELETE /api/blacklist/{id}                                 │
│                   ├→ RemoveFromBlacklistCommand ✅                          │
│                   │  L5内部步骤: 主管审批→移除名单→失效通知                  │
│                   │  返回: {blacklist_id, removed_at}                       │
│                   │  状态: REMOVED                                          │
│                                                                             │
│ [Query] 运营查询 GET /api/blacklist/search                                  │
│                   └→ QueryBlacklistQuery                                    │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.6 L2.6 争议处理域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.6.1 拒付处理流程(API调用视图)                                             │
│                                                                             │
│ [Callback] 卡组织拒付通知 POST /webhook/chargeback-notify                   │
│                   ├→ ChargebackNotifyCommand ✅                             │
│                   │  L5内部步骤: 验签→解析拒付原因→创建案件→分配处理人       │
│                   │  返回: {chargeback_id, reason, deadline}                │
│                   │  状态: CHARGEBACK_RECEIVED                              │
│                                                                             │
│ [API 1] 运营调用 POST /api/chargeback/{id}/dispute                          │
│                   ├→ DisputeChargebackCommand ✅                            │
│                   │  L5内部步骤: 收集证据→准备举证材料→提交抗辩              │
│                   │  返回: {dispute_id, evidence_submitted, status}         │
│                   │  状态: DISPUTE_SUBMITTED                                │
│                                                                             │
│ [API 2] 运营调用 POST /api/chargeback/{id}/accept                           │
│                   ├→ AcceptChargebackCommand ✅                             │
│                   │  L5内部步骤: 确认接受→账务冲正→资金退回→案件结案        │
│                   │  返回: {chargeback_id, refund_amount, status}           │
│                   │  状态: CHARGEBACK_ACCEPTED                              │
│                                                                             │
│ [Callback] 仲裁结果通知 POST /webhook/arbitration-result                    │
│                   ├→ ArbitrationResultCommand ✅                            │
│                   │  L5内部步骤: 解析裁决→更新案件状态→执行裁决→记录       │
│                   │  状态: ARBITRATION_WIN / ARBITRATION_LOSS               │
│                                                                             │
│ [Query] 运营查询 GET /api/chargeback/pending                                │
│                   └→ QueryPendingChargebacksQuery                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.6.2 争议仲裁流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/arbitration/initiate                             │
│                   ├→ InitiateArbitrationCommand ❌ **缺失**                 │
│                   │  L5内部步骤: 提交仲裁申请→缴纳仲裁费→提交证据链          │
│                   │  返回: {arbitration_id, case_number, status}            │
│                   │  状态: ARBITRATION_INITIATED                            │
│                                                                             │
│ [API 2] 运营调用 POST /api/arbitration/{id}/submit-evidence                 │
│                   ├→ SubmitArbitrationEvidenceCommand ❌ **缺失**           │
│                   │  L5内部步骤: 上传证据→审核→提交→通知对方                │
│                   │  返回: {evidence_id, submitted_at}                      │
│                                                                             │
│ [Query] 运营查询 GET /api/arbitration/{id}/status                           │
│                   └→ QueryArbitrationStatusQuery                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.6.3 赔付管理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/compensation/request                             │
│                   ├→ RequestCompensationCommand ❌ **缺失**                 │
│                   │  L5内部步骤: 责任认定→金额核算→生成赔付申请              │
│                   │  返回: {compensation_id, amount, reason}                │
│                   │  状态: PENDING_APPROVAL                                 │
│                                                                             │
│ [API 2] 运营调用 POST /api/compensation/{id}/approve                        │
│                   ├→ ApproveCompensationCommand ✅                          │
│                   │  L5内部步骤: 审批通过→资金划付→通知客户→结案归档        │
│                   │  返回: {compensation_id, approved_amount, status}       │
│                   │  状态: COMPENSATION_PAID                                │
│                                                                             │
│ [Query] 运营查询 GET /api/compensation/pending-approval                     │
│                   └→ QueryPendingCompensationQuery                          │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.6.4 举证调单流程(API调用视图)                                             │
│                                                                             │
│ [Callback] 卡组织调单请求 POST /webhook/retrieval-request                   │
│                   ├→ RetrievalRequestCommand ✅                             │
│                   │  L5内部步骤: 解析请求→查询原交易→收集资料→生成工单      │
│                   │  返回: {retrieval_id, deadline, required_docs}          │
│                   │  状态: RETRIEVAL_PENDING                                │
│                                                                             │
│ [API 1] 运营调用 POST /api/retrieval/{id}/submit-docs                       │
│                   ├→ SubmitRetrievalDocsCommand ❌ **缺失**                 │
│                   │  L5内部步骤: 整理证据→格式封装→提交卡组织→确认回执      │
│                   │  返回: {retrieval_id, submitted_at, docs_list}          │
│                   │  状态: DOCS_SUBMITTED                                   │
│                                                                             │
│ [Query] 运营查询 GET /api/retrieval/pending                                 │
│                   └→ QueryPendingRetrievalQuery                             │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.6.5 客户申诉流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/complaint/submit                                 │
│                   ├→ SubmitComplaintCommand ✅                              │
│                   │  L5内部步骤: 创建申诉工单→分配客服→设置优先级            │
│                   │  返回: {complaint_id, assigned_to, sla_deadline}        │
│                   │  状态: COMPLAINT_OPEN                                   │
│                                                                             │
│ [API 2] 客户调用 POST /api/complaint/{id}/add-evidence                      │
│                   ├→ SubmitEvidenceCommand ✅                               │
│                   │  L5内部步骤: 附件上传→关联工单→通知处理人                │
│                   │  返回: {evidence_id, uploaded_at}                       │
│                                                                             │
│ [API 3] 运营调用 POST /api/complaint/{id}/assign                            │
│                   ├→ AssignComplaintCommand ✅                              │
│                   │  L5内部步骤: 分配处理人→通知→更新工单                    │
│                   │  返回: {complaint_id, assigned_to}                      │
│                                                                             │
│ [API 4] 运营调用 POST /api/complaint/{id}/process                           │
│                   ├→ ProcessComplaintCommand ✅                             │
│                   │  L5内部步骤: 调查核实→处理方案→回复客户                  │
│                   │  返回: {complaint_id, resolution, status}               │
│                   │  状态: PROCESSING / RESOLVED                            │
│                                                                             │
│ [API 5] 运营调用 POST /api/complaint/{id}/close                             │
│                   ├→ CloseComplaintCommand ✅                               │
│                   │  L5内部步骤: 确认解决→满意度调查→结案归档                │
│                   │  返回: {complaint_id, closed_at, satisfaction}          │
│                   │  状态: CLOSED                                           │
│                                                                             │
│ [API 6] 客户调用 DELETE /api/complaint/{id}                                 │
│                   ├→ WithdrawComplaintCommand ✅                            │
│                   │  L5内部步骤: 确认撤回→更新状态→通知处理人                │
│                   │  状态: WITHDRAWN                                        │
│                                                                             │
│ [Query] 客户查询 GET /api/complaint/{id}/status                             │
│                   └→ QueryComplaintStatusQuery                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.7 L2.7 商户管理域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.7.1 商户入驻流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 商户调用 POST /api/merchant/apply                                   │
│                   ├→ SubmitMerchantApplicationCommand ✅                    │
│                   │  L5内部步骤: 提交申请→初审→生成入驻单                    │
│                   │  返回: {application_id, status}                         │
│                   │  状态: APPLICATION_SUBMITTED                            │
│                                                                             │
│ [API 2] 商户调用 POST /api/merchant/qualification/submit                    │
│                   ├→ SubmitQualificationCommand ✅                          │
│                   │  L5内部步骤: 上传资质→OCR识别→补充材料                   │
│                   │  返回: {qualification_id, uploaded_docs}                │
│                                                                             │
│ [API 3] 运营调用 POST /api/merchant/application/{id}/review                 │
│                   ├→ ReviewMerchantApplicationCommand ✅                    │
│                   │  L5内部步骤: KYB审核→资质验证→决策记录                   │
│                   │  返回: {application_id, decision, reason}               │
│                   │  决策: APPROVED / REJECTED / PENDING                    │
│                                                                             │
│ [API 4] 运营调用 POST /api/merchant/application/{id}/approve                │
│                   ├→ ApproveMerchantCommand ✅                              │
│                   │  L5内部步骤: 批准→创建商户账号→开通权限→发送通知        │
│                   │  返回: {merchant_id, credentials, api_key}              │
│                   │  状态: MERCHANT_ACTIVE                                  │
│                                                                             │
│ [Query] 商户查询 GET /api/merchant/application/{id}                         │
│                   └→ QueryApplicationStatusQuery                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.7.2 合同管理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/contract/draft                                   │
│                   ├→ DraftContractCommand ❌ **缺失**                       │
│                   │  L5内部步骤: 选择模板→条款协商→生成合同草案              │
│                   │  返回: {contract_id, draft_url}                         │
│                   │  状态: DRAFT                                            │
│                                                                             │
│ [API 2] 商户调用 POST /api/contract/{id}/sign                               │
│                   ├→ SignContractCommand ✅                                 │
│                   │  L5内部步骤: 在线签署→电子签名→合同归档                  │
│                   │  返回: {contract_id, signed_at, pdf_url}                │
│                   │  状态: CONTRACT_ACTIVE                                  │
│                                                                             │
│ [Query] 商户查询 GET /api/contract/list                                     │
│                   └→ QueryContractListQuery                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.7.3 商户分级流程(API调用视图)                                             │
│                                                                             │
│ 说明: 商户分级主要由系统根据交易数据自动计算,运营可手动调整                  │
│                                                                             │
│ [API 1] 运营调用 POST /api/merchant/{id}/tier/adjust                        │
│                   ├→ AdjustMerchantTierCommand ✅                           │
│                   │  L5内部步骤: 评估指标→等级调整→权益更新→通知商户        │
│                   │  返回: {merchant_id, new_tier, benefits}                │
│                   │  等级: VIP / PREMIUM / STANDARD / BASIC                 │
│                                                                             │
│ [Query] 商户查询 GET /api/merchant/{id}/tier/history                        │
│                   └→ QueryTierHistoryQuery                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.7.4 API接入流程(API调用视图)                                              │
│                                                                             │
│ [API 1] 商户调用 POST /api/merchant/api-access/apply                        │
│                   ├→ ApplyApiAccessCommand ✅                               │
│                   │  L5内部步骤: 申请审核→生成密钥→沙箱账号开通              │
│                   │  返回: {api_key, secret_key, sandbox_url}               │
│                   │  状态: SANDBOX_ACTIVE                                   │
│                                                                             │
│ [API 2] 商户调用 POST /api/merchant/api-key/reset                           │
│                   ├→ ResetApiKeyCommand ✅                                  │
│                   │  L5内部步骤: 验证身份→旧密钥失效→生成新密钥              │
│                   │  返回: {new_api_key, new_secret}                        │
│                                                                             │
│ [API 3] 商户调用 POST /api/merchant/webhook/configure                       │
│                   ├→ ConfigureWebhookCommand ✅                             │
│                   │  L5内部步骤: URL验证→签名配置→事件订阅                   │
│                   │  返回: {webhook_id, url, events}                        │
│                                                                             │
│ [API 4] 商户调用 POST /api/merchant/ip-whitelist/configure                  │
│                   ├→ ConfigureIpWhitelistCommand ✅                         │
│                   │  L5内部步骤: IP验证→规则配置→生效                        │
│                   │  返回: {whitelist_id, ips}                              │
│                                                                             │
│ [API 5] 运营调用 POST /api/merchant/{id}/api/activate                       │
│                   ├→ ActivateProductionApiCommand ❌ **缺失**               │
│                   │  L5内部步骤: 联调测试验收→正式环境激活→限额配置          │
│                   │  返回: {merchant_id, production_active}                 │
│                   │  状态: PRODUCTION_ACTIVE                                │
│                                                                             │
│ [Query] 商户查询 GET /api/merchant/api/credentials                          │
│                   └→ QueryApiCredentialsQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.7.5 商户结算流程(API调用视图)                                             │
│                                                                             │
│ 说明: 商户结算主要由系统按T+N周期自动执行,商户可查询和下载结算报表           │
│                                                                             │
│ [API 1] 运营调用 POST /api/merchant/{id}/settlement/trigger                 │
│                   ├→ TriggerMerchantSettlementCommand ❌ **缺失**           │
│                   │  L5内部步骤: 交易汇总→费用扣除→净额计算→打款执行        │
│                   │  返回: {settlement_id, net_amount, status}              │
│                   │  状态: SETTLEMENT_PROCESSING / COMPLETED                │
│                                                                             │
│ [Query] 商户查询 GET /api/merchant/settlement/history                       │
│                   └→ QuerySettlementHistoryQuery                            │
│                                                                             │
│ [Query] 商户查询 GET /api/merchant/settlement/{id}/report                   │
│                   └→ DownloadSettlementReportQuery                          │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.8 L2.8 账户管理域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.8.1 客户注册流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/auth/register                                    │
│                   ├→ SubmitRegistrationCommand ✅                           │
│                   │  L5内部步骤: 信息填写→重复检查→创建账户                  │
│                   │  返回: {user_id, verification_required}                 │
│                   │  状态: PENDING_VERIFICATION                             │
│                                                                             │
│ [API 2] 客户调用 POST /api/auth/verify-contact                              │
│                   ├→ VerifyContactCommand ✅                                │
│                   │  L5内部步骤: 发送验证码→验证→激活账户                    │
│                   │  返回: {verified: true, access_token}                   │
│                   │  状态: ACCOUNT_ACTIVE                                   │
│                                                                             │
│ [API 3] 客户调用 POST /api/auth/set-password                                │
│                   ├→ SetPasswordCommand ✅                                  │
│                   │  L5内部步骤: 密码强度校验→加密存储→通知                  │
│                   │  返回: {password_set: true}                             │
│                                                                             │
│ [Query] 客户查询 GET /api/user/profile                                      │
│                   └→ QueryUserProfileQuery                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.8.2 KYC/KYB认证流程(API调用视图)                                          │
│                                                                             │
│ [API 1] 客户调用 POST /api/kyc/submit                                       │
│                   ├→ SubmitKycDocumentsCommand ✅                           │
│                   │  L5内部步骤: 文件上传→OCR识别→活体检测→提交审核         │
│                   │  返回: {kyc_id, status, ocr_result}                     │
│                   │  状态: KYC_PENDING_REVIEW                               │
│                                                                             │
│ [API 2] 客户调用 POST /api/kyb/submit                                       │
│                   ├→ SubmitKybDocumentsCommand ✅                           │
│                   │  L5内部步骤: 企业资质上传→工商验证→法人验证→提交       │
│                   │  返回: {kyb_id, status, verification_result}            │
│                   │  状态: KYB_PENDING_REVIEW                               │
│                                                                             │
│ [API 3] 客户调用 POST /api/kyc/{id}/supplement                              │
│                   ├→ SubmitAdditionalDocsCommand ✅                         │
│                   │  L5内部步骤: 补充材料→重新提交                           │
│                   │  返回: {kyc_id, supplemented_docs}                      │
│                                                                             │
│ [API 4] 运营调用 POST /api/kyc/{id}/review                                  │
│                   ├→ ReviewKycCommand ✅                                    │
│                   │  L5内部步骤: 人工审核→OCR复核→决策记录                   │
│                   │  返回: {kyc_id, decision, reason}                       │
│                   │  决策: APPROVED / REJECTED / REQUEST_SUPPLEMENT         │
│                                                                             │
│ [API 5] 运营调用 POST /api/kyc/{id}/approve                                 │
│                   ├→ ApproveKycKybCommand ✅                                │
│                   │  L5内部步骤: 通过认证→开通权限→通知客户                  │
│                   │  返回: {kyc_id, approved_at, level}                     │
│                   │  状态: KYC_APPROVED                                     │
│                                                                             │
│ [Query] 客户查询 GET /api/kyc/status                                        │
│                   └→ QueryKycStatusQuery                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.8.3 账户开立流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/account/open                                     │
│                   ├→ ApplyAccountOpeningCommand ✅                          │
│                   │  L5内部步骤: KYC检查→账户类型选择→创建→初始化钱包       │
│                   │  返回: {account_id, account_number, wallets}            │
│                   │  状态: ACCOUNT_OPENED                                   │
│                                                                             │
│ [Query] 客户查询 GET /api/account/details                                   │
│                   └→ QueryAccountDetailsQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.8.4 钱包管理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/wallet/create                                    │
│                   ├→ CreateWalletCommand ✅                                 │
│                   │  L5内部步骤: 币种选择→钱包生成→限额配置→激活            │
│                   │  返回: {wallet_id, currency, balance}                   │
│                   │  状态: WALLET_ACTIVE                                    │
│                                                                             │
│ [Query] 客户查询 GET /api/wallet/balance                                    │
│                   └→ QueryBalanceQuery                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.8.5 余额管理流程(API调用视图)                                             │
│                                                                             │
│ 说明: 余额变动由交易触发,属于L5内部步骤,客户主要查询余额和交易明细           │
│                                                                             │
│ [Query] 客户查询 GET /api/balance/multi-currency                            │
│                   └→ QueryMultiCurrencyBalanceQuery                         │
│                                                                             │
│ [Query] 客户查询 GET /api/transactions/history                              │
│                   └→ QueryTransactionHistoryQuery                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.8.6 绑卡管理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 客户调用 POST /api/card/bind/initiate                               │
│                   ├→ InitiateBindCardCommand ✅                             │
│                   │  L5内部步骤: 卡要素验证→银行卡鉴权→发送短信              │
│                   │  返回: {bind_id, verification_code_sent}                │
│                   │  状态: PENDING_SMS_VERIFICATION                         │
│                                                                             │
│ [API 2] 客户调用 POST /api/card/bind/verify-sms                             │
│                   ├→ VerifyBindCardSmsCommand ✅                            │
│                   │  L5内部步骤: 验证码校验→协议签约→绑卡完成                │
│                   │  返回: {card_id, masked_number, bank_name}              │
│                   │  状态: CARD_BOUND                                       │
│                                                                             │
│ [API 3] 客户调用 DELETE /api/card/{id}                                      │
│                   ├→ UnbindCardCommand ✅                                   │
│                   │  L5内部步骤: 确认解绑→协议解约→移除卡信息                │
│                   │  返回: {card_id, unbound_at}                            │
│                   │  状态: CARD_UNBOUND                                     │
│                                                                             │
│ [API 4] 客户调用 PUT /api/card/{id}/set-default                             │
│                   ├→ SetDefaultCardCommand ✅                               │
│                   │  L5内部步骤: 更新默认卡→通知                             │
│                   │  返回: {card_id, is_default: true}                      │
│                                                                             │
│ [Query] 客户查询 GET /api/card/list                                         │
│                   └→ QueryBoundCardsQuery                                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.9 L2.9 通道管理域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.9.1 通道接入流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/channel/add                                      │
│                   ├→ AddChannelCommand ✅                                   │
│                   │  L5内部步骤: 通道评估→合同签署→技术对接→创建配置        │
│                   │  返回: {channel_id, name, status}                       │
│                   │  状态: CHANNEL_CREATED                                  │
│                                                                             │
│ [API 2] 运营调用 POST /api/channel/{id}/configure                           │
│                   ├→ ConfigureChannelCommand ✅                             │
│                   │  L5内部步骤: 参数配置→沙箱测试→联调验收→上线发布        │
│                   │  返回: {channel_id, config, test_result}                │
│                   │  状态: CHANNEL_CONFIGURED                               │
│                                                                             │
│ [API 3] 运营调用 PUT /api/channel/{id}/enable                               │
│                   ├→ EnableChannelCommand ✅                                │
│                   │  L5内部步骤: 启用通道→加入路由→通知                      │
│                   │  状态: CHANNEL_ACTIVE                                   │
│                                                                             │
│ [API 4] 运营调用 PUT /api/channel/{id}/disable                              │
│                   ├→ DisableChannelCommand ✅                               │
│                   │  L5内部步骤: 禁用通道→移出路由→通知                      │
│                   │  状态: CHANNEL_DISABLED                                 │
│                                                                             │
│ [Query] 运营查询 GET /api/channel/list                                      │
│                   └→ QueryChannelListQuery                                  │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.9.2 通道监控流程(API调用视图)                                             │
│                                                                             │
│ 说明: 通道健康检查属于系统定时任务,运营主要查看监控数据和处理告警            │
│                                                                             │
│ [Query] 运营查询 GET /api/channel/{id}/metrics                              │
│                   └→ QueryChannelMetricsQuery                               │
│                                                                             │
│ [Query] 运营查询 GET /api/channel/alerts                                    │
│                   └→ QueryChannelAlertsQuery                                │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.9.3 智能路由流程(API调用视图)                                             │
│                                                                             │
│ 说明: 智能路由是系统自动执行,运营进行规则配置                                │
│                                                                             │
│ [API 1] 运营调用 POST /api/routing/rule/configure                           │
│                   ├→ ConfigureRoutingRuleCommand ✅                         │
│                   │  L5内部步骤: 规则配置→优先级设置→权重分配→生效          │
│                   │  返回: {rule_id, priority, channels}                    │
│                   │  状态: RULE_ACTIVE                                      │
│                                                                             │
│ [API 2] 运营调用 PUT /api/routing/channel-weight/adjust                     │
│                   ├→ AdjustChannelWeightCommand ✅                          │
│                   │  L5内部步骤: 调整权重→重新分配流量→生效                  │
│                   │  返回: {channel_weights}                                │
│                                                                             │
│ [Query] 运营查询 GET /api/routing/rules                                     │
│                   └→ QueryRoutingRulesQuery                                 │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.9.4 限额管理流程(API调用视图)                                             │
│                                                                             │
│ [API 1] 运营调用 POST /api/channel/{id}/limit/set                           │
│                   ├→ SetChannelLimitCommand ✅                              │
│                   │  L5内部步骤: 审批→配置限额→生效→监控预警                │
│                   │  返回: {limit_id, daily_limit, transaction_limit}       │
│                   │  状态: LIMIT_ACTIVE                                     │
│                                                                             │
│ [Query] 运营查询 GET /api/channel/{id}/limit/usage                          │
│                   └→ QueryChannelLimitUsageQuery                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.9.5 通道切换流程(API调用视图)                                             │
│                                                                             │
│ 说明: 故障检测和自动切换属于系统监控任务,运营可手动触发切换                  │
│                                                                             │
│ [API 1] 运营调用 POST /api/channel/switch                                   │
│                   ├→ SwitchChannelCommand ✅                                │
│                   │  L5内部步骤: 切换决策→流量迁移→备用接管→监控验证        │
│                   │  返回: {switch_id, from_channel, to_channel, status}    │
│                   │  状态: SWITCH_COMPLETED                                 │
│                                                                             │
│ [Query] 运营查询 GET /api/channel/switch/history                            │
│                   └→ QueryChannelSwitchHistoryQuery                         │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.10 L2.10 费用管理域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.10.1 费率配置流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 运营调用 POST /api/fee/base-rate/configure                          │
│                   ├→ ConfigureBaseFeeCommand ✅                             │
│                   │  L5内部步骤: 费率配置→审批→生效→通知商户                │
│                   │  返回: {fee_config_id, rates, effective_date}           │
│                   │  状态: FEE_CONFIG_ACTIVE                                │
│                                                                             │
│ [API 2] 运营调用 POST /api/fee/tiered-rate/configure                        │
│                   ├→ ConfigureTieredFeeCommand ✅                           │
│                   │  L5内部步骤: 阶梯定价配置→审批→生效                      │
│                   │  返回: {tier_config_id, tiers}                          │
│                                                                             │
│ [API 3] 运营调用 POST /api/fee/special-rate/set                             │
│                   ├→ SetSpecialFeeCommand ✅                                │
│                   │  L5内部步骤: 特殊费率设置→客户绑定→生效                  │
│                   │  返回: {special_rate_id, customer_id, rate}             │
│                                                                             │
│ [Query] 运营查询 GET /api/fee/config/{customer_id}                          │
│                   └→ QueryFeeConfigQuery                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.10.2 阶梯定价流程(API调用视图)                                            │
│                                                                             │
│ 说明: 阶梯定价自动计算属于L5内部步骤,运营配置阶梯规则                        │
│                                                                             │
│ [Query] 客户查询 GET /api/fee/my-tier                                       │
│                   └→ QueryMyTierQuery                                       │
│                                                                             │
│ [Query] 客户查询 GET /api/fee/estimate                                      │
│                   └→ EstimateFeeQuery                                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.10.3 分润结算流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 运营调用 POST /api/profit-sharing/rule/configure                    │
│                   ├→ ConfigureSharingRuleCommand ✅                         │
│                   │  L5内部步骤: 分润比例配置→合作方绑定→生效                │
│                   │  返回: {rule_id, partners, ratios}                      │
│                                                                             │
│ [API 2] 运营调用 POST /api/profit-sharing/settle                            │
│                   ├→ ExecuteSharingSettlementCommand ✅                     │
│                   │  L5内部步骤: 收入统计→分润计算→审核→打款执行            │
│                   │  返回: {settlement_id, settlements, status}             │
│                   │  状态: SETTLEMENT_PROCESSING / COMPLETED                │
│                                                                             │
│ [Query] 运营查询 GET /api/profit-sharing/history                            │
│                   └→ QuerySharingHistoryQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.10.4 成本核算流程(API调用视图)                                            │
│                                                                             │
│ 说明: 成本核算主要由系统定期批量计算,运营查看报表                            │
│                                                                             │
│ [Query] 运营查询 GET /api/cost/report/{period}                              │
│                   └→ QueryCostReportQuery                                   │
│                                                                             │
│ [Query] 运营查询 GET /api/profit/analysis                                   │
│                   └→ QueryProfitAnalysisQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.10.5 账单生成流程(API调用视图)                                            │
│                                                                             │
│ 说明: 账单生成主要由系统定期自动执行,运营可查询和手动生成                    │
│                                                                             │
│ [API 1] 运营调用 POST /api/invoice/generate                                 │
│                   ├→ GenerateInvoiceCommand ✅                              │
│                   │  L5内部步骤: 费用汇总→账单生成→审核→发送商户            │
│                   │  返回: {invoice_id, amount, pdf_url}                    │
│                   │  状态: INVOICE_SENT                                     │
│                                                                             │
│ [API 2] 客户调用 POST /api/invoice/request                                  │
│                   ├→ RequestInvoiceCommand ✅                               │
│                   │  L5内部步骤: 开票申请→审核→生成→邮寄/下载                │
│                   │  返回: {invoice_request_id, status}                     │
│                                                                             │
│ [Query] 客户查询 GET /api/invoice/list                                      │
│                   └→ QueryInvoiceListQuery                                  │
│                                                                             │
│ [Query] 客户下载 GET /api/invoice/{id}/download                             │
│                   └→ DownloadInvoiceQuery                                   │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.11 L2.11 资金管理域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.11.1 备付金监控流程(API调用视图)                                          │
│                                                                             │
│ 说明: 备付金监控由系统自动执行,运营查看监控数据和处理告警                    │
│                                                                             │
│ [Query] 运营查询 GET /api/reserve/balance                                   │
│                   └→ QueryReserveBalanceQuery                               │
│                                                                             │
│ [Query] 运营查询 GET /api/reserve/alerts                                    │
│                   └→ QueryReserveAlertsQuery                                │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.11.2 资金调拨流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 运营调用 POST /api/fund-transfer/initiate                           │
│                   ├→ InitiateFundTransferCommand ✅                         │
│                   │  L5内部步骤: 调拨申请→审批→生成指令→发起划拨            │
│                   │  返回: {transfer_id, from_account, to_account, amount}  │
│                   │  状态: TRANSFER_PROCESSING                              │
│                                                                             │
│ [API 2] 运营调用 POST /api/fund-transfer/{id}/approve                       │
│                   ├→ ApproveFundTransferCommand ✅                          │
│                   │  L5内部步骤: 审批通过→执行划拨→等待到账→余额更新        │
│                   │  状态: TRANSFER_APPROVED / COMPLETED                    │
│                                                                             │
│ [Query] 运营查询 GET /api/fund-transfer/history                             │
│                   └→ QueryFundTransferHistoryQuery                          │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.11.3 流动性预警流程(API调用视图)                                          │
│                                                                             │
│ 说明: 流动性预测由系统自动执行,运营配置预警阈值                              │
│                                                                             │
│ [API 1] 运营调用 POST /api/liquidity/threshold/set                          │
│                   ├→ SetAlertThresholdCommand ✅                            │
│                   │  L5内部步骤: 配置阈值→启用监控→预警规则                  │
│                   │  返回: {threshold_id, levels}                           │
│                                                                             │
│ [API 2] 运营调用 POST /api/liquidity/alert/{id}/handle                      │
│                   ├→ HandleFundAlertCommand ✅                              │
│                   │  L5内部步骤: 分析预警→处理决策→执行→记录                │
│                   │  返回: {alert_id, action, status}                       │
│                                                                             │
│ [Query] 运营查询 GET /api/liquidity/forecast                                │
│                   └→ QueryLiquidityForecastQuery                            │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.11.4 头寸管理流程(API调用视图)                                            │
│                                                                             │
│ 说明: 头寸统计和检查由系统自动执行,运营查看并手动调整                        │
│                                                                             │
│ [Query] 运营查询 GET /api/position/summary                                  │
│                   └→ QueryPositionSummaryQuery                              │
│                                                                             │
│ [Query] 运营查询 GET /api/position/{currency}/detail                        │
│                   └→ QueryPositionDetailQuery                               │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.11.5 银行对接流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 运营调用 POST /api/bank/account/configure                           │
│                   ├→ ConfigureBankAccountCommand ✅                         │
│                   │  L5内部步骤: 开户信息配置→签约授权→渠道对接              │
│                   │  返回: {bank_account_id, bank_name, account_number}     │
│                   │  状态: BANK_ACCOUNT_ACTIVE                              │
│                                                                             │
│ [API 2] 运营调用 POST /api/bank/reconcile/config                            │
│                   ├→ ConfigureBankReconCommand ❌ **缺失**                  │
│                   │  L5内部步骤: 对账配置→文件格式→FTP路径→定时调度         │
│                   │  返回: {recon_config_id, schedule}                      │
│                                                                             │
│ [Query] 运营查询 GET /api/bank/statement/{date}                             │
│                   └→ QueryBankStatementQuery                                │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 2.3.12 L2.12 报表分析域 → L3 流程组

```
┌─────────────────────────────────────────────────────────────────────────────┐
│ L3.12.1 交易报表流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 客户调用 POST /api/report/transaction/export                        │
│                   ├→ ExportTransactionReportCommand ✅                      │
│                   │  L5内部步骤: 数据抽取→报表生成→格式导出                  │
│                   │  返回: {report_id, download_url, expires_at}            │
│                   │  格式: CSV / EXCEL / PDF                                │
│                                                                             │
│ [API 2] 客户调用 POST /api/report/custom/create                             │
│                   ├→ CustomizeReportCommand ✅                              │
│                   │  L5内部步骤: 模板配置→字段选择→过滤条件→保存模板        │
│                   │  返回: {template_id, name}                              │
│                                                                             │
│ [Query] 客户查询 GET /api/report/transaction/summary                        │
│                   └→ QueryTransactionSummaryQuery                           │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.12.2 财务报表流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 运营调用 POST /api/report/financial/generate                        │
│                   ├→ GenerateFinancialReportCommand ✅                      │
│                   │  L5内部步骤: 账务汇总→科目核对→试算平衡→报表生成        │
│                   │  返回: {report_id, period, pdf_url}                     │
│                   │  类型: 资产负债表 / 损益表 / 现金流量表                  │
│                                                                             │
│ [Query] 运营查询 GET /api/report/financial/history                          │
│                   └→ QueryFinancialReportHistoryQuery                       │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.12.3 监管报表流程(API调用视图)                                            │
│                                                                             │
│ [API 1] 运营调用 POST /api/report/regulatory/generate                       │
│                   ├→ GenerateRegulatoryReportCommand ✅                     │
│                   │  L5内部步骤: 数据准备→口径调整→报表填报→合规审核        │
│                   │  返回: {report_id, type, status}                        │
│                   │  类型: 反洗钱报告 / 外汇报告 / 大额交易报告              │
│                   │  状态: DRAFT / PENDING_APPROVAL                         │
│                                                                             │
│ [API 2] 运营调用 POST /api/report/regulatory/{id}/submit                    │
│                   ├→ SubmitRegulatoryReportCommand ✅                       │
│                   │  L5内部步骤: 审核通过→加密签名→监管提交→确认回执        │
│                   │  返回: {report_id, submitted_at, receipt}               │
│                   │  状态: SUBMITTED                                        │
│                                                                             │
│ [Query] 运营查询 GET /api/report/regulatory/pending                         │
│                   └→ QueryPendingRegulatoryReportsQuery                     │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.12.4 经营分析流程(API调用视图)                                            │
│                                                                             │
│ 说明: 经营分析由系统定期自动生成,运营查看多维度分析报告                      │
│                                                                             │
│ [Query] 运营查询 GET /api/analysis/business/{period}                        │
│                   └→ QueryBusinessAnalysisQuery                             │
│                                                                             │
│ [Query] 运营查询 GET /api/analysis/customer-segmentation                    │
│                   └→ QueryCustomerSegmentationQuery                         │
│                                                                             │
│ [Query] 运营查询 GET /api/analysis/trend-forecast                           │
│                   └→ QueryTrendForecastQuery                                │
├─────────────────────────────────────────────────────────────────────────────┤
│ L3.12.5 实时大屏流程(API调用视图)                                            │
│                                                                             │
│ 说明: 实时大屏数据由系统自动刷新,运营配置大屏指标和布局                      │
│                                                                             │
│ [API 1] 运营调用 POST /api/dashboard/configure                              │
│                   ├→ ConfigureDashboardCommand ✅                           │
│                   │  L5内部步骤: 指标配置→布局设计→数据源绑定→保存          │
│                   │  返回: {dashboard_id, widgets}                          │
│                                                                             │
│ [Query] 运营查询 GET /api/dashboard/realtime-metrics                        │
│                   └→ QueryRealtimeMetricsQuery (SSE推送)                    │
│                                                                             │
│ [API 2] 客户调用 POST /api/report/subscribe                                 │
│                   ├→ SubscribeReportCommand ✅                              │
│                   │  L5内部步骤: 订阅配置→设置周期→邮件通知                  │
│                   │  返回: {subscription_id, schedule}                      │
│                                                                             │
│ [API 3] 客户调用 DELETE /api/report/subscribe/{id}                          │
│                   ├→ UnsubscribeReportCommand ✅                            │
│                   │  L5内部步骤: 取消订阅→停止推送                           │
│                   │  状态: UNSUBSCRIBED                                     │
└─────────────────────────────────────────────────────────────────────────────┘
```

---


## 3. L3-L4 映射说明

### 3.0 架构设计原则 ⚠️ 重要

#### 3.0.1 L3与L4的关系澄清

**⚠️ 重要说明**：L3流程组展示的是**客户/运营可主动调用的API端点目录**，每个Command对应一次HTTP联机调用。

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        L3 API调用视图架构理解                                │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  L3 = API端点目录（谁可以调用什么）                                         │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │  [API 1] 客户调用 POST /api/orders                                   │  │
│  │           ├→ CreatePaymentOrderCommand  ← 一次HTTP调用               │  │
│  │           │  L5内部步骤: 验证→风控→路由→冻结  ← 系统自动化           │  │
│  │           │  返回: {order_id, status}                                │  │
│  │                                                                      │  │
│  │  [API 2] 客户调用 POST /api/fx/quote                                 │  │
│  │           ├→ RequestFxQuoteCommand      ← 一次HTTP调用               │  │
│  │           │  L5内部步骤: 聚合报价→计算加点                           │  │
│  │           │  返回: {quote_id, rate}                                  │  │
│  │                                                                      │  │
│  │  [Callback] 通道回调 POST /webhook/payment-result                    │  │
│  │           ├→ PaymentStatusCallbackCommand                            │  │
│  │           │  L5内部步骤: 验签→更新状态→通知                          │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
│  核心原则：                                                                 │
│  1. L3 Command = API endpoint = HTTP call                                  │
│  2. 系统自动化（风控/路由/出款）= Command内部的L5步骤                      │
│  3. Command之间通过事件驱动协作，非直接调用                                 │
│  4. 只展示可触发点，不展示内部流转                                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### 3.0.2 Command协作模式（非调用关系）

L4 Command之间**禁止直接调用**，必须通过以下机制协作：

**模式1：事件驱动（Event-Driven）- 推荐 ✅**

```rust
// ✅ 正确：Command执行完成后发布事件
impl CreatePaymentOrderCommand {
    async fn execute(&self, ctx: &CommandContext) -> Result<OrderId> {
        let order = Order::new(...);
        ctx.repo.save(&order).await?;

        // 发布领域事件（而不是调用其他Command）
        ctx.event_bus.publish(PaymentOrderCreatedEvent {
            order_id: order.id,
            user_id: order.user_id,
            amount: order.amount,
        }).await?;

        Ok(order.id)
    }
}

// 风控服务订阅事件，自动触发下一个Command
impl RiskServiceEventHandler {
    #[event_handler]
    async fn on_payment_order_created(&self, event: PaymentOrderCreatedEvent) {
        self.risk_scan_usecase
            .execute(RiskScanRequest {
                order_id: event.order_id,
            })
            .await
            .ok();
    }
}
```

**模式2：Saga编排（Saga Orchestration）**

```rust
// 流程编排器协调多个独立的Command
pub struct PaymentSagaOrchestrator {
    create_order: Arc<dyn CreatePaymentOrderUseCase>,
    risk_scan: Arc<dyn RiskScanUseCase>,
    fx_conversion: Arc<dyn FxConversionUseCase>,
}

impl PaymentSagaOrchestrator {
    async fn execute_payment_flow(&self, req: PaymentRequest) -> Result<()> {
        // 步骤1：创建订单
        let order_id = self.create_order.execute(req).await?;

        // 步骤2：风控检查
        let risk_result = self.risk_scan.execute(order_id).await?;
        if !risk_result.passed {
            self.compensate_order(order_id).await?; // 补偿事务
            return Err(RiskRejectedError);
        }

        // 步骤3：换汇处理
        self.fx_conversion.execute(order_id).await?;

        Ok(())
    }
}
```

**❌ 反模式：直接调用（禁止）**

```rust
// ❌ 错误：Command内部直接调用其他Command
impl CreatePaymentOrderCommand {
    async fn execute(&self) -> Result<()> {
        let order = Order::new(...);

        // ❌ 禁止：直接调用其他Command
        RiskScanCommand::execute(order.id).await?;
        FxConversionCommand::execute(order.id).await?;
        SubmitPaymentCommand::execute(order.id).await?;

        Ok(())
    }
}
```

**为什么禁止直接调用？**
- ❌ 违反单一职责原则
- ❌ 高耦合，难以测试
- ❌ 无法独立扩展
- ❌ 事务边界不清晰
- ❌ 难以处理异常和重试

#### 3.0.3 L3 vs L4 vs L5 层次对比

```
层次对比理解：

L3 = API调用视图（What can be called）
     展示：客户/运营可调用的HTTP端点
     示例：POST /api/orders → CreatePaymentOrderCommand
     职责：定义API边界和触发点

L4 = Command定义（What it does）
     展示：Command的职责和边界
     示例：CreatePaymentOrderCommand负责创建付款订单
     职责：业务逻辑编排

L5 = 实现步骤（How it's done）
     展示：Command内部执行步骤
     示例：验证→风控扫描→制裁筛查→路由→冻结资金
     职责：具体实现细节（包含系统自动化）

示例：B2B付款流程

L3视图（API端点目录）：
┌────────────────────────────────────────────┐
│ [API 1] POST /api/orders                   │
│         ├→ CreatePaymentOrderCommand       │
│                                            │
│ [API 2] POST /api/fx/lock                  │
│         ├→ LockFxRateCommand               │
│                                            │
│ [API 3] POST /api/orders/{id}/confirm      │
│         ├→ ConfirmFxDealCommand            │
└────────────────────────────────────────────┘

L4定义（Command职责）：
CreatePaymentOrderCommand: 创建付款订单并进行初步验证
LockFxRateCommand: 锁定汇率报价30秒
ConfirmFxDealCommand: 确认换汇成交并提交支付

L5实现（Command内部步骤）：
CreatePaymentOrderCommand内部：
  1. 验证订单参数
  2. 执行风控扫描（自动化）
  3. 制裁名单筛查（自动化）
  4. 通道路由选择（自动化）
  5. 冻结账户资金
  6. 发布OrderCreatedEvent

关键点：风控/路由/出款等"系统自动化"不是独立的L3 API，
       而是嵌入在Command内部的L5步骤！
```

### 3.1 L3 API调用视图标注说明

本文档在L3流程组中采用"API调用视图"格式，清晰展示每个可触发的API端点及其对应的L4 Command。

**标注格式示例**：
```
[API 1] 客户调用 POST /api/orders
                  ├→ CreatePaymentOrderCommand ✅
                  │  L5内部步骤: 验证→风控扫描→制裁筛查→路由→冻结资金
                  │  返回: {order_id, status, quote_id}
                  │  状态: PENDING_PAYMENT / RISK_REJECTED

[Query] 客户查询 GET /api/orders/{id}
                  └→ QueryPaymentStatusQuery

[Callback] 通道回调 POST /webhook/payment-callback
                  ├→ PaymentStatusCallbackCommand ✅
                  │  L5内部步骤: 验签→更新状态→通知
```

**触发者类型标识**：
- `[API N]` = 客户/运营主动调用的HTTP API端点
  - HTTP方法: POST/PUT/DELETE (Command改变状态)
  - 示例: `[API 1] 客户调用 POST /api/orders`

- `[Query]` = 只读查询操作(不改变状态)
  - HTTP方法: GET
  - 示例: `[Query] 客户查询 GET /api/orders/{id}`

- `[Callback]` = 外部系统异步回调
  - 来源: 支付通道/银行/SWIFT网络/卡组织
  - 示例: `[Callback] 通道回调 POST /webhook/payment-callback`

**Command完整度标记**：
- `✅` = Command已在part2文档中定义
- `❌` = Command缺失，需要新增（标记为**缺失**）
- `⚠️` = Command已定义但需要增强功能

**L5内部步骤说明**：
- 展示Command执行时的内部处理流程
- 包含系统自动化步骤（风控/路由/出款等）
- 使用箭头（→）表示步骤顺序
- 这些步骤不是独立的L3 API，而是Command内部实现细节

### 3.2 缺失Command统计

根据L3流程组的标注统计，缺失的Command主要集中在以下领域：

#### 🔴 P0 优先级（核心交易流程）
1. **C2C转账**：`CreateC2CTransferCommand`, `VerifyBeneficiaryCommand`
2. **收款入账**：`ParseSwiftMessageCommand`, `MatchRecipientAccountCommand`
3. **入金/出金**：`InvokeAcquirerCommand`, `InvokePayoutChannelCommand`, `DebitAccountBalanceCommand`

#### 🟠 P1 优先级（业务完整性）
1. **定期付款**：`CreateRecurringPaymentCommand`, `TriggerRecurringPaymentCommand`
2. **付款取消**：`ValidatePaymentStatusCommand`, `UnfreezeBalanceCommand`, `RevokeChannelPaymentCommand`
3. **付款退款**：`VerifyOriginalOrderCommand`, `ApproveRefundCommand`, `ExecuteRefundCommand`, `ReverseAccountingCommand`
4. **退汇处理**：`RequestReturnCommand`, `AnalyzeReturnReasonCommand`, `ExecuteReturnCommand`, `NotifyChannelReturnCommand`
5. **追索处理**：`RequestRecallCommand`, `CollectEvidenceCommand`, `InitiateGpiRecallCommand`

#### 🟡 P2 优先级（功能完整性）
1. **批量付款**（7个Command）：从解析到报告的完整批处理流程
2. **批量换汇**（6个Command）：批量换汇完整流程
3. **代收代付**（6个Command）：整个代收代付场景
4. **头寸对冲**（5个Command）：对冲决策与执行
5. **日终轧平**（6个Command）：日终头寸轧平流程
6. **实时换汇**（3个Command）：聚合报价、加点计算、头寸更新
7. **远期换汇**（5个Command）：远期定价、掉期点、保证金计算

### 3.3 L3流程重构完成度统计

**重构状态总览**：所有67个L3流程已全部从事件驱动序列图转换为API调用视图格式 ✅

| 流程域 | L3流程数 | 重构状态 | 完成时间 | 备注 |
|--------|---------|---------|----------|------|
| L2.1 汇出汇款 | 8 | ✅ 100% | 2025-12-03 | 含B2B/B2C/C2C/批量/定期/取消/退款/入金 |
| L2.2 汇入汇款 | 6 | ✅ 100% | 2025-12-03 | 含收款入账/退汇/追索/代收代付/出金 |
| L2.3 换汇域 | 6 | ✅ 100% | 2025-12-03 | 含实时换汇/远期换汇/批量换汇/头寸对冲/汇率管理 |
| L2.4 清结算域 | 5 | ✅ 100% | 2025-12-03 | 含日终清算/实时清算/对账/差错/资金归集 |
| L2.5 风控合规域 | 6 | ✅ 100% | 2025-12-03 | 含交易风控/AML/制裁筛查/合规报送/人工审核/黑名单 |
| L2.6 争议处理域 | 5 | ✅ 100% | 2025-12-03 | 含拒付/争议仲裁/赔付/举证调单/客户申诉 |
| L2.7 商户管理域 | 5 | ✅ 100% | 2025-12-03 | 含商户入驻/合同/分级/API接入/商户结算 |
| L2.8 账户管理域 | 6 | ✅ 100% | 2025-12-03 | 含客户注册/KYC-KYB/账户开立/钱包/余额/绑卡 |
| L2.9 通道管理域 | 5 | ✅ 100% | 2025-12-03 | 含通道接入/监控/路由/限额/切换 |
| L2.10 费用管理域 | 5 | ✅ 100% | 2025-12-03 | 含费率配置/阶梯定价/分润/成本核算/账单生成 |
| L2.11 资金管理域 | 5 | ✅ 100% | 2025-12-03 | 含备付金监控/资金调拨/流动性预警/头寸/银行对接 |
| L2.12 报表分析域 | 5 | ✅ 100% | 2025-12-03 | 含交易报表/财务报表/监管报表/经营分析/实时大屏 |
| **合计** | **67** | **✅ 100%** | **2025-12-03** | **所有L3流程已完成API调用视图转换** |

**重构成果**：
- ✅ **架构理解纠正**: L3从事件驱动序列图改为API端点目录
- ✅ **触发点清晰化**: 明确区分API调用/Query查询/外部回调
- ✅ **自动化归位**: 系统自动化（风控/路由/出款）移至L5内部步骤
- ✅ **Command映射**: 每个API端点清晰标注对应的Command及完整度
- ✅ **返回值规范**: 标注每个API的返回格式和状态码

**新增L3流程格式特点**：
1. **API中心视图**: 每个L3流程展示为一组可调用的API端点
2. **角色明确**: 清晰标识"客户调用"或"运营调用"
3. **HTTP语义**: 包含完整的HTTP方法和路径
4. **L5步骤透明**: 展示Command内部自动化流程
5. **回调区分**: 外部系统回调单独标注为[Callback]

### 3.4 L3重构工作总结与后续建议

#### 3.4.1 重构工作总结

**已完成工作** ✅：
1. ✅ **L3架构理念纠正**: 从事件驱动序列图改为API调用视图
2. ✅ **67个L3流程重构**: 覆盖12个L2流程域，全部转换为API端点格式
3. ✅ **触发点规范化**: 清晰区分[API]/[Query]/[Callback]三种触发类型
4. ✅ **系统自动化归位**: 风控/路由/出款等移至L5内部步骤
5. ✅ **Command映射完整**: 每个API端点标注对应Command及完整度状态
6. ✅ **文档结构优化**: Section 3架构说明全面更新

**核心成果**：
- **理念统一**: L3 = API endpoint = HTTP call = 一次联机调用
- **职责清晰**: L3定义"谁能调什么", L4定义"做什么", L5定义"怎么做"
- **可读性提升**: API视图比事件驱动序列图更直观易懂
- **实现指导性强**: 开发人员可直接根据L3设计RESTful API

#### 3.4.2 后续工作建议

**短期优先（建议1-2周内完成）**：

1. **补充缺失Command定义** （标记为❌的Command）
   - P0优先级: C2C转账、收款入账、入金出金相关Command
   - P1优先级: 定期付款、取消退款、退汇追索相关Command
   - 参考: Section 3.2的缺失Command统计

2. **增强现有Command功能** （标记为⚠️的Command）
   - 补充详细的L5步骤说明
   - 完善异常处理和补偿逻辑
   - 添加性能指标和SLA要求

3. **同步更新part2文档**
   - 将新增Command添加到L4任务清单
   - 更新触发者分类统计
   - 同步完成度数据

**中期规划（建议1-3个月内完成）**：

1. **设计API技术规范**
   - RESTful API设计规范
   - 请求/响应格式标准
   - 错误码定义
   - 认证授权机制
   - 接口版本管理策略

2. **编写OpenAPI规范**
   - 基于L3 API视图生成Swagger/OpenAPI文档
   - 自动化API文档生成
   - 接口契约测试

3. **实现原型系统**
   - 选择1-2个核心流程（如B2B付款）先行实现
   - 验证L3→L4→L5架构的可行性
   - 收集实施反馈并迭代优化

**长期改进（建议持续进行）**：

1. **性能优化**
   - 识别高频API并进行性能优化
   - 设置合理的SLA指标
   - 监控和告警机制

2. **安全加固**
   - API安全最佳实践
   - 防护OWASP Top 10漏洞
   - 敏感数据加密

3. **文档持续维护**
   - 随业务变化更新L3流程
   - 保持part1和part2文档同步
   - 定期审查架构合理性

详细的Command定义和实现规范，请参考：
- [下篇：L4任务与Command映射](pay-process2-part2.md)
