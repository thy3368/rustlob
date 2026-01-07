/// MPT 存储层
///
/// 提供存储抽象，遵循依赖倒置原则
/// 核心逻辑依赖抽象接口，具体实现可替换

use crate::entities::{MptError, MptResult, Node};
use std::collections::HashMap;

/// 存储接口 trait
///
/// 定义 MPT 节点存储的抽象接口
/// 实现者可以选择不同的存储后端（内存、磁盘、数据库等）
pub trait Storage: Send + Sync {
    /// 存储节点
    ///
    /// # 参数
    /// - `hash`: 节点的哈希（作为键）
    /// - `node`: 要存储的节点
    ///
    /// # 返回
    /// - `Ok(())`: 存储成功
    /// - `Err(MptError)`: 存储失败
    fn put(&mut self, hash: &[u8; 32], node: &Node) -> MptResult<()>;

    /// 获取节点
    ///
    /// # 参数
    /// - `hash`: 节点的哈希
    ///
    /// # 返回
    /// - `Ok(Some(node))`: 找到节点
    /// - `Ok(None)`: 节点不存在
    /// - `Err(MptError)`: 查询失败
    fn get(&self, hash: &[u8; 32]) -> MptResult<Option<Node>>;

    /// 删除节点
    ///
    /// # 参数
    /// - `hash`: 要删除的节点哈希
    ///
    /// # 返回
    /// - `Ok(true)`: 删除成功
    /// - `Ok(false)`: 节点不存在
    /// - `Err(MptError)`: 删除失败
    fn delete(&mut self, hash: &[u8; 32]) -> MptResult<bool>;

    /// 检查节点是否存在
    ///
    /// # 参数
    /// - `hash`: 节点哈希
    ///
    /// # 返回
    /// - `Ok(true)`: 节点存在
    /// - `Ok(false)`: 节点不存在
    /// - `Err(MptError)`: 检查失败
    fn contains(&self, hash: &[u8; 32]) -> MptResult<bool> {
        self.get(hash).map(|opt| opt.is_some())
    }

    /// 批量存储节点
    ///
    /// # 参数
    /// - `nodes`: 要存储的节点数组 (hash, node)
    ///
    /// # 返回
    /// - `Ok(count)`: 成功存储的节点数量
    /// - `Err(MptError)`: 批量存储失败
    fn batch_put(&mut self, nodes: &[([u8; 32], Node)]) -> MptResult<usize> {
        let mut count = 0;
        for (hash, node) in nodes {
            self.put(hash, node)?;
            count += 1;
        }
        Ok(count)
    }

    /// 批量获取节点
    ///
    /// # 参数
    /// - `hashes`: 节点哈希数组
    ///
    /// # 返回
    /// - `Ok(nodes)`: 获取的节点数组（None 表示不存在）
    /// - `Err(MptError)`: 批量获取失败
    fn batch_get(&self, hashes: &[[u8; 32]]) -> MptResult<Vec<Option<Node>>> {
        hashes.iter().map(|hash| self.get(hash)).collect()
    }

    /// 清空所有存储
    ///
    /// # 返回
    /// - `Ok(())`: 清空成功
    /// - `Err(MptError)`: 清空失败
    fn clear(&mut self) -> MptResult<()>;

    /// 获取存储的节点数量
    ///
    /// # 返回
    /// - 存储的节点总数
    fn len(&self) -> usize;

    /// 检查存储是否为空
    ///
    /// # 返回
    /// - true: 存储为空
    /// - false: 存储不为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// 内存存储实现
///
/// 使用 HashMap 在内存中存储节点
/// 适合测试和小规模应用
#[derive(Debug, Clone)]
pub struct InMemoryStorage {
    nodes: HashMap<[u8; 32], Node>,
}

impl InMemoryStorage {
    /// 创建新的内存存储
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// 使用指定容量创建内存存储
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: HashMap::with_capacity(capacity),
        }
    }

    /// 获取内存使用量（估算，字节）
    pub fn memory_usage(&self) -> usize {
        // 粗略估算：每个节点约 100 字节
        self.nodes.len() * 100
    }
}

impl Default for InMemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for InMemoryStorage {
    fn put(&mut self, hash: &[u8; 32], node: &Node) -> MptResult<()> {
        self.nodes.insert(*hash, node.clone());
        Ok(())
    }

    fn get(&self, hash: &[u8; 32]) -> MptResult<Option<Node>> {
        Ok(self.nodes.get(hash).cloned())
    }

    fn delete(&mut self, hash: &[u8; 32]) -> MptResult<bool> {
        Ok(self.nodes.remove(hash).is_some())
    }

    fn clear(&mut self) -> MptResult<()> {
        self.nodes.clear();
        Ok(())
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }
}

/// 缓存存储装饰器
///
/// 为任何存储实现添加缓存层，提高读取性能
pub struct CachedStorage<S: Storage> {
    /// 底层存储
    backend: S,

    /// 缓存
    cache: HashMap<[u8; 32], Node>,

    /// 缓存大小限制
    cache_size: usize,
}

impl<S: Storage> CachedStorage<S> {
    /// 创建缓存存储
    ///
    /// # 参数
    /// - `backend`: 底层存储实现
    /// - `cache_size`: 缓存大小限制
    pub fn new(backend: S, cache_size: usize) -> Self {
        Self {
            backend,
            cache: HashMap::new(),
            cache_size,
        }
    }

    /// 清空缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// 获取缓存命中率统计
    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            capacity: self.cache_size,
        }
    }
}

impl<S: Storage> Storage for CachedStorage<S> {
    fn put(&mut self, hash: &[u8; 32], node: &Node) -> MptResult<()> {
        // 更新缓存
        if self.cache.len() < self.cache_size {
            self.cache.insert(*hash, node.clone());
        }

        // 写入底层存储
        self.backend.put(hash, node)
    }

    fn get(&self, hash: &[u8; 32]) -> MptResult<Option<Node>> {
        // 先查缓存
        if let Some(node) = self.cache.get(hash) {
            return Ok(Some(node.clone()));
        }

        // 缓存未命中，查底层存储
        self.backend.get(hash)
    }

    fn delete(&mut self, hash: &[u8; 32]) -> MptResult<bool> {
        // 删除缓存
        self.cache.remove(hash);

        // 删除底层存储
        self.backend.delete(hash)
    }

    fn clear(&mut self) -> MptResult<()> {
        self.cache.clear();
        self.backend.clear()
    }

    fn len(&self) -> usize {
        self.backend.len()
    }
}

/// 缓存统计信息
#[derive(Debug, Clone, Copy)]
pub struct CacheStats {
    /// 当前缓存大小
    pub size: usize,

    /// 缓存容量
    pub capacity: usize,
}

impl CacheStats {
    /// 计算缓存使用率
    pub fn usage_ratio(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            self.size as f64 / self.capacity as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_storage() {
        let mut storage = InMemoryStorage::new();

        let hash = [1u8; 32];
        let node = Node::leaf(vec![1, 2, 3], vec![4, 5, 6]);

        // 测试存储
        storage.put(&hash, &node).unwrap();
        assert_eq!(storage.len(), 1);
        assert!(!storage.is_empty());

        // 测试获取
        let retrieved = storage.get(&hash).unwrap();
        assert_eq!(retrieved, Some(node.clone()));

        // 测试删除
        let deleted = storage.delete(&hash).unwrap();
        assert!(deleted);
        assert_eq!(storage.len(), 0);
        assert!(storage.is_empty());
    }

    #[test]
    fn test_cached_storage() {
        let backend = InMemoryStorage::new();
        let mut storage = CachedStorage::new(backend, 10);

        let hash = [2u8; 32];
        let node = Node::leaf(vec![1, 2], vec![3, 4]);

        // 测试缓存写入
        storage.put(&hash, &node).unwrap();

        // 测试缓存读取
        let retrieved = storage.get(&hash).unwrap();
        assert_eq!(retrieved, Some(node));

        // 检查缓存统计
        let stats = storage.cache_stats();
        assert_eq!(stats.size, 1);
        assert_eq!(stats.capacity, 10);
        assert!(stats.usage_ratio() > 0.0 && stats.usage_ratio() < 1.0);
    }
}
