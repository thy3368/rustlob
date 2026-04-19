# Error responses

Order and cancel errors are typically returned as a vector matching the length of the batched request.

## Batched error responses

| Error source | Error type | Error string |
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

## Pre-validation errors

Some errors stem from the payload itself and are returned during pre-validation as a single error for the entire batch. These include empty order batches, non-reduce-only TP/SL orders, and certain tick size validations.

API users implementing batching should handle scenarios where "a single error is returned for a batch of multiple orders." The response may be duplicated across all orders before callback execution.

For historical order statuses, consult the [order status query documentation](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint#query-order-status-by-oid-or-cloid).
