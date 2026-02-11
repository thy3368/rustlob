# Rust之从0-1低时延CEX：Wallet钱包行为设计

Wallet API 的 Rust 实现，提供完整的钱包管理功能，包括充提币、资产管理和账户信息查询。


## 行为流程图

### 命令处理流程

```
┌─────────────────────────────────────────────────────────────────┐
│                      用户应用层                                  │
│              创建 WalletCmdAny 命令                              │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────────┐
│                   WalletBehavior::handle()                       │
│                   命令分发和处理                                  │
└────────────────────────┬────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
   ┌─────────┐      ┌─────────┐      ┌─────────┐
   │ Capital │      │  Asset  │      │ Account │
   │ Handler │      │ Handler │      │ Handler │
   └────┬────┘      └────┬────┘      └────┬────┘
        │                │                │
        └────────────────┼────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────────────┐
│                    参数验证层                                    │
│  • 检查必需字段    • 格式验证    • 权限检查                      │
└────────────────────────┬────────────────────────────────────────┘
                         │
                    ┌────┴────┐
                    ↓         ↓
            ✓ 验证通过   ✗ 验证失败
                    │         │
                    │         └──→ InvalidParameter Error
                    ↓
┌─────────────────────────────────────────────────────────────────┐
│                   API 请求层                                    │
│  • 构建请求    • 签名    • 发送 HTTP                            │
└────────────────────────┬────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ↓                ↓                ↓
   网络错误         认证错误         服务器错误
        │                │                │
        └────────────────┼────────────────┘
                         │
                    ┌────┴────┐
                    ↓         ↓
            ✓ 请求成功   ✗ 请求失败
                    │         │
                    │         └──→ NetworkError / AuthError / ServerError
                    ↓
┌─────────────────────────────────────────────────────────────────┐
│                   响应解析层                                    │
│  • 解析 JSON    • 类型转换    • 数据验证                        │
└────────────────────────┬────────────────────────────────────────┘
                         │
                    ┌────┴────┐
                    ↓         ↓
            ✓ 解析成功   ✗ 解析失败
                    │         │
                    │         └──→ UnknownError
                    ↓
┌─────────────────────────────────────────────────────────────────┐
│                   缓存更新层                                    │
│  • 更新缓存    • 清除过期数据                                   │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────────┐
│              返回 Result<CmdResp<WalletRes>>                    │
│                   给用户应用层                                   │
└─────────────────────────────────────────────────────────────────┘
```

### 三类操作的处理流程

#### Capital 操作流程 (充提币)
```
用户命令
    ↓
┌─────────────────────────────────────────┐
│ GetAllCoins / Withdraw / History / Addr  │
└────────────┬────────────────────────────┘
             ↓
    ┌────────────────────┐
    │ 验证币种和网络信息  │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 调用 API   │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 返回交易结果       │
    └────────────────────┘
```

#### Asset 操作流程 (资产管理)
```
用户命令
    ↓
┌──────────────────────────────────────────────┐
│ GetAsset / Transfer / DustConvert / TradeFee  │
└────────────┬─────────────────────────────────┘
             ↓
    ┌────────────────────┐
    │ 验证账户和金额     │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 调用 API   │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 更新账户余额缓存   │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 返回操作结果       │
    └────────────────────┘
```

#### Account 操作流程 (账户信息)
```
用户命令
    ↓
┌──────────────────────────────────┐
│ GetAccountInfo / GetSnapshot      │
└────────────┬─────────────────────┘
             ↓
    ┌────────────────────┐
    │ 检查缓存           │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 调用 API   │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 缓存账户快照       │
    └────────┬───────────┘
             ↓
    ┌────────────────────┐
    │ 返回账户信息       │
    └────────────────────┘
```

### 错误处理流程

```
命令执行
    ↓
┌─────────────────────────────────────────────────────────┐
│                   错误检测                              │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┬──────────────┐
        ↓            ↓            ↓              ↓
   参数错误      网络错误      认证错误      限流错误
        │            │            │              │
        └────────────┼────────────┼──────────────┘
                     ↓
        ┌────────────────────────────────┐
        │ 生成 WalletCmdError            │
        │ • 错误类型                     │
        │ • 错误信息                     │
        │ • 建议的恢复操作               │
        └────────────┬───────────────────┘
                     ↓
        ┌────────────────────────────────┐
        │ 返回 Err(WalletCmdError)       │
        │ 给用户应用层处理               │
        └────────────────────────────────┘
```

## 模块概述

钱包模块基于 CQRS 架构设计，将所有钱包操作分为三个主要类别：

### 1. Capital Endpoints (钱包 - 充提币)

管理用户的充值和提现操作。

| 操作 | 端点 | 权重 | 描述 |
|------|------|------|------|
| GetAllCoins | GET /sapi/v1/capital/config/getall | 10 | 获取所有币种信息，包括充提状态和网络配置 |
| Withdraw | POST /sapi/v1/capital/withdraw/apply | 900 | 提币操作，支持多网络和自定义参数 |
| GetWithdrawHistory | GET /sapi/v1/capital/withdraw/hisrec | 1 | 查询提币历史记录 |
| GetDepositHistory | GET /sapi/v1/capital/deposit/hisrec | 1 | 查询充值历史记录 |
| GetDepositAddress | GET /sapi/v1/capital/deposit/address | 10 | 获取指定币种的充值地址 |

**主要特性：**
- 支持多个区块链网络（BSC、ETH、BNB等）
- 提现地址簿管理
- 充值地址生成和查询
- 完整的交易历史记录

### 2. Asset Endpoints (资产)

管理用户的资产和账户间转账。

| 操作 | 端点 | 权重 | 描述 |
|------|------|------|------|
| GetAssetDetail | GET /sapi/v1/asset/assetDetail | 1 | 获取资产详情（最小提现额、手续费等） |
| GetUserAssets | POST /sapi/v3/asset/getUserAsset | 5 | 查询用户持仓（仅返回非零余额） |
| UniversalTransfer | POST /sapi/v1/asset/transfer | 900 | 万向划转（现货、合约、杠杆、资金账户间转账） |
| GetUniversalTransferHistory | GET /sapi/v1/asset/transfer | 1 | 查询万向划转历史 |
| DustTransfer | POST /sapi/v1/asset/dust | 10 | 小额资产转换（转换为BNB） |
| GetTradeFee | GET /sapi/v1/asset/tradeFee | 1 | 查询交易手续费率 |
| GetFundingAsset | POST /sapi/v1/asset/get-funding-asset | 1 | 查询资金账户余额 |

**主要特性：**
- 支持14种账户类型间的转账
- 小额资产自动转换
- 实时手续费查询
- BTC估值支持

### 3. Account Endpoints (账户)

管理用户账户信息和快照。

| 操作 | 端点 | 权重 | 描述 |
|------|------|------|------|
| GetAccountInfo | GET /sapi/v1/account/info | 1 | 获取账户基本信息（VIP等级、功能开启状态等） |
| GetAccountSnapshot | GET /sapi/v1/accountSnapshot | 2400 | 查询每日资产快照（现货、杠杆、合约） |

**主要特性：**
- 账户功能状态查询
- 多账户类型快照支持
- 历史数据查询

## 架构设计

### CQRS 模式

模块采用 Command Query Responsibility Segregation (CQRS) 模式：

```
┌─────────────────────────────────────────────────────────┐
│                    WalletBehavior Trait                 │
│  fn handle(&mut self, cmd: WalletCmdAny) -> Result      │
└─────────────────────────────────────────────────────────┘
                            ↓
        ┌───────────────────┴───────────────────┐
        ↓                                       ↓
   WalletCmdAny                            WalletRes
   (14 Commands)                        (14 Responses)
        ↓                                       ↓
   ┌─────────────────────────────────────────────────┐
   │  Capital (5)  │  Asset (7)  │  Account (2)      │
   └─────────────────────────────────────────────────┘
```

### 数据流

1. **命令阶段**：用户创建特定的命令结构体（如 `WithdrawCmd`）
2. **处理阶段**：`WalletBehavior` trait 的实现处理命令
3. **响应阶段**：返回对应的响应类型（如 `WithdrawResponse`）

## 使用示例

### 基本使用

```rust
use wallet::traits::{WalletBehavior, WalletCmdAny, WithdrawCmd};

// 创建提币命令
let withdraw_cmd = WithdrawCmd {
    metadata: metadata,
    coin: "BTC".to_string(),
    address: "1A1z7agoat...".to_string(),
    amount: "0.5".to_string(),
    network: Some("BTC".to_string()),
    address_tag: None,
    withdraw_order_id: Some("order_123".to_string()),
    transaction_fee_flag: Some(false),
    name: Some("My Wallet".to_string()),
    wallet_type: Some(0),
};

// 执行命令
let result = wallet_handler.handle(WalletCmdAny::Withdraw(withdraw_cmd))?;
```

### 查询用户资产

```rust
use wallet::traits::{WalletBehavior, WalletCmdAny, GetUserAssetsCmd};

let assets_cmd = GetUserAssetsCmd {
    metadata: metadata,
    asset: None,  // 查询所有资产
    need_btc_valuation: Some(true),
};

let result = wallet_handler.handle(WalletCmdAny::GetUserAssets(assets_cmd))?;
```

## 数据结构

### 命令结构体

所有命令结构体都使用 `#[immutable]` 宏标记，确保不可变性：

- **GetAllCoinsCmd**：获取币种信息
- **WithdrawCmd**：提币操作
- **GetWithdrawHistoryCmd**：提币历史
- **GetDepositHistoryCmd**：充值历史
- **GetDepositAddressCmd**：充值地址
- **GetAssetDetailCmd**：资产详情
- **GetUserAssetsCmd**：用户持仓
- **UniversalTransferCmd**：万向划转
- **GetUniversalTransferHistoryCmd**：划转历史
- **DustTransferCmd**：小额转换
- **GetTradeFeeCmd**：手续费查询
- **GetFundingAssetCmd**：资金账户
- **GetAccountInfoCmd**：账户信息
- **GetAccountSnapshotCmd**：账户快照

### 响应类型

响应类型包含详细的数据结构：

- **CoinInfo**：币种信息（包含网络列表）
- **NetworkInfo**：网络配置信息
- **WithdrawResponse / WithdrawRecord**：提币相关
- **DepositRecord / DepositAddressResponse**：充值相关
- **UserAsset / AssetDetail**：资产相关
- **UniversalTransferResponse / UniversalTransferRecord**：转账相关
- **DustTransferResponse / DustTransferResult**：小额转换相关
- **TradeFee / FundingAsset**：费用和资金相关
- **AccountInfo / AccountSnapshot**：账户相关

## 错误处理

模块定义了统一的错误类型 `WalletCmdError`：

```rust
pub enum WalletCmdError {
    InvalidParameter(String),      // 参数错误
    NetworkError(String),          // 网络错误
    AuthenticationError(String),   // 认证错误
    RateLimitError(String),        // 限流错误
    ServerError(String),           // 服务器错误
    UnknownError(String),          // 未知错误
}
```

## 权重限制

API 请求权重说明：

- **IP 权重**：基于 IP 地址的限制，用于查询操作
- **UID 权重**：基于用户 ID 的限制，用于修改操作（提币、转账等）

高权重操作（如提币 900、快照查询 2400）需要特别注意频率限制。

## 特性

- ✅ 完整的 Wallet API 覆盖
- ✅ 类型安全的命令和响应
- ✅ 不可变数据结构
- ✅ 序列化/反序列化支持
- ✅ 统一的错误处理
- ✅ CQRS 架构设计
- ✅ 支持多网络和多账户类型

## 参考文档


- Capital 相关：`capital.md`, `capital-withdraw.md`, `capital-deposite-history.md` 等
- Asset 相关：`asset.md`, `asset-user-assets.md`, `asset-user-universal-transfer.md` 等
- Account 相关：`account.md`, `account-daily-account-snapshoot.md` 等

## 模块结构

```
wallet/
├── Cargo.toml
├── README.md (本文件)
└── src/
    ├── lib.rs
    └── traits/
        ├── mod.rs
        └── wallet_behavior.rs
```

## 集成指南

### 实现 WalletBehavior Trait

```rust
pub struct MyWalletHandler {
    // 你的实现字段
}

impl WalletBehavior for MyWalletHandler {
    fn handle(&mut self, cmd: WalletCmdAny) -> Result<CmdResp<WalletRes>, WalletCmdError> {
        match cmd {
            WalletCmdAny::GetAllCoins(cmd) => {
                // 实现获取币种信息的逻辑
            }
            WalletCmdAny::Withdraw(cmd) => {
                // 实现提币逻辑
            }
            // ... 其他命令
        }
    }
}
```

## 最佳实践

1. **参数验证**：在处理命令前验证所有必需参数
2. **错误处理**：使用统一的 `WalletCmdError` 类型
3. **权重管理**：监控 API 权重使用，避免触发限流
4. **缓存策略**：对频繁查询的数据（如币种信息）进行缓存
5. **异步处理**：对于高权重操作使用异步处理

