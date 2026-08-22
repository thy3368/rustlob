# RustLOB 生产上线确定性门控

本文档定义不依赖 `./scripts/` 的生产上线技术准入门控。默认命令只使用 Cargo、Rust 工具链和通用 cargo 子命令；若某条规则没有被编译、测试、clippy 或 cargo 子命令自动拦截，只能标为证据型检查或待工具化，不能宣称已经自动硬拦截。

## 门控通过的含义

生产上线技术准入通过必须满足：本文档列出的全部硬门控命令 exit code 为 0，且相关业务变更的测试与证据包齐备。

技术准入通过只表示代码可以进入生产发布流程，不替代业务审批、发布窗口确认、运维变更、生产配置、密钥、限流、监控、回滚方案和数据迁移确认。

不使用项目脚本时，架构与领域边界只能由编译、compile-fail 测试、clippy、use case/entity 测试和人工读取 `cargo metadata` / `cargo tree` 证据覆盖。凡是这些工具不能自动拦截的规则，必须写成证据型检查或待工具化事项。

## 不使用 ./scripts 的约束

默认上线门控不得调用项目脚本。不要把项目脚本包装进 make、alias 或 shell 函数后作为默认门控使用。

当前限制：

- 不能自动扫描所有 crate 的 core / adapter / infra 分层规则。
- 不能自动识别 adapter 中隐藏的业务规则。
- 不能自动给 use case 业务定义打分。
- 不能把 LLM Review、人工阅读、裸 `cargo metadata` 输出宣传成确定性硬门控。
- 若要把上述规则升级为硬门控，必须后续引入 compile-fail tests、自定义 checker、semgrep 规则或 Rust AST 工具。

## 推荐执行顺序

1. 基础正确性：格式化、编译、测试、clippy。
2. 架构与领域边界：先跑可硬拦截测试，再保存依赖证据。
3. 覆盖率：生成 workspace 覆盖率和 HTML 报告。
4. 安全、供应链、unsafe：跑 cargo deny、audit、geiger。
5. API 兼容性：对比 main 分支。
6. 性能基线：只运行已有 bench target，并与已记录基线比较。
7. 上线证据包：汇总命令、版本、输出路径、风险说明和人工确认项。

## 基础正确性门控

### 格式

用途：确认所有 Rust 代码符合仓库格式规范，避免格式漂移进入发布分支。

命令：

```bash
cargo fmt --all --check
```

通过条件：命令 exit code 为 0。

失败处理：运行 `cargo fmt --all` 修复格式后重新执行本门控。若格式化产生大量无关变更，应拆分提交或回退无关格式变更。

### 默认成员编译与测试

用途：验证 macOS 默认构建路径上的 workspace default-members 能通过编译和测试。

命令：

```bash
cargo test
```

通过条件：命令 exit code 为 0，且没有被忽略但应执行的发布相关测试。

失败处理：修复编译错误或失败测试。不要用删除断言、吞错、空分支或弱语义占位类型绕过失败。

### 全 workspace 编译与测试

用途：验证 workspace 全部成员在当前平台可构建范围内通过测试。

命令：

```bash
cargo test --workspace
```

通过条件：命令 exit code 为 0。macOS 默认不强行构建 Linux/eBPF 专属组件；若某 crate 需要 Linux 内核头、libelf 或特定系统环境，应在证据包中明确标注未在 macOS 执行，并在目标平台补跑。

失败处理：优先判断失败是否为真实代码问题、平台依赖问题或测试环境缺失。真实代码问题必须修复；平台依赖问题必须在目标平台补齐证据。

### 静态质量

用途：用 clippy 拦截常见代码质量、可维护性和部分性能问题。

命令：

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

通过条件：命令 exit code 为 0，且没有通过宽泛 allow 规避真实问题。

失败处理：修复 clippy 报告。新增 allow 必须局部、具体，并能说明为什么该 lint 在当前位置是误报或不适用。

## 架构与领域边界门控

RustLOB 按 core / adapter / infra 理解边界：`core` 包含 entity 与 use case，`adapter` 负责 inbound/outbound 转换，`infra` 承载框架、SDK、DB、runtime、第三方工具。依赖方向应保持为 inbound adapter 调用 use case，use case 使用 entity 和 outbound port，outbound adapter 实现 port 并连接 infra。

### 第一层：可硬拦截的编译/测试门控

用途：用已存在的 compile-fail / architecture test 锁住 core 边界，防止 core 直接依赖 adapter、infra、HTTP、DB、runtime 等外部机制。

命令：

```bash
cargo test -p l1_core --test architecture
```

通过条件：命令 exit code 为 0；任何让 core 依赖 adapter、infra、HTTP、DB、runtime 的反例都必须保持失败。

失败处理：若架构测试失败，先确认是否违反依赖方向。业务规则应留在 core，外部输入输出转换应留在 adapter，框架、SDK、DB、runtime 应留在 infra。不要通过削弱测试或删除 compile-fail 用例来掩盖边界破坏。

### 第二层：Cargo 级结构证据

用途：输出依赖事实，辅助确认 core / adapter / infra 的依赖方向。

命令：

```bash
cargo metadata --no-deps --format-version 1
cargo tree -p l1_core
cargo tree -p axum_server
cargo tree -p l1_adapter
```

通过条件：这些命令本身只构成证据包。没有项目脚本解析时，不把它们宣传为自动硬拦截；审阅者需要读取输出并确认依赖方向没有明显倒置。

失败处理：若命令无法运行，先确认 crate 名称是否变更，再用 `cargo metadata --no-deps --format-version 1` 查找实际 package 名。若输出显示 core 直接依赖 adapter/infra，必须重构依赖方向或补充硬拦截测试。

### 第三层：领域语义测试

用途：验证业务规则留在 entity/use case 层，并覆盖金融状态变更的确定性推导。

命令：

```bash
cargo test -p <core_crate>
cargo test -p <business_crate>
```

通过条件：相关 crate 的 entity 测试、use case 测试、集成测试全部通过。金融业务变更必须覆盖 entity 不变量、`Command + GivenState -> Changes`、拒绝路径、幂等、重放/事件一致性。

失败处理：缺少测试时先补测试，再修实现。若发现 adapter 中承载核心业务规则，应把规则上移到 core 的 entity 或 use case，并用测试锁住。

## 测试与覆盖率门控

### Workspace 覆盖率

用途：度量测试覆盖范围，识别发布变更附近的明显测试缺口。

命令：

```bash
cargo llvm-cov --workspace
```

通过条件：命令 exit code 为 0；覆盖率结果必须进入证据包。本文档不凭空指定统一覆盖率阈值，发布前必须与既有基线或本次发布要求比较。

失败处理：若覆盖率工具未安装，先安装对应 cargo 子命令后重跑。若覆盖率低于既有基线或发布要求，必须补测试或记录被批准的风险例外。

### HTML 覆盖率报告

用途：生成可审阅的覆盖率报告，便于定位未覆盖文件和分支。

命令：

```bash
cargo llvm-cov --html
```

通过条件：命令 exit code 为 0，HTML 报告路径记录在上线证据包中。

失败处理：修复工具、编译或测试失败后重跑。不要只保留终端摘要而丢失可审阅报告。

## 安全、供应链与 unsafe 门控

### 供应链策略

用途：检查 Rust 依赖的 advisories、licenses、sources 和 bans 策略。

命令：

```bash
cargo deny check advisories licenses sources bans --exclude-dev
```

通过条件：命令 exit code 为 0。

失败处理：升级、替换或移除问题依赖。许可证或来源例外必须有明确业务理由和审批记录。

### 漏洞补充检查

用途：用 RustSec advisory database 补充扫描依赖漏洞。

命令：

```bash
cargo audit
```

通过条件：命令 exit code 为 0。

失败处理：升级或替换受影响依赖。若短期无法修复，必须记录漏洞编号、影响面、缓解措施、到期时间和责任人。

### unsafe 统计

用途：统计 unsafe 使用位置，为人工审阅和风险记录提供证据。

命令：

```bash
cargo geiger
```

通过条件：命令 exit code 为 0，输出进入证据包。`cargo geiger` 是证据型检查，不等同于证明 unsafe 正确。

失败处理：若新增 unsafe，必须有局部 safety comment，并补充验证。生产代码不得新增无说明的 unsafe。

## API 兼容性门控

用途：检查公开 Rust API 相对 main 分支是否存在语义化版本不兼容变更。

命令：

```bash
cargo semver-checks check-release --baseline-rev main
```

通过条件：命令 exit code 为 0，或不兼容变更已被明确批准并反映在版本、发布说明和下游迁移计划中。

失败处理：优先保持兼容；确需破坏兼容时，必须记录影响 crate、破坏点、迁移方式和发布版本策略。

## 性能基线门控

用途：确认低延迟关键路径没有相对既有基线发生未经批准的性能退化。

命令：

```bash
cargo bench -p <crate> --bench <bench_name> -- --noplot
```

通过条件：只运行仓库已有 bench target；结果与已记录基线或本次发布前明确记录的基线比较，未出现未经批准的退化。

失败处理：先排除本机负载、CPU governor、热身不足等测量噪声，再定位分配、锁竞争、cache locality、序列化等低延迟风险。确有退化时必须修复或取得发布例外。

## 金融业务不变量门控

用途：确保交易、钱包、结算、行情、推送等金融业务变更保持确定性、可拒绝、可回放。

命令：

```bash
cargo test -p <business_crate>
cargo test -p <core_crate>
```

通过条件：相关测试覆盖以下内容：

- entity 不变量和构造校验。
- use case 的 `Command + GivenState -> Changes` 确定性推导。
- 非法命令、状态冲突、余额不足、精度错误、重复请求等拒绝路径。
- 幂等、重放、事件投影一致性。
- 与本次变更相关的集成路径。

失败处理：缺失哪类业务风险，就补哪类测试。不要把业务规则下沉到 inbound/outbound adapter，也不要用生产路径 `unwrap()` / `expect()` 替代明确错误。

## 上线证据包

每次生产上线至少保存以下证据：

- Git commit、分支、Rust toolchain、操作系统和关键环境变量。
- 全部硬门控命令、执行时间、exit code、关键输出摘要。
- `cargo metadata --no-deps --format-version 1` 输出。
- 关键 crate 的 `cargo tree` 输出。
- 覆盖率摘要和 HTML 报告路径。
- `cargo deny`、`cargo audit`、`cargo geiger` 输出。
- API 兼容性检查结果。
- 性能 bench 结果和对比基线。
- 业务审批、发布窗口、运维变更、生产配置、监控、回滚方案的确认记录。
- 未自动化覆盖的证据型检查、人工判断和已批准例外。

## 常见失败处理

工具未安装：安装对应 cargo 子命令后重跑，不要跳过硬门控。常见工具包括 `cargo-llvm-cov`、`cargo-deny`、`cargo-audit`、`cargo-geiger`、`cargo-semver-checks`。

平台依赖失败：区分 macOS 默认路径和 Linux/eBPF 专属路径。目标平台专属组件必须在目标平台补充执行证据。

依赖方向失败：按 core / adapter / infra 重新放置职责。core 不能直接依赖 HTTP、DB、SDK、runtime 或具体 adapter。

覆盖率或性能缺少基线：发布前先记录当前基线，并让审批方明确接受该基线。不要临时编造阈值。

安全漏洞无法立即修复：记录漏洞编号、影响面、缓解措施、到期时间和责任人，并取得发布批准。
