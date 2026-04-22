# HyperEVM

HyperEVM 包含集成到 Hyperliquid 执行层的 EVM 区块，从 HyperBFT 共识获得安全性。HYPE 作为原生 gas 代币。要将 HYPE 从 HyperCore 桥接到 HyperEVM，请将其发送到 `0x2222222222222222222222222222222222222222`。详细说明请参见[原生转账](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/hypercore-less-than-greater-than-hyperevm-transfers)。

目前，不存在官方的 EVM 前端组件。开发者可以创建自定义前端或适配现有的 EVM 应用。所有 EVM 交互通过 JSON-RPC 进行。用户可以使用 RPC URL 和链 ID 将链集成到钱包中。`rpc.hyperliquid.xyz/evm` 的 RPC 端点缺乏 websocket 支持，尽管替代实现可能提供它。

HyperEVM 实现了不带 blob 的 Cancun 硬分叉，启用了 EIP-1559。基础费用通过标准机制销毁，将其从 EVM 总供应量中移除。与典型的 EVM 链不同，由于 HyperBFT 共识，优先费用也被销毁，销毁的优先费用被导向零地址的 EVM 余额。

HYPE 在主网和测试网上都保持 18 位小数。环境之间的主要区别：

### 主网

- 链 ID: 999
- JSON-RPC 端点: `https://rpc.hyperliquid.xyz/evm`

### 测试网

- 链 ID: 998
- JSON-RPC 端点: `https://rpc.hyperliquid-testnet.xyz/evm`
