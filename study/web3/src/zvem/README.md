# Halo2 零知识证明示例

这个模块展示了如何使用 `halo2_proofs` 库来构建零知识证明电路。

## 概述

零知识证明（Zero-Knowledge Proof, ZKP）允许证明者向验证者证明某个声明是真的，而不需要透露任何额外的信息。

### 三个核心属性

1. **完备性（Completeness）**：如果声明是真的，诚实的证明者总能说服诚实的验证者
2. **可靠性（Soundness）**：如果声明是假的，欺骗的证明者无法说服诚实的验证者
3. **零知识性（Zero-Knowledge）**：验证者只能知道声明是真的，无法获得任何其他信息

## 示例电路

### 1. 平方电路（Square Circuit）

**声明**：我知道一个数 x，使得 x² = y

**电路约束**：`x * x = y`

**示例**：
- 私密输入：x = 5
- 公开输出：y = 25
- 证明者可以证明知道 x 使得 x² = 25，但不透露 x 的值

### 2. 加法电路（Addition Circuit）

**声明**：我知道两个数 a 和 b，使得 a + b = c

**电路约束**：`a + b = c`

**示例**：
- 私密输入：a = 3, b = 7
- 公开输出：c = 10
- 可能的组合有很多：(1,9), (2,8), (3,7), (4,6), (5,5) 等等

## 运行示例

### 编译和运行

```bash
# 编译
cargo build --bin zevm_demo

# 运行示例程序
cargo run --bin zevm_demo
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_square_circuit
cargo test test_add_circuit
```

## 代码结构

```
src/zvem/
├── mod.rs          # 模块声明
├── zevm.rs         # Halo2 零知识证明电路实现
├── main.rs         # 示例运行程序
└── README.md       # 本文档
```

## Halo2 电路组件

### 1. Config（配置）

定义电路使用的列和选择器：

```rust
struct SquareConfig {
    advice: Column<Advice>,      // 建议列（存放私密值）
    instance: Column<Instance>,  // 实例列（存放公开值）
    selector: Selector,          // 选择器（激活约束）
}
```

### 2. Chip（芯片）

封装电路逻辑的可重用组件：

```rust
struct SquareChip<F: Field> {
    config: SquareConfig,
    _marker: PhantomData<F>,
}
```

### 3. Circuit（电路）

实现完整的电路逻辑：

```rust
impl<F: Field> Circuit<F> for SquareCircuit<F> {
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config;
    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error>;
}
```

## 工作流程

### 证明生成流程

1. **定义电路**：创建电路结构和约束
2. **设置参数**：选择电路大小（k 参数）
3. **提供见证**：输入私密数据（witness）
4. **验证约束**：使用 MockProver 检查约束是否满足
5. **生成证明**：（在实际应用中）生成零知识证明

### 验证流程

1. **接收证明**：获取证明数据
2. **接收公开输入**：获取公开的声明
3. **验证证明**：验证证明的有效性
4. **得出结论**：证明有效则接受声明

## 实际应用场景

### 1. 隐私交易

- 证明交易有效但不透露金额和参与者
- 示例：Zcash, Tornado Cash

### 2. 身份验证

- 证明年龄超过18岁而不透露具体年龄
- 证明拥有某个证书而不透露证书内容

### 3. 可扩展性（Rollups）

- 将大量交易压缩成一个证明
- 示例：zkSync, StarkNet

### 4. 私密投票

- 证明投票有效但不透露具体投给谁
- 保护选民隐私

## 性能考虑

### MockProver vs 真实证明

本示例使用 `MockProver` 进行演示，它：
- ✅ 快速验证电路约束
- ✅ 适合开发和测试
- ❌ 不生成真实的零知识证明

要生成真实证明，需要：
1. 设置可信初始化（Trusted Setup）
2. 生成证明密钥和验证密钥
3. 使用 `create_proof` 生成证明
4. 使用 `verify_proof` 验证证明

### 电路大小

参数 `k` 决定电路大小：
- `k = 4`：2^4 = 16 行
- `k = 10`：2^10 = 1024 行
- `k = 20`：2^20 = 1,048,576 行

更大的电路可以表达更复杂的计算，但需要更多时间和内存。

## 扩展学习

### 推荐资源

1. **Halo2 Book**：https://zcash.github.io/halo2/
2. **ZK Whiteboard Sessions**：https://zkhack.dev/whiteboard/
3. **ZK Learning**：https://zkiap.com/

### 进阶主题

- PLONK 算法
- Polynomial Commitment Schemes
- Lookup Arguments
- Custom Gates
- Recursion

## 许可证

本示例代码仅用于学习和演示目的。
