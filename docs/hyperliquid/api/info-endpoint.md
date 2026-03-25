# Info Endpoint

## Pagination

Responses with time ranges return a maximum of 500 elements or data blocks. For larger ranges, use the last returned timestamp as the next `startTime` parameter.

## Perpetuals vs Spot

These endpoints work for both Perpetuals and Spot markets. For perpetuals, `coin` is the name from the `meta` response. For Spot, use `PURR/USDC` for PURR, or `@{index}` format for other tokens (e.g., `@107` for HYPE on mainnet). Some assets may be remapped on user interfaces—check the L1 name on the token details page to detect remappings.

## User Address

Query account data by passing the actual address of the master or sub-account. Using an agent wallet's address will return empty results.

## Retrieve Mids for All Coins

**POST** `https://api.hyperliquid.xyz/info`

If the book is empty, the last trade price serves as a fallback.

**Headers:** Content-Type: "application/json"

**Request Body:**
- `type` (required): "allMids"
- `dex` (optional): Perp dex name; defaults to first perp dex. Spot mids only included with first perp dex.

**Response:** Object with coin symbols as keys and mid prices as values.

## Retrieve User's Open Orders

**POST** `https://api.hyperliquid.xyz/info`

**Headers:** Content-Type: "application/json"

**Request Body:**
- `type` (required): "openOrders"
- `user` (required): 42-character hex address
- `dex` (optional): Perp dex name; defaults to first perp dex

**Response:** Array of order objects with fields: `coin`, `limitPx`, `oid`, `side`, `sz`, `timestamp`

## Retrieve User's Open Orders with Frontend Info

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "frontendOpenOrders"
- `user` (required): 42-character hex address
- `dex` (optional): Perp dex name

**Response:** Array including additional fields: `isPositionTpsl`, `isTrigger`, `orderType`, `origSz`, `reduceOnly`, `triggerCondition`, `triggerPx`

## Retrieve User's Fills

**POST** `https://api.hyperliquid.xyz/info`

Returns at most 2000 most recent fills.

**Request Body:**
- `type` (required): "userFills"
- `user` (required): 42-character hex address
- `aggregateByTime` (optional): Combines partial fills when crossing orders match multiple resting orders

**Response:** Array of fill objects including: `closedPnl`, `coin`, `crossed`, `dir`, `hash`, `oid`, `px`, `side`, `startPosition`, `sz`, `time`, `fee`, `feeToken`, `tid`

## Retrieve User's Fills by Time

**POST** `https://api.hyperliquid.xyz/info`

Returns at most 2000 fills per response; only 10,000 most recent fills available.

**Request Body:**
- `type` (required): "userFillsByTime"
- `user` (required): 42-character hex address
- `startTime` (required): Start time in milliseconds (inclusive)
- `endTime` (optional): End time in milliseconds (inclusive); defaults to current time
- `aggregateByTime` (optional): Combines partial fills

## Query User Rate Limits

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type`: "userRateLimit"
- `user`: 42-character hex address

**Response:** Object with `cumVlm`, `nRequestsUsed`, `nRequestsCap`, `nRequestsSurplus`

## Query Order Status by OID or CLOID

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "orderStatus"
- `user` (required): 42-character hex address
- `oid` (required): u64 order ID or 16-byte hex string (client order ID)

**Order Status Values:**
- `open`: Placed successfully
- `filled`: Filled
- `canceled`: Canceled by user
- `triggered`: Trigger order triggered
- `rejected`: Rejected at placement
- `marginCanceled`: Insufficient margin
- `vaultWithdrawalCanceled`: Vault withdrawal cancellation
- `openInterestCapCanceled`: Too aggressive at cap
- `selfTradeCanceled`: Self-trade prevention
- `reduceOnlyCanceled`: Doesn't reduce position
- `siblingFilledCanceled`: TP/SL sibling filled
- `delistedCanceled`: Asset delisted
- `liquidatedCanceled`: Liquidation
- `scheduledCancel`: Exceeded deadline
- `tickRejected`: Invalid tick price
- `minTradeNtlRejected`: Below minimum notional
- `perpMarginRejected`: Insufficient margin
- `reduceOnlyRejected`: Reduce only violation
- `badAloPxRejected`: Post-only immediate match
- `iocCancelRejected`: IOC no match
- `badTriggerPxRejected`: Invalid TP/SL price
- `marketOrderNoLiquidityRejected`: Insufficient liquidity
- `positionIncreaseAtOpenInterestCapRejected`: At cap
- `positionFlipAtOpenInterestCapRejected`: At cap
- `tooAggressiveAtOpenInterestCapRejected`: Price too aggressive
- `openInterestIncreaseRejected`: At cap
- `insufficientSpotBalanceRejected`: Insufficient balance
- `oracleRejected`: Price too far from oracle
- `perpMaxPositionRejected`: Exceeds margin tier limit

## L2 Book Snapshot

**POST** `https://api.hyperliquid.xyz/info`

Returns at most 20 levels per side.

**Request Body:**
- `type` (required): "l2Book"
- `coin` (required): Asset symbol
- `nSigFigs` (optional): Aggregate to significant figures (2, 3, 4, 5, or null for full precision)
- `mantissa` (optional): Only with nSigFigs=5; accepts 1, 2, or 5

**Response:** Object with `coin`, `time`, and `levels` array (bid/ask sides with `px`, `sz`, `n`)

## Candle Snapshot

**POST** `https://api.hyperliquid.xyz/info`

Only 5000 most recent candles available.

**Supported Intervals:** 1m, 3m, 5m, 15m, 30m, 1h, 2h, 4h, 8h, 12h, 1d, 3d, 1w, 1M

**Request Body:**
- `type` (required): "candleSnapshot"
- `req` (required): Object with `coin`, `interval`, `startTime`, `endTime` (all in milliseconds)

**Response:** Array of candle objects with: `T` (close time), `c` (close), `h` (high), `i` (interval), `l` (low), `n` (trades), `o` (open), `s` (symbol), `t` (open time), `v` (volume)

## Check Builder Fee Approval

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "maxBuilderFee"
- `user` (required): 42-character hex address
- `builder` (required): 42-character hex address

**Response:** Maximum fee approved in tenths of a basis point (1 = 0.001%)

## Retrieve User's Historical Orders

**POST** `https://api.hyperliquid.xyz/info`

Returns at most 2000 most recent historical orders.

**Request Body:**
- `type` (required): "historicalOrders"
- `user` (required): 42-character hex address

**Response:** Array of order objects with `order`, `status`, `statusTimestamp`

## Retrieve User's TWAP Slice Fills

**POST** `https://api.hyperliquid.xyz/info`

Returns at most 2000 most recent TWAP slice fills.

**Request Body:**
- `type` (required): "userTwapSliceFills"
- `user` (required): 42-character hex address

**Response:** Array with `fill` object and `twapId`

## Retrieve User's Subaccounts

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "subAccounts"
- `user` (required): 42-character hex address

**Response:** Array of subaccount objects with name, address, master, and account state

## Retrieve Vault Details

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "vaultDetails"
- `vaultAddress` (required): 42-character hex address
- `user` (optional): 42-character hex address

**Response:** Vault object with name, leader, description, portfolio history, APR, followers, and relationship data

## Retrieve User's Vault Deposits

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userVaultEquities"
- `user` (required): 42-character hex address

**Response:** Array of vault equity objects with `vaultAddress` and `equity`

## Query User's Role

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userRole"
- `user` (required): 42-character hex address

**Response:** Role object—possible values: "missing", "user", "agent", "vault", or "subAccount"

## Query User's Portfolio

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "portfolio"
- `user` (required): 42-character hex address

**Response:** Array with time periods (day, week, month, allTime, perpDay, perpWeek, perpMonth, perpAllTime) containing account value history, PnL history, and volume

## Query User's Referral Information

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "referral"
- `user` (required): 42-character hex address

**Response:** Object with referrer info, cumulative volume, claimed/unclaimed rewards, builder rewards, and referrer state

## Query User's Fees

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userFees"
- `user` (required): 42-character hex address

**Response:** Object with daily volume, fee schedule, user rates, referral discount, trial info, and staking discount

## Query User's Staking Delegations

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegations"
- `user` (required): 42-character hex address

**Response:** Array of delegation objects with validator, amount, and lock-up timestamp

## Query User's Staking Summary

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegatorSummary"
- `user` (required): 42-character hex address

**Response:** Object with delegated, undelegated, total pending withdrawal, and pending withdrawal count

## Query User's Staking History

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegatorHistory"
- `user` (required): 42-character hex address

**Response:** Array of history objects with time, hash, and delta (delegate/undelegate info)

## Query User's Staking Rewards

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "delegatorRewards"
- `user` (required): 42-character hex address

**Response:** Array of reward objects with time, source (delegation/commission), and total amount

## Query User's HIP-3 DEX Abstraction State

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userDexAbstraction"
- `user` (required): 42-character hex address

**Response:** Boolean

## Query User's Abstraction State

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "userAbstraction"
- `user` (required): 42-character hex address

**Response:** String—possible values: "unifiedAccount", "portfolioMargin", "disabled", "default", "dexAbstraction"

## Query Aligned Quote Token Status

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "alignedQuoteTokenInfo"
- `token` (required): Token index

**Response:** Object with alignment status, first aligned time, EVM minted supply, daily amounts owed, and predicted rate

## Query Borrow/Lend User State

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "borrowLendUserState"
- `user` (required): 42-character hex address

**Response:** Object with token states (borrow/supply basis and value), health status, and health factor

## Query Borrow/Lend Reserve State

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "borrowLendReserveState"
- `token` (required): Token index

**Response:** Object with borrow/supply yearly rates, balance, utilization, oracle price, LTV, total supplied, and total borrowed

## Query All Borrow/Lend Reserve States

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "allBorrowLendReserveStates"

**Response:** Array of token index and reserve state pairs

## Query Approved Builders for User

**POST** `https://api.hyperliquid.xyz/info`

**Request Body:**
- `type` (required): "approvedBuilders"
- `user` (required): 42-character hex address

**Response:** Array of builder addresses
