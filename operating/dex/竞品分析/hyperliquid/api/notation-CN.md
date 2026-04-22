# 符号

当前 v0 API 使用非标准符号，将在破坏性 v1 API 更改中标准化。

| 缩写 | 全名 | 说明 |
|---|---|---|
| Px | Price | — |
| Sz | Size | 以代币单位计，即基础货币 |
| Szi | Signed size | 多头为正，空头为负 |
| Ntl | Notional | USD 金额，Px * Sz |
| Side | 交易或订单簿方向 | B = Bid = 买入，A = Ask = 卖出。Side 对于交易是主动方。 |
| Asset | 资产 | 表示交易资产的整数。见下文说明 |
| Tif | Time in force | GTC = 成交前有效，ALO = 仅添加流动性（仅挂单），IOC = 立即成交或取消 |
