---
name: hyperliquid-use-case-implement
description: "Use when implementing a production-grade Hyperliquid use case after design is settled. Keywords: implement use case, production code, CommandUseCase, LoadPort, DomainEventPipeline, ReplyMapper, tests, Hyperliquid, trading, 实现用例, 生产级代码, 验收测试"
user-invocable: true
---

# Hyperliquid Use Case Implement

> **Purpose**: 在设计已收敛后，把 Hyperliquid Use Case 实现成完整的生产级代码，并保持 Clean Architecture 边界。

## When to Use

当用户已经明确 use case 设计，并希望：
- 写完整生产代码
- 接到现有模块结构
- 补测试
- 复用共享抽象
- 做真实可执行的 use case 实现

## Preconditions

开始实现前必须已经明确：

- Use Case 名称
- `Command` / `Query`
- `GivenState`
- `LoadPort`
- `Error`
- `Events / Result / Reply`
- 领域不变量
- 不在 scope 的内容

不要把内部校验步骤实现成独立 use case。`Check*` / `Validate*` / `Admissibility*` 默认应落在 invariant、domain policy 或 `validate_against_state`，除非它是可独立触发、授权、审计并产生独立业务结果的业务意图。

如果这些还不清楚，先切回 `hyperliquid-use-case-define`。

## Implementation Rules

命令型实现优先复用：

- `lib/common/cmd_handler/src/use_case_def.rs`
  - `CommandUseCase`
  - `UseCaseReplyMapper`
  - `DomainEventPipeline`
  - `CommandUseCaseExecutor`

参考组织方式：

- `lib/common/cmd_handler/src/use_case_example.rs`

实现时默认包含：

1. 独立 use case 文件
2. `UseCase`
3. `Error`
4. `LoadPort`
5. `StateSnapshot`
6. `GivenState`
7. `Events`
8. `ReplyMapper`
9. `DomainEventPipeline`
10. `#[cfg(test)]` 单元测试

其中 `performer` / `performer()` 的语义必须保持一致：
- 表示参与者的业务角色
- 用于 authorization / permission control
- 成功后可进入状态 / 事件 / audit trace
- 禁止使用 engine / handler / gateway / pod / queue 等技术实现名

## Hard Boundaries

实现时仍然禁止把这些放进 use case 核心：

- HTTP path
- JSON shape
- reqwest
- signer
- raw exchange response DTO
- gateway concrete impl
- database schema

这些属于 Interface Adapters 或 Frameworks & Drivers。

## Verification Rules

至少验证：

- 输入预检查
- `load_state`
- 不变量校验
- 成功事件产出
- `ReplyMapper` 在核心外部
- `persist -> replay -> publish` 顺序
- `cargo check`
- `cargo test`

## Output Contract

默认输出：

1. 会修改哪些文件
2. 复用哪些现有抽象
3. 生产代码实现
4. 测试覆盖点
5. 验证命令
6. 哪些内容仍留在 adapter / infra 层
