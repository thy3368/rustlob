# 速率限制和用户限制

## 基于 IP 的限制

REST 请求有每分钟 1200 的聚合权重限制：

- Exchange API 请求：权重 = `1 + floor(batch_length / 40)`
- 重 info 端点（权重 2）：`l2Book`、`allMids`、`clearinghouseState`、`orderStatus`、`spotClearinghouseState`、`exchangeStatus`
- `userRole`：权重 60
- 所有其他 info 请求：权重 20
- 分页端点每返回 20 项增加权重：`recentTrades`、`historicalOrders`、`userFills`、`userFillsByTime`、`fundingHistory` 等
- `candleSnapshot` 每返回 60 项增加权重
- Explorer API 请求：权重 40（`blockList` 有额外的每区块 1 限制）

WebSocket 限制：

- 10 个同时连接
- 每分钟 30 个新连接
- 1000 个总订阅
- 跨用户特定订阅的 10 个唯一用户
- 跨所有连接每分钟 2000 条消息
- 一次 100 个进行中的 post 消息

EVM JSON-RPC：`rpc.hyperliquid.xyz/evm` 每分钟 100 个请求。

## 基于地址的限制

适用于每个用户（子账户单独计算）。

逻辑允许"自地址创建以来每交易 1 USDC 累积 1 个请求"，起始缓冲为 10,000 个请求。受速率限制的地址每 10 秒获得一个请求。取消限制更宽松：`min(limit + 100000, limit * 2)`。

未结订单限制：

- 默认：1000 个订单
- 每 500 万 USDC 交易量 +1，总计最多 5000 个
- 当存在 1000+ 个未结订单时，仅减少和触发订单被拒绝

在拥塞期间，地址的上限为其做市商份额百分比的 2 倍区块空间。

## 批量请求

`n` 个订单的批量计为 IP 限制的 1 个请求，但地址限制的 `n` 个请求。
