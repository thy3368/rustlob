# Historical Data

## Exporting Additional User Trade History

The Enigma team has built a trade history export interface at [trade-export.hypedexer.com](https://trade-export.hypedexer.com/?v=1). It's a third-party integration maintained independently — direct any issues or feedback to the maintainers.

## Market Data (for advanced users)

Examples below use the [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) and [LZ4](https://github.com/lz4/lz4). Note that the requester pays for data transfer costs.

### Asset Data

Historical data is uploaded to the `hyperliquid-archive` bucket roughly once a month. "There is no guarantee of timely updates and data may be missing."

- L2 book snapshots: `market_data`
- Asset contexts: `asset_ctxs`

Format:
```
s3://hyperliquid-archive/market_data/[date]/[hour]/[datatype]/[coin].lz4
s3://hyperliquid-archive/asset_ctxs/[date].csv.lz4
```

Example usage:
```bash
aws s3 cp s3://hyperliquid-archive/market_data/20230916/9/l2Book/SOL.lz4 /tmp/SOL.lz4 --request-payer requester
unlz4 --rm /tmp/SOL.lz4
head /tmp/SOL
```

### Trade Data

`s3://hl-mainnet-node-data/node_fills_by_block` contains fills streamed via `--write-fills --batch-by-block` from a non-validating node. Older data lives in:

- `node_fills` — matches the API format
- `node_trades` — different format

### Historical Node Data

- `s3://hl-mainnet-node-data/explorer_blocks` — historical explorer blocks
- `s3://hl-mainnet-node-data/replica_cmds` — L1 transactions
