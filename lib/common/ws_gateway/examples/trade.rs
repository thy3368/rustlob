use sbe::trade_codec::{TradeEncoder, TradeDecoder};
use sbe::{WriteBuf, ReadBuf};

/// Trade message using SBE encoding
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trade {
    pub trade_id: u64,
    pub symbol: u8,
    pub price: f64,
    pub quantity: i32,
}

impl Trade {
    /// Create a new trade message
    pub fn new(trade_id: u64, symbol: u8, price: f64, quantity: i32) -> Self {
        Self {
            trade_id,
            symbol,
            price,
            quantity,
        }
    }

    /// Encode trade to SBE binary format
    ///
    /// Zero-copy encoding: directly writes to the provided buffer
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 64];
        let mut write_buf = WriteBuf::new(&mut buf);
        let mut encoder = TradeEncoder::default().wrap(write_buf, 0);

        encoder.trade_id(self.trade_id);
        encoder.symbol(self.symbol);
        encoder.price(self.price);
        encoder.quantity(self.quantity);

        buf.truncate(sbe::message_header_codec::ENCODED_LENGTH + sbe::trade_codec::SBE_BLOCK_LENGTH as usize);
        buf
    }

    /// Encode to a buffer at a specific offset
    ///
    /// Useful for batch encoding multiple trades
    pub fn encode_to_buffer(trades: &[Trade], buf: &mut [u8]) -> usize {
        let msg_size = sbe::message_header_codec::ENCODED_LENGTH + sbe::trade_codec::SBE_BLOCK_LENGTH as usize;
        let mut offset = 0;

        for trade in trades {
            if offset + msg_size > buf.len() {
                break;
            }

            let mut write_buf = WriteBuf::new(buf);
            let mut encoder = TradeEncoder::default().wrap(write_buf, offset);

            encoder.trade_id(trade.trade_id);
            encoder.symbol(trade.symbol);
            encoder.price(trade.price);
            encoder.quantity(trade.quantity);

            offset += msg_size;
        }

        offset
    }

    /// Decode trade from SBE binary format
    ///
    /// Zero-copy decoding: reads directly from the byte buffer
    pub fn decode(data: &[u8]) -> Option<Self> {
        let required_size = sbe::message_header_codec::ENCODED_LENGTH + sbe::trade_codec::SBE_BLOCK_LENGTH as usize;

        if data.len() < required_size {
            return None;
        }

        let read_buf = ReadBuf::new(data);
        let decoder = TradeDecoder::default()
            .wrap(read_buf, 0, sbe::trade_codec::SBE_BLOCK_LENGTH, sbe::SBE_SCHEMA_VERSION);

        Some(Self {
            trade_id: decoder.trade_id(),
            symbol: decoder.symbol(),
            price: decoder.price(),
            quantity: decoder.quantity(),
        })
    }

    /// Decode from buffer at a specific offset
    pub fn decode_from_buffer(data: &[u8], offset: usize) -> Option<Self> {
        let required_size = sbe::message_header_codec::ENCODED_LENGTH + sbe::trade_codec::SBE_BLOCK_LENGTH as usize;

        if offset + required_size > data.len() {
            return None;
        }

        let read_buf = ReadBuf::new(data);
        let decoder = TradeDecoder::default()
            .wrap(read_buf, offset, sbe::trade_codec::SBE_BLOCK_LENGTH, sbe::SBE_SCHEMA_VERSION);

        Some(Self {
            trade_id: decoder.trade_id(),
            symbol: decoder.symbol(),
            price: decoder.price(),
            quantity: decoder.quantity(),
        })
    }

    /// Get the encoded message size
    pub fn encoded_size() -> usize {
        sbe::message_header_codec::ENCODED_LENGTH + sbe::trade_codec::SBE_BLOCK_LENGTH as usize
    }

    /// Get the symbol as a character
    pub fn symbol_char(&self) -> char {
        self.symbol as char
    }

    /// Calculate the notional value (price * quantity)
    pub fn notional(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = Trade::new(12345, b'B', 100.5, 1000);
        let encoded = original.encode();
        let decoded = Trade::decode(&encoded).expect("Failed to decode");
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_batch_encoding() {
        let trades = vec![
            Trade::new(1, b'A', 100.0, 1000),
            Trade::new(2, b'B', 200.0, 2000),
            Trade::new(3, b'C', 300.0, 3000),
        ];

        let msg_size = Trade::encoded_size();
        let mut buf = vec![0u8; msg_size * trades.len()];
        let encoded_len = Trade::encode_to_buffer(&trades, &mut buf);

        assert_eq!(encoded_len, msg_size * 3);

        // Decode all trades from buffer
        for (i, expected) in trades.iter().enumerate() {
            let offset = i * msg_size;
            let decoded = Trade::decode_from_buffer(&buf, offset).expect("Failed to decode");
            assert_eq!(decoded, *expected);
        }
    }

    #[test]
    fn test_notional_calculation() {
        let trade = Trade::new(100, b'X', 50.5, 200);
        assert_eq!(trade.notional(), 50.5 * 200.0);
    }
}
