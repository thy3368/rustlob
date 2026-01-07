# SRE平台部署实践指南

## 目录

- [1. 快速开始](#1-快速开始)
- [2. Kubernetes集群搭建](#2-kubernetes集群搭建)
- [3. 可观测性栈部署](#3-可观测性栈部署)
- [4. 告警系统配置](#4-告警系统配置)
- [5. GitOps流水线](#5-gitops流水线)
- [6. 服务接入指南](#6-服务接入指南)
- [7. 故障排查](#7-故障排查)

---

## 1. 快速开始

### 1.1 前置条件

```bash
# 必需工具
- kubectl >= 1.30
- helm >= 3.14
- docker >= 24.0
- terraform >= 1.7 (可选)
- argocd CLI >= 2.10

# 资源要求（最小配置）
- 3个节点，每个节点：
  - CPU: 8 cores
  - Memory: 32GB
  - Storage: 500GB SSD
```

### 1.2 一键部署脚本

```bash
#!/bin/bash
# deploy-sre-platform.sh

set -e

echo "🚀 Starting SRE Platform Deployment..."

# 1. 验证前置条件
echo "✅ Checking prerequisites..."
command -v kubectl >/dev/null 2>&1 || { echo "kubectl not found"; exit 1; }
command -v helm >/dev/null 2>&1 || { echo "helm not found"; exit 1; }

# 2. 添加Helm仓库
echo "📦 Adding Helm repositories..."
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo add grafana https://grafana.github.io/helm-charts
helm repo add argo https://argoproj.github.io/argo-helm
helm repo add istio https://istio-release.storage.googleapis.com/charts
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update

# 3. 创建命名空间
echo "📁 Creating namespaces..."
kubectl create namespace observability --dry-run=client -o yaml | kubectl apply -f -
kubectl create namespace argocd --dry-run=client -o yaml | kubectl apply -f -
kubectl create namespace istio-system --dry-run=client -o yaml | kubectl apply -f -

# 4. 部署Istio
echo "🌐 Deploying Istio..."
helm install istio-base istio/base -n istio-system --wait
helm install istiod istio/istiod -n istio-system --wait
helm install istio-ingress istio/gateway -n istio-system --wait

# 5. 部署Prometheus Stack
echo "📊 Deploying Prometheus Stack..."
helm install kube-prometheus-stack prometheus-community/kube-prometheus-stack \
  -n observability \
  -f config/prometheus-values.yaml \
  --wait --timeout 10m

# 6. 部署VictoriaMetrics
echo "💾 Deploying VictoriaMetrics..."
helm install victoria-metrics-cluster prometheus-community/victoria-metrics-cluster \
  -n observability \
  -f config/victoriametrics-values.yaml \
  --wait

# 7. 部署Loki
echo "📝 Deploying Loki..."
helm install loki grafana/loki-stack \
  -n observability \
  -f config/loki-values.yaml \
  --wait

# 8. 部署Tempo
echo "🔍 Deploying Tempo..."
helm install tempo grafana/tempo-distributed \
  -n observability \
  -f config/tempo-values.yaml \
  --wait

# 9. 部署ArgoCD
echo "🔄 Deploying ArgoCD..."
helm install argocd argo/argo-cd \
  -n argocd \
  -f config/argocd-values.yaml \
  --wait

# 10. 部署SRE Platform应用
echo "🎯 Deploying SRE Platform..."
kubectl apply -f manifests/sre-platform/ -n observability

echo "✅ SRE Platform deployed successfully!"
echo ""
echo "📍 Access points:"
echo "  Grafana: kubectl port-forward -n observability svc/kube-prometheus-stack-grafana 3000:80"
echo "  ArgoCD: kubectl port-forward -n argocd svc/argocd-server 8080:443"
echo ""
echo "🔑 Grafana credentials:"
echo "  Username: admin"
echo "  Password: $(kubectl get secret -n observability kube-prometheus-stack-grafana -o jsonpath='{.data.admin-password}' | base64 -d)"
echo ""
echo "🔑 ArgoCD credentials:"
echo "  Username: admin"
echo "  Password: $(kubectl get secret -n argocd argocd-initial-admin-secret -o jsonpath='{.data.password}' | base64 -d)"
```

---

## 2. Kubernetes集群搭建

### 2.1 使用kubeadm搭建HA集群

```bash
#!/bin/bash
# init-k8s-cluster.sh

# 在所有节点执行
cat <<EOF | sudo tee /etc/modules-load.d/k8s.conf
overlay
br_netfilter
EOF

sudo modprobe overlay
sudo modprobe br_netfilter

cat <<EOF | sudo tee /etc/sysctl.d/k8s.conf
net.bridge.bridge-nf-call-iptables  = 1
net.bridge.bridge-nf-call-ip6tables = 1
net.ipv4.ip_forward                 = 1
EOF

sudo sysctl --system

# 安装容器运行时（containerd）
sudo apt-get update
sudo apt-get install -y containerd

sudo mkdir -p /etc/containerd
containerd config default | sudo tee /etc/containerd/config.toml
sudo systemctl restart containerd
sudo systemctl enable containerd

# 安装kubeadm、kubelet、kubectl
sudo apt-get update
sudo apt-get install -y apt-transport-https ca-certificates curl
curl -fsSL https://pkgs.k8s.io/core:/stable:/v1.30/deb/Release.key | \
  sudo gpg --dearmor -o /etc/apt/keyrings/kubernetes-apt-keyring.gpg

echo 'deb [signed-by=/etc/apt/keyrings/kubernetes-apt-keyring.gpg] https://pkgs.k8s.io/core:/stable:/v1.30/deb/ /' | \
  sudo tee /etc/apt/sources.list.d/kubernetes.list

sudo apt-get update
sudo apt-get install -y kubelet=1.30.0-1.1 kubeadm=1.30.0-1.1 kubectl=1.30.0-1.1
sudo apt-mark hold kubelet kubeadm kubectl

# 在第一个master节点初始化集群
sudo kubeadm init --control-plane-endpoint="load-balancer:6443" \
  --upload-certs \
  --pod-network-cidr=10.244.0.0/16 \
  --service-cidr=10.96.0.0/12

# 配置kubectl
mkdir -p $HOME/.kube
sudo cp -i /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config

# 安装CNI插件（Cilium）
helm repo add cilium https://helm.cilium.io/
helm install cilium cilium/cilium \
  --namespace kube-system \
  --set operator.replicas=1

# 其他master节点加入
# 使用kubeadm init输出的命令，类似：
# sudo kubeadm join load-balancer:6443 --token xxx \
#   --discovery-token-ca-cert-hash sha256:xxx \
#   --control-plane --certificate-key xxx

# Worker节点加入
# sudo kubeadm join load-balancer:6443 --token xxx \
#   --discovery-token-ca-cert-hash sha256:xxx
```

### 2.2 使用Terraform + EKS（AWS）

```hcl
# terraform/eks-cluster.tf
module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 20.0"

  cluster_name    = "sre-platform-cluster"
  cluster_version = "1.30"

  cluster_endpoint_public_access = true

  cluster_addons = {
    coredns = {
      most_recent = true
    }
    kube-proxy = {
      most_recent = true
    }
    vpc-cni = {
      most_recent = true
    }
    aws-ebs-csi-driver = {
      most_recent = true
    }
  }

  vpc_id     = module.vpc.vpc_id
  subnet_ids = module.vpc.private_subnets

  # EKS Managed Node Groups
  eks_managed_node_groups = {
    observability = {
      name = "observability-node-group"

      instance_types = ["m5.2xlarge"]
      capacity_type  = "ON_DEMAND"

      min_size     = 3
      max_size     = 10
      desired_size = 3

      labels = {
        workload = "observability"
      }

      taints = [{
        key    = "observability"
        value  = "true"
        effect = "NO_SCHEDULE"
      }]

      tags = {
        Environment = "production"
        Terraform   = "true"
      }
    }

    general = {
      name = "general-node-group"

      instance_types = ["m5.xlarge"]
      capacity_type  = "SPOT"

      min_size     = 2
      max_size     = 20
      desired_size = 5

      labels = {
        workload = "general"
      }
    }
  }

  # Cluster security group
  cluster_security_group_additional_rules = {
    ingress_nodes_ephemeral_ports_tcp = {
      description                = "Nodes on ephemeral ports"
      protocol                   = "tcp"
      from_port                  = 1025
      to_port                    = 65535
      type                       = "ingress"
      source_node_security_group = true
    }
  }

  # Node security group
  node_security_group_additional_rules = {
    ingress_self_all = {
      description = "Node to node all ports/protocols"
      protocol    = "-1"
      from_port   = 0
      to_port     = 0
      type        = "ingress"
      self        = true
    }
    egress_all = {
      description      = "Node all egress"
      protocol         = "-1"
      from_port        = 0
      to_port          = 0
      type             = "egress"
      cidr_blocks      = ["0.0.0.0/0"]
      ipv6_cidr_blocks = ["::/0"]
    }
  }

  tags = {
    Environment = "production"
    Terraform   = "true"
  }
}

# 配置kubectl
resource "null_resource" "kubectl_config" {
  depends_on = [module.eks]

  provisioner "local-exec" {
    command = "aws eks update-kubeconfig --region ${var.region} --name ${module.eks.cluster_name}"
  }
}
```

---

## 3. 可观测性栈部署

### 3.1 Prometheus Operator配置

```yaml
# config/prometheus-values.yaml
prometheus:
  prometheusSpec:
    replicas: 2
    retention: 15d
    retentionSize: "450GB"

    # 资源配置
    resources:
      requests:
        cpu: 2
        memory: 8Gi
      limits:
        cpu: 4
        memory: 16Gi

    # 存储配置
    storageSpec:
      volumeClaimTemplate:
        spec:
          storageClassName: fast-ssd
          accessModes: ["ReadWriteOnce"]
          resources:
            requests:
              storage: 500Gi

    # Remote Write to VictoriaMetrics
    remoteWrite:
      - url: http://victoria-metrics-cluster-vminsert.observability.svc:8480/insert/0/prometheus/api/v1/write
        queueConfig:
          maxSamplesPerSend: 10000
          maxShards: 30
          capacity: 100000

    # 服务发现配置
    serviceMonitorSelector: {}
    serviceMonitorNamespaceSelector: {}
    podMonitorSelector: {}
    podMonitorNamespaceSelector: {}

    # 高可用配置
    affinity:
      podAntiAffinity:
        requiredDuringSchedulingIgnoredDuringExecution:
          - labelSelector:
              matchExpressions:
                - key: app.kubernetes.io/name
                  operator: In
                  values:
                    - prometheus
            topologyKey: kubernetes.io/hostname

# Grafana配置
grafana:
  enabled: true
  replicas: 2

  # 持久化
  persistence:
    enabled: true
    storageClassName: standard
    size: 10Gi

  # 数据源
  additionalDataSources:
    - name: VictoriaMetrics
      type: prometheus
      url: http://victoria-metrics-cluster-vmselect.observability.svc:8481/select/0/prometheus
      access: proxy
      isDefault: true

    - name: Loki
      type: loki
      url: http://loki.observability.svc:3100
      access: proxy

    - name: Tempo
      type: tempo
      url: http://tempo-query-frontend.observability.svc:3100
      access: proxy

  # Dashboard配置
  dashboardProviders:
    dashboardproviders.yaml:
      apiVersion: 1
      providers:
        - name: 'default'
          orgId: 1
          folder: ''
          type: file
          disableDeletion: false
          editable: true
          options:
            path: /var/lib/grafana/dashboards/default

  # 预装Dashboard
  dashboards:
    default:
      kubernetes-cluster:
        gnetId: 7249
        revision: 1
        datasource: VictoriaMetrics
      node-exporter:
        gnetId: 1860
        revision: 27
        datasource: VictoriaMetrics
      prometheus-stats:
        gnetId: 2
        revision: 2
        datasource: VictoriaMetrics

  # 资源配置
  resources:
    requests:
      cpu: 500m
      memory: 1Gi
    limits:
      cpu: 1
      memory: 2Gi

# Alertmanager配置
alertmanager:
  alertmanagerSpec:
    replicas: 3

    storage:
      volumeClaimTemplate:
        spec:
          storageClassName: standard
          accessModes: ["ReadWriteOnce"]
          resources:
            requests:
              storage: 10Gi

    resources:
      requests:
        cpu: 100m
        memory: 256Mi
      limits:
        cpu: 200m
        memory: 512Mi
```

### 3.2 自定义ServiceMonitor示例

```yaml
# manifests/observability/service-monitors.yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: sre-platform-metrics
  namespace: observability
  labels:
    app: sre-platform
spec:
  selector:
    matchLabels:
      app: sre-platform
  endpoints:
    - port: metrics
      interval: 30s
      path: /metrics
      scheme: http
      # 自定义标签
      relabelings:
        - sourceLabels: [__meta_kubernetes_pod_name]
          targetLabel: pod
        - sourceLabels: [__meta_kubernetes_namespace]
          targetLabel: namespace
        - sourceLabels: [__meta_kubernetes_pod_node_name]
          targetLabel: node

---
apiVersion: monitoring.coreos.com/v1
kind: PodMonitor
metadata:
  name: istio-mesh-metrics
  namespace: observability
spec:
  selector:
    matchLabels:
      istio: monitor
  podMetricsEndpoints:
    - port: http-envoy-prom
      interval: 15s
      path: /stats/prometheus
      relabelings:
        - sourceLabels: [__meta_kubernetes_pod_container_name]
          action: keep
          regex: istio-proxy
```

### 3.3 Loki配置

```yaml
# config/loki-values.yaml
loki:
  auth_enabled: false

  server:
    http_listen_port: 3100

  ingester:
    lifecycler:
      ring:
        kvstore:
          store: inmemory
        replication_factor: 3
    chunk_idle_period: 15m
    chunk_retain_period: 30s
    max_chunk_age: 1h

  schema_config:
    configs:
      - from: 2024-01-01
        store: boltdb-shipper
        object_store: s3
        schema: v11
        index:
          prefix: loki_index_
          period: 24h

  storage_config:
    boltdb_shipper:
      active_index_directory: /loki/index
      cache_location: /loki/cache
      shared_store: s3

    aws:
      s3: s3://us-west-2/loki-bucket
      s3forcepathstyle: true

  compactor:
    working_directory: /loki/compactor
    shared_store: s3
    compaction_interval: 10m

  limits_config:
    enforce_metric_name: false
    reject_old_samples: true
    reject_old_samples_max_age: 168h
    ingestion_rate_mb: 50
    ingestion_burst_size_mb: 100

  chunk_store_config:
    max_look_back_period: 720h  # 30 days

  table_manager:
    retention_deletes_enabled: true
    retention_period: 720h

  ruler:
    storage:
      type: local
      local:
        directory: /rules
    rule_path: /tmp/rules
    alertmanager_url: http://kube-prometheus-stack-alertmanager.observability.svc:9093
    ring:
      kvstore:
        store: inmemory
    enable_api: true

# Promtail配置
promtail:
  enabled: true

  config:
    clients:
      - url: http://loki.observability.svc:3100/loki/api/v1/push
        tenant_id: default

    positions:
      filename: /run/promtail/positions.yaml

    scrape_configs:
      # Kubernetes Pod日志
      - job_name: kubernetes-pods
        kubernetes_sd_configs:
          - role: pod

        pipeline_stages:
          # 解析JSON日志
          - json:
              expressions:
                timestamp: timestamp
                level: level
                message: message
                trace_id: trace_id

          # 提取标签
          - labels:
              level:
              trace_id:

          # 时间戳解析
          - timestamp:
              source: timestamp
              format: RFC3339Nano

        relabel_configs:
          - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
            action: keep
            regex: true

          - source_labels: [__meta_kubernetes_pod_label_app]
            target_label: app

          - source_labels: [__meta_kubernetes_namespace]
            target_label: namespace

          - source_labels: [__meta_kubernetes_pod_name]
            target_label: pod
```

### 3.4 Tempo配置

```yaml
# config/tempo-values.yaml
tempo:
  replicas: 3

  storage:
    trace:
      backend: s3
      s3:
        bucket: tempo-traces
        endpoint: s3.us-west-2.amazonaws.com
        region: us-west-2

  compactor:
    compaction:
      block_retention: 720h  # 30 days

  distributor:
    receivers:
      otlp:
        protocols:
          grpc:
            endpoint: 0.0.0.0:4317
          http:
            endpoint: 0.0.0.0:4318
      jaeger:
        protocols:
          thrift_http:
            endpoint: 0.0.0.0:14268

  ingester:
    trace_idle_period: 10s
    max_block_bytes: 1048576
    max_block_duration: 5m

  querier:
    max_concurrent_queries: 20
    search:
      external_endpoints:
        - http://tempo-query-frontend.observability.svc:3100

  query_frontend:
    search:
      max_duration: 0
      default_result_limit: 20

# OpenTelemetry Collector
opentelemetry-collector:
  enabled: true
  mode: deployment

  config:
    receivers:
      otlp:
        protocols:
          grpc:
            endpoint: 0.0.0.0:4317
          http:
            endpoint: 0.0.0.0:4318

    processors:
      batch:
        timeout: 5s
        send_batch_size: 1024

      memory_limiter:
        check_interval: 1s
        limit_mib: 2048

      # 采样
      probabilistic_sampler:
        sampling_percentage: 10  # 10% 采样率

    exporters:
      otlp:
        endpoint: tempo-distributor.observability.svc:4317
        tls:
          insecure: true

      logging:
        loglevel: debug

    service:
      pipelines:
        traces:
          receivers: [otlp]
          processors: [memory_limiter, batch, probabilistic_sampler]
          exporters: [otlp, logging]
```

---

## 4. 告警系统配置

### 4.1 核心告警规则

```yaml
# manifests/observability/alert-rules.yaml
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: sre-core-alerts
  namespace: observability
spec:
  groups:
    # 服务可用性告警
    - name: service-availability
      interval: 30s
      rules:
        - alert: ServiceDown
          expr: up{job=~".*"} == 0
          for: 5m
          labels:
            severity: critical
            category: availability
          annotations:
            summary: "Service {{ $labels.job }} is down"
            description: "{{ $labels.job }} on {{ $labels.instance }} has been down for more than 5 minutes"
            runbook_url: "https://wiki.example.com/runbooks/service-down"

        - alert: HighErrorRate
          expr: |
            (
              sum(rate(http_requests_total{status=~"5.."}[5m])) by (service)
              /
              sum(rate(http_requests_total[5m])) by (service)
            ) > 0.05
          for: 5m
          labels:
            severity: warning
            category: errors
          annotations:
            summary: "High error rate on {{ $labels.service }}"
            description: "Error rate is {{ $value | humanizePercentage }} (threshold: 5%)"

    # 延迟告警
    - name: latency
      interval: 30s
      rules:
        - alert: HighLatencyP99
          expr: |
            histogram_quantile(0.99,
              sum(rate(http_request_duration_seconds_bucket[5m])) by (le, service)
            ) > 1
          for: 10m
          labels:
            severity: warning
            category: latency
          annotations:
            summary: "High P99 latency on {{ $labels.service }}"
            description: "P99 latency is {{ $value }}s (threshold: 1s)"

        - alert: HighLatencyP95
          expr: |
            histogram_quantile(0.95,
              sum(rate(http_request_duration_seconds_bucket[5m])) by (le, service)
            ) > 0.5
          for: 15m
          labels:
            severity: info
            category: latency
          annotations:
            summary: "Elevated P95 latency on {{ $labels.service }}"
            description: "P95 latency is {{ $value }}s (threshold: 0.5s)"

    # 资源告警
    - name: resources
      interval: 60s
      rules:
        - alert: HighCPUUsage
          expr: |
            100 - (avg by (instance) (
              rate(node_cpu_seconds_total{mode="idle"}[5m])
            ) * 100) > 80
          for: 15m
          labels:
            severity: warning
            category: resources
          annotations:
            summary: "High CPU usage on {{ $labels.instance }}"
            description: "CPU usage is {{ $value | humanize }}% (threshold: 80%)"

        - alert: HighMemoryUsage
          expr: |
            (1 - (
              node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes
            )) * 100 > 90
          for: 10m
          labels:
            severity: critical
            category: resources
          annotations:
            summary: "High memory usage on {{ $labels.instance }}"
            description: "Memory usage is {{ $value | humanize }}% (threshold: 90%)"

        - alert: DiskSpaceRunningOut
          expr: |
            (node_filesystem_avail_bytes{fstype!~"tmpfs|fuse.*"}
            / node_filesystem_size_bytes) * 100 < 10
          for: 5m
          labels:
            severity: critical
            category: resources
          annotations:
            summary: "Disk space running out on {{ $labels.instance }}"
            description: "Only {{ $value | humanize }}% space left on {{ $labels.mountpoint }}"

    # Kubernetes告警
    - name: kubernetes
      interval: 30s
      rules:
        - alert: PodCrashLooping
          expr: |
            rate(kube_pod_container_status_restarts_total[15m]) > 0
          for: 5m
          labels:
            severity: warning
            category: kubernetes
          annotations:
            summary: "Pod {{ $labels.namespace }}/{{ $labels.pod }} is crash looping"
            description: "Pod has restarted {{ $value }} times in the last 15 minutes"

        - alert: DeploymentReplicasMismatch
          expr: |
            kube_deployment_spec_replicas
            !=
            kube_deployment_status_replicas_available
          for: 15m
          labels:
            severity: warning
            category: kubernetes
          annotations:
            summary: "Deployment {{ $labels.namespace }}/{{ $labels.deployment }} replicas mismatch"
            description: "Expected replicas: {{ $labels.spec_replicas }}, Available: {{ $value }}"

        - alert: PodNotReady
          expr: |
            sum by (namespace, pod) (
              kube_pod_status_phase{phase!~"Running|Succeeded"}
            ) > 0
          for: 15m
          labels:
            severity: warning
            category: kubernetes
          annotations:
            summary: "Pod {{ $labels.namespace }}/{{ $labels.pod }} not ready"
            description: "Pod has been in {{ $labels.phase }} phase for more than 15 minutes"
```

### 4.2 Alertmanager配置

```yaml
# config/alertmanager-config.yaml
global:
  resolve_timeout: 5m
  slack_api_url: 'https://hooks.slack.com/services/YOUR/WEBHOOK'

# 路由树
route:
  receiver: 'default'
  group_by: ['alertname', 'cluster', 'service']
  group_wait: 10s
  group_interval: 10s
  repeat_interval: 12h

  routes:
    # P0告警 - 立即通知PagerDuty
    - match:
        severity: critical
      receiver: 'pagerduty-critical'
      group_wait: 10s
      repeat_interval: 5m
      continue: true

    # P1告警 - Slack + Email
    - match:
        severity: warning
      receiver: 'team-slack'
      group_wait: 30s
      repeat_interval: 4h

    # 业务时间内的告警
    - match:
        category: availability
      receiver: 'oncall-team'
      time_intervals:
        - business-hours

    # 特定服务的告警
    - match_re:
        service: payment-service|auth-service
      receiver: 'critical-services-team'
      continue: false

# 抑制规则
inhibit_rules:
  # 节点down时抑制该节点的所有告警
  - source_match:
      alertname: 'NodeDown'
    target_match_re:
      alertname: '.*'
    equal: ['instance']

  # 服务down时抑制高错误率告警
  - source_match:
      alertname: 'ServiceDown'
    target_match:
      alertname: 'HighErrorRate'
    equal: ['service']

  # Critical告警抑制Warning告警
  - source_match:
      severity: 'critical'
    target_match:
      severity: 'warning'
    equal: ['alertname', 'service']

# 时间窗口定义
time_intervals:
  - name: business-hours
    time_intervals:
      - times:
          - start_time: '09:00'
            end_time: '18:00'
        weekdays: ['monday:friday']
        location: 'America/New_York'

# 接收器配置
receivers:
  - name: 'default'
    slack_configs:
      - channel: '#alerts'
        title: 'SRE Alert'
        text: |
          {{ range .Alerts }}
            *Alert:* {{ .Labels.alertname }}
            *Severity:* {{ .Labels.severity }}
            *Description:* {{ .Annotations.description }}
            *Details:*
            {{ range .Labels.SortedPairs }} • *{{ .Name }}:* `{{ .Value }}`
            {{ end }}
          {{ end }}

  - name: 'pagerduty-critical'
    pagerduty_configs:
      - service_key: 'YOUR_PAGERDUTY_KEY'
        description: '{{ .GroupLabels.alertname }} - {{ .GroupLabels.service }}'
        severity: '{{ .CommonLabels.severity }}'
        details:
          firing: '{{ .Alerts.Firing | len }}'
          resolved: '{{ .Alerts.Resolved | len }}'
          num_firing: '{{ .Alerts.Firing | len }}'

  - name: 'team-slack'
    slack_configs:
      - channel: '#sre-team'
        send_resolved: true
        title: '[{{ .Status | toUpper }}] {{ .GroupLabels.alertname }}'
        text: |
          {{ range .Alerts }}
            *Alert:* {{ .Labels.alertname }}
            *Severity:* {{ .Labels.severity }}
            *Service:* {{ .Labels.service }}
            *Summary:* {{ .Annotations.summary }}
            *Description:* {{ .Annotations.description }}
            {{ if .Annotations.runbook_url }}*Runbook:* {{ .Annotations.runbook_url }}{{ end }}
          {{ end }}

  - name: 'oncall-team'
    email_configs:
      - to: 'oncall@example.com'
        from: 'alertmanager@example.com'
        smarthost: 'smtp.example.com:587'
        auth_username: 'alertmanager@example.com'
        auth_password: 'password'
        headers:
          Subject: '[{{ .Status }}] {{ .GroupLabels.alertname }}'

  - name: 'critical-services-team'
    webhook_configs:
      - url: 'http://sre-platform.observability.svc/api/v1/alerts/webhook'
        send_resolved: true
```

---

## 5. GitOps流水线

### 5.1 ArgoCD应用配置

```yaml
# argocd/applications/sre-platform.yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: sre-platform
  namespace: argocd
spec:
  project: default

  source:
    repoURL: https://github.com/your-org/sre-platform
    targetRevision: main
    path: manifests/sre-platform

    # Helm配置
    helm:
      valueFiles:
        - values-production.yaml
      parameters:
        - name: replicas
          value: "3"
        - name: image.tag
          value: "v1.2.3"

  destination:
    server: https://kubernetes.default.svc
    namespace: observability

  syncPolicy:
    automated:
      prune: true
      selfHeal: true
      allowEmpty: false

    syncOptions:
      - CreateNamespace=true
      - PrunePropagationPolicy=foreground
      - PruneLast=true

    retry:
      limit: 5
      backoff:
        duration: 5s
        factor: 2
        maxDuration: 3m

  # 健康检查
  ignoreDifferences:
    - group: apps
      kind: Deployment
      jsonPointers:
        - /spec/replicas

---
# argocd/projects/production.yaml
apiVersion: argoproj.io/v1alpha1
kind: AppProject
metadata:
  name: production
  namespace: argocd
spec:
  description: Production environment

  sourceRepos:
    - 'https://github.com/your-org/*'

  destinations:
    - namespace: 'observability'
      server: 'https://kubernetes.default.svc'
    - namespace: 'production'
      server: 'https://kubernetes.default.svc'

  clusterResourceWhitelist:
    - group: ''
      kind: Namespace
    - group: 'rbac.authorization.k8s.io'
      kind: ClusterRole
    - group: 'rbac.authorization.k8s.io'
      kind: ClusterRoleBinding

  namespaceResourceWhitelist:
    - group: '*'
      kind: '*'

  orphanedResources:
    warn: true
```

### 5.2 多环境管理

```
GitOps Repository Structure:
├── base/
│   ├── kustomization.yaml
│   ├── deployment.yaml
│   ├── service.yaml
│   └── configmap.yaml
├── overlays/
│   ├── dev/
│   │   ├── kustomization.yaml
│   │   ├── replicas.yaml
│   │   └── resources.yaml
│   ├── staging/
│   │   ├── kustomization.yaml
│   │   ├── replicas.yaml
│   │   └── resources.yaml
│   └── production/
│       ├── kustomization.yaml
│       ├── replicas.yaml
│       ├── resources.yaml
│       └── autoscaling.yaml
```

```yaml
# overlays/production/kustomization.yaml
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

namespace: production

resources:
  - ../../base

replicas:
  - name: sre-platform
    count: 3

images:
  - name: sre-platform
    newTag: v1.2.3

patches:
  - path: resources.yaml
  - path: autoscaling.yaml

configMapGenerator:
  - name: app-config
    behavior: merge
    literals:
      - ENVIRONMENT=production
      - LOG_LEVEL=info
      - METRICS_ENABLED=true
```

---

## 6. 服务接入指南

### 6.1 应用Instrumentation

#### Rust应用接入

```rust
// src/main.rs
use actix_web::{middleware, web, App, HttpServer};
use opentelemetry::{global, sdk::trace as sdktrace, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use prometheus::{Encoder, TextEncoder, register_histogram_vec, register_counter_vec};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 初始化可观测性
fn init_observability() {
    // 1. 配置OpenTelemetry追踪
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://otel-collector.observability.svc:4317")
        )
        .with_trace_config(
            sdktrace::config().with_resource(opentelemetry::sdk::Resource::new(vec![
                KeyValue::new("service.name", env!("CARGO_PKG_NAME")),
                KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                KeyValue::new("deployment.environment", std::env::var("ENVIRONMENT").unwrap_or_default()),
            ]))
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to initialize tracer");

    // 2. 配置日志
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_current_span(true)
                .with_span_list(true)
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // 3. 注册Prometheus指标
    register_metrics();
}

// 定义指标
lazy_static::lazy_static! {
    static ref HTTP_REQUEST_DURATION: prometheus::HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds",
        &["method", "endpoint", "status"]
    ).unwrap();

    static ref HTTP_REQUESTS_TOTAL: prometheus::CounterVec = register_counter_vec!(
        "http_requests_total",
        "Total HTTP requests",
        &["method", "endpoint", "status"]
    ).unwrap();
}

// Metrics端点
async fn metrics_handler() -> actix_web::Result<actix_web::HttpResponse> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(actix_web::HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(buffer))
}

// 中间件：记录请求指标
async fn metrics_middleware(
    req: actix_web::HttpRequest,
    srv: &mut actix_web::dev::ServiceRequest,
) -> Result<actix_web::dev::ServiceResponse, actix_web::Error> {
    let start = std::time::Instant::now();
    let method = req.method().to_string();
    let path = req.path().to_string();

    let res = srv.call().await?;

    let duration = start.elapsed().as_secs_f64();
    let status = res.status().as_u16().to_string();

    HTTP_REQUEST_DURATION
        .with_label_values(&[&method, &path, &status])
        .observe(duration);

    HTTP_REQUESTS_TOTAL
        .with_label_values(&[&method, &path, &status])
        .inc();

    Ok(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_observability();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap_fn(metrics_middleware)
            .route("/metrics", web::get().to(metrics_handler))
            .route("/health", web::get().to(health_check))
            .service(/* your routes */)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
```

#### Java应用接入

```java
// Application.java
import io.micrometer.core.instrument.MeterRegistry;
import io.micrometer.prometheus.PrometheusConfig;
import io.micrometer.prometheus.PrometheusMeterRegistry;
import io.opentelemetry.api.OpenTelemetry;
import io.opentelemetry.sdk.autoconfigure.AutoConfiguredOpenTelemetrySdk;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;

@SpringBootApplication
public class Application {

    public static void main(String[] args) {
        // 1. 初始化OpenTelemetry (通过环境变量配置)
        System.setProperty("otel.service.name", "my-service");
        System.setProperty("otel.exporter.otlp.endpoint", "http://otel-collector:4317");
        System.setProperty("otel.metrics.exporter", "otlp");
        System.setProperty("otel.logs.exporter", "otlp");

        OpenTelemetry openTelemetry = AutoConfiguredOpenTelemetrySdk.initialize()
                .getOpenTelemetrySdk();

        SpringApplication.run(Application.class, args);
    }

    @Bean
    public MeterRegistry meterRegistry() {
        return new PrometheusMeterRegistry(PrometheusConfig.DEFAULT);
    }
}

// MetricsController.java
import io.micrometer.prometheus.PrometheusMeterRegistry;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class MetricsController {

    private final PrometheusMeterRegistry registry;

    public MetricsController(PrometheusMeterRegistry registry) {
        this.registry = registry;
    }

    @GetMapping("/actuator/prometheus")
    public String prometheus() {
        return registry.scrape();
    }
}

// application.yml
management:
  endpoints:
    web:
      exposure:
        include: health,prometheus,metrics
  metrics:
    tags:
      application: ${spring.application.name}
      environment: ${ENVIRONMENT:dev}
    distribution:
      percentiles-histogram:
        http.server.requests: true
```

### 6.2 Kubernetes部署配置

```yaml
# manifests/service-deployment.yaml
apiVersion: v1
kind: Service
metadata:
  name: my-service
  namespace: production
  labels:
    app: my-service
    version: v1
  annotations:
    # Prometheus采集配置
    prometheus.io/scrape: "true"
    prometheus.io/port: "8080"
    prometheus.io/path: "/metrics"
spec:
  selector:
    app: my-service
  ports:
    - name: http
      port: 80
      targetPort: 8080
    - name: metrics
      port: 9090
      targetPort: 8080

---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-service
  namespace: production
  labels:
    app: my-service
    version: v1
spec:
  replicas: 3
  selector:
    matchLabels:
      app: my-service
      version: v1

  template:
    metadata:
      labels:
        app: my-service
        version: v1
      annotations:
        # Istio sidecar注入
        sidecar.istio.io/inject: "true"
        # Prometheus采集
        prometheus.io/scrape: "true"
        prometheus.io/port: "8080"
        prometheus.io/path: "/metrics"

    spec:
      # 服务账号
      serviceAccountName: my-service

      # 容器配置
      containers:
        - name: app
          image: your-registry/my-service:v1.2.3

          ports:
            - containerPort: 8080
              name: http
              protocol: TCP

          # 环境变量
          env:
            - name: ENVIRONMENT
              value: "production"
            - name: OTEL_EXPORTER_OTLP_ENDPOINT
              value: "http://otel-collector.observability.svc:4317"
            - name: LOG_LEVEL
              value: "info"

          # 资源限制
          resources:
            requests:
              cpu: 500m
              memory: 512Mi
            limits:
              cpu: 1000m
              memory: 1Gi

          # 健康检查
          livenessProbe:
            httpGet:
              path: /health/live
              port: 8080
            initialDelaySeconds: 30
            periodSeconds: 10
            timeoutSeconds: 5
            failureThreshold: 3

          readinessProbe:
            httpGet:
              path: /health/ready
              port: 8080
            initialDelaySeconds: 10
            periodSeconds: 5
            timeoutSeconds: 3
            failureThreshold: 3

          # 启动探针
          startupProbe:
            httpGet:
              path: /health/startup
              port: 8080
            initialDelaySeconds: 0
            periodSeconds: 10
            timeoutSeconds: 3
            failureThreshold: 30

      # Pod反亲和性（高可用）
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
            - weight: 100
              podAffinityTerm:
                labelSelector:
                  matchExpressions:
                    - key: app
                      operator: In
                      values:
                        - my-service
                topologyKey: kubernetes.io/hostname

---
# HPA配置
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: my-service
  namespace: production
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: my-service
  minReplicas: 3
  maxReplicas: 20
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
    - type: Resource
      resource:
        name: memory
        target:
          type: Utilization
          averageUtilization: 80
    - type: Pods
      pods:
        metric:
          name: http_requests_per_second
        target:
          type: AverageValue
          averageValue: "1000"
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
        - type: Percent
          value: 50
          periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
        - type: Percent
          value: 100
          periodSeconds: 15
        - type: Pods
          value: 4
          periodSeconds: 15
      selectPolicy: Max
```

---

## 7. 故障排查

### 7.1 常见问题排查流程

#### 7.1.1 服务不可用

```bash
# 1. 检查Pod状态
kubectl get pods -n production -l app=my-service
kubectl describe pod -n production <pod-name>

# 2. 查看日志
kubectl logs -n production <pod-name> --tail=100 --follow

# 3. 检查事件
kubectl get events -n production --sort-by='.lastTimestamp'

# 4. 检查服务端点
kubectl get endpoints -n production my-service

# 5. 测试服务连通性
kubectl run -it --rm debug --image=nicolaka/netshoot --restart=Never -- \
  curl http://my-service.production.svc/health

# 6. 查看Grafana监控
# 访问: http://grafana.example.com/d/service-dashboard
```

#### 7.1.2 高延迟排查

```bash
# 1. 查看Tempo追踪
# 在Grafana Explore中执行:
# Query: { service.name="my-service" } | duration > 1s

# 2. 检查慢查询日志
kubectl logs -n production <pod-name> | grep "duration_ms"

# 3. 分析Prometheus指标
# PromQL查询:
histogram_quantile(0.99,
  sum(rate(http_request_duration_seconds_bucket{service="my-service"}[5m])) by (le, endpoint)
)

# 4. 检查数据库连接
kubectl exec -it -n production <pod-name> -- netstat -an | grep ESTABLISHED
```

### 7.2 调试工具集

```bash
# debug-toolkit.sh
#!/bin/bash

# 部署调试Pod
kubectl apply -f - <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: debug-toolkit
  namespace: production
spec:
  containers:
    - name: toolkit
      image: nicolaka/netshoot
      command: ["/bin/bash"]
      args: ["-c", "sleep 3600"]
      securityContext:
        capabilities:
          add: ["NET_ADMIN", "SYS_PTRACE"]
EOF

# 等待Pod就绪
kubectl wait --for=condition=ready pod/debug-toolkit -n production --timeout=60s

echo "Debug toolkit deployed. Usage:"
echo "  kubectl exec -it -n production debug-toolkit -- bash"
echo ""
echo "Available tools:"
echo "  - curl, wget"
echo "  - tcpdump"
echo "  - nslookup, dig"
echo "  - netstat, ss"
echo "  - strace"
echo "  - iperf3"
```

---

## 总结

本部署指南提供了SRE平台从零到一的完整实施路径：

1. **基础设施**: Kubernetes集群搭建
2. **可观测性**: Prometheus、Loki、Tempo全栈部署
3. **告警系统**: 多层级告警配置与降噪策略
4. **自动化**: GitOps持续部署流水线
5. **服务接入**: 多语言应用Instrumentation
6. **故障排查**: 系统化排障流程与工具

通过遵循本指南，可在2-3周内搭建完整的企业级SRE平台，实现：
- ✅ 端到端可观测性
- ✅ 智能告警与自愈
- ✅ 声明式GitOps部署
- ✅ 99.99%服务可用性

**下一步**:
- 根据实际业务需求调整配置参数
- 建立SLO/SLI体系
- 定期进行混沌工程演练
- 持续优化成本与性能

祝部署顺利！🚀
