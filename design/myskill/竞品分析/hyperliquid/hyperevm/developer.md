# HyperEVM

The HyperEVM comprises EVM blocks integrated into Hyperliquid's execution layer, drawing security from HyperBFT consensus. HYPE serves as the native gas token. To bridge HYPE from HyperCore to HyperEVM, send it to `0x2222222222222222222222222222222222222222`. Refer to [Native Transfers](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/hyperevm/hypercore-less-than-greater-than-hyperevm-transfers) for detailed instructions.

Currently, no official EVM frontend components exist. Developers can create custom frontends or adapt existing EVM applications. All EVM interaction occurs through JSON-RPC. Users can integrate the chain into wallets using the RPC URL and chain ID. The RPC endpoint at `rpc.hyperliquid.xyz/evm` lacks websocket support, though alternative implementations may provide it.

HyperEVM implements the Cancun hardfork without blobs, with EIP-1559 enabled. Base fees are burned through standard mechanisms, removing them from total EVM supply. Unlike typical EVM chains, priority fees are also burned due to HyperBFT consensus, with burned priority fees directed to the zero address's EVM balance.

HYPE maintains 18 decimals on both mainnet and testnet. Key differences between environments:

### Mainnet

- Chain ID: 999
- JSON-RPC endpoint: `https://rpc.hyperliquid.xyz/evm`

### Testnet

- Chain ID: 998
- JSON-RPC endpoint: `https://rpc.hyperliquid-testnet.xyz/evm`
