# HyperEVM

HyperEVM is part of the Hyperliquid blockchain alongside HyperCore, both secured by HyperBFT consensus. This shared security model allows direct interaction between HyperEVM and HyperCore's spot and perp order books.

## What can I do on the HyperEVM?

Community-built directories of apps and tools:
- [ASXN](https://hyperscreener.asxn.xyz/ecosystem)
- [HypurrCo](https://www.hypurr.co/ecosystem-projects)
- [HL Eco](https://hl.eco/projects)
- [Hyperliquid.wiki](https://hyperliquid.wiki/)

See also the [HyperEVM onboarding FAQ](https://hyperliquid.gitbook.io/hyperliquid-docs/onboarding/how-to-use-the-hyperevm).

## Why build on the HyperEVM?

Builders get access to mature, liquid onchain order books via HyperCore + HyperEVM. Key use cases include:

- Deploying ERC20 tokens linked to native HyperCore spot assets — no bridging risk, fully permissionless
- Lending protocols that read prices directly from HyperCore order books via a read precompile, and execute liquidations via a write system contract

More details in the [HyperEVM developer section](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm) and [tools for builders](https://hyperliquid.gitbook.io/hyperliquid-docs/hyperevm/tools-for-hyperevm-builders).

## What stage is the HyperEVM in?

Currently in alpha. The gradual rollout reflects three priorities:

1. Staying true to a "no insiders" principle — equal access for everyone
2. Safely upgrading a system handling billions in daily volume
3. Iterating on real economic usage rather than testnet conditions

Higher throughput and write system contracts are not yet live on mainnet.
