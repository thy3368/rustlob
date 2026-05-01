# Rust之从0-1低时延DEX: L1 设计之 Rust VM / EVM 单节点无共识版

很多人在看 L1 架构时，第一反应都是共识：HotStuff、Tendermint、PoS、BFT、finality。

但如果把问题再拆开一点，你会发现：

**在共识之前，其实还有一个更基础的问题——L1 到底怎么执行。**

交易从哪里进来？
怎么进入 mempool？
区块是谁来触发执行？
执行时如何分发到不同 runtime？
状态怎么落盘？
如果一条链既想支持 Rust 原生产品逻辑，又想支持 Solidity contract，它的执行骨架应该怎么设计？

这篇文章讲的，就是这样一个版本：

**一个单节点、无共识、同时支持 Rust VM 和 EVM 的 L1 最小实现。**

它不是最终形态，但它把最关键的执行主链路先跑通了。对后续要接 HotStuff、要继续扩展 `spot` / `perp` / `option` / Solidity 能力的人来说，这一步反而是整个架构最重要的地基。

---

## 一、为什么先做“无共识版 L1”

很多系统一上来就把执行、网络、共识、存储一起做，最后的问题是：

- 一旦跑不通，不知道是执行错了，还是共识错了
- 一旦状态不对，不知道是 runtime 逻辑错了，还是 commit 路径错了
- 一旦扩展 VM，执行框架和共识框架又会互相缠住

所以更合理的顺序通常不是“先共识”，而是：

1. 先把交易入口打通
2. 先把 mempool 跑起来
3. 先把区块执行骨架稳定下来
4. 先把状态持久化模型固定下来
5. 先验证不同 runtime 能否共存
6. 最后再在外层包上共识

也就是说，**先把 execution layer 做扎实，再谈 consensus layer。**

这也是当前这个单节点版本的设计初衷。

它要解决的核心问题不是“多节点一致性”，而是：

> 一条 L1 执行链路，如何同时承载 Rust VM 和 EVM 两种执行环境。

---

## 二、这版 L1 到底是什么

如果要用一句话描述当前实现，可以这样说：

> 一个通过 REST 接收交易、通过内存 mempool 暂存请求、通过统一执行框架分发到 Rust VM / EVM、并最终通过 MDBX 持久化状态的单机版 L1。

它有几个非常鲜明的特征：

- 单节点执行
- 不做共识
- inbound 使用 REST API
- mempool 使用 `InMemoryMempool`
- 状态持久化使用 `MdbxStateStore`
- 区块执行时通过 `VmRegistry` 在多个 runtime 之间路由

在这个设计里，L1 不直接绑死某一种执行环境，而是提供一个统一的执行骨架。至于具体执行逻辑是 Rust VM 还是 EVM，由 runtime 自己负责。

这意味着这版设计的重点不是“链上治理”或“网络协商”，而是：

- 执行抽象是否清晰
- runtime 边界是否清晰
- 状态提交路径是否清晰
- 后续是否容易继续接共识

---

## 三、为什么是 Rust VM + EVM 双 runtime

只做 Rust VM，问题会比较简单：

- 产品逻辑直接写在 Rust 里
- 性能和领域控制力都比较强
- 很适合做 `spot`、`perp`、`option` 这类强约束业务

只做 EVM，也很直接：

- 对 Solidity 生态兼容更自然
- 合约隔离性更强
- 更利于表达一类标准 contract 能力

但如果系统目标是一个更通用的 L1，那么只选一边都会有局限：

- 只做 Rust VM，缺少标准 EVM 合约能力
- 只做 EVM，又不一定适合高性能、深领域耦合的撮合型产品

所以当前这版架构的选择，不是二选一，而是：

**用一个 L1 执行框架，同时接入 Rust VM 与 EVM。**

这样分工就会很自然：

- Rust VM 负责更贴近业务域、性能敏感、规则复杂的产品能力
- EVM 负责 Solidity contract 风格的可编程能力

换句话说，这不是两个系统硬拼，而是在同一个 L1 下做 execution specialization。

---

## 四、整体架构：四层最容易看懂

要理解这个实现，我建议从四层看。

### 1. `l1_core`：执行框架内核

这一层定义了最核心的 L1 抽象：

- 交易如何准入
- 区块如何执行
- VM runtime 要满足什么接口
- 执行后应产出什么结果

关键点在于，它不关心底下具体是 Rust VM 还是 EVM。

它只关心：

- 你是不是一个可执行 runtime
- 你能不能返回统一格式的执行结果
- 你的状态变更能不能被统一提交

这相当于整条 L1 执行主链路的“协议内核”。

相关入口包括：

- `ReceiveAndAdmitTransactionsUseCase`  
  `lib/core/l1/src/use_case/command_handler/receive_and_admit_transactions.rs:68`
- `ExecuteAndCommitBlockUseCase`  
  `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs:78`
- `VmRuntime` / `VmRuntimeResolver` / `VmRegistry`  
  `lib/core/l1/src/vm/runtime.rs:13`

### 2. `l1_adapter`：基础设施与通用能力

这一层负责把 `l1_core` 的抽象接到具体实现上。

当前主要包括：

- `InMemoryMempool`  
  `lib/core/l1_adapter/src/mempool/in_memory.rs:7`
- `MdbxStateStore`  
  `lib/core/l1_adapter/src/outbound/mdbx_state_store.rs:10`
- `EvmRuntimeAdapter`  
  `lib/core/l1_adapter/src/evm/runtime.rs:9`

这个分层非常关键。

尤其是 `EvmRuntimeAdapter` 被放进 `l1_adapter`，说明当前架构已经明确了一件事：

**EVM 是通用执行能力，不属于某个具体业务域。**

它和 mempool、MDBX 一样，都属于 L1 的通用基础设施能力。

### 3. `dex`：Rust VM 的产品语义层

Rust VM 对应的逻辑放在 `dex` 中，而不是放回 `l1_core` 或 `l1_adapter`。

入口是：

- `RustVmRuntimeAdapter`  
  `operating/dex/src/adapter/rust_vm_runtime/mod.rs:22`

它会根据 capability 把请求映射到具体产品，例如：

- `dex.prep.place_order` / `dex.perp.place_order`
- `dex.spot.place_order`
- `dex.option.place_order`
- `dex.treasury.deposit`

这说明在当前设计里，Rust VM 不是一个“虚拟机技术名词”，而是一个真正承载业务产品逻辑的执行面。

### 4. `l1_e2e`：单节点装配层

最后一层是 `l1_e2e`。

很多人会下意识把它看成“测试目录”，但在当前实现里，它其实更像一个单节点的 composition root：

- 创建 mempool
- 打开 MDBX store
- 注册 Rust VM / EVM runtime
- 暴露 HTTP 接口
- 串起接收交易和执行区块的主路径

核心装配入口在：

- `lib/core/l1_e2e/src/bootstrap.rs:9`

这层的价值在于，它把前面几层真正拼成了一个可运行节点，而不是停留在抽象定义上。

---

## 五、节点是怎么启动的

当前单节点的装配过程其实很简单，但非常有代表性。

在 `build_app_state()` 里，系统会完成这样几件事：

1. 确定 MDBX 状态目录
2. 创建内存 mempool
3. 打开 `MdbxStateStore`
4. 创建 `VmRegistry<PendingRequest>`
5. 注册 Rust VM runtime
6. 注册 EVM runtime
7. 构造对外 service

其中最关键的两行是：

- `vm_registry.register_runtime(VmKind::RustVm, Arc::new(RustVmRuntimeAdapter::new()));`  
  `lib/core/l1_e2e/src/bootstrap.rs:22`
- `vm_registry.register_runtime(VmKind::Evm, Arc::new(EvmRuntimeAdapter::new()));`  
  `lib/core/l1_e2e/src/bootstrap.rs:23`

这两行其实就说明了这版设计的灵魂：

**L1 先定义统一执行框架，再把不同 runtime 注册进去。**

不是给 Rust VM 单独造一条链，也不是给 EVM 单独造一条链，而是在同一条 L1 执行链路里路由不同请求。

---

## 六、对外提供了哪些能力

当前 HTTP 层暴露的接口很克制，只有三个：

- `GET /api/l1/health`
- `POST /api/l1/transactions`
- `POST /api/l1/blocks/execute`

入口定义在：

- `lib/core/l1_e2e/src/http/mod.rs:8`

这三个接口恰好对应单节点最小模型的三个动作：

- 看节点活着没
- 把交易送进来
- 手动触发一次区块执行

很多人看到这里会问：

“为什么没有自动出块？”

答案很简单：因为这版就是**无共识单节点版**。

在没有引入 leader、proposal、vote、QC、finality 之前，让区块执行显式触发，反而是最容易观察、最容易调试、最容易验证边界的方式。

这也是为什么当前接口不追求多，而追求最小闭环。

---

## 七、一笔交易是怎么走完的

### 第一步：交易进入系统

当请求打到 `POST /api/l1/transactions` 时，service 层会先把外部请求转换成内部表示：

- `SignedTransactionRequest`
- `PendingRequest`

然后构造 `ReceiveAndAdmitTransactionsCmd`，通过 `ReceiveAndAdmitTransactionsUseCase` 做准入处理，并通过 `MempoolWritingPipeline` 写入 mempool。

入口在：

- `lib/core/l1_e2e/src/service.rs:46`

这一阶段还没有真正执行交易，它做的是：

- 接收请求
- 转内部模型
- 做准入
- 入池

也就是说，系统先回答“这笔请求能不能进来”，然后再回答“什么时候执行”。

### 第二步：区块触发执行

当请求打到 `POST /api/l1/blocks/execute` 时，系统会从 mempool 拉取一批 pending requests，然后调用 `ExecuteAndCommitBlockUseCase` 进入真正的执行阶段。

入口在：

- `lib/core/l1_e2e/src/service.rs:76`

执行阶段最核心的事情是：

1. 遍历每个 pending request
2. 调用 `vm_resolver.execute(...)`
3. 根据 `vm_kind` 分发到对应 runtime
4. 聚合 runtime 返回的状态变更
5. 通过 state pipeline 落到 MDBX

关键点在这里：

**L1 执行层本身不直接理解 `spot`、`perp`、`option` 或 Solidity 合约语义。**

它只负责：

- 调度
- 分发
- 汇总
- 提交状态

这就是统一执行框架的价值所在。

---

## 八、Rust VM 在这版设计里扮演什么角色

Rust VM 当前不是一个概念验证，而是已经接了真实产品能力。

它的入口在：

- `operating/dex/src/adapter/rust_vm_runtime/mod.rs:22`

从 capability 映射看，它当前已经覆盖：

- `perp`
- `spot`
- `option`
- `treasury`

这代表一种非常明确的设计倾向：

**把高性能、强领域耦合的产品能力放到 Rust VM。**

为什么这么做？

因为像订单撮合、衍生品约束、保证金逻辑、资金账户更新这类路径，通常有三个特点：

- 对性能敏感
- 对领域模型控制要求高
- 对状态一致性要求强

Rust VM 很适合做这类事情。

它可以直接承接产品语言，而不需要先把复杂业务语义翻译成合约语言再执行。

所以从产品视角看，Rust VM 更像是：

- L1 的原生产品执行引擎
- 高性能业务逻辑承载层
- 面向 `spot` / `perp` / `option` 的核心执行平面

---

## 九、EVM 在这版设计里扮演什么角色

EVM 入口在：

- `lib/core/l1_adapter/src/evm/runtime.rs:9`

当前支持的 capability 包括：

- `evm.settlement.deploy`
- `evm.settlement.create`
- `evm.settlement.release`
- `evm.settlement.get`
- `evm.settlement.release_without_create`

底层执行依赖：

- `RevmExecutor`
- `contracts` 模块

这一层的意义，不只是“能跑一个 EVM demo”，而是让整个 L1 具备一种标准化的合约执行面。

它比较适合：

- Solidity contract 能力验证
- 与合约生态兼容的模块
- 某些希望与 Rust 产品逻辑隔离的可编程能力

从架构视角看，EVM 不是来替代 Rust VM 的，而是来补齐另一类执行能力。

所以这套设计不是 Rust VM 和 EVM 谁更高级的问题，而是：

- Rust VM 擅长原生产品逻辑
- EVM 擅长标准合约执行
- L1 负责把两者放进同一个执行框架

---

## 十、状态是怎么落盘的

这版系统虽然是单节点，但并不是“跑完就丢”的 toy demo。

状态持久化是完整接上的。

当前 mempool 用的是：

- `InMemoryMempool`  
  `lib/core/l1_adapter/src/mempool/in_memory.rs:7`

当前状态存储用的是：

- `MdbxStateStore`  
  `lib/core/l1_adapter/src/outbound/mdbx_state_store.rs:10`

区块执行后，runtime 产出的状态变更会通过 `apply_block_state_changes(...)` 落到 MDBX：

- `lib/core/l1_adapter/src/outbound/mdbx_state_store.rs:125`

这里持久化的内容包括：

- account delta
- storage delta
- code delta

这件事的意义很大，因为它说明系统已经不只是“能执行一下”，而是已经具备了完整的：

- 请求接收
- 请求入池
- 请求执行
- 状态产出
- 状态持久化

换句话说，这已经是一条真正闭环的 L1 执行主路径。

---

## 十一、为什么说这版虽然没共识，但已经很重要

很多人会天然觉得：没有共识，就还不算一条真正的链。

这个说法在“完整性”上有道理，但在“架构演进顺序”上并不成立。

因为共识并不能替你解决执行问题。

即使你把 HotStuff 接进来，系统最终仍然要回答这些问题：

- block 里的请求怎么执行？
- Rust VM 和 EVM 怎么共存？
- 状态变更怎么统一提交？
- 持久化边界在哪里？
- 哪一层负责调度，哪一层负责产品语义？

而这些问题，恰恰是这版单节点实现先解决掉的。

所以这版的价值，不在于“它已经有了共识”，而在于：

**它已经把后续接共识时最容易混乱的执行骨架先理顺了。**

---

## 十二、如果后面要接 HotStuff，会怎么演进

未来如果要接 HotStuff，最重要的原则不是重写执行层，而是：

**把共识层包在执行层外面。**

也就是说，下面这些能力应该尽量复用：

- HTTP / ingress 交易接收
- mempool
- `ExecuteAndCommitBlockUseCase`
- `VmRegistry`
- Rust VM / EVM runtime
- MDBX 状态提交路径

真正新增的，是共识相关的编排：

- validator set
- proposal
- vote
- quorum certificate
- pacemaker
- commit / finalize 流程

当前模式是：

- 外部调用 `POST /api/l1/blocks/execute`
- 本地节点从 mempool 取交易
- 本地执行并提交状态

未来接入 HotStuff 后，模式会变成：

1. leader 组装 block proposal
2. 向 validator 广播 proposal
3. 节点验证 proposal
4. 收集 vote 形成 QC
5. 在 commit 阶段触发 block finalize
6. 最终沿用现有执行骨架完成状态提交

注意，这里变化的是：

- 谁来决定 block 可以提交
- block 提交前需要经过什么一致性流程

而不是：

- Rust VM 要不要重写
- EVM 要不要重写
- `VmRegistry` 要不要推翻
- MDBX 落盘路径要不要推翻

这就是为什么说，**共识层包住执行层，而不是替代执行层。**

---

## 十三、这版设计最值得保留的是什么

如果要总结这版设计最重要的价值，我觉得不是“它支持了两种 VM”，而是它把三件事分得很清楚：

### 1. 执行框架和业务逻辑分开了

- `l1_core` 负责执行骨架
- `dex` 负责 Rust VM 产品逻辑
- `l1_adapter` 负责通用基础设施和 EVM 支撑

### 2. 多 runtime 共存方式清楚了

不是为每种 VM 造一套独立链路，而是通过 `VmRegistry` 统一接入。

### 3. 共识扩展点留出来了

当前虽然没共识，但执行框架已经足够清晰，后续可以在外层自然包上 HotStuff。

也就是说，这版设计真正建立起来的是：

**一个可扩展的 L1 执行地基。**

在这个地基之上：

- 可以继续做更多 Rust VM 产品
- 可以继续扩展更多 EVM contract 能力
- 可以继续接入 HotStuff 之类的共识层

---

## 十四、结语

如果只看“有没有共识”，这版系统当然还不是最终形态。

但如果从架构演进顺序看，它反而处在一个非常关键的位置：

- 它先把 L1 最核心的执行主链路跑通了
- 它先把 Rust VM 和 EVM 的边界划清楚了
- 它先把状态持久化和 runtime 分发模型固定下来了
- 它为后续接 HotStuff 预留了清晰的扩展面

所以这版 `Rust VM / EVM 单节点无共识 L1` 的意义，并不只是“先做了个 demo”。

它更像是在回答一个更本质的问题：

> 当一条 L1 既想承载原生 Rust 产品能力，又想承载 Solidity contract 能力时，底层执行框架应该如何设计？

当前这份实现给出的答案是：

**先做统一执行框架，运行时分层解耦，状态路径闭环，再把共识作为外层能力接进来。**
