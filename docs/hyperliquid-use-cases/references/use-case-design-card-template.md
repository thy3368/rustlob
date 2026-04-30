# Use Case Design Card Template

> Purpose: 在设计阶段固定 Use Case 的业务边界、状态、实体、不变量、端口和验收范围，避免过早陷入实现细节。

## 1. Use Case

名称：

类型：
- [ ] Command
- [ ] Query

业务意图：


## 2. Input

Command / Query：

关键字段：
-


## 3. GivenState

执行该 use case 前，需要加载哪些状态？

状态项：
-


## 4. Entities / Value Objects

涉及哪些领域对象？

Entities：
-

Value Objects：
-


## 5. Invariants

必须满足的不变量：

-


## 6. Output

成功输出什么？

可选：
- Events
- Result
- View
- Reply

输出内容：
-


## 7. Port

这个 use case 依赖哪些能力？

Port 名称：

Port 需要提供：
-


## 8. Business Errors

业务错误：

-


## 9. Not in Scope

明确不属于本 use case 的内容：

- HTTP path
- JSON shape
- serde
- signer
- reqwest
- gateway implementation
- database schema
- external API details


## 10. Acceptance

只验证 use case 本身：

- [ ] 输入能被接受或拒绝
- [ ] GivenState 能被加载
- [ ] 不变量校验生效
- [ ] 成功时输出正确 Events / Result / View
- [ ] ReplyMapper 在核心外部
- [ ] 不依赖 HTTP / JSON / signer
