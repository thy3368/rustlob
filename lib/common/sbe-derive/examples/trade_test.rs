//! Test for Trade message using derive macros

use sbe::{ReadBuf, WriteBuf};

#[test]
fn test_trade_encode_decode() {
    let mut buffer = vec![0u8; 1024];

    // Encode
    let write_buf = WriteBuf::new(&mut buffer);
    let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

    encoder.trade_id(12345);
    encoder.symbol(b'A');
    encoder.price(100.50);
    encoder.quantity(1000);

    // Decode
    let read_buf = ReadBuf::new(&buffer);
    let decoder = TradeDecoder::default().wrap(
        read_buf,
        0,
        SBE_BLOCK_LENGTH,
        0,
    );

    assert_eq!(decoder.trade_id(), 12345);
    assert_eq!(decoder.symbol(), b'A');
    assert_eq!(decoder.price(), 100.50);
    assert_eq!(decoder.quantity(), 1000);
}

#[test]
fn test_trade_with_header() {
    let mut buffer = vec![0u8; 1024];

    // Encode with header
    let write_buf = WriteBuf::new(&mut buffer);
    let encoder = TradeEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(99999);
    encoder.symbol(b'B');
    encoder.price(250.75);
    encoder.quantity(500);

    // Decode with header
    let read_buf = ReadBuf::new(&buffer);
    let header = sbe::message_header_codec::MessageHeaderDecoder::default()
        .wrap(read_buf, 0);

    assert_eq!(header.template_id(), SBE_TEMPLATE_ID);
    assert_eq!(header.schema_id(), sbe::SBE_SCHEMA_ID);
    assert_eq!(header.version(), sbe::SBE_SCHEMA_VERSION);

    let decoder = TradeDecoder::default().header(header, 0);

    assert_eq!(decoder.trade_id(), 99999);
    assert_eq!(decoder.symbol(), b'B');
    assert_eq!(decoder.price(), 250.75);
    assert_eq!(decoder.quantity(), 500);
}

#[test]
fn test_trade_constants() {
    assert_eq!(SBE_BLOCK_LENGTH, 21);
    assert_eq!(SBE_TEMPLATE_ID, 1);
}
