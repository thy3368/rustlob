//! 持仓相关类型定义
//!
//! 包含持仓信息、价格、数量、交易对符号等核心类型

use std::fmt;

use crate::base_types::{
    AssetId, Decimal, OrderId, OrderSide, PositionId, Price, Quantity, Timestamp, TradeId,
    TradingPair, UserId,
};

// ============================================================================
// 持仓相关类型定义
// ============================================================================

/// 持仓方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositionSide {
    /// 多头（做多）
    Long,
    /// 空头（做空）
    Short,
    /// 双向持仓模式
    Both,
}

impl fmt::Display for PositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSide::Long => write!(f, "LONG"),
            PositionSide::Short => write!(f, "SHORT"),
            PositionSide::Both => write!(f, "BOTH"),
        }
    }
}

impl Default for PositionSide {
    fn default() -> Self {
        PositionSide::Long
    }
}

// ============================================================================
// 持仓信息结构体
// ============================================================================

//todo fix "entity_derive::Entity" 报钷 "field `change_type` of struct `ChangeLogEntry` is private [E0616]
//
// field `entity_id` of struct `ChangeLogEntry` is private [E0616]"
/// 持仓信息
///
/// ## 字段说明（参考 Binance USDT-Margined Futures Position Information V3）
///
/// | 字段 | 说明 |
/// |------|------|
/// | symbol | 交易对 |
/// | positionSide | 持仓方向 (LONG/SHORT/BOTH) |
/// | positionAmt | 持仓数量 |
/// | entryPrice | 开仓价格 |
/// | breakEvenPrice | 盈亏平衡价格 |
/// | markPrice | 标记价格 |
/// | unRealizedProfit | 未实现盈亏 |
/// | liquidationPrice | 强平价格 |
/// | isolatedMargin | 逐仓保证金 |
/// | notional | 名义价值 |
/// | marginAsset | 保证金资产 |
/// | isolatedWallet | 逐仓钱包 |
/// | initialMargin | 初始保证金 |
/// | maintMargin | 维持保证金 |
/// | positionInitialMargin | 持仓初始保证金 |
/// | openOrderInitialMargin | 挂单初始保证金 |
/// | adl | 自动减仓排序 |
/// | bidNotional | 买一价名义价值 |
/// | askNotional | 卖一价名义价值 |
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "position_id")]
pub struct PrepPosition {
    /// 所属用户ID
    pub user_id: UserId,
    /// 持仓ID
    pub position_id: PositionId,

    /// 交易对（包含 base_asset 和 quote_asset）
    pub trading_pair: TradingPair,
    /// 持仓方向
    pub position_side: PositionSide,
    /// 持仓数量（正数表示多头，负数表示空头）
    pub quantity: Quantity,
    /// 开仓价格（持仓均价）
    pub entry_price: Price,
    /// 盈亏平衡价格
    pub break_even_price: Price,
    /// 标记价格（用于计算未实现盈亏）
    pub mark_price: Price,
    /// 未实现盈亏
    pub unrealized_pnl: Price,
    /// 已实现盈亏
    pub realized_pnl: Price,
    /// 杠杆倍数
    pub leverage: u8,
    /// 保证金资产（如 USDT）
    pub margin_asset: AssetId,
    /// 保证金总额
    pub margin: Price,
    /// 初始保证金
    pub initial_margin: Price,
    /// 维持保证金
    pub maint_margin: Price,
    /// 持仓初始保证金（以当前标记价格计算）
    pub position_initial_margin: Price,
    /// 挂单初始保证金（以当前标记价格计算）
    pub open_order_initial_margin: Price,
    /// 逐仓保证金（逐仓模式）
    pub isolated_margin: Price,
    /// 逐仓钱包（逐仓模式）
    pub isolated_wallet: Price,
    /// 名义价值（持仓数量 * 标记价格）
    pub notional: Price,
    /// 强平价格
    pub liquidation_price: Option<Price>,
    /// 自动减仓排序（ADL）
    pub adl: i32,
    /// 买一价名义价值
    pub bid_notional: Price,
    /// 卖一价名义价值
    pub ask_notional: Price,
    /// 更新时间戳（毫秒）
    pub updated_at: Timestamp,
}

impl PrepPosition {
    /// 创建空持仓
    pub fn empty(trading_pair: TradingPair, position_side: PositionSide) -> Self {
        Self {
            user_id: UserId(0),
            position_id: PositionId::generate(),
            trading_pair,
            position_side,
            quantity: Quantity::ZERO,
            entry_price: Price::ZERO,
            break_even_price: Price::ZERO,
            mark_price: Price::ZERO,
            unrealized_pnl: Price::ZERO,
            realized_pnl: Price::ZERO,
            leverage: 1,
            margin_asset: AssetId::from_str("USDT").unwrap(),
            margin: Price::ZERO,
            initial_margin: Price::ZERO,
            maint_margin: Price::ZERO,
            position_initial_margin: Price::ZERO,
            open_order_initial_margin: Price::ZERO,
            isolated_margin: Price::ZERO,
            isolated_wallet: Price::ZERO,
            notional: Price::ZERO,
            liquidation_price: None,
            adl: 0,
            bid_notional: Price::ZERO,
            ask_notional: Price::ZERO,
            updated_at: Timestamp::now_as_nanos(),
        }
    }

    /// 是否有持仓
    pub fn has_position(&self) -> bool {
        self.quantity.is_positive()
    }

    /// 是否为多头
    pub fn is_long(&self) -> bool {
        self.position_side == PositionSide::Long && self.quantity.is_positive()
    }

    /// 是否为空头
    pub fn is_short(&self) -> bool {
        self.position_side == PositionSide::Short && self.quantity.is_positive()
    }

    /// 计算下次资金费用
    pub fn calculate_next_funding_fee(&self, funding_rate: Price) -> Price {
        if !self.has_position() {
            return Price::ZERO;
        }

        let base_fee = self.mark_price * self.quantity * funding_rate;

        if self.position_side == PositionSide::Long { -base_fee } else { base_fee }
    }

    #[allow(dead_code)]
    fn calculate_liquidation_price(&self, position: &PrepPosition) -> Option<Price> {
        if !position.has_position() {
            return None;
        }

        let maintenance_margin_rate = Decimal::new(4, 3); // 0.4% 维持保证金率
        let entry = position.entry_price;
        let leverage = Decimal::from(position.leverage);

        let liq_price = match position.position_side {
            PositionSide::Long => {
                // 多仓：价格下跌到此价格时强平
                entry * (Decimal::ONE - Decimal::ONE / leverage + maintenance_margin_rate)
            }
            PositionSide::Short => {
                // 空仓：价格上涨到此价格时强平
                entry * (Decimal::ONE + Decimal::ONE / leverage - maintenance_margin_rate)
            }
            PositionSide::Both => {
                // 单向模式，暂时按多仓处理
                entry * (Decimal::ONE - Decimal::ONE / leverage + maintenance_margin_rate)
            }
        };

        Some(if liq_price.is_sign_negative() { Price::ZERO } else { liq_price })
    }

    #[allow(dead_code)]
    fn calculate_unrealized_pnl(&self, position: &PrepPosition) -> Price {
        if !position.has_position() {
            return Price::ZERO;
        }

        let pnl = match position.position_side {
            PositionSide::Long => (position.mark_price - position.entry_price) * position.quantity,
            PositionSide::Short => (position.entry_price - position.mark_price) * position.quantity,
            PositionSide::Both => {
                // 单向持仓模式，根据数量符号判断
                (position.mark_price - position.entry_price) * position.quantity
            }
        };

        pnl
    }

    /// 更新持仓数量、均价、杠杆和相关计算字段
    ///
    /// # 参数
    /// - `new_quantity`: 新成交数量
    /// - `new_price`: 新成交价格
    /// - `leverage`: 杠杆倍数
    /// - `side`: 订单方向
    /// - `position_side`: 持仓方向
    pub fn add(
        &mut self,
        new_quantity: Quantity,
        new_price: Price,
        leverage: u8,
        _side: crate::OrderSide,
        _position_side: crate::PositionSide,
    ) {
        // 计算新的持仓数量和均价（加权平均）
        let total_cost = self.quantity * self.entry_price + new_quantity * new_price;
        let total_qty = self.quantity + new_quantity;

        // 更新持仓数量和均价
        self.quantity = total_qty;
        self.entry_price =
            if total_qty > Quantity::ZERO { total_cost / total_qty } else { Price::ZERO };

        // 更新标记价格
        self.mark_price = new_price;

        // 更新盈亏平衡价格（简化处理：等于开仓均价）
        self.break_even_price = self.entry_price;

        // 更新杠杆
        self.leverage = leverage;

        // 计算名义价值 = 持仓数量 * 标记价格
        self.notional = self.mark_price * self.quantity;

        // 计算保证金 = 名义价值 / 杠杆倍数
        self.margin = self.notional / Decimal::from(leverage);

        // 计算初始保证金 = 名义价值 / 杠杆倍数
        self.initial_margin = self.margin;

        // 计算维持保证金 = 名义价值 * 维持保证金率 (0.5%)
        let maintenance_margin_rate = Decimal::new(5, 3);
        self.maint_margin = self.notional * maintenance_margin_rate;

        // 持仓初始保证金 = 初始保证金
        self.position_initial_margin = self.initial_margin;

        // 挂单初始保证金（暂无挂单）
        self.open_order_initial_margin = Price::ZERO;

        // 计算未实现盈亏
        self.unrealized_pnl = self.calculate_unrealized_pnl_value();

        // 计算强平价格
        self.liquidation_price = self.calculate_liquidation_price_value();

        // 更新时间戳
        self.updated_at = Timestamp::now_as_nanos();
    }

    /// 计算未实现盈亏值
    fn calculate_unrealized_pnl_value(&self) -> Price {
        if !self.has_position() {
            return Price::ZERO;
        }

        let pnl = match self.position_side {
            PositionSide::Long => (self.mark_price - self.entry_price) * self.quantity,
            PositionSide::Short => (self.entry_price - self.mark_price) * self.quantity,
            PositionSide::Both => {
                // 单向持仓模式，根据数量符号判断
                (self.mark_price - self.entry_price) * self.quantity
            }
        };

        pnl
    }

    /// 计算强平价格值
    fn calculate_liquidation_price_value(&self) -> Option<Price> {
        if !self.has_position() {
            return None;
        }

        let maintenance_margin_rate = Decimal::new(4, 3); // 0.4% 维持保证金率
        let entry = self.entry_price;
        let leverage = Decimal::from(self.leverage);

        let liq_price = match self.position_side {
            PositionSide::Long => {
                // 多仓：价格下跌到此价格时强平
                entry * (Decimal::ONE - Decimal::ONE / leverage + maintenance_margin_rate)
            }
            PositionSide::Short => {
                // 空仓：价格上涨到此价格时强平
                entry * (Decimal::ONE + Decimal::ONE / leverage - maintenance_margin_rate)
            }
            PositionSide::Both => {
                // 单向模式，暂时按多仓处理
                entry * (Decimal::ONE - Decimal::ONE / leverage + maintenance_margin_rate)
            }
        };

        Some(if liq_price.is_sign_negative() { Price::ZERO } else { liq_price })
    }

    /// 更新已实现盈亏
    ///
    /// # 参数
    /// - `pnl`: 盈亏金额
    pub fn update_realized_pnl(&mut self, pnl: Price) {
        self.realized_pnl = self.realized_pnl + pnl;
    }
}

// ============================================================================
// 成交相关类型定义
// ============================================================================

/// 成交记录（单次撮合成交）
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "trade_id")]
pub struct PrepTrade {
    /// 成交ID
    pub trade_id: TradeId,
    /// 主动订单ID
    pub taker_order_id: OrderId,
    /// 被动订单ID
    pub maker_order_id: OrderId,
    /// 交易对
    pub trading_pair: TradingPair,
    /// 主动方向
    pub taker_side: OrderSide,
    /// 成交价格
    pub price: Price,
    /// 成交数量
    pub quantity: Quantity,
    /// 手续费
    pub fee: Quantity,
    /// 手续费资产（通常是USDT）
    pub fee_asset: AssetId,
    /// 是否为Maker（流动性提供方）//todo 怎么判断？
    pub is_maker: bool,
    /// 成交时间戳（毫秒）
    pub timestamp: Timestamp,
}

impl PrepTrade {
    /// 创建新的成交记录
    pub fn new(
        trade_id: TradeId,
        ask_order_id: OrderId,
        bid_order_id: OrderId,
        symbol: TradingPair,
        side: OrderSide,
        price: Price,
        quantity: Quantity,
        fee: Price,
        fee_asset: AssetId,
        is_maker: bool,
    ) -> Self {
        Self {
            trade_id,
            taker_order_id: ask_order_id,
            maker_order_id: bid_order_id,
            trading_pair: symbol,
            taker_side: side,
            price,
            quantity,
            fee,
            fee_asset,
            is_maker,
            timestamp: Timestamp::now_as_nanos(),
        }
    }

    /// 计算成交金额（价格 * 数量）
    pub fn notional(&self) -> Price {
        self.price * self.quantity
    }
}
