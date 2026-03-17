/// 新订单命令（基础类型版本）
///
/// 使用基础类型，便于 SIMD 优化
///
/// # 设计目标
/// - 使用基础类型：u64、u8、[u8; N]
/// - 固定大小，避免动态分配
/// - 缓存行对齐，便于 SIMD 批量处理
/// - 使用位标志表示可选字段

/// 字段有效性标志位
pub mod field_flags {
    pub const TIME_IN_FORCE: u64 = 1 << 0;
    pub const QUANTITY: u64 = 1 << 1;
    pub const QUOTE_ORDER_QTY: u64 = 1 << 2;
    pub const PRICE: u64 = 1 << 3;
    pub const CLIENT_ORDER_ID: u64 = 1 << 4;
    pub const STRATEGY_ID: u64 = 1 << 5;
    pub const STRATEGY_TYPE: u64 = 1 << 6;
    pub const STOP_PRICE: u64 = 1 << 7;
    pub const TRAILING_DELTA: u64 = 1 << 8;
    pub const ICEBERG_QTY: u64 = 1 << 9;
    pub const RESP_TYPE: u64 = 1 << 10;
    pub const STP_MODE: u64 = 1 << 11;
    pub const PEG_PRICE_TYPE: u64 = 1 << 12;
    pub const PEG_OFFSET_VALUE: u64 = 1 << 13;
    pub const PEG_OFFSET_TYPE: u64 = 1 << 14;
}

/// 订单方向
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSideTag {
    Buy = 0,
    Sell = 1,
}

/// 订单类型
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderTypeTag {
    Limit = 0,
    Market = 1,
    StopLoss = 2,
    StopLossLimit = 3,
    TakeProfit = 4,
    TakeProfitLimit = 5,
    LimitMaker = 6,
}

/// 有效方式
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForceTag {
    GTC = 0, // Good Till Cancel
    IOC = 1, // Immediate Or Cancel
    FOK = 2, // Fill Or Kill
}

/// 订单响应类型
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespTypeTag {
    ACK = 0,
    RESULT = 1,
    FULL = 2,
}

/// 自成交保护模式
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum STPModeTag {
    None = 0,
    ExpireTaker = 1,
    ExpireMaker = 2,
    ExpireBoth = 3,
}

/// 价格钉住类型
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegPriceTypeTag {
    PrimaryPeg = 0,
    MarketPeg = 1,
}

/// 价格偏移类型
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PegOffsetTypeTag {
    PriceLevel = 0,
}

/// 新订单命令（基础类型版本）
///
/// 所有数值使用固定精度表示（例如：价格和数量乘以 10^8）
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewOrderCmdBase {
    // === 核心字段（必需） ===
    /// 时间戳（纳秒）
    pub timestamp: u64,
    /// 交易对（固定32字节，如 "BTCUSDT"）
    pub symbol: [u8; 32],
    /// 订单方向（0=Buy, 1=Sell）
    pub side: u8,
    /// 订单类型（0=LIMIT, 1=MARKET, ...）
    pub order_type: u8,

    // === 可选字段标志 ===
    /// 位标志，表示哪些可选字段有效
    pub field_flags: u64,

    // === 数值字段（u64，固定精度） ===
    /// 数量（固定精度，乘以 10^8）
    pub quantity: u64,
    /// 报价数量（固定精度，乘以 10^8）
    pub quote_order_qty: u64,
    /// 价格（固定精度，乘以 10^8）
    pub price: u64,
    /// 止损价格（固定精度，乘以 10^8）
    pub stop_price: u64,
    /// 冰山订单数量（固定精度，乘以 10^8）
    pub iceberg_qty: u64,

    // === 策略字段 ===
    /// 策略 ID
    pub strategy_id: i64,
    /// 策略类型
    pub strategy_type: i32,
    /// 跟踪止损回调幅度
    pub trailing_delta: i64,
    /// 价格偏移值
    pub peg_offset_value: i32,

    // === 枚举字段（u8） ===
    /// 有效方式（0=GTC, 1=IOC, 2=FOK）
    pub time_in_force: u8,
    /// 订单响应类型（0=ACK, 1=RESULT, 2=FULL）
    pub resp_type: u8,
    /// 自成交保护模式（0=NONE, 1=EXPIRE_TAKER, ...）
    pub stp_mode: u8,
    /// 价格钉住类型（0=PRIMARY_PEG, 1=MARKET_PEG）
    pub peg_price_type: u8,
    /// 价格偏移类型（0=PRICE_LEVEL）
    pub peg_offset_type: u8,

    // === 字符串字段 ===
    /// 用户自定义订单 ID（固定32字节）
    pub client_order_id: [u8; 32],

    // === Padding 对齐到 64 字节倍数 ===
    _padding: [u8; 3],
}

impl NewOrderCmdBase {
    /// 创建新的订单命令
    pub fn new(timestamp: u64, symbol: [u8; 32], side: u8, order_type: u8) -> Self {
        Self {
            timestamp,
            symbol,
            side,
            order_type,
            field_flags: 0,
            quantity: 0,
            quote_order_qty: 0,
            price: 0,
            stop_price: 0,
            iceberg_qty: 0,
            strategy_id: 0,
            strategy_type: 0,
            trailing_delta: 0,
            peg_offset_value: 0,
            time_in_force: 0,
            resp_type: 0,
            stp_mode: 0,
            peg_price_type: 0,
            peg_offset_type: 0,
            client_order_id: [0u8; 32],
            _padding: [0u8; 3],
        }
    }

    /// 从字符串创建交易对
    pub fn symbol_from_str(s: &str) -> [u8; 32] {
        let mut symbol = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        symbol[..len].copy_from_slice(&bytes[..len]);
        symbol
    }

    /// 获取交易对的字符串表示
    pub fn symbol_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let end = self.symbol.iter().position(|&b| b == 0).unwrap_or(32);
        std::str::from_utf8(&self.symbol[..end])
    }

    /// 从字符串创建订单 ID
    pub fn client_order_id_from_str(s: &str) -> [u8; 32] {
        let mut id = [0u8; 32];
        let bytes = s.as_bytes();
        let len = bytes.len().min(32);
        id[..len].copy_from_slice(&bytes[..len]);
        id
    }

    /// 获取订单 ID 的字符串表示
    pub fn client_order_id_as_str(&self) -> Result<&str, std::str::Utf8Error> {
        let end = self.client_order_id.iter().position(|&b| b == 0).unwrap_or(32);
        std::str::from_utf8(&self.client_order_id[..end])
    }

    // === 字段设置方法 ===

    pub fn set_quantity(&mut self, quantity: u64) {
        self.quantity = quantity;
        self.field_flags |= field_flags::QUANTITY;
    }

    pub fn set_quote_order_qty(&mut self, qty: u64) {
        self.quote_order_qty = qty;
        self.field_flags |= field_flags::QUOTE_ORDER_QTY;
    }

    pub fn set_price(&mut self, price: u64) {
        self.price = price;
        self.field_flags |= field_flags::PRICE;
    }

    pub fn set_stop_price(&mut self, price: u64) {
        self.stop_price = price;
        self.field_flags |= field_flags::STOP_PRICE;
    }

    pub fn set_iceberg_qty(&mut self, qty: u64) {
        self.iceberg_qty = qty;
        self.field_flags |= field_flags::ICEBERG_QTY;
    }

    pub fn set_time_in_force(&mut self, tif: u8) {
        self.time_in_force = tif;
        self.field_flags |= field_flags::TIME_IN_FORCE;
    }

    pub fn set_client_order_id(&mut self, id: [u8; 32]) {
        self.client_order_id = id;
        self.field_flags |= field_flags::CLIENT_ORDER_ID;
    }

    pub fn set_strategy(&mut self, id: i64, type_: i32) {
        self.strategy_id = id;
        self.strategy_type = type_;
        self.field_flags |= field_flags::STRATEGY_ID | field_flags::STRATEGY_TYPE;
    }

    // === 字段检查方法 ===

    pub fn has_quantity(&self) -> bool {
        self.field_flags & field_flags::QUANTITY != 0
    }

    pub fn has_price(&self) -> bool {
        self.field_flags & field_flags::PRICE != 0
    }

    pub fn has_stop_price(&self) -> bool {
        self.field_flags & field_flags::STOP_PRICE != 0
    }

    pub fn has_client_order_id(&self) -> bool {
        self.field_flags & field_flags::CLIENT_ORDER_ID != 0
    }
}

/// 新订单命令 SOA（Structure of Arrays）版本
///
/// 便于 SIMD 批量处理
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewOrderCmdSoa {
    /// 时间戳数组
    pub timestamps: Vec<u64>,
    /// 交易对数组
    pub symbols: Vec<[u8; 32]>,
    /// 订单方向数组
    pub sides: Vec<u8>,
    /// 订单类型数组
    pub order_types: Vec<u8>,
    /// 字段标志数组
    pub field_flags: Vec<u64>,
    /// 数量数组
    pub quantities: Vec<u64>,
    /// 报价数量数组
    pub quote_order_qtys: Vec<u64>,
    /// 价格数组
    pub prices: Vec<u64>,
    /// 止损价格数组
    pub stop_prices: Vec<u64>,
    /// 冰山订单数量数组
    pub iceberg_qtys: Vec<u64>,
    /// 策略 ID 数组
    pub strategy_ids: Vec<i64>,
    /// 策略类型数组
    pub strategy_types: Vec<i32>,
    /// 跟踪止损回调幅度数组
    pub trailing_deltas: Vec<i64>,
    /// 价格偏移值数组
    pub peg_offset_values: Vec<i32>,
    /// 有效方式数组
    pub time_in_forces: Vec<u8>,
    /// 响应类型数组
    pub resp_types: Vec<u8>,
    /// 自成交保护模式数组
    pub stp_modes: Vec<u8>,
    /// 价格钉住类型数组
    pub peg_price_types: Vec<u8>,
    /// 价格偏移类型数组
    pub peg_offset_types: Vec<u8>,
    /// 订单 ID 数组
    pub client_order_ids: Vec<[u8; 32]>,
}

impl NewOrderCmdSoa {
    /// 创建新的 SOA 结构
    pub fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            symbols: Vec::new(),
            sides: Vec::new(),
            order_types: Vec::new(),
            field_flags: Vec::new(),
            quantities: Vec::new(),
            quote_order_qtys: Vec::new(),
            prices: Vec::new(),
            stop_prices: Vec::new(),
            iceberg_qtys: Vec::new(),
            strategy_ids: Vec::new(),
            strategy_types: Vec::new(),
            trailing_deltas: Vec::new(),
            peg_offset_values: Vec::new(),
            time_in_forces: Vec::new(),
            resp_types: Vec::new(),
            stp_modes: Vec::new(),
            peg_price_types: Vec::new(),
            peg_offset_types: Vec::new(),
            client_order_ids: Vec::new(),
        }
    }

    /// 创建预分配容量的 SOA 结构
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            timestamps: Vec::with_capacity(capacity),
            symbols: Vec::with_capacity(capacity),
            sides: Vec::with_capacity(capacity),
            order_types: Vec::with_capacity(capacity),
            field_flags: Vec::with_capacity(capacity),
            quantities: Vec::with_capacity(capacity),
            quote_order_qtys: Vec::with_capacity(capacity),
            prices: Vec::with_capacity(capacity),
            stop_prices: Vec::with_capacity(capacity),
            iceberg_qtys: Vec::with_capacity(capacity),
            strategy_ids: Vec::with_capacity(capacity),
            strategy_types: Vec::with_capacity(capacity),
            trailing_deltas: Vec::with_capacity(capacity),
            peg_offset_values: Vec::with_capacity(capacity),
            time_in_forces: Vec::with_capacity(capacity),
            resp_types: Vec::with_capacity(capacity),
            stp_modes: Vec::with_capacity(capacity),
            peg_price_types: Vec::with_capacity(capacity),
            peg_offset_types: Vec::with_capacity(capacity),
            client_order_ids: Vec::with_capacity(capacity),
        }
    }

    /// 添加订单命令
    pub fn push(&mut self, cmd: NewOrderCmdBase) {
        self.timestamps.push(cmd.timestamp);
        self.symbols.push(cmd.symbol);
        self.sides.push(cmd.side);
        self.order_types.push(cmd.order_type);
        self.field_flags.push(cmd.field_flags);
        self.quantities.push(cmd.quantity);
        self.quote_order_qtys.push(cmd.quote_order_qty);
        self.prices.push(cmd.price);
        self.stop_prices.push(cmd.stop_price);
        self.iceberg_qtys.push(cmd.iceberg_qty);
        self.strategy_ids.push(cmd.strategy_id);
        self.strategy_types.push(cmd.strategy_type);
        self.trailing_deltas.push(cmd.trailing_delta);
        self.peg_offset_values.push(cmd.peg_offset_value);
        self.time_in_forces.push(cmd.time_in_force);
        self.resp_types.push(cmd.resp_type);
        self.stp_modes.push(cmd.stp_mode);
        self.peg_price_types.push(cmd.peg_price_type);
        self.peg_offset_types.push(cmd.peg_offset_type);
        self.client_order_ids.push(cmd.client_order_id);
    }

    /// 获取订单数量
    pub fn len(&self) -> usize {
        self.timestamps.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.timestamps.is_empty()
    }

    /// 获取指定索引的订单命令
    pub fn get(&self, index: usize) -> Option<NewOrderCmdBase> {
        if index >= self.len() {
            return None;
        }

        Some(NewOrderCmdBase {
            timestamp: self.timestamps[index],
            symbol: self.symbols[index],
            side: self.sides[index],
            order_type: self.order_types[index],
            field_flags: self.field_flags[index],
            quantity: self.quantities[index],
            quote_order_qty: self.quote_order_qtys[index],
            price: self.prices[index],
            stop_price: self.stop_prices[index],
            iceberg_qty: self.iceberg_qtys[index],
            strategy_id: self.strategy_ids[index],
            strategy_type: self.strategy_types[index],
            trailing_delta: self.trailing_deltas[index],
            peg_offset_value: self.peg_offset_values[index],
            time_in_force: self.time_in_forces[index],
            resp_type: self.resp_types[index],
            stp_mode: self.stp_modes[index],
            peg_price_type: self.peg_price_types[index],
            peg_offset_type: self.peg_offset_types[index],
            client_order_id: self.client_order_ids[index],
            _padding: [0u8; 3],
        })
    }

    /// 批量过滤：获取指定交易对的索引
    pub fn filter_by_symbol(&self, symbol: &[u8; 32]) -> Vec<usize> {
        self.symbols
            .iter()
            .enumerate()
            .filter(|(_, s)| *s == symbol)
            .map(|(i, _)| i)
            .collect()
    }

    /// 批量过滤：获取指定订单方向的索引
    pub fn filter_by_side(&self, side: u8) -> Vec<usize> {
        self.sides
            .iter()
            .enumerate()
            .filter(|(_, &s)| s == side)
            .map(|(i, _)| i)
            .collect()
    }

    /// 批量过滤：获取指定订单类型的索引
    pub fn filter_by_order_type(&self, order_type: u8) -> Vec<usize> {
        self.order_types
            .iter()
            .enumerate()
            .filter(|(_, &ot)| ot == order_type)
            .map(|(i, _)| i)
            .collect()
    }
}

impl Default for NewOrderCmdSoa {
    fn default() -> Self {
        Self::new()
    }
}

/// AOS 到 SOA 的转换
impl From<Vec<NewOrderCmdBase>> for NewOrderCmdSoa {
    fn from(cmds: Vec<NewOrderCmdBase>) -> Self {
        let mut soa = Self::with_capacity(cmds.len());
        for cmd in cmds {
            soa.push(cmd);
        }
        soa
    }
}

/// SOA 到 AOS 的转换
impl From<NewOrderCmdSoa> for Vec<NewOrderCmdBase> {
    fn from(soa: NewOrderCmdSoa) -> Self {
        (0..soa.len())
            .filter_map(|i| soa.get(i))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_cmd_base() {
        let symbol = NewOrderCmdBase::symbol_from_str("BTCUSDT");
        let mut cmd = NewOrderCmdBase::new(1000, symbol, 0, 0);

        assert_eq!(cmd.timestamp, 1000);
        assert_eq!(cmd.symbol_as_str().unwrap(), "BTCUSDT");
        assert_eq!(cmd.side, 0);
        assert_eq!(cmd.order_type, 0);

        cmd.set_quantity(100_000_000); // 1.0 BTC (固定精度)
        cmd.set_price(50000_00000000); // 50000 USDT (固定精度)

        assert!(cmd.has_quantity());
        assert!(cmd.has_price());
        assert_eq!(cmd.quantity, 100_000_000);
        assert_eq!(cmd.price, 50000_00000000);
    }

    #[test]
    fn test_soa_conversion() {
        let symbol1 = NewOrderCmdBase::symbol_from_str("BTCUSDT");
        let symbol2 = NewOrderCmdBase::symbol_from_str("ETHUSDT");

        let mut cmd1 = NewOrderCmdBase::new(1000, symbol1, 0, 0);
        cmd1.set_quantity(100_000_000);

        let mut cmd2 = NewOrderCmdBase::new(2000, symbol2, 1, 1);
        cmd2.set_quantity(200_000_000);

        let cmds = vec![cmd1, cmd2];
        let soa = NewOrderCmdSoa::from(cmds);

        assert_eq!(soa.len(), 2);
        assert_eq!(soa.timestamps[0], 1000);
        assert_eq!(soa.timestamps[1], 2000);
        assert_eq!(soa.quantities[0], 100_000_000);
        assert_eq!(soa.quantities[1], 200_000_000);
    }

    #[test]
    fn test_soa_filter() {
        let symbol = NewOrderCmdBase::symbol_from_str("BTCUSDT");
        let mut soa = NewOrderCmdSoa::new();

        for i in 0..10 {
            let mut cmd = NewOrderCmdBase::new(1000 + i, symbol, (i % 2) as u8, 0);
            cmd.set_quantity(100_000_000 * (i + 1));
            soa.push(cmd);
        }

        let buy_orders = soa.filter_by_side(0);
        assert_eq!(buy_orders.len(), 5);

        let sell_orders = soa.filter_by_side(1);
        assert_eq!(sell_orders.len(), 5);
    }
}
