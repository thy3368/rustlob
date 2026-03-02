//! Comprehensive test suite for SBE derive macros

use sbe_derive::{SbeDecode, SbeEncode, SbeEnum};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};

/// Test enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, SbeEnum)]
enum TestSide {
    Buy,
    Sell,
}

/// Comprehensive test message with all features
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 100, schema_id = 1, version = 2)]
struct TestMessage {
    // Primitive types
    #[sbe(id = 0)]
    field_u8: u8,

    #[sbe(id = 1)]
    field_u16: u16,

    #[sbe(id = 2)]
    field_u32: u32,

    #[sbe(id = 3)]
    field_u64: u64,

    #[sbe(id = 4)]
    field_i8: i8,

    #[sbe(id = 5)]
    field_i16: i16,

    #[sbe(id = 6)]
    field_i32: i32,

    #[sbe(id = 7)]
    field_i64: i64,

    #[sbe(id = 8)]
    field_f32: f32,

    #[sbe(id = 9)]
    field_f64: f64,

    // Boolean and char
    #[sbe(id = 10)]
    field_bool: bool,

    #[sbe(id = 11)]
    field_char: char,

    // Optional field
    #[sbe(id = 12, presence = "optional")]
    field_optional: Option<u64>,

    // Fixed array
    #[sbe(id = 13)]
    field_array: [u8; 8],

    // Constant field
    #[sbe(id = 14, presence = "constant", constant = "42")]
    field_constant: u8,

    // Enum (encoded as u8)
    #[sbe(id = 15)]
    field_enum: u8,

    // Version field
    #[sbe(id = 16, since_version = 1)]
    field_v1: u32,

    // Version 2 field
    #[sbe(id = 17, since_version = 2)]
    field_v2: u32,
}

#[test]
fn test_primitive_types() {
    use sbe::{ReadBuf, WriteBuf};

    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    // Encode
    let mut encoder = TestMessageEncoder::default().wrap(write_buf, 0);
    encoder.field_u8(255);
    encoder.field_u16(65535);
    encoder.field_u32(4294967295);
    encoder.field_u64(18446744073709551615);
    encoder.field_i8(-128);
    encoder.field_i16(-32768);
    encoder.field_i32(-2147483648);
    encoder.field_i64(-9223372036854775808);
    encoder.field_f32(3.14159);
    encoder.field_f64(2.71828);
    encoder.field_bool(true);
    encoder.field_char('A');
    encoder.field_optional(Some(12345));
    encoder.field_array(&[1, 2, 3, 4, 5, 6, 7, 8]);
    encoder.field_enum(TestSide::Buy.to_u8());
    encoder.field_v1(100);
    encoder.field_v2(200);

    // Decode
    let read_buf = ReadBuf::new(&buffer);
    let decoder = TestMessageDecoder::default().wrap(
        read_buf,
        0,
        test_message_encoder::SBE_BLOCK_LENGTH,
        2, // version 2
    );

    // Verify
    assert_eq!(decoder.field_u8(), 255);
    assert_eq!(decoder.field_u16(), 65535);
    assert_eq!(decoder.field_u32(), 4294967295);
    assert_eq!(decoder.field_u64(), 18446744073709551615);
    assert_eq!(decoder.field_i8(), -128);
    assert_eq!(decoder.field_i16(), -32768);
    assert_eq!(decoder.field_i32(), -2147483648);
    assert_eq!(decoder.field_i64(), -9223372036854775808);
    assert!((decoder.field_f32() - 3.14159).abs() < 0.0001);
    assert!((decoder.field_f64() - 2.71828).abs() < 0.00001);
    assert_eq!(decoder.field_bool(), true);
    assert_eq!(decoder.field_char(), 'A');
    assert_eq!(decoder.field_optional(), Some(12345));
    assert_eq!(decoder.field_array(), [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(decoder.field_constant(), 42);
    assert_eq!(decoder.field_enum(), TestSide::Buy.to_u8());
    assert_eq!(decoder.field_v1(), Some(100));
    assert_eq!(decoder.field_v2(), Some(200));
}

#[test]
fn test_optional_none() {
    use sbe::{ReadBuf, WriteBuf};

    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let mut encoder = TestMessageEncoder::default().wrap(write_buf, 0);
    encoder.field_optional(None);

    let read_buf = ReadBuf::new(&buffer);
    let decoder = TestMessageDecoder::default().wrap(
        read_buf,
        0,
        test_message_encoder::SBE_BLOCK_LENGTH,
        0,
    );

    assert_eq!(decoder.field_optional(), None);
}

#[test]
fn test_version_fields() {
    use sbe::{ReadBuf, WriteBuf};

    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let mut encoder = TestMessageEncoder::default().wrap(write_buf, 0);
    encoder.field_v1(100);
    encoder.field_v2(200);

    // Decode with version 0 (no version fields available)
    let read_buf = ReadBuf::new(&buffer);
    let decoder_v0 = TestMessageDecoder::default().wrap(
        read_buf,
        0,
        test_message_encoder::SBE_BLOCK_LENGTH,
        0,
    );

    assert_eq!(decoder_v0.field_v1(), None);
    assert_eq!(decoder_v0.field_v2(), None);

    // Decode with version 1 (v1 available, v2 not)
    let read_buf = ReadBuf::new(&buffer);
    let decoder_v1 = TestMessageDecoder::default().wrap(
        read_buf,
        0,
        test_message_encoder::SBE_BLOCK_LENGTH,
        1,
    );

    assert_eq!(decoder_v1.field_v1(), Some(100));
    assert_eq!(decoder_v1.field_v2(), None);

    // Decode with version 2 (both available)
    let read_buf = ReadBuf::new(&buffer);
    let decoder_v2 = TestMessageDecoder::default().wrap(
        read_buf,
        0,
        test_message_encoder::SBE_BLOCK_LENGTH,
        2,
    );

    assert_eq!(decoder_v2.field_v1(), Some(100));
    assert_eq!(decoder_v2.field_v2(), Some(200));
}

#[test]
fn test_enum_conversion() {
    assert_eq!(TestSide::Buy.to_u8(), 0);
    assert_eq!(TestSide::Sell.to_u8(), 1);
    assert_eq!(TestSide::from_u8(0), TestSide::Buy);
    assert_eq!(TestSide::from_u8(1), TestSide::Sell);
}

#[test]
fn test_constant_field() {
    use sbe::{ReadBuf, WriteBuf};

    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let _encoder = TestMessageEncoder::default().wrap(write_buf, 0);
    // Constant field has no setter

    let read_buf = ReadBuf::new(&buffer);
    let decoder = TestMessageDecoder::default().wrap(
        read_buf,
        0,
        test_message_encoder::SBE_BLOCK_LENGTH,
        0,
    );

    // Constant field always returns the constant value
    assert_eq!(decoder.field_constant(), 42);
}
