# BDD 测试运行指南

## 📋 目录

1. [测试文件说明](#测试文件说明)
2. [运行所有测试](#运行所有测试)
3. [运行单个场景](#运行单个场景)
4. [测试输出解读](#测试输出解读)
5. [持续集成配置](#持续集成配置)

---

## 测试文件说明

### 文件结构

```
rustlob/proc/operating/exchange/prep/
├── src/
│   └── proc/
│       └── workflow.rs              # 被测试的工作流代码
├── tests/
│   └── workflow_bdd_tests.rs        # BDD 风格的测试代码
└── docs/
    └── WORKFLOW_BDD_SPEC.md         # 业务规格文档（本文档）
```

### 测试文件组成

**workflow_bdd_tests.rs** 包含：
- ✅ Mock实现（模拟外部依赖）
- ✅ 10个BDD场景测试
- ✅ Given-When-Then结构
- ✅ 中文业务描述
- ✅ 详细的断言验证

---

## 运行所有测试

### 基本运行

```bash
cd /Users/hongyaotang/src/rustlob/proc/operating/exchange/derivatives

# 运行所有测试
cargo test

# 运行所有测试并显示输出
cargo test -- --nocapture

# 运行所有测试（详细模式）
cargo test -- --nocapture --test-threads=1
```

### 只运行BDD测试

```bash
# 运行workflow相关的BDD测试
cargo test workflow_bdd

# 显示完整输出
cargo test workflow_bdd -- --nocapture
```

### 预期输出

```
running 11 tests
test scenario_trader_opens_long_position_on_btc ... ok
test scenario_trader_closes_position_with_profit ... ok
test scenario_trader_adds_margin_to_avoid_liquidation ... ok
test scenario_trader_reduces_leverage_to_lower_risk ... ok
test scenario_trader_cancels_all_orders_in_emergency ... ok
test scenario_trader_switches_margin_type ... ok
test scenario_trader_enables_auto_add_margin ... ok
test scenario_trader_modifies_order_price ... ok
test scenario_validation_rejects_invalid_margin_amount ... ok
test scenario_validation_rejects_invalid_reduce_margin_amount ... ok
test scenario_complete_trading_lifecycle ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## 运行单个场景

### 场景1: 开仓测试

```bash
cargo test scenario_trader_opens_long_position_on_btc -- --nocapture
```

**输出示例**:
```
running 1 test
test scenario_trader_opens_long_position_on_btc ... ok

✅ 场景1通过: 交易员成功开仓做多BTC
   持仓ID: 1
   入场价: 50000.00
   强平价: Some(45000.00)

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out
```

### 场景3: 追加保证金测试

```bash
cargo test scenario_trader_adds_margin_to_avoid_liquidation -- --nocapture
```

**输出示例**:
```
running 1 test
⚠️  价格从50000跌至46000，接近强平价45000!
✅ 场景3通过: 交易员成功追加保证金避免强平
   追加金额: 500.00
   新的总保证金: 6000.00
   新的强平价: 43000.00

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out
```

### 场景10: 完整生命周期测试

```bash
cargo test scenario_complete_trading_lifecycle -- --nocapture
```

**输出示例**:
```
running 1 test

🔄 开始完整交易生命周期测试
============================================================

📍 步骤1: 开仓 10x 杠杆做多 1 BTC
   ✅ 开仓成功

📍 步骤2: 价格下跌，追加 1000 USDT 保证金
   ✅ 追加保证金成功

📍 步骤3: 降低杠杆到 5x
   ✅ 杠杆调整成功

📍 步骤4: 平仓获利
   ✅ 平仓成功

============================================================
✅ 完整交易生命周期测试通过！

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10 filtered out
```

---

## 测试输出解读

### Given-When-Then 结构

每个测试遵循BDD的Given-When-Then模式：

```rust
#[tokio::test]
async fn scenario_trader_opens_long_position_on_btc() {
    // Given: 交易员想要以10倍杠杆做多1个BTC
    let mut ctx = TestContext::new();
    let trader_id = "trader_001";
    let symbol = Symbol::new("BTCUSDT");

    // When: 交易员提交开仓请求
    let action = TradeAction::OpenPosition(open_cmd);
    let result = ctx.workflow.execute(action).await;

    // Then: 开仓成功，返回持仓信息
    assert!(result.is_ok(), "开仓应该成功");
    // ... 更多断言
}
```

### 断言类型

#### 1. 成功性断言
```rust
assert!(result.is_ok(), "开仓应该成功");
```

#### 2. 数值断言
```rust
assert_eq!(pos_result.position_id, 1, "持仓ID应该为1");
assert!(pos_result.entry_price.to_f64() > 0.0, "入场价格应该大于0");
```

#### 3. 逻辑断言
```rust
assert!(liq_price.to_f64() < pos_result.entry_price.to_f64(),
        "多仓的强平价格应该低于入场价格");
```

#### 4. 状态验证
```rust
let state = ctx.command_proc.state.lock().await;
assert_eq!(
    *state.leverage_settings.get(&key).unwrap(),
    target_leverage,
    "杠杆设置应该已保存"
);
```

---

## 测试覆盖报告

### 生成覆盖率报告

```bash
# 安装 tarpaulin（首次运行）
cargo install cargo-tarpaulin

# 生成覆盖率报告
cargo tarpaulin --out Html --output-dir coverage

# 打开报告
open coverage/index.html
```

### 查看特定模块覆盖率

```bash
cargo tarpaulin --out Html -- workflow
```

---

## 性能测试

### 运行性能基准测试

```bash
# 基准测试（需要 criterion）
cargo bench workflow

# 查看结果
cat target/criterion/report/index.html
```

### 延迟测试

```bash
# 运行延迟测试（确保 < 100μs）
cargo test --release -- --nocapture --test-threads=1 | grep "elapsed"
```

---

## 持续集成配置

### GitHub Actions 配置

创建 `.github/workflows/bdd_tests.yml`:

```yaml
name: BDD Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: 安装 Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: 运行 BDD 测试
      run: |
        cd proc/operating/exchange/prep
        cargo test workflow_bdd -- --nocapture

    - name: 生成覆盖率报告
      run: |
        cargo install cargo-tarpaulin
        cargo tarpaulin --out Xml

    - name: 上传覆盖率
      uses: codecov/codecov-action@v3
```

### GitLab CI 配置

创建 `.gitlab-ci.yml`:

```yaml
stages:
  - test

bdd_tests:
  stage: test
  image: rust:latest
  script:
    - cd proc/operating/exchange/derivatives
    - cargo test workflow_bdd -- --nocapture
  only:
    - main
    - develop
    - merge_requests
```

---

## 调试失败的测试

### 启用详细日志

```bash
# 设置 RUST_LOG 环境变量
RUST_LOG=debug cargo test scenario_trader_opens_long_position_on_btc -- --nocapture
```

### 使用 println! 调试

在测试中添加调试输出：

```rust
println!("调试信息: {:?}", some_value);
```

### 单步调试

使用 VS Code 或 IntelliJ IDEA 的调试器：

1. 在测试函数上设置断点
2. 点击"Debug Test"
3. 逐步执行代码

---

## 测试最佳实践

### 1. 保持测试独立

每个测试应该独立运行，不依赖其他测试的状态：

```rust
#[tokio::test]
async fn scenario_test() {
    // 每个测试创建新的上下文
    let mut ctx = TestContext::new();
    // ...
}
```

### 2. 使用描述性的断言消息

```rust
assert!(result.is_ok(), "开仓应该成功，但失败了: {:?}", result.err());
```

### 3. 测试边界条件

```rust
// 测试无效输入
let invalid_req = AddMarginRequest::new(
    "trader".to_string(),
    symbol,
    PositionSide::Long,
    Price::from_raw(0), // 边界条件：金额为0
);
assert!(invalid_req.validate().is_err());
```

### 4. 使用有意义的测试数据

```rust
// ❌ 不好
let trader = "t1";

// ✅ 好
let trader_id = "trader_001_conservative";
```

---

## 故障排除

### 问题1: 编译错误

**症状**:
```
error[E0433]: failed to resolve: use of undeclared crate or module `prep`
```

**解决方案**:
```bash
# 确保在正确的目录
cd /Users/hongyaotang/src/rustlob/proc/operating/exchange/derivatives

# 检查 Cargo.toml 依赖
cat Cargo.toml
```

### 问题2: 测试超时

**症状**:
```
test scenario_complete_trading_lifecycle ... timeout
```

**解决方案**:
```rust
// 增加超时时间
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[timeout(Duration::from_secs(10))]
async fn scenario_complete_trading_lifecycle() {
    // ...
}
```

### 问题3: Mock 状态不一致

**症状**:
```
assertion failed: state.positions.len() == 1
```

**解决方案**:
```rust
// 确保每个测试创建新的 Mock
let command_proc = MockPerpOrderExchProc::new(); // 每次创建新实例
```

---

## 快速参考

### 常用测试命令

| 命令 | 说明 |
|------|------|
| `cargo test` | 运行所有测试 |
| `cargo test workflow_bdd` | 运行 BDD 测试 |
| `cargo test -- --nocapture` | 显示输出 |
| `cargo test -- --test-threads=1` | 单线程运行 |
| `cargo test scenario_name` | 运行特定场景 |
| `cargo test --release` | 发布模式测试 |

### 测试场景索引

| 场景编号 | 场景名称 | 测试函数 |
|---------|---------|---------|
| 场景1 | 开仓做多 | `scenario_trader_opens_long_position_on_btc` |
| 场景2 | 平仓获利 | `scenario_trader_closes_position_with_profit` |
| 场景3 | 追加保证金 | `scenario_trader_adds_margin_to_avoid_liquidation` |
| 场景4 | 降低杠杆 | `scenario_trader_reduces_leverage_to_lower_risk` |
| 场景5 | 批量撤单 | `scenario_trader_cancels_all_orders_in_emergency` |
| 场景6 | 切换保证金类型 | `scenario_trader_switches_margin_type` |
| 场景7 | 自动追加保证金 | `scenario_trader_enables_auto_add_margin` |
| 场景8 | 修改订单 | `scenario_trader_modifies_order_price` |
| 场景9 | 参数验证 | `scenario_validation_rejects_invalid_margin_amount` |
| 场景10 | 完整生命周期 | `scenario_complete_trading_lifecycle` |

---

**文档版本**: v1.0
**最后更新**: 2025-12-13
**维护者**: RustLOB Exchange Team
