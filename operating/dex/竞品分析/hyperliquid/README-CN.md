# 关于 Hyperliquid

## Hyperliquid 是什么？

Hyperliquid 是一个高性能区块链，旨在构建一个完全链上的开放金融体系。流动性、用户应用和交易活动在一个统一的平台上协同作用，最终将成为整个金融领域的枢纽。

## 技术概览

Hyperliquid 是一个从零开始编写和优化的 layer one 区块链（L1）。

Hyperliquid 使用一种名为 HyperBFT 的自定义共识算法，其灵感来源于 Hotstuff 及其后续版本。该算法和网络栈都经过从零开始的优化，以满足 L1 的独特需求。

Hyperliquid 状态执行分为两个主要组件：HyperCore 和 HyperEVM。HyperCore 包括完全链上的永续期货和现货订单簿。每个订单、取消、交易和清算都在 HyperBFT 继承的单区块最终性下透明地进行。HyperCore 目前支持每秒 20 万订单的处理能力，随着节点软件的进一步优化，吞吐量还在不断提升。

HyperEVM 将以太坊开创的通用智能合约平台引入 Hyperliquid 区块链。通过 HyperEVM，HyperCore 的高性能流动性和金融原语可以作为无需许可的构建模块，供所有用户和开发者使用。更多技术细节请参见 HyperEVM 文档部分。
