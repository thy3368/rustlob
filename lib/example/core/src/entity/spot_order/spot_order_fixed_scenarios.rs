use common_entity::Entity;

use super::*;

/// active `SpotOrder` 的固定业务场景。
///
/// 这个文件不用 proptest，目的是把业务矩阵清楚列出来，方便 LLM 和人直接查看支持面。
#[derive(Debug, Clone)]
enum ActiveSpotOrderScenario {
    /// 买入限价单，尚未成交。
    BuyLimitOpen,
    /// 买入限价单，部分成交。
    BuyLimitPartiallyFilled,
    /// 买入限价单，完全成交。
    BuyLimitFilled,
    /// 买入限价单，已取消。
    BuyLimitCanceled,
    /// 卖出限价单，尚未成交。
    SellLimitOpen,
    /// 卖出限价单，部分成交。
    SellLimitPartiallyFilled,
    /// 卖出限价单，完全成交。
    SellLimitFilled,
    /// 卖出限价单，已取消。
    SellLimitCanceled,
    /// 买入市价意图，使用激进价格进入执行流程。
    BuyMarketOpen,
    /// 卖出市价意图，使用激进价格进入执行流程。
    SellMarketOpen,
}

impl ActiveSpotOrderScenario {
    const ALL: [Self; 10] = [
        Self::BuyLimitOpen,
        Self::BuyLimitPartiallyFilled,
        Self::BuyLimitFilled,
        Self::BuyLimitCanceled,
        Self::SellLimitOpen,
        Self::SellLimitPartiallyFilled,
        Self::SellLimitFilled,
        Self::SellLimitCanceled,
        Self::BuyMarketOpen,
        Self::SellMarketOpen,
    ];

    fn order(&self) -> SpotOrder {
        SpotOrder::new(
            "order-42".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            self.side(),
            self.execution(),
            self.time_in_force(),
            2,
            self.reserved_base(),
            self.reserved_quote(),
            Some("0123456789abcdef0123456789abcdef".to_string()),
        )
        .with_execution_state(self.status(), self.filled_qty())
    }

    const fn side(&self) -> SpotOrderSide {
        match self {
            Self::BuyLimitOpen
            | Self::BuyLimitPartiallyFilled
            | Self::BuyLimitFilled
            | Self::BuyLimitCanceled
            | Self::BuyMarketOpen => SpotOrderSide::Buy,
            Self::SellLimitOpen
            | Self::SellLimitPartiallyFilled
            | Self::SellLimitFilled
            | Self::SellLimitCanceled
            | Self::SellMarketOpen => SpotOrderSide::Sell,
        }
    }

    const fn execution(&self) -> SpotOrderExecution {
        match self {
            Self::BuyMarketOpen | Self::SellMarketOpen => {
                SpotOrderExecution::Market { aggressive_price: 110 }
            }
            Self::BuyLimitOpen
            | Self::BuyLimitPartiallyFilled
            | Self::BuyLimitFilled
            | Self::BuyLimitCanceled
            | Self::SellLimitOpen
            | Self::SellLimitPartiallyFilled
            | Self::SellLimitFilled
            | Self::SellLimitCanceled => SpotOrderExecution::Limit { price: 100 },
        }
    }

    const fn time_in_force(&self) -> SpotOrderTimeInForce {
        match self.execution() {
            SpotOrderExecution::Market { .. } => SpotOrderTimeInForce::Ioc,
            SpotOrderExecution::Limit { .. } => SpotOrderTimeInForce::Gtc,
        }
    }

    const fn status(&self) -> SpotOrderStatus {
        match self {
            Self::BuyLimitOpen
            | Self::SellLimitOpen
            | Self::BuyMarketOpen
            | Self::SellMarketOpen => SpotOrderStatus::Open,
            Self::BuyLimitPartiallyFilled | Self::SellLimitPartiallyFilled => {
                SpotOrderStatus::PartiallyFilled
            }
            Self::BuyLimitFilled | Self::SellLimitFilled => SpotOrderStatus::Filled,
            Self::BuyLimitCanceled | Self::SellLimitCanceled => SpotOrderStatus::Canceled,
        }
    }

    const fn filled_qty(&self) -> u64 {
        match self {
            Self::BuyLimitPartiallyFilled
            | Self::SellLimitPartiallyFilled
            | Self::BuyLimitCanceled
            | Self::SellLimitCanceled => 1,
            Self::BuyLimitFilled | Self::SellLimitFilled => 2,
            Self::BuyLimitOpen
            | Self::SellLimitOpen
            | Self::BuyMarketOpen
            | Self::SellMarketOpen => 0,
        }
    }

    const fn reserved_base(&self) -> u64 {
        match self.side() {
            SpotOrderSide::Buy => 0,
            SpotOrderSide::Sell => 2,
        }
    }

    const fn reserved_quote(&self) -> u64 {
        match self {
            Self::BuyMarketOpen => 220,
            Self::BuyLimitOpen
            | Self::BuyLimitPartiallyFilled
            | Self::BuyLimitFilled
            | Self::BuyLimitCanceled => 200,
            Self::SellLimitOpen
            | Self::SellLimitPartiallyFilled
            | Self::SellLimitFilled
            | Self::SellLimitCanceled
            | Self::SellMarketOpen => 0,
        }
    }
}

/// 未触发 `SpotConditionalOrder` 的固定业务场景。
#[derive(Debug, Clone)]
enum ConditionalSpotOrderScenario {
    /// 买入止损市价条件单，等待触发。
    BuyStopLossMarketOpen,
    /// 买入止盈限价条件单，等待触发。
    BuyTakeProfitLimitOpen,
    /// 卖出止损市价条件单，等待触发。
    SellStopLossMarketOpen,
    /// 卖出止盈限价条件单，等待触发。
    SellTakeProfitLimitOpen,
    /// 已触发，条件单本身不再可撤。
    Triggered,
    /// 已取消，不能重复撤销。
    Canceled,
    /// 已拒绝，不能撤销。
    Rejected,
    /// 已过期，不能撤销。
    Expired,
}

impl ConditionalSpotOrderScenario {
    const ALL: [Self; 8] = [
        Self::BuyStopLossMarketOpen,
        Self::BuyTakeProfitLimitOpen,
        Self::SellStopLossMarketOpen,
        Self::SellTakeProfitLimitOpen,
        Self::Triggered,
        Self::Canceled,
        Self::Rejected,
        Self::Expired,
    ];

    fn order(&self) -> SpotConditionalOrder {
        SpotConditionalOrder::new(
            "trigger-42".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            self.side(),
            90,
            self.trigger_role(),
            self.execution(),
            self.time_in_force(),
            2,
            Some("0123456789abcdef0123456789abcdef".to_string()),
        )
        .with_status(self.status())
    }

    const fn side(&self) -> SpotOrderSide {
        match self {
            Self::BuyStopLossMarketOpen | Self::BuyTakeProfitLimitOpen => SpotOrderSide::Buy,
            Self::SellStopLossMarketOpen
            | Self::SellTakeProfitLimitOpen
            | Self::Triggered
            | Self::Canceled
            | Self::Rejected
            | Self::Expired => SpotOrderSide::Sell,
        }
    }

    const fn trigger_role(&self) -> SpotOrderTriggerRole {
        match self {
            Self::BuyStopLossMarketOpen | Self::SellStopLossMarketOpen => {
                SpotOrderTriggerRole::StopLoss
            }
            Self::BuyTakeProfitLimitOpen
            | Self::SellTakeProfitLimitOpen
            | Self::Triggered
            | Self::Canceled
            | Self::Rejected
            | Self::Expired => SpotOrderTriggerRole::TakeProfit,
        }
    }

    const fn execution(&self) -> SpotOrderExecution {
        match self {
            Self::BuyStopLossMarketOpen | Self::SellStopLossMarketOpen => {
                SpotOrderExecution::Market { aggressive_price: 95 }
            }
            Self::BuyTakeProfitLimitOpen
            | Self::SellTakeProfitLimitOpen
            | Self::Triggered
            | Self::Canceled
            | Self::Rejected
            | Self::Expired => SpotOrderExecution::Limit { price: 100 },
        }
    }

    const fn time_in_force(&self) -> SpotOrderTimeInForce {
        match self.execution() {
            SpotOrderExecution::Market { .. } => SpotOrderTimeInForce::Ioc,
            SpotOrderExecution::Limit { .. } => SpotOrderTimeInForce::Gtc,
        }
    }

    const fn status(&self) -> SpotConditionalOrderStatus {
        match self {
            Self::BuyStopLossMarketOpen
            | Self::BuyTakeProfitLimitOpen
            | Self::SellStopLossMarketOpen
            | Self::SellTakeProfitLimitOpen => SpotConditionalOrderStatus::Open,
            Self::Triggered => SpotConditionalOrderStatus::Triggered,
            Self::Canceled => SpotConditionalOrderStatus::Canceled,
            Self::Rejected => SpotConditionalOrderStatus::Rejected,
            Self::Expired => SpotConditionalOrderStatus::Expired,
        }
    }
}

fn created_value<E>(entity: &E, field_name: &str) -> Option<String>
where
    E: Entity,
{
    entity
        .created_field_changes()
        .into_iter()
        .find(|change| change.field_name == field_name)
        .map(|change| change.new_value)
}

fn print_active_order_detail(scenario: &ActiveSpotOrderScenario, order: &SpotOrder) {
    println!(
        "active scenario={scenario:?}, order_id={}, asset={}, exchange_oid={:?}, account_id={}, symbol={}, side={:?}, execution={:?}, tif={:?}, qty={}, filled_qty={}, status={:?}, reserved_base={}, reserved_quote={}, can_cancel={}, client_order_id={:?}",
        order.order_id,
        order.asset,
        order.exchange_oid,
        order.account_id,
        order.symbol,
        order.side,
        order.execution,
        order.time_in_force,
        order.qty,
        order.filled_qty,
        order.status,
        order.reserved_base,
        order.reserved_quote,
        order.can_be_cancelled(),
        order.client_order_id,
    );
}

fn print_conditional_order_detail(
    scenario: &ConditionalSpotOrderScenario,
    order: &SpotConditionalOrder,
) {
    println!(
        "conditional scenario={scenario:?}, trigger_order_id={}, asset={}, exchange_oid={:?}, account_id={}, symbol={}, side={:?}, trigger_price={}, trigger_role={:?}, execution={:?}, tif={:?}, qty={}, status={:?}, can_cancel={}, client_order_id={:?}",
        order.trigger_order_id,
        order.asset,
        order.exchange_oid,
        order.account_id,
        order.symbol,
        order.side,
        order.trigger_price,
        order.trigger_role,
        order.execution,
        order.time_in_force,
        order.qty,
        order.status,
        order.can_be_cancelled(),
        order.client_order_id,
    );
}

#[test]
fn active_spot_order_fixed_scenarios_cover_business_matrix() {
    assert_eq!(ActiveSpotOrderScenario::ALL.len(), 10);

    for scenario in ActiveSpotOrderScenario::ALL {
        let order = scenario.order();
        print_active_order_detail(&scenario, &order);

        assert_eq!(order.side, scenario.side());
        assert_eq!(order.status, scenario.status());
        assert_eq!(order.filled_qty, scenario.filled_qty());
        assert_eq!(order.can_be_cancelled(), order.status.is_cancelable());
        assert!(order.has_consistent_execution_state());
        assert!(order.has_consistent_reserved_base());
        assert!(order.has_consistent_reserved_quote());
        assert_eq!(order.base_to_release_on_cancel(), scenario.reserved_base());
        assert_eq!(order.quote_to_release_on_cancel(), scenario.reserved_quote());
        assert_eq!(created_value(&order, "asset"), Some("10001".to_string()));
    }
}

#[test]
fn conditional_spot_order_fixed_scenarios_cover_business_matrix() {
    assert_eq!(ConditionalSpotOrderScenario::ALL.len(), 8);

    for scenario in ConditionalSpotOrderScenario::ALL {
        let order = scenario.order();
        print_conditional_order_detail(&scenario, &order);

        assert_eq!(order.side, scenario.side());
        assert_eq!(order.status, scenario.status());
        assert_eq!(order.can_be_cancelled(), order.status == SpotConditionalOrderStatus::Open);
        assert_eq!(created_value(&order, "trigger_price"), Some("90".to_string()));
        assert_eq!(
            created_value(&order, "trigger_role"),
            Some(order.trigger_role.as_str().to_string())
        );
    }
}

#[test]
fn open_conditional_fixed_scenarios_trigger_active_spot_order() {
    for scenario in [
        ConditionalSpotOrderScenario::BuyStopLossMarketOpen,
        ConditionalSpotOrderScenario::BuyTakeProfitLimitOpen,
        ConditionalSpotOrderScenario::SellStopLossMarketOpen,
        ConditionalSpotOrderScenario::SellTakeProfitLimitOpen,
    ] {
        let conditional = scenario.order();
        let active = conditional.triggered_order("active-42".to_string(), 2, 0);

        assert_eq!(active.order_id, "active-42");
        assert_eq!(active.asset, conditional.asset);
        assert_eq!(active.account_id, conditional.account_id);
        assert_eq!(active.side, conditional.side);
        assert_eq!(active.execution, conditional.execution);
        assert_eq!(active.time_in_force, conditional.time_in_force);
        assert_eq!(active.qty, conditional.qty);
    }
}
