# 提交前硬门控体系

本文定义本仓库的质量门控分层。命令退出码非零、必需工具未安装、基线测试失败或环境缺失，均表示门控未通过，不得静默跳过。

## 门控分层

### 提交前硬门控

每次提交前必须通过以下命令。版本化 hook 位于 `.githooks/pre-commit`；安装方式见该文件。只修改单个 crate 时可将 `cargo check`、`cargo test` 和 `cargo clippy` 收窄为对应 package；修改多个 crate 或 workspace 配置时使用 workspace 级命令。

```bash
cargo fmt --all -- --check
cargo check
cargo test
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check advisories licenses sources bans --exclude-dev
cargo audit
```

业务代码变更必须补齐对应 entity/use case 测试，覆盖拒绝路径、幂等、重放和事件一致性。业务金融不变量通过 core/use case 测试硬拦截。

本地 Git hook 可被显式绕过，因此 CI 是合并的最终权威准入条件。

### 合并前门控

Pull request 和受保护分支还必须通过 CI 中的慢速或基线相关检查：`cargo crap`、`cargo dupes`、`cargo mutants` 及覆盖率检查。它们不塞入每次提交，避免把不稳定或耗时检查伪装成提交级门控。

### 发布前门控

发布前执行 `cargo semver-checks`、性能 bench、完整 `cargo llvm-cov --html` 和上线证据包审查。发布门控依赖版本基线或平台环境，不作为每次本地提交的快速硬门控。

`cargo metadata`、`cargo tree`、`cargo geiger` 是证据检查，除非后续增加解析器或 compile-fail 测试，否则不宣称为自动硬拦截。

当前 workspace 没有 `l1_core` package 或 `architecture` 集成测试，因此不配置虚假的架构测试命令。未来新增与当前 workspace 匹配的架构脚本或测试后，再将其接入对应门控。

## 逐 crate 修改后的检查

## 每个 crate 修改后

每完成一个 crate 的代码修改，必须按以下顺序逐项执行。这里的 `<package>` 是 Cargo package 名，`<crate-directory>` 是该 crate 的目录路径：

```bash
cargo fmt --all -- --check

cargo check -p <package>

cargo test -p <package>

cargo clippy -p <package> --all-targets -- -D warnings

cargo crap -p <package> --summary

cargo dupes stats \
  --path <crate-directory> \
  --exclude-tests

cargo mutants \
  --package <package> \
  --timeout 60
```

一次改动涉及多个 crate 时，每个 crate 都必须单独运行完整流程，不能只用 workspace 命令替代逐 crate 检查。`cargo dupes` 使用 crate 目录定位，`cargo mutants` 使用 Cargo package 名定位。

## 单文件突变测试

只验证某个 Rust 文件时，使用 workspace 根目录下的完整相对路径：

```bash
cargo mutants \
  --package <package> \
  --file '<workspace-relative-file>' \
  --cargo-test-arg '<test-filter>' \
  --timeout 60
```

例如：

```bash
cargo mutants \
  --package example_core_entity \
  --file 'lib/example/entity/src/entity/spot/spot_order_v3.rs' \
  --cargo-test-arg 'spot_order_v3::tests' \
  --timeout 60
```

`--file` 只限制 mutant 生成范围，`--cargo-test-arg` 才限制测试范围；`--file` 不会自动限制 package 的测试范围。

## 所有修改完成后的 workspace 汇总

```bash
cargo fmt --all -- --check

cargo test --workspace

cargo clippy --workspace --all-targets -- -D warnings

cargo crap --workspace --summary

cargo dupes stats \
  --path . \
  --exclude-tests \
  --exclude target \
  --exclude .git
```

全 workspace 突变测试只在明确需要时运行：

```bash
cargo mutants --workspace --timeout 60
```

## 失败处理

- 任一门控失败时，必须报告首个根因。
- `cargo mutants` 基线测试失败、超时、中断或未安装时，标记为未通过。
- 有对应 LCOV 文件时，`cargo crap` 追加 `--lcov <file>` 使用真实覆盖率；没有 LCOV 时，必须注明 CRAP 结果按 0% 覆盖率估算。
- crate 编译失败但静态分析完成时，同时报告编译失败和分析结果。
- 只修改文档、配置或 workspace 根文件时，不执行逐 crate 代码门控，但执行适用的格式、配置或 workspace 验证。
