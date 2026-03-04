# OKX Account API

## Overview

Account endpoints provide access to account balance, positions, settings, and risk management. These endpoints require authentication.

**Base URL**: `https://www.okx.com/api/v5/account`

---

## Balance Endpoints

### GET /api/v5/account/balance

Get account balance across all assets.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Single currency or multiple (max 20) |

#### Request Example

```
GET /api/v5/account/balance
```

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "adjEq": "12345.67",
      "totalEq": "12345.67",
      "isoEq": "1000.00",
      "availEq": "11000.00",
      "details": [
        {
          "availBal": "1000.00",
          "availEq": "1000.00",
          "bal": "1000.00",
          "ccy": "BTC",
          "crossLiab": "0",
          "eq": "1000.00",
          "eqUsd": "30000.00",
          "frozenBal": "0",
          "interest": "0",
          "isoBal": "0",
          "isoLiab": "0",
          "liab": "0",
          "maxLoan": "50000",
          "mgnRatio": "",
          "notionalLever": "0",
          "ordFrozen": "0",
          "spotInUseAmt": "",
          "stakedEq": "0",
          "twap": "0",
          "uTime": "1678886400000"
        }
      ]
    }
  ]
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| adjEq | String | Adjusted equity |
| totalEq | String | Total equity |
| isoEq | String | Isolated margin equity |
| availEq | String | Available equity |
| details[] | Array | Currency details |
| details[].ccy | String | Currency |
| details[].bal | String | Balance |
| details[].availBal | String | Available balance |
| details[].frozenBal | String | Frozen balance |
| details[].eq | String | Equity |
| details[].eqUsd | String | Equity in USD |

---

### GET /api/v5/account/positions

Get all open positions.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | No | Instrument type: `MARGIN`, `FUTURES`, `SWAP`, `OPTION` |
| instId | String | No | Instrument ID |
| posId | String | No | Position ID |

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "adl": "1",
      "avgPx": "30000.0",
      "cTime": "1678886400000",
      "ccy": "USDT",
      "deltaBS": "",
      "deltaPA": "",
      "gammaBS": "",
      "gammaPA": "",
      "instId": "BTC-USDT-SWAP",
      "instType": "SWAP",
      "isoEq": "0",
      "last": "31000.0",
      "lever": "10",
      "liab": "",
      "liabCcy": "",
      "mgnMode": "cross",
      "mgnRatio": "15.5",
      "mmr": "0.00046",
      "notionalUsd": "3100.00",
      "optVal": "",
      "pos": "0.1",
      "posCcy": "BTC",
      "posId": "123456789",
      "posSide": "long",
      "thetaBS": "",
      "thetaPA": "",
      "uTime": "1678886400000",
      "upl": "100.00",
      "uplRatio": "0.1",
      "vegaBS": "",
      "vegaPA": ""
    }
  ]
}
```

---

### GET /api/v5/account/positions-history

Get position history.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | No | Instrument type |
| instId | String | No | Instrument ID |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100, default 100 |

---

## Account Configuration

### GET /api/v5/account/config

Get account configuration.

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "acctLv": "3",
      "autoLoan": false,
      "ctIsoMode": "automatic",
      "dcs": "true",
      "df": "true",
      "瓜等级": "VIP0",
      "liquidationSw": "false",
      "mgnIsoMode": "automatic",
      "optSw": "true",
      "posMode": "long_short_mode",
      "spotOffsetType": "1",
      "sweepSP": "true",
      "sweepSPN": "true",
      "ts": "1678886400000",
      "uid": "123456789"
    }
  ]
}
```

---

### POST /api/v5/account/set-position-mode

Set position mode.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| posMode | String | Yes | `long_short_mode` or `net_mode` |

---

### POST /api/v5/account/set-leverage

Set leverage for an instrument.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| lever | String | Yes | Leverage level |
| mgnMode | String | Yes | Margin mode: `cross`, `isolated` |
| posSide | String | Cond | Position side (required for isolated in long/short mode) |

#### Request Example

```json
{
  "instId": "BTC-USDT-SWAP",
  "lever": "10",
  "mgnMode": "cross"
}
```

---

### GET /api/v5/account/leverage

Get leverage information.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instId | String | Yes | Instrument ID |
| mgnMode | String | Yes | Margin mode |

---

### POST /api/v5/account/set-riskOffsetCoeff

Set risk offset coefficient.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| riskOffsetCoeff | String | Yes | Risk offset coefficient |
| type | String | Yes | Type: `all`, `borrow`, `transfer` |

---

## Position Tiers

### GET /api/v5/public/position-tiers

Get position tier information.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | Yes | Instrument type: `MARGIN`, `FUTURES`, `SWAP` |
| uly | String | No | Underlying |

---

## Risk Management

### GET /api/v5/account/risk-state

Get risk state.

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "colRatio": "150.00",
      "mgnRatio": "15.00",
      "mmr": "0.50",
      "tStart": "0",
      "uTime": "1678886400000"
    }
  ]
}
```

---

### GET /api/v5/account/position-risk

Get account and position risk.

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "adjEq": "12345.67",
      "totalEq": "12345.67",
      "imr": "1000.00",
      "mmr": "50.00",
      "mgnRatio": "12.00",
      "posEq": "5000.00",
      "spotEq": "7345.67",
      "ts": "1678886400000"
    }
  ]
}
```

---

## Trading Fees

### GET /api/v5/account/trade-fee

Get trading fee rates.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| instType | String | No | Instrument type |
| instId | String | No | Instrument ID |

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "deliveryFee": "",
      "exerciseFee": "",
      "feeTier": "0",
      "instType": "SPOT",
      "maker": "-0.0008",
      "makerU": "-0.0008",
      "taker": "0.001",
      "takerU": "0.001",
      "vip": "false"
    }
  ]
}
```

---

### GET /api/v5/account/interest-accrued

Get interest accrued.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |
| after | String | No | Pagination |
| limit | String | No | Max 100 |

---

### GET /api/v5/account/interest-rate

Get interest rate.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |

---

## Greeks (Options)

### GET /api/v5/account/account-position-greeks

Get account and position Greeks.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |

---

## Funding

### GET /api/v5/asset/balances

Get funding account balance.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "availBal": "37.11827078",
      "bal": "37.11827078",
      "ccy": "ETH",
      "frozenBal": "0"
    }
  ]
}
```

---

### GET /api/v5/asset/bills

Get asset bills (transaction history).

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100, default 100 |
| type | String | No | Bill type |

#### Bill Types

| Type | Description |
|------|-------------|
| 1 | Deposit |
| 2 | Withdrawal |
| 12 | Transfer to sub-account |
| 13 | Transfer from sub-account |
| 14 | Transfer to main account |
| 15 | Transfer from main account |

---

### POST /api/v5/asset/transfer

Transfer funds between accounts.

#### Request Body

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | Yes | Currency |
| amt | String | Yes | Amount |
| from | String | Yes | From: `18` (Funding), `19` (Trading) |
| to | String | Yes | To: `18` (Funding), `19` (Trading) |
| subAcct | String | No | Sub-account ID |
| instId | String | No | Instrument ID (for transfer to trading) |

---

### POST /api/v5/asset/deposit

Get deposit address.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | Yes | Currency |

---

### GET /api/v5/asset/deposit-history

Get deposit history.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100 |

---

### GET /api/v5/asset/withdrawal-history

Get withdrawal history.

#### Query Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| ccy | String | No | Currency |
| after | String | No | Pagination |
| before | String | No | Pagination |
| limit | String | No | Max 100 |

---

## API Key Management

### GET /api/v5/account/api-key-info

Get API key information.

#### Response

```json
{
  "code": "0",
  "msg": "",
  "data": [
    {
      "apiKey": "your_api_key",
      "caps": "0",
      "ctValList": [],
      "expireTime": "1735689600000",
      "ip": "",
      "lastTime": "1678886400000",
      "note": "My API Key",
      "perm": "read_only,trade,option",
      "apiKeyName": "TestKey",
      "pubKey": "",
      "type": "1"
    }
  ]
}
```

---

## Implementation Notes

### Position Modes

| Mode | Description |
|------|-------------|
| `long_short_mode` | Separate long and short positions |
| `net_mode` | Net position (one position per instrument) |

### Margin Modes

| Mode | Description |
|------|-------------|
| `cross` | Cross margin - uses entire account balance |
| `isolated` | Isolated margin - position has separate balance |

### Leverage

- Range: 0.1 - 125 (varies by instrument)
- Supported for: MARGIN, FUTURES, SWAP
- Not supported for: SPOT

### Risk Metrics

| Metric | Description |
|--------|-------------|
| adjEq | Adjusted equity |
| totalEq | Total equity |
| imr | Initial margin requirement |
| mmr | Maintenance margin requirement |
| mgnRatio | Margin ratio |
| colRatio | Collateral ratio |
| upl | Unrealized P&L |
| uplRatio | Unrealized P&L ratio |

### Fee Structure

| Tier | Maker Fee | Taker Fee |
|------|-----------|-----------|
| VIP 0 | -0.0008 | 0.001 |
| VIP 1+ | -0.0006 | 0.0008 |
| VIP 3+ | -0.0004 | 0.0006 |
| VIP 5+ | -0.0002 | 0.0004 |

Note: Negative maker fees = rebate
