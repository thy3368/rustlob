# Staking

## Basics

HYPE staking on Hyperliquid operates within HyperCore. HYPE can be moved between spot and staking accounts, similar to how USDC moves between perps and spot.

Within the staking account, HYPE can be delegated to any number of validators. "Delegate" and "stake" are used interchangeably since Hyperliquid only supports delegated proof of stake.

Key rules for validators:
- 10k HYPE self-delegation required to become active, locked for one year
- Dropping below 10k HYPE puts the validator in undelegate-only mode
- Commission increases are capped unless the new rate is ≤ 1%

Delegation lockup is 1 day, after which you can undelegate partially or fully at any time. Undelegated balances reflect instantly in the staking account.

Transfers from spot → staking are instant. Staking → spot transfers enter a 7-day unstaking queue, with a max of 5 pending withdrawals per address.

The reward rate is "inversely proportional to the square root of total HYPE staked." At 400M staked, that's roughly 2.37%/year. Rewards accrue every minute, distribute daily, and are auto-compounded to the staked validator.

## Technical Details

A *quorum* is any validator set holding more than ⅔ of total network stake — the foundation of HyperBFT consensus.

Consensus proceeds in *rounds* (bundles of transactions + validator signatures). Rounds that meet commit conditions get sent to execution state. Execution blocks use a separate *height* counter that only increments when a round contains at least one transaction.

The validator set updates in epochs of 100k rounds (~90 minutes on mainnet).

Validators can vote to *jail* peers with poor latency or responsiveness. A jailed validator stops participating in consensus and earns no rewards for delegators. Unjailing is possible but subject to on-chain rate limits. Jailing differs from slashing — slashing is reserved for provably malicious behavior like double-signing.
