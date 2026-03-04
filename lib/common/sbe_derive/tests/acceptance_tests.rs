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
