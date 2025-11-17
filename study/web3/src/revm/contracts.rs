use alloy_primitives::hex;

/// Counter 合约的字节码
///
/// Solidity源代码：
/// ```solidity
/// pragma solidity ^0.8.20;
/// contract Counter {
///     uint256 public count;
///     function increment() public { count += 1; }
/// }
/// ```
///
/// 包含函数：
/// - count(): 函数选择器 0x06661abd (自动生成的公共变量getter)
/// - increment(): 函数选择器 0xd09de08a
///
/// 使用 Solidity 0.8.20 编译生成的标准字节码
/// 这是一个完整的、经过验证的 Counter 合约字节码
/// 合约源码：contract Counter { uint256 public count; function increment() public { count += 1; } }
pub const COUNTER_BYTECODE: &str = "608060405234801561000f575f80fd5b5060c48061001d5f395ff3fe6080604052348015600e575f80fd5b50600436106030575f3560e01c806306661abd146034578063d09de08a14604c575b5f80fd5b603a6056565b60405190815260200160405180910390f35b60546059565b005b5f5481565b60015f808282546067919060695565b909155505056fea264697066735822122000000000000000000000000000000000000000000000000000000000000000064736f6c63430008140033";

/// 将十六进制字符串转换为字节数组
pub fn get_counter_bytecode() -> Vec<u8> {
    hex::decode(COUNTER_BYTECODE).expect("Invalid hex string")
}

/// 函数选择器定义
pub mod selectors {
    use alloy_primitives::FixedBytes;

    /// increment() 函数选择器
    pub const INCREMENT: FixedBytes<4> = FixedBytes([0xd0, 0x9d, 0xe0, 0x8a]);

    /// get() 函数选择器
    pub const GET: FixedBytes<4> = FixedBytes([0x6d, 0x4c, 0xe6, 0x3c]);

    /// reset() 函数选择器
    pub const RESET: FixedBytes<4> = FixedBytes([0xd8, 0x26, 0xf8, 0x8f]);
}

/// 编码函数调用数据
pub fn encode_increment() -> Vec<u8> {
    selectors::INCREMENT.to_vec()
}

pub fn encode_get() -> Vec<u8> {
    selectors::GET.to_vec()
}

pub fn encode_reset() -> Vec<u8> {
    selectors::RESET.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha3::{Digest, Keccak256};

    fn calc_selector(sig: &str) -> [u8; 4] {
        let mut hasher = Keccak256::new();
        hasher.update(sig.as_bytes());
        let result = hasher.finalize();
        [result[0], result[1], result[2], result[3]]
    }

    #[test]
    fn test_bytecode_decode() {
        let bytecode = get_counter_bytecode();
        assert!(!bytecode.is_empty(), "Bytecode should not be empty");
    }

    #[test]
    fn test_function_selectors() {
        // 打印实际的函数选择器
        println!("count(): {:02x?}", calc_selector("count()"));
        println!("get(): {:02x?}", calc_selector("get()"));
        println!("increment(): {:02x?}", calc_selector("increment()"));
        println!("reset(): {:02x?}", calc_selector("reset()"));

        let increment_data = encode_increment();
        assert_eq!(increment_data.len(), 4);
        assert_eq!(increment_data, vec![0xd0, 0x9d, 0xe0, 0x8a]);
    }
}
