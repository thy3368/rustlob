# Rate Limits and User Limits

## IP-Based Limits

REST requests have an aggregated weight limit of 1200/minute:

- Exchange API requests: weight = `1 + floor(batch_length / 40)`
- Heavy info endpoints (weight 2): `l2Book`, `allMids`, `clearinghouseState`, `orderStatus`, `spotClearinghouseState`, `exchangeStatus`
- `userRole`: weight 60
- All other info requests: weight 20
- Paginated endpoints add weight per 20 items returned: `recentTrades`, `historicalOrders`, `userFills`, `userFillsByTime`, `fundingHistory`, and others
- `candleSnapshot` adds weight per 60 items returned
- Explorer API requests: weight 40 (`blockList` has an extra 1-per-block limit)

WebSocket limits:

- 10 simultaneous connections
- 30 new connections/minute
- 1000 total subscriptions
- 10 unique users across user-specific subscriptions
- 2000 messages/minute across all connections
- 100 inflight post messages at once

EVM JSON-RPC: 100 requests/minute at `rpc.hyperliquid.xyz/evm`.

## Address-Based Limits

Applies per user (sub-accounts count separately).

The logic allows "1 request per 1 USDC traded cumulatively since address inception," with a starting buffer of 10,000 requests. Rate-limited addresses get one request every 10 seconds. Cancel limits are more generous: `min(limit + 100000, limit * 2)`.

Open order limits:

- Default: 1000 orders
- +1 per 5M USDC volume, capped at 5000 total
- Reduce-only and trigger orders are rejected when 1000+ open orders exist

During congestion, addresses are capped at 2x their maker share percentage of block space.

## Batched Requests

A batch of `n` orders counts as 1 request for IP limiting, but `n` requests for address-based limiting.
