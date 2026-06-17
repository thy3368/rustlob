行业级 Use Case 分级：华为五级建模与银行业五级建模

您说得非常对。在大型企业架构中，Use Case 的分级是连接战略、流程与系统的核心骨架。华为的“五级流程架构”和银行业的“五级流程建模”是这一领域的经典实践。

🏢 华为五级流程架构（L1-L5）

华为的流程体系是其管理体系的基石，它将业务逐级分解，确保战略落地。

级别 名称 核心关注点 对应 Use Case 层级 XPDL 映射 Clean 架构对应

L1 流程分类 公司战略与业务领域 价值流 / 业务能力 Package (业务域) 企业上下文映射

L2 流程组 端到端业务流程 史诗级 Use Case Package (流程包) 限界上下文

L3 流程 部门级核心流程 复杂 Use Case WorkflowProcess (主流程) 应用服务编排

L4 子流程 具体活动与决策 中等 Use Case SubProcess / 复杂 Activity 应用层 Use Case

L5 活动 操作步骤与系统任务 简单/微型 Use Case Task / Activity 领域服务/适配器调用

具体示例：从战略到操作


L1 客户关系管理 (CRM)
↓
L2 线索到现金 (Lead to Cash)
↓
L3 销售机会管理流程
↓
L4【子流程】商机评估
↓
L5【活动】创建商机 (CreateOpportunityUseCase)


在 XPDL 中的体现：
<Package Id="L2_LeadToCash" Name="L2-线索到现金流程包">
<WorkflowProcesses>
<WorkflowProcess Id="L3_SalesOppManagement" Name="L3-销售机会管理">
<Activities>
<Activity Id="L4_OpportunityAssessment" Name="L4-商机评估">
<Implementation>
<SubFlow Id="AssessOpportunitySubProcess"/>
</Implementation>
</Activity>
<Activity Id="L5_CreateOpportunity" Name="L5-创建商机">
<Implementation>
<Task>
<TaskApplication>
<Application Id="CreateOpportunityUseCase"/>
</TaskApplication>
</Task>
</Implementation>
</Activity>
</Activities>
</WorkflowProcess>
</WorkflowProcesses>
</Package>


🏦 银行业五级流程建模

银行业因强监管、高复杂性，其流程分级尤为严谨，常与TOGAF、BIAN等架构框架结合。

级别 名称 描述 银行业示例 对应 Use Case 类型

L1 业务领域 战略业务单元 零售银行、公司金融、金融市场 业务能力模型

L2 业务线流程 端到端客户旅程 个人贷款旅程、对公开户旅程 客户旅程地图

L3 核心业务流程 可独立管理、度量的流程 贷款审批流程、反洗钱检查流程 复杂/史诗 Use Case

L4 业务活动 部门内可完成的具体活动 “验证客户身份”、“评估信用风险” 中等 Use Case

L5 操作步骤 人工或系统执行的最小单元 “调用人行征信接口”、“生成贷款合同” 简单/微型 Use Case

银行业示例：个人信用贷款


L1 零售银行业务
↓
L2 个人信贷业务线
↓
L3 信用贷款申请审批流程
↓
L4【活动】信用评估
↓
L5【步骤】查询央行征信 (QueryPBOCCreditReportUseCase)


在 Clean 架构中的实现：
// L5: 操作步骤 - 简单查询 Use Case
public class QueryPBOCCreditReportUseCase implements UseCase<String, CreditReport> {
private final CreditBureauAdapter bureauAdapter; // 外部适配器

    public CreditReport execute(String idCard) {
        return bureauAdapter.queryCreditReport(idCard);
    }
}

// L4: 业务活动 - 中等协调 Use Case  
public class AssessCreditRiskUseCase implements UseCase<Application, RiskRating> {
private final QueryPBOCCreditReportUseCase queryReportUC;
private final InternalScoringService scoringService;
private final RuleEngine ruleEngine;

    public RiskRating execute(Application app) {
        // 协调多个L5步骤和领域服务
        CreditReport report = queryReportUC.execute(app.getIdCard());
        InternalScore score = scoringService.calculate(app);
        
        return ruleEngine.evaluate(report, score, app.getLoanAmount());
    }
}

// L3: 核心业务流程 - 复杂流程 Use Case (Saga协调器)
public class LoanApprovalOrchestrationUseCase {
// 协调多个L4活动：身份验证、信用评估、抵押品评估、审批决策
private final List<ActivityUseCase> l4Activities;
private final SagaCoordinator sagaCoordinator;

    public ApprovalResult execute(LoanApplication app) {
        // 定义Saga，包含多个L4活动及补偿逻辑
        Saga saga = sagaCoordinator
            .step("VerifyIdentity", verifyIdentityUC, compensateVerify)
            .step("AssessCredit", assessCreditRiskUC, compensateCreditCheck)
            .step("Approve", makeApprovalDecisionUC, null); // 最终步骤无补偿
        
        return saga.execute(app);
    }
}


🔗 分级的核心价值与映射关系

1. 确保战略-流程-系统对齐


企业战略 → L1/L2 (价值流) → 业务架构
业务流程 → L3/L4 (核心流程/活动) → 应用架构
系统操作 → L5 (操作步骤) → 技术架构


2. 职责清晰分离

• 业务部门 主导 L1-L3：定义“做什么”（What）

• 业务分析师 细化 L4：定义“怎么做”（How）

• 技术团队 实现 L5：定义“如何实现”（Implementation）

3. 与敏捷开发匹配


史诗 (Epic) → L2/L3 流程
特性 (Feature) → L4 活动
用户故事 (User Story) → L5 操作步骤
任务 (Task) → Use Case 的具体实现


📋 实施建议：如何应用五级建模

步骤1：顶层设计（L1-L2）

# 定义业务能力地图
business_capabilities:
- id: BC_LOAN
  name: 信贷业务能力
  level: L1
  processes:
    - id: P_PERSONAL_LOAN
      name: 个人贷款流程
      level: L2
      description: 从申请到放款的端到端流程


步骤2：流程分解（L3-L4）

<!-- L3 流程定义 -->
<WorkflowProcess Id="PERSONAL_LOAN_APPROVAL" Name="个人贷款审批流程" Level="L3">
  <Activities>
    <!-- L4 活动 -->
    <Activity Id="ACT_CREDIT_ASSESSMENT" Name="信用评估活动" Level="L4">
      <ExtendedAttributes>
        <ExtendedAttribute Name="Owner" Value="风险管理部"/>
        <ExtendedAttribute Name="SLA" Value="2小时"/>
      </ExtendedAttributes>
    </Activity>
  </Activities>
</WorkflowProcess>


步骤3：Use Case 实现（L5）

// 为每个L5步骤定义明确的Use Case
@UseCase
@Component
public class CalculateLoanToValueUseCase {
// 输入：抵押物价值、贷款金额
// 输出：LTV比率、是否超限
// 业务规则：LTV ≤ 70%
}


步骤4：建立追溯矩阵

流程级别 流程ID Use Case ID 负责团队 验收标准

L3 贷款审批 UC_ORCH_LOAN_APPROVAL 业务平台部 流程完成时间≤24h

L4 信用评估 UC_ACT_CREDIT_ASSESS 风险技术部 评估准确率≥95%

L5 查询征信 UC_TASK_QUERY_PBOC 集成团队 接口响应≤3s

🎯 总结：分级的意义

1. 结构化沟通：为业务、分析、技术提供统一语言
2. 精准估算：L5 Use Case 可作为工作量估算的基础单元
3. 影响分析：变更一个L5 Use Case，可清晰追溯影响的L4活动、L3流程
4. 复用管理：识别可复用的L5操作步骤（如“验证身份”）
5. 合规审计：满足监管对流程可追溯、可审计的要求

无论是华为的实践还是银行业的标准，其核心思想都是通过分级实现从战略到操作、从业务到技术的端到端对齐。在您的项目中，可以借鉴这一框架，结合Clean Architecture和XPDL，构建清晰、可管理、可追溯的Use Case体系。