# OKX Exchange API Design Document

## Overview

OKX (欧易) is a cryptocurrency exchange offering comprehensive trading APIs under the **REST API v5** version. This document provides detailed API specifications for integration with the RustLOB trading system.

## API Base URLs

| Environment | REST API | WebSocket |
|-------------|----------|-----------|
| Production | `https://www.okx.com` | `wss://ws.okx.com:8443` |
| Demo/Test | `https://www.okx.com` | `wss://wspap.okx.com:8443` |

## API Categories

OKX REST API v5 is organized into the following functional categories:

| Category | Prefix | Description |
|----------|--------|-------------|
| Public Data | `/api/v5/public` | Market instruments, indices, system time |
| Market Data | `/api/v5/market` | Tickers, order books, klines, trades |
| Trading | `/api/v5/trade` | Orders, algo orders, account operations |
| Account | `/api/v5/account` | Balance, positions, settings |
| Funding | `/api/v5/asset` | Deposits, withdrawals, transfers |
| Sub-account | `/api/v5/users` | Sub-account management |

## Authentication

### REST API Authentication

OKX uses **HMAC-SHA256** signature authentication for private endpoints.

#### Authentication Headers

| Header | Description |
|--------|-------------|
| `OK-ACCESS-KEY` | API Key |
| `OK-ACCESS-SIGN` | Base64-encoded HMAC-SHA256 signature |
| `OK-ACCESS-TIMESTAMP` | Timestamp (Unix epoch in seconds) |
| `OK-ACCESS-PASSPHRASE` | API passphrase |

#### Signature Generation

```
Signature = Base64(HMAC-SHA256(timestamp + method + requestPath + body, secretKey))
```

#### Request Example

```bash
# Headers
OK-ACCESS-KEY: your_api_key
OK-ACCESS-SIGN: your_generated_signature
OK-ACCESS-TIMESTAMP: 1597026383085
OK-ACCESS-PASSPHRASE: your_passphrase
Content-Type: application/json
```

### WebSocket Authentication

WebSocket private channels require login with the following message:

```json
{
  "op": "login",
  "args": [
    {
      "apiKey": "your_api_key",
      "passphrase": "your_passphrase",
      "timestamp": "1597026383",
      "sign": "your_signature"
    }
  ]
}
```

## Rate Limits

### REST API Rate Limits

| Endpoint Category | Limit |
|-------------------|-------|
| Public Data | 20 requests/2s (IP-based) |
| Market Data | 20 requests/2s (IP-based) |
| Trading (Place Order) | 60 requests/2s (User + Instrument) |
| Trading (Cancel/Amend) | 60 requests/2s (User + Instrument) |
| Account | 20 requests/2s |
| Rate Limit Info | 1 request/1s |

### Account Rate Limits

- **VIP 5+ users**: Custom rate limits based on fill ratio
- **Standard users**: 2000 orders/2s (sub-account)

### Rate Limit Headers

```
X-RateLimit-Limit: 2000
X-RateLimit-Remaining: 1999
X-RateLimit-Reset: 1597026383085
```

## Common Response Format

All API responses follow a consistent structure:

```json
{
  "code": "0",
  "msg": "",
  "data": [...],
  "inTime": "1597026383085123",
  "outTime": "1597026383085456"
}
```

| Field | Type | Description |
|-------|------|-------------|
| code | String | Response code. `0` = success |
| msg | String | Error message (empty if success) |
| data | Array | Response data array |
| inTime | String | Request receipt timestamp (microseconds) |
| outTime | String | Response sent timestamp (microseconds) |

## Instrument Types

OKX supports multiple instrument types:

| Type | Code | Description |
|------|------|-------------|
| Spot | `SPOT` | Spot trading |
| Margin | `MARGIN` | Margin trading |
| Futures | `FUTURES` | Delivery futures |
| Perpetual Swap | `SWAP` | USDT/USD inverse perpetual |
| Options | `OPTION` | Options contracts |

## Trade Modes

| Mode | Code | Description |
|------|------|-------------|
| Cash | `cash` | Non-margin spot trading |
| Cross Margin | `cross` | Cross margin mode |
| Isolated Margin | `isolated` | Isolated margin mode |
| Spot Isolated | `spot_isolated` | Spot lead trading |

## Order Types

| Type | Code | Description |
|------|------|-------------|
| Market | `market` | Market order |
| Limit | `limit` | Limit order |
| Post Only | `post_only` | Maker only |
| FOK | `fok` | Fill or Kill |
| IOC | `ioc` | Immediate or Cancel |
| Optimal Limit IOC | `optimal_limit_ioc` | Market limit IOC |
| MMP | `mmp` | Market Maker Protection |
| Conditional | `conditional` | Conditional order |
| Trigger | `trigger` | Trigger order |
| TWAP | `twap` | Time-Weighted Average Price |
| Trailing Stop | `move_order_stop` | Trailing stop |

## Error Codes

### Common Error Codes

| Code | Description |
|------|-------------|
| `0` | Success |
| `1` | General error |
| `50001` | Missing parameter |
| `50002` | Invalid parameter |
| `50003` | Invalid request |
| `50101` | Authentication failed |
| `50102` | API key not found |
| `50103` | Signature mismatch |
| `50104` | Invalid timestamp |
| `50105` | Invalid passphrase |
| `51101` | Insufficient balance |
| `51102` | Order too large |
| `51103` | Price too high/low |
| `51201` | Order not found |
| `51202` | Cannot amend |
| `51203` | Cannot cancel |

## Base URLs by Service

### REST API

```
https://www.okx.com/api/v5
```

### WebSocket

```
# Public channels
wss://ws.okx.com:8443/ws/v5/public

# Private channels  
wss://ws.okx.com:8443/ws/v5/private
```

## References

- [Official API Documentation](https://www.okx.com/docs-v5/en)
- [API Console](https://www.okx.com/docs-v5/en/index)
