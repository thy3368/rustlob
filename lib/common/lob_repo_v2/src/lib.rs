use std::collections::HashMap;

use example_core_entity::{SpotOrderSide, SpotOrderStatus, SpotOrderV2};

/// 现货订单簿。订单簿只保存可撮合的活跃订单，完整订单仍由调用方持有。
#[derive(Debug, Clone, Default)]
pub struct LobRepoV2 {
    books: HashMap<(u32, String), Vec<SpotOrderV2>>,
}

impl LobRepoV2 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn upsert(&mut self, order: SpotOrderV2) {
        self.remove(order.asset(), order.symbol(), order.order_id());
        if is_active(&order) {
            self.books.entry((order.asset(), order.symbol().to_owned())).or_default().push(order);
        }
    }

    pub fn remove(&mut self, asset: u32, symbol: &str, order_id: u64) {
        if let Some(orders) = self.books.get_mut(&(asset, symbol.to_owned())) {
            orders.retain(|order| order.order_id() != order_id);
        }
    }

    pub fn seed_order(&mut self, order: SpotOrderV2) {
        self.upsert(order);
    }

    pub fn maker_orders(
        &self,
        asset: u32,
        symbol: &str,
        taker_side: SpotOrderSide,
    ) -> Vec<SpotOrderV2> {
        let mut orders = self.books.get(&(asset, symbol.to_owned())).cloned().unwrap_or_default();
        orders.retain(|order| order.side() != taker_side);
        orders.sort_by(|a, b| {
            let price = match a.side() {
                SpotOrderSide::Buy => b.limit_price.cmp(&a.limit_price),
                SpotOrderSide::Sell => a.limit_price.cmp(&b.limit_price),
            };
            price
                .then_with(|| a.created_at.cmp(&b.created_at))
                .then_with(|| a.order_id().cmp(&b.order_id()))
        });
        orders
    }

    pub fn clear(&mut self) {
        self.books.clear();
    }
}

fn is_active(order: &SpotOrderV2) -> bool {
    matches!(order.status(), SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled)
        && order.filled_qty() < order.qty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sorts_and_isolates_books() {
        let mut lob = LobRepoV2::new();
        let mut orders = Vec::new();
        for (id, price, asset) in [(1, 101, 1), (2, 100, 1), (3, 101, 1), (4, 1, 2)] {
            let mut o = SpotOrderV2::new_pending_limit(
                id,
                asset,
                "a".into(),
                "BTCUSDT".into(),
                SpotOrderSide::Buy,
                1,
                price,
                example_core_entity::SpotOrderType::Limit {
                    tif: example_core_entity::SpotOrderTif::Gtc,
                },
                None,
                1,
                0,
            );
            o.status = SpotOrderStatus::Open;
            orders.push(o);
        }
        for o in orders {
            lob.upsert(o);
        }
        let got = lob.maker_orders(1, "BTCUSDT", SpotOrderSide::Sell);
        assert_eq!(got.iter().map(|o| o.order_id()).collect::<Vec<_>>(), vec![1, 3, 2]);
        assert!(lob.maker_orders(2, "BTCUSDT", SpotOrderSide::Sell).len() == 1);
    }
}
