use common_entity::{DomainReadModel, QueryUseCase};
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

/// 查询 Hyperliquid perp 开放单列表时需要的已加载读模型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpOpenOrdersReadModel {
    /// 已加载的开放单候选集合。
    pub orders: Vec<HyperliquidPerpOrder>,
}

impl DomainReadModel for QueryHyperliquidPerpOpenOrdersReadModel {}

/// 单条 Hyperliquid perp 开放委托单的业务视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpOpenOrderView {
    pub order_id: u64,
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
    OrderNotOwned { order_id: u64 },
    /// 读模型中包含非开放状态的订单。
    #[error("order {order_id} is not open; actual status is {status:?}")]
    OrderNotOpen { order_id: u64, status: HyperliquidPerpOrderStatus },
    /// 读模型中包含不符合 symbol 过滤的订单。
    #[error("order {order_id} does not match query filter")]
    OrderFilterMismatch { order_id: u64 },
    /// 读模型中的订单执行状态不一致。
    #[error("order {order_id} state is inconsistent")]
    InconsistentOrderState { order_id: u64 },
}

/// 查询账户当前开放中的 Hyperliquid perp 委托单列表。
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryHyperliquidPerpOpenOrdersUseCase;

impl QueryUseCase<QueryHyperliquidPerpOpenOrders, QueryHyperliquidPerpOpenOrdersReadModel>
    for QueryHyperliquidPerpOpenOrdersUseCase
{
    type Output = Vec<HyperliquidPerpOpenOrderView>;
    type Error = QueryHyperliquidPerpOpenOrdersError;

    fn validate_query(&self, query: &QueryHyperliquidPerpOpenOrders) -> Result<(), Self::Error> {
        if query.party_id.trim().is_empty() {
            return Err(QueryHyperliquidPerpOpenOrdersError::InvalidPartyId);
        }
        if query.symbol.as_deref().is_some_and(|symbol| symbol.trim().is_empty()) {
            return Err(QueryHyperliquidPerpOpenOrdersError::InvalidSymbol);
        }
        Ok(())
    }

    fn compute_query(
        &self,
        query: &QueryHyperliquidPerpOpenOrders,
        read_model: QueryHyperliquidPerpOpenOrdersReadModel,
    ) -> Result<Self::Output, Self::Error> {
        for order in &read_model.orders {
            if !order.belongs_to_account(query.party_id.as_str()) {
                return Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOwned {
                    order_id: order.order_id,
                });
            }
            if !order.has_consistent_execution_state() {
                return Err(QueryHyperliquidPerpOpenOrdersError::InconsistentOrderState {
                    order_id: order.order_id,
                });
            }
            if !matches!(
                order.status,
                HyperliquidPerpOrderStatus::Open | HyperliquidPerpOrderStatus::PartiallyFilled
            ) {
                return Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOpen {
                    order_id: order.order_id,
                    status: order.status,
                });
            }
            if query.symbol.as_deref().is_some_and(|symbol| !order.trades_symbol(symbol)) {
                return Err(QueryHyperliquidPerpOpenOrdersError::OrderFilterMismatch {
                    order_id: order.order_id,
                });
            }
        }

        read_model
            .orders
            .into_iter()
            .map(|order| {
                let remaining_qty = order.remaining_qty().ok_or(
                    QueryHyperliquidPerpOpenOrdersError::InconsistentOrderState {
                        order_id: order.order_id,
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
                    qty: order.sz,
                    filled_qty: order.filled_sz,
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
mod compute_query_tests {
    use super::*;

    fn query(symbol: Option<&str>) -> QueryHyperliquidPerpOpenOrders {
        QueryHyperliquidPerpOpenOrders {
            party_id: "trader-1".to_string(),
            symbol: symbol.map(str::to_string),
        }
    }

    fn order(
        account_id: &str,
        symbol: &str,
        status: HyperliquidPerpOrderStatus,
    ) -> HyperliquidPerpOrder {
        HyperliquidPerpOrder::new(
            1,
            Some(42),
            7,
            account_id.to_string(),
            symbol.to_string(),
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 100_000 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            5,
            false,
            None,
        )
        .with_execution_state(status, 0)
    }

    #[test]
    fn validate_query_rejects_empty_or_whitespace_party_and_symbol() {
        for party_id in ["", "   ", "\t\n"] {
            assert_eq!(
                QueryHyperliquidPerpOpenOrdersUseCase.validate_query(
                    &QueryHyperliquidPerpOpenOrders {
                        party_id: party_id.to_string(),
                        symbol: None,
                    },
                ),
                Err(QueryHyperliquidPerpOpenOrdersError::InvalidPartyId)
            );
        }
        for symbol in ["", "   ", "\t\n"] {
            assert_eq!(
                QueryHyperliquidPerpOpenOrdersUseCase.validate_query(
                    &QueryHyperliquidPerpOpenOrders {
                        party_id: "trader-1".to_string(),
                        symbol: Some(symbol.to_string()),
                    },
                ),
                Err(QueryHyperliquidPerpOpenOrdersError::InvalidSymbol)
            );
        }
        assert_eq!(QueryHyperliquidPerpOpenOrdersUseCase.validate_query(&query(None)), Ok(()));
        assert_eq!(
            QueryHyperliquidPerpOpenOrdersUseCase.validate_query(&query(Some("BTC-PERP"))),
            Ok(())
        );
    }

    #[test]
    fn ownership_open_status_and_filter_are_decided_by_the_use_case() {
        let non_owner = QueryHyperliquidPerpOpenOrdersReadModel {
            orders: vec![order("trader-2", "BTC-PERP", HyperliquidPerpOrderStatus::Open)],
        };
        assert_eq!(
            QueryHyperliquidPerpOpenOrdersUseCase.compute_query(&query(None), non_owner),
            Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOwned { order_id: 1 })
        );

        let closed = QueryHyperliquidPerpOpenOrdersReadModel {
            orders: vec![order("trader-1", "BTC-PERP", HyperliquidPerpOrderStatus::Canceled)],
        };
        assert!(matches!(
            QueryHyperliquidPerpOpenOrdersUseCase.compute_query(&query(None), closed),
            Err(QueryHyperliquidPerpOpenOrdersError::OrderNotOpen { .. })
        ));

        let mismatch = QueryHyperliquidPerpOpenOrdersReadModel {
            orders: vec![order("trader-1", "BTC-PERP", HyperliquidPerpOrderStatus::Open)],
        };
        assert_eq!(
            QueryHyperliquidPerpOpenOrdersUseCase.compute_query(&query(Some("ETH-PERP")), mismatch),
            Err(QueryHyperliquidPerpOpenOrdersError::OrderFilterMismatch { order_id: 1 })
        );
    }

    #[test]
    fn accepted_orders_are_mapped_to_views() {
        let read_model = QueryHyperliquidPerpOpenOrdersReadModel {
            orders: vec![order("trader-1", "BTC-PERP", HyperliquidPerpOrderStatus::Open)],
        };

        let result = QueryHyperliquidPerpOpenOrdersUseCase.compute_query(&query(None), read_model);

        assert_eq!(result.map(|views| views.len()), Ok(1));
    }
}
