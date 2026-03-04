//! Acceptance tests for SBE derive macro
//!
//! This test suite validates the implementation against the plan requirements.

use sbe_derive::{SbeDecode, SbeEncode, SbeEnum};
use sbe::{ReadBuf, WriteBuf};

/// Test message header format (8 bytes: blockLength + templateId + schemaId + version)
#[test]
fn test_message_header_format() {
    use sbe::message_header_codec::MessageHeaderDecoder;

    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 100, schema_id = 1, version = 2)]
    struct TestMsg {
        #[sbe(id = 0)]
        value: u64,
    }

    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    // Encode with header
    let encoder = TestMsgEncoder::default().wrap(write_buf, 0);
    let mut header = encoder.header(0);
    let mut encoder = header.parent().expect("Failed to get encoder from header");
    encoder.value(12345);
    // Encoding completes when encoder is dropped

    // Verify header format (8 bytes)
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);

    // Verify: blockLength(2) + templateId(2) + schemaId(2) + version(2)
    assert_eq!(header.template_id(), 100);
    assert_eq!(header.schema_id(), 1);
    assert_eq!(header.version(), 2);
    assert_eq!(header.block_length(), test_msg_encoder::SBE_BLOCK_LENGTH);

    // Verify message body data
    let decoder = TestMsgDecoder::default().header(header, 0);
    assert_eq!(decoder.value(), 12345);
}

/// Test variable-length data encoding/decoding
#[test]
fn test_var_data_encode_decode() {
    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 200, schema_id = 1, version = 1)]
    struct VarDataMsg {
        #[sbe(id = 0)]
        sequence: u64,
        #[sbe(id = 1)]
        payload: Vec<u8>,
    }

    // Test case 1: Empty payload
    {
        let mut buffer = vec![0u8; 1024];
        let write_buf = WriteBuf::new(&mut buffer);

        let mut encoder = VarDataMsgEncoder::default().wrap(write_buf, 0);
        encoder.sequence(100);
        encoder.payload(&[]);
        drop(encoder);

        let read_buf = ReadBuf::new(&buffer);
        let decoder = VarDataMsgDecoder::default().wrap(read_buf, 0, var_data_msg_encoder::SBE_BLOCK_LENGTH, 0);
        assert_eq!(decoder.sequence(), 100);
        assert_eq!(decoder.payload(), Vec::<u8>::new());
    }

    // Test case 2: Small payload
    {
        let mut buffer = vec![0u8; 1024];
        let write_buf = WriteBuf::new(&mut buffer);

        let test_data = b"Hello, SBE!";
        let mut encoder = VarDataMsgEncoder::default().wrap(write_buf, 0);
        encoder.sequence(200);
        encoder.payload(test_data);
        drop(encoder);

        let read_buf = ReadBuf::new(&buffer);
        let decoder = VarDataMsgDecoder::default().wrap(read_buf, 0, var_data_msg_encoder::SBE_BLOCK_LENGTH, 0);
        assert_eq!(decoder.sequence(), 200);
        assert_eq!(decoder.payload(), test_data.to_vec());
    }

    // Test case 3: Large payload (1KB)
    {
        let mut buffer = vec![0u8; 2048];
        let write_buf = WriteBuf::new(&mut buffer);

        let test_data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
        let mut encoder = VarDataMsgEncoder::default().wrap(write_buf, 0);
        encoder.sequence(300);
        encoder.payload(&test_data);
        drop(encoder);

        let read_buf = ReadBuf::new(&buffer);
        let decoder = VarDataMsgDecoder::default().wrap(read_buf, 0, var_data_msg_encoder::SBE_BLOCK_LENGTH, 0);
        assert_eq!(decoder.sequence(), 300);
        assert_eq!(decoder.payload(), test_data);
    }

    // Test case 4: Roundtrip consistency
    {
        let mut buffer = vec![0u8; 1024];
        let write_buf = WriteBuf::new(&mut buffer);

        let original_data = b"Roundtrip test data with special chars: \x00\xFF\xAB\xCD";
        let mut encoder = VarDataMsgEncoder::default().wrap(write_buf, 0);
        encoder.sequence(999);
        encoder.payload(original_data);
        drop(encoder);

        let read_buf = ReadBuf::new(&buffer);
        let decoder = VarDataMsgDecoder::default().wrap(read_buf, 0, var_data_msg_encoder::SBE_BLOCK_LENGTH, 0);
        let decoded_data = decoder.payload();

        assert_eq!(decoded_data.len(), original_data.len());
        assert_eq!(decoded_data, original_data.to_vec());
    }
}

/// Placeholder test for Repeating Groups feature
///
/// This test demonstrates the expected API for repeating groups once fully integrated.
/// Currently ignored because:
/// 1. groups.rs contains generate_group_encoder/generate_group_decoder functions
/// 2. These functions are not integrated into codegen.rs pipeline
/// 3. Vec<T> field detection and group generation needs to be added
///
/// Integration requirements:
/// - Detect Vec<T> fields with #[sbe(group)] attribute
/// - Generate group dimension header (blockLength: u16, numInGroup: u16)
/// - Generate GroupEncoder with add_entry() method
/// - Generate GroupDecoder with count() and iter() methods
/// - Integrate generate_group_encoder/generate_group_decoder into codegen.rs
#[test]
#[ignore = "Repeating groups infrastructure exists but not integrated into codegen pipeline"]
fn test_repeating_groups_placeholder() {
    // Expected schema definition
    #[allow(dead_code)]
    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 300, schema_id = 1, version = 1)]
    struct OrderBook {
        #[sbe(id = 0)]
        symbol: u64,

        // Repeating group: bid levels
        #[sbe(id = 1, group)]
        bids: Vec<BidLevel>,
    }

    #[allow(dead_code)]
    #[derive(SbeEncode, SbeDecode)]
    struct BidLevel {
        #[sbe(id = 0)]
        price: u64,
        #[sbe(id = 1)]
        quantity: u64,
    }

    // Expected encoding workflow:
    // let mut buffer = vec![0u8; 1024];
    // let write_buf = WriteBuf::new(&mut buffer);
    //
    // let mut encoder = OrderBookEncoder::default().wrap(write_buf, 0);
    // encoder.symbol(12345);
    //
    // // Encode repeating group with dimension header
    // let mut bids_encoder = encoder.bids_count(2); // Writes dimension header
    // bids_encoder.next().price(50000).quantity(100);
    // bids_encoder.next().price(49900).quantity(200);
    // drop(encoder);

    // Expected decoding workflow:
    // let read_buf = ReadBuf::new(&buffer);
    // let decoder = OrderBookDecoder::default().wrap(
    //     read_buf, 0, order_book_encoder::SBE_BLOCK_LENGTH, 0
    // );
    //
    // assert_eq!(decoder.symbol(), 12345);
    //
    // let bids_decoder = decoder.bids();
    // assert_eq!(bids_decoder.count(), 2);
    //
    // for (i, bid) in bids_decoder.iter().enumerate() {
    //     match i {
    //         0 => {
    //             assert_eq!(bid.price(), 50000);
    //             assert_eq!(bid.quantity(), 100);
    //         }
    //         1 => {
    //             assert_eq!(bid.price(), 49900);
    //             assert_eq!(bid.quantity(), 200);
    //         }
    //         _ => panic!("Unexpected bid entry"),
    //     }
    // }

    // This test is a placeholder - actual implementation requires:
    // 1. Integration of groups.rs functions into codegen.rs
    // 2. Attribute parsing for #[sbe(group)]
    // 3. Group dimension header encoding (blockLength + numInGroup)
    // 4. GroupEncoder/GroupDecoder generation
    // 5. Iterator support for group entries
}

/// Placeholder test for Nested Messages feature
///
/// This test demonstrates the expected API for nested messages (composite types).
/// Currently ignored because:
/// 1. Composite type encoding/decoding infrastructure needs implementation
/// 2. Nested struct field detection and inline encoding not yet supported
/// 3. No support for #[sbe(composite)] attribute parsing
///
/// Implementation requirements:
/// - Detect nested struct fields with #[sbe(composite)] attribute
/// - Generate inline encoding (no length prefix, fixed layout)
/// - Support nested field access through generated methods
/// - Maintain zero-copy semantics for nested structures
/// - Calculate correct block length including nested types
///
/// Wire format: Nested fields are encoded inline in parent message block
/// Example: Parent { field1: u64, nested: Nested { a: u32, b: u32 } }
/// Layout: [field1: 8 bytes][nested.a: 4 bytes][nested.b: 4 bytes]
#[test]
#[ignore = "Nested messages (composite types) not yet implemented"]
fn test_nested_messages_encode_decode() {
    // Expected schema definition
    #[allow(dead_code)]
    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 400, schema_id = 1, version = 1)]
    struct TradeReport {
        #[sbe(id = 0)]
        trade_id: u64,

        // Nested composite type
        #[sbe(id = 1, composite)]
        price_qty: PriceQuantity,

        #[sbe(id = 2)]
        timestamp: u64,
    }

    #[allow(dead_code)]
    #[derive(SbeEncode, SbeDecode)]
    struct PriceQuantity {
        #[sbe(id = 0)]
        price: u64,
        #[sbe(id = 1)]
        quantity: u64,
    }

    // Expected encoding workflow:
    // let mut buffer = vec![0u8; 1024];
    // let write_buf = WriteBuf::new(&mut buffer);
    //
    // let mut encoder = TradeReportEncoder::default().wrap(write_buf, 0);
    // encoder.trade_id(123456);
    //
    // // Inline composite encoding
    // let mut price_qty = encoder.price_qty();
    // price_qty.price(50000);
    // price_qty.quantity(100);
    //
    // encoder.timestamp(1234567890);
    // drop(encoder);

    // Expected decoding workflow:
    // let read_buf = ReadBuf::new(&buffer);
    // let decoder = TradeReportDecoder::default().wrap(
    //     read_buf, 0, trade_report_encoder::SBE_BLOCK_LENGTH, 0
    // );
    //
    // assert_eq!(decoder.trade_id(), 123456);
    //
    // // Inline composite decoding
    // let price_qty = decoder.price_qty();
    // assert_eq!(price_qty.price(), 50000);
    // assert_eq!(price_qty.quantity(), 100);
    //
    // assert_eq!(decoder.timestamp(), 1234567890);

    // Wire format verification:
    // Offset 0-7:   trade_id (u64)
    // Offset 8-15:  price_qty.price (u64)
    // Offset 16-23: price_qty.quantity (u64)
    // Offset 24-31: timestamp (u64)
    // Total block length: 32 bytes
}

/// Placeholder test for Decimal Types feature
///
/// This test demonstrates the expected API for decimal encoding (mantissa/exponent).
/// Currently ignored because:
/// 1. Decimal type representation not implemented
/// 2. No support for mantissa/exponent encoding
/// 3. Missing #[sbe(decimal)] attribute parsing
///
/// Implementation requirements:
/// - Support Decimal type with configurable precision
/// - Encode as mantissa (i64) + exponent (i8) pair
/// - Generate accessor methods that return Decimal wrapper type
/// - Provide conversion from/to f64 with precision control
/// - Handle special values (NaN, Infinity) according to SBE spec
///
/// Wire format: [mantissa: 8 bytes (i64)][exponent: 1 byte (i8)]
/// Example: 123.45 with exponent -2 → mantissa=12345, exponent=-2
/// Calculation: value = mantissa × 10^exponent
#[test]
#[ignore = "Decimal types (mantissa/exponent encoding) not yet implemented"]
fn test_decimal_types_encode_decode() {
    // Expected schema definition
    #[allow(dead_code)]
    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 500, schema_id = 1, version = 1)]
    struct PriceUpdate {
        #[sbe(id = 0)]
        symbol_id: u64,

        // Decimal field with mantissa/exponent encoding
        #[sbe(id = 1, decimal, exponent = -8)]
        price: Decimal,

        #[sbe(id = 2, decimal, exponent = -4)]
        quantity: Decimal,
    }

    // Expected encoding workflow:
    // let mut buffer = vec![0u8; 1024];
    // let write_buf = WriteBuf::new(&mut buffer);
    //
    // let mut encoder = PriceUpdateEncoder::default().wrap(write_buf, 0);
    // encoder.symbol_id(12345);
    //
    // // Encode decimal: 50000.12345678 with exponent -8
    // encoder.price(Decimal::new(5000012345678, -8));
    //
    // // Encode decimal: 100.5000 with exponent -4
    // encoder.quantity(Decimal::new(1005000, -4));
    // drop(encoder);

    // Expected decoding workflow:
    // let read_buf = ReadBuf::new(&buffer);
    // let decoder = PriceUpdateDecoder::default().wrap(
    //     read_buf, 0, price_update_encoder::SBE_BLOCK_LENGTH, 0
    // );
    //
    // assert_eq!(decoder.symbol_id(), 12345);
    //
    // let price = decoder.price();
    // assert_eq!(price.mantissa(), 5000012345678);
    // assert_eq!(price.exponent(), -8);
    // assert_eq!(price.to_f64(), 50000.12345678);
    //
    // let quantity = decoder.quantity();
    // assert_eq!(quantity.mantissa(), 1005000);
    // assert_eq!(quantity.exponent(), -4);
    // assert_eq!(quantity.to_f64(), 100.5000);

    // Wire format verification:
    // Offset 0-7:   symbol_id (u64)
    // Offset 8-15:  price.mantissa (i64)
    // Offset 16:    price.exponent (i8)
    // Offset 17-24: quantity.mantissa (i64)
    // Offset 25:    quantity.exponent (i8)
    // Total block length: 26 bytes
}

/// Placeholder test for Time Types feature
///
/// This test demonstrates the expected API for timestamp encoding.
/// Currently ignored because:
/// 1. Time type representation not implemented
/// 2. No support for UTC timestamp encoding
/// 3. Missing #[sbe(timestamp)] attribute parsing
///
/// Implementation requirements:
/// - Support timestamp types with configurable time units
/// - Encode as u64 nanoseconds since Unix epoch (UTC)
/// - Generate accessor methods that return timestamp wrapper type
/// - Provide conversion from/to standard time types (SystemTime, DateTime)
/// - Support different time units (nanos, micros, millis, seconds)
///
/// Wire format: [timestamp: 8 bytes (u64)] - nanoseconds since Unix epoch
/// Example: 2024-01-01 00:00:00 UTC → 1704067200000000000 nanoseconds
/// Time unit conversion handled by wrapper type
#[test]
#[ignore = "Time types (UTC timestamp encoding) not yet implemented"]
fn test_time_types_encode_decode() {
    // Expected schema definition
    #[allow(dead_code)]
    #[derive(SbeEncode, SbeDecode)]
    #[sbe(template_id = 600, schema_id = 1, version = 1)]
    struct OrderEvent {
        #[sbe(id = 0)]
        order_id: u64,

        // Timestamp field with nanosecond precision
        #[sbe(id = 1, timestamp, unit = "nanosecond")]
        created_at: Timestamp,

        // Timestamp field with millisecond precision
        #[sbe(id = 2, timestamp, unit = "millisecond")]
        updated_at: Timestamp,
    }

    // Expected encoding workflow:
    // let mut buffer = vec![0u8; 1024];
    // let write_buf = WriteBuf::new(&mut buffer);
    //
    // let mut encoder = OrderEventEncoder::default().wrap(write_buf, 0);
    // encoder.order_id(789);
    //
    // // Encode timestamp in nanoseconds
    // encoder.created_at(Timestamp::from_nanos(1704067200000000000));
    //
    // // Encode timestamp in milliseconds (auto-converted to nanos)
    // encoder.updated_at(Timestamp::from_millis(1704067200000));
    // drop(encoder);

    // Expected decoding workflow:
    // let read_buf = ReadBuf::new(&buffer);
    // let decoder = OrderEventDecoder::default().wrap(
    //     read_buf, 0, order_event_encoder::SBE_BLOCK_LENGTH, 0
    // );
    //
    // assert_eq!(decoder.order_id(), 789);
    //
    // let created_at = decoder.created_at();
    // assert_eq!(created_at.as_nanos(), 1704067200000000000);
    //
    // let updated_at = decoder.updated_at();
    // assert_eq!(updated_at.as_millis(), 1704067200000);
    // assert_eq!(updated_at.as_nanos(), 1704067200000000000);

    // Wire format verification:
    // Offset 0-7:   order_id (u64)
    // Offset 8-15:  created_at (u64 nanoseconds)
    // Offset 16-23: updated_at (u64 nanoseconds)
    // Total block length: 24 bytes
}
