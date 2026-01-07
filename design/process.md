# 中心化加密交易所CEX流程域设计（事件溯源架构）模块/微服务都是狗屎

## 架构说明

### 核心理念

```
外部角色 → API调用 → 触发任务(Command) → 系统执行业务逻辑 → 生成EntityEvent → 驱动实体状态变化
```

### 层次结构

```
流程域 (Process Domain)
  └── 流程组 (Process Group)
        └── 流程 (Process)
              └── 任务 (Command) - 由客户/运营通过API触发
```

### 角色定义

| 角色类型 | 说明 | 典型权限 |
|---------|------|---------|
| **Trader** | 普通交易用户 | 下单、撤单、查询 |
| **VIPTrader** | VIP交易用户 | 更高限额、优先撮合 |
| **APITrader** | API交易用户 | 程序化交易 |
| **Operator** | 运营人员 | 配置管理、活动管理 |
| **Admin** | 系统管理员 | 系统配置、用户管理 |
| **ComplianceOfficer** | 合规审核员 | KYC审核、AML调查 |
| **RiskManager** | 风控管理员 | 风控规则配置 |
| **CustomerSupport** | 客服人员 | 工单处理 |

---

## 1. 交易域 (Trading Domain)

### 1.1 现货交易流程组 (Spot Trading Process Group)

#### 流程：订单管理 (Order Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **SubmitLimitOrder** | 提交限价单，以指定价格挂单等待成交 | Trader | `POST /api/v1/orders/limit` | symbol, side, price, quantity, timeInForce | `OrderSubmittedEvent`<br>`BalanceFrozenEvent` | Order: Initial→Pending<br>Balance: Available减少, Frozen增加 |
| **SubmitMarketOrder** | 提交市价单，以最优市场价立即成交 | Trader | `POST /api/v1/orders/market` | symbol, side, quantity | `MarketOrderSubmittedEvent`<br>`OrderMatchedEvent`<br>`TradeSettledEvent` | Order: Initial→Filled<br>Balance: 即时结算 |
| **SubmitStopLimitOrder** | 提交止损限价单，触发价到达后转为限价单 | Trader | `POST /api/v1/orders/stop-limit` | symbol, side, stopPrice, price, quantity | `StopOrderSubmittedEvent` | StopOrder: Created→Monitoring |
| **CancelOrder** | 取消指定的挂单 | Trader | `DELETE /api/v1/orders/{orderId}` | orderId | `OrderCancelRequestedEvent`<br>`OrderCancelledEvent`<br>`BalanceUnfrozenEvent` | Order: Any→Cancelled<br>Balance: Frozen减少, Available增加 |
| **CancelAllOrders** | 批量取消所有或指定交易对/方向的挂单 | Trader | `DELETE /api/v1/orders` | symbol?, side? | `BatchCancelRequestedEvent`<br>`OrdersCancelledEvent[]` | 批量Order: Any→Cancelled |
| **ModifyOrder** | 修改订单价格或数量（实际为撤单+重新下单） | Trader | `PUT /api/v1/orders/{orderId}` | orderId, newPrice?, newQuantity? | `OrderModifyRequestedEvent`<br>`OrderCancelledEvent`<br>`OrderSubmittedEvent` | Order: Cancelled→新订单Created |
| **QueryOpenOrders** | 查询当前活跃的挂单列表 | Trader | `GET /api/v1/orders/open` | symbol?, side?, page | - | 查询类不产生事件 |
| **QueryOrderDetail** | 查询指定订单的详细信息 | Trader | `GET /api/v1/orders/{orderId}` | orderId | - | 查询类不产生事件 |
| **QueryOrderHistory** | 查询历史订单记录 | Trader | `GET /api/v1/orders/history` | symbol?, startTime, endTime, page | - | 查询类不产生事件 |
| **QueryTradeHistory** | 查询历史成交记录 | Trader | `GET /api/v1/trades` | symbol?, orderId?, startTime, endTime | - | 查询类不产生事件 |
#### 流程：交易结算 (Trade Settlement)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **SettleTrade** | 成交结算，处理资金和资产的实际交割 | Admin | `POST /api/admin/settlement/trade` | tradeId | `TradeSettledEvent`<br>`BalanceUpdatedEvent` | Trade: Matched→Settled<br>Balance: 更新 |
| **ReconcileBalance** | 余额对账，确保账户余额准确性 | Admin | `POST /api/admin/settlement/reconcile` | accountId, asset | `BalanceReconciledEvent` | Balance: Reconciled |
| **GenerateTradeReport** | 生成交易报告 | Admin | `POST /api/admin/settlement/report` | startTime, endTime, reportType | `TradeReportGeneratedEvent` | Report: Generated |
| **QuerySettlementStatus** | 查询结算状态 | Admin | `GET /api/admin/settlement/status` | tradeId | - | 查询类 |


#### 流程：做市商管理 (Market Maker Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **RegisterMarketMaker** | 注册做市商，授予做市商权限和义务 | Admin | `POST /api/admin/market-makers` | userId, tradingPairs[], obligations | `MarketMakerRegisteredEvent` | MarketMaker: Created |
| **SubmitQuote** | 提交做市商双边报价 | MarketMaker | `POST /api/v1/mm/quotes` | symbol, bidPrice, bidQty, askPrice, askQty | `QuoteSubmittedEvent`<br>`OrderBookUpdatedEvent` | Quote: Active<br>OrderBook: 更新 |
| **CancelQuote** | 取消做市商报价 | MarketMaker | `DELETE /api/v1/mm/quotes/{quoteId}` | quoteId | `QuoteCancelledEvent` | Quote: Cancelled |
| **QueryMMPerformance** | 查询做市商履约表现 | MarketMaker | `GET /api/v1/mm/performance` | period | - | 查询类 |
#### 流程：币种管理 (Asset Listing Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **SubmitListingApplication** | 提交上币申请 | Operator | `POST /api/admin/listing/apply` | assetInfo, blockchain, contract, documentation | `ListingApplicationSubmittedEvent` | Application: Submitted |
| **ReviewListingApplication** | 审核上币申请 | Admin | `POST /api/admin/listing/{id}/review` | applicationId, reviewResult, notes | `ListingApplicationReviewedEvent` | Application: UnderReview |
| **ApproveListingApplication** | 批准上币申请 | Admin | `POST /api/admin/listing/{id}/approve` | applicationId, tradingPairs[], initialLiquidity | `ListingApplicationApprovedEvent` | Application: Approved |
| **RejectListingApplication** | 拒绝上币申请 | Admin | `POST /api/admin/listing/{id}/reject` | applicationId, rejectReasons[] | `ListingApplicationRejectedEvent` | Application: Rejected |
| **ListNewAsset** | 正式上币，开启交易 | Admin | `POST /api/admin/assets/list` | asset, tradingPairs[], launchTime | `AssetListedEvent`<br>`TradingPairCreatedEvent` | Asset: Listed<br>TradingPair: Active |
| **AnnounceListingSchedule** | 公告上币时间表 | Operator | `POST /api/admin/listing/{id}/announce` | applicationId, launchTime, tradingRules | `ListingScheduleAnnouncedEvent` | Announcement: Published |
| **SuspendAssetTrading** | 暂停币种交易 | Admin | `POST /api/admin/assets/{asset}/suspend` | asset, reason | `AssetTradingSuspendedEvent` | Asset: Suspended |
| **ResumeAssetTrading** | 恢复币种交易 | Admin | `POST /api/admin/assets/{asset}/resume` | asset | `AssetTradingResumedEvent` | Asset: Active |
| **DelistAsset** | 下币，停止所有交易对 | Admin | `POST /api/admin/assets/{asset}/delist` | asset, delistTime, withdrawalDeadline | `AssetDelistedEvent`<br>`TradingPairsClosedEvent` | Asset: Delisted<br>TradingPairs: Closed |
| **AnnounceDelisting** | 公告下币通知 | Operator | `POST /api/admin/delisting/{asset}/announce` | asset, delistTime, withdrawalDeadline, reason | `DelistingAnnouncedEvent` | Announcement: Published |
| **QueryListedAssets** | 查询已上线币种列表 | Trader | `GET /api/v1/assets/listed` | status?, page | - | 查询类 |
| **QueryListingApplications** | 查询上币申请列表 | Admin | `GET /api/admin/listing/applications` | status?, page | - | 查询类 |
### 1.2 行情数据流程组 (Market Data Process Group)

#### 流程：行情订阅 (Market Data Subscription)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **SubscribeTicker** | 订阅实时Ticker行情推送 | Trader | `WS /stream/ticker` | symbol | `TickerSubscribedEvent` | Subscription: Active |
| **SubscribeDepth** | 订阅市场深度数据推送 | Trader | `WS /stream/depth` | symbol, levels | `DepthSubscribedEvent` | Subscription: Active |
| **SubscribeTrades** | 订阅逐笔成交推送 | Trader | `WS /stream/trades` | symbol | `TradesSubscribedEvent` | Subscription: Active |
| **SubscribeKline** | 订阅K线数据推送 | Trader | `WS /stream/kline` | symbol, interval | `KlineSubscribedEvent` | Subscription: Active |
| **Unsubscribe** | 取消行情订阅 | Trader | `WS /stream/unsubscribe` | subscriptionId | `UnsubscribedEvent` | Subscription: Closed |
#### 流程：历史数据查询 (Historical Data Query)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryHistoricalKlines** | 查询历史K线数据 | Trader | `GET /api/v1/klines` | symbol, interval, startTime, endTime | - | 查询类 |
| **QueryHistoricalTrades** | 查询历史成交数据 | Trader | `GET /api/v1/historical-trades` | symbol, startTime, endTime | - | 查询类 |
| **Query24hStats** | 查询24小时统计数据（含价格、交易量、涨跌幅等） | Trader | `GET /api/v1/ticker/24hr` | symbol | - | 查询类 |
| **QueryTradingVolume** | 查询指定时间段的交易量统计 | Trader | `GET /api/v1/volume` | symbol, interval, startTime?, endTime? | - | 查询类 |
| **QueryVolumeRanking** | 查询交易量排行榜 | Trader | `GET /api/v1/volume/ranking` | quoteCurrency, limit, period | - | 查询类 |
| **ExportHistoricalData** | 导出历史数据文件 | Trader | `POST /api/v1/export/market-data` | symbol, dataType, dateRange | `ExportRequestedEvent` | ExportJob: Created |
---

## 2. 资金域 (Asset Domain)

### 2.1 账户管理流程组 (Account Management Process Group)

#### 流程：余额查询 (Balance Query)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryAccountBalance** | 查询账户各币种余额 | Trader | `GET /api/v1/account/balance` | accountType? | - | 查询类 |
| **QueryAssetValuation** | 查询资产总估值 | Trader | `GET /api/v1/account/valuation` | quoteCurrency | - | 查询类 |
| **QueryBalanceSnapshot** | 查询指定时间点的资产快照 | Trader | `GET /api/v1/account/snapshot` | timestamp | - | 查询类 |
### 2.2 内部资金流程组 (Internal Fund Process Group)

#### 流程：资金划转 (Asset Transfer)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **TransferSpotToFutures** | 从现货账户划转到合约账户 | Trader | `POST /api/v1/transfer/spot-to-futures` | asset, amount | `TransferRequestedEvent`<br>`BalanceDebitedEvent`<br>`BalanceCreditedEvent`<br>`TransferCompletedEvent` | SpotBalance: 减少<br>FuturesBalance: 增加 |
| **TransferFuturesToSpot** | 从合约账户划转到现货账户 | Trader | `POST /api/v1/transfer/futures-to-spot` | asset, amount | `TransferRequestedEvent`<br>`BalanceDebitedEvent`<br>`BalanceCreditedEvent`<br>`TransferCompletedEvent` | FuturesBalance: 减少<br>SpotBalance: 增加 |
| **TransferToSubAccount** | 主账户划转到子账户 | Trader | `POST /api/v1/transfer/to-sub` | subAccountId, asset, amount | `SubAccountTransferRequestedEvent`<br>`TransferCompletedEvent` | MainAccount: 减少<br>SubAccount: 增加 |
| **QueryTransferHistory** | 查询划转历史记录 | Trader | `GET /api/v1/transfer/history` | startTime, endTime | - | 查询类 |
#### 流程：内部转账 (Internal Transfer)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **InternalTransfer** | 交易所内用户间转账 | Trader | `POST /api/v1/transfer/internal` | toUserId, asset, amount | `InternalTransferRequestedEvent`<br>`BalanceDebitedEvent`<br>`BalanceCreditedEvent`<br>`TransferCompletedEvent` | FromUser: 减少<br>ToUser: 增加 |
### 2.3 充值提现流程组 (Deposit & Withdrawal Process Group)

#### 流程：充值管理 (Deposit Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **GetDepositAddress** | 获取充值地址 | Trader | `GET /api/v1/deposit/address` | asset, network | - | 查询类 |
| **GenerateDepositAddress** | 生成新的充值地址 | Trader | `POST /api/v1/deposit/address` | asset, network | `DepositAddressGeneratedEvent` | DepositAddress: Created |
| **QueryDepositHistory** | 查询充值历史记录 | Trader | `GET /api/v1/deposit/history` | asset?, startTime, endTime | - | 查询类 |
| **QueryDepositStatus** | 查询充值状态和确认数 | Trader | `GET /api/v1/deposit/{depositId}` | depositId | - | 查询类 |
#### 流程：提现管理 (Withdrawal Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **SubmitWithdrawal** | 提交提现申请 | Trader | `POST /api/v1/withdrawal` | asset, address, amount, network, memo? | `WithdrawalRequestedEvent` | Withdrawal: Created |
| **VerifyWithdrawal2FA** | 提现二次验证确认 | Trader | `POST /api/v1/withdrawal/{id}/verify` | withdrawalId, 2faCode | `Withdrawal2FAVerifiedEvent` | Withdrawal: Verified |
| **ApproveWithdrawal** | 人工审核通过提现 | ComplianceOfficer | `POST /api/admin/withdrawal/{id}/approve` | withdrawalId, notes | `WithdrawalApprovedEvent` | Withdrawal: Approved |
| **RejectWithdrawal** | 人工拒绝提现 | ComplianceOfficer | `POST /api/admin/withdrawal/{id}/reject` | withdrawalId, reason | `WithdrawalRejectedEvent` | Withdrawal: Rejected |
| **CancelWithdrawal** | 用户取消提现 | Trader | `DELETE /api/v1/withdrawal/{id}` | withdrawalId | `WithdrawalCancelledEvent` | Withdrawal: Cancelled |
| **QueryWithdrawalHistory** | 查询提现历史记录 | Trader | `GET /api/v1/withdrawal/history` | asset?, startTime, endTime | - | 查询类 |
| **QueryWithdrawalStatus** | 查询提现状态和进度 | Trader | `GET /api/v1/withdrawal/{id}` | withdrawalId | - | 查询类 |


#### 流程：地址管理 (Address Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **AddWithdrawalAddress** | 添加提现地址到白名单 | Trader | `POST /api/v1/address/whitelist` | asset, network, address, label | `WithdrawalAddressAddedEvent` | AddressWhitelist: Added |
| **RemoveWithdrawalAddress** | 从白名单删除提现地址 | Trader | `DELETE /api/v1/address/whitelist/{id}` | addressId | `WithdrawalAddressRemovedEvent` | AddressWhitelist: Removed |
| **QueryAddressWhitelist** | 查询提现地址白名单 | Trader | `GET /api/v1/address/whitelist` | asset? | - | 查询类 |
---

## 3. 合约域 (Derivatives Domain)

### 3.1 永续合约流程组 (Perpetual Contract Process Group)

#### 流程：持仓管理 (Position Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **OpenLongPosition** | 开多仓 | Trader | `POST /api/v1/futures/long` | symbol, quantity, leverage, orderType, price? | `PositionOpenRequestedEvent`<br>`MarginAllocatedEvent`<br>`PositionOpenedEvent` | Position: Created<br>Margin: 冻结 |
| **OpenShortPosition** | 开空仓 | Trader | `POST /api/v1/futures/short` | symbol, quantity, leverage, orderType, price? | `PositionOpenRequestedEvent`<br>`MarginAllocatedEvent`<br>`PositionOpenedEvent` | Position: Created<br>Margin: 冻结 |
| **CloseLongPosition** | 平多仓 | Trader | `POST /api/v1/futures/close-long` | positionId, quantity?, price? | `PositionCloseRequestedEvent`<br>`PositionClosedEvent`<br>`PnLSettledEvent` | Position: Closed<br>Margin: 释放 |
| **CloseShortPosition** | 平空仓 | Trader | `POST /api/v1/futures/close-short` | positionId, quantity?, price? | `PositionCloseRequestedEvent`<br>`PositionClosedEvent`<br>`PnLSettledEvent` | Position: Closed<br>Margin: 释放 |
| **AdjustLeverage** | 调整持仓杠杆倍数 | Trader | `PUT /api/v1/futures/leverage` | positionId, newLeverage | `LeverageAdjustedEvent`<br>`MarginRecalculatedEvent` | Position: Leverage调整<br>Margin: 调整 |
| **AddMargin** | 追加保证金 | Trader | `POST /api/v1/futures/margin/add` | positionId, amount | `MarginAddedEvent` | Position: Margin增加 |
| **ReduceMargin** | 减少保证金 | Trader | `POST /api/v1/futures/margin/reduce` | positionId, amount | `MarginReducedEvent` | Position: Margin减少 |
| **QueryPositions** | 查询当前持仓 | Trader | `GET /api/v1/futures/positions` | symbol? | - | 查询类 |
| **QueryPositionPnL** | 查询持仓盈亏 | Trader | `GET /api/v1/futures/position/{id}/pnl` | positionId | - | 查询类 |
#### 流程：标记价格查询 (Mark Price Query)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryMarkPrice** | 查询当前标记价格 | Trader | `GET /api/v1/futures/mark-price` | symbol | - | 查询类 |
| **QueryIndexPrice** | 查询当前指数价格 | Trader | `GET /api/v1/futures/index-price` | symbol | - | 查询类 |
| **QueryHistoricalMarkPrice** | 查询历史标记价格 | Trader | `GET /api/v1/futures/mark-price/history` | symbol, startTime, endTime | - | 查询类 |
#### 流程：资金费率查询 (Funding Rate Query)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryCurrentFundingRate** | 查询当前资金费率 | Trader | `GET /api/v1/futures/funding-rate` | symbol | - | 查询类 |
| **QueryHistoricalFundingRates** | 查询历史资金费率 | Trader | `GET /api/v1/futures/funding-rate/history` | symbol, startTime, endTime | - | 查询类 |
| **QueryFundingFeeHistory** | 查询资金费用收支记录 | Trader | `GET /api/v1/futures/funding-fee/history` | symbol?, startTime, endTime | - | 查询类 |
### 3.2 交割合约流程组 (Futures Contract Process Group)

#### 流程：合约管理 (Contract Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **ListFuturesContract** | 上线新的交割合约 | Admin | `POST /api/admin/futures/contracts` | symbol, deliveryDate, specifications | `ContractListedEvent` | Contract: Created |
| **EnableContractTrading** | 开启合约交易 | Admin | `POST /api/admin/futures/contracts/{id}/enable` | contractId | `TradingEnabledEvent` | Contract: Active |
| **PauseContractTrading** | 暂停合约交易 | Admin | `POST /api/admin/futures/contracts/{id}/pause` | contractId, reason | `TradingPausedEvent` | Contract: Paused |
| **DelistContract** | 下线交割合约 | Admin | `POST /api/admin/futures/contracts/{id}/delist` | contractId | `ContractDelistedEvent` | Contract: Delisted |
---

## 4. 风控域 (Risk Management Domain)

### 4.1 实时风控流程组 (Real-time Risk Control Process Group)

#### 流程：账户风控管理 (Account Risk Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **FreezeAccount** | 冻结用户账户 | RiskManager | `POST /api/admin/risk/freeze-account` | userId, reason | `AccountFrozenEvent` | Account: Frozen |
| **UnfreezeAccount** | 解冻用户账户 | RiskManager | `POST /api/admin/risk/unfreeze-account` | userId | `AccountUnfrozenEvent` | Account: Active |
| **RestrictTrading** | 限制交易功能 | RiskManager | `POST /api/admin/risk/restrict-trading` | userId, restrictions | `TradingRestrictedEvent` | Account: Restricted |
| **RestrictWithdrawal** | 限制提现功能 | RiskManager | `POST /api/admin/risk/restrict-withdrawal` | userId, maxAmount? | `WithdrawalRestrictedEvent` | Account: WithdrawalRestricted |
| **QueryRiskScore** | 查询用户风险评分 | RiskManager | `GET /api/admin/risk/score/{userId}` | userId | - | 查询类 |
#### 流程：实时监控 (Real-time Monitoring)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **MonitorAnomalyBehavior** | 监控异常行为模式 | RiskManager | `POST /api/admin/risk/monitor/anomaly` | userId, behaviorPattern | `AnomalyDetectedEvent` | RiskAlert: Created |
| **TriggerRiskAlert** | 触发风险告警 | RiskManager | `POST /api/admin/risk/alert/trigger` | userId, alertType, severity | `RiskAlertTriggeredEvent` | Alert: Triggered |
| **ReviewHighRiskAccount** | 审查高风险账户 | RiskManager | `POST /api/admin/risk/review/high-risk` | userId, riskIndicators[] | `HighRiskAccountReviewedEvent` | Account: UnderReview |
| **InvestigateTransaction** | 调查可疑交易 | RiskManager | `POST /api/admin/risk/investigate/tx` | transactionId, suspiciousReasons[] | `TransactionInvestigationStartedEvent` | Investigation: Started |
| **UpdateRiskProfile** | 更新用户风险画像 | RiskManager | `PUT /api/admin/risk/profile/{userId}` | userId, riskFactors | `RiskProfileUpdatedEvent` | RiskProfile: Updated |
| **QueryActiveAlerts** | 查询活跃告警列表 | RiskManager | `GET /api/admin/risk/alerts/active` | severity?, page | - | 查询类 |
#### 流程：熔断管理 (Circuit Breaker Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **TriggerCircuitBreaker** | 触发市场熔断 | RiskManager | `POST /api/admin/risk/circuit-breaker/trigger` | symbol, reason | `CircuitBreakerTriggeredEvent` | Market: Halted |
| **ReleaseCircuitBreaker** | 解除市场熔断 | RiskManager | `POST /api/admin/risk/circuit-breaker/release` | symbol | `CircuitBreakerReleasedEvent` | Market: Resumed |
### 4.2 合规风控流程组 (Compliance Process Group)

#### 流程：KYC认证 (KYC Verification)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **SubmitKYCApplication** | 提交KYC认证申请 | Trader | `POST /api/v1/kyc/apply` | personalInfo, documents[] | `KYCApplicationSubmittedEvent` | KYC: Submitted |
| **UploadIDDocument** | 上传身份证件照片 | Trader | `POST /api/v1/kyc/document` | documentType, imageFile | `DocumentUploadedEvent` | Document: Uploaded |
| **PerformFaceVerification** | 执行人脸识别验证 | Trader | `POST /api/v1/kyc/face-verify` | livePhoto | `FaceVerificationRequestedEvent` | FaceVerification: Pending |
| **ApproveKYC** | 审核通过KYC申请 | ComplianceOfficer | `POST /api/admin/kyc/{id}/approve` | applicationId, verificationLevel | `KYCApprovedEvent` | KYC: Approved |
| **RejectKYC** | 拒绝KYC申请 | ComplianceOfficer | `POST /api/admin/kyc/{id}/reject` | applicationId, reasons[] | `KYCRejectedEvent` | KYC: Rejected |
| **RequestMoreInfo** | 要求补充认证材料 | ComplianceOfficer | `POST /api/admin/kyc/{id}/request-info` | applicationId, requiredInfo[] | `MoreInfoRequestedEvent` | KYC: PendingMoreInfo |
| **QueryKYCStatus** | 查询KYC认证状态 | Trader | `GET /api/v1/kyc/status` | - | - | 查询类 |


#### 流程：AML调查 (AML Investigation)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **CreateAMLCase** | 创建AML调查案例 | ComplianceOfficer | `POST /api/admin/aml/cases` | userId, alertIds[], severity | `AMLCaseCreatedEvent` | AMLCase: Created |
| **UpdateCaseInvestigation** | 更新案例调查进展 | ComplianceOfficer | `PUT /api/admin/aml/cases/{id}` | caseId, findings | `CaseInvestigationUpdatedEvent` | AMLCase: Investigating |
| **CloseAMLCase** | 关闭AML案例 | ComplianceOfficer | `POST /api/admin/aml/cases/{id}/close` | caseId, resolution | `AMLCaseClosedEvent` | AMLCase: Closed |
| **FileSAR** | 提交可疑交易报告 | ComplianceOfficer | `POST /api/admin/aml/sar` | caseId, regulatoryBody, report | `SARFiledEvent` | SAR: Filed |
| **QueryAMLCases** | 查询AML案例列表 | ComplianceOfficer | `GET /api/admin/aml/cases` | status?, severity? | - | 查询类 |
---

## 5. 用户域 (User Domain)

### 5.1 账户安全流程组 (Account Security Process Group)

#### 流程：用户认证 (User Authentication)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **RegisterUser** | 用户注册 | Guest | `POST /api/v1/auth/register` | email, password, inviteCode? | `UserRegisteredEvent` | User: Created |
| **VerifyEmail** | 邮箱验证 | Guest | `POST /api/v1/auth/verify-email` | verificationCode | `EmailVerifiedEvent` | User: EmailVerified |
| **Login** | 用户登录 | Guest | `POST /api/v1/auth/login` | email/phone, password, deviceInfo | `LoginAttemptedEvent`<br>`LoginSucceededEvent` | Session: Created |
| **Verify2FA** | 二次认证验证 | Trader | `POST /api/v1/auth/2fa/verify` | sessionId, code | `2FAVerifiedEvent` | Session: Authenticated |
| **RefreshToken** | 刷新访问令牌 | Trader | `POST /api/v1/auth/refresh` | refreshToken | `TokenRefreshedEvent` | Session: Renewed |
| **Logout** | 用户登出 | Trader | `POST /api/v1/auth/logout` | - | `LogoutEvent` | Session: Terminated |
| **ResetPassword** | 重置密码 | Guest | `POST /api/v1/auth/reset-password` | email, verificationCode, newPassword | `PasswordResetEvent` | User: PasswordChanged |
| **ChangePassword** | 修改密码 | Trader | `POST /api/v1/auth/change-password` | oldPassword, newPassword | `PasswordChangedEvent` | User: PasswordChanged |
#### 流程：安全设置 (Security Settings)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **Enable2FA** | 开启二次认证 | Trader | `POST /api/v1/security/2fa/enable` | - | `2FAEnabledEvent` | User: 2FAEnabled |
| **Disable2FA** | 关闭二次认证 | Trader | `POST /api/v1/security/2fa/disable` | password, 2faCode | `2FADisabledEvent` | User: 2FADisabled |
| **BindPhone** | 绑定手机号 | Trader | `POST /api/v1/security/phone/bind` | phoneNumber, verificationCode | `PhoneBoundEvent` | User: PhoneBound |
| **BindEmail** | 绑定邮箱 | Trader | `POST /api/v1/security/email/bind` | email, verificationCode | `EmailBoundEvent` | User: EmailBound |
| **CreateAPIKey** | 创建API密钥 | Trader | `POST /api/v1/security/api-key` | label, permissions[], ipWhitelist? | `APIKeyCreatedEvent` | APIKey: Created |
| **DeleteAPIKey** | 删除API密钥 | Trader | `DELETE /api/v1/security/api-key/{id}` | apiKeyId, 2faCode | `APIKeyDeletedEvent` | APIKey: Deleted |
| **ModifyAPIPermissions** | 修改API权限 | Trader | `PUT /api/v1/security/api-key/{id}/permissions` | apiKeyId, newPermissions[] | `APIPermissionsModifiedEvent` | APIKey: Updated |
| **QueryAPIKeys** | 查询API密钥列表 | Trader | `GET /api/v1/security/api-keys` | - | - | 查询类 |
#### 流程：安全日志查询 (Security Log Query)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryLoginHistory** | 查询登录历史 | Trader | `GET /api/v1/security/login-history` | startTime, endTime | - | 查询类 |
| **QuerySecurityLog** | 查询安全日志 | Trader | `GET /api/v1/security/log` | startTime, endTime | - | 查询类 |
### 5.2 子账户管理流程组 (Sub-Account Process Group)

#### 流程：子账户管理 (Sub-Account Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **CreateSubAccount** | 创建子账户 | Trader | `POST /api/v1/sub-account` | email, label, permissions[] | `SubAccountCreatedEvent` | SubAccount: Created |
| **DeleteSubAccount** | 删除子账户 | Trader | `DELETE /api/v1/sub-account/{id}` | subAccountId | `SubAccountDeletedEvent` | SubAccount: Deleted |
| **ModifySubAccountPermissions** | 修改子账户权限 | Trader | `PUT /api/v1/sub-account/{id}/permissions` | subAccountId, newPermissions[] | `SubAccountPermissionsModifiedEvent` | SubAccount: Updated |
| **QuerySubAccounts** | 查询子账户列表 | Trader | `GET /api/v1/sub-accounts` | - | - | 查询类 |
| **QuerySubAccountAssets** | 查询子账户资产 | Trader | `GET /api/v1/sub-account/{id}/assets` | subAccountId | - | 查询类 |
---

## 6. 运营域 (Operations Domain)

### 6.1 营销活动流程组 (Marketing Campaign Process Group)

#### 流程：邀请返佣 (Referral Rebate)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **GenerateInviteCode** | 生成邀请码 | Trader | `POST /api/v1/referral/code` | - | `InviteCodeGeneratedEvent` | InviteCode: Created |
| **QueryInviteeList** | 查询被邀请人列表 | Trader | `GET /api/v1/referral/invitees` | - | - | 查询类 |
| **QueryRebateEarnings** | 查询返佣收益 | Trader | `GET /api/v1/referral/earnings` | startTime, endTime | - | 查询类 |
#### 流程：交易大赛 (Trading Competition)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **CreateCompetition** | 创建交易大赛活动 | Operator | `POST /api/admin/competition` | details, rules, rewards | `CompetitionCreatedEvent` | Competition: Created |
| **PublishCompetition** | 发布交易大赛 | Operator | `POST /api/admin/competition/{id}/publish` | competitionId | `CompetitionPublishedEvent` | Competition: Published |
| **RegisterForCompetition** | 报名参加交易大赛 | Trader | `POST /api/v1/competition/{id}/register` | competitionId | `ParticipantRegisteredEvent` | Participant: Registered |
| **QueryCompetitionDetails** | 查询大赛详情 | Trader | `GET /api/v1/competition/{id}` | competitionId | - | 查询类 |
| **QueryLeaderboard** | 查询排行榜 | Trader | `GET /api/v1/competition/{id}/leaderboard` | competitionId | - | 查询类 |
#### 流程：VIP等级管理 (VIP Level Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryVIPBenefits** | 查询VIP等级权益 | Trader | `GET /api/v1/vip/benefits` | - | - | 查询类 |
| **ConfigureVIPRules** | 配置VIP等级规则 | Operator | `POST /api/admin/vip/rules` | levelRules[] | `VIPRulesConfiguredEvent` | VIPConfig: Updated |
### 6.2 公告通知流程组 (Announcement & Notification Process Group)

#### 流程：公告管理 (Announcement Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **PublishAnnouncement** | 发布系统公告 | Operator | `POST /api/admin/announcement` | title, content, category, targetUsers | `AnnouncementPublishedEvent` | Announcement: Published |
| **QueryAnnouncements** | 查询公告列表 | Trader | `GET /api/v1/announcements` | category?, page | - | 查询类 |
#### 流程：消息管理 (Message Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QueryUnreadMessages** | 查询未读消息 | Trader | `GET /api/v1/messages/unread` | - | - | 查询类 |
| **MarkMessageAsRead** | 标记消息已读 | Trader | `POST /api/v1/messages/{id}/read` | messageId | `MessageReadEvent` | Message: Read |
### 6.3 客服支持流程组 (Customer Support Process Group)

#### 流程：工单管理 (Ticket Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **CreateTicket** | 创建客服工单 | Trader | `POST /api/v1/support/ticket` | category, subject, description, attachments | `TicketCreatedEvent` | Ticket: Created |
| **ReplyTicket** | 回复工单 | Trader/CustomerSupport | `POST /api/v1/support/ticket/{id}/reply` | ticketId, message, attachments? | `TicketRepliedEvent` | Ticket: Updated |
| **CloseTicket** | 关闭工单 | Trader/CustomerSupport | `POST /api/v1/support/ticket/{id}/close` | ticketId | `TicketClosedEvent` | Ticket: Closed |
| **QueryTickets** | 查询工单列表 | Trader | `GET /api/v1/support/tickets` | status?, page | - | 查询类 |
| **QueryTicketDetail** | 查询工单详情 | Trader | `GET /api/v1/support/ticket/{id}` | ticketId | - | 查询类 |
| **AssignTicket** | 分配工单给客服 | CustomerSupport | `POST /api/admin/support/ticket/{id}/assign` | ticketId, agentId | `TicketAssignedEvent` | Ticket: Assigned |
---

## 7. 系统域 (System Domain)

### 7.1 系统监控流程组 (System Monitoring Process Group)

#### 流程：告警管理 (Alert Management)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **QuerySystemStatus** | 查询系统状态 | Admin | `GET /api/admin/system/status` | - | - | 查询类 |
| **QueryPerformanceMetrics** | 查询性能指标 | Admin | `GET /api/admin/system/metrics` | service, metric, timeRange | - | 查询类 |
| **AcknowledgeAlert** | 确认告警 | Admin | `POST /api/admin/system/alert/{id}/ack` | alertId | `AlertAcknowledgedEvent` | Alert: Acknowledged |
| **CreateAlertRule** | 创建告警规则 | Admin | `POST /api/admin/system/alert/rule` | condition, threshold, actions | `AlertRuleCreatedEvent` | AlertRule: Created |
### 7.2 系统配置流程组 (System Configuration Process Group)

#### 流程：系统配置 (System Configuration)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **UpdateSystemConfig** | 更新系统配置 | Admin | `PUT /api/admin/system/config` | key, value | `SystemConfigUpdatedEvent` | Config: Updated |
| **QuerySystemConfig** | 查询系统配置 | Admin | `GET /api/admin/system/config` | key? | - | 查询类 |
| **EnableFeatureFlag** | 开启功能开关 | Admin | `POST /api/admin/system/feature/{name}/enable` | featureName | `FeatureFlagEnabledEvent` | Feature: Enabled |
| **DisableFeatureFlag** | 关闭功能开关 | Admin | `POST /api/admin/system/feature/{name}/disable` | featureName | `FeatureFlagDisabledEvent` | Feature: Disabled |
#### 流程：限流配置 (Rate Limit Configuration)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **ConfigureAPIRateLimit** | 配置API接口限流规则 | Admin | `POST /api/admin/system/rate-limit/api` | endpoint, limit, window | `RateLimitConfiguredEvent` | RateLimit: Configured |
| **ConfigureUserRateLimit** | 配置用户级限流规则 | Admin | `POST /api/admin/system/rate-limit/user` | userId, action, limit, window | `UserRateLimitConfiguredEvent` | RateLimit: Configured |
| **ConfigureIPRateLimit** | 配置IP限流规则 | Admin | `POST /api/admin/system/rate-limit/ip` | ipRange, limit, window | `IPRateLimitConfiguredEvent` | RateLimit: Configured |
| **QueryRateLimitConfig** | 查询限流配置 | Admin | `GET /api/admin/system/rate-limit/config` | type, target | - | 查询类 |
#### 流程：熔断配置 (Circuit Breaker Configuration)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **ConfigureServiceCircuitBreaker** | 配置服务熔断规则 | Admin | `POST /api/admin/system/circuit-breaker/service` | service, failureThreshold, timeout, halfOpenRequests | `CircuitBreakerConfiguredEvent` | CircuitBreaker: Configured |
| **ConfigureMarketCircuitBreaker** | 配置市场熔断规则 | Admin | `POST /api/admin/system/circuit-breaker/market` | symbol, priceChangeThreshold, volumeThreshold | `MarketCircuitBreakerConfiguredEvent` | CircuitBreaker: Configured |
| **TestCircuitBreaker** | 测试熔断规则 | Admin | `POST /api/admin/system/circuit-breaker/test` | target, testType | `CircuitBreakerTestedEvent` | Test: Executed |
| **QueryCircuitBreakerStatus** | 查询熔断器状态 | Admin | `GET /api/admin/system/circuit-breaker/status` | service? | - | 查询类 |
#### 流程：降级配置 (Service Degradation Configuration)

| 任务(Command) | 说明 | 触发角色 | API端点 | 请求参数 | 生成的EntityEvent | 驱动的实体状态变化 |
|--------------|------|---------|---------|---------|---------------------|------------------|
| **ConfigureDegradationStrategy** | 配置服务降级策略 | Admin | `POST /api/admin/system/degradation/strategy` | service, trigger, fallbackBehavior | `DegradationStrategyConfiguredEvent` | Strategy: Configured |
| **EnableDegradation** | 启用服务降级 | Admin | `POST /api/admin/system/degradation/{service}/enable` | service, reason | `DegradationEnabledEvent` | Service: Degraded |
| **DisableDegradation** | 禁用服务降级 | Admin | `POST /api/admin/system/degradation/{service}/disable` | service | `DegradationDisabledEvent` | Service: Normal |
| **QueryDegradationStatus** | 查询降级状态 | Admin | `GET /api/admin/system/degradation/status` | service? | - | 查询类 |
---

## EntityEvent 标准格式

### 核心数据结构定义

基于实际实现代码 `lib/core/exchange/lob/src/lob/domain/entity/lob_types.rs`：

```rust
/// 实体事件
/// 遵循事件溯源模式，支持批量记录多条记录的字段变更
#[derive(Debug, Clone)]
pub struct EntityEvent {
    /// 事件序列号（全局唯一）
    pub event_id: u64,
    /// 事务ID（同一事务中的多个事件共享此ID，确保原子性）
    pub transaction_id: u64,
    /// 实体名称（如 "Order", "Trade", "PricePoint"）
    pub entity_name: &'static str,
    /// 操作类型
    pub operation: EventOperation,
    /// 批量记录变更（支持多条记录）
    pub changes: Vec<RecordChange>,
}

/// 事件操作类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EventOperation {
    Create = 1, // 创建
    Update = 2, // 更新
    Delete = 3, // 删除
}

/// 单条记录的变更
#[derive(Debug, Clone)]
pub struct RecordChange {
    /// 记录ID（如order_id, trade_id等）
    pub entity_id: u64,
    /// 该记录的字段变更列表
    pub field_changes: Vec<FieldChange>,
}

/// 字段变更记录 (field_name, old_value, new_value)
#[derive(Debug, Clone)]
pub struct FieldChange {
    pub field_name: &'static str,
    pub old_value: Option<FieldValue>,
    pub new_value: Option<FieldValue>,
}

/// 字段值类型（支持订单簿常用数据类型）
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    U32(u32),
    U64(u64),
    OptionUsize(Option<usize>),
    TraderId(TraderId),
    OrderId(OrderId),
    Quantity(Quantity),
    Side(Side),
}
```

### 使用示例

#### 示例1: 创建订单事件

```rust
// SubmitLimitOrder 命令生成的事件
let order_created_event = EntityEvent::single(
    event_id: 1001,
    transaction_id: 5001,
    entity_name: "Order",
    operation: EventOperation::Create,
    entity_id: order_id.0,
    field_changes: vec![
        FieldChange::created("trader_id", FieldValue::TraderId(trader_id)),
        FieldChange::created("side", FieldValue::Side(Side::Buy)),
        FieldChange::created("price", FieldValue::U32(50000)),
        FieldChange::created("quantity", FieldValue::Quantity(100)),
        FieldChange::created("filled_quantity", FieldValue::Quantity(0)),
    ],
);

// 同一事务中的余额冻结事件
let balance_frozen_event = EntityEvent::single(
    event_id: 1002,
    transaction_id: 5001,  // 相同的transaction_id保证原子性
    entity_name: "Balance",
    operation: EventOperation::Update,
    entity_id: balance_id,
    field_changes: vec![
        FieldChange::updated(
            "available",
            FieldValue::U64(100000),
            FieldValue::U64(50000),
        ),
        FieldChange::updated(
            "frozen",
            FieldValue::U64(0),
            FieldValue::U64(50000),
        ),
    ],
);
```

#### 示例2: 订单成交事件（批量更新）

```rust
// CancelAllOrders 命令生成的批量事件
let batch_cancel_event = EntityEvent::batch(
    event_id: 1003,
    transaction_id: 5002,
    entity_name: "Order",
    operation: EventOperation::Update,
    changes: vec![
        RecordChange::new(
            entity_id: order_id_1,
            field_changes: vec![
                FieldChange::updated(
                    "status",
                    FieldValue::U32(OrderStatus::Pending as u32),
                    FieldValue::U32(OrderStatus::Cancelled as u32),
                ),
            ],
        ),
        RecordChange::new(
            entity_id: order_id_2,
            field_changes: vec![
                FieldChange::updated(
                    "status",
                    FieldValue::U32(OrderStatus::Pending as u32),
                    FieldValue::U32(OrderStatus::Cancelled as u32),
                ),
            ],
        ),
        // ... 更多订单
    ],
);
```

#### 示例3: 订单删除事件

```rust
// CancelOrder 命令生成的删除事件
let order_deleted_event = EntityEvent::single(
    event_id: 1004,
    transaction_id: 5003,
    entity_name: "Order",
    operation: EventOperation::Delete,
    entity_id: order_id.0,
    field_changes: vec![
        FieldChange::deleted("trader_id", FieldValue::TraderId(trader_id)),
        FieldChange::deleted("side", FieldValue::Side(Side::Buy)),
        FieldChange::deleted("price", FieldValue::U32(50000)),
        FieldChange::deleted("quantity", FieldValue::Quantity(100)),
    ],
);
```

### 设计特点

1. **批量支持**: `changes: Vec<RecordChange>` 支持单个事件记录多条记录的变更
2. **事务原子性**: `transaction_id` 确保同一事务中的多个事件要么全部成功，要么全部失败
3. **字段级追踪**: `FieldChange` 记录 `old_value` 和 `new_value`，支持完整的状态回溯
4. **类型安全**: `FieldValue` 枚举提供类型安全的值存储
5. **零拷贝**: `entity_name` 使用 `&'static str` 避免运行时分配
6. **高性能**: 所有类型都实现了 `Clone`，支持事件流式处理

### 与设计文档中事件名称的映射

| 设计文档事件名 | EntityEvent 表示 | 说明 |
|--------------|-----------------|------|
| `OrderSubmittedEvent` | `entity_name: "Order"`, `operation: Create` | 创建订单 |
| `OrderCancelledEvent` | `entity_name: "Order"`, `operation: Update` + status字段更新 | 订单状态变更 |
| `OrderMatchedEvent` | `entity_name: "Trade"`, `operation: Create` | 创建成交记录 |
| `BalanceFrozenEvent` | `entity_name: "Balance"`, `operation: Update` + frozen字段增加 | 余额冻结 |
| `BalanceUnfrozenEvent` | `entity_name: "Balance"`, `operation: Update` + frozen字段减少 | 余额解冻 |

**注意**: 设计文档中的具体事件名称（如 `OrderSubmittedEvent`）是领域层的业务事件，而 `EntityEvent` 是持久化层的通用事件结构。两者通过 `entity_name` + `operation` + `field_changes` 的组合来表达业务语义。

### 聚合根类型清单

| 聚合根类型 | 描述 | 主要事件 |
|-----------|------|---------|
| **Order** | 订单 | OrderSubmittedEvent, OrderMatchedEvent, OrderCancelledEvent |
| **Balance** | 账户余额 | BalanceFrozenEvent, BalanceUnfrozenEvent, BalanceCreditedEvent, BalanceDebitedEvent |
| **Position** | 合约持仓 | PositionOpenedEvent, PositionClosedEvent, LiquidationTriggeredEvent |
| **Withdrawal** | 提现 | WithdrawalRequestedEvent, WithdrawalApprovedEvent, WithdrawalRejectedEvent, WithdrawalCompletedEvent |
| **Deposit** | 充值 | DepositDetectedEvent, DepositConfirmedEvent, DepositCreditedEvent |
| **Trade** | 成交 | TradeExecutedEvent, TradeSettledEvent |
| **Transfer** | 资金划转 | TransferRequestedEvent, TransferCompletedEvent |
| **User** | 用户 | UserRegisteredEvent, EmailVerifiedEvent, KYCApprovedEvent |
| **KYCApplication** | KYC申请 | KYCApplicationSubmittedEvent, KYCApprovedEvent, KYCRejectedEvent |
| **AMLCase** | AML案例 | AMLCaseCreatedEvent, CaseInvestigationUpdatedEvent, AMLCaseClosedEvent |
| **Session** | 用户会话 | LoginSucceededEvent, LogoutEvent, TokenRefreshedEvent |
| **APIKey** | API密钥 | APIKeyCreatedEvent, APIKeyDeletedEvent, APIPermissionsModifiedEvent |
| **Campaign** | 营销活动 | CampaignCreatedEvent, ParticipantRegisteredEvent, RewardsDistributedEvent |
| **Ticket** | 客服工单 | TicketCreatedEvent, TicketRepliedEvent, TicketClosedEvent |

---



## 性能要求

### 时延要求（按流程域）

| 流程域 | Command执行 | 事件持久化 | 投影更新 | 端到端 |
|-------|-----------|-----------|---------|-------|
| **交易域** | < 50μs | < 50μs | < 100μs | < 200μs |
| **资金域** | < 100μs | < 100μs | < 200μs | < 500μs |
| **合约域** | < 100μs | < 200μs | < 500μs | < 1ms |
| **风控域** | < 50μs | < 100μs | < 200μs | < 500μs |
| **用户域** | < 10ms | < 20ms | < 50ms | < 100ms |
| **运营域** | < 50ms | < 100ms | < 500ms | < 1s |

### 吞吐量要求

| 流程组 | 峰值TPS | 日均TPS |
|-------|---------|---------|
| 订单管理 | 500K+ | 100K+ |
| 账户查询 | 1M+ | 200K+ |
| 行情推送 | 10M+ | 1M+ |
| 充值提现 | 10K+ | 2K+ |
| KYC审核 | 1K+ | 100+ |

---

## 参考架构

### 币安技术栈
- **撮合引擎**: 自研高性能C++引擎（< 5μs延迟）
- **事件存储**: Kafka + 自研时序数据库
- **缓存**: Redis Cluster
- **数据库**: PostgreSQL + Cassandra
- **API网关**: Kong + Nginx

### OKX技术栈
- **合约引擎**: Rust实现
- **事件总线**: Apache Pulsar
- **状态存储**: TiKV
- **流计算**: Apache Flink
- **监控**: Prometheus + Grafana

---

**文档版本**: v1.1 (优化版)
**最后更新**: 2025-01-04
**维护者**: 系统架构组

**本次更新内容**:
- 拆分资金域流程组为3个独立流程组
- 补充风控实时监控流程(6个任务)
- 补充交易结算流程(4个任务)
- 拆分系统配置为限流/熔断/降级3个流程(12个任务)
- 新增币种管理流程(12个任务，含上币/下币完整流程)
- 流程数从31个增加到36个，任务数从135个增加到167个
