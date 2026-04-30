# Use Case Design Card Template

> Purpose: 在设计阶段固定 Use Case 的业务边界、状态、实体、不变量、端口和验收范围，避免过早陷入实现细节。

```yaml
---
use_case: XxxUseCase
kind: command
status: draft
performer: XxxPerformer
outputs:
  - XxxEvents
ports:
  - XxxLoadPort
---
```

> 建议：设计卡使用 `.md`，但顶部保留 YAML frontmatter。Markdown 正文适合人类评审，frontmatter 适合后续自动化检索、路由与生成。

## 1. Use Case

名称：

类型：
- [ ] Command
- [ ] Query

业务意图：


## 2. Performer

参与者的业务角色，用于权限控制，并最终落到状态 / 事件 / 审计追溯中。
禁止写成 HTTP handler、DB、queue、pod、engine、gateway 或具体进程。

Performer：
-


## 3. Input

Command / Query：

关键字段：
-


## 4. GivenState

执行该 use case 前，需要加载哪些状态？

状态项：
-


## 5. Entities / Value Objects

涉及哪些领域对象？

Entities：
-

Value Objects：
-


## 6. Invariants

必须满足的不变量：

-


## 7. Output

成功输出什么？

可选：
- Events
- Result
- View
- Reply

输出内容：
-


## 8. Port

这个 use case 依赖哪些能力？

Port 名称：

Port 需要提供：
-


## 9. Business Errors

业务错误：

-


## 10. Not in Scope

明确不属于本 use case 的内容：

- HTTP path
- JSON shape
- serde
- signer
- reqwest
- gateway implementation
- database schema
- external API details


## 11. Acceptance

只验证 use case 本身：

- [ ] 输入能被接受或拒绝
- [ ] GivenState 能被加载
- [ ] 不变量校验生效
- [ ] 成功时输出正确 Events / Result / View
- [ ] ReplyMapper 在核心外部
- [ ] 不依赖 HTTP / JSON / signer
