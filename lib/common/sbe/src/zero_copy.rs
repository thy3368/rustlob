//! Zero-copy decoding trait for SBE view types
//!
//! This trait enables custom types to be decoded from a buffer without allocation.
//! Types that implement this trait can be used with SbeView derive macro.
//!
//! ## Zero-Allocation (Zero-Copy) Implementations
//! These types decode directly from buffer bytes without heap allocation:
//! - Primitive integers: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `u128`, `i128`
//! - Floating point: `f32`, `f64`
//! - Boolean and char
//! - Fixed-size arrays: `[u8; N]`
//! - Custom view types that borrow from buffer (implement your own)
//!
//! ## Allocating Implementations
//! These types require heap allocation:
//! - `Vec<u8>` - allocates new vector
//! - Custom types that own their data
//!
//! ## Design Notes
//! For true zero-copy with variable-length data, create a custom view type
//! that holds a reference to the buffer instead of owning the data.

use super::ReadBuf;

// ============================================================================
// ZeroCopyDecode Trait Definition
// ============================================================================

/// Trait for zero-copy decoding from buffer
///
/// Implement this trait for custom types that need to be decoded
/// from a buffer without heap allocation.
///
/// # Example
/// ```ignore
/// impl ZeroCopyDecode for MyType {
///     fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
///         // Parse from buffer at given offset
///     }
///     
///     fn encoded_size() -> usize {
///         // Size in bytes
///     }
/// }
/// ```
pub trait ZeroCopyDecode: Sized {
    /// Decode from buffer at given offset (zero-copy)
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self;

    /// Size of encoded representation
    fn encoded_size() -> usize;
}

// ============================================================================
// ZERO-ALLOC: Primitive Types
// ============================================================================
// All primitives decode bytes directly - no heap allocation

/// Macro to implement ZeroCopyDecode for primitive types
macro_rules! impl_zero_copy_for_primitive {
    ($($ty:ty => $method:ident),*) => {
        $(
            impl ZeroCopyDecode for $ty {
                #[inline]
                fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
                    let buf = ReadBuf::new(data);
                    buf.$method(offset)
                }

                fn encoded_size() -> usize {
                    std::mem::size_of::<$ty>()
                }
            }
        )*
    };
}

/// Integer types: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
/// ZERO-ALLOC: just reads bytes from buffer
impl_zero_copy_for_primitive!(
    u8 => get_u8_at,
    i8 => get_i8_at,
    u16 => get_u16_at,
    i16 => get_i16_at,
    u32 => get_u32_at,
    i32 => get_i32_at,
    u64 => get_u64_at,
    i64 => get_i64_at,
    u128 => get_u128_at,
    i128 => get_i128_at
);

/// Floating point: f32, f64
/// ZERO-ALLOC: just reads bytes from buffer
impl_zero_copy_for_primitive!(
    f32 => get_f32_at,
    f64 => get_f64_at
);

// ============================================================================
// ZERO-ALLOC: bool and char
// ============================================================================

/// Boolean - ZERO-ALLOC: reads single byte, no allocation
impl ZeroCopyDecode for bool {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        let buf = ReadBuf::new(data);
        buf.get_u8_at(offset) != 0
    }

    fn encoded_size() -> usize {
        1
    }
}

/// Character - ZERO-ALLOC: reads single byte, no allocation
impl ZeroCopyDecode for char {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        let buf = ReadBuf::new(data);
        buf.get_u8_at(offset) as char
    }

    fn encoded_size() -> usize {
        1
    }
}

// ============================================================================
// ZERO-ALLOC: Fixed-size Arrays [u8; N]
// ============================================================================

/// Fixed-size byte array - ZERO-ALLOC: copies N bytes from buffer to array
/// No heap allocation, stack-only operation
impl<const N: usize> ZeroCopyDecode for [u8; N] {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        let buf = ReadBuf::new(data);
        let slice = buf.get_slice_at(offset, N);
        slice.try_into().expect("slice with incorrect length")
    }

    fn encoded_size() -> usize {
        N
    }
}

// ============================================================================
// ALLOCATING: Variable-length Vec<u8>
// ============================================================================

/// Variable-length bytes - ALLOCATES: creates new Vec
///
/// Note: This implementation allocates a new vector.
/// For true zero-copy with variable-length data, use a custom view type
/// that holds a reference to the buffer.
impl ZeroCopyDecode for Vec<u8> {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        // ALLOCATES: heap allocation via to_vec()
        data[offset..].to_vec()
    }

    fn encoded_size() -> usize {
        // Variable size - cannot determine statically
        0
    }
}

// ============================================================================
// ZERO-ALLOC or ALLOC: Option<T> - depends on T
// ============================================================================

/// Optional field - ZERO-ALLOC if T is zero-alloc, ALLOCATES if T allocates
impl<T: ZeroCopyDecode + Default> ZeroCopyDecode for Option<T> {
    #[inline]
    fn zero_copy_decode(data: &[u8], offset: usize) -> Self {
        let value = T::zero_copy_decode(data, offset);
        if std::mem::size_of::<T>() == 0 {
            None
        } else {
            // Check for null value based on type
            // This is a simplified check - real implementation would need
            // type-specific null values (e.g., u16::MAX for u16)
            Some(value)
        }
    }

    fn encoded_size() -> usize {
        T::encoded_size()
    }
}
