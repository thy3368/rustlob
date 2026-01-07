//! EVM execution engine
//!
//! Based on RISC-V design principles:
//! 1. Simplicity: Clear execution flow
//! 2. Orthogonality: Independent component design
//! 3. Extensibility: Modular architecture
//! 4. Performance: Efficient implementation

use crate::evm::instruction::OpCode;
use std::collections::HashMap;

pub type Word = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    StackUnderflow,
    StackOverflow,
    InvalidJump,
    InvalidOpcode,
    OutOfGas,
    MemoryOutOfBounds,
    DivisionByZero,
    Revert(Vec<u8>),
    Stop,
    Return(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Stack {
    data: Vec<Word>,
    max_depth: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(1024),
            max_depth: 1024,
        }
    }

    pub fn push(&mut self, value: Word) -> Result<(), ExecError> {
        if self.data.len() >= self.max_depth {
            return Err(ExecError::StackOverflow);
        }
        self.data.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Word, ExecError> {
        self.data.pop().ok_or(ExecError::StackUnderflow)
    }

    pub fn peek(&self) -> Result<&Word, ExecError> {
        self.data.last().ok_or(ExecError::StackUnderflow)
    }

    pub fn dup(&mut self, n: usize) -> Result<(), ExecError> {
        if n == 0 || n > 16 {
            return Err(ExecError::StackUnderflow);
        }

        let len = self.data.len();
        if n > len {
            return Err(ExecError::StackUnderflow);
        }

        let value = self.data[len - n];
        self.push(value)
    }

    pub fn swap(&mut self, n: usize) -> Result<(), ExecError> {
        if n == 0 || n > 16 {
            return Err(ExecError::StackUnderflow);
        }

        let len = self.data.len();
        if n >= len {
            return Err(ExecError::StackUnderflow);
        }

        let last_idx = len - 1;
        let swap_idx = len - 1 - n;
        self.data.swap(last_idx, swap_idx);
        Ok(())
    }

    pub fn depth(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn read(&self, offset: usize, size: usize) -> Result<Vec<u8>, ExecError> {
        if offset + size > self.data.len() {
            return Err(ExecError::MemoryOutOfBounds);
        }
        Ok(self.data[offset..offset + size].to_vec())
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), ExecError> {
        let required_size = offset + data.len();
        if required_size > self.data.len() {
            let new_size = ((required_size + 31) / 32) * 32;
            self.data.resize(new_size, 0);
        }
        self.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    data: HashMap<Word, Word>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn read(&self, key: &Word) -> Word {
        self.data.get(key).copied().unwrap_or([0u8; 32])
    }

    pub fn write(&mut self, key: Word, value: Word) {
        if value == [0u8; 32] {
            self.data.remove(&key);
        } else {
            self.data.insert(key, value);
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub pc: usize,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub call_depth: usize,
}

impl Context {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            pc: 0,
            gas_used: 0,
            gas_limit,
            call_depth: 0,
        }
    }

    pub fn consume_gas(&mut self, amount: u64) -> Result<(), ExecError> {
        if self.gas_used + amount > self.gas_limit {
            return Err(ExecError::OutOfGas);
        }
        self.gas_used += amount;
        Ok(())
    }

    pub fn jump(&mut self, target: usize) -> Result<(), ExecError> {
        self.pc = target;
        Ok(())
    }

    pub fn step(&mut self, size: usize) {
        self.pc += size;
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub stack: Stack,
    pub memory: Memory,
    pub storage: Storage,
    pub context: Context,
    pub code: Vec<u8>,
    jump_dests: Vec<bool>,
}

impl Interpreter {
    pub fn new(code: Vec<u8>, gas_limit: u64) -> Self {
        let jump_dests = Self::analyze_jump_destinations(&code);
        Self {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
            context: Context::new(gas_limit),
            code,
            jump_dests,
        }
    }

    fn analyze_jump_destinations(code: &[u8]) -> Vec<bool> {
        let mut jump_dests = vec![false; code.len()];
        let mut i = 0;

        while i < code.len() {
            if let Some(opcode) = OpCode::from_u8(code[i]) {
                if opcode == OpCode::JUMPDEST {
                    jump_dests[i] = true;
                }

                if let Some(push_bytes) = opcode.push_bytes() {
                    i += push_bytes as usize;
                }
            }
            i += 1;
        }

        jump_dests
    }

    fn is_valid_jump_dest(&self, pc: usize) -> bool {
        pc < self.jump_dests.len() && self.jump_dests[pc]
    }

    pub fn execute(&mut self) -> Result<Vec<u8>, ExecError> {
        loop {
            if self.context.pc >= self.code.len() {
                return Err(ExecError::Stop);
            }

            let opcode_byte = self.code[self.context.pc];
            let opcode = OpCode::from_u8(opcode_byte)
                .ok_or(ExecError::InvalidOpcode)?;

            match self.execute_opcode(opcode)? {
                ExecutionResult::Continue => {
                    self.context.step(1);
                }
                ExecutionResult::Jump(target) => {
                    if !self.is_valid_jump_dest(target) {
                        return Err(ExecError::InvalidJump);
                    }
                    self.context.jump(target)?;
                }
                ExecutionResult::Stop => {
                    return Err(ExecError::Stop);
                }
                ExecutionResult::Return(data) => {
                    return Ok(data);
                }
                ExecutionResult::Revert(data) => {
                    return Err(ExecError::Revert(data));
                }
            }
        }
    }

    fn execute_opcode(&mut self, opcode: OpCode) -> Result<ExecutionResult, ExecError> {
        let gas_cost = self.calculate_gas_cost(&opcode);
        self.context.consume_gas(gas_cost)?;

        match opcode {
            OpCode::STOP => Ok(ExecutionResult::Stop),

            OpCode::ADD => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = word_add(&a, &b);
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::MUL => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = word_mul(&a, &b);
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::SUB => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = word_sub(&a, &b);
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::DIV => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                if word_is_zero(&b) {
                    self.stack.push([0u8; 32])?;
                } else {
                    let result = word_div(&a, &b);
                    self.stack.push(result)?;
                }
                Ok(ExecutionResult::Continue)
            }

            OpCode::LT => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = if word_lt(&a, &b) {
                    word_from_u64(1)
                } else {
                    [0u8; 32]
                };
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::GT => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = if word_lt(&b, &a) {
                    word_from_u64(1)
                } else {
                    [0u8; 32]
                };
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::EQ => {
                let a = self.stack.pop()?;
                let b = self.stack.pop()?;
                let result = if a == b {
                    word_from_u64(1)
                } else {
                    [0u8; 32]
                };
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::ISZERO => {
                let a = self.stack.pop()?;
                let result = if word_is_zero(&a) {
                    word_from_u64(1)
                } else {
                    [0u8; 32]
                };
                self.stack.push(result)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::POP => {
                self.stack.pop()?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::MLOAD => {
                let offset = word_to_usize(&self.stack.pop()?);
                let data = self.memory.read(offset, 32)?;
                let mut word = [0u8; 32];
                word.copy_from_slice(&data);
                self.stack.push(word)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::MSTORE => {
                let offset = word_to_usize(&self.stack.pop()?);
                let value = self.stack.pop()?;
                self.memory.write(offset, &value)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::SLOAD => {
                let key = self.stack.pop()?;
                let value = self.storage.read(&key);
                self.stack.push(value)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::SSTORE => {
                let key = self.stack.pop()?;
                let value = self.stack.pop()?;
                self.storage.write(key, value);
                Ok(ExecutionResult::Continue)
            }

            OpCode::JUMP => {
                let target = word_to_usize(&self.stack.pop()?);
                Ok(ExecutionResult::Jump(target))
            }

            OpCode::JUMPI => {
                let target = word_to_usize(&self.stack.pop()?);
                let condition = self.stack.pop()?;
                if !word_is_zero(&condition) {
                    Ok(ExecutionResult::Jump(target))
                } else {
                    Ok(ExecutionResult::Continue)
                }
            }

            OpCode::PC => {
                let pc = word_from_u64(self.context.pc as u64);
                self.stack.push(pc)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::JUMPDEST => {
                Ok(ExecutionResult::Continue)
            }

            opcode if opcode.is_push() => {
                let push_bytes = opcode.push_bytes().unwrap() as usize;
                let mut value = [0u8; 32];

                let start = self.context.pc + 1;
                let end = (start + push_bytes).min(self.code.len());
                let actual_bytes = end - start;

                value[32 - actual_bytes..].copy_from_slice(&self.code[start..end]);

                self.stack.push(value)?;
                self.context.step(push_bytes);
                Ok(ExecutionResult::Continue)
            }

            opcode if opcode.is_dup() => {
                let n = (opcode as u8 - 0x7f) as usize;
                self.stack.dup(n)?;
                Ok(ExecutionResult::Continue)
            }

            opcode if opcode.is_swap() => {
                let n = (opcode as u8 - 0x8f) as usize;
                self.stack.swap(n)?;
                Ok(ExecutionResult::Continue)
            }

            OpCode::RETURN => {
                let offset = word_to_usize(&self.stack.pop()?);
                let size = word_to_usize(&self.stack.pop()?);
                let data = self.memory.read(offset, size)?;
                Ok(ExecutionResult::Return(data))
            }

            OpCode::REVERT => {
                let offset = word_to_usize(&self.stack.pop()?);
                let size = word_to_usize(&self.stack.pop()?);
                let data = self.memory.read(offset, size)?;
                Ok(ExecutionResult::Revert(data))
            }

            _ => Err(ExecError::InvalidOpcode),
        }
    }

    fn calculate_gas_cost(&self, opcode: &OpCode) -> u64 {
        if opcode.is_push() {
            return 3;
        }
        if opcode.is_dup() {
            return 3;
        }
        if opcode.is_swap() {
            return 3;
        }

        match opcode {
            OpCode::STOP => 0,
            OpCode::ADD | OpCode::SUB | OpCode::MUL | OpCode::DIV => 3,
            OpCode::LT | OpCode::GT | OpCode::EQ | OpCode::ISZERO => 3,
            OpCode::POP => 2,
            OpCode::MLOAD | OpCode::MSTORE => 3,
            OpCode::SLOAD => 200,
            OpCode::SSTORE => 5000,
            OpCode::JUMP | OpCode::JUMPI => 8,
            OpCode::PC | OpCode::JUMPDEST => 1,
            OpCode::RETURN | OpCode::REVERT => 0,
            _ => 1,
        }
    }

    pub fn reset(&mut self) {
        self.stack.clear();
        self.memory.clear();
        self.storage.clear();
        self.context.pc = 0;
        self.context.gas_used = 0;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ExecutionResult {
    Continue,
    Jump(usize),
    Stop,
    Return(Vec<u8>),
    Revert(Vec<u8>),
}

fn word_from_u64(value: u64) -> Word {
    let mut word = [0u8; 32];
    word[24..32].copy_from_slice(&value.to_be_bytes());
    word
}

fn word_to_usize(word: &Word) -> usize {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&word[24..32]);
    u64::from_be_bytes(bytes) as usize
}

fn word_is_zero(word: &Word) -> bool {
    word.iter().all(|&b| b == 0)
}

fn word_add(a: &Word, b: &Word) -> Word {
    let mut result = [0u8; 32];
    let mut carry = 0u16;

    for i in (0..32).rev() {
        let sum = a[i] as u16 + b[i] as u16 + carry;
        result[i] = sum as u8;
        carry = sum >> 8;
    }

    result
}

fn word_sub(a: &Word, b: &Word) -> Word {
    let mut result = [0u8; 32];
    let mut borrow = 0i16;

    for i in (0..32).rev() {
        let diff = a[i] as i16 - b[i] as i16 - borrow;
        if diff < 0 {
            result[i] = (diff + 256) as u8;
            borrow = 1;
        } else {
            result[i] = diff as u8;
            borrow = 0;
        }
    }

    result
}

fn word_mul(a: &Word, b: &Word) -> Word {
    let mut result = [0u8; 32];

    for i in (0..32).rev() {
        if b[i] == 0 {
            continue;
        }

        let mut carry = 0u16;
        for j in (0..32).rev() {
            let k = i + j;
            if k < 31 {
                continue;
            }

            let idx = k - 31;
            if idx >= 32 {
                break;
            }

            let product = a[j] as u16 * b[i] as u16 + result[idx] as u16 + carry;
            result[idx] = product as u8;
            carry = product >> 8;
        }
    }

    result
}

fn word_div(a: &Word, b: &Word) -> Word {
    if word_is_zero(b) {
        return [0u8; 32];
    }

    let a_val = word_to_usize(a) as u64;
    let b_val = word_to_usize(b) as u64;

    if b_val == 0 {
        [0u8; 32]
    } else {
        word_from_u64(a_val / b_val)
    }
}

fn word_lt(a: &Word, b: &Word) -> bool {
    for i in 0..32 {
        if a[i] < b[i] {
            return true;
        }
        if a[i] > b[i] {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();

        let value = word_from_u64(42);
        stack.push(value).unwrap();
        assert_eq!(stack.depth(), 1);

        let popped = stack.pop().unwrap();
        assert_eq!(popped, value);
        assert_eq!(stack.depth(), 0);

        assert_eq!(stack.pop(), Err(ExecError::StackUnderflow));
    }

    #[test]
    fn test_memory_operations() {
        let mut memory = Memory::new();

        let data = vec![1, 2, 3, 4, 5];
        memory.write(0, &data).unwrap();

        let read_data = memory.read(0, 5).unwrap();
        assert_eq!(read_data, data);

        memory.write(100, &data).unwrap();
        assert!(memory.size() >= 105);
    }

    #[test]
    fn test_storage_operations() {
        let mut storage = Storage::new();

        let key = word_from_u64(1);
        let value = word_from_u64(42);

        storage.write(key, value);

        let read_value = storage.read(&key);
        assert_eq!(read_value, value);

        storage.write(key, [0u8; 32]);
        let read_zero = storage.read(&key);
        assert_eq!(read_zero, [0u8; 32]);
    }

    #[test]
    fn test_simple_add() {
        // PUSH1 5, PUSH1 3, ADD, STOP
        let code = vec![0x60, 0x05, 0x60, 0x03, 0x01, 0x00];
        let mut interpreter = Interpreter::new(code, 1000);

        let _ = interpreter.execute();

        let result = interpreter.stack.pop().unwrap();
        assert_eq!(result, word_from_u64(8));
    }

    #[test]
    fn test_jump() {
        // PUSH1 6, JUMP, INVALID, JUMPDEST, PUSH1 42, STOP
        let code = vec![0x60, 0x06, 0x56, 0xfe, 0xfe, 0xfe, 0x5b, 0x60, 0x2a, 0x00];
        let mut interpreter = Interpreter::new(code, 1000);

        let _ = interpreter.execute();

        let result = interpreter.stack.pop().unwrap();
        assert_eq!(result, word_from_u64(42));
    }
}
