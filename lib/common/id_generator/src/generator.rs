use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH}
};

/// Snowflake ID生成器
///
/// ID结构 (64位):
/// - 1位: 符号位(固定为0)
/// - 41位: 时间戳(毫秒)
/// - 5位: 节点ID (支持32个节点)
/// - 12位: 序列号 (每毫秒4096个ID)

pub struct IdGenerator {
    /// 组合的时间戳和序列号 (高48位时间戳 + 低16位序列号)
    ts_and_seq: AtomicU64,
    /// 自定义起始时间 (2024-01-01 00:00:00 UTC)
    epoch: i64,
    /// 节点ID (0-31)
    node_id: u8
}

impl IdGenerator {
    const NODE_ID_BITS: u8 = 5;
    const SEQUENCE_BITS: u8 = 12;
    const MAX_NODE_ID: u8 = (1 << Self::NODE_ID_BITS) - 1; // 31
    const MAX_SEQUENCE: u16 = (1 << Self::SEQUENCE_BITS) - 1; // 4095

    /// 创建新的ID生成器
    ///
    /// # 参数
    /// - `node_id`: 节点ID，范围 0-31
    ///
    /// # 示例
    /// ```
    /// use id_generator::generator::IdGenerator;
    /// let generator = IdGenerator::new(0);
    /// ```
    pub fn new(node_id: u8) -> Self {
        Self {
            epoch: 1704067200000, // 2024-01-01 00:00:00 UTC
            node_id: node_id & Self::MAX_NODE_ID,
            ts_and_seq: AtomicU64::new(0)
        }
    }

    /// 生成下一个ID
    ///
    /// 线程安全，无锁实现，高性能
    ///
    /// # 返回
    /// 64位唯一ID
    ///
    /// # 示例
    /// ```
    /// use id_generator::generator::IdGenerator;
    /// let generator = IdGenerator::new(0);
    /// let id = generator.next_id();
    /// println!("Generated ID: {}", id);
    /// ```
    pub fn next_id(&self) -> i64 {
        loop {
            let now = self.current_millis();
            let current = self.ts_and_seq.load(Ordering::Acquire);
            let last_ts = current >> 16;
            let last_seq = current & 0xFFFF;

            let (new_ts, new_seq) = if now == last_ts as i64 {
                // 同一毫秒内，递增序列号
                let seq = last_seq + 1;
                if seq > Self::MAX_SEQUENCE as u64 {
                    // 序列号溢出，等待下一毫秒
                    continue;
                }
                (now as u64, seq)
            } else {
                // 新的毫秒，重置序列号
                (now as u64, 0)
            };

            let new_value = (new_ts << 16) | new_seq;

            // 使用CAS确保原子性
            match self.ts_and_seq.compare_exchange(current, new_value, Ordering::SeqCst, Ordering::Acquire) {
                Ok(_) => {
                    // 组装ID: [41位时间戳][5位节点ID][12位序列号]
                    let timestamp = now - self.epoch;
                    return (timestamp << (Self::NODE_ID_BITS + Self::SEQUENCE_BITS))
                        | ((self.node_id as i64) << Self::SEQUENCE_BITS)
                        | (new_seq as i64);
                }
                Err(_) => {
                    // CAS失败，其他线程已更新，重试
                    continue;
                }
            }
        }
    }

    /// 从ID中提取时间戳
    ///
    /// # 参数
    /// - `id`: 要解析的ID
    ///
    /// # 返回
    /// Unix时间戳(毫秒)
    pub fn extract_timestamp(&self, id: i64) -> i64 { (id >> (Self::NODE_ID_BITS + Self::SEQUENCE_BITS)) + self.epoch }

    /// 从ID中提取节点ID
    pub fn extract_node_id(&self, id: i64) -> u8 {
        ((id >> Self::SEQUENCE_BITS) & ((1 << Self::NODE_ID_BITS) - 1)) as u8
    }

    /// 从ID中提取序列号
    pub fn extract_sequence(&self, id: i64) -> u16 { (id & ((1 << Self::SEQUENCE_BITS) - 1)) as u16 }

    /// 获取当前时间戳(毫秒)
    #[inline]
    fn current_millis(&self) -> i64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;

    use super::*;

    #[test]
    fn test_generate_id() {
        let generator = IdGenerator::new(0);
        let id = generator.next_id();
        assert!(id > 0);
        println!("Generated ID: {}", id);
    }

    #[test]
    fn test_uniqueness() {
        let generator = IdGenerator::new(0);
        let id1 = generator.next_id();
        let id2 = generator.next_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_sequential() {
        let generator = IdGenerator::new(0);
        let id1 = generator.next_id();
        let id2 = generator.next_id();
        assert!(id2 > id1);
    }

    #[test]
    fn test_extract_node_id() {
        let generator = IdGenerator::new(15);
        let id = generator.next_id();
        let node_id = generator.extract_node_id(id);
        assert_eq!(node_id, 15);
    }

    #[test]
    fn test_extract_timestamp() {
        let generator = IdGenerator::new(0);
        let id = generator.next_id();
        let timestamp = generator.extract_timestamp(id);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
        assert!((timestamp - now).abs() < 1000);
    }

    #[test]
    fn test_concurrent() {
        use std::{sync::Arc, thread};

        let generator = Arc::new(IdGenerator::new(0));
        let handles: Vec<_> = (0..4)
            .map(|_| {
                let gen = Arc::clone(&generator);
                thread::spawn(move || {
                    let mut ids = Vec::new();
                    for _ in 0..1000 {
                        ids.push(gen.next_id());
                    }
                    ids
                })
            })
            .collect();

        let mut all_ids = Vec::new();
        for h in handles {
            all_ids.extend(h.join().unwrap());
        }

        let original_len = all_ids.len();
        all_ids.sort_unstable();
        all_ids.dedup();
        assert_eq!(all_ids.len(), original_len);
        println!("✅ {} unique IDs", original_len);
    }

    #[test]
    fn abc() {
        static ORDER_ID_GEN: Lazy<IdGenerator> = Lazy::new(|| {
            let node_id = std::env::var("NODE_ID").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
            IdGenerator::new(node_id)
        });
        let id = ORDER_ID_GEN.next_id();

        println!("✅ {} unique IDs", id);
    }
}
