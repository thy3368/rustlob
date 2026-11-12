use std::collections::HashMap;

use common_entity::EntityError;
use example_core_entity::account::symbol::Symbol;
use example_core_entity::{BalanceV2, OrderSide, OrderType, SpotOrderV7};
use rust_decimal::Decimal;

use crate::pipeline::place_spot_order::SpotOrderV3Balances;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceNormalSpotOrderV3AfterChanges {
    /// 没有成交，订单继续留在订单簿；本次没有其它业务副作用。
    Resting { taker_order_after: SpotOrderV7, balances_after: BalanceV2 },
    /// 发生成交但仍有剩余数量，订单继续可撮合。
    PartiallyFilled {
        taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        balances_after: Vec<BalanceV2>,
        balances_by_order_id: HashMap<u64, SpotOrderV3Balances>,
        created_trades: Vec<SpotTrade3>,
    },
    /// 成交数量达到订单总数量，订单终止为 Filled。
    Filled {
        filled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        balances_after: Vec<BalanceV2>,
        created_trades: Vec<SpotTrade3>,
    },
    /// IOC 部分成交后，剩余数量被取消并释放冻结。
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV7,
        maker_orders_after: Vec<SpotOrderV7>,
        balances_after: Vec<BalanceV2>,
        created_trades: Vec<SpotTrade3>,
    },
    /// ALO crossing maker 或 IOC 无流动性导致拒单。
    Rejected { rejected_taker_order_after: SpotOrderV7, balances_after: Vec<BalanceV2> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceSpotOrderV3OrderCmd {
    pub party_id: u64,
    pub order_id: u64,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub price: Decimal,
    pub size: Decimal,
    pub order_type: OrderType,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceSpotOrderV3Cmd {
    Single(crate::pipeline::place_spot_order::PlaceSpotOrderV3OrderCmd),
    NormalTpsl {
        parent: crate::pipeline::place_spot_order::PlaceSpotOrderV3OrderCmd,
        children: Vec<crate::pipeline::place_spot_order::PlaceSpotOrderV3OrderCmd>,
    },
}

pub struct PlaceDomainService;

impl PlaceDomainService {
    pub fn place_order_pipeline(
        cmd: PlaceSpotOrderV3Cmd,
        mut makers: Vec<SpotOrderV7>,
        mut taker_balances: SpotOrderV3Balances,
        balances_by_order_id: HashMap<u64, SpotOrderV3Balances>,
    ) -> Result<PlaceNormalSpotOrderV3AfterChanges, EntityError> {
        todo!()
    }
}
