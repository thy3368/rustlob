# Rust 项目 7 大门控实施方案

## 一、总体架构

Rust 工具链高度统一在 `cargo` 生态，编译器自带内存安全与类型安全，因此相比 C++ 方案：**工具更少、配置更简、检查更严**。

```
┌─────────────────────────────────────────────────────────┐
│ 门控1 代码规范       → rustfmt + clippy                  │
│ 门控2 构建与测试     → cargo + rustc + cargo-tarpaulin + miri │
│ 门控3 静态质量与安全 → clippy + cargo-audit + cargo-geiger │
│ 门控4 接口与契约     → cargo-semver-checks + buf + pact  │
│ 门控5 Agentic 智能审查 → OCR + 沙箱执行（语言无关）        │
│ 门控6 性能与资源     → criterion + tokio-console + perf  │
│ 门控7 集成与业务合规 → cucumber + tracing + 业务规则引擎  │
├─────────────────────────────────────────────────────────┤
│ 统一输出：SARIF / JSON / JUnit XML → 效能平台聚合         │
└─────────────────────────────────────────────────────────┘
```

---

## 二、门控1：代码规范门控

### 目标
把主业务代码的格式与基础 lint 统一收口到 `rustfmt + clippy`，用现有配置文件作为唯一口径，避免每个 crate 各自漂移出一套规则。

### 工具选型

| 工具 | 职责 | 配置文件 |
|---|---|---|
| `rustfmt` | 代码格式化 | `rustfmt.toml` |
| `clippy` | 基础 lint、命名、风格、可读性、局部性能建议 | `clippy.toml` |

### 作用范围

- 默认严格执行 `app/`、`inbound_adapter/`、`lib/`、`operating/` 中的主业务 crate；`study/` 这类研究/分析 crate 默认不作为同一层级的硬门槛，按单独纳管或阶段性收紧
- 明确豁免生成物、vendored 第三方代码、测试、示例、基准目录
- `#[allow(...)]` 只接受局部、可审查、带理由的豁免，不接受 crate 级的宽泛放行
- `missing_docs` 不作为门控1的全仓库硬门槛，只对新公开 API 或单独纳管的 crate 逐步收紧
- `unwrap`、`expect`、`panic!` 仍按仓库根部 `clippy.toml` 作为硬禁止项处理

### 关键配置

**rustfmt.toml**
```toml
edition = "2021"
style_edition = "2024"
use_small_heuristics = "Max"
merge_derives = false
group_imports = "StdExternalCrate"
imports_granularity = "Module"
use_field_init_shorthand = true
```

**clippy.toml**
```rust
// 仓库根部统一配置
// - 禁止 unwrap / expect / panic
// - 对需要豁免的 lint 做局部 allow，而不是 crate 级放行
```

### 集成方式
- **本地**：`cargo fmt --all --check`，`cargo clippy --workspace --all-targets -- -D warnings`
- **CI**：同一组命令作为门禁，不再额外引入第二套 lint 基线
- **IDE**：rust-analyzer 实时提示

---

## 三、门控2：构建与测试门控

### 目标
拦截编译错误/告警、类型误用、单元测试失败、覆盖率不达标、未定义行为。

### 工具选型

| 维度 | 工具 | 说明 |
|---|---|---|
| 构建 | `cargo build` | 统一构建系统，无需 CMake |
| 编译器 | `rustc`（stable 最新） | `-D warnings` 将告警升级为错误 |
| 单元测试 | `cargo test`（内置） | 原生 `#[test]`，无需第三方框架 |
| 覆盖率 | `cargo-tarpaulin` / `cargo-llvm-cov` | 行覆盖率 + 分支覆盖率 |
| 未定义行为 | `miri`（需 nightly） | 解释执行字节码，检测 UB |
| 内存泄漏 | `miri` / `valgrind` | 运行时泄漏检测 |

### 关键命令

```bash
# 编译告警零容忍
RUSTFLAGS="-D warnings" cargo build --release

# 运行全部测试（单元 + 集成 + 文档测试）
cargo test --all-targets

# 覆盖率门禁（行覆盖率 ≥70%，分支 ≥60%）
cargo tarpaulin --out Xml --fail-under-line 70 --fail-under-branch 60

# Miri 检测未定义行为
cargo +nightly miri test
```

### Rust 特有优势
编译器默认防住空指针解引用、数据竞争、越界访问、use-after-free——C++ 需靠 ASan/UBSan 运行时才发现的问题，Rust 编译期就拦截。

---

## 四、门控3：静态质量与安全门控

### 目标
拦截可疑逻辑、恒真/恒假分支、性能反模式、依赖 CVE、`unsafe` 代码滥用、许可证不合规。

### 工具选型

| 层级 | 工具 | 定位 |
|---|---|---|
| L1 深度 Lint | `clippy`（`correctness` / `suspicious` / `perf` 类别） | 编译期静态分析，500+ 规则 |
| L2 模式扫描 | `semgrep`（支持 Rust） | 自定义规则，跨语言统一 |
| L3 依赖安全 | `cargo-audit` | 扫描 RUSTSEC 数据库的 CVE |
| L4 许可证/废弃 | `cargo-deny` | 许可证白名单、废弃依赖、重复版本检查 |
| L5 unsafe 审计 | `cargo-geiger` | 统计 `unsafe` 块数量与覆盖率，防止蔓延 |
| L6 企业平台 | SonarQube（Rust Plugin） | 长期质量门禁与趋势 |

### 关键命令

```bash
# 依赖漏洞扫描，发现漏洞即失败
cargo audit -D warnings

# 许可证 + 漏洞 + 废弃包统一检查
cargo deny check

# unsafe 代码统计
cargo geiger --output-format Json
```

### 关键检查项

| 问题类型 | 工具 | 规则示例 |
|---|---|---|
| 恒真/恒假条件 | clippy | `clippy::const_is_empty` / `clippy::absurd_extreme_comparisons` |
| 可疑比较 | clippy | `clippy::suspicious_comparisons` |
| 未使用结果 | clippy | `clippy::must_use_candidate` |
| 性能反模式 | clippy | `clippy::perf` 类别（如 `clippy::redundant_clone`） |
| 依赖 CVE | cargo-audit | RUSTSEC 全量匹配 |
| unsafe 蔓延 | cargo-geiger | 新增 unsafe 块需审批 |

---

## 五、门控4：接口与契约门控

### 目标
拦截 API 破坏性变更、Protobuf 协议非兼容变更、跨模块契约破坏、返回值被忽略。

### 工具选型

| 维度 | 工具 | 说明 |
|---|---|---|
| API 语义版本兼容 | `cargo-semver-checks` | 基于 rustc 编译数据，精准检测 breaking change |
| Protobuf 兼容 | `buf breaking` | 字段编号/类型/标签变更检查 |
| 契约测试 | `pact_consumer` / `pact_verifier` | 消费者驱动契约测试 |
| 返回值契约 | `#[must_use]` + clippy | 编译器属性 + `clippy::must_use_candidate` |

### 关键命令

```bash
# 对比上一个版本，检测 API 是否有 semver breaking change
cargo semver-checks check-release --baseline-rev HEAD~1

# Protobuf 兼容性检查
buf breaking proto/ --against ".git#branch=main,subdir=proto"
```

### Rust 与 C++ 的关键差异

- **无 ABI 兼容问题**：Rust ABI 不稳定，动态链接不常见，无需 `abidiff`
- **API 检查更精准**：`cargo-semver-checks` 能识别"新增 trait impl 导致的破坏性变更"等语义级变化，直接告知该 bump 大/中/小版本
- **`#[must_use]` 为语言内置属性**，无需编译器扩展

---

## 六、门控5：Agentic 智能审查门控

### 目标
拦截未校验返回值、隐含语义变化、范围外修改、设计合理性问题，并通过沙箱执行复现异常分支。

### 工具选型（语言无关）

| 阶段 | 工具 | 说明 |
|---|---|---|
| 语义审查 | OCR（确定性工程 + LLM） | diff 解析 → 上下文构建 → LLM 语义判断 |
| 沙箱执行 | 自研沙箱执行器 | 编译可疑片段，构造异常数据运行验证 |

### Rust 项目审查重点
- `unwrap()` / `expect()` 使用是否合理（是否可能 panic）
- `unsafe` 块是否有安全注释（`// SAFETY:`）
- 异步代码中是否存在阻塞点（`std::sync::Mutex` 在 async 中持有）
- `?` 运算符传播的错误类型是否合理
- 生命周期标注是否引入不必要的约束

---

## 七、门控6：性能与资源门控

### 目标
拦截延迟回归、吞吐量下降、内存泄漏、资源耗尽、异步任务堆积。

### 工具选型

| 维度 | 工具 | 说明 |
|---|---|---|
| 基准测试 | `criterion`（`cargo bench`） | 统计严谨，自动检测性能回归 |
| 缓存级基准 | `iai` | 基于 Cachegrind，测量指令/缓存命中，无噪声 |
| 火焰图 | `cargo flamegraph` / `samply` | CPU 性能画像 |
| 内存画像 | `dhat` / `valgrind --tool=massif` | 堆分配统计与泄漏检测 |
| 异步监控 | `tokio-console` | Tokio 运行时任务堆积、阻塞点、任务泄漏 |
| 压测 | `oha` / `wrk` / 自研压测平台 | HTTP/gRPC 吞吐量与延迟测试 |

### 关键配置

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
iai = "0.1"
dhat = "0.3"
```

```bash
# 基准测试（自动对比基线，回归超过阈值即失败）
cargo bench

# 生成火焰图
cargo flamegraph --bin myapp --bench mybench

# 异步运行时监控
tokio-console
```

### Rust 特有维度
`tokio-console` 是 Rust 异步生态独有的可观测性工具，能实时查看每个异步任务的状态、创建位置、等待时长——C++ 异步方案（回调/协程）没有统一运行时，无法做到这一点。

---

## 八、门控7：集成与业务合规门控

### 目标
拦截端到端链路回归、业务不变量破坏、合规审计缺失、监管报送字段不全。

### 工具选型

| 维度 | 工具 | 说明 |
|---|---|---|
| BDD 集成测试 | `cucumber-rust` | 自然语言描述测试用例，业务可读 |
| 端到端测试 | `cargo test`（`tests/` 目录） | 原生集成测试支持 |
| 结构化日志 | `tracing` + `tracing-subscriber` | JSON 日志、span 链路追踪，合规审计必备 |
| 业务规则引擎 | 自研 + `rhai`（嵌入式脚本） | 动态业务规则，无需重新编译 |
| 合规检查 | 自研脚本 + `cargo-deny`（许可证） | 审计日志完整性、权限校验、监管字段 |

### 关键配置

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
rhai = "1.16"
```

```rust
// 初始化结构化 JSON 日志（合规审计格式）
tracing_subscriber::fmt()
    .json()
    .with_env_filter("info")
    .with_current_span(true)
    .init();
```

### Rust 特有优势
`tracing` 原生支持结构化 JSON 日志 + span 链路追踪，金融合规要求的"每笔交易全链路可追溯"无需额外框架即可实现。

---

## 九、工具版本矩阵

| 门控 | 工具 | 推荐版本 | 输出格式 | 许可证 |
|---|---|---|---|---|
| 1 | rustfmt | 随 stable 工具链 | plain | MIT/Apache |
| 1 | clippy | 随 stable 工具链 | SARIF / JSON | MIT/Apache |
| 2 | rustc | stable 最新（≥1.75） | 编译日志 | MIT/Apache |
| 2 | cargo | 随 rustc | - | MIT/Apache |
| 2 | cargo-tarpaulin | 0.27+ | Cobertura XML | MIT/Apache |
| 2 | miri | nightly 最新 | plain | MIT/Apache |
| 3 | cargo-audit | 0.18+ | JSON | MIT/Apache |
| 3 | cargo-deny | 0.14+ | JSON | MIT/Apache |
| 3 | cargo-geiger | 0.11+ | JSON | MIT |
| 4 | cargo-semver-checks | 0.24+ | JSON | Apache-2.0 |
| 4 | buf | 1.30+ | JSON | Apache-2.0 |
| 4 | pact_consumer/verifier | 1.0+ | Pact JSON | MIT |
| 5 | OCR + 沙箱 | 自研 | JSON | - |
| 6 | criterion | 0.5+ | HTML / JSON | Apache-2.0/MIT |
| 6 | tokio-console | 0.1+ | 实时 | MIT |
| 7 | cucumber-rust | 0.20+ | JSON / JUnit | MIT |
| 7 | tracing | 0.1+ | JSON | MIT |

---

## 十、门禁阈值

| 门控 | 拦截条件 | 豁免机制 |
|---|---|---|
| 1 代码规范 | `cargo fmt --all --check` 有 diff → 失败；`cargo clippy --workspace --all-targets -- -D warnings` 报错 → 失败 | `#[allow(clippy::xxx)]` 仅限局部 + Reviewer 确认 |
| 2 构建测试 | 编译 warning ≥1 → 失败（`-D warnings`）；测试失败 → 失败；行覆盖率 <70% → 失败 | 覆盖率豁免需标注原因审批 |
| 3 静态安全 | clippy correctness 级 ≥1 → 失败；cargo-audit 发现漏洞 → 失败；新增 unsafe 块 → 需审批 | 漏洞可标记 `ignore` + 安全负责人审批 |
| 4 接口契约 | cargo-semver-checks 报 breaking change → 失败（除非 bump 大版本）；buf breaking 非兼容 → 失败 | 大版本升级同步更新 CHANGELOG |
| 5 AI 审查 | High 级语义问题 ≥1 → 需人工 Reviewer 确认 | 标记 false positive + 理由 |
| 6 性能资源 | 核心接口 P99 延迟劣化 >10% → 失败；基准测试回归 >5% → 失败；内存泄漏趋势 → 失败 | 性能豁免需架构师审批 |
| 7 业务合规 | 端到端链路失败 → 失败；业务不变量破坏 → 失败；审计日志字段缺失 → 失败 | 合规问题零豁免 |

---

## 十一、CI 配置示例（GitHub Actions）

```yaml
name: Rust Gate Checks

on: [pull_request, push]

jobs:
  # ── 门控1：代码规范 ──
  gate1-lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - name: rustfmt
        run: cargo fmt --all --check
      - name: clippy
        run: cargo clippy --workspace --all-targets -- -D warnings

  # ── 门控2：构建与测试 ──
  gate2-build-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Build
        run: RUSTFLAGS="-D warnings" cargo build --release
      - name: Test
        run: cargo test --all-targets
      - name: Coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml --fail-under-line 70 --fail-under-branch 60

  # ── 门控3：静态质量与安全 ──
  gate3-static-security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Dependency Audit
        run: |
          cargo install cargo-audit
          cargo audit -D warnings
      - name: License & Deprecation Check
        run: |
          cargo install cargo-deny
          cargo deny check
      - name: Unsafe Audit
        run: |
          cargo install cargo-geiger
          cargo geiger --output-format Json

  # ── 门控4：接口与契约 ──
  gate4-api-contract:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          fetch-depth: 0  # semver-checks 需要历史
      - uses: dtolnay/rust-toolchain@stable
      - name: Semver Check
        run: |
          cargo install cargo-semver-checks
          cargo semver-checks check-release --baseline-rev origin/main
      - name: Protobuf Breaking Check
        uses: bufbuild/buf-setup-action@v1
        with:
          github_token: ${{ github.token }}
      - run: buf breaking proto/ --against ".git#branch=main,subdir=proto"

  # ── 门控5：Agentic 智能审查 ──
  gate5-agentic-review:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: OCR Semantic Review
        run: ocr review --diff origin/main --output sarif
      - name: Sandbox Execution
        run: ocr sandbox --targets high-risk --output json

  # ── 门控6：性能与资源 ──
  gate6-performance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: Benchmark
        run: cargo bench -- --output-format bencher | tee bench.txt
      - name: Performance Regression Check
        run: |
          # 对比基线，回归 >5% 即失败
          cargo install critcmp
          critcmp --threshold 5 baseline.txt bench.txt

  # ── 门控7：集成与业务合规 ──
  gate7-integration-compliance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - name: E2E Tests
        run: cargo test --test '*' -- --nocapture
      - name: BDD Tests
        run: cargo test --test cucumber
      - name: Compliance Check
        run: ./scripts/compliance-check.sh
```

---

## 十二、落地路径

| 阶段 | 周期 | 内容 | 交付物 |
|---|---|---|---|
| P0 基线 | 第1周 | 统一 rustfmt/clippy 配置；CI 接入 build + test + 覆盖率 | `rustfmt.toml`、CI 流水线 |
| P1 安全 | 第2周 | 接入 cargo-audit + cargo-deny + cargo-geiger；清理存量告警 | 依赖安全报告、unsafe 基线 |
| P2 契约 | 第3周 | 接入 cargo-semver-checks + buf breaking；Pact 契约试点 | API 兼容性基线 |
| P3 性能 | 第4周 | criterion 基准测试接入；tokio-console 接入异步服务；建立性能基线 | 性能基准报告 |
| P4 业务 | 第5-6周 | cucumber 集成测试覆盖核心链路；tracing 结构化日志；合规检查脚本 | 端到端用例集、合规看板 |
| P5 AI | 持续 | OCR 语义审查接入；沙箱执行覆盖高风险模块；阈值调优 | AI 审查报告 |

---

## 十三、Rust  vs C++ 方案核心差异总结

| 维度 | C++ | Rust |
|---|---|---|
| 构建系统 | CMake + Ninja（需手写配置） | cargo（开箱即用） |
| 内存安全 | 靠 ASan/UBSan 运行时检测 | 编译器编译期保证 |
| Lint 工具 | clang-tidy + cpplint（两个工具） | clippy（一个工具全覆盖） |
| 测试框架 | GoogleTest（需引入） | 内置 `#[test]` |
| ABI 兼容 | abidiff（二进制对比） | 无 ABI 问题，用 cargo-semver-checks（API 语义级） |
| 特有维度 | - | unsafe 审计（cargo-geiger）、异步监控（tokio-console） |
| 工具数量 | 每门控 3-5 个工具拼凑 | 每门控 1-2 个 cargo 子命令 |

> **一句话**：Rust 方案用更少的工具实现更严的检查，省下来的精力可以放在业务合规和性能优化上。
