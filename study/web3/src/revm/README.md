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

## 扩展示例

### 部署自定义合约

```rust
use web3::revm::RevmExecutor;

let mut executor = RevmExecutor::new();

// 你的合约字节码
let bytecode = vec![/* ... */];

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

## 依赖项

```toml
revm = { version = "18.0.0", features = ["std"] }
alloy-primitives = "0.8"
hex = "0.4"
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
