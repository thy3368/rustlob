# 资产 ID

资产 ID 是通过操作发送订单和取消的整数表示。更多详情请参见 [exchange 端点文档](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint)。

## 永续

永续端点期望一个整数 `asset`，这是在 `meta` info 响应中找到的代币索引。例如，主网上 `BTC = 0`。

构建者部署的永续使用公式 `100000 + perp_dex_index * 10000 + index_in_meta`。例如，测试网上的 `test:ABC` 有 `perp_dex_index = 1`，`index_in_meta = 0`，结果为 `asset = 110000`。构建者部署的永续始终遵循命名格式 `{dex}:{coin}`。

## 现货

现货端点期望 `10000 + spotInfo["index"]`，其中 `spotInfo` 是 `spotMeta` 中包含所需报价和基础代币的相应对象。例如，提交 `PURR/USDC` 订单时，使用资产 `10000`，因为其现货信息索引是 `0`。

现货 ID 与代币 ID 不同，主网和测试网有不同的资产 ID。以 HYPE 为例：

- 主网代币 ID: 150
- 主网现货 ID: 107
- 测试网代币 ID: 1105
- 测试网现货 ID: 1035

## 结果

结果与现货交易共享大多数实现细节，每个结果方由不同的代币表示。但是，API 表示与现货和永续都不同。

结果资产来自 `outcome` id 加上二进制 `side`，在 `outcomeMeta` info 响应中找到。

对于具有 id `outcome` 和 side `side` 的结果：

```
encoding = 10 * outcome + side
```

只有 side `0` 和 `1` 有效。

示例：

- outcome `1`, side `0` → encoding `10`

相同的 `encoding` 出现在三种表示中：

- 结果现货币：`#<encoding>`
- 结果代币名称：`+<encoding>`
- 结果资产 ID：`100_000_000 + encoding`

示例：

- `#10` = outcome `1`, side `0`
- `+10` = 相应的代币名称
- `100000010` = 资产 ID
