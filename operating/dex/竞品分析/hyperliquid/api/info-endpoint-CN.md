# Info 端点

## 分页

带时间范围的响应最多返回 500 个元素或数据块。对于更大范围，使用最后返回的时间戳作为下一个 `startTime` 参数。

## 永续 vs 现货

这些端点适用于永续和现货市场。对于永续，`coin` 是 `meta` 响应中的名称。对于现货，PURR 使用 `PURR/USDC`，其他代币使用 `@{index}` 格式（例如主网上 HYPE 使用 `@107`）。某些资产可能在用户界面上重新映射 — 在代币详情页面上检查 L1 名称以检测重新映射。

## 用户地址

通过传递主账户或子账户的实际地址来查询账户数据。使用代理钱包的地址将返回空结果。

## 检索所有代币的中间价

**POST** `https://api.hyperliquid.xyz/info`

如果订单簿为空，则最后交易价格作为回退。

**Headers:** Content-Type: "application/json"

**Request Body:**
- `type` (required): "allMids"
- `dex` (optional): 永续 dex 名称；默认为第一个永续 dex。仅在第一个永续 dex 时包含现货中间价。

**Response:** 对象，以代币符号为键，中间价为值。

## 检索用户的未结订单

**POST** `https://api.hyperliquid.xyz/info`

**Headers:** Content-Type: "application/json"

**Request Body:**
- `type` (required): "openOrders"
- `user` (required): 42 字符十六进制地址
- `dex` (optional): 永续 dex 名称；默认为第一个永续 dex

**Response:** 订单对象数组，包含字段：`coin`, `limitPx`, `oid`, `side`, `sz`, `timestamp`

## 检索带前端信息的用户未结订单

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "frontendOpenOrders"
- `user` (required): 42 字符十六进制地址
- `dex` (optional): 永续 dex 名称

**Response:** 数组包含额外字段：`isPositionTpsl`, `isTrigger`, `orderType`, `origSz`, `reduceOnly`, `triggerCondition`, `triggerPx`

## 检索用户的成交

**POST** `https://api.hyperliquid.xyz/info`

最多返回 2000 个最近的成交。

**Request Body:**
- `type` (required): "userFills"
- `user` (required): 42 字符十六进制地址
- `aggregateByTime` (optional): 当交叉订单匹配多个挂单时合并部分成交

**Response:** 成交对象数组，包括：`closedPnl`, `coin`, `crossed`, `dir`, `hash`, `oid`, `px`, `side`, `startPosition`, `sz`, `time`, `fee`, `feeToken`, `tid`

## 按时间检索用户的成交

**POST** `https://api.hyperliquid.xyz/info`

每次响应最多返回 2000 个成交；仅最近 10,000 个成交可用。

**Request Body:**
- `type` (required): "userFillsByTime"
- `user` (required): 42 字符十六进制地址
- `startTime` (required): 开始时间毫秒（包含）
- `endTime` (optional): 结束时间毫秒（包含）；默认为当前时间
- `aggregateByTime` (optional): 合并部分成交

## 查询用户速率限制

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type`: "userRateLimit"
- `user`: 42 字符十六进制地址

**Response:** 对象包含 `cumVlm`, `nRequestsUsed`, `nRequestsCap`, `nRequestsSurplus`

## 按 OID 或 CLOID 查询订单状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "orderStatus"
- `user` (required): 42 字符十六进制地址
- `oid` (required): u64 订单 ID 或 16 字节十六进制字符串（客户订单 ID）

**订单状态值：**
- `open`: 成功下单
- `filled`: 已成交
- `canceled`: 用户取消
- `triggered`: 触发订单已触发
- `rejected`: 下单时被拒绝
- `marginCanceled`: 保证金不足
- `vaultWithdrawalCanceled`: 金库提款取消
- `openInterestCapCanceled`: 在持仓上限时过于激进
- `selfTradeCanceled`: 自成交预防
- `reduceOnlyCanceled`: 不减少仓位
- `siblingFilledCanceled`: TP/SL 兄弟订单已成交
- `delistedCanceled`: 资产下架
- `liquidatedCanceled`: 清算
- `scheduledCancel`: 超过期限
- `tickRejected`: 无效 tick 价格
- `minTradeNtlRejected`: 低于最低名义价值
- `perpMarginRejected`: 保证金不足
- `reduceOnlyRejected`: 仅减少违规
- `badAloPxRejected`: 仅挂单单立即匹配
- `iocCancelRejected`: IOC 未匹配
- `badTriggerPxRejected`: 无效 TP/SL 价格
- `marketOrderNoLiquidityRejected`: 流动性不足
- `positionIncreaseAtOpenInterestCapRejected`: 在持仓上限
- `positionFlipAtOpenInterestCapRejected`: 在持仓上限
- `tooAggressiveAtOpenInterestCapRejected`: 价格过于激进
- `openInterestIncreaseRejected`: 在持仓上限
- `insufficientSpotBalanceRejected`: 余额不足
- `oracleRejected`: 价格离预言机太远
- `perpMaxPositionRejected`: 超过保证金等级限制

## L2 订单簿快照

**POST** `https://api.hyperliquid.xyz/info`

每侧最多返回 20 个档位。

**Request Body:**
- `type` (required): "l2Book"
- `coin` (required): 资产符号
- `nSigFigs` (optional): 聚合到有效数字（2, 3, 4, 5，或 null 表示完全精度）
- `mantissa` (optional): 仅在 nSigFigs=5 时；接受 1, 2, 或 5

**Response:** 对象包含 `coin`, `time`, 和 `levels` 数组（买/卖侧，含 `px`, `sz`, `n`）

## K 线快照

**POST** `https://api.hyperliquid.xyz/info`

仅最近 5000 根 K 线可用。

**支持的周期：** 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 8h, 12h, 1d, 3d, 1w, 1M

**Request Body:**
- `type` (required): "candleSnapshot"
- `req` (required): 对象包含 `coin`, `interval`, `startTime`, `endTime`（均为毫秒）

**Response:** K 线对象数组，包含：`T`（收盘时间）, `c`（收盘）, `h`（最高）, `i`（周期）, `l`（最低）, `n`（成交数）, `o`（开盘）, `s`（符号）, `t`（开盘时间）, `v`（成交量）

## 检查 Builder 费用批准

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "maxBuilderFee"
- `user` (required): 42 字符十六进制地址
- `builder` (required): 42 字符十六进制地址

**Response:** 以十分之一基点为单位的最大批准费用（1 = 0.001%）

## 检索用户的历史订单

**POST** `https://api.hyperliquid.xyz/info`

最多返回 2000 个最近的历史订单。

**Request Body:**
- `type` (required): "historicalOrders"
- `user` (required): 42 字符十六进制地址

**Response:** 订单对象数组，包含 `order`, `status`, `statusTimestamp`

## 检索用户的 TWAP 切片成交

**POST** `https://api.hyperliquid.xyz/info`

最多返回 2000 个最近的 TWAP 切片成交。

**Request Body:**
- `type` (required): "userTwapSliceFills"
- `user` (required): 42 字符十六进制地址

**Response:** 数组包含 `fill` 对象和 `twapId`

## 检索用户的子账户

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "subAccounts"
- `user` (required): 42 字符十六进制地址

**Response:** 子账户对象数组，包含名称、地址、主账户和账户状态

## 检索金库详情

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "vaultDetails"
- `vaultAddress` (required): 42 字符十六进制地址
- `user` (optional): 42 字符十六进制地址

**Response:** 金库对象，包含名称、领导者、描述、组合历史、APR、追随者和关系数据

## 检索用户的金库存款

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userVaultEquities"
- `user` (required): 42 字符十六进制地址

**Response:** 金库权益对象数组，包含 `vaultAddress` 和 `equity`

## 查询用户角色

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userRole"
- `user` (required): 42 字符十六进制地址

**Response:** 角色对象 — 可能值："missing", "user", "agent", "vault", 或 "subAccount"

## 查询用户组合

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "portfolio"
- `user` (required): 42 字符十六进制地址

**Response:** 数组包含时间段（day, week, month, allTime, perpDay, perpWeek, perpMonth, perpAllTime），包含账户价值历史、盈亏历史和交易量

## 查询用户推荐信息

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "referral"
- `user` (required): 42 字符十六进制地址

**Response:** 对象包含推荐人信息、累积交易量、已认领/未认领奖励、构建者奖励和推荐人状态

## 查询用户费用

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userFees"
- `user` (required): 42 字符十六进制地址

**Response:** 对象包含每日交易量、费用表、用户费率、推荐折扣、试用信息和质押折扣

## 查询用户质押委托

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegations"
- `user` (required): 42 字符十六进制地址

**Response:** 委托对象数组，包含验证者、金额和锁定时间戳

## 查询用户质押摘要

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegatorSummary"
- `user` (required): 42 字符十六进制地址

**Response:** 对象包含已委托、已解除委托、总待处理提款和待处理提款计数

## 查询用户质押历史

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegatorHistory"
- `user` (required): 42 字符十六进制地址

**Response:** 历史对象数组，包含时间、哈希和 delta（委托/解除委托信息）

## 查询用户质押奖励

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegatorRewards"
- `user` (required): 42 字符十六进制地址

**Response:** 奖励对象数组，包含时间、来源（委托/佣金）和总金额

## 查询用户 HIP-3 DEX 抽象状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userDexAbstraction"
- `user` (required): 42 字符十六进制地址

**Response:** 布尔值

## 查询用户抽象状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userAbstraction"
- `user` (required): 42 字符十六进制地址

**Response:** 字符串 — 可能值："unifiedAccount", "portfolioMargin", "disabled", "default", "dexAbstraction"

## 查询对齐报价代币状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "alignedQuoteTokenInfo"
- `token` (required): 代币索引

**Response:** 对象包含对齐状态、首次对齐时间、EVM 铸造供应量、每日应付金额和预测费率

## 查询借/贷用户状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "borrowLendUserState"
- `user` (required): 42 字符十六进制地址

**Response:** 对象包含代币状态（借/供基础和值）、健康状态和健康因子

## 查询借/贷储备状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "borrowLendReserveState"
- `token` (required): 代币索引

**Response:** 对象包含借/供年利率、余额、利用率、预言机价格、LTV、总供应和总借入

## 查询所有借/贷储备状态

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "allBorrowLendReserveStates"

**Response:** 代币索引和储备状态对数组

## 查询用户的批准构建者

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "approvedBuilders"
- `user` (required): 42 字符十六进制地址

**Response:** 构建者地址数组
