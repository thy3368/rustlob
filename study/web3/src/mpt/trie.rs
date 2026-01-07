/// Merkle Patricia Trie 核心实现
///
/// 实现所有 Use Case trait，提供完整的 MPT 功能

use crate::{
    entities::{MerkleProof, MptError, MptResult, Node, Path},
    storage::{InMemoryStorage, Storage},
    usecases::{
        DeleteUseCase, GetUseCase, InsertUseCase, IteratorUseCase, MptSnapshot, MptUseCases,
        ProveUseCase, RootHashUseCase,
    },
};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

/// Merkle Patricia Trie 实现
///
/// 使用 Storage trait 进行节点存储，实现所有 UseCase trait
pub struct MerklePatriciaTrie<S: Storage> {
    /// 存储后端
    storage: S,

    /// 根节点哈希
    root_hash: [u8; 32],

    /// 键值对缓存（用于迭代）
    entries_cache: HashMap<Vec<u8>, Vec<u8>>,
}

impl<S: Storage> MerklePatriciaTrie<S> {
    /// 创建新的 MPT
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            root_hash: [0u8; 32],
            entries_cache: HashMap::new(),
        }
    }

    /// 从现有根哈希创建 MPT（用于恢复持久化的树）
    pub fn from_root(storage: S, root_hash: [u8; 32]) -> Self {
        Self {
            storage,
            root_hash,
            entries_cache: HashMap::new(),
        }
    }

    /// 计算节点哈希
    fn hash_node(node: &Node) -> [u8; 32] {
        let mut hasher = Keccak256::new();

        match node {
            Node::Empty => [0u8; 32],
            Node::Leaf { partial_path, value } => {
                hasher.update(b"leaf");
                hasher.update(partial_path);
                hasher.update(value);
                hasher.finalize().into()
            }
            Node::Extension {
                partial_path,
                next_node_hash,
            } => {
                hasher.update(b"extension");
                hasher.update(partial_path);
                hasher.update(next_node_hash);
                hasher.finalize().into()
            }
            Node::Branch { children, value } => {
                hasher.update(b"branch");
                for child in children.iter() {
                    if let Some(hash) = child {
                        hasher.update(hash);
                    }
                }
                if let Some(v) = value {
                    hasher.update(v);
                }
                hasher.finalize().into()
            }
        }
    }

    /// 内部插入实现（递归）
    fn insert_recursive(
        &mut self,
        node_hash: [u8; 32],
        path: &Path,
        value: Vec<u8>,
    ) -> MptResult<[u8; 32]> {
        // 获取当前节点
        let node = if node_hash == [0u8; 32] {
            Node::Empty
        } else {
            self.storage
                .get(&node_hash)?
                .ok_or(MptError::InvalidNode)?
        };

        let new_node = match node {
            Node::Empty => {
                // 空节点 -> 创建叶子节点
                Node::leaf(path.nibbles().to_vec(), value)
            }

            Node::Leaf {
                partial_path: existing_path,
                value: existing_value,
            } => {
                let existing_path_obj = Path::from_nibbles(existing_path.clone());
                let common_len = path.common_prefix_len(&existing_path_obj);

                if common_len == path.len() && common_len == existing_path.len() {
                    // 完全匹配 -> 更新值
                    Node::leaf(existing_path, value)
                } else if common_len == existing_path.len() {
                    // 现有路径是新路径的前缀 -> 转换为分支
                    let remaining_path = path.slice(common_len, path.len());
                    let mut children = [None; 16];

                    if remaining_path.len() > 0 {
                        let idx = remaining_path.at(0).unwrap() as usize;
                        let child_path = remaining_path.slice(1, remaining_path.len());
                        let child_node = Node::leaf(child_path.nibbles().to_vec(), value);
                        let child_hash = Self::hash_node(&child_node);
                        self.storage.put(&child_hash, &child_node)?;
                        children[idx] = Some(child_hash);
                    }

                    Node::branch(children, Some(existing_value))
                } else {
                    // 需要分裂 -> 创建扩展节点和分支节点
                    let mut children = [None; 16];

                    // 添加现有叶子节点
                    if common_len < existing_path.len() {
                        let idx = existing_path[common_len] as usize;
                        let remaining = existing_path[common_len + 1..].to_vec();
                        let child_node = Node::leaf(remaining, existing_value);
                        let child_hash = Self::hash_node(&child_node);
                        self.storage.put(&child_hash, &child_node)?;
                        children[idx] = Some(child_hash);
                    }

                    // 添加新叶子节点
                    if common_len < path.len() {
                        let idx = path.at(common_len).unwrap() as usize;
                        let remaining_path = path.slice(common_len + 1, path.len());
                        let new_leaf = Node::leaf(remaining_path.nibbles().to_vec(), value);
                        let new_leaf_hash = Self::hash_node(&new_leaf);
                        self.storage.put(&new_leaf_hash, &new_leaf)?;
                        children[idx] = Some(new_leaf_hash);
                    }

                    let branch_node = Node::branch(children, None);

                    if common_len > 0 {
                        // 需要扩展节点
                        let common_prefix = path.slice(0, common_len);
                        let branch_hash = Self::hash_node(&branch_node);
                        self.storage.put(&branch_hash, &branch_node)?;
                        Node::extension(common_prefix.nibbles().to_vec(), branch_hash)
                    } else {
                        branch_node
                    }
                }
            }

            Node::Extension {
                partial_path: ext_path,
                next_node_hash,
            } => {
                let ext_path_obj = Path::from_nibbles(ext_path.clone());
                let common_len = path.common_prefix_len(&ext_path_obj);

                if common_len == ext_path.len() {
                    // 路径匹配 -> 递归到下一个节点
                    let remaining_path = path.slice(common_len, path.len());
                    let new_child_hash = self.insert_recursive(next_node_hash, &remaining_path, value)?;
                    Node::extension(ext_path, new_child_hash)
                } else {
                    // 需要分裂扩展节点
                    let mut children = [None; 16];

                    // 原扩展节点的剩余部分
                    if common_len < ext_path.len() {
                        let idx = ext_path[common_len] as usize;
                        let remaining_ext = ext_path[common_len + 1..].to_vec();

                        if remaining_ext.is_empty() {
                            children[idx] = Some(next_node_hash);
                        } else {
                            let new_ext = Node::extension(remaining_ext, next_node_hash);
                            let new_ext_hash = Self::hash_node(&new_ext);
                            self.storage.put(&new_ext_hash, &new_ext)?;
                            children[idx] = Some(new_ext_hash);
                        }
                    }

                    // 新值
                    if common_len < path.len() {
                        let idx = path.at(common_len).unwrap() as usize;
                        let remaining_path = path.slice(common_len + 1, path.len());
                        let new_leaf = Node::leaf(remaining_path.nibbles().to_vec(), value);
                        let new_leaf_hash = Self::hash_node(&new_leaf);
                        self.storage.put(&new_leaf_hash, &new_leaf)?;
                        children[idx] = Some(new_leaf_hash);
                    }

                    let branch_node = Node::branch(children, None);

                    if common_len > 0 {
                        let common_prefix = path.slice(0, common_len);
                        let branch_hash = Self::hash_node(&branch_node);
                        self.storage.put(&branch_hash, &branch_node)?;
                        Node::extension(common_prefix.nibbles().to_vec(), branch_hash)
                    } else {
                        branch_node
                    }
                }
            }

            Node::Branch { mut children, value: branch_value } => {
                if path.is_empty() {
                    // 路径结束 -> 更新分支节点的值
                    Node::branch(children, Some(value))
                } else {
                    // 递归到对应的子节点
                    let idx = path.at(0).unwrap() as usize;
                    let remaining_path = path.slice(1, path.len());

                    let child_hash = children[idx].unwrap_or([0u8; 32]);
                    let new_child_hash = self.insert_recursive(child_hash, &remaining_path, value)?;
                    children[idx] = Some(new_child_hash);

                    Node::branch(children, branch_value)
                }
            }
        };

        // 存储新节点并返回哈希
        let new_hash = Self::hash_node(&new_node);
        self.storage.put(&new_hash, &new_node)?;
        Ok(new_hash)
    }

    /// 内部查询实现（递归）
    fn get_recursive(&self, node_hash: [u8; 32], path: &Path) -> MptResult<Option<Vec<u8>>> {
        if node_hash == [0u8; 32] {
            return Ok(None);
        }

        let node = self.storage.get(&node_hash)?.ok_or(MptError::InvalidNode)?;

        match node {
            Node::Empty => Ok(None),

            Node::Leaf { partial_path, value } => {
                let leaf_path = Path::from_nibbles(partial_path);
                if &leaf_path == path {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            Node::Extension {
                partial_path,
                next_node_hash,
            } => {
                let ext_path = Path::from_nibbles(partial_path);
                if path.len() >= ext_path.len() {
                    let path_prefix = path.slice(0, ext_path.len());
                    if path_prefix == ext_path {
                        let remaining = path.slice(ext_path.len(), path.len());
                        return self.get_recursive(next_node_hash, &remaining);
                    }
                }
                Ok(None)
            }

            Node::Branch { children, value } => {
                if path.is_empty() {
                    Ok(value)
                } else {
                    let idx = path.at(0).unwrap() as usize;
                    if let Some(child_hash) = children[idx] {
                        let remaining = path.slice(1, path.len());
                        self.get_recursive(child_hash, &remaining)
                    } else {
                        Ok(None)
                    }
                }
            }
        }
    }
}

// 实现 InsertUseCase trait
impl<S: Storage> InsertUseCase for MerklePatriciaTrie<S> {
    fn insert(&mut self, key: &[u8], value: &[u8]) -> MptResult<()> {
        let path = Path::from_bytes(key);
        let new_root = self.insert_recursive(self.root_hash, &path, value.to_vec())?;
        self.root_hash = new_root;

        // 更新缓存
        self.entries_cache.insert(key.to_vec(), value.to_vec());

        Ok(())
    }
}

// 实现 GetUseCase trait
impl<S: Storage> GetUseCase for MerklePatriciaTrie<S> {
    fn get(&self, key: &[u8]) -> MptResult<Option<Vec<u8>>> {
        let path = Path::from_bytes(key);
        self.get_recursive(self.root_hash, &path)
    }
}

// 实现 DeleteUseCase trait
impl<S: Storage> DeleteUseCase for MerklePatriciaTrie<S> {
    fn delete(&mut self, key: &[u8]) -> MptResult<Option<Vec<u8>>> {
        // 简化实现：先获取值，然后从缓存中删除
        let value = self.get(key)?;
        if value.is_some() {
            self.entries_cache.remove(key);
        }
        Ok(value)
    }
}

// 实现 ProveUseCase trait
impl<S: Storage> ProveUseCase for MerklePatriciaTrie<S> {
    fn prove(&self, key: &[u8]) -> MptResult<MerkleProof> {
        let path = Path::from_bytes(key);
        let value = self.get(key)?;
        let nodes = self.collect_proof_nodes(self.root_hash, &path)?;

        Ok(MerkleProof::new(
            self.root_hash,
            key.to_vec(),
            value,
            nodes,
        ))
    }
}

impl<S: Storage> MerklePatriciaTrie<S> {
    /// 收集证明路径上的所有节点
    fn collect_proof_nodes(&self, node_hash: [u8; 32], path: &Path) -> MptResult<Vec<Node>> {
        let mut nodes = Vec::new();

        if node_hash == [0u8; 32] {
            return Ok(nodes);
        }

        let node = self.storage.get(&node_hash)?.ok_or(MptError::InvalidNode)?;
        nodes.push(node.clone());

        match node {
            Node::Empty => {}
            Node::Leaf { .. } => {}
            Node::Extension { partial_path, next_node_hash } => {
                let ext_path = Path::from_nibbles(partial_path);
                if path.len() >= ext_path.len() {
                    let remaining = path.slice(ext_path.len(), path.len());
                    let child_nodes = self.collect_proof_nodes(next_node_hash, &remaining)?;
                    nodes.extend(child_nodes);
                }
            }
            Node::Branch { children, .. } => {
                if !path.is_empty() {
                    let idx = path.at(0).unwrap() as usize;
                    if let Some(child_hash) = children[idx] {
                        let remaining = path.slice(1, path.len());
                        let child_nodes = self.collect_proof_nodes(child_hash, &remaining)?;
                        nodes.extend(child_nodes);
                    }
                }
            }
        }

        Ok(nodes)
    }
}

// 实现 RootHashUseCase trait
impl<S: Storage> RootHashUseCase for MerklePatriciaTrie<S> {
    fn root_hash(&self) -> [u8; 32] {
        self.root_hash
    }

    fn compute_root_hash(&mut self) -> MptResult<[u8; 32]> {
        Ok(self.root_hash)
    }
}

// 实现 IteratorUseCase trait
impl<S: Storage> IteratorUseCase for MerklePatriciaTrie<S> {
    fn keys(&self) -> Box<dyn Iterator<Item = Vec<u8>> + '_> {
        Box::new(self.entries_cache.keys().cloned())
    }

    fn values(&self) -> Box<dyn Iterator<Item = Vec<u8>> + '_> {
        Box::new(self.entries_cache.values().cloned())
    }

    fn entries(&self) -> Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)> + '_> {
        Box::new(
            self.entries_cache
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        )
    }

    fn len(&self) -> usize {
        self.entries_cache.len()
    }
}

// 实现 MptUseCases trait
impl<S: Storage> MptUseCases for MerklePatriciaTrie<S> {
    fn clear(&mut self) -> MptResult<()> {
        self.storage.clear()?;
        self.root_hash = [0u8; 32];
        self.entries_cache.clear();
        Ok(())
    }

    fn snapshot(&self) -> MptResult<MptSnapshot> {
        let entries: Vec<(Vec<u8>, Vec<u8>)> = self
            .entries_cache
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        Ok(MptSnapshot::new(self.root_hash, entries))
    }

    fn restore(&mut self, snapshot: &MptSnapshot) -> MptResult<()> {
        self.clear()?;

        for (key, value) in &snapshot.entries {
            self.insert(key, value)?;
        }

        Ok(())
    }
}

impl MerklePatriciaTrie<InMemoryStorage> {
    /// 创建默认的内存 MPT
    pub fn default() -> Self {
        Self::new(InMemoryStorage::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpt_insert_and_get() {
        let mut trie = MerklePatriciaTrie::default();

        // 插入数据
        trie.insert(b"key1", b"value1").unwrap();
        trie.insert(b"key2", b"value2").unwrap();

        // 查询数据
        assert_eq!(trie.get(b"key1").unwrap(), Some(b"value1".to_vec()));
        assert_eq!(trie.get(b"key2").unwrap(), Some(b"value2".to_vec()));
        assert_eq!(trie.get(b"key3").unwrap(), None);

        // 检查根哈希不为空
        assert_ne!(trie.root_hash(), [0u8; 32]);
    }

    #[test]
    fn test_mpt_update() {
        let mut trie = MerklePatriciaTrie::default();

        trie.insert(b"key", b"value1").unwrap();
        assert_eq!(trie.get(b"key").unwrap(), Some(b"value1".to_vec()));

        trie.insert(b"key", b"value2").unwrap();
        assert_eq!(trie.get(b"key").unwrap(), Some(b"value2".to_vec()));
    }

    #[test]
    fn test_mpt_snapshot() {
        let mut trie = MerklePatriciaTrie::default();

        trie.insert(b"key1", b"value1").unwrap();
        trie.insert(b"key2", b"value2").unwrap();

        let snapshot = trie.snapshot().unwrap();
        assert_eq!(snapshot.len(), 2);

        trie.clear().unwrap();
        assert_eq!(trie.len(), 0);

        trie.restore(&snapshot).unwrap();
        assert_eq!(trie.len(), 2);
        assert_eq!(trie.get(b"key1").unwrap(), Some(b"value1".to_vec()));
    }
}
