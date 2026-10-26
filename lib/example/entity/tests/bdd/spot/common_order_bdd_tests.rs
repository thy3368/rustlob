use base_types::UserId;
use example_core_entity::entity::common::common_order::{
    CommonOrder, CommonOrderExecution, CommonOrderTrigger,
};
use example_core_entity::{
    OrderGroupRelation, OrderSide, OrderTif, OrderTriggerRole, SpotOrderStatus,
};

fn make_order(
    execution: CommonOrderExecution,
    tif: OrderTif,
    trigger: CommonOrderTrigger,
    trigger_px: Option<u64>,
    is_position_tpsl: bool,
    group_relation: OrderGroupRelation,
    limit_px: u64,
) -> CommonOrder {
    CommonOrder {
        owner: UserId(1),
        order_id: 7,
        asset: 10001,
        pair: "BTCUSDT".into(),
        side: OrderSide::Buy,
        limit_px,
        size: 2,
        orig_size: 2,
        execution,
        tif,
        trigger,
        trigger_px,
        is_position_tpsl,
        group_relation,
        reduce_only: false,
        status: SpotOrderStatus::Open,
        taker_max_fee: 1,
        cloid: Some("c1".into()),
        created_at: 1,
        updated_at: 1,
        version: 1,
    }
}

#[test]
fn valid_order_combinations_preserve_execution_and_trigger_facts() {
    let cases = [
        (
            "ordinary_limit_gtc",
            CommonOrderExecution::Limit,
            OrderTif::Gtc,
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::Standalone,
            OrderTif::Gtc,
            false,
            None,
        ),
        (
            "ordinary_limit_ioc",
            CommonOrderExecution::Limit,
            OrderTif::Ioc,
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::Standalone,
            OrderTif::Ioc,
            false,
            None,
        ),
        (
            "ordinary_limit_alo",
            CommonOrderExecution::Limit,
            OrderTif::Alo,
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::Standalone,
            OrderTif::Alo,
            false,
            None,
        ),
        (
            "ordinary_market_execution",
            CommonOrderExecution::Market,
            OrderTif::Gtc,
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::Standalone,
            OrderTif::Ioc,
            false,
            None,
        ),
        (
            "normal_tpsl_parent",
            CommonOrderExecution::Limit,
            OrderTif::Gtc,
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::NormalTpslParent,
            OrderTif::Gtc,
            false,
            None,
        ),
        (
            "normal_tpsl_take_profit_child",
            CommonOrderExecution::Limit,
            OrderTif::Gtc,
            CommonOrderTrigger::TakeProfit,
            Some(105),
            false,
            OrderGroupRelation::NormalTpslChild { parent_order_id: 7 },
            OrderTif::Gtc,
            true,
            Some(OrderTriggerRole::TakeProfit),
        ),
        (
            "normal_tpsl_stop_loss_child",
            CommonOrderExecution::Market,
            OrderTif::Gtc,
            CommonOrderTrigger::StopLoss,
            Some(95),
            false,
            OrderGroupRelation::NormalTpslChild { parent_order_id: 7 },
            OrderTif::Ioc,
            true,
            Some(OrderTriggerRole::StopLoss),
        ),
        (
            "standalone_take_profit_trigger",
            CommonOrderExecution::Limit,
            OrderTif::Gtc,
            CommonOrderTrigger::TakeProfit,
            Some(105),
            false,
            OrderGroupRelation::Standalone,
            OrderTif::Gtc,
            true,
            Some(OrderTriggerRole::TakeProfit),
        ),
        (
            "standalone_stop_loss_trigger",
            CommonOrderExecution::Market,
            OrderTif::Gtc,
            CommonOrderTrigger::StopLoss,
            Some(95),
            false,
            OrderGroupRelation::Standalone,
            OrderTif::Ioc,
            true,
            Some(OrderTriggerRole::StopLoss),
        ),
        (
            "position_take_profit",
            CommonOrderExecution::Limit,
            OrderTif::Gtc,
            CommonOrderTrigger::TakeProfit,
            Some(105),
            true,
            OrderGroupRelation::Standalone,
            OrderTif::Gtc,
            true,
            Some(OrderTriggerRole::TakeProfit),
        ),
        (
            "position_stop_loss",
            CommonOrderExecution::Market,
            OrderTif::Gtc,
            CommonOrderTrigger::StopLoss,
            Some(95),
            true,
            OrderGroupRelation::Standalone,
            OrderTif::Ioc,
            true,
            Some(OrderTriggerRole::StopLoss),
        ),
    ];
    for (
        name,
        execution,
        tif,
        trigger,
        trigger_px,
        is_position_tpsl,
        relation,
        expected_tif,
        expected_trigger,
        expected_role,
    ) in cases
    {
        let order =
            make_order(execution, tif, trigger, trigger_px, is_position_tpsl, relation, 101);
        assert_eq!(order.effective_tif(), expected_tif, "{name}");
        assert_eq!(order.is_trigger(), expected_trigger, "{name}");
        assert_eq!(order.trigger_role(), expected_role, "{name}");
        assert!(order.has_consistent_execution_facts(), "{name}");
        let restored: CommonOrder =
            serde_json::from_str(&serde_json::to_string(&order).unwrap()).unwrap();
        assert_eq!(restored, order, "{name}");
    }
}

#[test]
fn invalid_order_combinations_are_rejected() {
    let cases = [
        (
            "ordinary_order_with_trigger_price",
            CommonOrderTrigger::None,
            Some(105),
            false,
            OrderGroupRelation::Standalone,
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "take_profit_without_trigger_price",
            CommonOrderTrigger::TakeProfit,
            None,
            false,
            OrderGroupRelation::Standalone,
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "stop_loss_with_zero_trigger_price",
            CommonOrderTrigger::StopLoss,
            Some(0),
            false,
            OrderGroupRelation::Standalone,
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "normal_tpsl_parent_marked_take_profit",
            CommonOrderTrigger::TakeProfit,
            Some(105),
            false,
            OrderGroupRelation::NormalTpslParent,
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "normal_tpsl_child_without_trigger",
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::NormalTpslChild { parent_order_id: 7 },
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "position_tpsl_using_normal_tpsl_parent",
            CommonOrderTrigger::TakeProfit,
            Some(105),
            true,
            OrderGroupRelation::NormalTpslParent,
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "position_tpsl_using_normal_tpsl_child",
            CommonOrderTrigger::StopLoss,
            Some(95),
            true,
            OrderGroupRelation::NormalTpslChild { parent_order_id: 7 },
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "position_tpsl_without_condition",
            CommonOrderTrigger::None,
            None,
            true,
            OrderGroupRelation::Standalone,
            CommonOrderExecution::Limit,
            100,
        ),
        (
            "market_order_without_aggressive_limit",
            CommonOrderTrigger::None,
            None,
            false,
            OrderGroupRelation::Standalone,
            CommonOrderExecution::Market,
            0,
        ),
    ];
    for (name, trigger, trigger_px, is_position_tpsl, relation, execution, limit_px) in cases {
        assert!(
            !make_order(
                execution,
                OrderTif::Gtc,
                trigger,
                trigger_px,
                is_position_tpsl,
                relation,
                limit_px
            )
            .has_consistent_execution_facts(),
            "{name}"
        );
    }
}
