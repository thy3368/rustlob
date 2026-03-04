# OKX WebSocket API

## Overview

OKX WebSocket API provides real-time data streaming for market data and private account updates. WebSocket connections are bidirectional and support both public (unauthenticated) and private (authenticated) channels.

**WebSocket Endpoints**:

| Environment | Public Channels | Private Channels |
|-------------|-----------------|------------------|
| Production | `wss://ws.okx.com:8443/ws/v5/public` | `wss://ws.okx.com:8443/ws/v5/private` |
| Demo | `wss://wspap.okx.com:8443/ws/v5/public` | `wss://wspap.okx.com:8443/ws/v5/private` |

---

## Connection Management

### Establishing Connection

```javascript
// Public channel
const ws = new WebSocket('wss://ws.okx.com:8443/ws/v5/public');

// Private channel
const ws = new WebSocket('wss://ws.okx.com:8443/ws/v5/private');
```

### Connection Login (Private Channels)

After establishing connection, private channels require login:

```json
{
  "op": "login",
  "args": [
    {
      "apiKey": "your_api_key",
      "passphrase": "your_passphrase",
      "timestamp": "1597026383",
      "sign": "your_generated_signature"
    }
  ]
}
```

#### Signature Generation for WebSocket

```
timestamp = Unix timestamp in seconds
sign = Base64(HMAC-SHA256(timestamp + 'GET' + '/users/self/verify', secretKey))
```

---

## Subscription Operations

### Subscribe

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "tickers",
      "instId": "BTC-USDT"
    }
  ]
}
```

### Unsubscribe

```json
{
  "op": "unsubscribe",
  "args": [
    {
      "channel": "tickers",
      "instId": "BTC-USDT"
    }
  ]
}
```

### Response Format

**Success Response**:
```json
{
  "id": "1512",
  "event": "subscribe",
  "arg": {
    "channel": "tickers",
    "instId": "BTC-USDT"
  },
  "connId": "accb8e21"
}
```

**Error Response**:
```json
{
  "id": "1512",
  "event": "error",
  "code": "60012",
  "msg": "Invalid request",
  "connId": "accb8e21"
}
```

---

## Public Channels

Public channels do not require authentication.

### Tickers Channel

Subscribe to ticker updates for instruments.

#### Channel Format

```json
{
  "channel": "tickers",
  "instId": "BTC-USDT"
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "tickers",
    "instId": "BTC-USDT"
  },
  "data": [
    {
      "instId": "BTC-USDT",
      "last": "30000.0",
      "lastSz": "0.1",
      "askPx": "30000.5",
      "askSz": "5.2",
      "bidPx": "29999.5",
      "bidSz": "3.1",
      "open24h": "29000.0",
      "high24h": "30500.0",
      "low24h": "28500.0",
      "volCcy24h": "123456789.12",
      "vol24h": "4234567.89",
      "ts": "1678886400000",
      "sodUtc0": "28000.0",
      "sodUtc8": "28500.0"
    }
  ]
}
```

---

### K-Line/Candlestick Channel

Subscribe to candlestick updates.

#### Channel Format

```json
{
  "channel": "candle{period}",
  "instId": "BTC-USDT",
  "bar": "1m"
}
```

#### Supported Periods

| Period | Description |
|--------|-------------|
| `candle1m` | 1 minute |
| `candle3m` | 3 minutes |
| `candle5m` | 5 minutes |
| `candle15m` | 15 minutes |
| `candle30m` | 30 minutes |
| `candle1H` | 1 hour |
| `candle2H` | 2 hours |
| `candle4H` | 4 hours |
| `candle6H` | 6 hours |
| `candle12H` | 12 hours |
| `candle1D` | 1 day |
| `candle1W` | 1 week |
| `candle1M` | 1 month |

#### Push Data Format

```json
{
  "arg": {
    "channel": "candle1m",
    "instId": "BTC-USDT"
  },
  "data": [
    [
      "1678886400000",
      "30000.0",
      "30100.0",
      "29900.0",
      "30050.0",
      "1234.56"
    ]
  ]
}
```

Format: `[timestamp, open, high, low, close, volume]`

---

### Order Book Channel

Subscribe to order book updates.

#### Channel Format

```json
{
  "channel": "books",
  "instId": "BTC-USDT"
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "books",
    "instId": "BTC-USDT"
  },
  "data": [
    {
      "asks": [
        ["30000.5", "5.2", "0", "1"]
      ],
      "bids": [
        ["29999.5", "3.1", "0", "1"]
      ],
      "ts": "1678886400000",
      "tsBuf": "1678886400123"
    }
  ]
}
```

#### Order Book Depth

| Channel | Depth | Update Frequency |
|---------|-------|------------------|
| `books` | 400 levels | 100ms |
| `books10` | 10 levels | 100ms |
| `books50` | 50 levels | 100ms |
| `booksL2_25` | 25 levels | Real-time |

---

### Trade Channel

Subscribe to trade updates.

#### Channel Format

```json
{
  "channel": "trades",
  "instId": "BTC-USDT"
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "trades",
    "instId": "BTC-USDT"
  },
  "data": [
    ["1234567890", "0.1", "30000.5", "buy", "1652882888"]
  ]
}
```

Format: `[tradeId, price, size, side, timestamp]`

---

### Mark Price Channel

Subscribe to mark price updates.

#### Channel Format

```json
{
  "channel": "mark-price",
  "instId": "BTC-USDT-SWAP"
}
```

---

### Mark Price K-Line Channel

Subscribe to mark price candlestick updates.

#### Channel Format

```json
{
  "channel": "mark-price-candle1m",
  "instId": "BTC-USDT-SWAP"
}
```

---

### Funding Rate Channel

Subscribe to funding rate updates.

#### Channel Format

```json
{
  "channel": "funding-rate",
  "instId": "BTC-USDT-SWAP"
}
```

---

### Index Ticker Channel

Subscribe to index ticker updates.

#### Channel Format

```json
{
  "channel": "index-tickers",
  "instId": "BTC-USDT"
}
```

---

## Private Channels

Private channels require authentication via login.

### Account Channel

Subscribe to account balance and position updates.

#### Channel Format

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "account",
      "ccy": "BTC"
    }
  ]
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "account",
    "ccy": "BTC"
  },
  "data": [
    {
      "adjEq": "12345.67",
      "availBal": "1000.00",
      "availEq": "11000.00",
      "ccy": "BTC",
      "eq": "1000.00",
      "eqUsd": "30000.00",
      "frozenBal": "0",
      "ordFrozen": "0",
      "uTime": "1678886400000"
    }
  ]
}
```

#### Configuration Options

```json
{
  "channel": "account",
  "extraParams": "{\"updateInterval\": \"0\"}"
}
```

- `updateInterval`: `0` for event-driven only, or interval in milliseconds

---

### Positions Channel

Subscribe to position updates.

#### Channel Format

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "positions",
      "instType": "SWAP",
      "instId": "BTC-USDT-SWAP"
    }
  ]
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "positions",
    "instType": "SWAP"
  },
  "data": [
    {
      "adl": "1",
      "avgPx": "30000.0",
      "cTime": "1678886400000",
      "ccy": "USDT",
      "instId": "BTC-USDT-SWAP",
      "instType": "SWAP",
      "last": "31000.0",
      "lever": "10",
      "mgnMode": "cross",
      "mgnRatio": "15.5",
      "notionalUsd": "3100.00",
      "pos": "0.1",
      "posId": "123456789",
      "posSide": "long",
      "uTime": "1678886400000",
      "upl": "100.00",
      "uplRatio": "0.1"
    }
  ]
}
```

---

### Orders Channel

Subscribe to order updates.

#### Channel Format

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "orders",
      "instType": "SPOT"
    }
  ]
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "orders",
    "instType": "SPOT"
  },
  "data": [
    {
      "accFillSz": "0",
      "avgPx": "0",
      "cTime": "1678886400000",
      "clOrdId": "custom_order_id",
      "fee": "0",
      "feeCcy": "USDT",
      "instId": "BTC-USDT",
      "instType": "SPOT",
      "lastFillPx": "0",
      "lastFillSz": "0",
      "lever": "0",
      "ordId": "1234567890",
      "ordType": "limit",
      "pnl": "0",
      "posSide": "net",
      "px": "30000",
      "reduceOnly": "false",
      "side": "buy",
      "slTriggerPx": "0",
      "slTriggerPxType": "",
      "state": "live",
      "sz": "0.01",
      "tdMode": "cash",
      "tpTriggerPx": "0",
      "tpTriggerPxType": "",
      "uTime": "1678886400000"
    }
  ]
}
```

---

### Order Fills Channel

Subscribe to order fill/transaction updates.

#### Channel Format

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "fills",
      "instType": "SPOT"
    }
  ]
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "fills",
    "instType": "SPOT"
  },
  "data": [
    {
      "accFillSz": "0.01",
      "avgFillPx": "30000.0",
      "cTime": "1678886400000",
      "execType": "M",
      "fee": "-0.01",
      "feeCcy": "USDT",
      "fillFee": "-0.01",
      "fillFeeCcy": "USDT",
      "fillPnl": "0",
      "fillPx": "30000.0",
      "fillSz": "0.01",
      "instId": "BTC-USDT",
      "instType": "SPOT",
      "lever": "0",
      "ordId": "1234567890",
      "clOrdId": "",
      "pnl": "0",
      "posSide": "net",
      "side": "buy",
      "sz": "0.01",
      "tdMode": "cash",
      "tradeId": "9876543210",
      "uTime": "1678886400000"
    }
  ]
}
```

---

### Liquidation Warning Channel

Subscribe to liquidation warnings.

#### Channel Format

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "liquidation-warning",
      "instType": "ANY"
    }
  ]
}
```

#### Push Data Format

```json
{
  "arg": {
    "channel": "liquidation-warning",
    "uid": "77982378738415879",
    "instType": "FUTURES"
  },
  "data": [
    {
      "cTime": "1619507758793",
      "ccy": "ETH",
      "instId": "ETH-USD-210430",
      "instType": "FUTURES",
      "lever": "10",
      "markPx": "2353.849",
      "mgnMode": "isolated",
      "mgnRatio": "11.73",
      "pTime": "1619507761462",
      "pos": "1",
      "posCcy": "",
      "posId": "307173036051017730",
      "posSide": "long",
      "uTime": "1619507761462"
    }
  ]
}
```

---

### Algo Orders Channel

Subscribe to algo order updates.

#### Channel Format

```json
{
  "op": "subscribe",
  "args": [
    {
      "channel": "algo-orders",
      "instType": "SWAP"
    }
  ]
}
```

---

## Trading via WebSocket

### Place Order (Private WebSocket)

```json
{
  "id": "1512",
  "op": "order",
  "args": [
    {
      "instId": "BTC-USDT",
      "tdMode": "cash",
      "side": "buy",
      "ordType": "limit",
      "sz": "0.01",
      "px": "30000"
    }
  ]
}
```

#### Response

```json
{
  "id": "1512",
  "op": "order",
  "data": [
    {
      "ordId": "1234567890",
      "clOrdId": "",
      "sCode": "0",
      "sMsg": ""
    }
  ],
  "inTime": "1597026383085123",
  "outTime": "1597026383085456"
}
```

---

### Cancel Order

```json
{
  "id": "1512",
  "op": "cancel-order",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "1234567890"
    }
  ]
}
```

---

### Amend Order

```json
{
  "id": "1512",
  "op": "amend-order",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "1234567890",
      "newSz": "2",
      "newPx": "31000"
    }
  ]
}
```

---

## Heartbeat

### Ping

```json
{
  "op": "ping"
}
```

### Pong (Server Response)

```json
{
  "op": "pong"
}
```

---

## Error Codes

| Code | Description |
|------|-------------|
| 60012 | Invalid request |
| 60013 | Invalid JSON |
| 60014 | Authentication failed |
| 60015 | Connection already exists |
| 60016 | Connection limit exceeded |
| 60017 | Invalid channel |
| 60018 | Invalid parameter |
| 60019 | Invalid instrument |

---

## Implementation Notes

### Subscription Limits

- Maximum 100 subscriptions per WebSocket connection
- Total channel data size cannot exceed 64KB

### Reconnection Strategy

1. Detect disconnection
2. Wait 1 second
3. Reconnect and login
4. Resubscribe to channels
5. Resume from last known state

### Message ID

- Optional but recommended
- Used to correlate request and response
- Max 32 alphanumeric characters

### Timestamp Handling

- `cTime`: Creation time (milliseconds)
- `uTime`: Update time (milliseconds)
- `ts`: Data timestamp (milliseconds)

### Order Book Snapshot

- First message after subscription is full snapshot
- Subsequent messages are deltas
- Use `books` channel for full order book
- Use `booksL2_25` for incremental updates with sequence numbers
