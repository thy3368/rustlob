# PROJECT KNOWLEDGE BASE

**Generated:** 2026-06-15
**Commit:** 7ef4b84
**Branch:** main

## CONVENTIONS (THIS PROJECT)

- **注释/说明**: 优先使用中文。
- **use case command 建模**: 优先参考 Hyperliquid endpoint API，字段语义更精确。
- **目录风格**: 不是传统单 crate `src/` 结构，而是 workspace 多 crate 横向拆分。
- **架构分层**: 优先用 `core / adapter / infra` 语义理解代码边界，不要把业务规则塞进 adapter。
- **命名**: 模块使用 snake_case，类型使用 PascalCase。
- **错误处理**: 使用 `Result<T, E>` 和 `?`，避免在生产路径里直接 `unwrap()` / `expect()`。
- **测试组织**: 既有 inline `#[test]`，也有独立测试 crate 与 `tests/` 风格。
- **性能取向**: 默认按低延迟系统思路读代码，关注分配、锁竞争、cache locality、序列化开销。

## USEFUL COMMANDS

```bash
# 格式化
cargo fmt --all

# 默认成员测试（macOS 更稳妥）
cargo test

# 全 workspace 测试
cargo test --workspace

# lint
cargo clippy -- -D warnings

# 查看 workspace 成员
cargo metadata --no-deps --format-version 1
```

## AGENT CRAP CHECKS

- Agent 每次完成一个 crate 的代码改动后，必须立即运行 `cargo crap -p <package> --summary`，再继续修改其他 crate。
- 一次改动涉及多个 crate 时，必须按 package 分别运行，不能只用一次 workspace 汇总代替逐 crate 检查。
- 有对应 LCOV 文件时，追加 `--lcov <file>` 使用真实覆盖率；没有时必须在交付说明中标注结果按 0% 覆盖率估算。
- `cargo crap` 执行失败时停止后续代码改动并报告首个根因；crate 编译失败但静态分析成功时，同时报告编译失败和静态 CRAP 结果。
- 所有 crate 修改完成后，再运行 `cargo crap --workspace --summary` 作为整体汇总；仅修改文档、配置或 workspace 根文件时，只需运行
  workspace 汇总。
- Agent 每次完成一个 crate 的代码改动后，还必须运行 `cargo dupes stats --path <crate-directory> --exclude-tests` 和 `cargo mutants --package <package> --timeout 60`。
- `cargo dupes` 用 crate 目录定位，`cargo mutants` 用 Cargo package 名定位；多个 crate 改动时三项检查都必须逐 crate 执行。
- 如果只检查指定 Rust 文件，使用 `cargo mutants --package <package> --file '<relative-file>' --cargo-test-arg '<test-filter>' --timeout 60`；`--file` 只限制生成 mutant 的文件，`--cargo-test-arg` 才限制运行的测试。路径包含 `/` 时按 workspace 根目录传入完整相对路径。
- 例如只检查 `lib/example/entity/src/entity/spot/spot_order_v3.rs`：`cargo mutants --package example_core_entity --file 'lib/example/entity/src/entity/spot/spot_order_v3.rs' --cargo-test-arg 'spot_order_v3::tests' --timeout 60`。
- `cargo mutants` 未安装、基线测试失败、执行超时或工具失败时，必须报告失败原因，不得把未运行当作通过；全部改动完成后再运行对应的 workspace 汇总检查。


