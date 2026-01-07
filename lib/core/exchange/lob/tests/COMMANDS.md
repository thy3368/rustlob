# 测试命令参考

## 快速开始

### 运行所有集成测试
```bash
cargo test --test lob_integration_tests
```

### 运行单个测试
```bash
cargo test --test lob_integration_tests test_exact_match
```

### 运行特定类别的测试
```bash
# 订单匹配测试
cargo test --test lob_integration_tests test_match

# 订单取消测试
cargo test --test lob_integration_tests test_cancel

# 价格相关测试
cargo test --test lob_integration_tests test_price

# 边界条件测试
cargo test --test lob_integration_tests test_boundary
```

## 详细输出

### 显示测试输出
```bash
cargo test --test lob_integration_tests -- --nocapture
```

### 显示测试名称
```bash
cargo test --test lob_integration_tests -- --list
```

### 单线程运行（调试用）
```bash
cargo test --test lob_integration_tests -- --test-threads=1
```

## 性能测试

### 运行高负载测试
```bash
cargo test --test lob_integration_tests test_high_volume -- --nocapture
```

### 运行压力测试
```bash
cargo test --test lob_integration_tests -- --ignored
```

## 代码覆盖率

### 使用tarpaulin生成覆盖率报告
```bash
# 安装tarpaulin
cargo install cargo-tarpaulin

# 运行覆盖率分析
cargo tarpaulin --test lob_integration_tests --out Html

# 查看报告
open tarpaulin-report.html
```

### 使用llvm-cov（Rust 1.60+）
```bash
# 安装llvm-tools
rustup component add llvm-tools-preview

# 运行覆盖率分析
cargo llvm-cov --test lob_integration_tests --html

# 查看报告
open target/llvm-cov/html/index.html
```

## 基准测试（需要配置）

### 启用基准测试
1. 在Cargo.toml中添加criterion依赖
2. 重命名benchmark_template.rs为benches/lob_benchmarks.rs
3. 取消注释代码

### 运行基准测试
```bash
cargo bench

# 查看报告
open target/criterion/report/index.html
```

## CI/CD集成

### GitHub Actions示例
```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --test lob_integration_tests
      - name: Run benchmarks
        run: cargo bench --no-run
```

## 调试技巧

### 运行失败的测试
```bash
# 只运行失败的测试
cargo test --test lob_integration_tests -- --failed
```

### 使用RUST_BACKTRACE
```bash
RUST_BACKTRACE=1 cargo test --test lob_integration_tests
```

### 使用RUST_LOG
```bash
RUST_LOG=debug cargo test --test lob_integration_tests -- --nocapture
```

## 内存分析

### 使用Valgrind检测内存泄漏
```bash
# 编译测试
cargo test --test lob_integration_tests --no-run

# 运行valgrind
valgrind --leak-check=full \
  target/debug/deps/lob_integration_tests-*
```

### 使用heaptrack分析内存使用
```bash
heaptrack target/debug/deps/lob_integration_tests-*
heaptrack_gui heaptrack.lob_integration_tests.*.gz
```

## 性能分析

### 使用perf进行性能分析
```bash
# 编译优化版本
cargo test --test lob_integration_tests --release --no-run

# 运行perf
perf record -g target/release/deps/lob_integration_tests-*
perf report
```

### 使用flamegraph
```bash
# 安装flamegraph
cargo install flamegraph

# 生成火焰图
cargo flamegraph --test lob_integration_tests
```

## 持续集成建议

### 每次提交运行
```bash
cargo test --test lob_integration_tests
```

### 每日构建
```bash
cargo test --test lob_integration_tests -- --test-threads=1
cargo bench
cargo tarpaulin --test lob_integration_tests
```

### 发布前检查
```bash
cargo test --test lob_integration_tests --release
cargo bench
cargo doc --no-deps
```

## 故障排查

### 测试超时
```bash
# 增加超时时间
cargo test --test lob_integration_tests -- --timeout 60
```

### 测试卡住
```bash
# 单线程运行找出问题
cargo test --test lob_integration_tests -- --test-threads=1 --nocapture
```

### 间歇性失败
```bash
# 多次运行
for i in {1..100}; do
  cargo test --test lob_integration_tests || break
done
```

## 最佳实践

1. **开发时**: 运行相关测试
   ```bash
   cargo test --test lob_integration_tests test_<feature>
   ```

2. **提交前**: 运行完整测试套件
   ```bash
   cargo test --test lob_integration_tests
   ```

3. **定期**: 运行性能基准测试
   ```bash
   cargo bench
   ```

4. **发布前**: 完整的质量检查
   ```bash
   cargo test --all
   cargo clippy
   cargo fmt --check
   cargo doc --no-deps
   ```
