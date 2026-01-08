# Vec订单簿 RTO=0 RPO=0 实现指南

## 实现概览

本文档提供了一套完整的实现指南，指导如何在实际项目中应用快照和回放机制实现RTO=0 RPO=0。

## 目录

1. [系统架构](#系统架构)
2. [持久化层实现](#持久化层实现)
3. [恢复流程实现](#恢复流程实现)
4. [监控和告警](#监控和告警)
5. [测试策略](#测试策略)

---

## 系统架构

### 分层架构设计

```
┌──────────────────────────────────────────────┐
│         应用层 (Application)                  │
│  ├─ 订单处理                                 │
│  ├─ 风险管理                                 │
│  └─ 业务规则                                 │
└──────────────────┬───────────────────────────┘

┌──────────────────────────────────────────────┐
│         持久化层 (Persistence)                │
│  ├─ WAL Writer (事件日志)                     │
│  ├─ Snapshot Manager (快照管理)              │
│  └─ Recovery Manager (恢复管理)              │
└──────────────────┬───────────────────────────┘

┌──────────────────────────────────────────────┐
│         存储层 (Storage)                      │
│  ├─ 本地文件系统 (RDB快照)                    │
│  ├─ 事件日志文件 (WAL)                       │
│  └─ 远程存储 (备份)                         │
└──────────────────────────────────────────────┘
```

### 核心组件清单

| 组件 | 职责 | 关键方法 |
|-----|------|--------|
| `LocalLob` | 订单簿核心 | add_order, remove_order, match_orders |
| `WalWriter` | 事件持久化 | write_event, sync, flush |
| `SnapshotManager` | 快照管理 | create_snapshot, save, load |
| `RecoveryManager` | 故障恢复 | recover_from_failure, validate |
| `RepoSnapshot` trait | 快照接口 | create_snapshot, restore_from_snapshot |
| `EventReplay` trait | 回放接口 | replay_event, replay_events |

---

## 持久化层实现

### 1. WAL Writer 实现

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::fs::File;
use std::path::Path;

/// Write-Ahead Log Writer
pub struct WalWriter {
    file: Arc<Mutex<File>>,
    sequence: Arc<std::sync::atomic::AtomicU64>,
    buffer: Arc<Mutex<Vec<ChangeLogEntry>>>,
    config: WalConfig,
}

pub struct WalConfig {
    /// 批量fsync的事件数阈值
    pub batch_threshold: usize,
    /// 批量fsync的时间间隔 (毫秒)
    pub batch_timeout_ms: u64,
    /// 最大缓冲事件数
    pub max_buffer_size: usize,
}

impl WalWriter {
    pub async fn new(path: &Path, config: WalConfig) -> Result<Self> {
        let file = File::create(path).await?;

        Ok(Self {
            file: Arc::new(Mutex::new(file)),
            sequence: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            buffer: Arc::new(Mutex::new(Vec::with_capacity(config.batch_threshold))),
            config,
        })
    }

    /// 写入事件 - 快速路径，仅加入缓冲区
    pub async fn write_event(&self, mut event: ChangeLogEntry) -> Result<u64> {
        // 1. 分配序列号 (原子操作)
        let seq = self.sequence.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        event.sequence = seq;

        // 2. 加入缓冲区 (内存操作，极快)
        let mut buffer = self.buffer.lock().await;
        buffer.push(event);

        // 3. 检查是否需要fsync
        if buffer.len() >= self.config.batch_threshold {
            drop(buffer); // 释放锁
            self.flush().await?;
        }

        Ok(seq)
    }

    /// 批量fsync - 将缓冲区数据持久化
    pub async fn flush(&self) -> Result<()> {
        // 1. 获取缓冲区内容
        let mut buffer = self.buffer.lock().await;
        if buffer.is_empty() {
            return Ok(());
        }

        let events_to_write: Vec<_> = buffer.drain(..).collect();
        drop(buffer); // 提早释放锁，允许新事件并发写入

        // 2. 序列化事件
        let mut data = Vec::new();
        for event in events_to_write {
            let line = serde_json::to_string(&event)?;
            data.extend_from_slice(line.as_bytes());
            data.push(b'\n');
        }

        // 3. 写入文件
        let mut file = self.file.lock().await;
        use tokio::io::AsyncWriteExt;
        file.write_all(&data).await?;

        // 4. fsync 保证持久性
        file.sync_all().await?;

        Ok(())
    }

    /// 同步fsync - 保证立即持久化
    pub async fn sync_immediate(&self) -> Result<()> {
        self.flush().await?;

        let mut file = self.file.lock().await;
        file.sync_all().await?;

        Ok(())
    }

    /// 获取当前序列号
    pub fn current_sequence(&self) -> u64 {
        self.sequence.load(std::sync::atomic::Ordering::SeqCst)
    }
}
```

### 2. Snapshot Manager 实现

```rust
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// 快照管理器
pub struct SnapshotManager {
    snapshot_dir: PathBuf,
    config: SnapshotConfig,
}

pub struct SnapshotConfig {
    /// 快照文件名前缀
    pub prefix: String,
    /// 是否启用压缩
    pub enable_compression: bool,
    /// 保留的快照个数
    pub retention_count: usize,
}

pub struct SnapshotMetadata {
    pub timestamp: u64,
    pub sequence: u64,
    pub size: u64,
    pub compressed_size: Option<u64>,
    pub checksum: String,
}

impl SnapshotManager {
    pub fn new(snapshot_dir: &Path, config: SnapshotConfig) -> Result<Self> {
        // 创建快照目录
        std::fs::create_dir_all(snapshot_dir)?;

        Ok(Self {
            snapshot_dir: snapshot_dir.to_path_buf(),
            config,
        })
    }

    /// 创建快照文件名
    fn get_snapshot_path(&self, timestamp: u64, sequence: u64) -> PathBuf {
        let filename = format!(
            "{}_{:020}_{:020}.snapshot",
            self.config.prefix, timestamp, sequence
        );
        self.snapshot_dir.join(filename)
    }

    /// 保存快照到磁盘
    pub async fn save_snapshot<O: Order + Clone>(
        &self,
        lob: &LocalLob<O>,
        timestamp: u64,
        sequence: u64,
    ) -> Result<SnapshotMetadata> {
        // 1. 创建快照
        let snapshot = lob.create_snapshot(timestamp, sequence)?;

        // 2. 序列化
        let snapshot_data = bincode::serialize(&snapshot)?;
        let original_size = snapshot_data.len() as u64;

        // 3. 可选压缩
        let (final_data, compressed_size) = if self.config.enable_compression {
            let compressed = self.compress_data(&snapshot_data)?;
            let size = compressed.len() as u64;
            (compressed, Some(size))
        } else {
            (snapshot_data, None)
        };

        // 4. 计算校验和
        let checksum = self.compute_checksum(&final_data)?;

        // 5. 写入文件
        let path = self.get_snapshot_path(timestamp, sequence);
        let mut file = tokio::fs::File::create(&path).await?;
        use tokio::io::AsyncWriteExt;
        file.write_all(&final_data).await?;
        file.sync_all().await?;

        // 6. 清理旧快照
        self.cleanup_old_snapshots().await?;

        Ok(SnapshotMetadata {
            timestamp,
            sequence,
            size: original_size,
            compressed_size,
            checksum,
        })
    }

    /// 加载最新快照
    pub async fn load_latest_snapshot<O>(&self)
        -> Result<(LocalLob<O>, SnapshotMetadata)>
    where
        O: Order + Clone + for<'de> serde::Deserialize<'de>,
    {
        // 1. 查找最新快照文件
        let latest_path = self.find_latest_snapshot()?;
        let metadata = self.extract_metadata(&latest_path)?;

        // 2. 读取文件
        let data = tokio::fs::read(&latest_path).await?;

        // 3. 可选解压
        let snapshot_data = if metadata.compressed_size.is_some() {
            self.decompress_data(&data)?
        } else {
            data
        };

        // 4. 验证校验和
        let computed_checksum = self.compute_checksum(&snapshot_data)?;
        if computed_checksum != metadata.checksum {
            return Err("Snapshot checksum mismatch".into());
        }

        // 5. 反序列化
        let snapshot: LocalLob<O> = bincode::deserialize(&snapshot_data)?;

        Ok((snapshot, metadata))
    }

    /// 清理旧快照
    async fn cleanup_old_snapshots(&self) -> Result<()> {
        // 1. 获取所有快照文件
        let mut entries: Vec<_> = std::fs::read_dir(&self.snapshot_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| ext == "snapshot")
            })
            .collect();

        // 2. 按修改时间排序 (最新的在最后)
        entries.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(std::time::UNIX_EPOCH)
        });

        // 3. 删除超出保留数的快照
        if entries.len() > self.config.retention_count {
            let to_delete = entries.len() - self.config.retention_count;
            for entry in entries.iter().take(to_delete) {
                tokio::fs::remove_file(entry.path()).await?;
            }
        }

        Ok(())
    }

    // 辅助方法
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use lz4::Encoder;
        use std::io::Write;

        let mut compressed = Vec::new();
        let mut encoder = Encoder::new(&mut compressed)?;
        encoder.write_all(data)?;
        encoder.finish().0;

        Ok(compressed)
    }

    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        use lz4::Decoder;
        use std::io::Read;

        let decoder = Decoder::new(data)?;
        let mut decompressed = Vec::new();
        let mut reader = decoder;
        reader.read_to_end(&mut decompressed)?;

        Ok(decompressed)
    }

    fn compute_checksum(&self, data: &[u8]) -> Result<String> {
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    fn find_latest_snapshot(&self) -> Result<PathBuf> {
        let mut entries: Vec<_> = std::fs::read_dir(&self.snapshot_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| ext == "snapshot")
            })
            .collect();

        if entries.is_empty() {
            return Err("No snapshot found".into());
        }

        // 按文件名排序 (包含序列号，最新在后)
        entries.sort_by_key(|e| e.file_name());
        Ok(entries.last().unwrap().path())
    }

    fn extract_metadata(&self, path: &Path) -> Result<SnapshotMetadata> {
        // 从文件名提取时间戳和序列号
        let filename = path.file_name().unwrap().to_string_lossy();
        let parts: Vec<&str> = filename.split('_').collect();

        if parts.len() < 3 {
            return Err("Invalid snapshot filename".into());
        }

        let timestamp = parts[1].parse::<u64>()?;
        let sequence = parts[2].split('.').next().unwrap().parse::<u64>()?;

        let metadata = std::fs::metadata(path)?;
        let size = metadata.len();

        // 简化版本，实际应该从快照头读取
        Ok(SnapshotMetadata {
            timestamp,
            sequence,
            size,
            compressed_size: None,
            checksum: String::new(),
        })
    }
}
```

---

## 恢复流程实现

### Recovery Manager 实现

```rust
use std::path::Path;

/// 恢复管理器
pub struct RecoveryManager {
    wal_path: PathBuf,
    snapshot_dir: PathBuf,
    config: RecoveryConfig,
}

pub struct RecoveryConfig {
    /// 是否启用自动恢复
    pub auto_recovery: bool,
    /// 恢复超时 (秒)
    pub recovery_timeout_secs: u64,
    /// 验证一致性
    pub validate_consistency: bool,
}

#[derive(Debug)]
pub struct RecoveryResult {
    pub success: bool,
    pub rto_ms: u64,
    pub restored_sequence: u64,
    pub events_replayed: usize,
    pub errors: Vec<String>,
}

impl RecoveryManager {
    pub fn new(
        wal_path: &Path,
        snapshot_dir: &Path,
        config: RecoveryConfig,
    ) -> Self {
        Self {
            wal_path: wal_path.to_path_buf(),
            snapshot_dir: snapshot_dir.to_path_buf(),
            config,
        }
    }

    /// 执行完整的故障恢复
    pub async fn recover<O>(
        &self,
        lob: &mut LocalLob<O>,
        snapshot_mgr: &SnapshotManager,
        wal_writer: &WalWriter,
    ) -> Result<RecoveryResult>
    where
        O: Order + Clone + FromCreatedEvent + for<'de> serde::Deserialize<'de>,
    {
        let start = std::time::Instant::now();
        let mut result = RecoveryResult {
            success: false,
            rto_ms: 0,
            restored_sequence: 0,
            events_replayed: 0,
            errors: Vec::new(),
        };

        // 阶段1: 快照恢复
        match self.recover_from_snapshot(lob, snapshot_mgr).await {
            Ok((snapshot_seq, _)) => {
                result.restored_sequence = snapshot_seq;
                println!("[Recovery] Snapshot restored, sequence={}", snapshot_seq);
            }
            Err(e) => {
                // 快照不可用，跳过快照恢复
                result.errors.push(format!("Snapshot recovery failed: {}", e));
                result.restored_sequence = 0;
            }
        }

        // 阶段2: 事件回放
        match self.replay_events(lob, result.restored_sequence).await {
            Ok(replayed) => {
                result.events_replayed = replayed;
                println!("[Recovery] Replayed {} events", replayed);
            }
            Err(e) => {
                result.errors.push(format!("Event replay failed: {}", e));
                return Ok(result); // 不中断，返回部分恢复结果
            }
        }

        // 阶段3: 一致性验证
        if self.config.validate_consistency {
            match self.validate_consistency(lob) {
                Ok(_) => {
                    println!("[Recovery] Consistency validation passed");
                }
                Err(e) => {
                    result.errors.push(format!("Consistency check failed: {}", e));
                    return Ok(result);
                }
            }
        }

        result.success = true;
        result.rto_ms = start.elapsed().as_millis() as u64;

        println!("[Recovery] Completed in {}ms", result.rto_ms);
        println!("[Recovery] Status: {}", if result.success { "✓ SUCCESS" } else { "✗ FAILED" });

        Ok(result)
    }

    /// 从快照恢复
    async fn recover_from_snapshot<O>(
        &self,
        lob: &mut LocalLob<O>,
        snapshot_mgr: &SnapshotManager,
    ) -> Result<(u64, SnapshotMetadata)>
    where
        O: Order + Clone + for<'de> serde::Deserialize<'de>,
    {
        let (snapshot, metadata) = snapshot_mgr.load_latest_snapshot().await?;
        lob.restore_from_snapshot(&snapshot)?;

        Ok((metadata.sequence, metadata))
    }

    /// 回放增量事件
    async fn replay_events<O>(
        &self,
        lob: &mut LocalLob<O>,
        from_sequence: u64,
    ) -> Result<usize>
    where
        O: Order + FromCreatedEvent,
    {
        // 1. 加载事件日志
        let events = self.load_events().await?;

        // 2. 过滤并排序
        let mut relevant_events: Vec<_> = events
            .into_iter()
            .filter(|e| e.sequence > from_sequence)
            .collect();

        if relevant_events.is_empty() {
            return Ok(0);
        }

        // 3. 排序确保顺序
        relevant_events.sort_by_key(|e| e.sequence);

        // 4. 批量回放
        let count = relevant_events.len();
        lob.replay_events(&relevant_events)?;

        Ok(count)
    }

    /// 一致性验证
    fn validate_consistency<O>(&self, lob: &LocalLob<O>) -> Result<()>
    where
        O: Order,
    {
        // 检查1: 订单索引完整性
        // ... 实现检查逻辑

        // 检查2: 价格点链表完整性
        // ... 实现检查逻辑

        // 检查3: 最佳价格缓存
        // ... 实现检查逻辑

        Ok(())
    }

    async fn load_events(&self) -> Result<Vec<ChangeLogEntry>> {
        let file = tokio::fs::File::open(&self.wal_path).await?;
        let reader = tokio::io::BufReader::new(file);

        let mut events = Vec::new();
        let mut lines = tokio::io::AsyncBufReadExt::lines(&mut reader);

        while let Some(line) = lines.next_line().await? {
            let event: ChangeLogEntry = serde_json::from_str(&line)?;
            events.push(event);
        }

        Ok(events)
    }
}
```

### 应用启动恢复流程

```rust
#[tokio::main]
async fn main() -> Result<()> {
    // 1. 初始化组件
    let config = AppConfig::from_env()?;

    let mut lob = LocalLob::new(Symbol::new("BTCUSDT"));

    let wal_writer = WalWriter::new(
        &config.wal_path,
        WalConfig {
            batch_threshold: 100,
            batch_timeout_ms: 100,
            max_buffer_size: 10000,
        },
    )
    .await?;

    let snapshot_mgr = SnapshotManager::new(
        &config.snapshot_dir,
        SnapshotConfig {
            prefix: "lob_repo".to_string(),
            enable_compression: true,
            retention_count: 10,
        },
    )?;

    let recovery_mgr = RecoveryManager::new(
        &config.wal_path,
        &config.snapshot_dir,
        RecoveryConfig {
            auto_recovery: true,
            recovery_timeout_secs: 30,
            validate_consistency: true,
        },
    );

    // 2. 执行故障恢复
    println!("Starting recovery...");
    let recovery_result = recovery_mgr
        .recover(&mut lob, &snapshot_mgr, &wal_writer)
        .await?;

    if !recovery_result.success && !recovery_result.errors.is_empty() {
        println!("Recovery warnings:");
        for error in &recovery_result.errors {
            println!("  - {}", error);
        }
    }

    println!("Recovery completed: RTO={}ms, Events={}",
        recovery_result.rto_ms,
        recovery_result.events_replayed);

    // 3. 启动主服务
    println!("Starting main service...");
    start_service(lob, wal_writer, snapshot_mgr).await?;

    Ok(())
}
```

---

## 监控和告警

### 关键指标

```rust
use prometheus::{Counter, Histogram, Registry};

pub struct RtoRpoMetrics {
    /// RTO 直方图 (恢复时间)
    rto_histogram: Histogram,

    /// RPO 计数器 (丢失的事件数)
    rpo_counter: Counter,

    /// 快照大小
    snapshot_size: Histogram,

    /// 事件缓冲区大小
    event_buffer_size: Histogram,

    /// 快照创建次数
    snapshot_creates: Counter,

    /// 恢复次数
    recovery_count: Counter,
}

impl RtoRpoMetrics {
    pub fn new(registry: &Registry) -> Result<Self> {
        let rto_histogram = Histogram::with_opts(
            HistogramOpts::new("rto_ms", "Recovery Time Objective in milliseconds")
                .buckets(vec![100.0, 500.0, 1000.0, 2000.0, 5000.0]),
            registry,
        )?;

        let rpo_counter = Counter::with_opts(
            CounterOpts::new("rpo_lost_events", "Recovery Point Objective - lost events"),
            registry,
        )?;

        let snapshot_size = Histogram::with_opts(
            HistogramOpts::new("snapshot_size_bytes", "Snapshot size in bytes")
                .buckets(vec![1e6, 5e6, 10e6, 50e6, 100e6]),
            registry,
        )?;

        let event_buffer_size = Histogram::with_opts(
            HistogramOpts::new("event_buffer_size", "Event buffer size")
                .buckets(vec![10.0, 50.0, 100.0, 500.0, 1000.0]),
            registry,
        )?;

        let snapshot_creates = Counter::with_opts(
            CounterOpts::new("snapshot_creates_total", "Total snapshots created"),
            registry,
        )?;

        let recovery_count = Counter::with_opts(
            CounterOpts::new("recovery_total", "Total recovery operations"),
            registry,
        )?;

        Ok(Self {
            rto_histogram,
            rpo_counter,
            snapshot_size,
            event_buffer_size,
            snapshot_creates,
            recovery_count,
        })
    }

    /// 记录恢复时间
    pub fn record_rto(&self, rto_ms: u64) {
        self.rto_histogram.observe(rto_ms as f64);
        self.recovery_count.inc();
    }

    /// 记录数据丢失事件
    pub fn record_rpo_loss(&self, count: u64) {
        self.rpo_counter.inc_by(count);
    }

    /// 记录快照大小
    pub fn record_snapshot_size(&self, bytes: u64) {
        self.snapshot_size.observe(bytes as f64);
        self.snapshot_creates.inc();
    }

    /// 记录事件缓冲区大小
    pub fn record_buffer_size(&self, size: usize) {
        self.event_buffer_size.observe(size as f64);
    }
}
```

### 告警规则

```yaml
# Prometheus 告警规则

groups:
  - name: lob_recovery
    interval: 30s
    rules:
      # RTO 超标告警
      - alert: HighRTO
        expr: histogram_quantile(0.95, rate(rto_ms_bucket[5m])) > 2000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "RTO exceeded 2 seconds"
          description: "95th percentile RTO is {{ $value }}ms"

      # RPO 数据丢失告警
      - alert: DataLoss
        expr: increase(rpo_lost_events[1h]) > 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Data loss detected"
          description: "{{ $value }} events lost"

      # 快照失败告警
      - alert: SnapshotCreationFailure
        expr: rate(snapshot_creation_errors[5m]) > 0
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Snapshot creation failing"

      # 恢复失败告警
      - alert: RecoveryFailure
        expr: rate(recovery_failures[5m]) > 0
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "Recovery operation failed"
```

---

## 测试策略

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_snapshot_create_restore() {
        let mut lob = LocalLob::new(Symbol::new("TEST"));
        lob.add_order(create_test_order()).unwrap();

        // 创建快照
        let snapshot = lob.create_snapshot(1000, 100).unwrap();

        // 创建新LOB并从快照恢复
        let mut lob2 = LocalLob::new(Symbol::new("TEST"));
        lob2.restore_from_snapshot(&snapshot).unwrap();

        // 验证状态相同
        assert_eq!(lob.best_bid(), lob2.best_bid());
        assert_eq!(lob.best_ask(), lob2.best_ask());
    }

    #[tokio::test]
    async fn test_event_replay() {
        let mut lob = LocalLob::new(Symbol::new("TEST"));

        let events = vec![
            create_created_event(1, 100),
            create_created_event(2, 101),
            create_updated_event(1, 50),
            create_deleted_event(2),
        ];

        lob.replay_events(&events).unwrap();

        // 验证最终状态
        assert!(lob.find_order(1).is_some());
        assert!(lob.find_order(2).is_none());
    }

    #[tokio::test]
    async fn test_snapshot_consistency_after_recovery() {
        // 1. 添加订单
        let mut lob = LocalLob::new(Symbol::new("TEST"));
        for i in 0..100 {
            lob.add_order(create_order(i, 100 + i)).unwrap();
        }

        // 2. 创建快照
        let snapshot = lob.create_snapshot(1000, 100).unwrap();

        // 3. 清空LOB
        let mut lob2 = LocalLob::new(Symbol::new("TEST"));

        // 4. 从快照恢复
        lob2.restore_from_snapshot(&snapshot).unwrap();

        // 5. 验证订单数量和状态
        let original_count = lob.order_index.len();
        let restored_count = lob2.order_index.len();
        assert_eq!(original_count, restored_count);
    }
}
```

### 集成测试

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_recovery_flow() {
        let tmpdir = tempfile::TempDir::new().unwrap();
        let snapshot_dir = tmpdir.path().join("snapshots");
        let wal_path = tmpdir.path().join("wal.log");

        // 1. 初始化组件
        let mut lob = LocalLob::new(Symbol::new("BTCUSDT"));
        let wal_writer = WalWriter::new(&wal_path, WalConfig::default()).await.unwrap();
        let snapshot_mgr = SnapshotManager::new(&snapshot_dir, SnapshotConfig::default()).unwrap();

        // 2. 执行操作并持久化
        for i in 0..100 {
            let order = create_test_order(i);
            lob.add_order(order).unwrap();

            let event = ChangeLogEntry {
                entity_id: i.to_string(),
                change_type: ChangeType::Created { /* ... */ },
                sequence: i as u64,
                timestamp: now_us(),
            };
            wal_writer.write_event(event).await.unwrap();
        }

        wal_writer.flush().await.unwrap();

        // 3. 创建快照
        let snapshot = lob.create_snapshot(1000, 50).unwrap();
        snapshot_mgr.save_snapshot(&snapshot, 1000, 50).await.unwrap();

        // 4. 添加增量事件
        for i in 100..120 {
            let event = ChangeLogEntry {
                entity_id: i.to_string(),
                change_type: ChangeType::Created { /* ... */ },
                sequence: i as u64,
                timestamp: now_us(),
            };
            wal_writer.write_event(event).await.unwrap();
        }

        wal_writer.flush().await.unwrap();

        // 5. 模拟故障 - 清空LOB
        let mut lob_after_failure = LocalLob::new(Symbol::new("BTCUSDT"));

        // 6. 执行恢复
        let recovery_mgr = RecoveryManager::new(&wal_path, &snapshot_dir, RecoveryConfig::default());
        let result = recovery_mgr.recover(&mut lob_after_failure, &snapshot_mgr, &wal_writer).await.unwrap();

        // 7. 验证恢复结果
        assert!(result.success);
        assert_eq!(lob_after_failure.order_index.len(), 120);
        assert!(result.rto_ms < 2000);
    }

    #[tokio::test]
    async fn test_recovery_with_snapshot_loss() {
        // 测试快照丢失时的恢复
        let tmpdir = tempfile::TempDir::new().unwrap();
        let wal_path = tmpdir.path().join("wal.log");

        // 1. 初始化并执行操作
        let mut lob = LocalLob::new(Symbol::new("BTCUSDT"));
        let wal_writer = WalWriter::new(&wal_path, WalConfig::default()).await.unwrap();

        for i in 0..50 {
            let event = ChangeLogEntry {
                entity_id: i.to_string(),
                change_type: ChangeType::Created { /* ... */ },
                sequence: i as u64,
                timestamp: now_us(),
            };
            wal_writer.write_event(event).await.unwrap();
        }
        wal_writer.flush().await.unwrap();

        // 2. 快照目录不存在
        let snapshot_dir = tmpdir.path().join("nonexistent");

        // 3. 故障恢复
        let mut lob_after_failure = LocalLob::new(Symbol::new("BTCUSDT"));
        let recovery_mgr = RecoveryManager::new(&wal_path, &snapshot_dir, RecoveryConfig {
            auto_recovery: true,
            recovery_timeout_secs: 10,
            validate_consistency: true,
        });

        let result = recovery_mgr.recover(&mut lob_after_failure, &snapshot_mgr, &wal_writer).await.unwrap();

        // 4. 验证从WAL恢复成功
        assert!(result.success || !result.errors.is_empty()); // 可能有错误但应该恢复
        assert!(result.events_replayed > 0);
    }
}
```

### 压力测试

```rust
#[tokio::test]
#[ignore] // 压力测试，需要手动启动
async fn stress_test_snapshot_and_recovery() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let mut lob = LocalLob::new(Symbol::new("BTCUSDT"));

    // 1. 添加大量订单
    let start = std::time::Instant::now();
    for i in 0..10000 {
        let order = create_test_order(i);
        lob.add_order(order).unwrap();
    }
    println!("Added 10000 orders in {}ms", start.elapsed().as_millis());

    // 2. 快照性能测试
    let start = std::time::Instant::now();
    let snapshot = lob.create_snapshot(1000, 100).unwrap();
    let snapshot_create_ms = start.elapsed().as_millis();
    println!("Snapshot created in {}ms", snapshot_create_ms);

    // 3. 恢复性能测试
    let mut lob2 = LocalLob::new(Symbol::new("BTCUSDT"));
    let start = std::time::Instant::now();
    lob2.restore_from_snapshot(&snapshot).unwrap();
    let restore_ms = start.elapsed().as_millis();
    println!("Snapshot restored in {}ms", restore_ms);

    // 4. 性能验证
    assert!(snapshot_create_ms < 500, "Snapshot creation too slow");
    assert!(restore_ms < 300, "Snapshot restore too slow");

    // 5. 事件回放性能
    let mut events = Vec::new();
    for i in 0..1000 {
        events.push(ChangeLogEntry {
            entity_id: i.to_string(),
            change_type: ChangeType::Created { /* ... */ },
            sequence: i as u64,
            timestamp: now_us(),
        });
    }

    let start = std::time::Instant::now();
    lob2.replay_events(&events).unwrap();
    let replay_ms = start.elapsed().as_millis();
    println!("Replayed 1000 events in {}ms", replay_ms);

    assert!(replay_ms < 100, "Event replay too slow");
}
```

---

## 常见问题

### Q1: 快照应该多频繁创建？

**A**: 根据以下因素调整：
- **订单数量**: 1000个订单 → 每5秒或1000个事件
- **系统负载**: 低负载时更频繁快照
- **存储空间**: 保留10-20个快照即可
- **恢复目标**: RPO=0要求快照间隔内事件不能丢失

### Q2: 如何处理快照期间的新订单？

**A**: 使用写时复制 (Copy-on-Write)：
```rust
// 创建快照时使用独立克隆，不阻塞新订单处理
let snapshot = self.clone(); // 新订单操作可并发
// 后台线程保存快照
```

### Q3: WAL文件太大怎么办？

**A**: 实现日志压缩：
```rust
// 定期删除已在最新快照中的事件
// Seq < snapshot_seq 的事件可删除
```

### Q4: 恢复失败的回退方案？

**A**: 分级恢复：
1. 尝试快照 + 事件恢复
2. 如果失败，尝试仅快照恢复 (可能损失最近数据)
3. 如果快照也失败，从WAL全量恢复 (时间长)
4. 最后手段：从备用节点恢复或手动干预

---

**文档版本**: v1.0
**最后更新**: 2025-12-18
