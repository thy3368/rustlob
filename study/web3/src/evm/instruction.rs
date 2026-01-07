//! EVM 指令集定义
//!
//! 基于 RISC-V 设计原则构建的以太坊虚拟机指令集
//!
//! 设计原则：
//! 1. 简单性 - 指令定义清晰，易于实现
//! 2. 正交性 - 指令功能独立，最小重叠
//! 3. 可扩展性 - 便于添加新指令
//! 4. 性能优化 - 高频指令优化执行路径
//!
//! 指令分类：
//! - 算术运算：ADD, MUL, SUB, DIV, MOD, EXP
//! - 比较运算：LT, GT, EQ, ISZERO
//! - 位运算：AND, OR, XOR, NOT, SHL, SHR
//! - 栈操作：PUSH, POP, DUP, SWAP
//! - 内存操作：MLOAD, MSTORE, MSIZE
//! - 存储操作：SLOAD, SSTORE
//! - 控制流：JUMP, JUMPI, JUMPDEST, PC
//! - 区块链相关：BLOCKHASH, COINBASE, TIMESTAMP, NUMBER
//! - 系统操作：CALL, RETURN, REVERT, SELFDESTRUCT

use std::fmt;

/// EVM 指令操作码 (参考以太坊黄皮书)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OpCode {
    // ========================================
    // 0x00 范围：停止和算术操作
    // ========================================

    /// 0x00 - 停止执行
    STOP = 0x00,

    /// 0x01 - 加法：a + b
    ADD = 0x01,

    /// 0x02 - 乘法：a * b
    MUL = 0x02,

    /// 0x03 - 减法：a - b
    SUB = 0x03,

    /// 0x04 - 除法：a / b (整数除法)
    DIV = 0x04,

    /// 0x05 - 有符号除法：a / b
    SDIV = 0x05,

    /// 0x06 - 取模：a % b
    MOD = 0x06,

    /// 0x07 - 有符号取模
    SMOD = 0x07,

    /// 0x08 - 模加：(a + b) % m
    ADDMOD = 0x08,

    /// 0x09 - 模乘：(a * b) % m
    MULMOD = 0x09,

    /// 0x0a - 指数：a^b
    EXP = 0x0a,

    /// 0x0b - 符号扩展
    SIGNEXTEND = 0x0b,

    // ========================================
    // 0x10 范围：比较和位运算
    // ========================================

    /// 0x10 - 小于：a < b
    LT = 0x10,

    /// 0x11 - 大于：a > b
    GT = 0x11,

    /// 0x12 - 有符号小于
    SLT = 0x12,

    /// 0x13 - 有符号大于
    SGT = 0x13,

    /// 0x14 - 等于：a == b
    EQ = 0x14,

    /// 0x15 - 是否为零：a == 0
    ISZERO = 0x15,

    /// 0x16 - 按位与：a & b
    AND = 0x16,

    /// 0x17 - 按位或：a | b
    OR = 0x17,

    /// 0x18 - 按位异或：a ^ b
    XOR = 0x18,

    /// 0x19 - 按位取反：~a
    NOT = 0x19,

    /// 0x1a - 取字节：从 word 中取第 i 个字节
    BYTE = 0x1a,

    /// 0x1b - 左移：a << b
    SHL = 0x1b,

    /// 0x1c - 逻辑右移：a >> b
    SHR = 0x1c,

    /// 0x1d - 算术右移
    SAR = 0x1d,

    // ========================================
    // 0x20 范围：哈希运算
    // ========================================

    /// 0x20 - Keccak-256 哈希
    KECCAK256 = 0x20,

    // ========================================
    // 0x30 范围：环境信息
    // ========================================

    /// 0x30 - 获取地址
    ADDRESS = 0x30,

    /// 0x31 - 获取余额
    BALANCE = 0x31,

    /// 0x32 - 获取交易发起者
    ORIGIN = 0x32,

    /// 0x33 - 获取调用者
    CALLER = 0x33,

    /// 0x34 - 获取调用值 (msg.value)
    CALLVALUE = 0x34,

    /// 0x35 - 加载调用数据
    CALLDATALOAD = 0x35,

    /// 0x36 - 获取调用数据大小
    CALLDATASIZE = 0x36,

    /// 0x37 - 复制调用数据
    CALLDATACOPY = 0x37,

    /// 0x38 - 获取代码大小
    CODESIZE = 0x38,

    /// 0x39 - 复制代码
    CODECOPY = 0x39,

    /// 0x3a - 获取 gas 价格
    GASPRICE = 0x3a,

    /// 0x3b - 获取外部代码大小
    EXTCODESIZE = 0x3b,

    /// 0x3c - 复制外部代码
    EXTCODECOPY = 0x3c,

    /// 0x3d - 返回数据大小
    RETURNDATASIZE = 0x3d,

    /// 0x3e - 复制返回数据
    RETURNDATACOPY = 0x3e,

    /// 0x3f - 获取外部代码哈希
    EXTCODEHASH = 0x3f,

    // ========================================
    // 0x40 范围：区块信息
    // ========================================

    /// 0x40 - 获取区块哈希
    BLOCKHASH = 0x40,

    /// 0x41 - 获取挖矿者地址 (coinbase)
    COINBASE = 0x41,

    /// 0x42 - 获取时间戳
    TIMESTAMP = 0x42,

    /// 0x43 - 获取区块号
    NUMBER = 0x43,

    /// 0x44 - 获取难度
    DIFFICULTY = 0x44,

    /// 0x45 - 获取 gas limit
    GASLIMIT = 0x45,

    /// 0x46 - 获取链 ID
    CHAINID = 0x46,

    /// 0x47 - 获取当前余额
    SELFBALANCE = 0x47,

    /// 0x48 - 获取 base fee
    BASEFEE = 0x48,

    // ========================================
    // 0x50 范围：栈、内存、存储和控制流
    // ========================================

    /// 0x50 - 移除栈顶元素
    POP = 0x50,

    /// 0x51 - 从内存加载
    MLOAD = 0x51,

    /// 0x52 - 存储到内存
    MSTORE = 0x52,

    /// 0x53 - 存储单字节到内存
    MSTORE8 = 0x53,

    /// 0x54 - 从存储加载
    SLOAD = 0x54,

    /// 0x55 - 存储到存储
    SSTORE = 0x55,

    /// 0x56 - 无条件跳转
    JUMP = 0x56,

    /// 0x57 - 条件跳转
    JUMPI = 0x57,

    /// 0x58 - 获取程序计数器
    PC = 0x58,

    /// 0x59 - 获取内存大小
    MSIZE = 0x59,

    /// 0x5a - 获取剩余 gas
    GAS = 0x5a,

    /// 0x5b - 跳转目标标记
    JUMPDEST = 0x5b,

    // ========================================
    // 0x60-0x7f 范围：PUSH 操作
    // ========================================

    /// 0x60 - 压入 1 字节
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6a,
    PUSH12 = 0x6b,
    PUSH13 = 0x6c,
    PUSH14 = 0x6d,
    PUSH15 = 0x6e,
    PUSH16 = 0x6f,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7a,
    PUSH28 = 0x7b,
    PUSH29 = 0x7c,
    PUSH30 = 0x7d,
    PUSH31 = 0x7e,

    /// 0x7f - 压入 32 字节
    PUSH32 = 0x7f,

    // ========================================
    // 0x80-0x8f 范围：DUP 操作
    // ========================================

    /// 0x80 - 复制第 1 个栈元素
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8a,
    DUP12 = 0x8b,
    DUP13 = 0x8c,
    DUP14 = 0x8d,
    DUP15 = 0x8e,

    /// 0x8f - 复制第 16 个栈元素
    DUP16 = 0x8f,

    // ========================================
    // 0x90-0x9f 范围：SWAP 操作
    // ========================================

    /// 0x90 - 交换第 1 和第 2 个元素
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9a,
    SWAP12 = 0x9b,
    SWAP13 = 0x9c,
    SWAP14 = 0x9d,
    SWAP15 = 0x9e,

    /// 0x9f - 交换第 1 和第 17 个元素
    SWAP16 = 0x9f,

    // ========================================
    // 0xa0-0xa4 范围：日志操作
    // ========================================

    /// 0xa0 - 产生 0 个主题的日志
    LOG0 = 0xa0,
    LOG1 = 0xa1,
    LOG2 = 0xa2,
    LOG3 = 0xa3,

    /// 0xa4 - 产生 4 个主题的日志
    LOG4 = 0xa4,

    // ========================================
    // 0xf0-0xff 范围：系统操作
    // ========================================

    /// 0xf0 - 创建新合约
    CREATE = 0xf0,

    /// 0xf1 - 消息调用
    CALL = 0xf1,

    /// 0xf2 - 带代码的消息调用
    CALLCODE = 0xf2,

    /// 0xf3 - 停止执行并返回数据
    RETURN = 0xf3,

    /// 0xf4 - 委托调用
    DELEGATECALL = 0xf4,

    /// 0xf5 - 创建新合约 (CREATE2)
    CREATE2 = 0xf5,

    /// 0xfa - 静态调用
    STATICCALL = 0xfa,

    /// 0xfd - 恢复执行并返回数据
    REVERT = 0xfd,

    /// 0xfe - 无效指令
    INVALID = 0xfe,

    /// 0xff - 销毁合约
    SELFDESTRUCT = 0xff,
}

impl OpCode {
    /// 从字节转换为操作码
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(OpCode::STOP),
            0x01 => Some(OpCode::ADD),
            0x02 => Some(OpCode::MUL),
            0x03 => Some(OpCode::SUB),
            0x04 => Some(OpCode::DIV),
            0x05 => Some(OpCode::SDIV),
            0x06 => Some(OpCode::MOD),
            0x07 => Some(OpCode::SMOD),
            0x08 => Some(OpCode::ADDMOD),
            0x09 => Some(OpCode::MULMOD),
            0x0a => Some(OpCode::EXP),
            0x0b => Some(OpCode::SIGNEXTEND),

            0x10 => Some(OpCode::LT),
            0x11 => Some(OpCode::GT),
            0x12 => Some(OpCode::SLT),
            0x13 => Some(OpCode::SGT),
            0x14 => Some(OpCode::EQ),
            0x15 => Some(OpCode::ISZERO),
            0x16 => Some(OpCode::AND),
            0x17 => Some(OpCode::OR),
            0x18 => Some(OpCode::XOR),
            0x19 => Some(OpCode::NOT),
            0x1a => Some(OpCode::BYTE),
            0x1b => Some(OpCode::SHL),
            0x1c => Some(OpCode::SHR),
            0x1d => Some(OpCode::SAR),

            0x20 => Some(OpCode::KECCAK256),

            0x30 => Some(OpCode::ADDRESS),
            0x31 => Some(OpCode::BALANCE),
            0x32 => Some(OpCode::ORIGIN),
            0x33 => Some(OpCode::CALLER),
            0x34 => Some(OpCode::CALLVALUE),
            0x35 => Some(OpCode::CALLDATALOAD),
            0x36 => Some(OpCode::CALLDATASIZE),
            0x37 => Some(OpCode::CALLDATACOPY),
            0x38 => Some(OpCode::CODESIZE),
            0x39 => Some(OpCode::CODECOPY),
            0x3a => Some(OpCode::GASPRICE),
            0x3b => Some(OpCode::EXTCODESIZE),
            0x3c => Some(OpCode::EXTCODECOPY),
            0x3d => Some(OpCode::RETURNDATASIZE),
            0x3e => Some(OpCode::RETURNDATACOPY),
            0x3f => Some(OpCode::EXTCODEHASH),

            0x40 => Some(OpCode::BLOCKHASH),
            0x41 => Some(OpCode::COINBASE),
            0x42 => Some(OpCode::TIMESTAMP),
            0x43 => Some(OpCode::NUMBER),
            0x44 => Some(OpCode::DIFFICULTY),
            0x45 => Some(OpCode::GASLIMIT),
            0x46 => Some(OpCode::CHAINID),
            0x47 => Some(OpCode::SELFBALANCE),
            0x48 => Some(OpCode::BASEFEE),

            0x50 => Some(OpCode::POP),
            0x51 => Some(OpCode::MLOAD),
            0x52 => Some(OpCode::MSTORE),
            0x53 => Some(OpCode::MSTORE8),
            0x54 => Some(OpCode::SLOAD),
            0x55 => Some(OpCode::SSTORE),
            0x56 => Some(OpCode::JUMP),
            0x57 => Some(OpCode::JUMPI),
            0x58 => Some(OpCode::PC),
            0x59 => Some(OpCode::MSIZE),
            0x5a => Some(OpCode::GAS),
            0x5b => Some(OpCode::JUMPDEST),

            0x60..=0x7f => Some(unsafe { std::mem::transmute(byte) }),
            0x80..=0x8f => Some(unsafe { std::mem::transmute(byte) }),
            0x90..=0x9f => Some(unsafe { std::mem::transmute(byte) }),

            0xa0 => Some(OpCode::LOG0),
            0xa1 => Some(OpCode::LOG1),
            0xa2 => Some(OpCode::LOG2),
            0xa3 => Some(OpCode::LOG3),
            0xa4 => Some(OpCode::LOG4),

            0xf0 => Some(OpCode::CREATE),
            0xf1 => Some(OpCode::CALL),
            0xf2 => Some(OpCode::CALLCODE),
            0xf3 => Some(OpCode::RETURN),
            0xf4 => Some(OpCode::DELEGATECALL),
            0xf5 => Some(OpCode::CREATE2),
            0xfa => Some(OpCode::STATICCALL),
            0xfd => Some(OpCode::REVERT),
            0xfe => Some(OpCode::INVALID),
            0xff => Some(OpCode::SELFDESTRUCT),

            _ => None,
        }
    }

    /// 获取 PUSH 指令压入的字节数
    pub fn push_bytes(&self) -> Option<u8> {
        if self.is_push() {
            Some((*self as u8) - 0x5f)
        } else {
            None
        }
    }

    /// 检查是否是 PUSH 指令
    pub fn is_push(&self) -> bool {
        let opcode = *self as u8;
        (0x60..=0x7f).contains(&opcode)
    }

    /// 检查是否是 DUP 指令
    pub fn is_dup(&self) -> bool {
        let opcode = *self as u8;
        (0x80..=0x8f).contains(&opcode)
    }

    /// 检查是否是 SWAP 指令
    pub fn is_swap(&self) -> bool {
        let opcode = *self as u8;
        (0x90..=0x9f).contains(&opcode)
    }

    /// 获取指令名称
    pub fn name(&self) -> &'static str {
        match self {
            OpCode::STOP => "STOP",
            OpCode::ADD => "ADD",
            OpCode::MUL => "MUL",
            OpCode::SUB => "SUB",
            OpCode::DIV => "DIV",
            OpCode::SDIV => "SDIV",
            OpCode::MOD => "MOD",
            OpCode::SMOD => "SMOD",
            OpCode::ADDMOD => "ADDMOD",
            OpCode::MULMOD => "MULMOD",
            OpCode::EXP => "EXP",
            OpCode::SIGNEXTEND => "SIGNEXTEND",
            OpCode::LT => "LT",
            OpCode::GT => "GT",
            OpCode::SLT => "SLT",
            OpCode::SGT => "SGT",
            OpCode::EQ => "EQ",
            OpCode::ISZERO => "ISZERO",
            OpCode::AND => "AND",
            OpCode::OR => "OR",
            OpCode::XOR => "XOR",
            OpCode::NOT => "NOT",
            OpCode::BYTE => "BYTE",
            OpCode::SHL => "SHL",
            OpCode::SHR => "SHR",
            OpCode::SAR => "SAR",
            OpCode::KECCAK256 => "KECCAK256",
            OpCode::ADDRESS => "ADDRESS",
            OpCode::BALANCE => "BALANCE",
            OpCode::ORIGIN => "ORIGIN",
            OpCode::CALLER => "CALLER",
            OpCode::CALLVALUE => "CALLVALUE",
            OpCode::CALLDATALOAD => "CALLDATALOAD",
            OpCode::CALLDATASIZE => "CALLDATASIZE",
            OpCode::CALLDATACOPY => "CALLDATACOPY",
            OpCode::CODESIZE => "CODESIZE",
            OpCode::CODECOPY => "CODECOPY",
            OpCode::GASPRICE => "GASPRICE",
            OpCode::EXTCODESIZE => "EXTCODESIZE",
            OpCode::EXTCODECOPY => "EXTCODECOPY",
            OpCode::RETURNDATASIZE => "RETURNDATASIZE",
            OpCode::RETURNDATACOPY => "RETURNDATACOPY",
            OpCode::EXTCODEHASH => "EXTCODEHASH",
            OpCode::BLOCKHASH => "BLOCKHASH",
            OpCode::COINBASE => "COINBASE",
            OpCode::TIMESTAMP => "TIMESTAMP",
            OpCode::NUMBER => "NUMBER",
            OpCode::DIFFICULTY => "DIFFICULTY",
            OpCode::GASLIMIT => "GASLIMIT",
            OpCode::CHAINID => "CHAINID",
            OpCode::SELFBALANCE => "SELFBALANCE",
            OpCode::BASEFEE => "BASEFEE",
            OpCode::POP => "POP",
            OpCode::MLOAD => "MLOAD",
            OpCode::MSTORE => "MSTORE",
            OpCode::MSTORE8 => "MSTORE8",
            OpCode::SLOAD => "SLOAD",
            OpCode::SSTORE => "SSTORE",
            OpCode::JUMP => "JUMP",
            OpCode::JUMPI => "JUMPI",
            OpCode::PC => "PC",
            OpCode::MSIZE => "MSIZE",
            OpCode::GAS => "GAS",
            OpCode::JUMPDEST => "JUMPDEST",
            OpCode::PUSH1 => "PUSH1",
            OpCode::PUSH2 => "PUSH2",
            OpCode::PUSH3 => "PUSH3",
            OpCode::PUSH4 => "PUSH4",
            OpCode::PUSH5 => "PUSH5",
            OpCode::PUSH6 => "PUSH6",
            OpCode::PUSH7 => "PUSH7",
            OpCode::PUSH8 => "PUSH8",
            OpCode::PUSH9 => "PUSH9",
            OpCode::PUSH10 => "PUSH10",
            OpCode::PUSH11 => "PUSH11",
            OpCode::PUSH12 => "PUSH12",
            OpCode::PUSH13 => "PUSH13",
            OpCode::PUSH14 => "PUSH14",
            OpCode::PUSH15 => "PUSH15",
            OpCode::PUSH16 => "PUSH16",
            OpCode::PUSH17 => "PUSH17",
            OpCode::PUSH18 => "PUSH18",
            OpCode::PUSH19 => "PUSH19",
            OpCode::PUSH20 => "PUSH20",
            OpCode::PUSH21 => "PUSH21",
            OpCode::PUSH22 => "PUSH22",
            OpCode::PUSH23 => "PUSH23",
            OpCode::PUSH24 => "PUSH24",
            OpCode::PUSH25 => "PUSH25",
            OpCode::PUSH26 => "PUSH26",
            OpCode::PUSH27 => "PUSH27",
            OpCode::PUSH28 => "PUSH28",
            OpCode::PUSH29 => "PUSH29",
            OpCode::PUSH30 => "PUSH30",
            OpCode::PUSH31 => "PUSH31",
            OpCode::PUSH32 => "PUSH32",
            OpCode::DUP1 => "DUP1",
            OpCode::DUP2 => "DUP2",
            OpCode::DUP3 => "DUP3",
            OpCode::DUP4 => "DUP4",
            OpCode::DUP5 => "DUP5",
            OpCode::DUP6 => "DUP6",
            OpCode::DUP7 => "DUP7",
            OpCode::DUP8 => "DUP8",
            OpCode::DUP9 => "DUP9",
            OpCode::DUP10 => "DUP10",
            OpCode::DUP11 => "DUP11",
            OpCode::DUP12 => "DUP12",
            OpCode::DUP13 => "DUP13",
            OpCode::DUP14 => "DUP14",
            OpCode::DUP15 => "DUP15",
            OpCode::DUP16 => "DUP16",
            OpCode::SWAP1 => "SWAP1",
            OpCode::SWAP2 => "SWAP2",
            OpCode::SWAP3 => "SWAP3",
            OpCode::SWAP4 => "SWAP4",
            OpCode::SWAP5 => "SWAP5",
            OpCode::SWAP6 => "SWAP6",
            OpCode::SWAP7 => "SWAP7",
            OpCode::SWAP8 => "SWAP8",
            OpCode::SWAP9 => "SWAP9",
            OpCode::SWAP10 => "SWAP10",
            OpCode::SWAP11 => "SWAP11",
            OpCode::SWAP12 => "SWAP12",
            OpCode::SWAP13 => "SWAP13",
            OpCode::SWAP14 => "SWAP14",
            OpCode::SWAP15 => "SWAP15",
            OpCode::SWAP16 => "SWAP16",
            OpCode::LOG0 => "LOG0",
            OpCode::LOG1 => "LOG1",
            OpCode::LOG2 => "LOG2",
            OpCode::LOG3 => "LOG3",
            OpCode::LOG4 => "LOG4",
            OpCode::CREATE => "CREATE",
            OpCode::CALL => "CALL",
            OpCode::CALLCODE => "CALLCODE",
            OpCode::RETURN => "RETURN",
            OpCode::DELEGATECALL => "DELEGATECALL",
            OpCode::CREATE2 => "CREATE2",
            OpCode::STATICCALL => "STATICCALL",
            OpCode::REVERT => "REVERT",
            OpCode::INVALID => "INVALID",
            OpCode::SELFDESTRUCT => "SELFDESTRUCT",
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_u8() {
        assert_eq!(OpCode::from_u8(0x00), Some(OpCode::STOP));
        assert_eq!(OpCode::from_u8(0x01), Some(OpCode::ADD));
        assert_eq!(OpCode::from_u8(0x60), Some(OpCode::PUSH1));
        assert_eq!(OpCode::from_u8(0x7f), Some(OpCode::PUSH32));
        assert_eq!(OpCode::from_u8(0xff), Some(OpCode::SELFDESTRUCT));
        assert_eq!(OpCode::from_u8(0x0c), None); // 无效操作码
    }

    #[test]
    fn test_push_bytes() {
        assert_eq!(OpCode::PUSH1.push_bytes(), Some(1));
        assert_eq!(OpCode::PUSH32.push_bytes(), Some(32));
        assert_eq!(OpCode::ADD.push_bytes(), None);
    }

    #[test]
    fn test_is_push() {
        assert!(OpCode::PUSH1.is_push());
        assert!(OpCode::PUSH32.is_push());
        assert!(!OpCode::ADD.is_push());
    }

    #[test]
    fn test_opcode_name() {
        assert_eq!(OpCode::ADD.name(), "ADD");
        assert_eq!(OpCode::PUSH1.name(), "PUSH1");
        assert_eq!(OpCode::SELFDESTRUCT.name(), "SELFDESTRUCT");
    }
}