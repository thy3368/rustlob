//! Example demonstrating enum support

use sbe_derive::{SbeDecode, SbeEncode, SbeEnum};
use sbe::{ReadBuf, WriteBuf, Writer, Encoder, Reader, Decoder, ActingVersion};

/// Order side enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, SbeEnum)]
enum Side {
    Buy,
    Sell,
}

/// Order type enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, SbeEnum)]
enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// Trade message with enum fields
#[derive(SbeEncode, SbeDecode)]
#[sbe(template_id = 3, schema_id = 1, version = 0)]
struct TradeWithEnum {
    #[sbe(id = 0)]
    trade_id: u64,

    #[sbe(id = 1)]
    side: u8, // Will be encoded as Side enum

    #[sbe(id = 2)]
    order_type: u8, // Will be encoded as OrderType enum

    #[sbe(id = 3)]
    price: f64,

    #[sbe(id = 4)]
    quantity: i32,
}

fn main() {
    use sbe::{message_header_codec::MessageHeaderDecoder, ReadBuf, WriteBuf};

    println!("SBE Enum Support Example");
    println!("========================\n");

    // Test enum conversions
    let buy_side = Side::Buy;
    let sell_side = Side::Sell;

    println!("Enum conversions:");
    println!("  Side::Buy -> u8: {}", buy_side.to_u8());
    println!("  Side::Sell -> u8: {}", sell_side.to_u8());
    println!("  u8(0) -> Side: {:?}", Side::from_u8(0));
    println!("  u8(1) -> Side: {:?}", Side::from_u8(1));

    let market = OrderType::Market;
    let limit = OrderType::Limit;

    println!("\n  OrderType::Market -> u8: {}", market.to_u8());
    println!("  OrderType::Limit -> u8: {}", limit.to_u8());
    println!("  u8(0) -> OrderType: {:?}", OrderType::from_u8(0));
    println!("  u8(1) -> OrderType: {:?}", OrderType::from_u8(1));

    // Encode a trade with enums
    println!("\nEncoding trade with enums...");
    let mut buffer = vec![0u8; 1024];
    let write_buf = WriteBuf::new(&mut buffer);

    let encoder = TradeWithEnumEncoder::default();
    let mut header = encoder.header(0);
    let mut encoder = header.parent().unwrap();

    encoder.trade_id(12345);
    encoder.side(Side::Buy.to_u8());
    encoder.order_type(OrderType::Limit.to_u8());
    encoder.price(100.50);
    encoder.quantity(1000);

    println!("  Trade ID: 12345");
    println!("  Side: {:?} ({})", Side::Buy, Side::Buy.to_u8());
    println!("  Order Type: {:?} ({})", OrderType::Limit, OrderType::Limit.to_u8());
    println!("  Price: 100.50");
    println!("  Quantity: 1000");

    // Decode the trade
    println!("\nDecoding trade with enums...");
    let read_buf = ReadBuf::new(&buffer);
    let header = MessageHeaderDecoder::default().wrap(read_buf, 0);
    let decoder = TradeWithEnumDecoder::default().header(header, 0);

    let decoded_side = Side::from_u8(decoder.side());
    let decoded_order_type = OrderType::from_u8(decoder.order_type());

    println!("  Trade ID: {}", decoder.trade_id());
    println!("  Side: {:?} ({})", decoded_side, decoder.side());
    println!("  Order Type: {:?} ({})", decoded_order_type, decoder.order_type());
    println!("  Price: {}", decoder.price());
    println!("  Quantity: {}", decoder.quantity());

    println!("\n✓ Enum encoding and decoding successful!");
}
