# Bridge2

## 一般信息

Hyperliquid 和 Arbitrum 之间的桥接位于 `0x2df1c51e09aecf9cacb7bc98cb1742757f163df7`。桥接代码可在 GitHub 的 Hyperliquid 合约仓库中获取。

## 存款

用户将原生 USDC 发送到桥接，它在不到 1 分钟内记入发送账户。最低存款为 5 USDC；低于此阈值的金额不会被记入并永久丢失。

## 提款

提款只需要用户在 Hyperliquid 上的钱包签名 — 无需 Arbitrum 交易。验证者完全处理 Arbitrum 提款，资金在 3-4 分钟内到达。

提款有效负载使用 EIP-712 签名，包含以下字段：

- `signatureChainId`: 链标识符
- `hyperliquidChain`: 链名称（主网或测试网）
- `destination`: 接收地址
- `amount`: 提款金额
- `time`: 时间戳

签名操作包括提款详情、与时间字段匹配的 nonce 和签名组件（r, s, v）。

## 带 Permit 的存款

桥接支持通过 `batchedDepositWithPermit` 代表其他用户存款。用户使用 EIP-712 类型数据签名包含所有者、花费者、值、nonce 和截止日期的 permit 有效负载。

桥接地址：
- 主网：`0x2df1c51e09aecf9cacb7bc98cb1742757f163df7`
- 测试网：`0x08cfc1B6b2dCF36A1480b99353A354AA8AC56f89`
