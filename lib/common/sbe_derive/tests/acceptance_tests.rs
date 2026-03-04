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
