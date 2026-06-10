use cmd_handler::query_use_case_def2::QueryUseCase;
use cmd_handler::command_use_case_def2::IssuedByParty;
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
    /// 本系统内部稳定订单 ID。
    pub order_id: String,
}

impl IssuedByParty for QueryHyperliquidPerpOrderDetail {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 查询单张 Hyperliquid perp 委托单详情时需要的已加载读模型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryHyperliquidPerpOrderDetailReadModel {
    /// 订单主键是否存在。
    pub order_exists: bool,
    /// 该订单是否属于当前查询账户。
    pub owned_by_party: bool,
    /// 已加载的订单本体；不存在时允许为空。
    pub order: Option<HyperliquidPerpOrder>,
}

/// Hyperliquid perp 委托单详情查询的业务返回视图。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPerpOrderDetailView {
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

impl QueryUseCase for QueryHyperliquidPerpOrderDetailUseCase {
    type Query = QueryHyperliquidPerpOrderDetail;
    type ReadModel = QueryHyperliquidPerpOrderDetailReadModel;
    type View = HyperliquidPerpOrderDetailView;
    type Error = QueryHyperliquidPerpOrderDetailError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_query(&self, query: &Self::Query) -> Result<(), Self::Error> {
        if query.party_id.is_empty() {
            return Err(QueryHyperliquidPerpOrderDetailError::InvalidPartyId);
        }
        if query.order_id.is_empty() {
            return Err(QueryHyperliquidPerpOrderDetailError::InvalidOrderId);
        }
        Ok(())
    }

    fn validate_against_read_model(
        &self,
        query: &Self::Query,
        read_model: &Self::ReadModel,
    ) -> Result<(), Self::Error> {
        if !read_model.order_exists {
            return Err(QueryHyperliquidPerpOrderDetailError::OrderNotFound);
        }
        if !read_model.owned_by_party {
            return Err(QueryHyperliquidPerpOrderDetailError::NotOrderOwner);
        }

        let Some(order) = read_model.order.as_ref() else {
            return Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState);
        };

        if order.order_id != query.order_id
            || !order.belongs_to_account(query.party_id.as_str())
            || !order.has_consistent_execution_state()
        {
            return Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState);
        }

        Ok(())
    }

    fn compute_view(
        &self,
        _query: &Self::Query,
        read_model: Self::ReadModel,
    ) -> Result<Self::View, Self::Error> {
        let Some(order) = read_model.order else {
            return Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState);
        };

        Ok(HyperliquidPerpOrderDetailView {
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
            status: order.status,
            reduce_only: order.reduce_only,
            client_order_id: order.client_order_id,
            version: order.version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_order() -> HyperliquidPerpOrder {
        let mut order = HyperliquidPerpOrder::new(
            "order-1".to_string(),
            Some(42),
            7,
            "trader-1".to_string(),
            "BTC-PERP".to_string(),
            HyperliquidPerpOrderSide::Buy,
            HyperliquidPerpOrderExecution::Limit { price: 100_000 },
            HyperliquidPerpOrderTimeInForce::Gtc,
            5,
            true,
            Some("client-1".to_string()),
        )
        .with_execution_state(HyperliquidPerpOrderStatus::PartiallyFilled, 2);
        order.version = 3;
        order
    }

    #[test]
    fn role_returns_trader() {
        assert_eq!(QueryHyperliquidPerpOrderDetailUseCase.role(), "Trader");
    }

    #[test]
    fn pre_check_rejects_blank_party_id() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.pre_check_query(
            &QueryHyperliquidPerpOrderDetail {
                party_id: String::new(),
                order_id: "order-1".to_string(),
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOrderDetailError::InvalidPartyId));
    }

    #[test]
    fn pre_check_rejects_blank_order_id() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.pre_check_query(
            &QueryHyperliquidPerpOrderDetail {
                party_id: "trader-1".to_string(),
                order_id: String::new(),
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOrderDetailError::InvalidOrderId));
    }

    #[test]
    fn validate_rejects_order_not_found() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOrderDetail {
                party_id: "trader-1".to_string(),
                order_id: "order-1".to_string(),
            },
            &QueryHyperliquidPerpOrderDetailReadModel {
                order_exists: false,
                owned_by_party: false,
                order: None,
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOrderDetailError::OrderNotFound));
    }

    #[test]
    fn validate_rejects_non_owner() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOrderDetail {
                party_id: "trader-1".to_string(),
                order_id: "order-1".to_string(),
            },
            &QueryHyperliquidPerpOrderDetailReadModel {
                order_exists: true,
                owned_by_party: false,
                order: Some(sample_order()),
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOrderDetailError::NotOrderOwner));
    }

    #[test]
    fn validate_rejects_missing_order_when_order_exists() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOrderDetail {
                party_id: "trader-1".to_string(),
                order_id: "order-1".to_string(),
            },
            &QueryHyperliquidPerpOrderDetailReadModel {
                order_exists: true,
                owned_by_party: true,
                order: None,
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState));
    }

    #[test]
    fn validate_rejects_inconsistent_order_state() {
        let mut order = sample_order();
        order.status = HyperliquidPerpOrderStatus::Filled;
        order.filled_qty = 1;

        let result = QueryHyperliquidPerpOrderDetailUseCase.validate_against_read_model(
            &QueryHyperliquidPerpOrderDetail {
                party_id: "trader-1".to_string(),
                order_id: "order-1".to_string(),
            },
            &QueryHyperliquidPerpOrderDetailReadModel {
                order_exists: true,
                owned_by_party: true,
                order: Some(order),
            },
        );

        assert_eq!(result, Err(QueryHyperliquidPerpOrderDetailError::InconsistentOrderState));
    }

    #[test]
    fn compute_view_returns_order_body_fields() {
        let result = QueryHyperliquidPerpOrderDetailUseCase.compute_view(
            &QueryHyperliquidPerpOrderDetail {
                party_id: "trader-1".to_string(),
                order_id: "order-1".to_string(),
            },
            QueryHyperliquidPerpOrderDetailReadModel {
                order_exists: true,
                owned_by_party: true,
                order: Some(sample_order()),
            },
        );

        assert_eq!(
            result,
            Ok(HyperliquidPerpOrderDetailView {
                order_id: "order-1".to_string(),
                exchange_oid: Some(42),
                asset: 7,
                account_id: "trader-1".to_string(),
                symbol: "BTC-PERP".to_string(),
                side: HyperliquidPerpOrderSide::Buy,
                execution: HyperliquidPerpOrderExecution::Limit { price: 100_000 },
                time_in_force: HyperliquidPerpOrderTimeInForce::Gtc,
                qty: 5,
                filled_qty: 2,
                status: HyperliquidPerpOrderStatus::PartiallyFilled,
                reduce_only: true,
                client_order_id: Some("client-1".to_string()),
                version: 3,
            })
        );
    }
}
