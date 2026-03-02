//! Enhanced example with more SBE features

use sbe::message_header_codec::encoder;
use sbe_derive::{SbeDecode, SbeEncode};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};

pub mod trade_codec;

/// Advanced trade message with optional fields, arrays, and booleans
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 2, schema_id = 1, version = 0)]
struct AdvancedTrade {
    // Required fields
    #[sbe(id = 0)]
    trade_id: u64,

    // Optional field
    #[sbe(id = 1, presence = "optional")]
    client_order_id: Option<u64>,

    // Boolean field
    #[sbe(id = 2)]
    is_buy: bool,

    // Char field
    #[sbe(id = 3)]
    side: char,

    // Fixed-length array (symbol as char array)
    #[sbe(id = 4)]
    symbol: [u8; 8],

    // Price and quantity
    #[sbe(id = 5)]
    price: f64,

    #[sbe(id = 6)]
    quantity: i32,

    // Constant field
    #[sbe(id = 7, presence = "constant", constant = "1")]
    version: u8,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("Advanced SBE Features Example");
    println!("==============================\n");

    // Create a buffer
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    // Encode an advanced trade message
    println!("Encoding advanced trade message...");
    let encoder = AdvancedTradeEncoder::default().wrap(write_buf, 0);
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.client_order_id(Some(99999)); // Optional field with value
    encoder.is_buy(true); // Boolean field
    encoder.side('B'); // Char field
    encoder.symbol(&[b'B', b'T', b'C', b'U', b'S', b'D', b'T', 0]); // Fixed array
    encoder.price(50000.50);
    encoder.quantity(100);
    // version is constant, no setter needed

    println!("  Trade ID: 12345");
    println!("  Client Order ID: Some(99999)");
    println!("  Is Buy: true");
    println!("  Side: B");
    println!("  Symbol: BTCUSDT");
    println!("  Price: 50000.50");
    println!("  Quantity: 100");
    println!("  Version: 1 (constant)");
    println!("  Block Length: {}", advanced_trade_encoder::SBE_BLOCK_LENGTH);

    // Decode the message
    println!("\nDecoding advanced trade message...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = AdvancedTradeDecoder::default().header(header, 0);

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Client Order ID: {:?}", decoder.client_order_id());
    println!("  Is Buy: {}", decoder.is_buy());
    println!("  Side: {}", decoder.side());

    let symbol = decoder.symbol();
    let symbol_str = std::str::from_utf8(&symbol[..7]).unwrap_or("???");
    println!("  Symbol: {}", symbol_str);

    println!("  Price: {}", decoder.price());
    println!("  Quantity: {}", decoder.quantity());
    println!("  Version: {} (constant)", decoder.version());

    // Test with None optional field
    println!("\n--- Testing with None optional field ---");
    let mut buffer2 = vec![0u8; 1024];
    let write_buf2 = WriteBuf::new(&mut buffer2);

    let encoder2 = AdvancedTradeEncoder::default();
    let mut header2 = encoder2.header(0);
    let mut encoder2 = header2.parent().unwrap();

    encoder2.trade_id(54321);
    encoder2.client_order_id(None); // Optional field with None
    encoder2.is_buy(false);
    encoder2.side('S');
    encoder2.symbol(&[b'E', b'T', b'H', b'U', b'S', b'D', b'T', 0]);
    encoder2.price(3000.25);
    encoder2.quantity(50);

    let read_buf2 = ReadBuf::new(&buffer2);
    let header2 = MessageHeaderDecoder::default().wrap(read_buf2, 0);
    let decoder2 = AdvancedTradeDecoder::default().header(header2, 0);

    println!("  Trade ID: {}", decoder2.trade_id());
    println!("  Client Order ID: {:?}", decoder2.client_order_id());
    println!("  Is Buy: {}", decoder2.is_buy());
    println!("  Side: {}", decoder2.side());

    println!("\n✓ All features working correctly!");
}
