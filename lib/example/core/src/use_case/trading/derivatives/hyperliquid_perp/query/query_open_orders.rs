use cmd_handler::command_use_case_def2::IssuedByParty;
use cmd_handler::query_use_case_def::QueryUseCase;
use thiserror::Error;

use crate::{
    HyperliquidPerpOrder, HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide,
    HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
};

/// 查询账户当前开放中的 Hyperliquid perp 委托单列表。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpOpenOrders {
    /// 发起查询的业务账户 ID。
    pub party_id: String,
    /// 可选合约过滤；为空时表示查询该账户全部开放单。
    pub symbol: Option<String>,
}

impl IssuedByParty for QueryHyperliquidPerpOpenOrders {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 查询 Hyperliquid perp 开放单列表时需要的已加载读模型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpOpenOrdersReadModel {
    /// 已加载的开放单候选集合。
    pub orders: Vec<HyperliquidPerpOrder>,
}

/// 单条 Hyperliquid perp 开放委托单的业务视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpOpenOrderView {
    pub order_id: String,
    pub exchange_oid: Option<u64>,
    pub asset: u32,
    pub account_id: String,
    pub symbol: String,
    pub side: HyperliquidPerpOrderSide,
    pub execution: HyperliquidPerpOrderExecution,
    pub time_in_force: HyperliquidPerpOrderTimeInForce,
    pub qty: u64,
    pub filled_qty: u64,
    pub remaining_qty: u64,
    pub status: HyperliquidPerpOrderStatus,
    pub reduce_only: bool,
    pub client_order_id: Option<String>,
    pub version: u64,
}

/// 查询 Hyperliquid perp 开放单列表时的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QueryHyperliquidPerpOpenOrdersError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 合约过滤不能为空字符串。
    #[error("symbol must not be empty")]
    InvalidSymbol,
    /// 订单不属于当前查询账户。
    #[error("order {order_id} does not belong to query party")]
    OrderNotOwned { order_id: String },
    /// 读模型中包含非开放状态的订单。
    #[error("order {order_id} is not open; actual status is {status:?}")]
    OrderNotOpen { order_id: String, status: HyperliquidPerpOrderStatus },
    /// 读模型中包含不符合 symbol 过滤的订单。
    #[error("order {order_id} does not match query filter")]
    OrderFilterMismatch { order_id: String },
    /// 读模型中的订单执行状态不一致。
    #[error("order {order_id} state is inconsistent")]
    InconsistentOrderState { order_id: String },
}

/// 查询账户当前开放中的 Hyperliquid perp 委托单列表。
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryHyperliquidPerpOpenOrdersUseCase;

impl QueryUseCase for QueryHyperliquidPerpOpenOrdersUseCase {
    type Query = QueryHyperliquidPerpOpenOrders;
    type ReadModel = QueryHyperliquidPerpOpenOrdersReadModel;
    type View = Vec<HyperliquidPerpOpenOrderView>;
    type Error = QueryHyperliquidPerpOpenOrdersError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_query(&self, query: &Self::Query) -> Result<(), Self::Error> {
        if query.party_id.is_empty() {
            return Err(QueryHyperliquidPerpOpenOrdersError::InvalidPartyId);
        }
        if query.symbol.as_deref().is_some_and(str::is_empty) {
            return Err(QueryHyperliquidPerpOpenOrdersError::InvalidSymbol);
        }
        Ok(())
    }

    fn validate_against_read_model(
        &self,
        query: &Self::Query,
        read_model: &Self::ReadModel,
    ) -> Result<(), Self::Error> {
        for order in &read_model.orders {
            if !order.belongs_to_account(query.party_id.as_str()) {
                return Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOwned {
                    order_id: order.order_id.clone(),
                });
            }
            if !order.has_consistent_execution_state() {
                return Err(QueryHyperliquidPerpOpenOrdersError::InconsistentOrderState {
                    order_id: order.order_id.clone(),
                });
            }
            if !matches!(
                order.status,
                HyperliquidPerpOrderStatus::Open | HyperliquidPerpOrderStatus::PartiallyFilled
            ) {
                return Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOpen {
                    order_id: order.order_id.clone(),
                    status: order.status,
                });
            }
            if query.symbol.as_deref().is_some_and(|symbol| !order.trades_symbol(symbol)) {
                return Err(QueryHyperliquidPerpOpenOrdersError::OrderFilterMismatch {
                    order_id: order.order_id.clone(),
                });
            }
        }

        Ok(())
    }

    fn compute_view(
        &self,
        _query: &Self::Query,
        read_model: Self::ReadModel,
    ) -> Result<Self::View, Self::Error> {
        read_model
            .orders
            .into_iter()
            .map(|order| {
                let remaining_qty = order.remaining_qty().ok_or(
                    QueryHyperliquidPerpOpenOrdersError::InconsistentOrderState {
                        order_id: order.order_id.clone(),
                    },
                )?;

                Ok(HyperliquidPerpOpenOrderView {
                    order_id: order.order_id,
                    exchange_oid: order.exchange_oid,
                    asset: order.asset,
                    account_id: order.account_id,
                    symbol: order.symbol,
                    side: order.side,
                    execution: order.execution,
                    time_in_force: order.time_in_force,
                    qty: order.qty,
                    filled_qty: order.filled_qty,
                    remaining_qty,
                    status: order.status,
                    reduce_only: order.reduce_only,
                    client_order_id: order.client_order_id,
                    version: order.version,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Reservation, ReservationKind, ReservationMarketKind};

    fn open_order(order_id: &str, symbol: &str) -> HyperliquidPerpOrder {
        let mut order = HyperliquidPerpOrder::new(
            order_id.to_string(),
            Some(42),
            7,
            "trader-1".to_string(),
            symbol.to_string(),
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 100_000 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            5,
            false,
            Some(format!("cloid-{order_id}")),
            Some(
                Reservation::new(
                    format!("reservation:{order_id}"),
                    "trader-1".to_string(),
                    order_id.to_string(),
                    ReservationMarketKind::Perp,
                    ReservationKind::PerpOpenMargin,
                    "USDC".to_string(),
                    1,
                )
                .unwrap(),
            ),
        );
        order.version = 3;
        order
    }

    fn partially_filled_order(order_id: &str, symbol: &str) -> HyperliquidPerpOrder {
        let mut order = HyperliquidPerpOrder::new(
            order_id.to_string(),
            Some(52),
            7,
            "trader-1".to_string(),
            symbol.to_string(),
            HyperliquidPerpOrderSide::Sell,
            HyperliquidPerpOrderExecution::Limit { price: 99_500 },
            HyperliquidPerpOrderTimeInForce::Alo,
            8,
            true,
            Some(format!("cloid-{order_id}")),
            Some(
                Reservation::new(
                    format!("reservation:{order_id}"),
                    "trader-1".to_string(),
                    order_id.to_string(),
                    ReservationMarketKind::Perp,
                    ReservationKind::PerpOpenMargin,
                    "USDC".to_string(),
                    1,
                )
                .unwrap(),
            ),
        )
        .with_execution_state(HyperliquidPerpOrderStatus::PartiallyFilled, 3);
        order.version = 5;
        order
    }

    #[test]
    fn role_returns_trader() {
        assert_eq!(QueryHyperliquidPerpOpenOrdersUseCase.role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_blank_party_id() {
        let result = QueryHyperliquidPerpOpenOrdersUseCase.pre_check_query(
            &QueryHyperliquidPerpOpenOrders { party_id: String::new(), symbol: None },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOpenOrdersError::InvalidPartyId));
    }

    #[test]
    fn pre_check_rejects_blank_symbol() {
        let result = QueryHyperliquidPerpOpenOrdersUseCase.pre_check_query(
            &QueryHyperliquidPerpOpenOrders {
                party_id: "trader-1".to_string(),
                symbol: Some(String::new()),
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOpenOrdersError::InvalidSymbol));
    }

    #[test]
    fn validate_rejects_non_owner() {
        let mut order = open_order("order-1", "BTC-PERP");
        order.account_id = "trader-2".to_string();

        let result = QueryHyperliquidPerpOpenOrdersUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOpenOrders { party_id: "trader-1".to_string(), symbol: None },
            &QueryHyperliquidPerpOpenOrdersReadModel { orders: vec![order] },
        );

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOwned {
                order_id: "order-1".to_string(),
            })
        );
    }

    #[test]
    fn validate_rejects_non_open_status() {
        let order = open_order("order-1", "BTC-PERP")
            .with_execution_state(HyperliquidPerpOrderStatus::Filled, 5);

        let result = QueryHyperliquidPerpOpenOrdersUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOpenOrders { party_id: "trader-1".to_string(), symbol: None },
            &QueryHyperliquidPerpOpenOrdersReadModel { orders: vec![order] },
        );

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOpen {
                order_id: "order-1".to_string(),
                status: HyperliquidPerpOrderStatus::Filled,
            })
        );
    }

    #[test]
    fn validate_rejects_symbol_filter_mismatch() {
        let result = QueryHyperliquidPerpOpenOrdersUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOpenOrders {
                party_id: "trader-1".to_string(),
                symbol: Some("ETH-PERP".to_string()),
            },
            &QueryHyperliquidPerpOpenOrdersReadModel {
                orders: vec![open_order("order-1", "BTC-PERP")],
            },
        );

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpOpenOrdersError::OrderFilterMismatch {
                order_id: "order-1".to_string(),
            })
        );
    }

    #[test]
    fn validate_rejects_inconsistent_order_state() {
        let mut order = open_order("order-1", "BTC-PERP");
        order.filled_qty = 9;

        let result = QueryHyperliquidPerpOpenOrdersUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOpenOrders { party_id: "trader-1".to_string(), symbol: None },
            &QueryHyperliquidPerpOpenOrdersReadModel { orders: vec![order] },
        );

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpOpenOrdersError::InconsistentOrderState {
                order_id: "order-1".to_string(),
            })
        );
    }

    #[test]
    fn compute_view_maps_open_and_partially_filled_orders() {
        let result = QueryHyperliquidPerpOpenOrdersUseCase.compute_view(
            &QueryHyperliquidPerpOpenOrders {
                party_id: "trader-1".to_string(),
                symbol: Some("BTC-PERP".to_string()),
            },
            QueryHyperliquidPerpOpenOrdersReadModel {
                orders: vec![
                    open_order("order-1", "BTC-PERP"),
                    partially_filled_order("order-2", "BTC-PERP"),
                ],
            },
        );

        assert_eq!(
            result,
            Ok(vec![
                HyperliquidPerpOpenOrderView {
                    order_id: "order-1".to_string(),
                    exchange_oid: Some(42),
                    asset: 7,
                    account_id: "trader-1".to_string(),
                    symbol: "BTC-PERP".to_string(),
                    side: HyperliquidPerpOrderSide::Buy,
                    execution: HyperliquidPerpOrderExecution::Limit { price: 100_000 },
                    time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
                    qty: 5,
                    filled_qty: 0,
                    remaining_qty: 5,
                    status: HyperliquidPerpOrderStatus::Open,
                    reduce_only: false,
                    client_order_id: Some("cloid-order-1".to_string()),
                    version: 3,
                },
                HyperliquidPerpOpenOrderView {
                    order_id: "order-2".to_string(),
                    exchange_oid: Some(52),
                    asset: 7,
                    account_id: "trader-1".to_string(),
                    symbol: "BTC-PERP".to_string(),
                    side: HyperliquidPerpOrderSide::Sell,
                    execution: HyperliquidPerpOrderExecution::Limit { price: 99_500 },
                    time_in_force: HyperliquidPerpOrderTimeInForce::Alo,
                    qty: 8,
                    filled_qty: 3,
                    remaining_qty: 5,
                    status: HyperliquidPerpOrderStatus::PartiallyFilled,
                    reduce_only: true,
                    client_order_id: Some("cloid-order-2".to_string()),
                    version: 5,
                },
            ])
        );
    }

    #[test]
    fn compute_view_allows_empty_result() {
        let result = QueryHyperliquidPerpOpenOrdersUseCase.compute_view(
            &QueryHyperliquidPerpOpenOrders { party_id: "trader-1".to_string(), symbol: None },
            QueryHyperliquidPerpOpenOrdersReadModel { orders: Vec::new() },
        );

        assert_eq!(result, Ok(Vec::new()));
    }

    #[test]
    fn compute_view_rejects_remaining_qty_overflow() {
        let mut order = open_order("order-1", "BTC-PERP");
        order.filled_qty = 9;

        let result = QueryHyperliquidPerpOpenOrdersUseCase.compute_view(
            &QueryHyperliquidPerpOpenOrders { party_id: "trader-1".to_string(), symbol: None },
            QueryHyperliquidPerpOpenOrdersReadModel { orders: vec![order] },
        );

        assert_eq!(
            result,
            Err(QueryHyperliquidPerpOpenOrdersError::InconsistentOrderState {
                order_id: "order-1".to_string(),
            })
        );
    }
}
