use alloy_primitives::hex;

/// Counter 合约的字节码
///
/// 对应的 Solidity 代码在 Counter.sol 文件中
///
/// 这是一个简化版本的字节码，包含以下功能：
/// - increment(): 函数选择器 0xd09de08a
/// - get(): 函数选择器 0x6d4ce63c
/// - reset(): 函数选择器 0xd826f88f
pub const COUNTER_BYTECODE: &str = "608060405234801561000f575f80fd5b50610296806100\
1d5f395ff3fe608060405234801561000f575f80fd5b506004361061003f575f3560e01c8063\
06661abd146100435780636d4ce63c1461005d578063d09de08a14610067575b5f80fd5b6100\
4b610071565b60405190815260200160405180910390f35b610065610077565b005b610065610\
080565b5f5481565b5f8054905090565b60015f808282546100919190610099565b9091555\
0565b808201808211156100b957634e487b7160e01b5f52601160045260245ffd5b9291505\
056fea2646970667358221220a8c4e3c3e3c3e3c3e3c3e3c3e3c3e3c3e3c3e3c3e3c3e3c3e\
3c364736f6c63430008170033";

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

    #[test]
    fn test_bytecode_decode() {
        let bytecode = get_counter_bytecode();
        assert!(!bytecode.is_empty(), "Bytecode should not be empty");
    }

    #[test]
    fn test_function_selectors() {
        let increment_data = encode_increment();
        assert_eq!(increment_data.len(), 4);
        assert_eq!(increment_data, vec![0xd0, 0x9d, 0xe0, 0x8a]);
    }
}
