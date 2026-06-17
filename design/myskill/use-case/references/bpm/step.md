我将采用逐步披露的方式，从简单到完整展示如何使用 YAML 定义 Clean Architecture 中的 Use Case，并与 BDD 集成。

Clean Architecture + BDD 的 YAML Use Case 定义

🎯 第 1 步：最简核心定义

只包含 Use Case 的最基本信息
# usecases/open_account/contract.yaml
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户


🎯 第 2 步：添加架构分层信息

明确 Clean Architecture 的分层归属
# usecases/open_account/contract.yaml
metadata:
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户

# Clean Architecture 分层
clean_architecture:
layer: application       # 应用层
domain: banking         # 所属领域
bounded_context: account_management


🎯 第 3 步：定义输入输出

添加 Use Case 的输入输出契约
# usecases/open_account/contract.yaml
metadata:
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户
clean_layer: application
domain: banking

# 输入 DTO
input:
customer_id: string
account_type: "savings" | "checking" | "business"
initial_deposit: number

# 输出 DTO
output:
account_id: string
account_number: string
status: "active" | "pending" | "rejected"


🎯 第 4 步：添加业务规则

定义领域层的业务规则
# usecases/open_account/contract.yaml
metadata:
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户
clean_layer: application
domain: banking

input:
customer_id: string
account_type: "savings" | "checking" | "business"
initial_deposit: number

output:
account_id: string
account_number: string
status: "active" | "pending" | "rejected"

# 业务规则（属于领域层）
business_rules:
- 客户必须年满18岁
- 初始存款必须 ≥ 账户类型最低要求
- 客户必须通过身份验证


🎯 第 5 步：定义端口依赖

明确 Use Case 依赖的端口（Ports）
# usecases/open_account/contract.yaml
metadata:
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户
clean_layer: application
domain: banking

input:
customer_id: string
account_type: "savings" | "checking" | "business"
initial_deposit: number

output:
account_id: string
account_number: string
status: "active" | "pending" | "rejected"

business_rules:
- 客户必须年满18岁
- 初始存款必须 ≥ 账户类型最低要求
- 客户必须通过身份验证

# 端口依赖（遵循依赖倒置原则）
ports:
repositories:
- CustomerRepository  # 依赖抽象，不依赖具体实现
- AccountRepository

services:
- ComplianceService
- NotificationService


🎯 第 6 步：添加执行步骤

定义 Use Case 的执行流程
# usecases/open_account/contract.yaml
metadata:
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户
clean_layer: application
domain: banking

input:
customer_id: string
account_type: "savings" | "checking" | "business"
initial_deposit: number

output:
account_id: string
account_number: string
status: "active" | "pending" | "rejected"

business_rules:
- 客户必须年满18岁
- 初始存款必须 ≥ 账户类型最低要求
- 客户必须通过身份验证

ports:
repositories:
- CustomerRepository
- AccountRepository
services:
- ComplianceService
- NotificationService

# 执行步骤（应用层协调逻辑）
steps:
- 验证输入参数
- 加载客户聚合根
- 验证业务规则
- 创建账户聚合根
- 保存账户
- 发送通知
- 返回结果


🎯 第 7 步：添加 BDD 验收场景

定义 BDD 验收测试场景
# usecases/open_account/bdd/scenarios.yaml
feature: 开设银行账户
description: 客户成功开设新账户

scenarios:
- name: 成功开设储蓄账户
  given:
    - 我是一个已认证的客户
    - 我年满18岁
      when:
    - 我申请开设储蓄账户
    - 存入1000元
      then:
    - 账户创建成功
    - 账户状态为"active"
    - 余额为1000元
    - 收到欢迎邮件

- name: 年龄不足开户失败
  given:
    - 我是一个已认证的客户
    - 我16岁
      when:
    - 我尝试开设账户
      then:
    - 开户被拒绝
    - 提示"年龄必须满18岁"


🎯 第 8 步：添加 BDD 步骤映射

定义 BDD 步骤到实际代码的映射
# usecases/open_account/bdd/steps.yaml
step_definitions:
given:
- pattern: "我是一个已认证的客户"
implementation: |
const customer = Customer.create({
id: "cust-001",
verified: true
})
testContext.customer = customer

    - pattern: "我{int}岁"
      implementation: |
        testContext.customer.age = {int}

when:
- pattern: "我申请开设{word}账户"
implementation: |
const useCase = new OpenAccountUseCase(
customerRepo,
accountRepo
)
testContext.result = await useCase.execute({
customer_id: testContext.customer.id,
account_type: "{word}",
initial_deposit: 1000
})

then:
- pattern: "账户创建成功"
implementation: |
expect(testContext.result.account_id).toBeDefined()

    - pattern: "账户状态为{string}"
      implementation: |
        expect(testContext.result.status).toBe("{string}")


🎯 第 9 步：完整的 Use Case 定义

整合所有信息，生成完整定义
# usecases/open_account/contract.yaml
# =================================
# 1. 元数据
metadata:
id: open_account_v1
name: OpenAccount
description: 为客户开设银行账户
version: 1.0.0
clean_architecture:
layer: application
domain: banking
aggregate_root: BankAccount

# 2. 输入输出
input:
schema:
customer_id: string
account_type: "savings" | "checking" | "business"
initial_deposit: number
currency: string
validation:
- initial_deposit > 0
- customer_id 格式正确

output:
schema:
account_id: string
account_number: string
status: "active" | "pending" | "rejected"
opened_at: string

# 3. 业务规则
business_rules:
- id: BR-001
  name: 年龄要求
  condition: "customer.age >= 18"
  error_code: "CUSTOMER_UNDERAGE"
  layer: domain

- id: BR-002
  name: 最低存款
  condition: "initial_deposit >= 100"
  error_code: "INSUFFICIENT_DEPOSIT"
  layer: domain

# 4. 依赖端口
ports:
# 输入端口
input_port: OpenAccountCommand

# 输出端口
output_port: OpenAccountResult

# 所需端口
required_ports:
repositories:
- interface: ICustomerRepository
methods: ["findById"]

      - interface: IAccountRepository
        methods: ["save", "generateAccountNumber"]
        
    services:
      - interface: IComplianceService
        methods: ["checkAML"]
        
    events:
      - interface: IEventPublisher
        events: ["AccountOpened"]

# 5. 执行流程
execution_flow:
steps:
- step: validate_input
layer: application

    - step: load_customer
      layer: application
      dependency: ICustomerRepository
      
    - step: check_business_rules
      layer: domain
      
    - step: create_account
      layer: domain
      
    - step: save_account
      layer: application
      dependency: IAccountRepository
      
    - step: publish_event
      layer: application
      event: AccountOpened

events:
published:
- name: AccountOpened
aggregate: BankAccount
version: 1

# 6. 错误处理
errors:
- code: "CUSTOMER_NOT_FOUND"
  type: validation_error
  message: "客户不存在"
  layer: application

- code: "CUSTOMER_UNDERAGE"
  type: business_error
  message: "客户年龄必须满18岁"
  layer: domain

# 7. BDD 集成
bdd:
feature_file: "./bdd/scenarios.yaml"
step_definitions: "./bdd/steps.yaml"
tags: ["@open_account", "@smoke"]

# 8. 测试配置
testing:
unit_tests:
- name: 成功开户
input:
customer_id: "cust-001"
account_type: "savings"
initial_deposit: 1000
expected_output:
status: "active"

integration_tests:
- name: 与客户服务集成
mocks:
ICustomerRepository:
findById: { id: "cust-001", age: 25 }

bdd_tests:
scenarios: ["成功开设储蓄账户", "年龄不足开户失败"]


🎯 第 10 步：生成的代码

从 YAML 生成的 TypeScript 代码
// 生成自: contract.yaml
// 位置: application/use-cases/open-account

// 输入 DTO
interface OpenAccountInput {
customer_id: string;
account_type: 'savings' | 'checking' | 'business';
initial_deposit: number;
currency: string;
}

// 输出 DTO
interface OpenAccountOutput {
account_id: string;
account_number: string;
status: 'active' | 'pending' | 'rejected';
opened_at: string;
}

// Use Case 实现
export class OpenAccountUseCase {
constructor(
private readonly customerRepo: ICustomerRepository,
private readonly accountRepo: IAccountRepository,
private readonly complianceService: IComplianceService,
private readonly eventPublisher: IEventPublisher
) {}

async execute(input: OpenAccountInput): Promise<OpenAccountOutput> {
// 步骤1: 验证输入
this.validateInput(input);

    // 步骤2: 加载客户
    const customer = await this.customerRepo.findById(input.customer_id);
    if (!customer) {
      throw new CustomerNotFoundError();
    }
    
    // 步骤3: 检查业务规则
    this.checkBusinessRules(customer, input);
    
    // 步骤4: 创建账户
    const account = BankAccount.create({
      customerId: customer.id,
      accountType: input.account_type,
      initialDeposit: input.initial_deposit,
      currency: input.currency
    });
    
    // 步骤5: 保存账户
    await this.accountRepo.save(account);
    
    // 步骤6: 发布事件
    await this.eventPublisher.publish(
      new AccountOpenedEvent(account.id, customer.id)
    );
    
    // 返回结果
    return {
      account_id: account.id,
      account_number: account.accountNumber,
      status: account.status,
      opened_at: account.openedAt.toISOString()
    };
}

private validateInput(input: OpenAccountInput): void {
if (input.initial_deposit <= 0) {
throw new ValidationError('存款金额必须大于0');
}
}

private checkBusinessRules(customer: Customer, input: OpenAccountInput): void {
// 规则 BR-001: 年龄要求
if (customer.age < 18) {
throw new BusinessRuleError('CUSTOMER_UNDERAGE', '客户年龄必须满18岁');
}

    // 规则 BR-002: 最低存款
    if (input.initial_deposit < 100) {
      throw new BusinessRuleError('INSUFFICIENT_DEPOSIT', '存款金额不足');
    }
}
}


🔄 工作流程总结


业务需求
↓
编写 YAML Use Case 定义
├─→ 生成 Clean Architecture 代码
├─→ 生成 BDD 验收场景
└─→ 生成测试代码
↓
实现 BDD 步骤
↓
运行 BDD 测试
↓
重构优化


📁 完整项目结构


src/
├── domains/                          # 领域层
│   └── banking/
│       ├── entities/
│       ├── value-objects/
│       └── business-rules/
├── usecases/                         # 应用层
│   └── open_account/
│       ├── contract.yaml            # 1. Use Case 定义
│       ├── bdd/
│       │   ├── scenarios.yaml       # 2. BDD 场景
│       │   └── steps.yaml           # 3. BDD 步骤
│       └── implementation/
│           ├── OpenAccountUseCase.ts
│           ├── ports/
│           └── dtos/
└── tests/
└── bdd/
├── features/
└── step-definitions/


🎯 核心优势

1. 单一事实来源 - YAML 是权威定义
2. 架构可追溯 - 从需求到代码的完整链路
3. 自动生成 - 减少重复代码编写
4. 可执行文档 - BDD 场景即验收测试
5. 架构约束 - 强制 Clean Architecture 分层

通过这 10 个步骤，您可以循序渐进地定义复杂的 Use Case，同时保持代码的整洁架构和可测试性。