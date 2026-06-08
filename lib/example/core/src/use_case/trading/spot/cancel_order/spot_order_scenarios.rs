use proptest::prelude::*;

use super::*;
use crate::entity::{ActiveSpotOrderScenario, active_spot_order_scenario_strategy};

/// 撤单用例对 active `SpotOrder` 场景的薄适配。
///
/// 订单本身的业务矩阵统一来自 `entity/spot_order/spot_order_scenarios.rs`；
/// 这里只补充撤单用例需要的账户快照和负向状态。
#[derive(Debug, Clone)]
pub(super) enum SpotOrderCancelScenario {
    /// 订单来自 entity 场景，账户冻结余额足够释放。
    EntityOrder {
        order_scenario: ActiveSpotOrderScenario,
        available_base: u64,
        available_quote: u64,
    },
    /// 订单归属和命令发起方不一致。
    OwnerMismatch { order_scenario: ActiveSpotOrderScenario },
    /// 账户冻结余额小于订单剩余冻结余额。
    FrozenBalanceMismatch { order_scenario: ActiveSpotOrderScenario },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ExpectedAccountRelease {
    pub(super) available_field: &'static str,
    pub(super) frozen_field: &'static str,
    pub(super) next_available: u64,
}

impl SpotOrderCancelScenario {
    /// 构造包含指定订单状态的 given state。
    pub(super) fn state(&self) -> CancelSpotOrderState {
        match self {
            Self::EntityOrder { order_scenario, available_base, available_quote } => {
                let order = order_scenario.order();
                CancelSpotOrderState {
                    account: account(
                        "trader-1",
                        *available_base,
                        order.base_to_release_on_cancel(),
                        *available_quote,
                        order.quote_to_release_on_cancel(),
                    ),
                    open_order: Some(order),
                }
            }
            Self::OwnerMismatch { order_scenario } => {
                let mut order = order_scenario.order();
                order.account_id = "trader-2".to_string();

                CancelSpotOrderState {
                    account: account(
                        "trader-1",
                        0,
                        order.base_to_release_on_cancel(),
                        100,
                        order.quote_to_release_on_cancel(),
                    ),
                    open_order: Some(order),
                }
            }
            Self::FrozenBalanceMismatch { order_scenario } => {
                let order = order_scenario.order();
                let frozen_base = lower_than(order.base_to_release_on_cancel());
                let frozen_quote = lower_than(order.quote_to_release_on_cancel());

                CancelSpotOrderState {
                    account: account("trader-1", 0, frozen_base, 100, frozen_quote),
                    open_order: Some(order),
                }
            }
        }
    }

    /// 当前订单状态下，撤单状态校验应返回的业务结果。
    pub(super) fn expected_validate(&self) -> Result<(), CancelSpotOrderError> {
        match self {
            Self::EntityOrder { order_scenario, .. } if order_scenario.status().is_cancelable() => {
                Ok(())
            }
            Self::EntityOrder { .. } => Err(CancelSpotOrderError::OrderNotCancelable),
            Self::OwnerMismatch { .. } => Err(CancelSpotOrderError::OrderOwnerMismatch),
            Self::FrozenBalanceMismatch { .. } => Err(CancelSpotOrderError::FrozenBalanceMismatch),
        }
    }

    /// 成功撤单时账户事件应释放的字段和值。
    pub(super) fn expected_account_release(&self) -> Option<ExpectedAccountRelease> {
        match self {
            Self::EntityOrder { order_scenario, available_base, available_quote }
                if order_scenario.status().is_cancelable() =>
            {
                match order_scenario.side() {
                    crate::SpotOrderSide::Buy => Some(ExpectedAccountRelease {
                        available_field: "available_quote",
                        frozen_field: "frozen_quote",
                        next_available: available_quote + order_scenario.reserved_quote(),
                    }),
                    crate::SpotOrderSide::Sell => Some(ExpectedAccountRelease {
                        available_field: "available_base",
                        frozen_field: "frozen_base",
                        next_available: available_base + order_scenario.reserved_base(),
                    }),
                }
            }
            Self::EntityOrder { .. }
            | Self::OwnerMismatch { .. }
            | Self::FrozenBalanceMismatch { .. } => None,
        }
    }
}

/// 枚举所有含 active `SpotOrder` 的撤单状态场景。
pub(super) fn spot_order_cancel_scenario_strategy() -> impl Strategy<Value = SpotOrderCancelScenario>
{
    prop_oneof![
        (active_spot_order_scenario_strategy(), 0_u64..=1_000_000, 0_u64..=1_000_000,).prop_map(
            |(order_scenario, available_base, available_quote)| {
                SpotOrderCancelScenario::EntityOrder {
                    order_scenario,
                    available_base,
                    available_quote,
                }
            }
        ),
        active_spot_order_scenario_strategy().prop_map(|order_scenario| {
            SpotOrderCancelScenario::OwnerMismatch { order_scenario }
        }),
        cancelable_entity_spot_order_scenario_strategy().prop_map(|order_scenario| {
            SpotOrderCancelScenario::FrozenBalanceMismatch { order_scenario }
        }),
    ]
}

/// 只枚举可以成功撤销的订单状态。
pub(super) fn cancelable_spot_order_scenario_strategy()
-> impl Strategy<Value = SpotOrderCancelScenario> {
    (cancelable_entity_spot_order_scenario_strategy(), 0_u64..=1_000_000, 0_u64..=1_000_000)
        .prop_map(|(order_scenario, available_base, available_quote)| {
            SpotOrderCancelScenario::EntityOrder { order_scenario, available_base, available_quote }
        })
}

fn cancelable_entity_spot_order_scenario_strategy() -> impl Strategy<Value = ActiveSpotOrderScenario>
{
    active_spot_order_scenario_strategy()
        .prop_filter("cancel order only accepts open or partially filled orders", |scenario| {
            scenario.status().is_cancelable()
        })
}

fn lower_than(value: u64) -> u64 {
    value.saturating_sub(1)
}

fn account(
    account_id: &str,
    available_base: u64,
    frozen_base: u64,
    available_quote: u64,
    frozen_quote: u64,
) -> TradingAccount {
    TradingAccount {
        account_id: account_id.to_string(),
        available_base,
        frozen_base,
        available_quote,
        frozen_quote,
        version: 3,
    }
}
