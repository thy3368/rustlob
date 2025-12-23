use account::Balance;
use base_types::{AssetId, OrderId, PrepTrade, Price, Quantity, Side, Timestamp, TradeId, TradingPair};

use crate::proc::trading_prep_order_proc::{OrderStatus, OrderType};

/// 内部订单状态（扩展字段用于撮合引擎）
#[derive(Debug, Clone, entity_derive::Entity)]
#[entity(id = "order_id")]
pub struct PrepOrder {
    pub order_id: OrderId,
    pub trading_pair: TradingPair,
    pub side: Side,
    pub order_type: OrderType,
    pub quantity: Quantity,
    pub price: Option<Price>,
    pub filled_quantity: Quantity,
    pub status: OrderStatus,
    pub created_at: u64,
    /// 冻结的保证金金额（用于订单取消时归还）
    pub frozen_margin: Price
}

impl PrepOrder {
    pub fn frozen_margin(&mut self, mut balance: Balance, now: Timestamp) {
        balance.deduct_balance(self.frozen_margin, now);

        self.change2reject();
    }
}


impl PrepOrder {
    pub fn Pending(
        order_id: u64, trading_pair: TradingPair, side: Side, order_type: OrderType, quantity: Quantity,
        price: Option<Price>, leverage: u8
    ) -> PrepOrder {
        let estimate_price = price.unwrap_or_else(|| Price::from_f64(50000.0));

        let required_margin = Self::calculate_required_margin(estimate_price, quantity, leverage);


        let mut internal_order = PrepOrder {
            order_id,
            trading_pair,
            side,
            order_type,
            quantity,
            price,
            filled_quantity: Quantity::from_raw(0),
            status: OrderStatus::Pending,
            created_at: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64,
            frozen_margin: required_margin
        };

        internal_order
    }

    pub fn calculate_required_margin(price: Price, quantity: Quantity, leverage: u8) -> Price {
        let notional = price.to_f64() * quantity.to_f64();
        let margin = notional / leverage as f64;
        Price::from_f64(margin)
    }
}

impl PrepOrder {
    pub fn remaining_qty(&self) -> f64 {
        let remaining_qty = self.quantity.to_f64() - self.filled_quantity.to_f64();
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


    pub fn filled_qty(&mut self, qty: f64) -> f64 {
        self.filled_quantity = Quantity::from_f64(self.filled_quantity.to_f64() + qty);

        if self.remaining_qty() <= 0.0001 {
            self.status = OrderStatus::Filled
        } else {
            self.status = OrderStatus::PartiallyFilled
        }
        self.filled_quantity.to_f64()
    }

    pub fn make_trade(&mut self, matched_order: &PrepOrder) -> PrepTrade {
        let filled = self.remaining_qty().min(matched_order.quantity.to_f64());

        self.filled_qty(filled);

        // 计算成交金额和手续费（限价单为 Maker，费率 0.02%）
        let price = matched_order.price.unwrap_or_else(|| Price::from_raw(0));
        let notional = price.to_f64() * filled;
        let fee = Price::from_f64(notional * 0.0002);

        // 创建成交记录
        let trade = PrepTrade::new(
            TradeId::generate(),
            self.order_id.clone(),
            matched_order.order_id.clone(),
            self.trading_pair,
            self.side,
            price,
            Quantity::from_f64(filled),
            fee,
            AssetId::USDT,
            true // Maker
        );

        // todo position 变化
        // todo 保证金变化

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
