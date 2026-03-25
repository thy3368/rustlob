# Overview

## Consensus

Hyperliquid relies on HyperBFT, a HotStuff consensus variant. Like proof of stake systems generally, validators produce blocks proportional to their staked native tokens.

## Execution

Hyperliquid's state combines HyperCore and the general-purpose HyperEVM. HyperCore manages margin and matching engine operations without depending on off-chain order books. The design prioritizes full decentralization through consistent transaction ordering via HyperBFT consensus.

## Latency

HyperBFT optimizes for end-to-end latency—the time between sending a request and receiving a committed response. For geographically proximate clients, median latency reaches 0.2 seconds with a 99th percentile of 0.9 seconds. This performance enables users to migrate automated strategies from other platforms with minimal adaptation and provides retail users with immediate UI feedback.

## Throughput

Mainnet currently handles roughly 200,000 orders per second. Execution represents the primary constraint; the consensus algorithm and networking infrastructure could theoretically scale to millions of orders per second once execution improves. Future optimization efforts will address execution logic as demand increases.
