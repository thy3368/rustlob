# SRE运维领域能力体系

## 目录

- [1. 运维能力矩阵](#1-运维能力矩阵)
- [2. 可靠性工程](#2-可靠性工程)
- [3. 性能工程](#3-性能工程)
- [4. 安全运维](#4-安全运维)
- [5. 成本优化](#5-成本优化)
- [6. 自动化运维](#6-自动化运维)
- [7. 混沌工程](#7-混沌工程)
- [8. 事件响应](#8-事件响应)
- [9. 容量规划](#9-容量规划)
- [10. 最佳实践案例](#10-最佳实践案例)

---

## 1. 运维能力矩阵

### 1.1 能力成熟度模型

| 能力域 | L1 基础 | L2 标准 | L3 优化 | L4 创新 | L5 引领 |
|--------|---------|---------|---------|---------|---------|
| **可观测性** | 基础监控 | 全链路追踪 | 智能告警 | 预测性监控 | AIOps |
| **自动化** | 脚本化 | CI/CD | GitOps | 自愈系统 | 自主运维 |
| **可靠性** | 99% SLA | 99.9% SLA | 99.99% SLA | 容错设计 | 混沌工程常态化 |
| **安全** | 基础认证 | RBAC | 零信任 | 自动化安全扫描 | 安全左移 |
| **性能** | 基础优化 | APM | 实时优化 | 智能调优 | 自适应系统 |
| **成本** | 账单监控 | 成本归因 | 自动优化 | FinOps | 全局成本优化 |

### 1.2 核心能力地图

```
                    ┌─────────────────────────────────┐
                    │      SRE 核心能力体系           │
                    └───────────┬─────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        v                       v                       v
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│  可靠性工程    │      │  性能工程      │      │  安全运维      │
├───────────────┤      ├───────────────┤      ├───────────────┤
│ - SLO/SLI     │      │ - 性能基线     │      │ - 身份认证     │
│ - 故障预防     │      │ - 瓶颈分析     │      │ - 权限管理     │
│ - 容错设计     │      │ - 自动调优     │      │ - 漏洞扫描     │
│ - 灾难恢复     │      │ - 压测         │      │ - 合规审计     │
└───────────────┘      └───────────────┘      └───────────────┘
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        │                       │                       │
        v                       v                       v
┌───────────────┐      ┌───────────────┐      ┌───────────────┐
│  成本优化      │      │  自动化运维    │      │  混沌工程      │
├───────────────┤      ├───────────────┤      ├───────────────┤
│ - 资源优化     │      │ - IaC         │      │ - 故障注入     │
│ - 成本归因     │      │ - 自愈系统     │      │ - 韧性测试     │
│ - FinOps      │      │ - 智能调度     │      │ - 演练         │
│ - ROI分析     │      │ - 自动扩缩容   │      │ - 弱点发现     │
└───────────────┘      └───────────────┘      └───────────────┘
```

---

## 2. 可靠性工程

### 2.1 SLO/SLI体系设计

#### 2.1.1 SLI指标定义

```rust
// src/domain/entities/reliability/sli.rs

/// 服务级别指标(SLI)
pub struct ServiceLevelIndicator {
    pub id: SliId,
    pub name: String,
    pub service: ServiceId,
    pub metric: SliMetric,
    pub measurement_window: Duration,
    pub calculation_method: CalculationMethod,
}

pub enum SliMetric {
    /// 可用性: 成功请求比例
    Availability {
        success_criteria: SuccessCriteria,
    },
    /// 延迟: P99延迟
    Latency {
        threshold: Duration,
        percentile: f64,
    },
    /// 错误率
    ErrorRate {
        max_error_rate: f64,
    },
    /// 吞吐量
    Throughput {
        min_requests_per_second: f64,
    },
    /// 数据一致性
    Correctness {
        validation_rules: Vec<ValidationRule>,
    },
}

pub enum SuccessCriteria {
    /// HTTP状态码
    HttpStatus { accepted_codes: Vec<u16> },
    /// gRPC状态码
    GrpcStatus { accepted_codes: Vec<i32> },
    /// 自定义业务规则
    Custom { predicate: Box<dyn Fn(&Response) -> bool> },
}

impl ServiceLevelIndicator {
    /// 计算SLI当前值
    pub fn calculate(&self, metrics: &[Metric]) -> Result<f64, SliError> {
        match &self.metric {
            SliMetric::Availability { success_criteria } => {
                let total = metrics.len() as f64;
                let successful = metrics.iter()
                    .filter(|m| success_criteria.is_success(m))
                    .count() as f64;
                Ok(successful / total * 100.0)
            }
            SliMetric::Latency { threshold, percentile } => {
                let latencies: Vec<f64> = metrics.iter()
                    .map(|m| m.latency.as_secs_f64())
                    .collect();
                let p_value = calculate_percentile(&latencies, *percentile);
                Ok((p_value < threshold.as_secs_f64()) as u8 as f64 * 100.0)
            }
            SliMetric::ErrorRate { .. } => {
                let total = metrics.len() as f64;
                let errors = metrics.iter()
                    .filter(|m| m.is_error())
                    .count() as f64;
                Ok((1.0 - errors / total) * 100.0)
            }
            _ => unimplemented!(),
        }
    }
}
```

#### 2.1.2 SLO目标定义

```rust
// src/domain/entities/reliability/slo.rs

/// 服务级别目标(SLO)
pub struct ServiceLevelObjective {
    pub id: SloId,
    pub name: String,
    pub service: ServiceId,
    pub sli: SliId,
    pub target: f64,  // 目标值 (例如 99.9%)
    pub time_window: TimeWindow,
    pub error_budget: ErrorBudget,
    pub alert_thresholds: Vec<AlertThreshold>,
}

pub enum TimeWindow {
    Rolling { duration: Duration },
    Calendar { period: CalendarPeriod },
}

pub enum CalendarPeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

pub struct ErrorBudget {
    pub remaining: f64,
    pub consumed: f64,
    pub total: f64,
    pub burn_rate: f64,  // 当前消耗速率
}

impl ServiceLevelObjective {
    /// 计算错误预算
    pub fn calculate_error_budget(&self, current_sli: f64) -> ErrorBudget {
        let total_budget = 100.0 - self.target;
        let consumed = 100.0 - current_sli;
        let remaining = total_budget - consumed;
        let burn_rate = consumed / self.time_window.as_hours() as f64;

        ErrorBudget {
            remaining,
            consumed,
            total: total_budget,
            burn_rate,
        }
    }

    /// 检查是否需要告警
    pub fn should_alert(&self, error_budget: &ErrorBudget) -> Option<AlertLevel> {
        for threshold in &self.alert_thresholds {
            if error_budget.burn_rate > threshold.burn_rate_threshold {
                return Some(threshold.alert_level.clone());
            }
        }
        None
    }

    /// 是否应该停止发布
    pub fn should_freeze_deployments(&self, error_budget: &ErrorBudget) -> bool {
        error_budget.remaining < 10.0  // 剩余不足10%
    }
}

pub struct AlertThreshold {
    pub burn_rate_threshold: f64,
    pub alert_level: AlertLevel,
    pub notification_channels: Vec<NotificationChannel>,
}

pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}
```

#### 2.1.3 SLO配置示例

```yaml
# config/slos/payment-service.yaml
apiVersion: sre.example.com/v1
kind: ServiceLevelObjective
metadata:
  name: payment-service-availability
  namespace: production
spec:
  service: payment-service
  description: "Payment service availability SLO"

  # SLI定义
  sli:
    metric: availability
    successCriteria:
      httpStatusCodes: [200, 201, 202]
    measurementWindow: 5m

  # SLO目标
  target: 99.95  # 99.95% 可用性
  timeWindow:
    type: rolling
    duration: 30d

  # 告警阈值
  alerting:
    # 快速burn rate（5分钟窗口）
    - window: 5m
      burnRateThreshold: 14.4  # 1小时内耗尽错误预算
      severity: critical
      notificationChannels:
        - pagerduty-critical

    # 中速burn rate（1小时窗口）
    - window: 1h
      burnRateThreshold: 6.0  # 5小时内耗尽
      severity: warning
      notificationChannels:
        - slack-sre-team

    # 慢速burn rate（6小时窗口）
    - window: 6h
      burnRateThreshold: 1.0  # 30天正常消耗
      severity: info
      notificationChannels:
        - email-team

  # 错误预算策略
  errorBudgetPolicy:
    # 剩余预算 < 10% 时冻结非紧急发布
    freezeDeploymentsThreshold: 10
    # 剩余预算 < 5% 时回滚最近发布
    autoRollbackThreshold: 5

---
apiVersion: sre.example.com/v1
kind: ServiceLevelObjective
metadata:
  name: payment-service-latency
  namespace: production
spec:
  service: payment-service
  description: "Payment service P99 latency SLO"

  sli:
    metric: latency
    percentile: 99
    threshold: 500ms
    measurementWindow: 5m

  target: 99.0  # 99% 的请求 < 500ms
  timeWindow:
    type: rolling
    duration: 30d

  alerting:
    - window: 5m
      burnRateThreshold: 10.0
      severity: warning
      notificationChannels:
        - slack-sre-team
```

### 2.2 故障预防机制

#### 2.2.1 预变更影响分析

```rust
// src/application/usecases/reliability/change_impact_analysis.rs

pub struct ChangeImpactAnalyzer {
    dependency_graph: Arc<ServiceDependencyGraph>,
    historical_incidents: Arc<dyn IncidentRepository>,
    slo_calculator: Arc<dyn SloCalculator>,
}

impl ChangeImpactAnalyzer {
    /// 分析变更的潜在影响
    pub async fn analyze_change_impact(
        &self,
        change: &Change,
    ) -> Result<ImpactAnalysisReport, AnalysisError> {
        // 1. 识别受影响的服务
        let affected_services = self.identify_affected_services(change).await?;

        // 2. 计算影响范围
        let blast_radius = self.calculate_blast_radius(&affected_services).await?;

        // 3. 分析历史风险
        let historical_risk = self.analyze_historical_risk(change).await?;

        // 4. 评估SLO影响
        let slo_impact = self.assess_slo_impact(&affected_services).await?;

        // 5. 生成建议
        let recommendations = self.generate_recommendations(
            &blast_radius,
            &historical_risk,
            &slo_impact,
        );

        Ok(ImpactAnalysisReport {
            change_id: change.id.clone(),
            affected_services,
            blast_radius,
            historical_risk,
            slo_impact,
            recommendations,
            risk_score: self.calculate_risk_score(&blast_radius, &historical_risk),
        })
    }

    async fn identify_affected_services(
        &self,
        change: &Change,
    ) -> Result<Vec<ServiceId>, AnalysisError> {
        let mut affected = vec![change.target_service.clone()];

        // 识别下游依赖
        let downstream = self.dependency_graph
            .downstream_services(&change.target_service)
            .await?;

        affected.extend(downstream);

        // 识别共享资源依赖
        let shared_resources = self.dependency_graph
            .shared_resources(&change.target_service)
            .await?;

        for resource in shared_resources {
            let consumers = self.dependency_graph
                .resource_consumers(&resource)
                .await?;
            affected.extend(consumers);
        }

        Ok(affected)
    }

    async fn calculate_blast_radius(
        &self,
        services: &[ServiceId],
    ) -> Result<BlastRadius, AnalysisError> {
        let total_services = self.dependency_graph.total_services().await?;
        let affected_percentage = (services.len() as f64 / total_services as f64) * 100.0;

        let total_traffic = self.get_total_traffic().await?;
        let affected_traffic: f64 = services.iter()
            .map(|s| self.get_service_traffic(s))
            .sum::<f64>()
            .await?;
        let traffic_percentage = (affected_traffic / total_traffic) * 100.0;

        Ok(BlastRadius {
            affected_services: services.len(),
            affected_percentage,
            affected_traffic,
            traffic_percentage,
            critical_services: self.identify_critical_services(services).await?,
        })
    }

    async fn analyze_historical_risk(
        &self,
        change: &Change,
    ) -> Result<HistoricalRisk, AnalysisError> {
        // 查询类似变更的历史事故
        let similar_changes = self.historical_incidents
            .find_similar_changes(change)
            .await?;

        let incident_rate = similar_changes.iter()
            .filter(|c| c.caused_incident)
            .count() as f64 / similar_changes.len() as f64;

        let mean_time_to_detect = similar_changes.iter()
            .filter_map(|c| c.time_to_detect)
            .sum::<Duration>()
            / similar_changes.len() as u32;

        let mean_time_to_resolve = similar_changes.iter()
            .filter_map(|c| c.time_to_resolve)
            .sum::<Duration>()
            / similar_changes.len() as u32;

        Ok(HistoricalRisk {
            similar_changes_count: similar_changes.len(),
            incident_rate,
            mean_time_to_detect,
            mean_time_to_resolve,
            common_failure_modes: self.identify_common_failures(&similar_changes),
        })
    }

    fn generate_recommendations(
        &self,
        blast_radius: &BlastRadius,
        historical_risk: &HistoricalRisk,
        slo_impact: &SloImpact,
    ) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();

        // 根据影响范围推荐策略
        if blast_radius.affected_percentage > 50.0 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: "使用蓝绿部署策略，确保可快速回滚".to_string(),
            });
        }

        if blast_radius.critical_services.len() > 0 {
            recommendations.push(Recommendation {
                priority: Priority::Critical,
                action: "影响关键服务，建议在低峰期进行变更".to_string(),
            });
        }

        // 根据历史风险推荐
        if historical_risk.incident_rate > 0.1 {
            recommendations.push(Recommendation {
                priority: Priority::High,
                action: format!(
                    "类似变更历史事故率{:.1}%，建议增加测试覆盖",
                    historical_risk.incident_rate * 100.0
                ),
            });
        }

        // 根据SLO影响推荐
        if slo_impact.error_budget_at_risk {
            recommendations.push(Recommendation {
                priority: Priority::Critical,
                action: "当前错误预算不足，建议推迟非紧急变更".to_string(),
            });
        }

        recommendations
    }
}
```

#### 2.2.2 自动化金丝雀分析

```rust
// src/application/usecases/reliability/canary_analysis.rs

pub struct AutomatedCanaryAnalyzer {
    metrics_repo: Arc<dyn MetricRepository>,
    statistical_analyzer: Arc<dyn StatisticalAnalyzer>,
    slo_calculator: Arc<dyn SloCalculator>,
}

impl AutomatedCanaryAnalyzer {
    /// 自动分析金丝雀部署健康状况
    pub async fn analyze_canary(
        &self,
        deployment: &Deployment,
        canary_percentage: u8,
    ) -> Result<CanaryAnalysisResult, AnalysisError> {
        let baseline_metrics = self.collect_baseline_metrics(deployment).await?;
        let canary_metrics = self.collect_canary_metrics(deployment).await?;

        // 1. 统计显著性检验
        let statistical_comparison = self.statistical_analyzer
            .compare_distributions(&baseline_metrics, &canary_metrics)
            .await?;

        // 2. SLO合规性检查
        let slo_compliance = self.check_slo_compliance(&canary_metrics).await?;

        // 3. 异常检测
        let anomalies = self.detect_anomalies(&canary_metrics, &baseline_metrics).await?;

        // 4. 综合评分
        let health_score = self.calculate_health_score(
            &statistical_comparison,
            &slo_compliance,
            &anomalies,
        );

        let verdict = self.make_verdict(health_score, &anomalies);

        Ok(CanaryAnalysisResult {
            deployment_id: deployment.id.clone(),
            canary_percentage,
            statistical_comparison,
            slo_compliance,
            anomalies,
            health_score,
            verdict,
            recommendation: self.generate_recommendation(&verdict, &anomalies),
        })
    }

    async fn compare_distributions(
        &self,
        baseline: &[Metric],
        canary: &[Metric],
    ) -> Result<StatisticalComparison, AnalysisError> {
        // Mann-Whitney U检验（非参数检验）
        let latency_pvalue = self.mann_whitney_u_test(
            &baseline.iter().map(|m| m.latency).collect::<Vec<_>>(),
            &canary.iter().map(|m| m.latency).collect::<Vec<_>>(),
        )?;

        // 卡方检验（错误率）
        let error_rate_pvalue = self.chi_square_test(
            baseline.iter().filter(|m| m.is_error()).count(),
            baseline.len(),
            canary.iter().filter(|m| m.is_error()).count(),
            canary.len(),
        )?;

        Ok(StatisticalComparison {
            latency_difference: LatencyDifference {
                baseline_p99: calculate_percentile(&baseline, 0.99),
                canary_p99: calculate_percentile(&canary, 0.99),
                pvalue: latency_pvalue,
                is_significant: latency_pvalue < 0.05,
            },
            error_rate_difference: ErrorRateDifference {
                baseline_rate: self.calculate_error_rate(baseline),
                canary_rate: self.calculate_error_rate(canary),
                pvalue: error_rate_pvalue,
                is_significant: error_rate_pvalue < 0.05,
            },
        })
    }

    fn make_verdict(
        &self,
        health_score: f64,
        anomalies: &[Anomaly],
    ) -> CanaryVerdict {
        if health_score >= 90.0 && anomalies.is_empty() {
            CanaryVerdict::Pass
        } else if health_score >= 70.0 && anomalies.iter().all(|a| a.severity != Severity::Critical) {
            CanaryVerdict::Marginal
        } else {
            CanaryVerdict::Fail {
                reasons: self.collect_failure_reasons(anomalies),
            }
        }
    }
}
```

### 2.3 容错设计模式

#### 2.3.1 断路器模式

```rust
// src/infrastructure/resilience/circuit_breaker.rs

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct CircuitBreaker {
    state: Arc<Mutex<CircuitBreakerState>>,
    config: CircuitBreakerConfig,
    metrics: Arc<Mutex<CircuitBreakerMetrics>>,
}

pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout: Duration,
    pub half_open_max_calls: u32,
}

struct CircuitBreakerState {
    status: CircuitStatus,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<Instant>,
    half_open_calls: u32,
}

pub enum CircuitStatus {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(CircuitBreakerState {
                status: CircuitStatus::Closed,
                failure_count: 0,
                success_count: 0,
                last_failure_time: None,
                half_open_calls: 0,
            })),
            config,
            metrics: Arc::new(Mutex::new(CircuitBreakerMetrics::default())),
        }
    }

    pub async fn call<F, T, E>(&self, operation: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Result<T, E> + Send,
        E: std::error::Error,
    {
        // 检查断路器状态
        if !self.allow_request()? {
            return Err(CircuitBreakerError::Open);
        }

        // 执行操作
        let start = Instant::now();
        let result = operation();
        let duration = start.elapsed();

        // 记录结果
        match result {
            Ok(value) => {
                self.record_success(duration);
                Ok(value)
            }
            Err(e) => {
                self.record_failure(duration);
                Err(CircuitBreakerError::CallFailed(e.to_string()))
            }
        }
    }

    fn allow_request(&self) -> Result<bool, CircuitBreakerError> {
        let mut state = self.state.lock().unwrap();

        match state.status {
            CircuitStatus::Closed => Ok(true),
            CircuitStatus::Open => {
                // 检查是否应该进入半开状态
                if let Some(last_failure) = state.last_failure_time {
                    if last_failure.elapsed() > self.config.timeout {
                        state.status = CircuitStatus::HalfOpen;
                        state.half_open_calls = 0;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            CircuitStatus::HalfOpen => {
                // 半开状态限制并发请求数
                if state.half_open_calls < self.config.half_open_max_calls {
                    state.half_open_calls += 1;
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
        }
    }

    fn record_success(&self, duration: Duration) {
        let mut state = self.state.lock().unwrap();
        let mut metrics = self.metrics.lock().unwrap();

        metrics.record_success(duration);

        match state.status {
            CircuitStatus::HalfOpen => {
                state.success_count += 1;
                if state.success_count >= self.config.success_threshold {
                    // 恢复到关闭状态
                    state.status = CircuitStatus::Closed;
                    state.failure_count = 0;
                    state.success_count = 0;
                    tracing::info!("Circuit breaker closed");
                }
            }
            CircuitStatus::Closed => {
                state.failure_count = 0;
            }
            _ => {}
        }
    }

    fn record_failure(&self, duration: Duration) {
        let mut state = self.state.lock().unwrap();
        let mut metrics = self.metrics.lock().unwrap();

        metrics.record_failure(duration);
        state.last_failure_time = Some(Instant::now());

        match state.status {
            CircuitStatus::HalfOpen => {
                // 半开状态失败则立即打开
                state.status = CircuitStatus::Open;
                state.failure_count = 0;
                state.success_count = 0;
                tracing::warn!("Circuit breaker opened from half-open state");
            }
            CircuitStatus::Closed => {
                state.failure_count += 1;
                if state.failure_count >= self.config.failure_threshold {
                    state.status = CircuitStatus::Open;
                    tracing::warn!("Circuit breaker opened");
                }
            }
            _ => {}
        }
    }

    pub fn metrics(&self) -> CircuitBreakerMetrics {
        self.metrics.lock().unwrap().clone()
    }
}

#[derive(Clone, Default)]
pub struct CircuitBreakerMetrics {
    pub total_calls: u64,
    pub successful_calls: u64,
    pub failed_calls: u64,
    pub rejected_calls: u64,
    pub avg_response_time: Duration,
}
```

#### 2.3.2 重试与退避策略

```rust
// src/infrastructure/resilience/retry.rs

pub struct RetryPolicy {
    pub max_attempts: u32,
    pub backoff_strategy: BackoffStrategy,
    pub retryable_errors: Vec<ErrorType>,
}

pub enum BackoffStrategy {
    Fixed { interval: Duration },
    Linear { initial: Duration, increment: Duration },
    Exponential { initial: Duration, multiplier: f64, max: Duration },
    ExponentialWithJitter { initial: Duration, multiplier: f64, max: Duration },
}

impl BackoffStrategy {
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        match self {
            Self::Fixed { interval } => *interval,
            Self::Linear { initial, increment } => {
                *initial + *increment * attempt
            }
            Self::Exponential { initial, multiplier, max } => {
                let delay = initial.as_millis() as f64 * multiplier.powi(attempt as i32);
                Duration::from_millis(delay.min(max.as_millis() as f64) as u64)
            }
            Self::ExponentialWithJitter { initial, multiplier, max } => {
                let base_delay = initial.as_millis() as f64 * multiplier.powi(attempt as i32);
                let jitter = rand::random::<f64>() * base_delay * 0.1;  // ±10% jitter
                let delay = base_delay + jitter;
                Duration::from_millis(delay.min(max.as_millis() as f64) as u64)
            }
        }
    }
}

pub async fn retry_with_policy<F, T, E>(
    policy: &RetryPolicy,
    operation: F,
) -> Result<T, E>
where
    F: Fn() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
    E: std::error::Error,
{
    let mut attempt = 0;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt >= policy.max_attempts {
                    tracing::error!("Max retry attempts reached: {}", attempt);
                    return Err(error);
                }

                if !policy.is_retryable(&error) {
                    tracing::warn!("Non-retryable error encountered");
                    return Err(error);
                }

                let delay = policy.backoff_strategy.calculate_delay(attempt);
                tracing::warn!(
                    "Retry attempt {} after {:?}: {}",
                    attempt,
                    delay,
                    error
                );

                tokio::time::sleep(delay).await;
            }
        }
    }
}
```

---

## 3. 性能工程

### 3.1 性能基线建立

```rust
// src/application/usecases/performance/baseline_calculator.rs

pub struct PerformanceBaselineCalculator {
    metrics_repo: Arc<dyn MetricRepository>,
    statistical_analyzer: Arc<dyn StatisticalAnalyzer>,
}

impl PerformanceBaselineCalculator {
    /// 计算服务性能基线
    pub async fn calculate_baseline(
        &self,
        service: &ServiceId,
        time_range: &TimeRange,
    ) -> Result<PerformanceBaseline, CalculationError> {
        // 收集历史指标数据
        let metrics = self.metrics_repo
            .query_service_metrics(service, time_range)
            .await?;

        // 过滤异常值（使用IQR方法）
        let filtered_metrics = self.filter_outliers(&metrics);

        // 计算各维度基线
        let latency_baseline = self.calculate_latency_baseline(&filtered_metrics)?;
        let throughput_baseline = self.calculate_throughput_baseline(&filtered_metrics)?;
        let error_rate_baseline = self.calculate_error_rate_baseline(&filtered_metrics)?;
        let resource_baseline = self.calculate_resource_baseline(&filtered_metrics)?;

        Ok(PerformanceBaseline {
            service: service.clone(),
            calculated_at: Timestamp::now(),
            time_range: time_range.clone(),
            sample_size: filtered_metrics.len(),
            latency: latency_baseline,
            throughput: throughput_baseline,
            error_rate: error_rate_baseline,
            resources: resource_baseline,
        })
    }

    fn calculate_latency_baseline(
        &self,
        metrics: &[Metric],
    ) -> Result<LatencyBaseline, CalculationError> {
        let latencies: Vec<f64> = metrics.iter()
            .map(|m| m.latency.as_secs_f64())
            .collect();

        Ok(LatencyBaseline {
            p50: calculate_percentile(&latencies, 0.50),
            p95: calculate_percentile(&latencies, 0.95),
            p99: calculate_percentile(&latencies, 0.99),
            p999: calculate_percentile(&latencies, 0.999),
            mean: latencies.iter().sum::<f64>() / latencies.len() as f64,
            stddev: self.statistical_analyzer.standard_deviation(&latencies),
        })
    }

    fn filter_outliers(&self, metrics: &[Metric]) -> Vec<Metric> {
        let latencies: Vec<f64> = metrics.iter()
            .map(|m| m.latency.as_secs_f64())
            .collect();

        let q1 = calculate_percentile(&latencies, 0.25);
        let q3 = calculate_percentile(&latencies, 0.75);
        let iqr = q3 - q1;
        let lower_bound = q1 - 1.5 * iqr;
        let upper_bound = q3 + 1.5 * iqr;

        metrics.iter()
            .filter(|m| {
                let latency = m.latency.as_secs_f64();
                latency >= lower_bound && latency <= upper_bound
            })
            .cloned()
            .collect()
    }
}

pub struct PerformanceBaseline {
    pub service: ServiceId,
    pub calculated_at: Timestamp,
    pub time_range: TimeRange,
    pub sample_size: usize,
    pub latency: LatencyBaseline,
    pub throughput: ThroughputBaseline,
    pub error_rate: ErrorRateBaseline,
    pub resources: ResourceBaseline,
}

pub struct LatencyBaseline {
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub p999: f64,
    pub mean: f64,
    pub stddev: f64,
}
```

### 3.2 自动化性能测试

```yaml
# k6-load-test.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: k6-load-test
  namespace: performance
spec:
  template:
    spec:
      containers:
        - name: k6
          image: grafana/k6:latest
          args:
            - run
            - --out
            - influxdb=http://influxdb.performance.svc:8086/k6
            - /scripts/load-test.js
          env:
            - name: TARGET_URL
              value: "http://my-service.production.svc"
            - name: VUS
              value: "100"  # 虚拟用户数
            - name: DURATION
              value: "5m"
          volumeMounts:
            - name: scripts
              mountPath: /scripts
      volumes:
        - name: scripts
          configMap:
            name: k6-scripts
      restartPolicy: Never
```

```javascript
// k6-scripts/load-test.js
import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend } from 'k6/metrics';

// 自定义指标
const errorRate = new Rate('errors');
const latencyTrend = new Trend('latency');

export const options = {
  stages: [
    { duration: '1m', target: 50 },   // 爬坡
    { duration: '3m', target: 100 },  // 稳定负载
    { duration: '1m', target: 200 },  // 峰值负载
    { duration: '1m', target: 0 },    // 下降
  ],
  thresholds: {
    'http_req_duration': ['p(95)<500', 'p(99)<1000'],  // 95% < 500ms
    'errors': ['rate<0.01'],  // 错误率 < 1%
  },
};

export default function () {
  const payload = JSON.stringify({
    userId: `user_${__VU}`,
    action: 'purchase',
    amount: Math.random() * 1000,
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
    tags: {
      name: 'PaymentAPI',
    },
  };

  const response = http.post(
    `${__ENV.TARGET_URL}/api/v1/payment`,
    payload,
    params
  );

  // 检查响应
  const success = check(response, {
    'status is 200': (r) => r.status === 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });

  errorRate.add(!success);
  latencyTrend.add(response.timings.duration);

  sleep(1);
}

export function handleSummary(data) {
  return {
    '/tmp/summary.json': JSON.stringify(data),
    'stdout': textSummary(data, { indent: ' ', enableColors: true }),
  };
}
```

---

## 4. 安全运维

### 4.1 零信任架构实现

```yaml
# istio-authorization-policies.yaml
# 默认拒绝所有流量
apiVersion: security.istio.io/v1beta1
kind: AuthorizationPolicy
metadata:
  name: deny-all
  namespace: production
spec:
  {}  # 空规则 = 拒绝所有

---
# 允许特定服务间通信
apiVersion: security.istio.io/v1beta1
kind: AuthorizationPolicy
metadata:
  name: frontend-to-backend
  namespace: production
spec:
  selector:
    matchLabels:
      app: backend-service
  action: ALLOW
  rules:
    - from:
        - source:
            principals: ["cluster.local/ns/production/sa/frontend-service"]
      to:
        - operation:
            methods: ["GET", "POST"]
            paths: ["/api/v1/*"]
      when:
        - key: request.auth.claims[role]
          values: ["service-account"]

---
# JWT验证
apiVersion: security.istio.io/v1beta1
kind: RequestAuthentication
metadata:
  name: jwt-auth
  namespace: production
spec:
  selector:
    matchLabels:
      app: backend-service
  jwtRules:
    - issuer: "https://auth.example.com"
      jwksUri: "https://auth.example.com/.well-known/jwks.json"
      audiences:
        - "backend-service"
```

### 4.2 自动化安全扫描

```yaml
# trivy-scan-cronjob.yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: trivy-security-scan
  namespace: security
spec:
  schedule: "0 2 * * *"  # 每天凌晨2点
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: trivy
              image: aquasec/trivy:latest
              args:
                - image
                - --format
                - json
                - --output
                - /reports/scan-result.json
                - --severity
                - CRITICAL,HIGH
                - your-registry/your-image:latest
              volumeMounts:
                - name: reports
                  mountPath: /reports
          volumes:
            - name: reports
              persistentVolumeClaim:
                claimName: security-reports
          restartPolicy: OnFailure
```

---

## 5. 成本优化

### 5.1 资源右sizing

```rust
// src/application/usecases/cost/resource_optimizer.rs

pub struct ResourceOptimizer {
    metrics_repo: Arc<dyn MetricRepository>,
    cost_calculator: Arc<dyn CostCalculator>,
}

impl ResourceOptimizer {
    pub async fn analyze_resource_usage(
        &self,
        service: &ServiceId,
        time_range: &TimeRange,
    ) -> Result<OptimizationRecommendation, OptimizerError> {
        // 收集资源使用指标
        let cpu_usage = self.metrics_repo
            .query_cpu_usage(service, time_range)
            .await?;

        let memory_usage = self.metrics_repo
            .query_memory_usage(service, time_range)
            .await?;

        // 计算P95使用率
        let cpu_p95 = calculate_percentile(&cpu_usage, 0.95);
        let memory_p95 = calculate_percentile(&memory_usage, 0.95);

        // 当前资源配置
        let current_resources = self.get_current_resources(service).await?;

        // 计算推荐配置（P95 + 20%缓冲）
        let recommended_cpu = (cpu_p95 * 1.2).ceil() as u32;
        let recommended_memory = (memory_p95 * 1.2).ceil() as u32;

        // 计算成本节省
        let current_cost = self.cost_calculator
            .calculate_monthly_cost(&current_resources)
            .await?;

        let optimized_resources = ResourceSpec {
            cpu: recommended_cpu,
            memory: recommended_memory,
        };

        let optimized_cost = self.cost_calculator
            .calculate_monthly_cost(&optimized_resources)
            .await?;

        Ok(OptimizationRecommendation {
            service: service.clone(),
            current_resources,
            recommended_resources: optimized_resources,
            current_cost,
            optimized_cost,
            potential_savings: current_cost - optimized_cost,
            confidence: self.calculate_confidence(&cpu_usage, &memory_usage),
        })
    }
}
```

---

## 6. 混沌工程

### 6.1 Chaos Mesh实验

```yaml
# chaos-experiments/network-delay.yaml
apiVersion: chaos-mesh.org/v1alpha1
kind: NetworkChaos
metadata:
  name: network-delay-experiment
  namespace: chaos-testing
spec:
  action: delay
  mode: one
  selector:
    namespaces:
      - production
    labelSelectors:
      app: payment-service
  delay:
    latency: "100ms"
    correlation: "50"
    jitter: "10ms"
  duration: "5m"
  scheduler:
    cron: "@every 1h"

---
# Pod失败注入
apiVersion: chaos-mesh.org/v1alpha1
kind: PodChaos
metadata:
  name: pod-failure-experiment
  namespace: chaos-testing
spec:
  action: pod-failure
  mode: fixed-percent
  value: "10"  # 10% Pods失败
  selector:
    namespaces:
      - production
    labelSelectors:
      app: backend-service
  duration: "2m"

---
# 压力测试
apiVersion: chaos-mesh.org/v1alpha1
kind: StressChaos
metadata:
  name: cpu-stress-experiment
  namespace: chaos-testing
spec:
  mode: one
  selector:
    namespaces:
      - production
    labelSelectors:
      app: data-processor
  stressors:
    cpu:
      workers: 4
      load: 80
  duration: "10m"
```

---

## 10. 最佳实践案例

### 10.1 案例1：大规模微服务可观测性

**场景**: 1000+微服务，日处理10亿+请求

**挑战**:
- 海量指标存储成本高
- 全链路追踪采样率低导致问题难复现
- 告警风暴难以处理

**解决方案**:
```yaml
# 分层采样策略
sampling:
  # 默认采样率1%
  default: 0.01

  # 错误请求100%采样
  rules:
    - condition: status_code >= 500
      rate: 1.0

    # 慢请求100%采样
    - condition: duration > 1s
      rate: 1.0

    # VIP用户10%采样
    - condition: user_tier == "premium"
      rate: 0.1

# 指标聚合策略
aggregation:
  # 5分钟聚合窗口
  window: 5m

  # 只保留关键维度
  dimensions:
    - service
    - endpoint
    - status_code

  # 预聚合常用查询
  materialized_views:
    - name: service_error_rate_5m
      query: |
        sum(rate(http_requests_total{status=~"5.."}[5m])) by (service)
        /
        sum(rate(http_requests_total[5m])) by (service)
```

**效果**:
- 存储成本降低70%
- 关键问题追踪覆盖率达到100%
- 告警噪音降低85%

### 10.2 案例2：零停机数据库迁移

**场景**: PostgreSQL单机迁移至Aurora集群

**方案**:
1. **双写阶段**（Week 1-2）
   ```rust
   pub async fn write_order(&self, order: Order) -> Result<(), Error> {
       // 主库写入
       self.primary_db.insert_order(&order).await?;

       // 新库异步写入
       let new_db = self.new_db.clone();
       let order_clone = order.clone();
       tokio::spawn(async move {
           if let Err(e) = new_db.insert_order(&order_clone).await {
               tracing::error!("Failed to write to new DB: {}", e);
           }
       });

       Ok(())
   }
   ```

2. **数据校验**（Week 3）
   - 定时任务对比两个数据库数据一致性
   - 自动修复不一致数据

3. **流量切换**（Week 4）
   - 使用特性开关逐步切换读流量
   - 金丝雀发布：5% -> 25% -> 50% -> 100%

4. **回滚预案**
   - 一键切回旧库的脚本
   - 实时监控SLO指标

**效果**:
- ✅ 零停机完成迁移
- ✅ 性能提升3倍
- ✅ 成本降低40%

---

## 总结

本文档详细阐述了SRE运维领域的核心能力体系：

1. **可靠性工程**: SLO/SLI、故障预防、容错设计
2. **性能工程**: 基线建立、性能测试、持续优化
3. **安全运维**: 零信任架构、自动化扫描、合规审计
4. **成本优化**: 资源右sizing、FinOps实践
5. **自动化运维**: IaC、自愈系统、智能调度
6. **混沌工程**: 韧性测试、故障演练
7. **事件响应**: 快速定位、根因分析、自动修复

通过这些能力的系统化建设，可实现：
- 🎯 99.99%+ 服务可用性
- ⚡ MTTR < 5分钟
- 💰 成本降低30%+
- 🚀 部署频率提升10倍+

**持续改进方向**:
- AI驱动的智能运维（AIOps）
- 更精细的成本优化策略
- 全栈可观测性覆盖
- 自动化韧性测试

希望这套体系能帮助团队构建世界级的SRE能力！
