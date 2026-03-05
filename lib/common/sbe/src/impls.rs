//! SBE 编解码标准类型实现
//!
//! 为 Rust 标准库类型提供 SbeEncode/SbeDecode 实现。
//! 参考 serde 的 impls.rs 设计模式。


// ============================================================================
// 原始类型 (Primitive Types)
// ============================================================================

use crate::codec::codec::{SbeDecode, SbeDecoder, SbeEncode, SbeEncoder};

macro_rules! primitive_impl {
    ($ty:ident, $encode_method:ident, $decode_method:ident) => {
        impl SbeEncode for $ty {
            #[inline]
            fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
                encoder.$encode_method(*self)
            }
        }

        impl SbeDecode for $ty {
            #[inline]
            fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
                decoder.$decode_method()
            }
        }
    };
}

primitive_impl!(bool, encode_bool, decode_bool);
primitive_impl!(i8, encode_i8, decode_i8);
primitive_impl!(i16, encode_i16, decode_i16);
primitive_impl!(i32, encode_i32, decode_i32);
primitive_impl!(i64, encode_i64, decode_i64);
primitive_impl!(i128, encode_i128, decode_i128);
primitive_impl!(u8, encode_u8, decode_u8);
primitive_impl!(u16, encode_u16, decode_u16);
primitive_impl!(u32, encode_u32, decode_u32);
primitive_impl!(u64, encode_u64, decode_u64);
primitive_impl!(u128, encode_u128, decode_u128);
primitive_impl!(f32, encode_f32, decode_f32);
primitive_impl!(f64, encode_f64, decode_f64);
primitive_impl!(char, encode_char, decode_char);

// isize/usize 映射到 i64/u64（跨平台兼容）
impl SbeEncode for isize {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_i64(*self as i64)
    }
}

impl SbeDecode for isize {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(decoder.decode_i64()? as isize)
    }
}

impl SbeEncode for usize {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_u64(*self as u64)
    }
}

impl SbeDecode for usize {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(decoder.decode_u64()? as usize)
    }
}

// ============================================================================
// 字符串类型 (String Types)
// ============================================================================

// str 映射到 variable-length data
impl SbeEncode for str {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self)
    }
}

// String 映射到 variable-length data
impl SbeEncode for String {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_str(self.as_str())
    }
}

impl SbeDecode for String {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(decoder.decode_str()?.to_string())
    }
}

// ============================================================================
// 引用类型 (Reference Types)
// ============================================================================

impl<T: SbeEncode + ?Sized> SbeEncode for &T {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        (**self).sbe_encode(encoder)
    }
}

impl<T: SbeEncode + ?Sized> SbeEncode for &mut T {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        (**self).sbe_encode(encoder)
    }
}

// ============================================================================
// Option 类型
// ============================================================================

impl<T: SbeEncode> SbeEncode for Option<T> {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self {
            Some(v) => {
                encoder.encode_u8(1)?; // presence flag
                v.sbe_encode(encoder)
            }
            None => encoder.encode_u8(0), // null
        }
    }
}

impl<T: SbeDecode> SbeDecode for Option<T> {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        let flag = decoder.decode_u8()?;
        if flag == 0 {
            Ok(None)
        } else {
            Ok(Some(T::sbe_decode(decoder)?))
        }
    }
}

// ============================================================================
// Result 类型
// ============================================================================

impl<T: SbeEncode, E: SbeEncode> SbeEncode for Result<T, E> {
    #[inline]
    fn sbe_encode<Enc: SbeEncoder>(&self, encoder: &mut Enc) -> Result<(), Enc::Error> {
        match self {
            Ok(v) => {
                encoder.encode_u8(0)?; // Ok variant
                v.sbe_encode(encoder)
            }
            Err(e) => {
                encoder.encode_u8(1)?; // Err variant
                e.sbe_encode(encoder)
            }
        }
    }
}

impl<T: SbeDecode, E: SbeDecode> SbeDecode for Result<T, E> {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        let variant = decoder.decode_u8()?;
        if variant == 0 {
            Ok(Ok(T::sbe_decode(decoder)?))
        } else {
            Ok(Err(E::sbe_decode(decoder)?))
        }
    }
}

// ============================================================================
// 数组类型 (Array Types)
// ============================================================================

// 固定长度字节数组 [u8; N] - 直接编码，无长度前缀
impl<const N: usize> SbeEncode for [u8; N] {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_array(self)
    }
}

impl<const N: usize> SbeDecode for [u8; N] {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        decoder.decode_array::<N>()
    }
}

// ============================================================================
// 切片类型 (Slice Types)
// ============================================================================

// [u8] 切片 - 编码为可变长度字节数组
impl SbeEncode for [u8] {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        encoder.encode_bytes(self)
    }
}

// ============================================================================
// Vec 类型
// ============================================================================

// Vec<T> - 泛型向量支持
impl<T: SbeEncode> SbeEncode for Vec<T> {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        // 编码长度前缀
        encoder.encode_u16(self.len() as u16)?;
        // 编码每个元素
        for item in self {
            item.sbe_encode(encoder)?;
        }
        Ok(())
    }
}

impl<T: SbeDecode> SbeDecode for Vec<T> {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        // 解码长度前缀
        let len = decoder.decode_u16()? as usize;
        // 解码每个元素
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::sbe_decode(decoder)?);
        }
        Ok(vec)
    }
}

// ============================================================================
// Box 类型
// ============================================================================

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: SbeEncode + ?Sized> SbeEncode for Box<T> {
    #[inline]
    fn sbe_encode<E: SbeEncoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        (**self).sbe_encode(encoder)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: SbeDecode> SbeDecode for Box<T> {
    #[inline]
    fn sbe_decode<'de, D: SbeDecoder<'de>>(decoder: &mut D) -> Result<Self, D::Error> {
        Ok(Box::new(T::sbe_decode(decoder)?))
    }
}
