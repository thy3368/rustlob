# Architecture Output Template

固定用这个骨架回答 clean architecture / layering / dependency questions。

## 1. Core

### use_case
- 这里放业务动作
- 输入优先表达为 `command` / `query`
- 这里定义 use case 需要的 port/interface
- `use_case -> entity`

### entity
- 这里放核心业务规则
- 这里放关键业务数据与 invariants
- `entity` 不知道 `command/query`

## 2. Adapter

### inbound
- 这里放 HTTP / CLI / Event / GUI 到 use case 的转换
- 这里只负责翻译输入，不承载核心业务规则

### outbound
- 这里放 use case port 的实现
- 这里只实现 port，不定义 port
- 这里对接 DB / HTTP API / SDK / presenter / queue

## 3. Infra

- 这里放 external frameworks, runtimes, SDKs, database drivers, third-party tools
- `infra` 不应污染 `core`

## 4. Architecture views

先写 role view：

```text
core
  policies, use_case, entity

adapter
  glue / translation layer between core and infra

infra
  frameworks, SDKs, drivers, runtimes, third-party tools
```

再写 source dependency view：

```text
inbound -> use_case -> entity
outbound -> port <- use_case
outbound -> infra
```

最后写 call flow view：

```text
inbound -> use_case -> outbound -> infra
```

## 5. Violations

逐项指出：
- 哪个 `use_case` 直接依赖了外部工具
- 哪个 `entity` 被 transport / ORM / SDK 污染
- 哪个命名只是实现模式，却被误当成架构层

## 6. Minimal restructuring advice

只给满足当前目标的最小调整建议：
- 哪些代码进入 `core.use_case`
- 哪些代码进入 `core.entity`
- 哪些代码进入 `adapter.inbound`
- 哪些代码进入 `adapter.outbound`
- 哪些保留在 `infra`

## 7. Complexity check

如果用户提议复杂机制，再补：
1. `Question`
2. `Delete`
3. `Simplify`
4. optional `Accelerate`
5. optional `Automate`
