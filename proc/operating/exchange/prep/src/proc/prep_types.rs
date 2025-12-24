use account::Balance;
use base_types::{
    AccountId, AssetId, OrderId, PrepPosition, PrepTrade, Price, Quantity, Side, Timestamp, TradeId, TradingPair
};

use crate::proc::trading_prep_order_proc::{OrderStatus, OrderType};

/// 内部订单状态（扩展字段用于撮合引擎）
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "order_id")]
pub struct PrepOrder {
    pub order_id: OrderId,
    /// 账户ID（固定账户）
    pub account_id: AccountId,
    pub trading_pair: TradingPair,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: Quantity,
    pub price: Option<Price>,
    pub filled_quantity: Quantity,
    pub status: OrderStatus,
    pub created_at: u64,
    /// 冻结的保证金金额（用于订单取消时归还）
    pub frozen_margin: Price,
    pub leverage: u8
}


impl PrepOrder {
    pub fn frozen_margin(&mut self, mut balance: Balance, now: Timestamp) {
        assert!(self.status == OrderStatus::Pending, "Pending状态才能冻结");

        let estimate_price = self.price.unwrap_or_else(|| Price::from_f64(50000.0));
        // 直接使用 Price，无需转换
        self.frozen_margin = Self::calculate_required_margin(estimate_price, self.quantity, self.leverage);
        balance.frozen(self.frozen_margin, now);
        // todo 冻结不成功 则reject
        self.change2reject();
    }


    pub fn pending(
        order_id: u64, account_id: AccountId, trading_pair: TradingPair, side: Side, order_type: OrderType,
        quantity: Quantity, price: Option<Price>, leverage: u8
    ) -> PrepOrder {
        let mut internal_order = PrepOrder {
            order_id,
            account_id,
            trading_pair,
            side,
            order_type,
            quantity,
            price,
            filled_quantity: Quantity::from_raw(0),
            status: OrderStatus::Pending,
            created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
            frozen_margin: Price::from_raw(0),
            leverage
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
        self.status = OrderStatus::Submitted;
    }

    pub fn change2reject(&mut self) {
        // 如果filled_quantity==0
        self.status = OrderStatus::Rejected;
    }


    // 取消订单
    pub fn cancel(&mut self, balance: &mut Balance, now: Timestamp) {
        // 如果filled_quantity==0
        self.status = OrderStatus::Rejected;

        balance.un_frozen(self.frozen_margin, now);
        self.frozen_margin = Price::from_raw(0);
    }


    pub fn frozen2pay(&mut self, balance: &mut Balance, now: Timestamp) {
        // 直接使用 Price，无需转换

        balance.frozen2pay(self.frozen_margin, now);
    }


    pub fn filled_qty(&mut self, balance: &mut Balance, match_p: &mut PrepPosition, qty: i64, now: Timestamp) -> f64 {
        match self.side {
            Side::Buy => {}
            Side::Sell => {}
        }

        self.filled_quantity = Quantity::from_raw(self.filled_quantity.raw() + qty);

        // 真实资金操作
        self.frozen2pay(balance, now);

        // 处理 position - 更新被匹配方的持仓
        let fill_qty = Quantity::from_raw(qty);
        let fill_price = self.price.unwrap_or_else(|| Price::from_f64(50000.0));

        // 直接调用 position.update() 更新持仓
        match_p.add(
            fill_qty,
            fill_price,
            self.leverage,
            self.side,
            crate::proc::trading_prep_order_proc::PositionSide::Long
        );

        if self.remaining_qty() == 0 {
            self.status = OrderStatus::Filled
        } else {
            self.status = OrderStatus::PartiallyFilled
        }
        self.filled_quantity.to_f64()
    }


    // todo 每个order会有两个balance, 根据买卖来决定资产流转
    pub fn make_trade(
        &mut self, matched_order: &mut PrepOrder, matched_b: &mut Balance, matched_p: &mut PrepPosition,
        my_b: &mut Balance, my_p: &mut PrepPosition, now: Timestamp
    ) -> PrepTrade {
        let filled = self.remaining_qty().min(matched_order.quantity.raw());

        self.filled_qty(my_b, my_p, filled, now);
        matched_order.filled_qty(matched_b, matched_p, filled, now);


        matched_b.frozen2pay(Price::from_raw(0), now);


        // 计算成交金额和手续费（限价单为 Maker，费率 0.02%）
        let price = matched_order.price.unwrap_or_else(|| Price::from_raw(0));
        let notional = price.raw() * filled;
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
            AssetId::USDT,
            true // Maker
        );

        // position 变化已在 filled_qty 方法中处理

        trade
    }
}

impl PrepOrder {
    pub fn is_all_filled(&self) -> bool { self.quantity == self.filled_quantity }
}
/// 实现 Order trait 以适配 LOB 仓储
impl lob_repo::core::symbol_lob_repo::Order for PrepOrder {
    fn order_id(&self) -> base_types::OrderId { self.order_id }

    fn price(&self) -> Price { self.price.unwrap_or_else(|| Price::from_raw(0)) }

    fn quantity(&self) -> Quantity { self.quantity }

    fn filled_quantity(&self) -> Quantity { self.filled_quantity }

    fn side(&self) -> base_types::Side { self.side }

    fn symbol(&self) -> TradingPair { self.trading_pair }
}
