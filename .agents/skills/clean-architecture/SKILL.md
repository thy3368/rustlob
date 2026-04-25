---
name: clean-architecture
author: Tokenaissance (https://tokenaissance.com)
description: >
  整洁架构实战指南。当用户询问架构设计、代码分层、重构、依赖管理、技术选型、过度设计时使用。
  基于 Robert C. Martin 整洁架构、SOLID 原则、马斯克五步工作法。
  务必在以下场景使用：分层架构、依赖反转、接口设计、组件划分、重构建议、技术选型评估、
  识别过度设计、代码解耦、边界划分、模块组织。
---

# Clean Architecture Response Rules

When answering architecture questions, use Clean Architecture terminology first, then map it to implementation patterns.

## Core Terminology

Use these four Clean Architecture layer terms:

1. **Entities** (实体) - Core business rules
2. **Use Cases** (用例) - Application-specific business logic
3. **Interface Adapters** (接口适配器) - Glue/conversion boundary between use cases and external tools
4. **Frameworks & Drivers** (框架与驱动) - Imported external tools, not project directories

Interface Adapters 是 Use Cases 与 Frameworks & Drivers 的胶水代码。
Frameworks & Drivers 通过 import 方式从第三方包、SDK、runtime、driver 中引用，不应该出现在代码目录中作为 infrastructure 层。
Interface Adapters 应该分为 inbound_adapter（如 HTTP/CLI/Event → Use Case）和 outbound_adapter（如 Use Case Port → DB/HTTP/API/SDK）。

Do not use these implementation patterns as primary layer names:

- ❌ "Controller Layer" / "控制器层"
- ❌ "Service Layer" / "服务层" / "业务逻辑层"
- ❌ "Repository Layer" / "仓储层" / "数据访问层"
- ❌ "Infrastructure Layer" / "基础设施层"
- ❌ "Model Layer" / "模型层"
- ❌ "Business Layer" / "业务层"
- ❌ "Data Layer" / "数据层"

These are implementation patterns, not Clean Architecture layers. Teach the architectural concepts first, then map them to the user's stack.

---

## Read References First

Before answering architecture questions, read only the relevant sections of:

- `references/clean-architecture.md`

Focus on:

- Section 6: 整洁架构模型 (四层同心圆)
- Section 6: 依赖关系规则 (The Dependency Rule)
- Section 3: SOLID 设计原则, especially DIP

For over-engineering or technology-selection questions, also read:

- `references/musk-algorithm.md`

---

## Response Pattern for Layering Questions

Include these sections when the user asks how to layer code.

### 1. Introduce Clean Architecture Layers

```text
# 整洁架构的四层模型

Robert C. Martin 的整洁架构定义了四个同心圆层次：

┌─────────────────────────────────────────┐
│  Frameworks & Drivers (框架与驱动)        │  ← 最外层概念
│  - imported packages: express, pg        │
│  - SDKs, runtimes, database drivers      │
│  - 不作为 src/ 下的 architecture 目录      │
└──────────────┬──────────────────────────┘
               │ 通过 adapter import 使用
┌──────────────▼──────────────────────────┐
│  Interface Adapters (接口适配器)         │
│  - inbound_adapter: HTTP/CLI/Event → UC │
│  - outbound_adapter: UC Port → DB/API   │
│  - adapter code lives in project        │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  Use Cases (用例)                        │
│  - 应用特定的业务逻辑                     │
│  - 编排实体间的数据流                     │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│  Entities (实体)                         │  ← 最内层
│  - 核心业务规则                          │
│  - 关键业务数据                          │
└─────────────────────────────────────────┘
```

### 2. Explain The Dependency Rule

```text
## 依赖关系规则 (The Dependency Rule)

核心原则：源码中的依赖关系必须只指向同心圆的内层，即由低层机制指向高层策略。

依赖方向：
Imported Frameworks & Drivers → Adapter → Use Case → Entity
(外部机制)                                      (内层策略)

✅ 正确的依赖：
- Inbound Adapter 依赖 Use Case
- Use Case 依赖 Entity
- Use Case 定义 Gateway/Port 接口（抽象）
- Outbound Adapter 实现 Use Case 定义的接口（依赖反转）
- Outbound Adapter import 第三方 Frameworks & Drivers

❌ 错误的依赖：
- Entity 依赖数据库、SDK、HTTP framework
- Use Case 依赖 HTTP framework
- Entity 依赖 Use Case
- Use Case 直接依赖具体数据库实现、SDK、ORM
```

### 3. Map to the User's Stack

Use the user's stack, not Express.js by default. If the stack is Express.js, this mapping is valid:

| Clean Architecture 层 | Express.js 实现 |
|---------------------|----------------|
| Entities | `domain/entities/` - 纯业务对象 |
| Use Cases | `application/use-cases/` - application-specific workflows |
| Interface Adapters | `interface_adapters/inbound_adapter/` + `interface_adapters/outbound_adapter/` |
| Frameworks & Drivers | imported dependencies - `express`, `sequelize`, `pg`, `axios`, SDKs |

`infrastructure/` 不作为 Clean Architecture 层目录出现；外部框架和驱动由 adapter import 使用。

### 4. Show a Short Before/After Sketch

```javascript
// ❌ Before: inbound code directly depends on database model / framework detail
// controllers/userController.js
const UserModel = require('../models/UserModel');

exports.createUser = async (req, res) => {
  const user = await UserModel.create(req.body);
  res.json(user);
};
```

```javascript
// ✅ After: Use Case owns the port; adapters translate and import external tools

// ========== Entity 层 ==========
// domain/entities/User.js
class User {}

// ========== Use Case 层 ==========
// application/use-cases/CreateUser.js
class CreateUserUseCase {
  constructor(userRepository) {
    this.userRepository = userRepository;
  }

  async execute(input) {
    return this.userRepository.create(input);
  }
}

// ========== Use Case 层：Gateway/Port 接口 ==========
// application/interfaces/IUserRepository.js
class IUserRepository {
  async create(userData) {
    throw new Error('Must implement');
  }
}

// ========== Interface Adapter 层：outbound_adapter ==========
// interface_adapters/outbound_adapter/persistence/SequelizeUserRepository.js
const UserModel = require('../models/UserModel'); // imported Frameworks & Drivers

class SequelizeUserRepository extends IUserRepository {
  async create(userData) {
    return UserModel.create(userData);
  }
}

// ========== Interface Adapter 层：inbound_adapter ==========
// interface_adapters/inbound_adapter/http/UserController.js
class UserController {
  constructor(createUserUseCase) {
    this.createUserUseCase = createUserUseCase;
  }

  async createUser(req, res) {
    const user = await this.createUserUseCase.execute(req.body);
    res.status(201).json(user);
  }
}
```

Dependency direction:

- Inbound Adapter → Use Case ✅
- Use Case → Entity ✅
- Use Case → Port Interface ✅
- Outbound Adapter → Port Interface ✅
- Outbound Adapter imports Frameworks & Drivers ✅
- Use Case ❌ does not depend on concrete outbound adapters, SDKs, ORMs, or DB drivers

---

## Directory Structure

Provide a structure that reflects Clean Architecture layers:

```text
src/
├── domain/                              # Entities 层（最内层）
│   └── entities/
│       └── User.js
│
├── application/                         # Use Cases 层
│   ├── use-cases/
│   │   └── CreateUser.js
│   └── interfaces/                      # Gateway/Port 抽象
│       └── IUserRepository.js
│
└── interface_adapters/                  # Interface Adapters 层
    ├── inbound_adapter/                 # 外部输入 → Use Case
    │   └── http/
    │       └── UserController.js
    └── outbound_adapter/                # Use Case Port → 外部工具
        ├── persistence/
        │   └── SequelizeUserRepository.js
        └── http/
            └── ExternalApiClient.js
```

`express`、数据库 driver、ORM、HTTP SDK 等属于 Frameworks & Drivers：通过 package manager 安装，并由 `interface_adapters/*` 中的代码 import，不在 `src/` 中作为架构层目录建模。

---

## Scenario: Dependency Direction Problems

Use this diagnostic pattern when Entity or Use Case depends on an SDK, ORM, HTTP framework, database driver, or external API client.

1. Identify the violation
   - Current dependency: `Entity/Use Case → Framework/SDK/ORM/Driver`
   - Problem: inner policy depends on outer mechanism

2. Explain why it is a problem
   - Entity and Use Case should remain stable business policy
   - External vendors, SDKs, databases, and frameworks change independently
   - Direct dependencies make tests slower and replacement harder

3. Fix the dependency direction
   - Define a Gateway/Port interface at the Use Case boundary
   - Make the Use Case depend on that interface
   - Move concrete external calls to `interface_adapters/outbound_adapter`
   - Let the outbound adapter implement the port and import the SDK/driver

Corrected direction:

```text
Imported Frameworks & Drivers → outbound_adapter → Use Case Port → Use Case → Entity
```

Example rule:

- Stripe SDK, DB driver, ORM, HTTP client → `interface_adapters/outbound_adapter`
- HTTP/CLI/Event input parsing → `interface_adapters/inbound_adapter`
- Business workflow and port definitions → `application/use-cases` / `application/interfaces`
- Business invariants → `domain/entities`

---

## Scenario: Over-Engineering & Technology Selection

When the user asks whether to add microservices, Kafka, Redis, Kubernetes, service mesh, or similar complexity, apply Musk's Algorithm:

1. **Question the requirement**
   - What concrete problem does this solve?
   - What measured bottleneck or team constraint exists?

2. **Delete unnecessary complexity**
   - Prefer no new component unless the requirement is proven.

3. **Simplify**
   - Recommend the simplest architecture that satisfies current constraints.

4. **Accelerate**
   - Use mature tools and boring technology where possible.

5. **Automate**
   - Automate only after the process is understood and stable.

Default bias: do not recommend complex infrastructure for small teams, simple CRUD, or unmeasured performance concerns.

---

## Critical Rules Summary

✅ You must:

1. Use Clean Architecture terminology: `Entities`, `Use Cases`, `Interface Adapters`, `Frameworks & Drivers`
2. Explain The Dependency Rule explicitly
3. Show dependency direction with arrows
4. Split Interface Adapters into `inbound_adapter` and `outbound_adapter`
5. Treat Frameworks & Drivers as imported dependencies, not project directories
6. Question requirements before recommending complex technologies

❌ You must not:

1. Use generic terms like Service/Repository/Controller as primary layer names
2. Recommend `src/infrastructure/` as a Clean Architecture layer directory
3. Let Entity or Use Case depend on external frameworks, SDKs, ORMs, or drivers
4. Skip the port/interface boundary for outbound dependencies
5. Recommend complex infrastructure without validating the requirement

---

## Reference Files

- `references/clean-architecture.md` - Complete Clean Architecture principles
- `references/musk-algorithm.md` - Musk's 5-step algorithm
- `references/engineering-philosophy.md` - Dialectical engineering philosophy
