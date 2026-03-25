# Permissionless Spot Quote Assets

Becoming a spot quote asset on Hyperliquid is open to anyone, provided the following requirements are met:

## Technical Requirements

- Wei decimals of 8, size decimals of 2
- Zero deployer fee share on the quote token

## Staking Requirement

200k HYPE must be staked, with slashing conditions based on validator voting:

**Peg mechanism (1 USD):**
- QUOTE/USDC needs 100k USDC size on both sides within the 0.998–1.002 price range
- QUOTE/USDC needs 1M USDC size on both sides within 0.99–1.01

**Liquid HYPE/QUOTE book:**
- HYPE/QUOTE needs 50k QUOTE size on both sides within a 0.5% spread

USDC and USDT are exempt from the staking requirement given their established track record.

## Stake Commitment

The 200k HYPE is locked for 3 years from deployment, after which it can be unstaked. Slashing is subject to validator vote if quality conditions are violated.

A condition is considered violated if it goes unmet for a majority of 1-second samples over any 3-day window.

## Testnet

On testnet, the staking requirement is reduced to 50 HYPE. Once requirements are satisfied, the deployer sends an `enableQuoteToken` transaction — irreversible, no gas cost.

New account transfer fees can be paid in 1 unit of any spot quote asset.
