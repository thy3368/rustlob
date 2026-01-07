//! Settlement Domain Entities
//!
//! 结算模块领域实体，遵循 DDD 设计原则:
//! - 聚合根: Settlement 结算单
//! - 值对象: newtype 包装类型
//! - 高性能: 64 字节缓存对齐
//! - 幂等性: IdempotencyKey 防重复

// ============================================================================
// Value Objects (值对象)
// ============================================================================

/// 结算ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SettlementId(pub u64);

/// 清算ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ClearingId(pub u64);

/// 交易ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TradeId(pub u64);

/// 账户ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AccountId(pub u64);

/// 交易者ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TraderId(pub u64);

/// 仓位ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct PositionId(pub u64);

/// 分录ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct EntryId(pub u64);

/// 命令ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CommandId(pub u64);

/// 资产ID (u32 高性能)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AssetId(pub u32);

/// 交易对ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SymbolId(pub u32);

/// 金额 - 有符号整数，支持盈亏
///
/// 8位小数精度 (1e8)
/// 示例: 1.00000000 USDT = 100_000_000
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Amount(pub i64);

impl Amount {
    pub const PRECISION: u32 = 8;
    pub const SCALE: i64 = 100_000_000;
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    #[inline]
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    #[inline]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    #[inline]
    pub fn saturating_add(&self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }

    #[inline]
    pub fn saturating_sub(&self, other: Self) -> Self {
        Self(self.0.saturating_sub(other.0))
    }
}

/// 价格 (u64, 8位小数精度)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Price(pub u64);

impl Price {
    pub const PRECISION: u32 = 8;
    pub const SCALE: u64 = 100_000_000;
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// 数量 (u64, 8位小数精度)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Quantity(pub u64);

impl Quantity {
    pub const PRECISION: u32 = 8;
    pub const SCALE: u64 = 100_000_000;
    pub const ZERO: Self = Self(0);

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// 时间戳 (纳秒)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn now() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self(nanos)
    }

    #[inline]
    pub fn as_millis(&self) -> u64 {
        self.0 / 1_000_000
    }

    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.0 / 1_000_000_000
    }
}

// ============================================================================
// Idempotency Key (幂等键)
// ============================================================================

/// 幂等键 (固定 32 字节高性能)
///
/// 防止重复结算执行
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct IdempotencyKey(pub [u8; 32]);

impl IdempotencyKey {
    /// 从交易ID创建
    /// 格式: "TRADE:{trade_id:016x}"
    pub fn from_trade(trade_id: TradeId) -> Self {
        let mut key = [0u8; 32];
        key[0..6].copy_from_slice(b"TRADE:");
        let hex = format!("{:016x}", trade_id.0);
        key[6..22].copy_from_slice(hex.as_bytes());
        Self(key)
    }

    /// 从资金费率结算创建
    /// 格式: "FUND:{round:08x}:{position_id:016x}"
    pub fn from_funding(round: u32, position_id: PositionId) -> Self {
        let mut key = [0u8; 32];
        key[0..5].copy_from_slice(b"FUND:");
        let content = format!("{:08x}:{:016x}", round, position_id.0);
        let bytes = content.as_bytes();
        let len = bytes.len().min(27);
        key[5..5 + len].copy_from_slice(&bytes[..len]);
        Self(key)
    }

    /// 从结算分录创建
    /// 格式: "SETT:{settlement_id:016x}:{seq:02x}"
    pub fn from_settlement_entry(settlement_id: SettlementId, seq: u8) -> Self {
        let mut key = [0u8; 32];
        key[0..5].copy_from_slice(b"SETT:");
        let content = format!("{:016x}:{:02x}", settlement_id.0, seq);
        let bytes = content.as_bytes();
        let len = bytes.len().min(27);
        key[5..5 + len].copy_from_slice(&bytes[..len]);
        Self(key)
    }
}

impl std::fmt::Debug for IdempotencyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 找到第一个0字节，截断显示
        let end = self.0.iter().position(|&b| b == 0).unwrap_or(32);
        let s = std::str::from_utf8(&self.0[..end]).unwrap_or("<invalid>");
        write!(f, "IdempotencyKey({})", s)
    }
}

// ============================================================================
// Enums (枚举类型)
// ============================================================================

/// 交易方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Side {
    Buy = 0,
    Sell = 1,
}

/// 仓位方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PositionSide {
    /// 单向持仓
    Both = 0,
    /// 多头
    Long = 1,
    /// 空头
    Short = 2,
}

/// 品种类型（从 base_types 导入）
pub use base_types::InstrumentType;

/// 结算类型
///
/// 注意: 保证金操作已内化到各 Settlement Command 中
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SettlementType {
    // === Spot (0x1X) ===
    /// 现货即时结算
    SpotInstant = 0x10,

    // === Perpetual Trade (0x2X) ===
    /// 永续开仓结算（含保证金锁定）
    PrepOpen = 0x20,
    /// 永续平仓结算（含保证金释放 + 盈亏结算）
    PrepClose = 0x21,
    /// 永续减仓结算（含部分保证金释放 + 盈亏结算）
    PrepReduce = 0x22,

    // === Perpetual Periodic (0x3X) ===
    /// 资金费率结算
    PrepFundingRate = 0x30,

    // === Risk (0x4X) ===
    /// 强平结算（含保证金清算 + 保险基金）
    PrepLiquidation = 0x40,
    /// ADL自动减仓结算
    PrepADL = 0x41,

    // === Admin (0xFX) ===
    /// 保证金调整（逐仓模式手动追加/减少）
    MarginAdjust = 0xF0,
    /// 冲正/回滚
    Reversal = 0xF1,
}

/// 结算状态（状态机）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SettlementStatus {
    /// 待处理
    Pending = 0,
    /// 处理中
    Processing = 1,
    /// 已完成
    Completed = 2,
    /// 失败
    Failed = 3,
    /// 已冲正
    Reversed = 4,
}

impl SettlementStatus {
    /// 检查状态转换是否有效
    pub fn can_transition_to(&self, target: Self) -> bool {
        use SettlementStatus::*;
        matches!(
            (self, target),
            (Pending, Processing)
                | (Processing, Completed)
                | (Processing, Failed)
                | (Completed, Reversed)
                | (Failed, Pending) // 重试
        )
    }
}

/// 资金方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    /// 贷方（余额增加）
    Credit = 0,
    /// 借方（余额减少）
    Debit = 1,
}

/// 分录类型（按业务分类）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntryType {
    // === 现货 (0x1X) ===
    /// 现货资产转移
    SpotTransfer = 0x10,
    /// 现货手续费
    SpotFee = 0x11,

    // === 保证金 (0x2X) ===
    /// 锁定保证金
    MarginLock = 0x20,
    /// 释放保证金
    MarginRelease = 0x21,
    /// 保证金调整
    MarginTransfer = 0x22,

    // === 盈亏 (0x3X) ===
    /// 已实现盈亏
    RealizedPnl = 0x30,
    /// 未实现盈亏
    UnrealizedPnl = 0x31,
    /// 资金费用
    FundingFee = 0x32,

    // === 手续费 (0x4X) ===
    /// 永续交易手续费
    TradingFee = 0x40,
    /// Maker 返佣
    MakerRebate = 0x41,

    // === 风控 (0x5X) ===
    /// 强平罚金
    LiquidationPenalty = 0x50,
    /// 保险基金注入
    InsuranceFund = 0x51,
    /// 社会化分摊
    SocializedLoss = 0x52,
    /// ADL 平仓
    ADLClose = 0x53,

    // === 管理 (0xFX) ===
    /// 人工调账
    Adjustment = 0xF0,
    /// 冲正分录
    Reversal = 0xF1,
}

impl EntryType {
    /// 获取该分录类型的默认方向
    pub fn default_direction(&self) -> Option<Direction> {
        use Direction::*;
        use EntryType::*;
        match self {
            // 固定借方
            SpotFee | TradingFee | LiquidationPenalty | MarginLock | SocializedLoss => Some(Debit),
            // 固定贷方
            MakerRebate | MarginRelease | InsuranceFund => Some(Credit),
            // 视情况而定
            _ => None,
        }
    }
}

// ============================================================================
// Aggregate Root: Settlement (结算聚合根)
// ============================================================================

/// 结算单 - 聚合根
///
/// 包含一笔完整结算的元数据，
/// Entry 通过 Repository 单独加载和保存
#[repr(align(64))]
pub struct Settlement {
    /// 结算ID
    pub id: SettlementId,

    /// 幂等键
    pub idempotency_key: IdempotencyKey,

    /// 关联清算ID
    pub clearing_id: ClearingId,

    /// 关联交易ID
    pub trade_id: Option<TradeId>,

    /// 结算类型
    pub settlement_type: SettlementType,

    /// 主账户
    pub primary_account: AccountId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 结算总金额
    pub total_amount: Amount,

    /// 分录数量
    pub entry_count: u8,

    /// 状态
    pub status: SettlementStatus,

    /// 创建时间
    pub created_at: Timestamp,

    /// 完成时间
    pub settled_at: Option<Timestamp>,

    /// 乐观锁版本
    pub version: u64,
}

impl Settlement {
    /// 创建新结算
    pub fn new(
        id: SettlementId,
        idempotency_key: IdempotencyKey,
        clearing_id: ClearingId,
        trade_id: Option<TradeId>,
        settlement_type: SettlementType,
        primary_account: AccountId,
        asset_id: AssetId,
        total_amount: Amount,
        entry_count: u8,
    ) -> Self {
        Self {
            id,
            idempotency_key,
            clearing_id,
            trade_id,
            settlement_type,
            primary_account,
            asset_id,
            total_amount,
            entry_count,
            status: SettlementStatus::Pending,
            created_at: Timestamp::now(),
            settled_at: None,
            version: 0,
        }
    }

    /// 状态转换
    pub fn transition_to(&mut self, status: SettlementStatus) -> Result<(), &'static str> {
        if self.status.can_transition_to(status) {
            self.status = status;
            self.version += 1;
            if status == SettlementStatus::Completed {
                self.settled_at = Some(Timestamp::now());
            }
            Ok(())
        } else {
            Err("Invalid status transition")
        }
    }
}

// ============================================================================
// Entity: SettlementEntry (结算分录)
// ============================================================================

/// 结算分录 - 单笔资产变动记录
///
/// 一个结算包含多个分录，append-only模式
#[repr(align(64))]
pub struct SettlementEntry {
    /// 分录ID
    pub id: EntryId,

    /// 所属结算ID
    pub settlement_id: SettlementId,

    /// 分录序号
    pub sequence: u8,

    /// 账户ID
    pub account_id: AccountId,

    /// 仓位ID（永续用）
    pub position_id: PositionId,

    /// 资产ID
    pub asset_id: AssetId,

    /// 分录类型
    pub entry_type: EntryType,

    /// 资金方向
    pub direction: Direction,

    /// 金额
    pub amount: Amount,

    /// 变动前余额
    pub balance_before: Amount,

    /// 变动后余额
    pub balance_after: Amount,

    /// 时间戳
    pub timestamp: Timestamp,

    /// 备注
    pub memo: u64,
}

impl SettlementEntry {
    /// 创建新分录
    pub fn new(
        id: EntryId,
        settlement_id: SettlementId,
        sequence: u8,
        account_id: AccountId,
        position_id: PositionId,
        asset_id: AssetId,
        entry_type: EntryType,
        direction: Direction,
        amount: Amount,
    ) -> Self {
        Self {
            id,
            settlement_id,
            sequence,
            account_id,
            position_id,
            asset_id,
            entry_type,
            direction,
            amount,
            balance_before: Amount::default(),
            balance_after: Amount::default(),
            timestamp: Timestamp::now(),
            memo: 0,
        }
    }

    /// 设置余额快照
    pub fn with_balance(mut self, before: Amount, after: Amount) -> Self {
        self.balance_before = before;
        self.balance_after = after;
        self
    }

    /// 设置备注
    pub fn with_memo(mut self, memo: u64) -> Self {
        self.memo = memo;
        self
    }
}

// ============================================================================
// Entity: ClearingRecord (清算记录)
// ============================================================================

/// 清算参与方
#[derive(Debug, Clone)]
pub struct ClearingParty {
    /// 账户ID
    pub account_id: AccountId,
    /// 仓位ID（永续）
    pub position_id: Option<PositionId>,
    /// 是否 Maker
    pub is_maker: bool,
    /// 手续费
    pub fee: Amount,
}

/// 清算记录
///
/// Trade → ClearingRecord → Settlement(s) → Entry(s)
#[repr(align(64))]
pub struct ClearingRecord {
    /// 清算ID
    pub id: ClearingId,

    /// 交易ID
    pub trade_id: TradeId,

    /// 品种类型
    pub instrument_type: InstrumentType,

    /// 交易对ID
    pub symbol_id: SymbolId,

    /// 买方
    pub buyer: ClearingParty,

    /// 卖方
    pub seller: ClearingParty,

    /// 成交价格
    pub price: Price,

    /// 成交数量
    pub quantity: Quantity,

    /// 成交金额
    pub notional: Amount,

    /// 成交时间
    pub trade_time: Timestamp,

    /// 清算时间
    pub cleared_at: Timestamp,

    /// 关联的结算ID列表
    pub settlement_ids: Vec<SettlementId>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idempotency_key_from_trade() {
        let key = IdempotencyKey::from_trade(TradeId(12345));
        let s = format!("{:?}", key);
        assert!(s.contains("TRADE:"));
    }

    #[test]
    fn test_settlement_status_transition() {
        let mut settlement = Settlement::new(
            SettlementId(1),
            IdempotencyKey::from_trade(TradeId(1)),
            ClearingId(1),
            Some(TradeId(1)),
            SettlementType::SpotInstant,
            AccountId(100),
            AssetId(1),
            Amount(1000),
            2,
        );

        assert_eq!(settlement.status, SettlementStatus::Pending);
        assert!(settlement.transition_to(SettlementStatus::Processing).is_ok());
        assert_eq!(settlement.status, SettlementStatus::Processing);
        assert!(settlement.transition_to(SettlementStatus::Completed).is_ok());
        assert_eq!(settlement.status, SettlementStatus::Completed);
        assert!(settlement.settled_at.is_some());
    }

    #[test]
    fn test_invalid_status_transition() {
        let mut settlement = Settlement::new(
            SettlementId(1),
            IdempotencyKey::from_trade(TradeId(1)),
            ClearingId(1),
            None,
            SettlementType::PrepOpen,
            AccountId(100),
            AssetId(1),
            Amount(5000),
            2,
        );

        // Pending -> Completed 不允许直接跳转
        assert!(settlement.transition_to(SettlementStatus::Completed).is_err());
    }

    #[test]
    fn test_entry_type_default_direction() {
        assert_eq!(
            EntryType::TradingFee.default_direction(),
            Some(Direction::Debit)
        );
        assert_eq!(
            EntryType::MarginRelease.default_direction(),
            Some(Direction::Credit)
        );
        assert_eq!(EntryType::RealizedPnl.default_direction(), None);
    }

    #[test]
    fn test_amount_operations() {
        let a = Amount(100);
        let b = Amount(-50);

        assert!(a.is_positive());
        assert!(b.is_negative());
        assert_eq!(a.saturating_add(b), Amount(50));
        assert_eq!(b.abs(), Amount(50));
    }
}
