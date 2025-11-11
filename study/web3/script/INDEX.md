# Reth Node Docker Scripts - 文件索引

## 📁 文件清单

### 核心文件

| 文件 | 类型 | 说明 |
|------|------|------|
| `docker-compose.yml` | Docker Compose | Docker 容器编排配置 |
| `run-reth.sh` | Shell Script | 主要管理脚本（可执行） |
| `Makefile` | Makefile | 便捷命令快捷方式 |
| `test-rpc.sh` | Shell Script | RPC 测试脚本（可执行） |

### 配置文件

| 文件 | 类型 | 说明 |
|------|------|------|
| `.env.example` | 环境配置 | 环境变量模板 |
| `.gitignore` | Git | Git 忽略规则 |

### 文档文件

| 文件 | 类型 | 说明 |
|------|------|------|
| `README.md` | 文档 | 完整使用文档 |
| `QUICKSTART.md` | 文档 | 快速入门指南 |
| `INDEX.md` | 文档 | 本文件 - 文件索引 |

## 🚀 快速开始

### 方式一：使用 Makefile（推荐）

```bash
# 初始化
make init

# 启动节点
make start

# 查看状态
make status

# 查看日志
make logs
```

### 方式二：使用 Shell 脚本

```bash
# 初始化
./run-reth.sh init

# 启动节点
./run-reth.sh start

# 查看状态
./run-reth.sh status

# 查看日志
./run-reth.sh logs
```

## 📚 文档指南

### 新手入门
👉 **阅读顺序**：`QUICKSTART.md` → `README.md`

- **QUICKSTART.md**：快速上手，5 分钟启动节点
- **README.md**：完整文档，包含所有配置和高级用法

### 开发者
- 查看 `docker-compose.yml` 了解容器配置
- 查看 `run-reth.sh` 了解脚本实现
- 使用 `test-rpc.sh` 测试 RPC 功能

## 🔧 核心功能

### 1. 节点管理（run-reth.sh）

```bash
./run-reth.sh init      # 初始化设置
./run-reth.sh start     # 启动节点
./run-reth.sh stop      # 停止节点
./run-reth.sh restart   # 重启节点
./run-reth.sh status    # 查看状态
./run-reth.sh logs      # 查看日志
./run-reth.sh cleanup   # 清理数据
./run-reth.sh rpc       # RPC 调用
```

### 2. Makefile 快捷命令

```bash
make init              # 初始化
make start             # 启动
make stop              # 停止
make restart           # 重启
make status            # 状态
make logs              # 日志
make rpc-test          # RPC 测试
make full-node         # 完整节点（执行层+共识层）
make backup            # 备份数据
make stats             # 资源统计
make update            # 更新 Reth
make shell             # 进入容器
make metrics           # 查看指标
```

### 3. RPC 测试（test-rpc.sh）

```bash
# 运行所有测试
./test-rpc.sh

# 详细输出
./test-rpc.sh -v

# 测试特定功能
./test-rpc.sh connectivity
./test-rpc.sh version
./test-rpc.sh network
./test-rpc.sh peers
./test-rpc.sh sync
./test-rpc.sh block
./test-rpc.sh balance

# 自定义 RPC URL
./test-rpc.sh -u http://example.com:8545 all
```

## 🌐 网络配置

编辑 `.env` 文件选择网络：

```bash
# Sepolia 测试网（推荐开发）
CHAIN=sepolia

# 主网
CHAIN=mainnet

# Holesky 测试网
CHAIN=holesky
```

## 🔍 文件详细说明

### docker-compose.yml
- 定义 Reth 容器配置
- 可选：Lighthouse 共识层客户端
- 端口映射和 volume 配置
- 健康检查和网络配置

### run-reth.sh
- 主要管理脚本（424 行）
- 彩色输出和交互式界面
- 自动生成 JWT secret
- RPC 调用功能
- 状态检查和日志查看

### test-rpc.sh
- 完整的 RPC 测试套件（420 行）
- 测试 11 个核心 RPC 方法
- 彩色输出和详细报告
- 支持自定义 RPC 端点

### Makefile
- 简化命令输入
- 19 个预定义任务
- 彩色输出
- 链式操作支持

## 📊 目录结构

```
script/
├── docker-compose.yml      # Docker 编排
├── run-reth.sh            # 主脚本 ⭐
├── test-rpc.sh            # 测试脚本
├── Makefile               # 便捷命令
├── .env.example           # 配置模板
├── .gitignore             # Git 忽略
├── README.md              # 完整文档 📖
├── QUICKSTART.md          # 快速入门 🚀
└── INDEX.md               # 本文件 📋

运行时生成的文件（不提交到 Git）：
├── .env                   # 实际配置
├── jwt.hex                # JWT 密钥
└── config/                # 自定义配置目录
```

## 🎯 使用场景

### 场景 1：开发测试
```bash
# 使用 Sepolia 测试网
make init
# 编辑 .env，设置 CHAIN=sepolia
make start
make rpc-test
```

### 场景 2：运行主网节点
```bash
make init
# 编辑 .env，设置 CHAIN=mainnet
make start
make status
```

### 场景 3：完整节点（执行层+共识层）
```bash
make init
# 编辑 .env，取消注释 COMPOSE_PROFILES=full-node
make full-node
make status
```

### 场景 4：RPC 接口开发
```bash
make start
# 等待同步
./test-rpc.sh
./run-reth.sh rpc eth_blockNumber
```

## 🔐 安全提示

⚠️ **不要提交以下文件到 Git**：
- `.env`（包含配置）
- `jwt.hex`（密钥文件）
- `config/`（可能包含敏感信息）

✅ `.gitignore` 已自动配置忽略这些文件

## 📞 获取帮助

```bash
# 脚本帮助
./run-reth.sh help

# Makefile 帮助
make help

# 测试脚本帮助
./test-rpc.sh --help
```

## 🔗 相关资源

- [Reth 官方文档](https://paradigmxyz.github.io/reth/)
- [Reth GitHub](https://github.com/paradigmxyz/reth)
- [以太坊官方文档](https://ethereum.org/developers)
- [JSON-RPC API 文档](https://ethereum.org/en/developers/docs/apis/json-rpc/)

## 📝 版本信息

- **创建日期**：2024-11-12
- **脚本版本**：1.0.0
- **Reth 版本**：latest (from Docker Hub)
- **兼容系统**：macOS, Linux

## 🤝 贡献

欢迎提交问题和改进建议！

---

**快速链接**：
- 📖 [完整文档](./README.md)
- 🚀 [快速开始](./QUICKSTART.md)
- 🔧 [配置示例](./.env.example)
