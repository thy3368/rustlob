# Bridge2

## General Information

The bridge between Hyperliquid and Arbitrum is located at `0x2df1c51e09aecf9cacb7bc98cb1742757f163df7`. The bridge code is available on GitHub at the Hyperliquid contracts repository.

## Deposit

Users send native USDC to the bridge, and it credits the sending account within less than 1 minute. The minimum deposit is 5 USDC; amounts below this threshold will not be credited and are permanently lost.

## Withdraw

Withdrawals require only a user wallet signature on Hyperliquid—no Arbitrum transaction needed. Validators handle the Arbitrum withdrawal entirely, with funds arriving in 3-4 minutes.

The withdrawal payload uses EIP-712 signing with these fields:

- `signatureChainId`: Chain identifier
- `hyperliquidChain`: Chain designation (Mainnet or Testnet)
- `destination`: Recipient address
- `amount`: Withdrawal amount
- `time`: Timestamp

The signed action includes the withdrawal details, a nonce matching the time field, and the signature components (r, s, v).

## Deposit with Permit

The bridge supports depositing on behalf of another user via `batchedDepositWithPermit`. Users sign a permit payload containing owner, spender, value, nonce, and deadline fields using EIP-712 typed data signing.

Bridge addresses:
- Mainnet: `0x2df1c51e09aecf9cacb7bc98cb1742757f163df7`
- Testnet: `0x08cfc1B6b2dCF36A1480b99353A354AA8AC56f89`
