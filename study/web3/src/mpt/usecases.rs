/// MPT Use Cases 层
///
/// 遵循 Clean Architecture 原则，使用 trait 定义业务用例接口
/// 每个 trait 代表一个独立的用例，确保单一职责原则

use crate::entities::{MerkleProof, MptResult};

/// 插入用例 - 向 MPT 中插入键值对
///
/// # Use Case
/// 作为用户，我希望能够向 Merkle Patricia Trie 中插入键值对，
/// 以便存储状态数据并自动更新根哈希
pub trait InsertUseCase {
    /// 插入键值对
    ///
    /// # 参数
    /// - `key`: 键（字节数组）
    /// - `value`: 值（字节数组）
    ///
    /// # 返回
    /// - `Ok(())`: 插入成功
    /// - `Err(MptError)`: 插入失败
    ///
    /// # 副作用
    /// - 修改树的内部结构
    /// - 更新根哈希
    /// - 可能触发节点的分裂或合并
    fn insert(&mut self, key: &[u8], value: &[u8]) -> MptResult<()>;

    /// 批量插入键值对
    ///
    /// # 参数
    /// - `entries`: 键值对数组
    ///
    /// # 返回
    /// - `Ok(usize)`: 成功插入的键值对数量
    /// - `Err(MptError)`: 批量插入失败
    fn batch_insert(&mut self, entries: &[(&[u8], &[u8])]) -> MptResult<usize> {
        let mut count = 0;
        for (key, value) in entries {
            self.insert(key, value)?;
            count += 1;
        }
        Ok(count)
    }
}

/// 查询用例 - 从 MPT 中查询值
///
/// # Use Case
/// 作为用户，我希望能够通过键从 Merkle Patricia Trie 中查询对应的值，
/// 以便读取已存储的状态数据
pub trait GetUseCase {
    /// 查询键对应的值
    ///
    /// # 参数
    /// - `key`: 键（字节数组）
    ///
    /// # 返回
    /// - `Ok(Some(value))`: 找到键对应的值
    /// - `Ok(None)`: 键不存在
    /// - `Err(MptError)`: 查询失败
    fn get(&self, key: &[u8]) -> MptResult<Option<Vec<u8>>>;

    /// 检查键是否存在
    ///
    /// # 参数
    /// - `key`: 键（字节数组）
    ///
    /// # 返回
    /// - `Ok(true)`: 键存在
    /// - `Ok(false)`: 键不存在
    /// - `Err(MptError)`: 检查失败
    fn contains(&self, key: &[u8]) -> MptResult<bool> {
        self.get(key).map(|opt| opt.is_some())
    }

    /// 批量查询
    ///
    /// # 参数
    /// - `keys`: 键数组
    ///
    /// # 返回
    /// - `Ok(values)`: 查询结果数组（None 表示键不存在）
    /// - `Err(MptError)`: 批量查询失败
    fn batch_get(&self, keys: &[&[u8]]) -> MptResult<Vec<Option<Vec<u8>>>> {
        keys.iter().map(|key| self.get(key)).collect()
    }
}

/// 删除用例 - 从 MPT 中删除键值对
///
/// # Use Case
/// 作为用户，我希望能够从 Merkle Patricia Trie 中删除指定的键值对，
/// 以便移除不再需要的状态数据并更新根哈希
pub trait DeleteUseCase {
    /// 删除键值对
    ///
    /// # 参数
    /// - `key`: 要删除的键
    ///
    /// # 返回
    /// - `Ok(Some(value))`: 删除成功，返回被删除的值
    /// - `Ok(None)`: 键不存在
    /// - `Err(MptError)`: 删除失败
    ///
    /// # 副作用
    /// - 修改树的内部结构
    /// - 更新根哈希
    /// - 可能触发节点的合并或删除
    fn delete(&mut self, key: &[u8]) -> MptResult<Option<Vec<u8>>>;

    /// 批量删除键值对
    ///
    /// # 参数
    /// - `keys`: 要删除的键数组
    ///
    /// # 返回
    /// - `Ok(count)`: 成功删除的键值对数量
    /// - `Err(MptError)`: 批量删除失败
    fn batch_delete(&mut self, keys: &[&[u8]]) -> MptResult<usize> {
        let mut count = 0;
        for key in keys {
            if self.delete(key)?.is_some() {
                count += 1;
            }
        }
        Ok(count)
    }
}

/// 证明用例 - 生成和验证 Merkle 证明
///
/// # Use Case
/// 作为用户，我希望能够生成 Merkle 证明来证明某个键值对存在于 MPT 中，
/// 以便在不暴露完整树的情况下验证数据完整性
pub trait ProveUseCase {
    /// 生成 Merkle 证明
    ///
    /// # 参数
    /// - `key`: 要证明的键
    ///
    /// # 返回
    /// - `Ok(proof)`: 生成的 Merkle 证明
    /// - `Err(MptError)`: 证明生成失败
    ///
    /// # 证明内容
    /// - 根哈希
    /// - 键
    /// - 值（如果存在）
    /// - 从根到叶子的路径上的所有节点
    fn prove(&self, key: &[u8]) -> MptResult<MerkleProof>;

    /// 验证 Merkle 证明
    ///
    /// # 参数
    /// - `proof`: 要验证的 Merkle 证明
    ///
    /// # 返回
    /// - `Ok(true)`: 证明有效
    /// - `Ok(false)`: 证明无效
    /// - `Err(MptError)`: 验证失败
    fn verify_proof(&self, proof: &MerkleProof) -> MptResult<bool> {
        proof.verify()
    }
}

/// 根哈希用例 - 获取和验证根哈希
///
/// # Use Case
/// 作为用户，我希望能够获取 MPT 的根哈希，
/// 以便验证树的完整性和在区块链中使用
pub trait RootHashUseCase {
    /// 获取根哈希
    ///
    /// # 返回
    /// - 当前 MPT 的根哈希（32字节）
    fn root_hash(&self) -> [u8; 32];

    /// 计算并更新根哈希
    ///
    /// # 返回
    /// - `Ok(hash)`: 新计算的根哈希
    /// - `Err(MptError)`: 计算失败
    fn compute_root_hash(&mut self) -> MptResult<[u8; 32]>;
}

/// 迭代器用例 - 遍历 MPT 中的所有键值对
///
/// # Use Case
/// 作为用户，我希望能够遍历 Merkle Patricia Trie 中的所有键值对，
/// 以便进行批量操作或数据导出
pub trait IteratorUseCase {
    /// 返回所有键的迭代器
    ///
    /// # 返回
    /// - 键的迭代器
    fn keys(&self) -> Box<dyn Iterator<Item = Vec<u8>> + '_>;

    /// 返回所有值的迭代器
    ///
    /// # 返回
    /// - 值的迭代器
    fn values(&self) -> Box<dyn Iterator<Item = Vec<u8>> + '_>;

    /// 返回所有键值对的迭代器
    ///
    /// # 返回
    /// - 键值对的迭代器
    fn entries(&self) -> Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + '_>;

    /// 获取键值对数量
    ///
    /// # 返回
    /// - MPT 中的键值对总数
    fn len(&self) -> usize;

    /// 检查 MPT 是否为空
    ///
    /// # 返回
    /// - true: MPT 为空
    /// - false: MPT 不为空
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// MPT 完整用例接口
///
/// 组合所有用例 trait，提供完整的 MPT 功能
pub trait MptUseCases:
    InsertUseCase + GetUseCase + DeleteUseCase + ProveUseCase + RootHashUseCase + IteratorUseCase
{
    /// 清空 MPT
    ///
    /// # 副作用
    /// - 删除所有键值对
    /// - 重置根哈希
    fn clear(&mut self) -> MptResult<()>;

    /// 创建 MPT 的快照
    ///
    /// # 返回
    /// - `Ok(snapshot)`: 当前状态的快照
    /// - `Err(MptError)`: 快照创建失败
    fn snapshot(&self) -> MptResult<MptSnapshot>;

    /// 从快照恢复 MPT
    ///
    /// # 参数
    /// - `snapshot`: 要恢复的快照
    ///
    /// # 返回
    /// - `Ok(())`: 恢复成功
    /// - `Err(MptError)`: 恢复失败
    fn restore(&mut self, snapshot: &MptSnapshot) -> MptResult<()>;
}

/// MPT 快照
///
/// 用于保存和恢复 MPT 的状态
#[derive(Debug, Clone)]
pub struct MptSnapshot {
    /// 根哈希
    pub root_hash: [u8; 32],

    /// 所有键值对
    pub entries: Vec<(Vec<u8>, Vec<u8>)>,

    /// 快照时间戳（纳秒）
    pub timestamp: u128,
}

impl MptSnapshot {
    /// 创建新快照
    pub fn new(root_hash: [u8; 32], entries: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        Self {
            root_hash,
            entries,
            timestamp,
        }
    }

    /// 获取快照大小（字节）
    pub fn size_bytes(&self) -> usize {
        32 + // root_hash
        self.entries.iter().map(|(k, v)| k.len() + v.len()).sum::<usize>() +
        16 // timestamp
    }

    /// 获取键值对数量
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 检查快照是否为空
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpt_snapshot() {
        let entries = vec![
            (vec![1, 2, 3], vec![4, 5, 6]),
            (vec![7, 8, 9], vec![10, 11, 12]),
        ];

        let snapshot = MptSnapshot::new([0u8; 32], entries.clone());

        assert_eq!(snapshot.len(), 2);
        assert!(!snapshot.is_empty());
        assert!(snapshot.size_bytes() > 0);
    }
}
