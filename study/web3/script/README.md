# Reth Node Docker Setup

这是一个用于在 Docker 中运行 [Reth](https://github.com/paradigmxyz/reth) 以太坊执行层客户端的完整脚本。

## 什么是 Reth？

Reth 是由 Paradigm 开发的高性能以太坊执行层客户端，使用 Rust 编写，具有以下特点：
- **极致性能**：优化的数据库和同步机制
- **模块化设计**：易于扩展和定制
- **低资源消耗**：相比其他客户端占用更少资源
- **开发者友好**：提供丰富的 RPC API

## 系统要求

### 硬件要求

| 配置 | 最低要求 | 推荐配置 |
|------|---------|---------|
| CPU | 4 核 | 8+ 核 |
| 内存 | 16 GB | 32 GB+ |
| 存储 | 1 TB SSD | 2 TB NVMe SSD |
| 网络 | 25 Mbps | 100 Mbps+ |

### 软件要求

- Docker 20.10+
- Docker Compose 2.0+
- OpenSSL（用于生成 JWT）
- curl 和 jq（可选，用于 RPC 调用）

## 快速开始

### 1. 初始化设置

```bash
cd /Users/hongyaotang/src/rustlob/study/web3/script
./run-reth.sh init
```

这将：
- 生成 JWT secret（用于共识层通信）
- 创建 `.env` 配置文件
- 创建必要的目录

### 2. 配置网络

编辑 `.env` 文件选择网络：

```bash
# 主网（需要大量存储空间和同步时间）
CHAIN=mainnet

# Sepolia 测试网（推荐用于开发）
CHAIN=sepolia

# Holesky 测试网
CHAIN=holesky
```

### 3. 启动节点

```bash
./run-reth.sh start
```

### 4. 查看状态

```bash
./run-reth.sh status
```

### 5. 查看日志

```bash
./run-reth.sh logs
```

## 脚本命令

```bash
./run-reth.sh [command]
```

| 命令 | 说明 |
|------|------|
| `init` | 初始化设置（生成 JWT，创建配置） |
| `start` | 启动 Reth 节点 |
| `stop` | 停止 Reth 节点 |
| `restart` | 重启 Reth 节点 |
| `status` | 显示节点状态和当前区块 |
| `logs` | 显示实时日志 |
| `cleanup` | 清理所有容器和数据（谨慎使用）|
| `rpc <method> [params]` | 执行 RPC 调用 |
| `help` | 显示帮助信息 |

## RPC 使用示例

### 获取当前区块号

```bash
./run-reth.sh rpc eth_blockNumber
```

### 获取最新区块信息

```bash
./run-reth.sh rpc eth_getBlockByNumber '["latest",false]'
```

### 获取账户余额

```bash
./run-reth.sh rpc eth_getBalance '["0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb","latest"]'
```

### 发送原始交易

```bash
./run-reth.sh rpc eth_sendRawTransaction '["0x..."]'
```

## 端口说明

| 端口 | 协议 | 用途 |
|------|------|------|
| 30303 | TCP/UDP | P2P 网络通信 |
| 8545 | HTTP | JSON-RPC API |
| 8546 | WebSocket | WebSocket API |
| 8551 | HTTP | Engine API（共识层通信） |
| 9001 | HTTP | Prometheus 指标 |

## 数据目录

数据存储在 Docker volume 中：

```bash
# 查看 volume
docker volume ls | grep reth

# 检查 volume 详情
docker volume inspect reth-data
```

## 完整节点配置（执行层 + 共识层）

如果要运行完整的以太坊节点（包括共识层），需要同时运行 Lighthouse：

### 1. 修改 .env 文件

```bash
# 取消注释这一行
COMPOSE_PROFILES=full-node
```

### 2. 启动完整节点

```bash
docker-compose --profile full-node up -d
```

这将同时启动：
- **Reth**（执行层客户端）
- **Lighthouse**（共识层客户端）

## 监控和调试

### 查看容器状态

```bash
docker-compose ps
```

### 查看资源使用

```bash
docker stats reth-node
```

### 进入容器调试

```bash
docker exec -it reth-node sh
```

### 查看 Prometheus 指标

```bash
curl http://localhost:9001/metrics
```

## 常见问题

### 1. 同步需要多长时间？

- **Sepolia 测试网**：几小时到半天
- **主网**：几天到几周（取决于网络和硬件）

### 2. 如何加速同步？

Reth 支持 checkpoint sync：

```bash
# 编辑 docker-compose.yml，添加：
--datadir.static-files /path/to/checkpoint
```

### 3. 磁盘空间不足怎么办？

启用 pruning（修剪模式）：

```bash
# 在 docker-compose.yml 的 command 中添加：
--full
```

### 4. 如何备份数据？

```bash
# 停止节点
./run-reth.sh stop

# 备份 volume
docker run --rm -v reth-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/reth-backup.tar.gz /data

# 重启节点
./run-reth.sh start
```

### 5. 如何恢复数据？

```bash
# 停止节点
./run-reth.sh stop

# 恢复 volume
docker run --rm -v reth-data:/data -v $(pwd):/backup \
  alpine tar xzf /backup/reth-backup.tar.gz -C /

# 重启节点
./run-reth.sh start
```

## 网络配置

### Sepolia 测试网（推荐开发使用）

```bash
CHAIN=sepolia
```

- 区块链大小：~100 GB
- 同步时间：几小时
- 免费测试 ETH：https://sepoliafaucet.com/

### 主网

```bash
CHAIN=mainnet
```

- 区块链大小：~1 TB+（不断增长）
- 同步时间：几天到几周
- 需要真实 ETH

### Holesky 测试网

```bash
CHAIN=holesky
```

- 新的长期测试网
- 区块链大小：~200 GB
- 免费测试 ETH

## 性能优化

### 1. 增加数据库缓存

编辑 `docker-compose.yml`，在 command 中添加：

```bash
--db.max-cache-size 4096
```

### 2. 调整并发连接数

```bash
--peers.max-concurrent 100
```

### 3. 启用 HTTP 压缩

```bash
--http.compression
```

## 安全建议

1. **不要在公网暴露 RPC 端口**：默认配置仅绑定到 localhost
2. **使用防火墙**：仅开放必要的 P2P 端口（30303）
3. **定期更新**：及时更新 Reth 版本
4. **保护 JWT secret**：不要泄露 `jwt.hex` 文件
5. **限制 RPC 访问**：生产环境使用反向代理和认证

## 连接到 Web3 应用

### JavaScript/TypeScript（ethers.js）

```javascript
import { ethers } from 'ethers';

const provider = new ethers.JsonRpcProvider('http://localhost:8545');
const blockNumber = await provider.getBlockNumber();
console.log('Current block:', blockNumber);
```

### Rust（ethers-rs）

```rust
use ethers::providers::{Provider, Http};

let provider = Provider::<Http>::try_from("http://localhost:8545")?;
let block_number = provider.get_block_number().await?;
println!("Current block: {}", block_number);
```

### Python（web3.py）

```python
from web3 import Web3

w3 = Web3(Web3.HTTPProvider('http://localhost:8545'))
block_number = w3.eth.block_number
print(f'Current block: {block_number}')
```

## 故障排除

### 节点无法启动

```bash
# 查看详细日志
docker-compose logs reth

# 检查端口占用
lsof -i :8545
```

### 同步停滞

```bash
# 重启节点
./run-reth.sh restart

# 检查网络连接
docker exec reth-node sh -c "netstat -an | grep 30303"
```

### 内存不足

```bash
# 减少数据库缓存
# 编辑 docker-compose.yml，降低 --db.max-cache-size
```

## 卸载

### 完全清理（删除所有数据）

```bash
./run-reth.sh cleanup
```

这将删除：
- 所有容器
- 所有 volume 数据
- 区块链数据

## 更多资源

- **Reth 官方文档**：https://paradigmxyz.github.io/reth/
- **GitHub 仓库**：https://github.com/paradigmxyz/reth
- **Discord 社区**：https://paradigm.xyz/discord
- **以太坊官方文档**：https://ethereum.org/developers

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License
