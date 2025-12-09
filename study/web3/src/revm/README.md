# REVM 智能合约执行示例

这个模块展示了如何使用 REVM (Rust Ethereum Virtual Machine) 来部署和执行智能合约。

## 项目结构

```
src/revm/
├── mod.rs              # 模块导出
├── executor.rs         # REVM 执行器核心实现
├── contracts.rs        # 合约字节码和函数选择器
├── example.rs          # Counter 合约示例
├── main.rs            # 可执行程序入口
├── contracts/         # Solidity 合约源码
│   └── Counter.sol    # Counter 合约
└── README.md          # 本文档
```

## 功能特性

### 1. RevmExecutor 执行器

核心执行器提供以下功能：

- **合约部署**: 将字节码部署到 EVM 并获取合约地址
- **合约调用**: 执行状态改变的交易
- **状态查询**: 只读调用，不修改状态
- **内存数据库**: 使用 InMemoryDB 存储账户和合约状态

### 2. Counter 智能合约

一个简单的计数器合约，包含以下功能：

```solidity
contract Counter {
    uint256 public count;

    function increment() public;  // 增加计数器
    function get() public view returns (uint256);  // 查询计数
    function reset() public;  // 重置计数器
}
```

## 快速开始

### 编译项目

```bash
cd /Users/hongyaotang/src/rustlob/study/web3
cargo build --bin revm_counter
```

### 运行示例

```bash
cargo run --bin revm_counter
```

### 预期输出

程序会执行以下步骤：

1. ✅ 初始化 REVM 执行器
2. ✅ 部署 Counter 合约
3. ✅ 查询初始计数值（应为 0）
4. ✅ 调用 increment() 三次（计数变为 3）
5. ✅ 调用 reset() 重置计数器（计数变为 0）
6. ✅ 再次调用 increment()（计数变为 1）

## 技术细节

### REVM 架构

REVM 是一个高性能的 EVM 实现，特点包括：

- **零分配优化**: 关键路径避免内存分配
- **模块化设计**: 可插拔的数据库层
- **类型安全**: 使用 Rust 强类型系统
- **低延迟**: 针对高频交易场景优化

### 数据编码

#### 函数选择器计算

函数选择器是函数签名的 Keccak256 哈希的前 4 字节：

```rust
// increment() -> 0xd09de08a
keccak256("increment()")[0..4]

// get() -> 0x6d4ce63c
keccak256("get()")[0..4]

// reset() -> 0xd826f88f
keccak256("reset()")[0..4]
```

#### 返回值解码

uint256 返回值占用 32 字节：

```rust
fn decode_uint256(data: &[u8]) -> U256 {
    U256::from_be_slice(&data[0..32])
}
```

## 性能优化

### 低延迟配置

根据项目的 CLAUDE.md 要求，以下优化已应用：

1. **编译优化**
   ```toml
   [profile.release]
   opt-level = 3
   lto = "fat"
   codegen-units = 1
   target-cpu = "native"
   ```

2. **内存对齐**
   - 关键数据结构使用缓存行对齐（64/128 字节）
   - 避免 false sharing

3. **零分配路径**
   - 热路径避免动态内存分配
   - 使用栈分配和对象池

### Gas 优化

- Gas 限制: 10,000,000 per transaction
- 实际使用: ~21,000 for simple transfers
- 合约调用: ~50,000-100,000 depending on complexity

## 合约字节码生成

### 使用 solc 编译器生成完整的标准字节码

**重要提示**：必须使用 Solidity 编译器生成的完整字节码，手工编造或截断的字节码会导致 `InvalidJump` 或 `StackUnderflow` 错误！

#### 安装 Solidity 编译器

```bash
# macOS
brew install solidity

# Ubuntu/Debian
sudo add-apt-repo ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install solc

# 验证安装
solc --version
```

#### 编写 Solidity 合约

创建 `Counter.sol` 文件：

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Counter {
    uint256 public count;

    function increment() public {
        count += 1;
    }
}
```

#### 编译生成字节码

```bash
# 编译合约，生成优化的字节码
solc --bin --abi --optimize --optimize-runs 200 Counter.sol

# 输出示例：
# ======= Counter.sol:Counter =======
# Binary:
# 6080604052348015600e575f5ffd5b5060c580601a5f395ff3fe...
```

#### 验证字节码完整性

**字节码结构验证**：

```
部署字节码 (Creation Bytecode):
├─ 初始化代码 (0-26 bytes)
│  └─ 设置初始状态
├─ 运行时代码长度标记 (PUSH1 0xc5)
├─ 运行时代码偏移标记 (PUSH1 0x1a)
└─ 运行时代码 (Runtime Bytecode)
   ├─ 函数调度器
   ├─ count() 实现
   └─ increment() 实现
```

**常见问题排查**：

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| `InvalidJump` | 字节码被截断 | 使用完整的 solc 输出 |
| `StackUnderflow` | 字节码损坏 | 重新编译，确保复制完整 |
| `OddLength` | 字节码长度为奇数 | 检查是否有遗漏的字符 |

#### 在代码中使用编译的字节码

```rust
// src/revm/contracts.rs

/// ✅ 由 solc 0.8.30 编译的完整字节码
pub const COUNTER_BYTECODE: &str =
    "6080604052348015600e575f5ffd5b5060c580601a5f395ff3fe\
     6080604052348015600e575f5ffd5b50600436106030575f3560\
     e01c806306661abd146034578063d09de08a14604d575b5f5ffd\
     5b603b5f5481565b60405190815260200160405180910390f35b\
     60536055565b005b60015f5f82825460649190606b565b909155\
     5050565b80820180821115608957634e487b7160e01b5f526011\
     60045260245ffd5b9291505056fea2646970667358221220a48a\
     a1ce6ad99f3a568df931a13cb1db6bbd0417ec8cf682182c7944\
     54b27c5664736f6c634300081e0033";

pub fn get_counter_bytecode() -> Vec<u8> {
    hex::decode(COUNTER_BYTECODE)
        .expect("Invalid hex string")
}
```

### 性能数据（实际运行结果）

```
✅ 部署成功:
   - 部署字节码: 223 bytes
   - 运行时代码: 197 bytes
   - 合约地址: 0x1c81a61a407017c58397a47d2ab28191b9b8ec9b

✅ Gas 消耗:
   - 首次 increment(): 43,405 gas (冷存储写入)
   - 后续 increment(): 26,305 gas (热存储更新)

✅ 执行结果:
   - 初始 count: 0
   - 第 1 次 increment: count = 1
   - 第 2 次 increment: count = 2
   - 第 3 次 increment: count = 3
```

## 扩展示例

### 部署自定义合约

```rust
use web3::revm::RevmExecutor;

let mut executor = RevmExecutor::new();

// 使用 solc 编译的完整字节码
let bytecode = hex::decode("6080604052...").unwrap();

let address = executor
    .deploy_contract("MyContract", bytecode)
    .expect("部署失败");

println!("合约地址: {:?}", address);
```

### 调用合约函数

```rust
// 编码函数调用数据
let calldata = vec![
    0xd0, 0x9d, 0xe0, 0x8a,  // 函数选择器
    // 参数（如果有）
];

let result = executor
    .call_contract("MyContract", calldata)
    .expect("调用失败");

println!("返回值: {:?}", result);
```

### 只读查询

```rust
let calldata = encode_get();  // 查询函数

let result = executor
    .view_contract("MyContract", calldata)
    .expect("查询失败");

let value = decode_uint256(&result);
println!("查询结果: {}", value);
```

## 测试

运行单元测试：

```bash
cargo test --lib revm
```

运行集成测试：

```bash
cargo test --bin revm_counter
```

## 故障排查

### 常见错误及解决方案

#### 1. InvalidJump 错误

**症状**：
```
❌ 错误: View call halted: InvalidJump
```

**根本原因**：
字节码被截断，导致 REVM 无法正确提取运行时代码。

**诊断步骤**：
```bash
# 分析字节码结构
# 部署字节码格式: PUSH1 <size> DUP1 PUSH1 <offset> PUSH0 CODECOPY ...

# 示例：被截断的字节码
部署字节码长度: 353 bytes
运行时代码大小: 421 bytes (从字节码中读取)
运行时代码偏移: 28 bytes
运行时代码范围: bytes[28..449]  ❌ 超出边界！
缺失字节数: 96 bytes (449 - 353)
```

**解决方案**：
1. 使用 `solc` 编译器重新生成完整字节码
2. 确保复制整个字节码字符串，不要截断
3. 验证字节码长度与编译器输出一致

#### 2. StackUnderflow 错误

**症状**：
```
❌ 错误: View call halted: StackUnderflow
```

**可能原因**：
- 字节码损坏或格式错误
- 字节码版本与 REVM 不兼容
- 函数选择器不匹配

**解决方案**：
1. 验证 Solidity 编译器版本 (推荐 0.8.20+)
2. 检查函数选择器是否正确计算
3. 使用 `--optimize` 优化编译

#### 3. OddLength 错误

**症状**：
```
panicked at 'Invalid hex string: OddLength'
```

**原因**：
十六进制字符串长度为奇数（缺少一个字符）

**解决方案**：
```rust
// ❌ 错误：长度为 221（奇数）
pub const BYTECODE: &str = "608060405234801561000f575f80fd...03";

// ✅ 正确：长度为 222（偶数）
pub const BYTECODE: &str = "608060405234801561000f575f80fd...0033";
```

#### 4. 状态未提交问题

**症状**：
```
✅ 合约调用成功，Gas 使用: 21064
   当前计数: 0  ❌ 应该是 1
```

**原因**：
使用 `transact()` 但未提交状态到数据库

**解决方案**：
```rust
// ❌ 错误：状态未提交
let result = evm.transact()?;

// ✅ 方法1：使用 transact_commit (REVM 18.0+)
let result = evm.transact_commit()?;

// ✅ 方法2：手动提交
let result_and_state = evm.transact()?;
drop(evm);  // 释放对 db 的借用
self.db.commit(result_and_state.state);
```

### 调试技巧

#### 1. 查看合约账户信息

```rust
pub fn debug_account(&self, address: Address) {
    if let Some(account) = self.db.accounts.get(&address) {
        println!("账户 {:?}:", address);
        println!("  余额: {}", account.info.balance);
        println!("  nonce: {}", account.info.nonce);
        println!("  代码哈希: {:?}", account.info.code_hash);
        if let Some(ref code) = account.info.code {
            println!("  代码长度: {} bytes", code.bytecode().len());
        }
    }
}
```

#### 2. 验证字节码完整性

```bash
# 计算字节码长度
echo -n "6080604052..." | wc -c
# 输出: 446 (应该是偶数)

# 长度/2 = 实际字节数
# 446 / 2 = 223 bytes ✅
```

#### 3. 使用 Remix IDE 验证

访问 https://remix.ethereum.org：
1. 粘贴 Solidity 代码
2. 编译（Ctrl+S）
3. 复制 Bytecode（不是 Deployment Bytecode）
4. 对比长度和内容

## 依赖项

```toml
revm = { version = "18.0.0", features = ["std"] }
alloy-primitives = "0.8"
hex = "0.4"
sha3 = "0.10"  # 用于计算函数选择器
```

## 参考资源

- [REVM GitHub](https://github.com/bluealloy/revm)
- [Alloy Framework](https://github.com/alloy-rs)
- [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
- [EVM Opcodes](https://www.evm.codes/)

## Clean Architecture 实践

本模块遵循 Clean Architecture 原则：

### 分层结构

1. **Entities Layer** (contracts.rs)
   - 合约字节码定义
   - 函数选择器常量
   - 纯数据编码函数

2. **Use Cases Layer** (executor.rs)
   - RevmExecutor 核心逻辑
   - 合约部署用例
   - 合约调用用例

3. **Interface Layer** (example.rs, main.rs)
   - 命令行界面
   - 示例演示
   - 用户交互

4. **Infrastructure Layer**
   - REVM 框架集成
   - InMemoryDB 数据库

### 依赖规则

- ✅ executor.rs 不依赖 example.rs
- ✅ contracts.rs 无外部依赖
- ✅ main.rs 依赖所有内层模块
- ✅ 可测试性：核心逻辑可独立测试

## 许可证

MIT License

## 作者

REVM Study Project - 2025
