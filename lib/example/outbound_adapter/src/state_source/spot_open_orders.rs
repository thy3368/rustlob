use common_entity::ReadModelSource;
use example_core_use_case::{OpenSpotOrdersQuery, SpotOrderV7ReadModel};

use crate::{HyperliquidScheduleCancelInMemoryOutbound, HyperliquidScheduleCancelOutboundError};

pub struct ScheduleCancelSpotOrderSource<'a> {
    outbound: &'a HyperliquidScheduleCancelInMemoryOutbound,
}

impl<'a> ScheduleCancelSpotOrderSource<'a> {
    pub fn new(outbound: &'a HyperliquidScheduleCancelInMemoryOutbound) -> Self {
        Self { outbound }
    }
}

impl ReadModelSource<OpenSpotOrdersQuery> for ScheduleCancelSpotOrderSource<'_> {
    type ReadModel = SpotOrderV7ReadModel;
    type Error = HyperliquidScheduleCancelOutboundError;

    fn load_read_model(
        &self,
        _query: &OpenSpotOrdersQuery,
    ) -> Result<Self::ReadModel, Self::Error> {
        Ok(SpotOrderV7ReadModel(self.outbound.snapshot()?.spot_orders))
    }
}

#[cfg(test)]
mod tests {
    use common_entity::ReadModelSource;
    use example_core_use_case::account::coin::Coin;
    use example_core_use_case::account::symbol::Symbol;
    use example_core_use_case::common::common_order::OrderBase;
    use example_core_use_case::spot::SpotOrderV7;
    use example_core_use_case::{OrderSide, SpotOrderStatus, SpotOrderV7, UserId};
    use rust_decimal::Decimal;

    use super::*;

    const USER: &str = "0x1111111111111111111111111111111111111111";

    #[test]
    fn source_loads_spot_order_v7_snapshot() {
        let outbound = HyperliquidScheduleCancelInMemoryOutbound::default();
        outbound.seed_spot_order(order(7)).expect("应写入测试订单");
        let source = ScheduleCancelSpotOrderSource::new(&outbound);

        let loaded = source
            .load_read_model(&OpenSpotOrdersQuery { user: USER.to_string(), dex: None })
            .expect("应加载快照");

        assert_eq!(loaded.0.len(), 1);
        assert_eq!(loaded.0[0].order_base().order_id, 7);
    }

    fn order(order_id: u64) -> SpotOrderV7 {
        SpotOrderV7::from_record(OrderBase {
            owner: UserId(common_entity::stable_string_entity_id(USER)),
            order_id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            limit_px: Decimal::new(29_792, 1),
            size: Decimal::new(15, 1),
            orig_size: Decimal::from(2),
            status: SpotOrderStatus::Open,
            created_at: 10,
            updated_at: 10,
            ..OrderBase::default()
        })
        .expect("测试订单应有效")
    }
}
