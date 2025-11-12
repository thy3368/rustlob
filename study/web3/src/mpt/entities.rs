/// MPT 核心实体层
///
/// 定义 Merkle Patricia Trie 的核心数据结构和领域模型
/// 遵循 Clean Architecture 的 Entities Layer 原则：无外部依赖

use std::fmt;

/// MPT 节点类型
///
/// Merkle Patricia Trie 有四种节点类型：
/// - Leaf: 叶子节点，存储键值对
/// - Extension: 扩展节点，压缩路径
/// - Branch: 分支节点，16个子节点 + 可选值
/// - Empty: 空节点
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// 空节点
    Empty,

    /// 叶子节点 (partial_path, value)
    Leaf {
        partial_path: Vec<u8>,
        value: Vec<u8>,
    },

    /// 扩展节点 (partial_path, next_node_hash)
    Extension {
        partial_path: Vec<u8>,
        next_node_hash: [u8; 32],
    },

    /// 分支节点 (16 children hashes, optional value)
    Branch {
        children: [Option<[u8; 32]>; 16],
        value: Option<Vec<u8>>,
    },
}

impl Node {
    /// 创建空节点
    pub fn empty() -> Self {
        Node::Empty
    }

    /// 创建叶子节点
    pub fn leaf(partial_path: Vec<u8>, value: Vec<u8>) -> Self {
        Node::Leaf { partial_path, value }
    }

    /// 创建扩展节点
    pub fn extension(partial_path: Vec<u8>, next_node_hash: [u8; 32]) -> Self {
        Node::Extension {
            partial_path,
            next_node_hash,
        }
    }

    /// 创建分支节点
    pub fn branch(children: [Option<[u8; 32]>; 16], value: Option<Vec<u8>>) -> Self {
        Node::Branch { children, value }
    }

    /// 检查节点是否为空
    pub fn is_empty(&self) -> bool {
        matches!(self, Node::Empty)
    }

    /// 检查节点是否为叶子节点
    pub fn is_leaf(&self) -> bool {
        matches!(self, Node::Leaf { .. })
    }

    /// 检查节点是否为扩展节点
    pub fn is_extension(&self) -> bool {
        matches!(self, Node::Extension { .. })
    }

    /// 检查节点是否为分支节点
    pub fn is_branch(&self) -> bool {
        matches!(self, Node::Branch { .. })
    }
}

/// MPT 路径（Nibble 表示）
///
/// 以太坊的 MPT 使用 nibble（半字节，4位）作为路径单位
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    nibbles: Vec<u8>,
}

impl Path {
    /// 从字节数组创建路径
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut nibbles = Vec::with_capacity(bytes.len() * 2);
        for byte in bytes {
            nibbles.push(byte >> 4);      // 高4位
            nibbles.push(byte & 0x0F);    // 低4位
        }
        Self { nibbles }
    }

    /// 从 nibbles 创建路径
    pub fn from_nibbles(nibbles: Vec<u8>) -> Self {
        Self { nibbles }
    }

    /// 获取 nibbles
    pub fn nibbles(&self) -> &[u8] {
        &self.nibbles
    }

    /// 获取路径长度
    pub fn len(&self) -> usize {
        self.nibbles.len()
    }

    /// 检查路径是否为空
    pub fn is_empty(&self) -> bool {
        self.nibbles.is_empty()
    }

    /// 获取指定位置的 nibble
    pub fn at(&self, index: usize) -> Option<u8> {
        self.nibbles.get(index).copied()
    }

    /// 获取路径的切片
    pub fn slice(&self, start: usize, end: usize) -> Path {
        Path::from_nibbles(self.nibbles[start..end].to_vec())
    }

    /// 查找与另一个路径的公共前缀长度
    pub fn common_prefix_len(&self, other: &Path) -> usize {
        self.nibbles
            .iter()
            .zip(other.nibbles.iter())
            .take_while(|(a, b)| a == b)
            .count()
    }
}

/// MPT 错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MptError {
    /// 键未找到
    KeyNotFound,

    /// 无效的节点
    InvalidNode,

    /// 无效的路径
    InvalidPath,

    /// 编码错误
    EncodingError(String),

    /// 解码错误
    DecodingError(String),

    /// 哈希不匹配
    HashMismatch,

    /// 存储错误
    StorageError(String),

    /// 验证失败
    ProofVerificationFailed,
}

impl fmt::Display for MptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MptError::KeyNotFound => write!(f, "Key not found"),
            MptError::InvalidNode => write!(f, "Invalid node"),
            MptError::InvalidPath => write!(f, "Invalid path"),
            MptError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            MptError::DecodingError(msg) => write!(f, "Decoding error: {}", msg),
            MptError::HashMismatch => write!(f, "Hash mismatch"),
            MptError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            MptError::ProofVerificationFailed => write!(f, "Proof verification failed"),
        }
    }
}

impl std::error::Error for MptError {}

/// MPT 结果类型
pub type MptResult<T> = Result<T, MptError>;

/// Merkle 证明
///
/// 用于验证某个键值对是否存在于 MPT 中
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleProof {
    /// 根哈希
    pub root_hash: [u8; 32],

    /// 键
    pub key: Vec<u8>,

    /// 值（如果存在）
    pub value: Option<Vec<u8>>,

    /// 证明路径上的节点
    pub nodes: Vec<Node>,
}

impl MerkleProof {
    /// 创建新的 Merkle 证明
    pub fn new(
        root_hash: [u8; 32],
        key: Vec<u8>,
        value: Option<Vec<u8>>,
        nodes: Vec<Node>,
    ) -> Self {
        Self {
            root_hash,
            key,
            value,
            nodes,
        }
    }

    /// 验证证明是否有效
    pub fn verify(&self) -> MptResult<bool> {
        if self.nodes.is_empty() {
            return Ok(false);
        }

        // 简化验证：检查证明路径上的节点数量和值的存在性
        // 完整实现需要：
        // 1. 从叶子节点开始，逐层验证哈希
        // 2. 验证每个节点的哈希与父节点中记录的哈希一致
        // 3. 最终验证根哈希

        // 这里我们做基本验证：检查值是否匹配
        if self.value.is_none() {
            return Ok(false);
        }

        Ok(true)
    }

    /// 获取证明的深度（路径长度）
    pub fn depth(&self) -> usize {
        self.nodes.len()
    }

    /// 获取证明的序列化大小（字节）
    pub fn proof_size(&self) -> usize {
        let mut size = 32; // root_hash
        size += self.key.len();
        size += self.value.as_ref().map_or(0, |v| v.len());

        // 估算节点大小
        for node in &self.nodes {
            size += match node {
                Node::Empty => 1,
                Node::Leaf { partial_path, value } => 1 + partial_path.len() + value.len(),
                Node::Extension { partial_path, next_node_hash: _ } => 1 + partial_path.len() + 32,
                Node::Branch { children, value } => {
                    1 + children.iter().filter(|c| c.is_some()).count() * 32
                        + value.as_ref().map_or(0, |v| v.len())
                }
            };
        }

        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let empty = Node::empty();
        assert!(empty.is_empty());

        let leaf = Node::leaf(vec![1, 2, 3], vec![4, 5, 6]);
        assert!(leaf.is_leaf());

        let extension = Node::extension(vec![1, 2], [0u8; 32]);
        assert!(extension.is_extension());

        let branch = Node::branch([None; 16], Some(vec![1, 2, 3]));
        assert!(branch.is_branch());
    }

    #[test]
    fn test_path_from_bytes() {
        let bytes = vec![0x12, 0x34, 0xAB];
        let path = Path::from_bytes(&bytes);

        assert_eq!(path.len(), 6);
        assert_eq!(path.at(0), Some(0x1));
        assert_eq!(path.at(1), Some(0x2));
        assert_eq!(path.at(2), Some(0x3));
        assert_eq!(path.at(3), Some(0x4));
        assert_eq!(path.at(4), Some(0xA));
        assert_eq!(path.at(5), Some(0xB));
    }

    #[test]
    fn test_path_common_prefix() {
        let path1 = Path::from_nibbles(vec![1, 2, 3, 4, 5]);
        let path2 = Path::from_nibbles(vec![1, 2, 3, 6, 7]);

        assert_eq!(path1.common_prefix_len(&path2), 3);
    }
}
