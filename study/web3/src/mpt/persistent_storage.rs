/// 持久化存储实现
///
/// 基于 geth 的区块持久化机制，使用嵌入式数据库实现
///
/// Geth 存储架构参考：
/// - ChainDB: 存储区块、交易、收据
/// - StateDB: 存储账户状态
/// - Key-Value 存储：LevelDB/RocksDB
///
/// 本实现使用文件系统作为简单的持久化后端，
/// 生产环境可替换为 RocksDB 或 LMDB

use crate::entities::{MptError, MptResult, Node};
use crate::storage::Storage;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// 持久化存储
///
/// 实现基于文件系统的持久化存储，模拟 geth 的 LevelDB 存储
pub struct PersistentStorage {
    /// 数据目录
    data_dir: PathBuf,

    /// 内存缓存（提升性能）
    cache: HashMap<[u8; 32], Node>,

    /// 缓存大小限制
    cache_size_limit: usize,
}

impl PersistentStorage {
    /// 创建新的持久化存储
    ///
    /// # 参数
    /// - `data_dir`: 数据存储目录
    /// - `cache_size`: 缓存大小（条目数量）
    pub fn new(data_dir: impl AsRef<Path>, cache_size: usize) -> MptResult<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();

        // 创建数据目录
        if !data_dir.exists() {
            fs::create_dir_all(&data_dir)
                .map_err(|e| MptError::StorageError(format!("Failed to create data dir: {}", e)))?;
        }

        Ok(Self {
            data_dir,
            cache: HashMap::new(),
            cache_size_limit: cache_size,
        })
    }

    /// 获取节点文件路径
    fn node_path(&self, hash: &[u8; 32]) -> PathBuf {
        // 使用哈希的前 2 个字节作为子目录（类似 git 对象存储）
        let dir_name = format!("{:02x}{:02x}", hash[0], hash[1]);
        let file_name = hex::encode(&hash[2..]);

        let dir_path = self.data_dir.join("nodes").join(dir_name);

        // 确保子目录存在
        let _ = fs::create_dir_all(&dir_path);

        dir_path.join(file_name)
    }

    /// 序列化节点
    fn serialize_node(node: &Node) -> Vec<u8> {
        // 简化的序列化格式
        // 生产环境应使用 RLP 编码（以太坊标准）或 Bincode
        match node {
            Node::Empty => vec![0],
            Node::Leaf { partial_path, value } => {
                let mut data = vec![1]; // 类型标记
                data.extend_from_slice(&(partial_path.len() as u32).to_le_bytes());
                data.extend_from_slice(partial_path);
                data.extend_from_slice(&(value.len() as u32).to_le_bytes());
                data.extend_from_slice(value);
                data
            }
            Node::Extension {
                partial_path,
                next_node_hash,
            } => {
                let mut data = vec![2]; // 类型标记
                data.extend_from_slice(&(partial_path.len() as u32).to_le_bytes());
                data.extend_from_slice(partial_path);
                data.extend_from_slice(next_node_hash);
                data
            }
            Node::Branch { children, value } => {
                let mut data = vec![3]; // 类型标记

                // 序列化 16 个子节点
                for child in children.iter() {
                    if let Some(hash) = child {
                        data.push(1); // 有子节点
                        data.extend_from_slice(hash);
                    } else {
                        data.push(0); // 无子节点
                    }
                }

                // 序列化值
                if let Some(v) = value {
                    data.push(1); // 有值
                    data.extend_from_slice(&(v.len() as u32).to_le_bytes());
                    data.extend_from_slice(v);
                } else {
                    data.push(0); // 无值
                }

                data
            }
        }
    }

    /// 反序列化节点
    fn deserialize_node(data: &[u8]) -> MptResult<Node> {
        if data.is_empty() {
            return Err(MptError::DecodingError("Empty data".to_string()));
        }

        let node_type = data[0];
        let mut offset = 1;

        match node_type {
            0 => Ok(Node::Empty),
            1 => {
                // Leaf
                if data.len() < 5 {
                    return Err(MptError::DecodingError("Invalid leaf data".to_string()));
                }

                let path_len = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;

                let partial_path = data[offset..offset + path_len].to_vec();
                offset += path_len;

                let value_len = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;

                let value = data[offset..offset + value_len].to_vec();

                Ok(Node::Leaf {
                    partial_path,
                    value,
                })
            }
            2 => {
                // Extension
                if data.len() < 37 {
                    return Err(MptError::DecodingError("Invalid extension data".to_string()));
                }

                let path_len = u32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]) as usize;
                offset += 4;

                let partial_path = data[offset..offset + path_len].to_vec();
                offset += path_len;

                let mut next_node_hash = [0u8; 32];
                next_node_hash.copy_from_slice(&data[offset..offset + 32]);

                Ok(Node::Extension {
                    partial_path,
                    next_node_hash,
                })
            }
            3 => {
                // Branch
                let mut children = [None; 16];

                for i in 0..16 {
                    if data[offset] == 1 {
                        offset += 1;
                        let mut hash = [0u8; 32];
                        hash.copy_from_slice(&data[offset..offset + 32]);
                        children[i] = Some(hash);
                        offset += 32;
                    } else {
                        offset += 1;
                    }
                }

                let value = if data[offset] == 1 {
                    offset += 1;
                    let value_len = u32::from_le_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]) as usize;
                    offset += 4;

                    Some(data[offset..offset + value_len].to_vec())
                } else {
                    None
                };

                Ok(Node::Branch { children, value })
            }
            _ => Err(MptError::DecodingError(format!(
                "Unknown node type: {}",
                node_type
            ))),
        }
    }

    /// 驱逐缓存中的旧条目
    fn evict_cache(&mut self) {
        if self.cache.len() > self.cache_size_limit {
            // 简单的 LRU：删除一半的缓存
            let to_remove = self.cache.len() / 2;
            let keys: Vec<[u8; 32]> = self.cache.keys().take(to_remove).copied().collect();

            for key in keys {
                self.cache.remove(&key);
            }
        }
    }

    /// 获取存储统计信息
    pub fn stats(&self) -> StorageStats {
        let nodes_dir = self.data_dir.join("nodes");
        let total_files = Self::count_files(&nodes_dir);
        let total_size = Self::calculate_dir_size(&nodes_dir);

        StorageStats {
            cache_size: self.cache.len(),
            cache_limit: self.cache_size_limit,
            total_nodes: total_files,
            disk_usage_bytes: total_size,
        }
    }

    /// 计算目录大小
    fn calculate_dir_size(path: &Path) -> u64 {
        if !path.exists() {
            return 0;
        }

        let mut total = 0u64;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        total += metadata.len();
                    } else if metadata.is_dir() {
                        total += Self::calculate_dir_size(&entry.path());
                    }
                }
            }
        }

        total
    }

    /// 计算文件数量
    fn count_files(path: &Path) -> usize {
        if !path.exists() {
            return 0;
        }

        let mut count = 0;

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        count += 1;
                    } else if metadata.is_dir() {
                        count += Self::count_files(&entry.path());
                    }
                }
            }
        }

        count
    }

    /// 清空所有数据
    pub fn clear_all(&mut self) -> MptResult<()> {
        self.cache.clear();

        let nodes_dir = self.data_dir.join("nodes");
        if nodes_dir.exists() {
            fs::remove_dir_all(&nodes_dir)
                .map_err(|e| MptError::StorageError(format!("Failed to clear storage: {}", e)))?;
            fs::create_dir_all(&nodes_dir)
                .map_err(|e| MptError::StorageError(format!("Failed to recreate dir: {}", e)))?;
        }

        Ok(())
    }
}

impl Storage for PersistentStorage {
    fn put(&mut self, hash: &[u8; 32], node: &Node) -> MptResult<()> {
        // 写入缓存
        self.cache.insert(*hash, node.clone());
        self.evict_cache();

        // 写入磁盘
        let path = self.node_path(hash);
        let data = Self::serialize_node(node);

        let mut file = File::create(&path)
            .map_err(|e| MptError::StorageError(format!("Failed to create file: {}", e)))?;

        file.write_all(&data)
            .map_err(|e| MptError::StorageError(format!("Failed to write file: {}", e)))?;

        Ok(())
    }

    fn get(&self, hash: &[u8; 32]) -> MptResult<Option<Node>> {
        // 先查缓存
        if let Some(node) = self.cache.get(hash) {
            return Ok(Some(node.clone()));
        }

        // 从磁盘读取
        let path = self.node_path(hash);

        if !path.exists() {
            return Ok(None);
        }

        let mut file = File::open(&path)
            .map_err(|e| MptError::StorageError(format!("Failed to open file: {}", e)))?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)
            .map_err(|e| MptError::StorageError(format!("Failed to read file: {}", e)))?;

        let node = Self::deserialize_node(&data)?;

        Ok(Some(node))
    }

    fn clear(&mut self) -> MptResult<()> {
        self.clear_all()
    }

    fn delete(&mut self, hash: &[u8; 32]) -> MptResult<bool> {
        // 从缓存删除
        let in_cache = self.cache.remove(hash).is_some();

        // 从磁盘删除
        let path = self.node_path(hash);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| MptError::StorageError(format!("Failed to delete file: {}", e)))?;
            Ok(true)
        } else {
            Ok(in_cache)
        }
    }

    fn len(&self) -> usize {
        self.stats().total_nodes
    }
}

/// 存储统计信息
#[derive(Debug, Clone)]
pub struct StorageStats {
    /// 缓存中的节点数量
    pub cache_size: usize,

    /// 缓存大小限制
    pub cache_limit: usize,

    /// 磁盘上的总节点数量
    pub total_nodes: usize,

    /// 磁盘使用量（字节）
    pub disk_usage_bytes: u64,
}

impl StorageStats {
    /// 获取磁盘使用量（MB）
    pub fn disk_usage_mb(&self) -> f64 {
        self.disk_usage_bytes as f64 / 1024.0 / 1024.0
    }

    /// 获取缓存命中率（需要外部统计）
    pub fn cache_usage_ratio(&self) -> f64 {
        if self.cache_limit == 0 {
            0.0
        } else {
            self.cache_size as f64 / self.cache_limit as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_persistent_storage_basic() {
        let temp_dir = tempdir().unwrap();
        let mut storage = PersistentStorage::new(temp_dir.path(), 100).unwrap();

        // 创建测试节点
        let node = Node::leaf(vec![1, 2, 3], vec![4, 5, 6]);
        let hash = [1u8; 32];

        // 存储节点
        storage.put(&hash, &node).unwrap();

        // 读取节点
        let retrieved = storage.get(&hash).unwrap();
        assert_eq!(retrieved, Some(node));
    }

    #[test]
    fn test_persistent_storage_persistence() {
        let temp_dir = tempdir().unwrap();

        let hash = [2u8; 32];
        let node = Node::leaf(vec![7, 8, 9], vec![10, 11, 12]);

        // 第一个存储实例
        {
            let mut storage = PersistentStorage::new(temp_dir.path(), 100).unwrap();
            storage.put(&hash, &node).unwrap();
        }

        // 第二个存储实例（应该能读取到数据）
        {
            let storage = PersistentStorage::new(temp_dir.path(), 100).unwrap();
            let retrieved = storage.get(&hash).unwrap();
            assert_eq!(retrieved, Some(node));
        }
    }

    #[test]
    fn test_storage_stats() {
        let temp_dir = tempdir().unwrap();
        let mut storage = PersistentStorage::new(temp_dir.path(), 100).unwrap();

        // 插入一些节点
        for i in 0..10 {
            let mut hash = [0u8; 32];
            hash[0] = i;
            let node = Node::leaf(vec![i], vec![i, i + 1]);
            storage.put(&hash, &node).unwrap();
        }

        let stats = storage.stats();
        assert_eq!(stats.total_nodes, 10);
        assert!(stats.disk_usage_bytes > 0);
    }
}
