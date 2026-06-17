# 多签

HyperCore 支持原生多签操作，使多个私钥能够控制单个账户。与其他链不同，这是内置原语而非智能合约功能。

## 工作流程

- 要转换用户，发送 `ConvertToMultiSigUser` 操作，包含授权用户和最低签名阈值。授权用户必须已存在于 Hyperliquid 上。转换后，所有操作必须通过多签进行。
- 每个授权用户对有效负载签名以产生签名。`MultiSig` 操作包装任何正常操作并包含收集的签名。
- `MultiSig` 有效负载包括目标多签用户和发送交易的授权用户 — 称为 `leader`。
  - 仅验证和更新 leader 的 nonce。
  - leader 也可以是授权用户的 API 钱包；然后检查该钱包的 nonce。
- 要更新授权用户或阈值，发送包装新 `ConvertToMultiSigUser` 操作的 `MultiSig`。
- 要恢复为普通用户，通过多签发送 `ConvertToMultiSigUser`，授权用户集为空。

## 注意事项

- leader 必须是授权用户，而非多签账户本身。
- "每个签名必须使用相同的信息，例如相同的 nonce、交易 lead 地址。"
- leader 必须在提交前收集所有签名。
- 用户可以同时是多签用户和其他多签账户的授权用户。每个账户最多授权用户：10 个。

## HyperEVM 警告

转换为多签后，HyperEVM 用户仍可由原始钱包控制。CoreWriter 对多签用户无效。多签用户在转换前后应避免 HyperEVM 交互。
