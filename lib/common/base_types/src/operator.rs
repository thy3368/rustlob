//! 算子定义
//!
//! 设计原则：
//! - 纯CPU操作：无IO、无数据库、无网络
//! - 无副作用：输入 -> 输出，函数式编程
//! - 标量/向量：算子可以有标量实现或向量实现（或两者都有）
//! - SIMD友好：向量实现可利用SIMD优化
//!
//! # 实现方式（类型别名 + 模块组织）
//!
//! 使用类型别名在签名上表达"这是算子"：
//!
//! ```rust,ignore
//! // 定义算子类型
//! pub type Operator<I, O, E> = fn(&I) -> Result<O, E>;
//! pub type VectorOp<I, O, E> = fn(&[I]) -> Result<Vec<O>, E>;
//! pub type SoaOp<IS, OS, E> = fn(&IS) -> Result<OS, E>;
//!
//! // 定义算子常量（签名上明确表达"这是算子"）
//! pub const DOUBLE: Operator<i64, i64, Infallible> = double;
//! pub const TRIPLE: Operator<i64, i64, Infallible> = triple;
//!
//! fn double(input: &i64) -> Result<i64, Infallible> {
//!     Ok(input * 2)
//! }
//!
//! fn triple(input: &i64) -> Result<i64, Infallible> {
//!     Ok(input * 3)
//! }
//!
//! // 调用：通过常量调用，语义清晰
//! let result1 = DOUBLE(&5)?;
//! let result2 = TRIPLE(&5)?;
//! ```

// ============================================================================
// 算子类型定义
// ============================================================================

/// 标量算子类型
///
/// 在签名上明确表达"这是一个算子"
pub type Operator<I, O, E = std::convert::Infallible> = fn(&I) -> Result<O, E>;

/// 向量算子类型
///
/// 批量处理算子
pub type VectorOp<I, O, E = std::convert::Infallible> = fn(&[I]) -> Result<Vec<O>, E>;

/// SoA算子类型
///
/// 针对Structure of Arrays优化的算子
pub type SoaOp<IS, OS, E = std::convert::Infallible> = fn(&IS) -> Result<OS, E>;

// ============================================================================
// 示例：展示如何在签名上表达算子
// ============================================================================

#[cfg(test)]
mod examples {
    use super::*;

    // 定义算子函数
    fn double(input: &i64) -> Result<i64, std::convert::Infallible> {
        Ok(input * 2)
    }

    fn triple(input: &i64) -> Result<i64, std::convert::Infallible> {
        Ok(input * 3)
    }

    fn square(input: &i64) -> Result<i64, std::convert::Infallible> {
        Ok(input * input)
    }

    fn batch_double(inputs: &[i64]) -> Result<Vec<i64>, std::convert::Infallible> {
        Ok(inputs.iter().map(|x| x * 2).collect())
    }

    // 使用类型别名定义算子常量（签名上明确表达"这是算子"）
    pub const DOUBLE: Operator<i64, i64> = double;
    pub const TRIPLE: Operator<i64, i64> = triple;
    pub const SQUARE: Operator<i64, i64> = square;
    pub const BATCH_DOUBLE: VectorOp<i64, i64> = batch_double;

    #[test]
    fn test_operator_constants() {
        // 通过常量调用，语义清晰
        assert_eq!(DOUBLE(&5).unwrap(), 10);
        assert_eq!(TRIPLE(&5).unwrap(), 15);
        assert_eq!(SQUARE(&5).unwrap(), 25);
        assert_eq!(BATCH_DOUBLE(&[1, 2, 3]).unwrap(), vec![2, 4, 6]);
    }

    #[test]
    fn test_direct_call() {
        // 也可以直接调用函数
        assert_eq!(double(&5).unwrap(), 10);
        assert_eq!(triple(&5).unwrap(), 15);
    }

    // 方式2: 使用模块组织算子
    pub mod operators {
        use super::*;

        pub const ADD: Operator<(i64, i64), i64> = add;
        pub const MULTIPLY: Operator<(i64, i64), i64> = multiply;

        fn add(input: &(i64, i64)) -> Result<i64, std::convert::Infallible> {
            Ok(input.0 + input.1)
        }

        fn multiply(input: &(i64, i64)) -> Result<i64, std::convert::Infallible> {
            Ok(input.0 * input.1)
        }
    }

    #[test]
    fn test_module_operators() {
        assert_eq!(operators::ADD(&(5, 3)).unwrap(), 8);
        assert_eq!(operators::MULTIPLY(&(5, 3)).unwrap(), 15);
    }

    // 方式3: 不同类型的算子
    fn double_f64(input: &f64) -> Result<f64, std::convert::Infallible> {
        Ok(input * 2.0)
    }

    pub const DOUBLE_F64: Operator<f64, f64> = double_f64;

    #[test]
    fn test_typed_operators() {
        assert_eq!(DOUBLE_F64(&5.0).unwrap(), 10.0);
    }
}

