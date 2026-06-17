# Risks

### Smart contract risk

Hyperliquid's onchain perp DEX depends on the Arbitrum bridge smart contracts — bugs or vulnerabilities could result in loss of user funds.

### L1 risk

Hyperliquid runs on its own L1, which hasn't had the same level of testing as established chains like Ethereum. Network downtime due to consensus issues is possible.

### Market liquidity risk

As a newer protocol, low liquidity could cause significant price slippage, negatively affecting traders and potentially leading to losses.

### Oracle manipulation risk

Price oracles are maintained by validators. If an oracle is "compromised or manipulated for an extended period of time, the mark price could be affected and liquidations could occur" before prices revert.

### Risk mitigation

Measures to counter oracle manipulation on less liquid assets include:

- Open interest caps (based on liquidity, basis, and leverage)
- When the cap is hit, no new positions can be opened
- Orders cannot rest further than 1% from the oracle price
- HLP is exempt from these restrictions to continue providing liquidity

> Note: this is not an exhaustive list of potential risks.
