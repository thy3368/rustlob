use std::collections::{BTreeMap, HashMap};

use base_types_v2::Symbol;
use common_entity::EntityReplayableEvent;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::lob::{AssetLobV2, LobOrder, OrderId};
use crate::LobError;

/// 价格点结构
///
/// 存储某个价格级别的所有订单的链表头尾指针
#[derive(Debug, Default, Clone)]
struct PricePoint {
    /// 该价格级别的第一个订单在 orders 中的索引
    first_order_idx: Option<usize>,
    /// 该价格级别的最后一个订单在 orders 中的索引
    last_order_idx: Option<usize>,
}

impl PricePoint {
    /// 将订单索引添加到链表尾部
    fn push_back(&mut self, idx: usize) {
        if self.first_order_idx.is_none() {
            self.first_order_idx = Some(idx);
        }
        self.last_order_idx = Some(idx);
    }
}

//todo 用type 代码范型

/// 订单包装器
///
/// 包装 Order trait 对象，并添加链表指针
struct OrderNode<O: LobOrder> {
    order: O,
    /// 指向同价格级别下一个订单的索引
    next_idx: Option<usize>,
}

impl<O: LobOrder> OrderNode<O> {
    fn new(order: O) -> Self {
        Self { order, next_idx: None }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
struct Px(Decimal);

pub struct LocalLobBtree<O: LobOrder> {
    /// 当前本地订单簿对应的交易标的，用于拒绝跨订单簿订单。
    symbol: Symbol,
    /// 买单价格点（key = tick 数量，自动升序排列）
    bids: BTreeMap<i64, PricePoint>,
    /// 卖单价格点（key = tick 数量，自动升序排列）
    asks: BTreeMap<i64, PricePoint>,
    /// 订单存储池
    orders: Vec<Option<OrderNode<O>>>,
    /// 订单ID到存储池索引的映射
    order_index: HashMap<OrderId, usize>,
    /// 最佳买价（最高买入价）
    bid_max: Option<Px>,
    /// 最佳卖价（最低卖出价）
    ask_min: Option<Px>,
    /// 最后一笔成交价
    last_trade_price: Option<Px>,
    /// 下一个可用的订单槽位索引
    next_slot: usize,
    /// 回放sequence
    sequence: Option<u64>,
}

impl<O: LobOrder> AssetLobV2 for crate::imp::lob_btree::LocalLobBtree<O> {
    type Order = O;

    fn maker_orders(
        &self,
        price: Decimal,
        quantity: Decimal,
        taker_side: <Self::Order as LobOrder>::Side,
    ) -> (Option<Vec<&Self::Order>>, Decimal) {
        todo!()
    }

    fn add_order(&mut self, order: Self::Order) -> Result<(), LobError> {
        todo!()
    }

    fn remove_order(&mut self, order_id: OrderId) -> Result<(), LobError> {
        todo!()
    }

    fn find_order(&self, order_id: OrderId) -> Result<Self::Order, LobError> {
        todo!()
    }

    fn best_bid(&self) -> Option<Decimal> {
        todo!()
    }

    fn best_ask(&self) -> Option<Decimal> {
        todo!()
    }

    fn symbol(&self) -> Symbol {
        todo!()
    }

    fn replay_events(&mut self, events: &[EntityReplayableEvent]) -> Result<(), LobError> {
        todo!()
    }

    fn replay_event(&mut self, event: &EntityReplayableEvent) -> Result<(), LobError> {
        todo!()
    }
}
