# Reth Node Quick Start Guide

## 最简单的启动流程

### 1. 一键初始化

```bash
cd /Users/hongyaotang/src/rustlob/study/web3/script
make init
```

或

```bash
./run-reth.sh init
```

### 2. 选择网络（可选）

编辑 `.env` 文件：

```bash
# 测试网（推荐）- 快速同步，适合开发
CHAIN=sepolia

# 或主网（需要更多时间和存储空间）
# CHAIN=mainnet
```

### 3. 启动节点

```bash
make start
```

或

```bash
./run-reth.sh start
```

### 4. 查看状态

```bash
make status
```

## 常用命令

| Makefile | Shell Script | 功能 |
|----------|--------------|------|
| `make init` | `./run-reth.sh init` | 初始化 |
| `make start` | `./run-reth.sh start` | 启动 |
| `make stop` | `./run-reth.sh stop` | 停止 |
| `make logs` | `./run-reth.sh logs` | 查看日志 |
| `make status` | `./run-reth.sh status` | 查看状态 |
| `make restart` | `./run-reth.sh restart` | 重启 |

## 测试 RPC

```bash
# 测试连接
make rpc-test

# 或手动测试
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}' \
  http://localhost:8545
```

## 预期输出

启动后应该看到：

```
✓ Counter actor spawned
After increment: 1
...
```

等待同步完成（可能需要几小时到几天，取决于网络）。

## 快速测试（使用 Sepolia）

```bash
# 1. 初始化
make init

# 2. 确认使用 Sepolia（编辑 .env）
echo "CHAIN=sepolia" > .env

# 3. 启动
make start

# 4. 等待 30 秒，然后查看状态
sleep 30 && make status

# 5. 查看日志
make logs
```

## 故障排除

### 无法启动？

```bash
# 检查 Docker 是否运行
docker ps

# 查看错误日志
docker-compose logs reth
```

### 端口冲突？

```bash
# 检查端口占用
lsof -i :8545
lsof -i :30303

# 修改 docker-compose.yml 中的端口映射
```

### 同步太慢？

Sepolia 测试网同步较快。主网同步需要更长时间，这是正常的。

## 下一步

查看 [README.md](./README.md) 了解：
- 详细配置选项
- RPC API 使用
- 性能优化
- 安全建议
