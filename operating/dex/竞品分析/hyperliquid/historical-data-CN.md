# 历史数据

## 导出额外的用户交易历史

Enigma 团队在 [trade-export.hypedexer.com](https://trade-export.hypedexer.com/?v=1) 构建了一个交易历史导出界面。这是一个独立维护的第三方集成 — 将任何问题或反馈直接反馈给维护者。

## 市场数据（适用于高级用户）

以下示例使用 [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) 和 [LZ4](https://github.com/lz4/lz4)。注意请求者支付数据传输成本。

### 资产数据

历史数据大约每月上传到 `hyperliquid-archive` 存储桶一次。"不保证及时更新，数据可能缺失。"

- L2 订单簿快照：`market_data`
- 资产上下文：`asset_ctxs`

格式：
```
s3://hyperliquid-archive/market_data/[date]/[hour]/[datatype]/[coin].lz4
s3://hyperliquid-archive/asset_ctxs/[date].csv.lz4
```

示例用法：
```bash
aws s3 cp s3://hyperliquid-archive/market_data/20230916/9/l2Book/SOL.lz4 /tmp/SOL.lz4 --request-payer requester
unlz4 --rm /tmp/SOL.lz4
head /tmp/SOL
```

### 交易数据

`s3://hl-mainnet-node-data/node_fills_by_block` 包含从非验证节点通过 `--write-fills --batch-by-block` 流式传输的成交。旧数据位于：

- `node_fills` — 匹配 API 格式
- `node_trades` — 不同格式

### 历史节点数据

- `s3://hl-mainnet-node-data/explorer_blocks` — 历史浏览器区块
- `s3://hl-mainnet-node-data/replica_cmds` — L1 交易
