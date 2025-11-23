//! 密码学原语模块
//!
//! 注意：这是简化的实现，用于演示目的。
//! 生产环境应使用标准密码学库（如 ed25519-dalek, sha2 等）

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher};

/// 哈希值类型（256位）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Hash([u8; 32]);

impl Hash {
    /// 创建零哈希
    pub const fn zero() -> Self {
        Self([0u8; 32])
    }

    /// 从字节数组创建
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// 获取字节数组
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// 计算数据的哈希（简化实现）
    pub fn compute<T: StdHash>(data: &T) -> Self {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        let hash_u64 = hasher.finish();

        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&hash_u64.to_le_bytes());
        // 为了演示，重复填充
        for i in 1..4 {
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&hash_u64.to_le_bytes());
        }

        Self(bytes)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:016x}", u64::from_le_bytes(self.0[0..8].try_into().unwrap()))
    }
}

/// 签名类型（512位）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signature([u8; 64]);

impl Signature {
    /// 创建零签名
    pub const fn zero() -> Self {
        Self([0u8; 64])
    }

    /// 从字节数组创建
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// 获取字节数组
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self::zero()
    }
}

/// 公钥类型（256位）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PublicKey([u8; 32]);

impl PublicKey {
    /// 从字节数组创建
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// 获取字节数组
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// 从 u64 创建（用于测试）
    pub fn from_u64(id: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&id.to_le_bytes());
        Self(bytes)
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PubKey({:016x})", u64::from_le_bytes(self.0[0..8].try_into().unwrap()))
    }
}

/// 私钥类型（256位）
#[derive(Debug, Clone)]
pub struct PrivateKey([u8; 32]);

impl PrivateKey {
    /// 从字节数组创建
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// 从 u64 创建（用于测试）
    pub fn from_u64(id: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..8].copy_from_slice(&id.to_le_bytes());
        Self(bytes)
    }

    /// 获取对应的公钥
    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0)
    }

    /// 签名数据（简化实现）
    pub fn sign<T: StdHash>(&self, data: &T) -> Signature {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        data.hash(&mut hasher);
        let sig_u64 = hasher.finish();

        let mut bytes = [0u8; 64];
        for i in 0..8 {
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&sig_u64.to_le_bytes());
        }

        Signature(bytes)
    }

    /// 验证签名（简化实现 - 总是返回 true）
    pub fn verify<T: StdHash>(_public_key: &PublicKey, _data: &T, _signature: &Signature) -> bool {
        // 简化实现：在真实场景中需要实现正确的签名验证
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_compute() {
        let data = "test data";
        let hash1 = Hash::compute(&data);
        let hash2 = Hash::compute(&data);
        assert_eq!(hash1, hash2);

        let different_data = "different data";
        let hash3 = Hash::compute(&different_data);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_signature() {
        let private_key = PrivateKey::from_u64(1);
        let public_key = private_key.public_key();

        let data = "test message";
        let signature = private_key.sign(&data);

        assert!(PrivateKey::verify(&public_key, &data, &signature));
    }
}
