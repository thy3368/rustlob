# OKX Market Data API

## Overview

Market Data endpoints provide public market data including ticker prices, order books, klines, and trade history. These endpoints do not require authentication.

**Base URL**: `https://www.okx.com/api/v5/market`

**Rate Limit**: 20 requests per 2 seconds (IP-based)

---

## Endpoints

### GET /api/v5/market/tickers

Retrieve the latest price snapshot, best bid/ask price, and trading volume in the last 24 hours.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | No | Instrument type: `SPOT`, `MARGIN`, `FUTURES`, `SWAP`, `OPTION` |
| instId | String | No | Instrument ID, e.g., `BTC-USDT` |

#### Request Example

```
GET /api/v5/market/tickers?instType=SWAP
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "instType": "SWAP",
      "instId": "BTC-USDT-SWAP",
      "last": "30000.5",
      "lastSz": "1.234",
      "askPx": "30001.0",
      "askSz": "10.0",
      "bidPx": "30000.0",
      "bidSz": "5.0",
      "open24h": "29000.0",
      "high24h": "31000.0",
      "low24h": "28000.0",
      "vol24h": "100000.0",
      "sTradingVol": "5000.0",
      "ts": "1678886400000"
    }
  ]
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| instType | String | Instrument type |
| instId | String | Instrument ID |
| last | String | Last traded price |
| lastSz | String | Last traded quantity |
| askPx | String | Best ask price |
| askSz | String | Best ask quantity |
| bidPx | String | Best bid price |
| bidSz | String | Best bid quantity |
| open24h | String | Open price in 24 hours |
| high24h | String | Highest price in 24 hours |
| low24h | String | Lowest price in 24 hours |
| vol24h | String | Trading volume in 24 hours (quote currency) |
| sTradingVol | String | Trading volume in 24 hours (base currency) |
| ts | String | Timestamp (milliseconds) |

---

### GET /api/v5/market/ticker

Retrieve ticker information for a specific instrument.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID, e.g., `BTC-USDT` |

#### Request Example

```
GET /api/v5/market/ticker?instId=BTC-USDT
```

#### Response

```json
{
  "code": "0",
  "msg": "",
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

### GET /api/v5/market/books

Retrieve order book data.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| sz | String | No | Order book depth size: `1-400`, default `400` |
| tg | String | No | Tick size: `0.1`, `0.01`, `0.001`, etc. |

#### Request Example

```
GET /api/v5/market/books?instId=BTC-USDT&sz=20
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "asks": [
        ["30000.5", "5.2", "0", "1"],
        ["30001.0", "3.1", "0", "2"]
      ],
      "bids": [
        ["29999.5", "3.1", "0", "1"],
        ["29999.0", "2.5", "0", "3"]
      ],
      "ts": "1678886400000",
      "tsBuf": "1678886400123"
    }
  ]
}
```

#### Order Book Entry Format

Each entry contains: `[price, size, liquidated orders, number of orders]`

---

### GET /api/v5/market/candles

Retrieve candlestick/chart data.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| bar | String | No | Bar size: `1m`, `3m`, `5m`, `15m`, `30m`, `1H`, `2H`, `4H`, `6H`, `12H`, `1D`, `1W`, `1M` |
| after | String | No | Pagination - get data after this timestamp |
| before | String | No | Pagination - get data before this timestamp |
| limit | String | No | Number of results, max `100`, default `100` |

#### Request Example

```
GET /api/v5/market/candles?instId=BTC-USDT&bar=1H&limit=10
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    ["1678886400000", "30000.0", "30100.0", "29900.0", "30050.0", "1234.56"],
    ["1678882800000", "29900.0", "30000.0", "29800.0", "29950.0", "2345.67"]
  ]
}
```

#### Candle Entry Format

Each entry contains: `[timestamp, open, high, low, close, volume]`

---

### GET /api/v5/market/history-candles

Retrieve historical candlestick data (more complete data than /candles).

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| bar | String | No | Bar size |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max `100`, default `100` |

---

### GET /api/v5/market/trades

Retrieve recent trade data.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| after | String | No | Get data after trade ID |
| before | String | No | Get data before trade ID |
| limit | String | No | Max `100`, default `100` |

#### Request Example

```
GET /api/v5/market/trades?instId=BTC-USDT&limit=10
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    ["1234567890", "0.1", "30000.5", "buy", "1652882888"],
    ["1234567889", "0.2", "30000.0", "sell", "1652882887"]
  ]
}
```

#### Trade Entry Format

Each entry contains: `[tradeId, price, size, side, timestamp]`

---

### GET /api/v5/market/mark-price

Retrieve mark price for derivatives.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |

---

### GET /api/mark-price-c/v5/marketandles

Retrieve mark price candlesticks.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| bar | String | No | Bar size |
| limit | String | No | Max `100`, default `100` |

---

## Public Data Endpoints

### GET /api/v5/public/instruments

Retrieve information about available trading instruments.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | Yes | Instrument type |
| uly | String | No | Underlying (for derivatives) |
| instFamily | String | No | Instrument family |
| instId | String | No | Filter by instrument |

#### Request Example

```
GET /api/v5/public/instruments?instType=SWAP
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "instId": "BTC-USDT-SWAP",
      "instType": "SWAP",
      "uly": "BTC-USDT",
      "settleCcy": "USDT",
      "ctVal": "0.01",
      "settleMode": "linear",
      "contractVal": "0.01"
    }
  ]
}
```

---

### GET /api/v5/public/funding-rate

Retrieve current funding rate for perpetual contracts.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |

---

### GET /api/v5/public/funding-rate-history

Retrieve historical funding rates.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max `100`, default `100` |

---

### GET /api/v5/public/open-interest

Retrieve open interest data.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |

---

### GET /api/v5/public/price-limit

Retrieve price limit information.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |

---

### GET /api/v5/public/time

Retrieve system time.

#### Request Example

```
GET /api/v5/public/time
```

#### Response

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

### GET /api/v5/public/exchange-rate

Retrieve exchange rates.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | No | Index instrument ID |

---

## Implementation Notes

### Order Book Depth Levels

- Default depth: 400 levels (200 bids + 200 asks)
- Supported sizes: 1, 5, 10, 20, 50, 100, 200, 400
- Tick size aggregation available via `tg` parameter

### Candlestick Intervals

| Code | Interval |
|------|----------|
| 1m | 1 minute |
| 3m | 3 minutes |
| 5m | 5 minutes |
| 15m | 15 minutes |
| 30m | 30 minutes |
| 1H | 1 hour |
| 2H | 2 hours |
| 4H | 4 hours |
| 6H | 6 hours |
| 12H | 12 hours |
| 1D | 1 day |
| 1W | 1 week |
| 1M | 1 month |

### Timestamp Handling

- All timestamps are in **milliseconds** (13 digits)
- Use `ts` field from responses for pagination
- `sodUtc0`: Price at UTC 00:00
- `sodUtc8`: Price at UTC 08:00 (for Asian markets)
