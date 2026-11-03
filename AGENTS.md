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

## AGENT QUALITY GATES

所有提交前硬门控、逐 crate 检查、失败处理和门控分层见 [QUALITY_GATES.md](QUALITY_GATES.md)。

- 本地提交必须启用版本化 hook：`git config core.hooksPath .githooks`。
- 提交前门控执行快速且确定的格式、编译、测试、Clippy、架构、依赖审计检查。
- 合并前门控由 CI 执行慢速或依赖基线的 CRAP、重复依赖和突变测试。
- 发布前门控包含 API 兼容性、性能、完整覆盖率和上线证据审查。
- Git hook 可以被本地绕过；CI 结果是合并的最终准入条件。

## MUTATION TESTING 经验

- 针对单个 use case 文件执行变异测试时，优先限定 package、file 和测试过滤条件，减少 workspace 中无关测试的影响：
  `cargo mutants --package <crate> --file '<path>' --no-shuffle --baseline skip --cargo-test-arg '<test-filter>'`。
- 首次运行应保留默认 baseline，确认完整 crate 测试状态；如果 workspace 中已有与目标文件无关的失败，不能把这些失败误判为目标变异问题。此时记录失败测试，并使用 `--baseline skip` 配合目标测试过滤器继续定位。
- 每个存活 mutant 都应补充针对业务边界的行为测试，不修改生产逻辑迎合 mutant。尤其要覆盖比较运算的边界值，例如 `< 0` 必须同时测试负数、零和正数，避免 `<= 0` 这类变异存活。
- 变异测试完成后应区分 `caught`、`missed` 和 `unviable`：`missed` 必须补测试；`unviable` 通常是编译不可行的变异，需要记录数量和原因；完整 crate 的既有失败应单独报告。


