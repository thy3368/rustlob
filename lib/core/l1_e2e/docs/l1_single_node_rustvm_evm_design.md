# L1 设计介绍：Rust VM / EVM 单节点无共识版

## 1. 这份文档讲什么

这是一份写给其它开发者的介绍文档，用来快速理解当前仓库里的 L1 最小实现。

这套实现不是一个完整的多节点区块链系统，而是一个已经可运行的 **单节点、无共识、双 runtime** 版本。它的目标很明确：

- 先把 L1 的执行主链路跑通
- 先验证 Rust VM 与 EVM 可以挂到同一条执行框架上
- 先把状态持久化、交易入口、区块执行这些基础能力固定下来
- 暂时不引入 HotStuff 这类共识复杂度

如果你第一次接触这部分代码，可以把它理解成：

> 一个用 REST 收交易、用内存 mempool 暂存请求、用 MDBX 持久化状态、并且能同时执行 Rust VM 与 EVM 请求的单机版 L1。

对应装配入口在 `lib/core/l1_e2e/src/bootstrap.rs:9`。

## 2. 先看整体：当前版本解决了什么问题

当前版本主要解决的是一个非常实际的问题：

**如何让同一条 L1 执行链路同时承载两类执行环境。**

这两类执行环境分别是：

- Rust VM：承载 `spot`、`perp`、`option`、`treasury` 这类 Rust 原生产品逻辑
- EVM：承载 Solidity contract 相关能力，当前底层通过 `revm` 执行

它的关键特征有六个：

- 单节点执行
- 不做共识
- inbound 用 REST API
- mempool 用 `InMemoryMempool`
- 状态持久化用 `MdbxStateStore`
- 区块执行时通过 `VmRegistry` 在 Rust VM / EVM 之间分发

所以这套设计更像一个 **execution-first L1 skeleton**，重点是执行框架和 runtime 扩展能力，而不是网络共识。

## 3. 代码分层怎么理解

如果你要快速读代码，推荐按下面四层理解。

### 3.1 `l1_core`：执行框架内核

`l1_core` 负责定义 L1 的核心执行抽象，不直接依赖具体基础设施。

重点包括：

- `ReceiveAndAdmitTransactionsUseCase`  
  `lib/core/l1/src/use_case/command_handler/receive_and_admit_transactions.rs:68`
- `ExecuteAndCommitBlockUseCase`  
  `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs:78`
- `VmRuntime` / `VmRuntimeResolver` / `VmRegistry`  
  `lib/core/l1/src/vm/runtime.rs:13`

可以把这一层理解成：

- 定义交易如何准入
- 定义区块如何执行
- 定义 VM runtime 需要满足的统一接口
- 定义执行之后要交出哪些状态变更和回执

这一层不关心底下跑的是 Rust VM 还是 EVM，它只关心“怎么统一调度执行”。

### 3.2 `l1_adapter`：基础设施与通用 adapter

`l1_adapter` 负责把 `l1_core` 的抽象接到具体实现上。

目前主要有三块：

- `InMemoryMempool`  
  `lib/core/l1_adapter/src/mempool/in_memory.rs:7`
- `MdbxStateStore`  
  `lib/core/l1_adapter/src/outbound/mdbx_state_store.rs:10`
- `EvmRuntimeAdapter`  
  `lib/core/l1_adapter/src/evm/runtime.rs:9`

这里尤其值得注意的是：

- EVM runtime 已经放在 `l1_adapter`
- 这表示 EVM 在当前架构里被看作一种通用执行能力
- 它不属于 `dex` 业务域

### 3.3 `dex`：Rust VM 的业务域实现

`dex` 里放的是 Rust VM 对应的业务执行逻辑，当前暴露出来的入口是：

- `RustVmRuntimeAdapter`  
  `operating/dex/src/adapter/rust_vm_runtime/mod.rs:22`

它会把 capability 映射到具体产品：

- `dex.prep.place_order` / `dex.perp.place_order`
- `dex.spot.place_order`
- `dex.option.place_order`
- `dex.treasury.deposit`

见 `operating/dex/src/adapter/rust_vm_runtime/mod.rs:90`。

所以 Rust VM 在这里不是一个抽象概念，而是已经承接了真实产品域语义。

### 3.4 `l1_e2e`：单节点装配层

`l1_e2e` 是这套最小 L1 的装配层，负责把前面几层拼成一个可运行节点。

关键入口有：

- bootstrap：`lib/core/l1_e2e/src/bootstrap.rs:9`
- service：`lib/core/l1_e2e/src/service.rs:15`
- router：`lib/core/l1_e2e/src/http/mod.rs:8`

这里不要把 `l1_e2e` 理解成“测试代码目录”而已。当前它实际上承担的是 composition root 的角色：

- 创建 mempool
- 打开 MDBX store
- 注册 runtime
- 暴露 HTTP API
- 串起接收交易与执行区块的主流程

## 4. 节点是怎么装起来的

当前单节点在 `build_app_state()` 中完成装配：

- `lib/core/l1_e2e/src/bootstrap.rs:9`

核心步骤非常直接：

1. 读取或生成 MDBX 路径
2. 创建 `InMemoryMempool`
3. 打开 `MdbxStateStore`
4. 创建 `VmRegistry<PendingRequest>`
5. 注册 Rust VM runtime
6. 注册 EVM runtime
7. 构造 `L1E2eService`

关键代码是：

- `vm_registry.register_runtime(VmKind::RustVm, Arc::new(RustVmRuntimeAdapter::new()));`  
  `lib/core/l1_e2e/src/bootstrap.rs:22`
- `vm_registry.register_runtime(VmKind::Evm, Arc::new(EvmRuntimeAdapter::new()));`  
  `lib/core/l1_e2e/src/bootstrap.rs:23`

这基本就是当前架构的核心：

**统一的 L1 执行框架，底下注册多个 runtime。**

## 5. 对外接口有哪些

HTTP 接口定义在 `lib/core/l1_e2e/src/http/mod.rs:8`。

当前只暴露三个最小接口：

- `GET /api/l1/health`
- `POST /api/l1/transactions`
- `POST /api/l1/blocks/execute`

对应代码：

- `lib/core/l1_e2e/src/http/mod.rs:10`
- `lib/core/l1_e2e/src/http/mod.rs:11`
- `lib/core/l1_e2e/src/http/mod.rs:12`

这三个接口对应的含义是：

- `health`：看服务是否正常、mempool 里是否有积压
- `transactions`：把交易送进 L1
- `blocks/execute`：手动触发一次区块执行

这里最容易误解的一点是：

当前版本 **没有自动出块器，也没有共识驱动的出块 loop**。区块执行是显式触发的，这正是“无共识单节点版”的定义。

## 6. 一笔请求是怎么流转的

### 6.1 交易接收链路

入口在 `L1E2eService::submit_transactions(...)`：

- `lib/core/l1_e2e/src/service.rs:46`

整体流程是：

1. HTTP 层收到 `SubmitTransactionsRequest`
2. service 把请求转换成：
   - `SignedTransactionRequest`
   - `PendingRequest`
3. 构造 `ReceiveAndAdmitTransactionsCmd`
4. 通过 `IngressLoadPort` 做准入装载
5. 通过 `MempoolWritingPipeline` 写入 mempool
6. 执行 `ReceiveAndAdmitTransactionsUseCase`

关键代码：

- `ReceiveAndAdmitTransactionsCmd`  
  `lib/core/l1_e2e/src/service.rs:61`
- `MempoolWritingPipeline::<ReceiveAndAdmitTransactionsEvents>::new(...)`  
  `lib/core/l1_e2e/src/service.rs:65`
- `ReceiveAndAdmitTransactionsUseCase`  
  `lib/core/l1_e2e/src/service.rs:68`

这一段的重点不是“执行”，而是“准入 + 入池”。

### 6.2 区块执行链路

入口在 `L1E2eService::execute_block(...)`：

- `lib/core/l1_e2e/src/service.rs:76`

整体流程是：

1. `MempoolReadingLoadPort` 从 mempool 取出待执行请求  
   `lib/core/l1_e2e/src/service.rs:80`
2. `ExecuteAndCommitBlockStatePipeline` 准备把状态写入 MDBX  
   `lib/core/l1_e2e/src/service.rs:81`
3. 构造 `ExecuteAndCommitBlockCmd`  
   `lib/core/l1_e2e/src/service.rs:84`
4. 调用 `ExecuteAndCommitBlockUseCase`  
   `lib/core/l1_e2e/src/service.rs:89`

真正的执行核心在：

- `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs:78`

它内部最重要的步骤是：

1. 遍历 pending request
2. 调用 `vm_resolver.execute(...)`  
   `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs:104`
3. 按 `vm_kind` 分发到 Rust VM 或 EVM
4. 合并多个 runtime 返回的 `BlockStateChanges`  
   `lib/core/l1/src/use_case/command_handler/execute_and_commit_block.rs:123`
5. 把最终状态交给 pipeline 落到 MDBX

对开发者来说，这条链路最重要的认知是：

**L1 区块执行层不直接懂 spot / perp / option / Solidity，它只负责把请求交给正确的 runtime。**

## 7. Rust VM 和 EVM 分别负责什么

### 7.1 Rust VM：承载高性能业务产品逻辑

Rust VM 当前入口是：

- `operating/dex/src/adapter/rust_vm_runtime/mod.rs:22`

它会先校验 `vm_kind == RustVm`，然后根据 capability 路由到对应产品命令：

- `operating/dex/src/adapter/rust_vm_runtime/mod.rs:113`
- `operating/dex/src/adapter/rust_vm_runtime/mod.rs:90`

当前已经接入：

- `perp`
- `spot`
- `option`
- `treasury`

执行核心依赖 `ExecuteTradingBatchHandler`：

- `operating/dex/src/adapter/rust_vm_runtime/mod.rs:23`
- `operating/dex/src/adapter/rust_vm_runtime/mod.rs:127`

可以把 Rust VM 理解成：

- 更贴近撮合、订单、资金这类领域逻辑
- 用 Rust 原生实现高性能产品能力
- 适合放强约束、高性能、强状态一致性的产品模块

### 7.2 EVM：承载 Solidity contract 能力

EVM 当前入口是：

- `lib/core/l1_adapter/src/evm/runtime.rs:9`

它会先校验 `vm_kind == Evm`，再按 capability 分发：

- `lib/core/l1_adapter/src/evm/runtime.rs:241`
- `lib/core/l1_adapter/src/evm/runtime.rs:248`

当前支持的 capability 包括：

- `evm.settlement.deploy`
- `evm.settlement.create`
- `evm.settlement.release`
- `evm.settlement.get`
- `evm.settlement.release_without_create`

底层通过 `RevmExecutor` 和 `contracts` 模块执行：

- `lib/core/l1_adapter/src/evm/runtime.rs:7`

可以把 EVM 理解成：

- 提供标准 Solidity contract 风格能力
- 适合做合约兼容验证
- 适合承载一些希望与 Rust 业务逻辑隔离的合约模块

## 8. 状态是怎么存的

### 8.1 Mempool

当前 mempool 是 `InMemoryMempool`：

- `lib/core/l1_adapter/src/mempool/in_memory.rs:7`

它只有两个最核心语义：

- `add_requests(...)` 写入请求  
  `lib/core/l1_adapter/src/mempool/in_memory.rs:18`
- `fetch_requests(limit)` 取出并删除请求  
  `lib/core/l1_adapter/src/mempool/in_memory.rs:31`

这说明当前阶段还没有引入复杂排序、优先级或分布式传播逻辑。

### 8.2 MDBX

当前持久化状态存储是 `MdbxStateStore`：

- `lib/core/l1_adapter/src/outbound/mdbx_state_store.rs:10`

区块执行后的状态变更写入入口是：

- `apply_block_state_changes(...)`  
  `lib/core/l1_adapter/src/outbound/mdbx_state_store.rs:125`

它会把 runtime 产出的：

- account delta
- storage delta
- code delta

写到 MDBX 里。

这意味着当前实现不是只有 demo 级别的“执行一下就完”，而是已经有完整的：

- 请求入池
- 区块执行
- 状态变更生成
- 状态持久化落盘

## 9. 为什么现在没有共识

这个版本不带共识，不是遗漏，而是刻意的阶段化设计。

当前优先级是先稳定下面这些东西：

- 交易入口
- mempool
- 区块执行框架
- Rust VM / EVM 共存模型
- 状态落盘模型

如果这些还没稳定，就直接把 HotStuff 引进来，会把问题混在一起：

- 是执行错了？
- 是 runtime 路由错了？
- 是状态持久化错了？
- 还是共识流程错了？

所以当前版本故意只保留单机执行，把执行层先做扎实。

## 10. 这套设计对开发者意味着什么

如果你要在这套 L1 上继续开发，大致可以这样判断落点：

- 要改执行框架、use case、runtime 抽象：看 `l1_core`
- 要改 mempool、MDBX、EVM 支撑能力：看 `l1_adapter`
- 要加 Rust VM 产品能力：看 `dex`
- 要改单节点装配或 HTTP 接口：看 `l1_e2e`

如果你要新增一种产品能力，也可以先想清楚它属于哪类：

- 更像高性能内建产品逻辑，就偏 Rust VM
- 更像 Solidity contract / 合约兼容能力，就偏 EVM

## 11. 一句话总结

当前这套 L1 设计可以概括成一句话：

> 用一个统一的单节点 L1 执行框架，把 Rust VM 的产品逻辑和 EVM 的合约能力同时接进来，并先把交易接收、区块执行、状态持久化这条主链路跑通。

对其它开发者来说，最重要的认知不是“它还没有共识”，而是：

- 执行骨架已经清晰
- runtime 边界已经清晰
- Rust VM / EVM 的职责已经清晰
- 后续无论加产品还是加共识，都有明确扩展点
