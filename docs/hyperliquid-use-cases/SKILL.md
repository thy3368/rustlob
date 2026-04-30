---
name: hyperliquid-use-cases
description: "Use as the routing entry for Hyperliquid use-case work. Keywords: hyperliquid, use case, trading, define use case, implement use case, production code, 用例, 定义用例, 实现用例, 生产级代码"
user-invocable: true
---

# Hyperliquid Use Cases

> **Purpose**: 作为 Hyperliquid 用例工作的统一入口，根据用户意图路由到“定义 use case”或“实现 production-grade use case”。

## Routing Rule

- 如果用户在问：
  - 怎么定义 use case
  - 怎么设计 use case
  - 核心要素是什么
  - 想先梳理 GivenState / Port / Error / Events
  - 不想陷入代码实现
  -> route to `hyperliquid-use-case-define`

- 如果用户在说：
  - 直接实现
  - 写生产级代码
  - 补测试
  - 接到现有模块
  - 基于已收敛设计落地
  -> route to `hyperliquid-use-case-implement`

## Keep the Boundary

这个 router skill 本身不承担完整设计卡产出，也不承担完整实现细节。
它只负责：

1. 识别当前请求属于 define 还是 implement
2. 引导到合适的 skill
3. 保持 Clean Architecture 边界不被混淆

## Sub-skill Responsibilities

### `hyperliquid-use-case-define`
负责：
- Use Case Design Card
- Minimal Rust Skeleton
- Acceptance Scope
- 明确业务边界与 Not in Scope

### `hyperliquid-use-case-implement`
负责：
- production-grade code
- `CommandUseCase` / `LoadPort` / `Events` / `ReplyMapper` / `DomainEventPipeline`
- unit tests
- `cargo check` / `cargo test` verification

## Clean Architecture Rule

无论路由到哪个子 skill，都必须遵守：
- Use Case 只表达业务动作
- 不引入 HTTP / JSON / signer / reqwest / gateway implementation
- Error 必须是业务错误
- 金融数值保持 `Decimal` 或明确 Value Object

## References

- `../hyperliquid-use-case-define/SKILL.md`
- `../hyperliquid-use-case-implement/SKILL.md`
