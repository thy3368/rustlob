# Aligned quote assets

The Hyperliquid protocol will support "aligned stablecoins" as a permissionless primitive for stablecoin issuers to leverage Hyperliquid's unique distribution and scale together with the protocol. Aligned stablecoins offer lower trading fees, better market maker rebates, and higher volume contribution toward fee tiers when used as the quote asset for a spot pair or the collateral asset for HIP-3 perps.

Hyperliquid will continue to support a wide variety of permissionless quote assets for spot and perps trading. There will be continual technical developments to ensure that the Hyperliquid L1 is the most performant infrastructure for general purpose asset issuance, liquidity, and building.

To be clear, the motivation behind alignment is not to exclude any issuers, but rather to introduce an opt-in setting for new stablecoin teams to bootstrap their network effects and share upside proportionally with the protocol. Aligned stables and other assets serve different purposes and audiences, and will coexist and complement each other. Similar to the builder-protocol synergy of permissionless spot listings, HIP-3, and builder codes, aligned stablecoins are part of the infrastructure to move all of finance onchain.

**Aligned stable benefits, applied to spot and perp trading:**

1. 20% lower taker fees
2. 50% better maker rebates
3. 20% more volume contribution toward fee tiers

Offchain conditions are ultimately voted upon by validator quorum, as any such conditions are not able to be reflected directly in protocol execution. Like on most other blockchains, independent validators on Hyperliquid achieve consensus on a self-contained state machine's execution. This state machine's evolution is entirely onchain. In the case of the offchain conditions for an aligned stablecoin, this evolution is driven by validator vote.

The following reflect views expressed by Hyperliquid Labs after careful consideration about the best outcome for the protocol and users.

**Onchain requirements:**

1. Enabled as a permissionless quote token
2. 800k additional staked HYPE by deployer, meaning a total of 1M staked HYPE including the 200k staked HYPE for the quote token deployment. This is to give builders and users assurance to use the aligned stablecoin.
3. 50% of the deployer's offchain reserve income must flow to the protocol. Validators may vote to update the calculation methodology as regulatory standards evolve. There will be follow-up work on the precise definition of risk-free rate, which will be updated according to an onchain stake-weighted median of validator reported values. A CoreWriter action will allow the deployer to reflect the exact minted balance from HyperEVM directly to HyperCore, which will allow a fully automated fee share mechanism as part of L1 execution.

**Offchain requirements, enforced through onchain quorum of validator votes:**

1. The stablecoin is 1:1 backed by cash, short-term US treasuries, and tokenized US treasury or money market funds to the extent permitted under applicable regulatory frameworks. Aligned issuers must also provide par redemption at all times, with a publicly disclosed and timely redemption service consistent with their applicable regulatory regime. These conditions can be revisited by the validators, in the spirit of building a regulatorily compliant chain for payments and banking opportunities. The guiding requirement is that a large percentage of the world's circulating dollars could compliantly be converted to the aligned stablecoin in the context of existing businesses and use cases in the financial world.
2. The full supply is natively minted on HyperEVM. Any supply on other chains or offchain must first be minted on HyperEVM as the source chain.
3. The deployer can only deploy assets that directly support the aligned stablecoin. For example, the underlying treasuries could be issued onchain. The net effect is that the deployer must share half of its offchain yield income through the existence of the aligned stablecoin. The deployer and its affiliates may not receive any economic benefits tied to conversion of the aligned stablecoin into another asset. "Benefit" includes but is not limited to revenue share, order-flow payments or any form of rate-linked compensation.
4. The team building an aligned stablecoin must be independent and dedicated to building on Hyperliquid.

## FAQ

**1. Offchain requirements are overly restrictive. The protocol should only enforce strictly onchain requirements such as staking requirements and yield share.**

Onchain requirements are almost always preferable to offchain ones. They are simpler, objective, and do not require validator enforcement. However, the real world is inherently nuanced and complex. Given the opportunity size of becoming the premier stablecoin chain and the difficulty with associated yield being fully offchain, the protocol must compromise with a system that accomplishes the goal of true alignment. The only obvious way to accomplish this goal is through validator quorum enforcing offchain conditions. That being said, the feedback is duly noted that conditions should be as simple as possible while accomplishing these goals.

**2. The requirements are too strict and will dampen the quality of projects ready to immediately deploy on Hyperliquid.**

Two responses. Firstly, the benefits of aligned stablecoins are substantial but by no means a requirement for a successful stablecoin deployment. Furthermore, many stablecoins that may not qualify for alignment will naturally have their own incentivization opportunities coming out of a much higher top-line yield. The opportunity exists for many stable assets to thrive and synergize. Secondly, even if a project insists on "aligned or nothing" and deprioritizes deployment on Hyperliquid as a result, the tradeoff can still be worthwhile for the protocol. The sheer size of the stablecoin opportunity as part of housing all finance is worth more than any short term metric boosts such as trading volume or TVL incentivized by specific stablecoin deployers.

**3. Users will naturally choose the most aligned stablecoins, so the offchain conditions are not necessary.**

While this would be true in an ideal state of the world, it's important to be realistic about the probability of it playing out. Such an outcome depends on 1) competent deployers choosing to remain aligned with the protocol and 2) users doing research, correctly identifying the most protocol-aligned stablecoin, and actively choosing to use it. Neither of these conditions are guaranteed. The protocol unfortunately does not have the luxury of experimentation here, and given the size of the opportunity, it would be too risky to leave this level of uncertainty in the outcome. Any aligned stable that achieves massive success will owe its initial distribution to the protocol. It is only fair that deployers seeking this benefit should recognize and commit upfront to sharing back with the protocol and community.

**4. The requirements kill the prospect of alternative stablecoins.**

This is not the intention and should have been clearer in the first draft of the proposal. The projected market for regulated stablecoins is orders of magnitude larger than that for alternative stablecoins. Of course, there is no guarantee on this outcome, but much of Hyperliquid's success has come from building infrastructure with real-world, practical context. Furthermore, alternative stablecoins usually have different yield characteristics that can offset the lack of trading benefits from alignment.
