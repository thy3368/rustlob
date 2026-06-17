# HyperEVM

HyperEVM 是 Hyperliquid 区块链的一部分，与 HyperCore 并列，两者都由 HyperBFT 共识保护。这种共享安全模型允许 HyperEVM 与 HyperCore 的现货和永续订单簿直接交互。

## 在 HyperEVM 上可以做什么？

社区构建的应用和工具目录：
- [ASXN](https://hyperscreener.asxn.xyz/ecosystem)
- [HypurrCo](https://www.hypurr.co/ecosystem-projects)
- [HL Eco](https://hl.eco/projects)
- [Hyperliquid.wiki](https://hyperliquid.wiki/)

另请参见 [HyperEVM 入门 FAQ](https://hyperliquid.gitbook.io/hyperliquid-docs/onboarding/how-to-use-the-hyperevm)。

## 为什么要在 HyperEVM 上构建？

开发者可以通过 HyperCore + HyperEVM 访问成熟、流动的链上订单簿。主要用例包括：

- 部署与原生 HyperCore 现货资产关联的 ERC20 代币 — 无桥接风险，完全无需许可
- 通过读取预编译直接从 HyperCore 订单簿读取价格的借贷协议，以及通过写入系统合约执行清算

更多详情请参见 [HyperEVM 开发者部分](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm) 和 [开发者工具](https://hyperliquid.gitbook.io/hyperliquid-docs/hyperevm/tools-for-hyperevm-builders)。

## HyperEVM 目前处于什么阶段？

目前处于 alpha 阶段。逐步推出反映了三个优先事项：

1. 坚持 "无内幕" 原则 — 人人平等访问
2. 安全升级处理数十亿日交易量的系统
3. 基于真实经济使用而非测试网条件进行迭代

更高的吞吐量和写入系统合约尚未在主网上线。
