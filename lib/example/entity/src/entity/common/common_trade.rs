use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::OrderSide;
use crate::account::symbol::Symbol;
use crate::entity::common::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TradeBase {
    /// 本系统稳定成交 ID。
    pub trade_id: u64,

    /// 交易对，例如 `BTCUSDT`。
    pub symbol: Symbol,

    pub taker_user: UserId,
    pub maker_user: UserId,

    /// 主动吃单订单 Hyperliquid spot `oid`。
    pub taker_order_id: u64,
    /// 被动挂单订单 Hyperliquid spot `oid`。
    pub maker_order_id: u64,

    /// taker 买卖方向。
    pub taker_side: OrderSide,
    /// 成交价格，取 maker 限价。
    pub price: Decimal,
    /// 成交数量，以 base asset 计价。
    pub size: Decimal,
    /// taker 为本笔成交支付的 quote 手续费金额。
    pub taker_fee: Decimal,
    /// maker 为本笔成交支付的 quote 手续费金额。
    pub maker_fee: Decimal,

    /// 订单创建时间，单位为 Unix 纳秒。
    pub created_at: u64,
    /// 订单最后更新时间，单位为 Unix 纳秒。
    pub updated_at: u64,
    /// 当前订单实体版本，用于生成可重放更新事件。
    pub version: u64,
}
