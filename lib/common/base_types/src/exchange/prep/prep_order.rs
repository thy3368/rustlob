use crate::account::balance::Balance;
use crate::exchange::spot::spot_types::OrderType;
use crate::lob::lob::LobOrder;
use crate::{
    AccountId, AssetId, OrderId, OrderSide, PositionSide, PrepPosition, PrepTrade, Price, Quantity,
    Timestamp, TradeId, TradingPair,
};

/// 订单有效期
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TimeInForce {
    /// Good Till Cancel - 取消前有效
    GTC = 1,
    /// Immediate Or Cancel - 立即成交否则取消
    IOC = 2,
    /// Fill or Kill - 全部成交否则取消
    FOK = 3,
}

impl TimeInForce {
    pub const fn as_str(self) -> &'static str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
        }
    }
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GTC
    }
}
// /// 订单类型
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// #[repr(u8)]
// pub enum OrderType {
//     /// 市价单
//     Market = 1,
//     /// 限价单
//     Limit = 2,
// }
//
// impl OrderType {
//     /// 转换为字符串
//     #[inline(always)]
//     pub const fn as_str(self) -> &'static str {
//         match self {
//             OrderType::Market => "MARKET",
//             OrderType::Limit => "LIMIT",
//         }
//     }
// }
//
// impl Default for OrderType {
//     fn default() -> Self {
//         OrderType::Limit
//     }
// }

/// 订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
// 参考下竞品，u本位永续合约 都有哪些status
pub enum FutureOrderStatus {
    /// 等待提交
    Pending = 1,
    /// 已提交
    Submitted = 2,
    /// 部分成交
    PartiallyFilled = 3,
    /// 完全成交
    Filled = 4,
    /// 已取消（用户主动）
    Cancelled = 5,
    /// 已拒绝（引擎拒绝）
    Rejected = 6,
    /// 已过期（GTC/FOK/IOC 过期）
    Expired = 7,
    /// 待取消（取消请求中）
    PendingCancel = 8,
}

impl FutureOrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            FutureOrderStatus::Pending => "PENDING",
            FutureOrderStatus::Submitted => "SUBMITTED",
            FutureOrderStatus::PartiallyFilled => "PARTIALLY_FILLED",
            FutureOrderStatus::Filled => "FILLED",
            FutureOrderStatus::Cancelled => "CANCELLED",
            FutureOrderStatus::Rejected => "REJECTED",
            FutureOrderStatus::Expired => "EXPIRED",
            FutureOrderStatus::PendingCancel => "PENDING_CANCEL",
        }
    }

    /// 是否为最终状态
    pub const fn is_final(self) -> bool {
        matches!(
            self,
            FutureOrderStatus::Filled
                | FutureOrderStatus::Cancelled
                | FutureOrderStatus::Rejected
                | FutureOrderStatus::Expired
        )
    }
}

impl Default for FutureOrderStatus {
    fn default() -> Self {
        FutureOrderStatus::Pending
    }
}

/// 内部订单状态（扩展字段用于撮合引擎）
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "order_id")]
// 参考竞口 永续合约order都有哪些字段？
pub struct PrepOrder {
    pub order_id: OrderId,
    /// todo remove account_id 账户ID（固定账户）
    pub account_id: AccountId,
    pub trading_pair: TradingPair,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: Quantity,
    pub price: Option<Price>,
    pub filled_quantity: Quantity,
    pub status: FutureOrderStatus,
    pub created_at: u64,
    /// 冻结的保证金金额（用于订单取消时归还）
    pub frozen_margin: Quantity,
    pub leverage: u8,
    /// 客户端订单ID
    pub client_order_id: Option<String>,
    /// 持仓方向（LONG/SHORT/BOTH）
    pub position_side: PositionSide,
    /// 订单有效期（GTC/IOC/FOK）
    pub time_in_force: TimeInForce,
    /// 平均成交价
    pub avg_price: Option<Price>,
    /// 累计成交额
    pub cum_quote: Option<Price>,
    /// 触发价格（止损/止盈）
    pub stop_price: Option<Price>,
    /// 只减仓
    pub reduce_only: bool,
    /// 全平
    pub close_position: bool,
    /// 更新时间
    pub update_time: u64,
}

impl PrepOrder {
    pub fn frozen_margin(&mut self, mut balance: Balance, now: Timestamp) {
        assert!(self.status == FutureOrderStatus::Pending, "Pending状态才能冻结");

        let estimate_price = self.price.unwrap_or_else(|| Price::from_f64(50000.0));
        // 直接使用 Price，无需转换
        self.frozen_margin =
            Self::calculate_required_margin(estimate_price, self.quantity, self.leverage);
        balance.frozen(self.frozen_margin, now);
        // todo 冻结不成功 则reject
        self.change2reject();
    }

    pub fn pending(
        order_id: u64,
        account_id: AccountId,
        trading_pair: TradingPair,
        side: OrderSide,
        order_type: OrderType,
        quantity: Quantity,
        price: Option<Price>,
        leverage: u8,
    ) -> PrepOrder {
        let internal_order = PrepOrder {
            order_id,
            account_id,
            trading_pair,
            side,
            order_type,
            quantity,
            price,
            filled_quantity: Quantity::from_raw(0),
            status: FutureOrderStatus::Pending,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            frozen_margin: Price::from_raw(0),
            leverage,
            client_order_id: None,
            position_side: PositionSide::Long,
            time_in_force: TimeInForce::GTC,
            avg_price: None,
            cum_quote: None,
            stop_price: None,
            reduce_only: false,
            close_position: false,
            update_time: 0,
        };

        internal_order
    }

    pub fn calculate_required_margin(price: Price, quantity: Quantity, leverage: u8) -> Price {
        let notional = price.to_f64() * quantity.to_f64();
        let margin = notional / leverage as f64;
        Price::from_f64(margin)
    }

    pub fn remaining_qty(&self) -> i64 {
        let remaining_qty = self.quantity.raw() - self.filled_quantity.raw();
        remaining_qty
    }

    pub fn change2submit(&mut self) {
        // 如果filled_quantity==0
        self.status = FutureOrderStatus::Submitted;
    }

    pub fn change2reject(&mut self) {
        // 如果filled_quantity==0
        self.status = FutureOrderStatus::Rejected;
    }

    // 取消订单
    pub fn cancel(&mut self, balance: &mut Balance, now: Timestamp) {
        // 如果filled_quantity==0
        self.status = FutureOrderStatus::Rejected;

        balance.un_frozen(self.frozen_margin, now);
        self.frozen_margin = Price::from_raw(0);
    }

    pub fn frozen2pay(&mut self, balance: &mut Balance, now: Timestamp) {
        // 直接使用 Price，无需转换

        balance.frozen2pay(self.frozen_margin, now);
    }

    pub fn filled_qty(
        &mut self,
        balance: &mut Balance,
        match_p: &mut PrepPosition,
        qty: i64,
        now: Timestamp,
    ) -> f64 {
        match self.side {
            OrderSide::Buy => {}
            OrderSide::Sell => {}
        }

        self.filled_quantity = Quantity::from_raw(self.filled_quantity.raw() + qty);

        // 真实资金操作
        self.frozen2pay(balance, now);

        // 处理 position - 更新被匹配方的持仓
        let fill_qty = Quantity::from_raw(qty);
        let fill_price = self.price.unwrap_or_else(|| Price::from_f64(50000.0));

        // 直接调用 position.update() 更新持仓
        match_p.add(fill_qty, fill_price, self.leverage, self.side, crate::PositionSide::Long);

        if self.remaining_qty() == 0 {
            self.status = FutureOrderStatus::Filled
        } else {
            self.status = FutureOrderStatus::PartiallyFilled
        }
        self.filled_quantity.to_f64()
    }

    // todo 每个order会有两个balance, 根据买卖来决定资产流转
    pub fn make_trade(
        &mut self,
        matched_order: &mut PrepOrder,
        matched_b: &mut Balance,
        matched_p: &mut PrepPosition,
        my_b: &mut Balance,
        my_p: &mut PrepPosition,
        now: Timestamp,
    ) -> PrepTrade {
        let filled = self.remaining_qty().min(matched_order.quantity.raw());

        self.filled_qty(my_b, my_p, filled, now);
        matched_order.filled_qty(matched_b, matched_p, filled, now);

        matched_b.frozen2pay(Price::from_raw(0), now);

        // 计算成交金额和手续费（限价单为 Maker，费率 0.02%）
        let price = matched_order.price.unwrap_or_else(|| Price::from_raw(0));
        let notional = price.to_f64() * (filled as f64);
        let fee = Price::from_f64(notional * 0.0002);

        // 创建成交记录
        let trade = PrepTrade::new(
            TradeId::generate(),
            self.order_id.clone(),
            matched_order.order_id.clone(),
            self.trading_pair,
            self.side,
            price,
            Quantity::from_raw(filled),
            fee,
            AssetId::Usdt,
            true, // Maker
        );

        // position 变化已在 filled_qty 方法中处理

        trade
    }
}

impl PrepOrder {
    pub fn is_all_filled(&self) -> bool {
        self.quantity == self.filled_quantity
    }
}
/// 实现 Order trait 以适配 LOB 仓储
impl LobOrder for PrepOrder {
    fn order_id(&self) -> crate::OrderId {
        self.order_id
    }

    fn price(&self) -> Price {
        self.price.unwrap_or_else(|| Price::from_raw(0))
    }

    fn quantity(&self) -> Quantity {
        self.quantity
    }

    fn filled_quantity(&self) -> Quantity {
        self.filled_quantity
    }

    fn side(&self) -> crate::OrderSide {
        self.side
    }

    fn symbol(&self) -> TradingPair {
        self.trading_pair
    }
}
