use sbe_derive::{SbeDecode, SbeEncode};

#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 1, schema_id = 1, version = 0)]
struct Trade {
    #[sbe(id = 0)]
    trade_id: u64,
    #[sbe(id = 1)]
    symbol: u8,
    #[sbe(id = 2)]
    price: f64,
    #[sbe(id = 3)]
    quantity: i32,
}

#[test]
fn test_basic_encode_decode() {
    use sbe::{ReadBuf, WriteBuf};

    // Create a buffer
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    // Create encoder and encode message
    let mut encoder = TradeEncoder::default().wrap(write_buf, 0);
    encoder.trade_id(12345);
    encoder.symbol(65); // 'A'
    encoder.price(100.50);
    encoder.quantity(1000);

    // Create decoder and decode message
    let read_buf = ReadBuf::new(&buffer);
    let decoder = TradeDecoder::default().wrap(read_buf, 0, encoder::SBE_BLOCK_LENGTH, 0);

    // Verify decoded values
    assert_eq!(decoder.trade_id(), 12345);
    assert_eq!(decoder.symbol(), 65);
    assert_eq!(decoder.price(), 100.50);
    assert_eq!(decoder.quantity(), 1000);
}

#[test]
fn test_with_message_header() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    // Create a buffer
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    // Encode with header
    let encoder = TradeEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();
    encoder.trade_id(99999);
    encoder.symbol(66); // 'B'
    encoder.price(250.75);
    encoder.quantity(500);

    // Decode with header
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = TradeDecoder::default().header(header, 0);

    // Verify decoded values
    assert_eq!(decoder.trade_id(), 99999);
    assert_eq!(decoder.symbol(), 66);
    assert_eq!(decoder.price(), 250.75);
    assert_eq!(decoder.quantity(), 500);
}
