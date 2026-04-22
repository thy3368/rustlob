# Exchange 端点

## 资产

对于永续，使用 `meta` 响应中 `universe` 字段的索引。对于现货资产，使用 `10000 + index`，其中索引来自 `spotMeta.universe`。例如：PURR/USDC 使用资产 `10000`，因为其现货索引是 `0`。

## 子账户和金库

子账户和金库没有私钥。主账户必须签名操作，`vaultAddress` 字段设置为子账户或金库地址。

## 过期时间

某些操作的可选字段 — 时间戳毫秒，在此之后操作被拒绝。用户签名的操作如 Core USDC 转账不支持此功能。因过期 `expiresAfter` 取消的操作消耗正常速率限制的 5 倍。

## 下单

`POST https://api.hyperliquid.xyz/exchange`

限价单使用 TIF（时间生效）：ALO（仅挂单）、IOC（立即成交或取消）或 GTC（成交前有效）。客户订单 ID（cloid）是可选的 — 128 位十六进制字符串。

**Headers:** Content-Type: "application/json"

**Request Body:**
- `action` (required): 订单对象，包含类型、订单数组、分组、可选构建者
- `nonce` (required): 当前时间戳毫秒
- `signature` (required)
- `vaultAddress` (optional): 42 字符十六进制地址
- `expiresAfter` (optional): 时间戳毫秒

**Responses:** 成功返回订单状态（挂单/成交）和 OID，或错误消息。

## 取消订单

`POST https://api.hyperliquid.xyz/exchange`

**Request Body:**
- `action`: 取消类型，包含资产和订单 ID
- `nonce`, `signature`, 可选 `vaultAddress` 和 `expiresAfter`

## 按 CLOID 取消

`POST https://api.hyperliquid.xyz/exchange`

使用客户订单 ID 而不是订单 ID 取消。

## 计划取消（死人开关）

`POST https://api.hyperliquid.xyz/exchange`

计划在未来时间取消所有订单。至少提前 5 秒。每天最多 10 个触发器（在 00:00 UTC 重置）。

## 修改订单

`POST https://api.hyperliquid.xyz/exchange`

按 OID 或 CLOID 修改现有订单，使用新参数。

## 批量修改订单

`POST https://api.hyperliquid.xyz/exchange`

一个请求中批量修改多个订单。

## 更新杠杆

`POST https://api.hyperliquid.xyz/exchange`

更新代币的全仓或逐仓杠杆。

## 更新逐仓保证金

`POST https://api.hyperliquid.xyz/exchange`

向逐仓仓位添加或移除保证金。有替代操作可用于针对特定杠杆而不是 USDC 值。

## Core USDC 转账

`POST https://api.hyperliquid.xyz/exchange`

无需接触 EVM 桥即可将 USD 发送到另一个地址。签名格式对人类可读，适合钱包界面。

## Core 现货转账

`POST https://api.hyperliquid.xyz/exchange`

将现货资产发送到另一个地址。需要 "name:id" 格式的代币。

## 发起提款请求

`POST https://api.hyperliquid.xyz/exchange`

开始提款流程。$1 费用，约 5 分钟完成。

## 现货与永续之间转账

`POST https://api.hyperliquid.xyz/exchange`

在现货和永续钱包之间转移 USDC。

## 发送资产

`POST https://api.hyperliquid.xyz/exchange`

永续 DEX、现货、用户和子账户之间的通用代币转账。默认 USDC 永续 DEX 使用 ""，现货使用 "spot"。

## 带数据发送到 EVM

`POST https://api.hyperliquid.xyz/exchange`

带数据有效负载的 Core 到 EVM 转账。需要支持 `coreReceiveWithData` 接口的链接合约。

## 存入质押

`POST https://api.hyperliquid.xyz/exchange`

将原生代币从现货转移到质押以进行验证者委托。

## 从质押提款

`POST https://api.hyperliquid.xyz/exchange`

将原生代币从质押转移到现货。适用 7 天解除质押队列。

## 委托或解除委托质押

`POST https://api.hyperliquid.xyz/exchange`

向/从验证者委托/解除委托代币。每个验证者 1 天锁定期。

## 存入或从金库提款

`POST https://api.hyperliquid.xyz/exchange`

向金库添加或移除资金。

## 批准 API 钱包

`POST https://api.hyperliquid.xyz/exchange`

批准 API 钱包（代理钱包）。每个账户一个未命名加最多三个命名；每个子账户额外两个。

## 批准构建者费用

`POST https://api.hyperliquid.xyz/exchange`

为构建者设置最高费用率。

## 下 TWAP 订单

`POST https://api.hyperliquid.xyz/exchange`

时间加权平均价格订单，带随机化选项。

## 取消 TWAP 订单

`POST https://api.hyperliquid.xyz/exchange`

按资产和 TWAP ID 取消现有 TWAP。

## 预留额外操作

`POST https://api.hyperliquid.xyz/exchange`

从永续余额中以每个请求 0.0005 USDC 预留额外操作。

## 使待处理 Nonce 无效（noop）

`POST https://api.hyperliquid.xyz/exchange`

无操作，将 nonce 标记为已使用 — 对取消进行中的订单有效。

## 启用 HIP-3 DEX 抽象

`POST https://api.hyperliquid.xyz/exchange`

（已弃用 — 使用 `userSetAbstraction`）自动从验证者 USDC 永续或现货转移抵押品。

## 设置用户抽象

`POST https://api.hyperliquid.xyz/exchange`

设置抽象模式："disabled", "unifiedAccount", 或 "portfolioMargin"。

## 设置用户抽象（代理）

`POST https://api.hyperliquid.xyz/exchange`

代理版本使用缩写代码："i" (disabled), "u" (unifiedAccount), "p" (portfolioMargin)。

## 验证者对无风险利率投票

`POST https://api.hyperliquid.xyz/exchange`

验证者对对齐报价资产的无风险利率投票。

## 领取奖励

`POST https://api.hyperliquid.xyz/exchange`

领取累积奖励。
