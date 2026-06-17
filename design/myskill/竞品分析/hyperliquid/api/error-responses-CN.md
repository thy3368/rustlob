# 错误响应

订单和取消错误通常作为与批量请求长度匹配的向量返回。

## 批量错误响应

| 错误来源 | 错误类型 | 错误字符串 |
|---|---|---|
| Order | Tick | Price must be divisible by tick size. |
| Order | MinTradeNtl | Order must have minimum value of $10. |
| Order | MinTradeSpotNtl | Order must have minimum value of 10 {quote_token}. |
| Order | PerpMargin | Insufficient margin to place order. |
| Order | ReduceOnly | Reduce only order would increase position. |
| Order | BadAloPx | Post only order would have immediately matched, bbo was {bbo}. |
| Order | IocCancel | Order could not immediately match against any resting orders. |
| Order | BadTriggerPx | Invalid TP/SL price. |
| Order | MarketOrderNoLiquidity | No liquidity available for market order. |
| Order | PositionIncreaseAtOpenInterestCap | Order would increase open interest while capped. |
| Order | PositionFlipAtOpenInterestCap | Order would increase open interest while capped. |
| Order | TooAggressiveAtOpenInterestCap | Order rejected due to price more aggressive than oracle at cap. |
| Order | OpenInterestIncrease | Order would increase open interest too quickly. |
| Order | InsufficientSpotBalance | Order has insufficient spot balance to trade (spot-only). |
| Order | Oracle | Order price too far from oracle. |
| Order | PerpMaxPosition | Order would exceed margin tier limit at current leverage. |
| Cancel | MissingOrder | Order was never placed, already canceled, or filled. |

## 预验证错误

某些错误源于有效负载本身，在预验证期间作为整个批处理的单个错误返回。这些包括空订单批次、非仅减少的 TP/SL 订单和某些 tick 大小验证。

实现批处理的 API 用户应处理"为多个订单的批处理返回单个错误"的场景。响应可能在回调执行之前跨所有订单重复。

有关历史订单状态，请参见[订单状态查询文档](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint#query-order-status-by-oid-or-cloid)。
