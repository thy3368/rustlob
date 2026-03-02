# Welcome to Exchange APIs

> Source: https://docs.cdp.coinbase.com/exchange/introduction/welcome

---

## Overview

Welcome to Coinbase Exchange API documentation for traders and developers! The APIs are separated into two categories, trading and market data:

* **Trading APIs** require authentication and let you place orders and access account information.
* **Market Data APIs** provide market data and are public.

---

## API Connectivity Options

Coinbase Exchange offers multiple connectivity options tailored to your trading and data needs:

### REST API
- For lower-frequency trading and general requests
- Documentation: https://docs.cdp.coinbase.com/exchange/rest-api/requests

### FIX Order Entry API
- For higher-frequency trading
- Documentation: https://docs.cdp.coinbase.com/exchange/fix-api/order-entry-messages/order-entry-messages5

### WebSocket Feed
- For market data
- Documentation: https://docs.cdp.coinbase.com/exchange/websocket-feed/overview

### FIX Market Data API
- For latency sensitive market data feeds
- Documentation: https://docs.cdp.coinbase.com/exchange/fix-api/market-data

---

## Important Notices

<Warning>
  **FIX 4.2 Order Entry Gateway Deprecation**

  FIX 4.2 Order Entry Gateway will be deprecated on **June 3rd, 2025**. For FIX based order entry, leverage the newer, more performant [FIX 5 Order Entry Gateway](/exchange/fix-api/order-entry-messages/order-entry-messages5).
</Warning>

<Info>
  By accessing the Exchange Market Data API, you agree to be bound by the [Market Data Terms of Use](https://www.coinbase.com/legal/market_data).
</Info>

---

## Getting Started

1. **Create a Coinbase Exchange account** if you don't have one
2. **Generate API keys** from the Exchange dashboard
3. **Authenticate** your requests using JWT (recommended) or API key
4. **Make your first request** to the market data API

---

## API Categories

### Trading APIs (Authenticated)

| Category | Description |
|----------|-------------|
| Accounts | Get account balances, ledger, holds |
| Orders | Place, cancel, edit orders |
| Fills | Get order execution details |
| Deposits/Withdrawals | Transfer funds |
| Profiles | Manage trading profiles |

### Market Data APIs (Public)

| Category | Description |
|----------|-------------|
| Products | Get available trading pairs |
| Product Book | Get order book (bids/asks) |
| Product Candles | Get historical OHLCV data |
| Product Tickers | Get 24h stats |
| Market Trades | Get recent trades |
| Server Time | Get exchange time |

---

## Base URLs

| Environment | URL |
|-------------|-----|
| Production | `https://api.coinbase.com` |
| Advanced Trade | `https://api.coinbase.com/api/v3/brokerage` |

---

## Quick Example: Get Market Data

```bash
# Get list of products
curl https://api.coinbase.com/api/v3/brokerage/products

# Get product candles (OHLCV)
curl "https://api.coinbase.com/api/v3/brokerage/products/BTC-USD/candles?granularity=ONE_MINUTE"
```

---

## Quick Example: Place Order (Authenticated)

```python
import requests

# Create order
response = requests.post(
    "https://api.coinbase.com/api/v3/brokerage/orders",
    headers={
        "Authorization": f"Bearer {jwt_token}",
        "Content-Type": "application/json"
    },
    json={
        "side": "BUY",
        "product_id": "BTC-USD",
        "order_type": "MARKET",
        "size": "0.01"
    }
)
print(response.json())
```
