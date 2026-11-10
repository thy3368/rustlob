use common_entity::{DomainReadModel, QueryUseCase};
use thiserror::Error;

use crate::{
    HyperliquidPerpOrder, HyperliquidPerpOrderExecution, HyperliquidPerpOrderSide,
    HyperliquidPerpOrderStatus, HyperliquidPerpOrderTimeInForce,
};

/// 查询单张 Hyperliquid perp 委托单详情的输入。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpOrderDetail {
    /// 发起查询的业务账户 ID。
    pub party_id: String,
    /// Hyperliquid perp `oid`。
    pub order_id: u64,
}

/// 查询单张 Hyperliquid perp 委托单详情时需要的已加载读模型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryHyperliquidPerpOrderDetailReadModel {
    NotFound,
    Found(HyperliquidPerpOrder),
}

impl DomainReadModel for QueryHyperliquidPerpOrderDetailReadModel {}

/// Hyperliquid perp 委托单详情查询的业务返回视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpOrderDetailView {
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
    pub status: HyperliquidPerpOrderStatus,
    pub reduce_only: bool,
    pub client_order_id: Option<String>,
    pub version: u64,
}

/// 查询 Hyperliquid perp 委托单详情时的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QueryHyperliquidPerpOrderDetailError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 订单 ID 不能为空。
    #[error("order_id must not be empty")]
    InvalidOrderId,
    /// 订单不存在。
    #[error("order was not found")]
    OrderNotFound,
    /// 订单不属于当前查询账户。
    #[error("order does not belong to query party")]
    NotOrderOwner,
    /// 读模型和订单执行状态不一致。
    #[error("order state is inconsistent")]
    InconsistentOrderState,
}

/// 查询单张 Hyperliquid perp 委托单本体详情。
#[derive(Debug, Clone, Copy, Default)]
pub struct QueryHyperliquidPerpOrderDetailUseCase;

impl QueryUseCase<QueryHyperliquidPerpOrderDetail, QueryHyperliquidPerpOrderDetailReadModel>
    for QueryHyperliquidPerpOrderDetailUseCase
{
    type Output = HyperliquidPerpOrderDetailView;
    type Error = QueryHyperliquidPerpOrderDetailError;

    fn validate_query(&self, query: &QueryHyperliquidPerpOrderDetail) -> Result<(), Self::Error> {
        if query.party_id.trim().is_empty() {
            return Err(QueryHyperliquidPerpOrderDetailError::InvalidPartyId);
        }
        if query.order_id == 0 {
            return Err(QueryHyperliquidPerpOrderDetailError::InvalidOrderId);
        }
        Ok(())
    }

    fn compute_query(
        &self,
        query: &QueryHyperliquidPerpOrderDetail,
        read_model: QueryHyperliquidPerpOrderDetailReadModel,
    ) -> Result<Self::Output, Self::Error> {
        let QueryHyperliquidPerpOrderDetailReadModel::Found(order) = read_model else {
            return Err(QueryHyperliquidPerpOrderDetailError::OrderNotFound);
        };

        if !order.belongs_to_account(query.party_id.as_str()) {
            return Err(QueryHyperliquidPerpOrderDetailError::NotOrderOwner);
        }
        if order.order_id != query.order_id || !order.has_consistent_execution_state() {
            return Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState);
        }

        Ok(HyperliquidPerpOrderDetailView {
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
            status: order.status,
            reduce_only: order.reduce_only,
            client_order_id: order.client_order_id,
            version: order.version,
        })
    }
}

#[cfg(test)]
mod compute_query_tests {
    use std::convert::Infallible;

    use common_entity::{QueryExecutionError, QueryExecutor, ReadModelSource};

    use super::*;

    struct NotFoundSource;

    impl ReadModelSource<QueryHyperliquidPerpOrderDetail> for NotFoundSource {
        type ReadModel = QueryHyperliquidPerpOrderDetailReadModel;
        type Error = Infallible;

        fn load_read_model(
            &self,
            _query: &QueryHyperliquidPerpOrderDetail,
        ) -> Result<Self::ReadModel, Self::Error> {
            Ok(QueryHyperliquidPerpOrderDetailReadModel::NotFound)
        }
    }

    fn query() -> QueryHyperliquidPerpOrderDetail {
        QueryHyperliquidPerpOrderDetail { party_id: "trader-1".to_string(), order_id: 1 }
    }

    fn sample_order() -> HyperliquidPerpOrder {
        HyperliquidPerpOrder::new(
            1,
            Some(42),
            7,
            "trader-1".to_string(),
            "BTC-PERP".to_string(),
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 100_000 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            5,
            true,
            None,
        )
    }

    #[test]
    fn validate_query_rejects_empty_or_whitespace_party_and_zero_order_id() {
        for party_id in ["", "   ", "\t\n"] {
            assert_eq!(
                QueryHyperliquidPerpOrderDetailUseCase.validate_query(
                    &QueryHyperliquidPerpOrderDetail {
                        party_id: party_id.to_string(),
                        order_id: 1
                    },
                ),
                Err(QueryHyperliquidPerpOrderDetailError::InvalidPartyId)
            );
        }
        assert_eq!(
            QueryHyperliquidPerpOrderDetailUseCase.validate_query(
                &QueryHyperliquidPerpOrderDetail { party_id: "trader-1".to_string(), order_id: 0 },
            ),
            Err(QueryHyperliquidPerpOrderDetailError::InvalidOrderId)
        );
        assert_eq!(QueryHyperliquidPerpOrderDetailUseCase.validate_query(&query()), Ok(()));
    }

    #[test]
    fn not_found_is_a_business_rejection() {
        assert_eq!(
            QueryExecutor.execute(
                &query(),
                &NotFoundSource,
                &QueryHyperliquidPerpOrderDetailUseCase,
            ),
            Err(QueryExecutionError::Business(QueryHyperliquidPerpOrderDetailError::OrderNotFound))
        );
    }

    #[test]
    fn ownership_is_decided_by_the_use_case() {
        let mut order = sample_order();
        order.account_id = "trader-2".to_string();

        assert_eq!(
            QueryHyperliquidPerpOrderDetailUseCase
                .compute_query(&query(), QueryHyperliquidPerpOrderDetailReadModel::Found(order),),
            Err(QueryHyperliquidPerpOrderDetailError::NotOrderOwner)
        );
    }

    #[test]
    fn mismatched_order_id_is_rejected() {
        let mut order = sample_order();
        order.order_id = 2;

        assert_eq!(
            QueryHyperliquidPerpOrderDetailUseCase
                .compute_query(&query(), QueryHyperliquidPerpOrderDetailReadModel::Found(order),),
            Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState)
        );
    }

    #[test]
    fn found_order_is_mapped_to_view() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.compute_query(
            &query(),
            QueryHyperliquidPerpOrderDetailReadModel::Found(sample_order()),
        );

        assert_eq!(result.map(|view| view.order_id), Ok(1));
    }
}
