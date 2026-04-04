# Exchange Endpoint

## Asset

For perpetuals, use the index from the `universe` field in the `meta` response. For spot assets, use `10000 + index` where index comes from `spotMeta.universe`. Example: PURR/USDC uses asset `10000` since its spot index is `0`.

## Subaccounts and Vaults

Subaccounts and vaults lack private keys. The master account must sign actions, with the `vaultAddress` field set to the subaccount or vault address.

## Expires After

Optional field for some actions—a timestamp in milliseconds after which the action gets rejected. User-signed actions like Core USDC transfers don't support this. Actions canceled due to stale `expiresAfter` consume 5x the normal rate limit.

## Place an Order

`POST https://api.hyperliquid.xyz/exchange`

Limit orders use TIF (time-in-force): ALO (post only), IOC (immediate or cancel), or GTC (good til canceled). Client Order ID (cloid) is optional—a 128-bit hex string.

**Headers:** Content-Type: "application/json"

**Request Body:**
- `action` (required): Order object with type, orders array, grouping, optional builder
- `nonce` (required): Current timestamp in milliseconds
- `signature` (required)
- `vaultAddress` (optional): 42-character hex address
- `expiresAfter` (optional): Timestamp in milliseconds

**Responses:** Success returns order status (resting/filled) with OID, or error message.

## Cancel Order(s)

`POST https://api.hyperliquid.xyz/exchange`

**Request Body:**
- `action`: Cancel type with asset and order ID
- `nonce`, `signature`, optional `vaultAddress` and `expiresAfter`

## Cancel by CLOID

`POST https://api.hyperliquid.xyz/exchange`

Cancel using client order ID instead of order ID.

## Schedule Cancel (Dead Man's Switch)

`POST https://api.hyperliquid.xyz/exchange`

Schedule all orders to cancel at a future time. Minimum 5 seconds ahead. Max 10 triggers per day (reset at 00:00 UTC).

## Modify an Order

`POST https://api.hyperliquid.xyz/exchange`

Modify existing order by OID or CLOID with new parameters.

## Modify Multiple Orders

`POST https://api.hyperliquid.xyz/exchange`

Batch modify multiple orders in one request.

## Update Leverage

`POST https://api.hyperliquid.xyz/exchange`

Update cross or isolated leverage on a coin.

## Update Isolated Margin

`POST https://api.hyperliquid.xyz/exchange`

Add or remove margin from isolated positions. Alternate action available to target specific leverage instead of USDC value.

## Core USDC Transfer

`POST https://api.hyperliquid.xyz/exchange`

Send USD to another address without touching the EVM bridge. Signature format is human-readable for wallet interfaces.

## Core Spot Transfer

`POST https://api.hyperliquid.xyz/exchange`

Send spot assets to another address. Requires token in "name:id" format.

## Initiate Withdrawal Request

`POST https://api.hyperliquid.xyz/exchange`

Start withdrawal flow. $1 fee, ~5 minutes to finalize.

## Transfer Spot to Perp (and Vice Versa)

`POST https://api.hyperliquid.xyz/exchange`

Transfer USDC between spot and perp wallets.

## Send Asset

`POST https://api.hyperliquid.xyz/exchange`

Generalized token transfer between perp DEXs, spot, users, and subaccounts. Use "" for default USDC perp DEX, "spot" for spot.

## Send to EVM with Data

`POST https://api.hyperliquid.xyz/exchange`

Core to EVM transfer with data payload. Requires linked contract supporting `coreReceiveWithData` interface.

## Deposit into Staking

`POST https://api.hyperliquid.xyz/exchange`

Transfer native token from spot to staking for validator delegation.

## Withdraw from Staking

`POST https://api.hyperliquid.xyz/exchange`

Transfer native token from staking to spot. 7-day unstaking queue applies.

## Delegate or Undelegate Stake

`POST https://api.hyperliquid.xyz/exchange`

Delegate/undelegate tokens to/from validators. 1-day lockup per validator.

## Deposit or Withdraw from Vault

`POST https://api.hyperliquid.xyz/exchange`

Add or remove funds from a vault.

## Approve an API Wallet

`POST https://api.hyperliquid.xyz/exchange`

Approve API Wallet (Agent Wallet). One unnamed plus up to three named per account; two additional per subaccount.

## Approve Builder Fee

`POST https://api.hyperliquid.xyz/exchange`

Set maximum fee rate for a builder.

## Place a TWAP Order

`POST https://api.hyperliquid.xyz/exchange`

Time-weighted average price order with randomization option.

## Cancel a TWAP Order

`POST https://api.hyperliquid.xyz/exchange`

Cancel existing TWAP by asset and TWAP ID.

## Reserve Additional Actions

`POST https://api.hyperliquid.xyz/exchange`

Reserve additional actions for 0.0005 USDC per request from Perps balance.

## Invalidate Pending Nonce (noop)

`POST https://api.hyperliquid.xyz/exchange`

No-operation that marks nonce as used—effective for canceling in-flight orders.

## Enable HIP-3 DEX Abstraction

`POST https://api.hyperliquid.xyz/exchange`

(Deprecated—use `userSetAbstraction`) Auto-transfer collateral from validator USDC perps or spot.

## Set User Abstraction

`POST https://api.hyperliquid.xyz/exchange`

Set abstraction mode: "disabled", "unifiedAccount", or "portfolioMargin".

## Set User Abstraction (Agent)

`POST https://api.hyperliquid.xyz/exchange`

Agent version using abbreviated codes: "i" (disabled), "u" (unifiedAccount), "p" (portfolioMargin).

## Validator Vote on Risk-Free Rate

`POST https://api.hyperliquid.xyz/exchange`

Validators vote on risk-free rate for aligned quote asset.

## Claim Rewards

`POST https://api.hyperliquid.xyz/exchange`

Claim accumulated rewards.
