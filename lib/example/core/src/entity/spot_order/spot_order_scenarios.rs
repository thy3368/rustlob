use common_entity::Entity;
use proptest::prelude::*;

use super::*;

/// active `SpotOrder` 的核心业务矩阵。
#[derive(Debug, Clone)]
pub(crate) enum ActiveSpotOrderScenario {
    /// 买入限价单，尚未成交。
    BuyLimitOpen { qty: u64, price: u64 },
    /// 买入限价单，部分成交。
    BuyLimitPartiallyFilled { qty: u64, price: u64, filled_qty: u64 },
    /// 买入限价单，完全成交。
    BuyLimitFilled { qty: u64, price: u64 },
    /// 买入限价单，已取消。
    BuyLimitCanceled { qty: u64, price: u64, filled_qty: u64 },
    /// 卖出限价单，尚未成交。
    SellLimitOpen { qty: u64, price: u64 },
    /// 卖出限价单，部分成交。
    SellLimitPartiallyFilled { qty: u64, price: u64, filled_qty: u64 },
    /// 卖出限价单，完全成交。
    SellLimitFilled { qty: u64, price: u64 },
    /// 卖出限价单，已取消。
    SellLimitCanceled { qty: u64, price: u64, filled_qty: u64 },
    /// 买入市价意图，使用激进价格进入执行流。
    BuyMarketOpen { qty: u64, aggressive_price: u64 },
    /// 卖出市价意图，使用激进价格进入执行流。
    SellMarketOpen { qty: u64, aggressive_price: u64 },
}

impl ActiveSpotOrderScenario {
    pub(crate) fn order(&self) -> SpotOrder {
        let order = SpotOrder::new(
            "order-42".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            self.side(),
            self.execution(),
            self.time_in_force(),
            self.qty(),
            self.reserved_base(),
            self.reserved_quote(),
            Some("0123456789abcdef0123456789abcdef".to_string()),
        );

        order.with_execution_state(self.status(), self.filled_qty())
    }

    pub(crate) fn side(&self) -> SpotOrderSide {
        match self {
            Self::BuyLimitOpen { .. }
            | Self::BuyLimitPartiallyFilled { .. }
            | Self::BuyLimitFilled { .. }
            | Self::BuyLimitCanceled { .. }
            | Self::BuyMarketOpen { .. } => SpotOrderSide::Buy,
            Self::SellLimitOpen { .. }
            | Self::SellLimitPartiallyFilled { .. }
            | Self::SellLimitFilled { .. }
            | Self::SellLimitCanceled { .. }
            | Self::SellMarketOpen { .. } => SpotOrderSide::Sell,
        }
    }

    fn execution(&self) -> SpotOrderExecution {
        match self {
            Self::BuyLimitOpen { price, .. }
            | Self::BuyLimitPartiallyFilled { price, .. }
            | Self::BuyLimitFilled { price, .. }
            | Self::BuyLimitCanceled { price, .. }
            | Self::SellLimitOpen { price, .. }
            | Self::SellLimitPartiallyFilled { price, .. }
            | Self::SellLimitFilled { price, .. }
            | Self::SellLimitCanceled { price, .. } => SpotOrderExecution::Limit { price: *price },
            Self::BuyMarketOpen { aggressive_price, .. }
            | Self::SellMarketOpen { aggressive_price, .. } => {
                SpotOrderExecution::Market { aggressive_price: *aggressive_price }
            }
        }
    }

    fn time_in_force(&self) -> SpotOrderTimeInForce {
        match self.execution() {
            SpotOrderExecution::Market { .. } => SpotOrderTimeInForce::Ioc,
            SpotOrderExecution::Limit { .. } => SpotOrderTimeInForce::Gtc,
        }
    }

    pub(crate) fn status(&self) -> SpotOrderStatus {
        match self {
            Self::BuyLimitOpen { .. }
            | Self::SellLimitOpen { .. }
            | Self::BuyMarketOpen { .. }
            | Self::SellMarketOpen { .. } => SpotOrderStatus::Open,
            Self::BuyLimitPartiallyFilled { .. } | Self::SellLimitPartiallyFilled { .. } => {
                SpotOrderStatus::PartiallyFilled
            }
            Self::BuyLimitFilled { .. } | Self::SellLimitFilled { .. } => SpotOrderStatus::Filled,
            Self::BuyLimitCanceled { .. } | Self::SellLimitCanceled { .. } => {
                SpotOrderStatus::Canceled
            }
        }
    }

    fn qty(&self) -> u64 {
        match self {
            Self::BuyLimitOpen { qty, .. }
            | Self::BuyLimitPartiallyFilled { qty, .. }
            | Self::BuyLimitFilled { qty, .. }
            | Self::BuyLimitCanceled { qty, .. }
            | Self::SellLimitOpen { qty, .. }
            | Self::SellLimitPartiallyFilled { qty, .. }
            | Self::SellLimitFilled { qty, .. }
            | Self::SellLimitCanceled { qty, .. }
            | Self::BuyMarketOpen { qty, .. }
            | Self::SellMarketOpen { qty, .. } => *qty,
        }
    }

    fn price_for_reserve(&self) -> u64 {
        self.execution().order_price()
    }

    pub(crate) fn filled_qty(&self) -> u64 {
        match self {
            Self::BuyLimitPartiallyFilled { filled_qty, .. }
            | Self::BuyLimitCanceled { filled_qty, .. }
            | Self::SellLimitPartiallyFilled { filled_qty, .. }
            | Self::SellLimitCanceled { filled_qty, .. } => *filled_qty,
            Self::BuyLimitFilled { qty, .. } | Self::SellLimitFilled { qty, .. } => *qty,
            Self::BuyLimitOpen { .. }
            | Self::SellLimitOpen { .. }
            | Self::BuyMarketOpen { .. }
            | Self::SellMarketOpen { .. } => 0,
        }
    }

    pub(crate) fn reserved_base(&self) -> u64 {
        match self.side() {
            SpotOrderSide::Buy => 0,
            SpotOrderSide::Sell => self.qty(),
        }
    }

    pub(crate) fn reserved_quote(&self) -> u64 {
        match self.side() {
            SpotOrderSide::Buy => self.qty() * self.price_for_reserve(),
            SpotOrderSide::Sell => 0,
        }
    }
}

/// 未触发条件单的业务矩阵。
#[derive(Debug, Clone)]
enum ConditionalSpotOrderScenario {
    /// 买入止损市价条件单，等待触发。
    BuyStopLossMarketOpen { qty: u64, trigger_price: u64, aggressive_price: u64 },
    /// 买入止盈限价条件单，等待触发。
    BuyTakeProfitLimitOpen { qty: u64, trigger_price: u64, price: u64 },
    /// 卖出止损市价条件单，等待触发。
    SellStopLossMarketOpen { qty: u64, trigger_price: u64, aggressive_price: u64 },
    /// 卖出止盈限价条件单，等待触发。
    SellTakeProfitLimitOpen { qty: u64, trigger_price: u64, price: u64 },
    /// 已触发的条件单，不能再按条件单撤销。
    Triggered,
    /// 已取消的条件单，不能重复撤销。
    Canceled,
    /// 被拒绝的条件单，不能撤销。
    Rejected,
    /// 过期的条件单，不能撤销。
    Expired,
}

impl ConditionalSpotOrderScenario {
    fn order(&self) -> SpotConditionalOrder {
        SpotConditionalOrder::new(
            "trigger-42".to_string(),
            10_001,
            Some(42),
            "trader-1".to_string(),
            "BTCUSDT".to_string(),
            self.side(),
            self.trigger_price(),
            self.trigger_role(),
            self.execution(),
            self.time_in_force(),
            self.qty(),
            Some("0123456789abcdef0123456789abcdef".to_string()),
        )
        .with_status(self.status())
    }

    fn side(&self) -> SpotOrderSide {
        match self {
            Self::BuyStopLossMarketOpen { .. } | Self::BuyTakeProfitLimitOpen { .. } => {
                SpotOrderSide::Buy
            }
            Self::SellStopLossMarketOpen { .. }
            | Self::SellTakeProfitLimitOpen { .. }
            | Self::Triggered
            | Self::Canceled
            | Self::Rejected
            | Self::Expired => SpotOrderSide::Sell,
        }
    }

    fn trigger_role(&self) -> SpotOrderTriggerRole {
        match self {
            Self::BuyStopLossMarketOpen { .. } | Self::SellStopLossMarketOpen { .. } => {
                SpotOrderTriggerRole::StopLoss
            }
            Self::BuyTakeProfitLimitOpen { .. }
            | Self::SellTakeProfitLimitOpen { .. }
            | Self::Triggered
            | Self::Canceled
            | Self::Rejected
            | Self::Expired => SpotOrderTriggerRole::TakeProfit,
        }
    }

    fn execution(&self) -> SpotOrderExecution {
        match self {
            Self::BuyStopLossMarketOpen { aggressive_price, .. }
            | Self::SellStopLossMarketOpen { aggressive_price, .. } => {
                SpotOrderExecution::Market { aggressive_price: *aggressive_price }
            }
            Self::BuyTakeProfitLimitOpen { price, .. }
            | Self::SellTakeProfitLimitOpen { price, .. } => {
                SpotOrderExecution::Limit { price: *price }
            }
            Self::Triggered | Self::Canceled | Self::Rejected | Self::Expired => {
                SpotOrderExecution::Limit { price: 100 }
            }
        }
    }

    fn time_in_force(&self) -> SpotOrderTimeInForce {
        match self.execution() {
            SpotOrderExecution::Market { .. } => SpotOrderTimeInForce::Ioc,
            SpotOrderExecution::Limit { .. } => SpotOrderTimeInForce::Gtc,
        }
    }

    fn qty(&self) -> u64 {
        match self {
            Self::BuyStopLossMarketOpen { qty, .. }
            | Self::BuyTakeProfitLimitOpen { qty, .. }
            | Self::SellStopLossMarketOpen { qty, .. }
            | Self::SellTakeProfitLimitOpen { qty, .. } => *qty,
            Self::Triggered | Self::Canceled | Self::Rejected | Self::Expired => 2,
        }
    }

    fn trigger_price(&self) -> u64 {
        match self {
            Self::BuyStopLossMarketOpen { trigger_price, .. }
            | Self::BuyTakeProfitLimitOpen { trigger_price, .. }
            | Self::SellStopLossMarketOpen { trigger_price, .. }
            | Self::SellTakeProfitLimitOpen { trigger_price, .. } => *trigger_price,
            Self::Triggered | Self::Canceled | Self::Rejected | Self::Expired => 90,
        }
    }

    fn status(&self) -> SpotConditionalOrderStatus {
        match self {
            Self::BuyStopLossMarketOpen { .. }
            | Self::BuyTakeProfitLimitOpen { .. }
            | Self::SellStopLossMarketOpen { .. }
            | Self::SellTakeProfitLimitOpen { .. } => SpotConditionalOrderStatus::Open,
            Self::Triggered => SpotConditionalOrderStatus::Triggered,
            Self::Canceled => SpotConditionalOrderStatus::Canceled,
            Self::Rejected => SpotConditionalOrderStatus::Rejected,
            Self::Expired => SpotConditionalOrderStatus::Expired,
        }
    }
}

fn small_qty_price() -> impl Strategy<Value = (u64, u64)> {
    (1_u64..=1_000_000, 1_u64..=1_000_000)
}

pub(crate) fn active_spot_order_scenario_strategy() -> impl Strategy<Value = ActiveSpotOrderScenario>
{
    prop_oneof![
        small_qty_price()
            .prop_map(|(qty, price)| ActiveSpotOrderScenario::BuyLimitOpen { qty, price }),
        (2_u64..=1_000_000, 1_u64..=1_000_000).prop_flat_map(|(qty, price)| {
            (1_u64..qty).prop_map(move |filled_qty| {
                ActiveSpotOrderScenario::BuyLimitPartiallyFilled { qty, price, filled_qty }
            })
        }),
        small_qty_price()
            .prop_map(|(qty, price)| ActiveSpotOrderScenario::BuyLimitFilled { qty, price }),
        (2_u64..=1_000_000, 1_u64..=1_000_000).prop_flat_map(|(qty, price)| {
            (0_u64..=qty).prop_map(move |filled_qty| ActiveSpotOrderScenario::BuyLimitCanceled {
                qty,
                price,
                filled_qty,
            })
        }),
        small_qty_price()
            .prop_map(|(qty, price)| ActiveSpotOrderScenario::SellLimitOpen { qty, price }),
        small_qty_price().prop_flat_map(|(qty, price)| {
            (1_u64..qty).prop_map(move |filled_qty| {
                ActiveSpotOrderScenario::SellLimitPartiallyFilled { qty, price, filled_qty }
            })
        }),
        small_qty_price()
            .prop_map(|(qty, price)| ActiveSpotOrderScenario::SellLimitFilled { qty, price }),
        small_qty_price().prop_flat_map(|(qty, price)| {
            (0_u64..=qty).prop_map(move |filled_qty| ActiveSpotOrderScenario::SellLimitCanceled {
                qty,
                price,
                filled_qty,
            })
        }),
        small_qty_price().prop_map(|(qty, aggressive_price)| {
            ActiveSpotOrderScenario::BuyMarketOpen { qty, aggressive_price }
        }),
        small_qty_price().prop_map(|(qty, aggressive_price)| {
            ActiveSpotOrderScenario::SellMarketOpen { qty, aggressive_price }
        }),
    ]
}

fn conditional_spot_order_scenario_strategy() -> impl Strategy<Value = ConditionalSpotOrderScenario>
{
    prop_oneof![
        (1_u64..=1_000_000, 1_u64..=1_000_000, 1_u64..=1_000_000).prop_map(
            |(qty, trigger_price, aggressive_price)| {
                ConditionalSpotOrderScenario::BuyStopLossMarketOpen {
                    qty,
                    trigger_price,
                    aggressive_price,
                }
            },
        ),
        (1_u64..=1_000_000, 1_u64..=1_000_000, 1_u64..=1_000_000).prop_map(
            |(qty, trigger_price, price)| {
                ConditionalSpotOrderScenario::BuyTakeProfitLimitOpen { qty, trigger_price, price }
            },
        ),
        (1_u64..=1_000_000, 1_u64..=1_000_000, 1_u64..=1_000_000).prop_map(
            |(qty, trigger_price, aggressive_price)| {
                ConditionalSpotOrderScenario::SellStopLossMarketOpen {
                    qty,
                    trigger_price,
                    aggressive_price,
                }
            },
        ),
        (1_u64..=1_000_000, 1_u64..=1_000_000, 1_u64..=1_000_000).prop_map(
            |(qty, trigger_price, price)| {
                ConditionalSpotOrderScenario::SellTakeProfitLimitOpen { qty, trigger_price, price }
            },
        ),
        Just(ConditionalSpotOrderScenario::Triggered),
        Just(ConditionalSpotOrderScenario::Canceled),
        Just(ConditionalSpotOrderScenario::Rejected),
        Just(ConditionalSpotOrderScenario::Expired),
    ]
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

proptest! {
    #[test]
    fn active_spot_order_scenarios_preserve_business_invariants(
        scenario in active_spot_order_scenario_strategy(),
    ) {
        let order = scenario.order();

        prop_assert_eq!(order.side, scenario.side());
        prop_assert_eq!(order.status, scenario.status());
        prop_assert_eq!(order.filled_qty, scenario.filled_qty());
        prop_assert_eq!(order.can_be_cancelled(), order.status.is_cancelable());
        prop_assert!(order.has_consistent_execution_state());
        prop_assert!(order.has_consistent_reserved_base());
        prop_assert!(order.has_consistent_reserved_quote());
        prop_assert_eq!(order.base_to_release_on_cancel(), scenario.reserved_base());
        prop_assert_eq!(order.quote_to_release_on_cancel(), scenario.reserved_quote());
        prop_assert_eq!(
            created_value(&order, "asset"),
            Some("10001".to_string())
        );
    }

    #[test]
    fn conditional_spot_order_scenarios_do_not_freeze_before_trigger(
        scenario in conditional_spot_order_scenario_strategy(),
    ) {
        let order = scenario.order();

        prop_assert_eq!(order.side, scenario.side());
        prop_assert_eq!(order.status, scenario.status());
        prop_assert_eq!(order.can_be_cancelled(), order.status == SpotConditionalOrderStatus::Open);
        prop_assert_eq!(created_value(&order, "trigger_price"), Some(order.trigger_price.to_string()));
        prop_assert_eq!(created_value(&order, "trigger_role"), Some(order.trigger_role.as_str().to_string()));
    }

    #[test]
    fn open_conditional_spot_order_triggers_active_spot_order(
        scenario in conditional_spot_order_scenario_strategy(),
        reserved_base in 0_u64..=1_000_000,
        reserved_quote in 0_u64..=1_000_000,
    ) {
        let conditional = scenario.order();
        let active = conditional.triggered_order(
            "active-42".to_string(),
            reserved_base,
            reserved_quote,
        );

        prop_assert_eq!(active.order_id, "active-42");
        prop_assert_eq!(active.asset, conditional.asset);
        prop_assert_eq!(active.account_id, conditional.account_id);
        prop_assert_eq!(active.side, conditional.side);
        prop_assert_eq!(active.execution, conditional.execution);
        prop_assert_eq!(active.time_in_force, conditional.time_in_force);
        prop_assert_eq!(active.qty, conditional.qty);
        prop_assert_eq!(active.reserved_base, reserved_base);
        prop_assert_eq!(active.reserved_quote, reserved_quote);
    }
}
