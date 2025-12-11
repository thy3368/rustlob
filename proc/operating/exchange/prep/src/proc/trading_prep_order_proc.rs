//! Binance Perpetual Contract Order Commands - Open/Close Position
//!
//! Core order commands for Binance perpetual futures trading
//! Following Clean Architecture and low-latency optimization standards

use std::fmt;

// ============================================================================
// Core Enums
// ============================================================================

/// Order side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    /// Buy
    Buy = 1,
    /// Sell
    Sell = 2,
}

impl Side {
    #[inline(always)]
    pub const fn opposite(self) -> Self {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }

    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }
}

/// Order type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OrderType {
    /// Market order
    Market = 1,
    /// Limit order
    Limit = 2,
}

impl OrderType {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
        }
    }
}

/// Position side for Binance hedge mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PositionSide {
    /// One-way position mode
    Both = 1,
    /// Long position
    Long = 2,
    /// Short position
    Short = 3,
}

impl PositionSide {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            PositionSide::Both => "BOTH",
            PositionSide::Long => "LONG",
            PositionSide::Short => "SHORT",
        }
    }
}

/// Time in Force
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC = 1,
    /// Immediate Or Cancel
    IOC = 2,
    /// Fill Or Kill
    FOK = 3,
    /// Post only
    GTX = 4,
}

impl TimeInForce {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::GTX => "GTX",
        }
    }
}

// ============================================================================
// Value Objects - Fixed-point arithmetic
// ============================================================================

/// Price with 8 decimal precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Price(i64);

impl Price {
    const SCALE: i64 = 100_000_000;

    #[inline(always)]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::SCALE as f64) as i64)
    }

    #[inline(always)]
    pub const fn raw(self) -> i64 {
        self.0
    }

    #[inline(always)]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.8}", self.to_f64())
    }
}

/// Quantity with 8 decimal precision
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Quantity(i64);

impl Quantity {
    const SCALE: i64 = 100_000_000;

    #[inline(always)]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub fn from_f64(value: f64) -> Self {
        Self((value * Self::SCALE as f64) as i64)
    }

    #[inline(always)]
    pub const fn raw(self) -> i64 {
        self.0
    }

    #[inline(always)]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / Self::SCALE as f64
    }

    #[inline(always)]
    pub const fn is_positive(self) -> bool {
        self.0 > 0
    }

    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.8}", self.to_f64())
    }
}

/// Trading symbol
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol([u8; 16]);

impl Symbol {
    #[inline]
    pub fn new(s: &str) -> Self {
        let mut data = [0u8; 16];
        let bytes = s.as_bytes();
        let len = bytes.len().min(16);
        data[..len].copy_from_slice(&bytes[..len]);
        Self(data)
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(16);
        unsafe { std::str::from_utf8_unchecked(&self.0[..len]) }
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Open Position Command
// ============================================================================

/// Open Position Command
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct OpenPositionCommand {
    /// Trading symbol
    pub symbol: Symbol,
    /// Order side (Buy=Long, Sell=Short)
    pub side: Side,
    /// Order type
    pub order_type: OrderType,
    /// Quantity
    pub quantity: Quantity,
    /// Limit price (None for market orders)
    pub price: Option<Price>,
    /// Position side
    pub position_side: PositionSide,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Leverage (1-125)
    pub leverage: u8,
}

impl OpenPositionCommand {
    /// Create market long order
    ///
    /// # Example
    /// ```
    /// use prep_proc::proc::trading_prep_order_proc::{OpenPositionCommand, Symbol, Quantity};
    ///
    /// // Open long BTC position with 1.0 quantity
    /// let cmd = OpenPositionCommand::market_long(
    ///     Symbol::new("BTCUSDT"),
    ///     Quantity::from_f64(1.0)
    /// );
    /// ```
    #[inline]
    pub fn market_long(symbol: Symbol, quantity: Quantity) -> Self {
        Self {
            symbol,
            side: Side::Buy,
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// Create market short order
    #[inline]
    pub fn market_short(symbol: Symbol, quantity: Quantity) -> Self {
        Self {
            symbol,
            side: Side::Sell,
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// Create limit long order
    #[inline]
    pub fn limit_long(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Buy,
            order_type: OrderType::Limit,
            quantity,
            price: Some(price),
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// Create limit short order
    #[inline]
    pub fn limit_short(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Sell,
            order_type: OrderType::Limit,
            quantity,
            price: Some(price),
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
            leverage: 1,
        }
    }

    /// Set leverage
    #[inline]
    pub fn with_leverage(mut self, leverage: u8) -> Self {
        self.leverage = leverage;
        self
    }

    /// Set time in force
    #[inline]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// Use one-way position mode
    #[inline]
    pub fn both_side(mut self) -> Self {
        self.position_side = PositionSide::Both;
        self
    }

    /// Validate order
    #[inline]
    pub fn validate(&self) -> Result<(), &'static str> {
        // Validate quantity
        if !self.quantity.is_positive() {
            return Err("Quantity must be positive");
        }

        // Validate limit order price
        if self.order_type == OrderType::Limit {
            match self.price {
                Some(p) if p.is_positive() => {}
                Some(_) => return Err("Limit order price must be positive"),
                None => return Err("Limit order requires price"),
            }
        }

        // Validate leverage
        if self.leverage == 0 || self.leverage > 125 {
            return Err("Leverage must be between 1-125");
        }

        Ok(())
    }
}

// ============================================================================
// Close Position Command
// ============================================================================

/// Close Position Command
#[repr(align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ClosePositionCommand {
    /// Trading symbol
    pub symbol: Symbol,
    /// Order side (Sell to close long, Buy to close short)
    pub side: Side,
    /// Order type
    pub order_type: OrderType,
    /// Quantity (None means close all)
    pub quantity: Option<Quantity>,
    /// Limit price
    pub price: Option<Price>,
    /// Position side
    pub position_side: PositionSide,
    /// Time in force
    pub time_in_force: TimeInForce,
}

impl ClosePositionCommand {
    /// Market close long position
    ///
    /// # Example
    /// ```
    /// use prep_proc::proc::trading_prep_order_proc::{ClosePositionCommand, Symbol, Quantity};
    ///
    /// // Close all long position
    /// let cmd = ClosePositionCommand::market_close_long(
    ///     Symbol::new("BTCUSDT"),
    ///     None  // None means close all
    /// );
    ///
    /// // Close partial long position
    /// let cmd = ClosePositionCommand::market_close_long(
    ///     Symbol::new("BTCUSDT"),
    ///     Some(Quantity::from_f64(0.5))
    /// );
    /// ```
    #[inline]
    pub fn market_close_long(symbol: Symbol, quantity: Option<Quantity>) -> Self {
        Self {
            symbol,
            side: Side::Sell,  // Sell to close long
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// Market close short position
    #[inline]
    pub fn market_close_short(symbol: Symbol, quantity: Option<Quantity>) -> Self {
        Self {
            symbol,
            side: Side::Buy,  // Buy to close short
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// Limit close long position
    #[inline]
    pub fn limit_close_long(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Sell,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// Limit close short position
    #[inline]
    pub fn limit_close_short(symbol: Symbol, quantity: Quantity, price: Price) -> Self {
        Self {
            symbol,
            side: Side::Buy,
            order_type: OrderType::Limit,
            quantity: Some(quantity),
            price: Some(price),
            position_side: PositionSide::Short,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// Market close position (one-way mode)
    #[inline]
    pub fn market_close_both(symbol: Symbol, side: Side, quantity: Option<Quantity>) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::Market,
            quantity,
            price: None,
            position_side: PositionSide::Both,
            time_in_force: TimeInForce::GTC,
        }
    }

    /// Set time in force
    #[inline]
    pub fn with_time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif;
        self
    }

    /// Validate order
    #[inline]
    pub fn validate(&self) -> Result<(), &'static str> {
        // Validate quantity (None means close all, which is allowed)
        if let Some(qty) = self.quantity {
            if !qty.is_positive() {
                return Err("Close quantity must be positive");
            }
        }

        // Validate limit order price
        if self.order_type == OrderType::Limit {
            match self.price {
                Some(p) if p.is_positive() => {}
                Some(_) => return Err("Limit order price must be positive"),
                None => return Err("Limit order requires price"),
            }
        }

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_position_market_long() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0))
            .with_leverage(10);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Buy);
        assert_eq!(cmd.position_side, PositionSide::Long);
        assert_eq!(cmd.order_type, OrderType::Market);
        assert_eq!(cmd.leverage, 10);
        assert!(cmd.price.is_none());
    }

    #[test]
    fn test_open_position_market_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = OpenPositionCommand::market_short(symbol, Quantity::from_f64(2.0))
            .with_leverage(5);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Sell);
        assert_eq!(cmd.position_side, PositionSide::Short);
        assert_eq!(cmd.leverage, 5);
    }

    #[test]
    fn test_open_position_limit_long() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::limit_long(
            symbol,
            Quantity::from_f64(0.5),
            Price::from_f64(50000.0),
        )
        .with_leverage(20);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.order_type, OrderType::Limit);
        assert!(cmd.price.is_some());
        assert_eq!(cmd.leverage, 20);
    }

    #[test]
    fn test_open_position_limit_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = OpenPositionCommand::limit_short(
            symbol,
            Quantity::from_f64(3.0),
            Price::from_f64(3000.0),
        );

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Sell);
        assert_eq!(cmd.position_side, PositionSide::Short);
    }

    #[test]
    fn test_close_position_market_long() {
        let symbol = Symbol::new("BTCUSDT");

        // Close all
        let cmd = ClosePositionCommand::market_close_long(symbol, None);
        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Sell);
        assert_eq!(cmd.position_side, PositionSide::Long);
        assert!(cmd.quantity.is_none());

        // Close partial
        let cmd = ClosePositionCommand::market_close_long(symbol, Some(Quantity::from_f64(0.5)));
        assert!(cmd.validate().is_ok());
        assert!(cmd.quantity.is_some());
    }

    #[test]
    fn test_close_position_market_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = ClosePositionCommand::market_close_short(symbol, None);

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Buy);  // Buy to close short
        assert_eq!(cmd.position_side, PositionSide::Short);
    }

    #[test]
    fn test_close_position_limit_long() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = ClosePositionCommand::limit_close_long(
            symbol,
            Quantity::from_f64(1.0),
            Price::from_f64(55000.0),
        );

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.order_type, OrderType::Limit);
        assert!(cmd.price.is_some());
    }

    #[test]
    fn test_close_position_limit_short() {
        let symbol = Symbol::new("ETHUSDT");
        let cmd = ClosePositionCommand::limit_close_short(
            symbol,
            Quantity::from_f64(2.0),
            Price::from_f64(2900.0),
        );

        assert!(cmd.validate().is_ok());
        assert_eq!(cmd.side, Side::Buy);
    }

    #[test]
    fn test_validation_invalid_quantity() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_raw(0));
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_leverage() {
        let symbol = Symbol::new("BTCUSDT");
        let cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0))
            .with_leverage(200);  // Over 125
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_validation_limit_order_missing_price() {
        let symbol = Symbol::new("BTCUSDT");
        let mut cmd = OpenPositionCommand::market_long(symbol, Quantity::from_f64(1.0));
        cmd.order_type = OrderType::Limit;
        cmd.price = None;
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_side_opposite() {
        assert_eq!(Side::Buy.opposite(), Side::Sell);
        assert_eq!(Side::Sell.opposite(), Side::Buy);
    }

    #[test]
    fn test_value_objects() {
        let price = Price::from_f64(50000.12345678);
        assert!((price.to_f64() - 50000.12345678).abs() < 0.00000001);

        let qty = Quantity::from_f64(1.5);
        assert!((qty.to_f64() - 1.5).abs() < 0.00000001);
        assert!(qty.is_positive());
        assert!(!qty.is_zero());
    }

    #[test]
    fn test_symbol() {
        let symbol = Symbol::new("BTCUSDT");
        assert_eq!(symbol.as_str(), "BTCUSDT");
    }

    #[test]
    fn test_memory_alignment() {
        use std::mem::align_of;
        assert_eq!(align_of::<OpenPositionCommand>(), 64);
        assert_eq!(align_of::<ClosePositionCommand>(), 64);
    }
}
