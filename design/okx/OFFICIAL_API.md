# OKX API Official Documentation

> Source: https://www.okx.com/docs-v5/en

## Table of Contents

1. [Overview](#overview)
2. [REST API](#rest-api)
   - [Authentication](#authentication)
   - [Trading](#trading)
   - [Account](#account)
   - [Market Data](#market-data)
   - [Public Data](#public-data)
   - [Funding](#funding)
   - [Sub-account](#sub-account)
3. [WebSocket API](#websocket-api)
4. [Best Practices](#best-practices)

---

# Overview

OKX API v5 provides comprehensive access to trading, account management, market data, and advanced trading features.

## Base URLs

| Service | URL |
|---------|-----|
| REST API | `https://www.okx.com` |
| WebSocket (Public) | `wss://ws.okx.com:8443/ws/v5/public` |
| WebSocket (Private) | `wss://ws.okx.com:8443/ws/v5/private` |

## API Categories

| Category | Prefix | Description |
|----------|--------|-------------|
| Public Data | `/api/v5/public` | System time, instruments, etc. |
| Market Data | `/api/v5/market` | Tickers, order books, klines |
| Trading | `/api/v5/trade` | Orders, algo orders |
| Account | `/api/v5/account` | Balance, positions, settings |
| Funding | `/api/v5/asset` | Deposits, withdrawals, transfers |
| Sub-account | `/api/v5/users` | Sub-account management |

---

# REST API

## Authentication

### Generating an API Key

1. Log in to OKX account
2. Go to API section in settings
3. Create API key with required permissions
4. Save API Key, Secret Key, and Passphrase

### API Key Permissions

| Permission | Description |
|------------|-------------|
| Read-only | Market data, account queries |
| Trade | Place, cancel, amend orders |
| Withdraw | Withdraw funds |

### Request Signing

All private endpoints require HMAC-SHA256 signature authentication.

#### Request Headers

| Header | Description |
|--------|-------------|
| `OK-ACCESS-KEY` | Your API key |
| `OK-ACCESS-SIGN` | Base64-encoded HMAC-SHA256 signature |
| `OK-ACCESS-TIMESTAMP` | UTC timestamp (ISO 8601 format) |
| `OK-ACCESS-PASSPHRASE` | Your API passphrase |

#### Signature Generation

```
timestamp = UTC timestamp in seconds (e.g., "1538054050")
method = "GET" or "POST"
requestPath = "/api/v5/account/balance"
body = Request body (empty string for GET requests)

signature = Base64(HMAC-SHA256(timestamp + method + requestPath + body, secretKey))
```

#### Python Example

```python
import hmac
import base64
import requests
from datetime import datetime

def sign(timestamp, method, request_path, body, secret_key):
    """Generate HMAC SHA256 signature for OKX API requests"""
    message = timestamp + method + request_path + body
    mac = hmac.new(
        bytes(secret_key, encoding='utf8'),
        bytes(message, encoding='utf-8'),
        digestmod='sha256'
    )
    d = mac.digest()
    return base64.b64encode(d).decode()

# Complete authenticated request example
api_key = "YOUR-API-KEY"
secret_key = "YOUR-SECRET-KEY"
passphrase = "YOUR-PASSPHRASE"

timestamp = datetime.utcnow().isoformat()[:-3] + 'Z'
method = 'GET'
request_path = '/api/v5/account/balance'
body = ''

signature = sign(timestamp, method, request_path, body, secret_key)

headers = {
    'OK-ACCESS-KEY': api_key,
    'OK-ACCESS-SIGN': signature,
    'OK-ACCESS-TIMESTAMP': timestamp,
    'OK-ACCESS-PASSPHRASE': passphrase
}

url = 'https://www.okx.com' + request_path
response = requests.get(url, headers=headers)
print(response.json())
```

### Response Format

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

---

## Trading

### POST /api/v5/trade/order

Place a new trading order.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID (e.g., `BTC-USDT`) |
| tdMode | String | Yes | Trade mode: `cash`, `cross`, `isolated` |
| side | String | Yes | Order side: `buy`, `sell` |
| ordType | String | Yes | Order type: `market`, `limit`, `post_only`, `fok`, `ioc` |
| sz | String | Yes | Order size |
| px | String | Conditional | Order price (required for non-market orders) |
| clOrdId | String | No | Client Order ID (max 32 chars) |
| reduceOnly | Boolean | No | Whether to close position only |
| posSide | String | No | Position side: `long`, `short`, `net` |

#### Request Example

```json
{
  "instId": "BTC-USDT-SWAP",
  "tdMode": "cross",
  "clOrdId": "testBTC0123",
  "side": "buy",
  "ordType": "limit",
  "px": "50912.4",
  "sz": "1"
}
```

#### Response Example

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "clOrdId": "testBTC0123",
      "ordId": "288981657420439575",
      "tag": "",
      "sCode": "0",
      "sMsg": ""
    }
  ]
}
```

### POST /api/v5/trade/cancel-order

Cancel an order.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| ordId | String | Conditional | Order ID (required if clOrdId not provided) |
| clOrdId | String | Conditional | Client Order ID |

### POST /api/v5/trade/amend-order

Amend an order.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| ordId | String | Conditional | Order ID |
| clOrdId | String | Conditional | Client Order ID |
| newSz | String | No | New order size |
| newPx | String | No | New order price |

### GET /api/v5/trade/order

Get order details by order ID.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| ordId | String | Conditional | Order ID |
| clOrdId | String | Conditional | Client Order ID |

### GET /api/v5/trade/orders-pending

Get all pending orders.

### GET /api/v5/trade/orders-history

Get order history (last 7 days).

### GET /api/v5/trade/fills

Get order fills (last 7 days).

---

## Account

### GET /api/v5/account/balance

Retrieve balance details for all assets.

#### Response Example

```json
{
    "code": "0",
    "msg": "",
    "data": [
        {
            "totalEq": "10679688.0460531643092577",
            "details": [
                {
                    "ccy": "USDT",
                    "eq": "10679688.0460531643092577",
                    "availBal": "10679688.0460531643092577"
                },
                {
                    "ccy": "BTC",
                    "eq": "1.5",
                    "availBal": "1.5"
                }
            ]
        }
    ]
}
```

### GET /api/v5/account/positions

Get all open positions.

### GET /api/v5/account/leverage

Get leverage information.

### POST /api/v5/account/set-leverage

Set leverage.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| lever | String | Yes | Leverage level |
| mgnMode | String | Yes | Margin mode: `cross`, `isolated` |
| posSide | String | Conditional | Position side (required for isolated) |

---

## Market Data

### GET /api/v5/market/ticker

Get ticker for specific instrument.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |

#### Response Example

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "instId": "BTC-USDT",
      "last": "30000.5",
      "lastSz": "0.001",
      "askPx": "30001.0",
      "bidPx": "30000.0",
      "bidSz": "0.002",
      "askSz": "0.001",
      "open24h": "29500.0",
      "high24h": "30500.0",
      "low24h": "29000.0",
      "vol24h": "1000.0",
      "volCcy24h": "30000000.0",
      "ts": "1672425600000"
    }
  ]
}
```

### GET /api/v5/market/tickers

Get tickers for all instruments.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | No | Instrument type: `SPOT`, `SWAP`, `FUTURES`, `OPTION` |

### GET /api/v5/market/books

Get order book.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| sz | String | No | Depth size (max 400) |

#### Response Example

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "instId": "BTC-USDT",
      "bids": [
        ["30000.0", "0.002"],
        ["29999.5", "0.001"]
      ],
      "asks": [
        ["30001.0", "0.001"],
        ["30001.5", "0.002"]
      ],
      "ts": "1672425600000"
    }
  ]
}
```

### GET /api/v5/market/candles

Get candlestick data.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| bar | String | Yes | Timeframe: `1m`, `3m`, `5m`, `15m`, `30m`, `1H`, `2H`, `4H`, `6H`, `12H`, `1D`, `1W`, `1M` |
| limit | String | No | Max results (default 100) |

#### Response Example

```json
{
  "code": "0",
  "msg": "",
  "data": [
    ["1672425600000", "29500.0", "30500.0", "29000.0", "30000.5", "1000.0"]
  ]
}
```

Format: `[timestamp, open, high, low, close, volume]`

---

## Public Data

### GET /api/v5/public/instruments

Get instrument list.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | Yes | Instrument type: `SPOT`, `MARGIN`, `FUTURES`, `SWAP`, `OPTION` |
| uly | String | No | Underlying (for derivatives) |

### GET /api/v5/public/funding-rate

Get current funding rate.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |

### GET /api/v5/public/time

Get system time.

#### Response Example

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "ts": "1678886400000"
    }
  ]
}
```

---

## Funding

### GET /api/v5/asset/balances

Get funding account balance.

### POST /api/v5/asset/transfer

Transfer funds between accounts.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | Yes | Currency |
| amt | String | Yes | Amount |
| from | String | Yes | From: `18` (Funding), `19` (Trading) |
| to | String | Yes | To: `18` (Funding), `19` (Trading) |

### GET /api/v5/asset/deposit-address

Get deposit address.

### GET /api/v5/asset/deposit-history

Get deposit history.

### GET /api/v5/asset/withdrawal-history

Get withdrawal history.

---

## Sub-account

### GET /api/v5/users/subaccount/list

Get sub-account list.

### POST /api/v5/users/subaccount/create

Create sub-account.

### POST /api/v5/users/subaccount/api-key

Create API key for sub-account.

---

# WebSocket API

## Connection

### Public Channels

```
wss://ws.okx.com:8443/ws/v5/public
```

### Private Channels

```
wss://ws.okx.com:8443/ws/v5/private
```

## Authentication

After establishing connection, private channels require login:

```json
{
    "op": "login",
    "args": [{
        "apiKey": "YOUR_API_KEY",
        "passphrase": "YOUR_PASSPHRASE",
        "timestamp": "1538054050",
        "sign": "YOUR_SIGNATURE"
    }]
}
```

### Signature Generation for WebSocket

```
timestamp = Unix timestamp in seconds
sign = Base64(HMAC-SHA256(timestamp + 'GET' + '/users/self/verify', secretKey))
```

## Subscribe

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

## Unsubscribe

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

## Public Channels

### Tickers Channel

```json
{
  "arg": {
    "channel": "tickers",
    "instId": "BTC-USDT"
  },
  "data": [...]
}
```

### Order Book Channel

| Channel | Depth | Update Frequency |
|---------|-------|------------------|
| `books` | 400 levels | 100ms |
| `books10` | 10 levels | 100ms |
| `books50` | 50 levels | 100ms |

### Candlesticks Channel

Channel format: `candle{period}`, e.g., `candle1m`, `candle15m`, `candle1H`, `candle1D`

### Trades Channel

```json
{
  "arg": {
    "channel": "trades",
    "instId": "BTC-USDT"
  },
  "data": [...]
}
```

## Private Channels

### Account Channel

```json
{
  "arg": {
    "channel": "account"
  },
  "data": [...]
}
```

### Positions Channel

```json
{
  "arg": {
    "channel": "positions",
    "instType": "SWAP"
  },
  "data": [...]
}
```

### Orders Channel

```json
{
  "arg": {
    "channel": "orders",
    "instType": "SPOT"
  },
  "data": [...]
}
```

### Fills Channel

```json
{
  "arg": {
    "channel": "fills",
    "instType": "SPOT"
  },
  "data": [...]
}
```

## Trading via WebSocket

### Place Order

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

### Amend Order

```json
{
  "id": "1512",
  "op": "amend-order",
  "args": [
    {
      "instId": "BTC-USDT",
      "ordId": "1234567890",
      "newSz": "2"
    }
  ]
}
```

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

# Best Practices

## Rate Limits

| Endpoint Category | Limit |
|-------------------|-------|
| Public Data | 20 requests/2s (IP-based) |
| Market Data | 20 requests/2s (IP-based) |
| Trading | 60 requests/2s (User + Instrument) |

## Error Codes

| Code | Description |
|------|-------------|
| `0` | Success |
| `1` | General error |
| `50001` | Missing parameter |
| `50002` | Invalid parameter |
| `50101` | Authentication failed |
| `51101` | Insufficient balance |
| `51201` | Order not found |

## Recommendations

1. **Use WebSocket for real-time data** - More efficient than polling
2. **Implement reconnection logic** - Handle network disruptions
3. **Use clOrdId for order tracking** - Client-side order identification
4. **Handle rate limiting** - Implement exponential backoff
5. **Validate orders before submission** - Use order precheck endpoint

## Python WebSocket Example

```python
import websockets
import json
import asyncio
import hmac
import base64
import datetime

class OKXWebSocketClient:
    def __init__(self, api_key, secret_key, passphrase):
        self.api_key = api_key
        self.secret_key = secret_key
        self.passphrase = passphrase
        self.ws = None
        self.connected = False
        
    def get_timestamp(self):
        now = datetime.datetime.utcnow()
        t = now.isoformat("T", "milliseconds")
        return t + "Z"
        
    def sign(self, timestamp, method, request_path, body):
        message = timestamp + method + request_path + body
        mac = hmac.new(
            bytes(self.secret_key, encoding='utf8'),
            bytes(message, encoding='utf-8'),
            digestmod='sha256'
        )
        d = mac.digest()
        return base64.b64encode(d).decode()
        
    async def connect(self):
        self.ws = await websockets.connect('wss://ws.okx.com:8443/ws/v5/private')
        timestamp = self.get_timestamp()
        sign = self.sign(timestamp, 'GET', '/users/self/verify', '')
        
        login_data = {
            "op": "login",
            "args": [{
                "apiKey": self.api_key,
                "passphrase": self.passphrase,
                "timestamp": timestamp,
                "sign": sign
            }]
        }
        
        await self.ws.send(json.dumps(login_data))
        response = await self.ws.recv()
        self.connected = True
        print(f"Connected: {response}")
            
    async def subscribe(self, channels):
        if not self.connected:
            await self.connect()
            
        subscribe_data = {
            "op": "subscribe",
            "args": channels
        }
        await self.ws.send(json.dumps(subscribe_data))
        response = await self.ws.recv()
        print(f"Subscription response: {response}")
            
    async def receive_messages(self):
        while True:
            message = await self.ws.recv()
            data = json.loads(message)
            print(f"Received: {data}")
```

---

# Official Resources

- [API Documentation](https://www.okx.com/docs-v5/en)
- [API Console](https://www.okx.com/docs-v5/en)
- [Change Log](https://www.okx.com/docs-v5/log_en)
- [Best Practices](https://www.okx.com/docs-v5/trick_en)
