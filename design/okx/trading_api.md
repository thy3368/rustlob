# OKX Trading API

## Overview

Trading endpoints enable order placement, cancellation, modification, and order history queries. These endpoints require authentication.

**Base URL**: `https://www.okx.com/api/v5/trade`

**Rate Limits**:
- Place Order: 60 requests/2s (User + Instrument)
- Cancel/Amend: 60 requests/2s (User + Instrument)

---

## Order Placement

### POST /api/v5/order

Place a new order.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID, e.g., `BTC-USDT` |
| tdMode | String | Yes | Trade mode: `cash`, `cross`, `isolated`, `spot_isolated` |
| ccy | String | No | Margin currency (for isolated/cross margin) |
| clOrdId | String | No | Client Order ID (max 32 chars) |
| tag | String | No | Order tag (max 16 chars) |
| side | String | Yes | Order side: `buy`, `sell` |
| posSide | String | Cond | Position side: `long`, `short`, `net` (required for futures in long/short mode) |
| ordType | String | Yes | Order type |
| sz | String | Yes | Order size |
| px | String | Cond | Order price (required for limit orders) |
| reduceOnly | Boolean | No | Reduce only flag |
| tgtCcy | String | No | Target currency: `base_ccy`, `quote_ccy` (for spot market orders) |
| banAmend | Boolean | No | Ban amend for spot market order |
| stpMode | String | No | Self-trade prevention: `cancel_maker`, `cancel_taker`, `cancel_both` |
| attachAlgoOrds | Array | No | Attached TP/SL orders |

#### Order Types

| Type | Code | Description |
|------|------|-------------|
| Market | `market` | Market order |
| Limit | `limit` | Limit order |
| Post Only | `post_only` | Maker only |
| FOK | `fok` | Fill or Kill |
| IOC | `ioc` | Immediate or Cancel |
| Optimal Limit IOC | `optimal_limit_ioc` | Market limit IOC |
| MMP | `mmp` | Market Maker Protection |

#### Request Examples

**Limit Order**
```json
{
  "instId": "BTC-USDT",
  "tdMode": "cash",
  "side": "buy",
  "ordType": "limit",
  "sz": "0.01",
  "px": "30000"
}
```

**Market Order**
```json
{
  "instId": "BTC-USDT",
  "tdMode": "cash",
  "side": "buy",
  "ordType": "market",
  "sz": "0.01"
}
```

**Limit Order with TP/SL**
```json
{
  "instId": "BTC-USDT",
  "tdMode": "cross",
  "side": "buy",
  "ordType": "limit",
  "sz": "1",
  "px": "30000",
  "attachAlgoOrds": [
    {
      "tpTriggerPx": "35000",
      "tpOrdPx": "34900",
      "slTriggerPx": "25000",
      "slOrdPx": "25100"
    }
  ]
}
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "ordId": "1234567890",
      "clOrdId": "custom_order_id",
      "slem": "live"
    }
  ],
  "inTime": "1597026383085123",
  "outTime": "1597026383085456"
}
```

---

## Order Cancellation

### POST /api/v5/trade/cancel-order

Cancel an incomplete order.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| ordId | String | Cond | Order ID (required if clOrdId not provided) |
| clOrdId | String | Cond | Client Order ID (required if ordId not provided) |
| cxlOnFailData | String | No | Cancel on fail data |

#### Request Example

```json
{
  "instId": "BTC-USDT",
  "ordId": "590908157585625111"
}
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "ordId": "590908157585625111",
      "clOrdId": "oktswap6",
      "tag": "",
      "ts": "1695190491421",
      "sCode": "0",
      "sMsg": ""
    }
  ]
}
```

---

## Order Amendment

### POST /api/v5/trade/amend-order

Amend an existing order.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| ordId | String | Cond | Order ID (required if clOrdId not provided) |
| clOrdId | String | Cond | Client Order ID |
| newSz | String | No | New order size |
| newPx | String | No | New order price |
| cxlOnFailData | String | No | Cancel on fail data |

#### Request Example

```json
{
  "instId": "BTC-USDT",
  "ordId": "590909145319051111",
  "newSz": "2",
  "newPx": "31000"
}
```

---

## Algo Orders

### POST /api/v5/trade/order-algo

Place algo orders (trigger, OCO, TWAP, trailing stop).

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| tdMode | String | Yes | Trade mode |
| side | String | Yes | Order side |
| ordType | String | Yes | Algo order type |
| sz | String | Yes | Order size |
| triggerPx | String | Cond | Trigger price (for trigger orders) |
| triggerPxType | String | Cond | Trigger price type: `last`, `index`, `mark` |
| orderPx | String | Cond | Order price |
| posSide | String | No | Position side |
| reduceOnly | Boolean | No | Reduce only |

#### Algo Order Types

| Type | Code | Description |
|------|------|-------------|
| Trigger | `trigger` | Trigger order |
| Conditional | `conditional` | Conditional order |
| OCO | `oco` | One-Cancels-the-Other |
| TWAP | `twap` | Time-Weighted Average Price |
| Trailing Stop | `move_order_stop` | Trailing stop |

#### Trigger Order Example

```json
{
  "instId": "BTC-USDT-SWAP",
  "side": "buy",
  "tdMode": "cross",
  "sz": "1",
  "ordType": "trigger",
  "triggerPx": "25920",
  "triggerPxType": "last",
  "orderPx": "-1"
}
```

#### TWAP Order Example

```json
{
  "instId": "BTC-USDT-SWAP",
  "tdMode": "cross",
  "side": "buy",
  "ordType": "twap",
  "sz": "10",
  "szLimit": "1",
  "pxLimit": "100",
  "timeInterval": "10"
}
```

#### Trailing Stop Example

```json
{
  "instId": "BTC-USDT-SWAP",
  "tdMode": "cross",
  "side": "buy",
  "ordType": "move_order_stop",
  "sz": "10",
  "callbackRatio": "0.05",
  "reduceOnly": true
}
```

---

## Order Queries

### GET /api/v5/trade/order

Get order details by order ID or client order ID.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| ordId | String | Cond | Order ID |
| clOrdId | String | Cond | Client Order ID |

---

### GET /api/v5/trade/orders-pending

Get all pending orders.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | No | Instrument ID |
| ordType | String | No | Order type |
| state | String | No | Order state |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100, default 100 |

---

### GET /api/v5/trade/orders-history

Get historical orders (last 7 days).

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | No | Instrument ID |
| ordType | String | No | Order type |
| state | String | No | Order state |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100, default 100 |

---

### GET /api/v5/trade/orders-history-archive

Get archived orders (last 3 months).

#### Query Parameters

Same as orders-history.

---

### GET /api/v5/trade/fills

Get order fills (last 7 days).

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | No | Instrument ID |
| ordId | String | No | Order ID |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100, default 100 |

---

## Order States

| State | Code | Description |
|-------|------|-------------|
| Canceled | `canceled` | Order canceled |
| Live | `live` | Order pending fill |
| Partially Filled | `partially_filled` | Order partially filled |
| Filled | `filled` | Order fully filled |

---

## Batch Orders

### POST /api/v5/trade/batch-orders

Place multiple orders in a single request (max 10 orders).

#### Request Example

```json
[
  {
    "instId": "BTC-USDT",
    "tdMode": "cash",
    "side": "buy",
    "ordType": "limit",
    "sz": "0.01",
    "px": "30000"
  },
  {
    "instId": "ETH-USDT",
    "tdMode": "cash",
    "side": "buy",
    "ordType": "limit",
    "sz": "0.1",
    "px": "2000"
  }
]
```

---

### POST /api/v5/trade/cancel-batch-orders

Cancel multiple orders in a single request.

---

### POST /api/v5/trade/amend-batch-orders

Amend multiple orders in a single request.

---

## Order Pre-check

### POST /api/v5/trade/order-precheck

Precheck account balance before placing an order.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| tdMode | String | Yes | Trade mode |
| side | String | Yes | Order side |
| ordType | String | Yes | Order type |
| sz | String | Yes | Order size |
| px | String | Yes | Order price |

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "availBal": "1000.50",
      "availBalChg": "-10.00",
      "imr": "5.00"
    }
  ]
}
```

---

## Rate Limit Info

### GET /api/v5/trade/account-rate-limit

Get account rate limit information.

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "accRateLimit": "2000",
      "fillRatio": "0.1234",
      "ts": "123456789000"
    }
  ]
}
```

---

## Implementation Notes

### Client Order ID (clOrdId)

- Maximum 32 alphanumeric characters
- Should be unique per order
- Can be used for idempotency
- Server returns same order ID for duplicate clOrdId

### Self-Trade Prevention (stpMode)

| Mode | Behavior |
|------|----------|
| `cancel_maker` | Cancel the maker order |
| `cancel_taker` | Cancel the taker order |
| `cancel_both` | Cancel both orders |

### Attach Algo Orders

For attaching TP/SL to an order:

```json
"attachAlgoOrds": [
  {
    "attachAlgoClOrdId": "tp_sl_001",
    "tpTriggerPx": "35000",
    "tpOrdPx": "34900",
    "tpTriggerPxType": "last",
    "slTriggerPx": "25000",
    "slOrdPx": "25100",
    "slTriggerPxType": "last"
  }
]
```

### Order Size (sz) Units

- **Spot**: Can be base or quote currency (controlled by `tgtCcy`)
- **Futures/Swap**: Contract multiplier from instrument details
- **Options**: Contract size from instrument details

### Price (px) Types

- For **options**: One of `px`, `pxUsd`, or `pxVol` must be provided
- `-1` for market price execution
- `pxType`: `last`, `index`, `mark` (for trigger orders)
