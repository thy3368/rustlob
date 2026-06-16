# PROJECT KNOWLEDGE BASE

**Generated:** 2026-06-15
**Commit:** 7ef4b84
**Branch:** main

## OVERVIEW

RustLOB 是一个用 Rust 编写的超低延迟交易系统/交易所单体仓库，当前 workspace 有 **53 个成员 crate**，其中 **49 个 default-members**。代码组织整体遵循 DDD + Clean Architecture，但目录布局已经从旧版 `app + lib + proc` 演进为 **`app + inbound_adapter + lib + operating + study`**。

## STRUCTURE

```text
rustlob/
├── app/                # 可执行程序与运行时集成（client, pingora, xdp）
├── inbound_adapter/    # HTTP / WebSocket 入站适配器
├── lib/                # 通用库、核心领域、example、veldra
├── operating/          # CEX / DEX / L2 业务用例与流程编排
├── study/              # 研究性项目与外部协议分析
├── design/             # 架构文档、交易所 API 资料、部署说明
├── scripts/            # 辅助脚本
└── Cargo.toml          # Workspace 根配置
```

## WHERE TO LOOK

| 任务 | 位置 | 说明 |
|------|------|------|
| 撮合盘口核心 | `lib/common/lob_repo/` | LOB 数据结构与撮合基础能力 |
| 精度与定点数 | `lib/common/fixed_point/` | 价格/数量精度处理 |
| HTTP 服务 | `inbound_adapter/axum_server/` | Axum REST 接口 |
| WebSocket 服务 | `inbound_adapter/websocket_sockudo/` | 实时推送入口 |
| CEX 现货业务 | `operating/cex/exchange/spot/` | 现货交易 use case |
| 衍生品业务 | `operating/cex/exchange/derivatives/` | 衍生品 use case |
| 钱包业务 | `operating/cex/exchange/wallet/` | 账户/钱包相关用例 |
| 行情业务 | `operating/cex/exchange/spot_market_data/` | 现货行情流程 |
| 推送链路 | `operating/cex/push/push/` | push 侧业务编排 |
| L1 核心 | `lib/core/l1/` | 较底层核心能力 |
| L1 端到端 | `lib/core/l1_e2e/` | 集成/E2E 验证 |
| Veldra 体系 | `lib/veldra/` | 独立 bounded context |
| Hyperliquid 研究 | `study/hyperliquid_analyzer/` | 外部协议分析与参考 |
| XDP 加速 | `app/xdp_libbpf/` | Linux/eBPF 相关程序 |

## WORKSPACE SNAPSHOT

- `members`: 53
- `default-members`: 49
- `resolver`: `2`
- `edition`: `2021`
- `release profile`: `lto = "fat"`, `codegen-units = 1`, `opt-level = 3`, `target-cpu = "native"`
- `rustfmt`: `style_edition = "2024"`, `imports_granularity = "Module"`, `group_imports = "StdExternalCrate"`
- `.github/workflows/`: 当前缺失

## IMPORTANT PATHS

| 符号/模块 | 类型 | 位置 | 说明 |
|-----------|------|------|------|
| `lob_repo` | lib | `lib/common/lob_repo` | 盘口与撮合核心 |
| `fixed_point` | lib | `lib/common/fixed_point` | 定点数精度能力 |
| `axum_server` | bin | `inbound_adapter/axum_server` | HTTP 入口 |
| `websocket_sockudo` | bin | `inbound_adapter/websocket_sockudo` | WebSocket 入口 |
| `spot` | lib | `operating/cex/exchange/spot` | 现货业务 |
| `derivatives` | lib | `operating/cex/exchange/derivatives` | 衍生品业务 |
| `wallet` | lib | `operating/cex/exchange/wallet` | 钱包业务 |
| `xdp_libbpf` | bin | `app/xdp_libbpf` | Linux XDP/eBPF 程序 |

## CONVENTIONS (THIS PROJECT)

- **注释/说明**: 优先使用中文。
- **use case command 建模**: 优先参考 Hyperliquid endpoint API，字段语义更精确。
- **目录风格**: 不是传统单 crate `src/` 结构，而是 workspace 多 crate 横向拆分。
- **架构分层**: 优先用 `core / adapter / infra` 语义理解代码边界，不要把业务规则塞进 adapter。
- **命名**: 模块使用 snake_case，类型使用 PascalCase。
- **错误处理**: 使用 `Result<T, E>` 和 `?`，避免在生产路径里直接 `unwrap()` / `expect()`。
- **测试组织**: 既有 inline `#[test]`，也有独立测试 crate 与 `tests/` 风格。
- **性能取向**: 默认按低延迟系统思路读代码，关注分配、锁竞争、cache locality、序列化开销。

## CURRENT REALITY / RISKS

- `TODO` 总数约 **96**，说明仍有明显未完成功能和技术债。
- 仓库里 `unwrap()` / `expect()` 调用约 **1997** 处，很多可能是测试或实验代码，但不能假定生产路径都已清理。
- 测试标记（`#[test]` / `tokio::test` / `proptest!` / `kani::proof`）约 **1104** 处，说明验证覆盖面不低，但不代表都在默认构建路径中执行。
- `app/xdp_libbpf` 依赖 Linux 内核头与 `libelf`，**macOS 默认不应强行构建**；当前已从根 workspace 成员中移出。
- `study/` 下内容偏研究/实验，不要把其中模式直接当成生产实现标准。

## ANTI-PATTERNS (ENFORCE DURING EDITS)

- 不要在生产代码新增 `unwrap()` / `expect()`
- 不要新增没有 safety comment 的 `unsafe`
- 不要引入全局可变状态
- 不要把业务规则下沉到 inbound/outbound adapter
- 不要留下新的 `TODO` 作为“完成”
- 不要为了通过编译而用弱语义占位类型或空分支吞错

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

## NOTES

- 旧资料中提到的 `proc/operating/exchange/` 已经过时；当前主要业务路径在 `operating/cex/exchange/`。
- 旧资料中提到的 `app/axum_server/`、`app/websocket_sockudo/` 已迁移到 `inbound_adapter/`。
- `app/xdp_libbpf/libbpf-rs-0.24.8/` 下有 vendored 第三方 crate，不要误当作本项目主业务模块。
- 做 use case 相关改动时，先确认它属于 `core`、`operating` 还是 `adapter`，再决定放置位置。
