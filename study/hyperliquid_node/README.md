# Hyperliquid Mainnet 非验证节点

这是一个独立的 Docker 部署工具包，不在 Rust crate 中实现节点业务。镜像构建时从官方 `binaries.hyperliquid.xyz` 下载与版本匹配的 `hl-visor`，并用官方公钥验证 detached signature；容器入口使用官方 `hl-visor run-non-validator`。

首次运行：

```bash
cd study/hyperliquid_node
./run_node.sh
```

脚本会从 `default.env` 初始化未跟踪的 `.env`。默认从链目录直接下载官方当前 `hl-visor`；如官方发布了版本子目录，可设置 `HL_VERSION` 使用该目录。在 Mainnet 同步前，建议将 `MAINNET_ROOT_IPS` 填入官方 `gossipRootIps` 返回的 seed peers；也可设置 `TESTNET_ROOT_IPS`、`RESERVED_PEER_IPS` 和 `EXTRA_FLAGS`。不要配置 validator private key。

默认开启 Info API、EVM RPC、trades、fills、raw book diffs 和 order statuses。端口分别为 Info `3000`、EVM RPC `3001`、P2P TCP/UDP `4000-4010`；可在 `.env` 修改。

Compose 以 root 启动 visor 是有意的：官方 visor 的签名自更新流程会替换
`/usr/local/bin/hl-node`，普通用户无法完成该 rename。节点数据仍限制在挂载的
`runtime/hl` 和 `runtime/config` 目录中。

常用操作：

```bash
./status_node.sh
./inspect_node_data.sh
./stop_node.sh
```

## 获取 replica_cmds 元始块

Rust 客户端要求调用方显式传入 `hl_getBlock` RPC URL。下面的命令请求
`block_number=0`，在终端打印完整 block，并把完整 JSON-RPC 响应写入指定文件：

```bash
cargo run -p hyperliquid_node --bin fetch_genesis_block -- \
  http://127.0.0.1:3000/replica_cmds \
  /tmp/hyperliquid_block_0.json
```

`replica_cmds` 是节点 RPC endpoint，官方公共 `https://rpc.hyperliquid.xyz` 当前不提供此路径；
请使用自建节点地址或确认过的第三方 replica endpoint。调用方可直接替换 URL；客户端内部不固定网络。代码调用使用
`replica_rpc::get_block(rpc_url, 0)`，返回值同时提供完整原始 JSON、原始 `result`
块和 `ReplicaBlock` 强类型访问器。价格与数量保持为字符串，避免浮点精度损失。

数据只写入 `runtime/hl/hyperliquid_data`（RocksDB/节点状态）、`runtime/hl/data`（导出数据）和 `runtime/config`（生成的 gossip 配置），均已被 Git 忽略。停止容器不会删除这些目录，重新运行会复用已有状态。
