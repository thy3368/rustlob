//! Time types for SBE encoding

/// SBE time type definitions according to FIX SBE 2.0 specification

/// UTCTimestamp - nanoseconds since Unix epoch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct UTCTimestamp(pub i64);

#[allow(dead_code)]
impl UTCTimestamp {
    pub fn from_nanos(nanos: i64) -> Self {
        Self(nanos)
    }

    pub fn to_nanos(&self) -> i64 {
        self.0
    }

    pub fn from_millis(millis: i64) -> Self {
        Self(millis * 1_000_000)
    }

    pub fn to_millis(&self) -> i64 {
        self.0 / 1_000_000
    }
}

/// UTCDateOnly - days since Unix epoch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct UTCDateOnly(pub i32);

#[allow(dead_code)]
impl UTCDateOnly {
    pub fn from_days(days: i32) -> Self {
        Self(days)
    }

    pub fn to_days(&self) -> i32 {
        self.0
    }
}

/// UTCTimeOnly - nanoseconds since midnight
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct UTCTimeOnly(pub i64);

#[allow(dead_code)]
impl UTCTimeOnly {
    pub fn from_nanos(nanos: i64) -> Self {
        Self(nanos)
    }

    pub fn to_nanos(&self) -> i64 {
        self.0
    }
}

/// MonthYear - YYYYMM format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct MonthYear {
    pub year: u16,
    pub month: u8,
}

#[allow(dead_code)]
impl MonthYear {
    pub fn new(year: u16, month: u8) -> Self {
        assert!(month >= 1 && month <= 12, "Month must be 1-12");
        Self { year, month }
    }

    pub fn to_u32(&self) -> u32 {
        (self.year as u32) * 100 + (self.month as u32)
    }

    pub fn from_u32(value: u32) -> Self {
        let year = (value / 100) as u16;
        let month = (value % 100) as u8;
        Self::new(year, month)
    }
}
