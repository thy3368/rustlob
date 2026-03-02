# Coinbase Exchange API

> Source: https://docs.cdp.coinbase.com/api-reference/exchange-api/rest-api/introduction.md

---

## Overview

The Exchange Trading APIs allow institutions to place orders and access account information. This is the legacy Exchange API - for new integrations, use the Advanced Trade API.

## Base URL

```
https://api.exchange.coinbase.com
```

## Authentication

See [Authentication](./authentication.md) for details.

## Rate Limits

| Endpoint Type | Limit |
|--------------|-------|
| Standard | 10 requests/second |
| Orders | 15 requests/second |

## Pagination

Most endpoints support pagination with `before` and `after` parameters.

## Related Documentation

- [Authentication](https://docs.cdp.coinbase.com/exchange/rest-api/authentication)
- [Rate Limits](https://docs.cdp.coinbase.com/exchange/rest-api/rate-limits)
- [Pagination](https://docs.cdp.coinbase.com/exchange/rest-api/pagination)
- [Quickstart](https://docs.cdp.coinbase.com/exchange/introduction/rest-quickstart)

## Endpoints

### Accounts
- [Get All Account Profile](./accounts/get-all-account-profile.md)
- [Get Single Account By ID](./accounts/get-single-account-by-id.md)
- [Get Account Holds](./accounts/get-single-account-hold.md)
- [Get Account Ledger](./accounts/get-single-account-ledger.md)
- [Get Account Transfers](./accounts/get-single-account-transfer.md)

### Address Book
- [Add Addresses](./address-book/add-addresses.md)
- [Delete Address](./address-book/delete-address.md)
- [Get Address Book](./address-book/get-address-book.md)
- [Get Counterparty Address Book](./address-book/get-counterparty-address-book.md)
- [Update Address Book](./address-book/update-address-book.md)

### Coinbase Accounts
- [Generate Crypto Address](./coinbase-accounts/generate-crypto-address.md)
- [Get All Coinbase Wallets](./coinbase-accounts/get-all-coinbase-wallets.md)

### Conversions
- [Convert Currency](./conversions/convert-currency.md)
- [Get Conversion](./conversions/get-a-conversion.md)
- [Get All Conversions](./conversions/get-all-conversions.md)
- [Get Conversion Fee Rates](./conversions/get-conversion-fee-rates.md)

### Currencies
- [Get Currency](./currencies/get-a-currency.md)
- [Get All Known Currencies](./currencies/get-all-known-currencies.md)

### Fees
- [Get Fees](./fees/get-fee.md)

### Futures
- [Get Auto Loan Setting](./futures/get-auto-loan-setting.md)
- [Get USDC Conversion](./futures/get-usdc-conversion.md)
- [Set Auto Loan](./futures/set-auto-loan.md)
- [Set USDC Conversion](./futures/set-usdc-conversion.md)

### Loan
- [Get Lending Overview](./loan/get-lending-overview.md)
- [Get Lending Overview XM](./loan/get-lending-overview-xm.md)
- [Get Loan Preview XM](./loan/get-loan-preview-xm.md)
- [Get New Loan Preview](./loan/get-new-loan-preview.md)
- [Get Principal Repayment Preview](./loan/get-principal-repayment-preview.md)
- [Get Repayment Preview XM](./loan/get-repayment-preview-xm.md)
- [List Interest Charges](./loan/list-interest-charges.md)
- [List Interest Rate History](./loan/list-interest-rate-history.md)
- [List Interest Summaries](./loan/list-interest-summaries.md)
- [List Loan Assets](./loan/list-loan-assets.md)
- [List Loans](./loan/list-loans.md)
- [List New Loan Options](./loan/list-new-loan-options.md)
- [Open New Loan](./loan/open-new-loan.md)
- [Repay Loan Interest](./loan/repay-loan-interest.md)
- [Repay Loan Principal](./loan/repay-loan-principal.md)

### Orders
- [Cancel All Orders](./orders/cancel-all-orders.md)
- [Cancel An Order](./orders/cancel-an-order.md)
- [Create New Order](./orders/create-new-order.md)
- [Get All Fills](./orders/get-all-fills.md)
- [Get All Orders](./orders/get-all-orders.md)
- [Get Single Order](./orders/get-single-order.md)

### Products
- [Get All Known Trading Pairs](./products/get-all-known-trading-pairs.md)
- [Get All Product Volume](./products/get-all-product-volume.md)
- [Get Product Book](./products/get-product-book.md)
- [Get Product Candles](./products/get-product-candles.md)
- [Get Product Stats](./products/get-product-stats.md)
- [Get Product Ticker](./products/get-product-ticker.md)
- [Get Product Trades](./products/get-product-trades.md)
- [Get Single Product](./products/get-single-product.md)

### Profiles
- [Create Profile](./profiles/create-profile.md)
- [Delete Profile](./profiles/delete-profile.md)
- [Get Profile By ID](./profiles/get-profile-by-id.md)
- [Get Profiles](./profiles/get-profiles.md)
- [Rename Profile](./profiles/rename-profile.md)
- [Transfer Funds Profile](./profiles/transfer-funds-profile.md)
