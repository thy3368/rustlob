use example_core_entity::{
    CancelSpotOrderV2Input, PlaceHyperliquidNormalTpslError, PlaceHyperliquidSpotOrderV2Input,
    SpotOrderExecution, SpotOrderGroupRelation, SpotOrderGroupRelationError, SpotOrderSide,
    SpotOrderStatus, SpotOrderStatusReason, SpotOrderTif, SpotOrderTriggerRole, SpotOrderType,
    SpotOrderV2, SpotOrderV2BehaviorError, SpotOrderV2MatchError, TriggerSpotOrderV2Input,
};

fn input(order_id: &str, order_type: SpotOrderType) -> PlaceHyperliquidSpotOrderV2Input {
    PlaceHyperliquidSpotOrderV2Input {
        order_id: order_id.to_owned(),
        asset: 10_000,
        account_id: "trader-1".to_owned(),
        symbol: "PURR/USDC".to_owned(),
        side: SpotOrderSide::Buy,
        limit_price: 110,
        qty: 2,
        reduce_only: true,
        order_type,
        base_asset_id: "PURR".to_owned(),
        quote_asset_id: "USDC".to_owned(),
        base_balance_entity_id: "balance:trader-1:PURR".to_owned(),
        quote_balance_entity_id: "balance:trader-1:USDC".to_owned(),
        maker_fee_bps: 2,
        taker_fee_bps: 5,
        client_order_id: Some(format!("cloid-{order_id}")),
    }
}

fn normal_tpsl_parent() -> PlaceHyperliquidSpotOrderV2Input {
    let mut parent = input("normal-tpsl-parent", SpotOrderType::Limit { tif: SpotOrderTif::Gtc });
    parent.reduce_only = false;
    parent
}

fn normal_tpsl_child(
    order_id: &str,
    role: SpotOrderTriggerRole,
) -> PlaceHyperliquidSpotOrderV2Input {
    let mut child = input(
        order_id,
        SpotOrderType::Trigger {
            is_market: true,
            trigger_price: match role {
                SpotOrderTriggerRole::TakeProfit => 130,
                SpotOrderTriggerRole::StopLoss => 90,
            },
            tpsl: role,
        },
    );
    child.side = SpotOrderSide::Sell;
    child
}

fn assert_relation_error(
    parent: PlaceHyperliquidSpotOrderV2Input,
    children: Vec<PlaceHyperliquidSpotOrderV2Input>,
    expected: SpotOrderGroupRelationError,
) {
    assert_eq!(
        SpotOrderV2::place_hyperliquid_normal_tpsl(parent, children),
        Err(PlaceHyperliquidNormalTpslError::Relation(expected))
    );
}

/// 场景：Given GTC 限价单；When 下单并撤单；Then 冻结存在且生命周期可正常结束。
#[test]
fn limit_gtc_reserves_and_remains_cancelable() -> Result<(), SpotOrderV2BehaviorError> {
    let outcome = SpotOrderV2::place_hyperliquid(input(
        "gtc",
        SpotOrderType::Limit { tif: SpotOrderTif::Gtc },
    ))?;
    let mut order = outcome.order;

    assert!(outcome.freeze_ledger_entry.is_some());
    assert_eq!(order.order_type, SpotOrderType::Limit { tif: SpotOrderTif::Gtc });
    assert!(order.reduce_only);
    assert_eq!(order.client_order_id.as_deref(), Some("cloid-gtc"));
    assert!(order.active_reservation().is_some());
    assert_eq!(order.version, 1);

    order
        .cancel(CancelSpotOrderV2Input { balance_entity_id: "balance:trader-1:USDC".to_owned() })?;
    assert_eq!(order.status, SpotOrderStatus::Canceled);
    assert_eq!(order.version, 2);
    Ok(())
}

/// 场景：Given ALO 限价单；When 价格会立即成交；Then 订单以 BadAloPx 明确拒绝。
#[test]
fn limit_alo_crossing_is_rejected_as_bad_alo_price() -> Result<(), SpotOrderV2MatchError> {
    let mut order = SpotOrderV2::place_hyperliquid(input(
        "alo",
        SpotOrderType::Limit { tif: SpotOrderTif::Alo },
    ))
    .map_err(|_| SpotOrderV2MatchError::OrderNotMatchable)?
    .order;

    order.reject_as_bad_alo()?;
    assert_eq!(order.order_type, SpotOrderType::Limit { tif: SpotOrderTif::Alo });
    assert_eq!(order.status, SpotOrderStatus::Rejected);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::BadAloPxRejected));
    assert_eq!(order.version, 2);
    Ok(())
}

/// 场景：Given IOC 限价单；When 仅部分成交；Then 保留成交量并取消剩余量。
#[test]
fn limit_ioc_keeps_partial_fill_and_cancels_remainder() -> Result<(), SpotOrderV2MatchError> {
    let mut order = SpotOrderV2::place_hyperliquid(input(
        "ioc",
        SpotOrderType::Limit { tif: SpotOrderTif::Ioc },
    ))
    .map_err(|_| SpotOrderV2MatchError::OrderNotMatchable)?
    .order;

    order.finish_after_match(1)?;
    assert_eq!(order.filled_qty, 1);
    assert_eq!(order.status, SpotOrderStatus::Canceled);
    assert_eq!(order.status_reason, Some(SpotOrderStatusReason::IocCancelRejected));
    assert_eq!(order.order_price(), 110);
    Ok(())
}

/// 场景：Given TP 触发限价单；When 创建后再触发；Then 触发前无冻结，触发后进入 GTC。
#[test]
fn trigger_limit_tp_has_no_hold_until_trigger_then_becomes_gtc()
-> Result<(), SpotOrderV2BehaviorError> {
    let outcome = SpotOrderV2::place_hyperliquid(input(
        "trigger-limit",
        SpotOrderType::Trigger {
            is_market: false,
            trigger_price: 105,
            tpsl: SpotOrderTriggerRole::TakeProfit,
        },
    ))?;
    let mut order = outcome.order;

    assert!(outcome.freeze_ledger_entry.is_none());
    assert!(order.is_trigger_pending());
    assert_eq!(order.status, SpotOrderStatus::Pending);
    assert!(order.active_reservation().is_none());
    assert_eq!(order.fill(1), Err(SpotOrderV2MatchError::OrderNotMatchable));

    order.trigger(TriggerSpotOrderV2Input {
        base_asset_id: "PURR".to_owned(),
        quote_asset_id: "USDC".to_owned(),
        maker_fee_bps: 2,
        taker_fee_bps: 5,
    })?;
    assert!(!order.is_trigger_pending());
    assert!(order.active_reservation().is_some());
    assert_eq!(order.time_in_force(), example_core_entity::SpotOrderTif::Gtc);
    assert_eq!(order.version, 2);
    Ok(())
}

/// 场景：Given SL 触发市价单；When 触发并完成撮合；Then 以 IOC 收敛剩余数量。
#[test]
fn trigger_market_sl_becomes_ioc_and_converges_after_match()
-> Result<(), Box<dyn std::error::Error>> {
    let mut order = SpotOrderV2::place_hyperliquid(input(
        "trigger-market",
        SpotOrderType::Trigger {
            is_market: true,
            trigger_price: 95,
            tpsl: SpotOrderTriggerRole::StopLoss,
        },
    ))?
    .order;

    order.trigger(TriggerSpotOrderV2Input {
        base_asset_id: "PURR".to_owned(),
        quote_asset_id: "USDC".to_owned(),
        maker_fee_bps: 2,
        taker_fee_bps: 5,
    })?;
    assert_eq!(order.execution(), SpotOrderExecution::Market { aggressive_price: 110 });
    assert_eq!(order.time_in_force(), example_core_entity::SpotOrderTif::Ioc);
    order.finish_after_match(1)?;
    assert_eq!(order.status, SpotOrderStatus::Canceled);
    assert_eq!(order.filled_qty, 1);
    Ok(())
}

/// 场景：Given 非法价格或数量；When 创建订单；Then 返回精确的输入业务错误。
#[test]
fn invalid_and_terminal_transitions_are_rejected() {
    let mut zero_price = input("zero-price", SpotOrderType::Limit { tif: SpotOrderTif::Gtc });
    zero_price.limit_price = 0;
    assert_eq!(
        SpotOrderV2::place_hyperliquid(zero_price),
        Err(SpotOrderV2BehaviorError::InvalidPrice)
    );

    let mut zero_qty = input("zero-qty", SpotOrderType::Limit { tif: SpotOrderTif::Gtc });
    zero_qty.qty = 0;
    assert_eq!(
        SpotOrderV2::place_hyperliquid(zero_qty),
        Err(SpotOrderV2BehaviorError::InvalidQuantity)
    );
}

/// 场景：Given 一张 entry 父单及 TP/SL 子单；When 原子创建；Then ID 唯一且父子关系完整。
#[test]
fn normal_tpsl_atomically_creates_parent_and_tp_sl_children()
-> Result<(), PlaceHyperliquidNormalTpslError> {
    let mut outcome = SpotOrderV2::place_hyperliquid_normal_tpsl(
        normal_tpsl_parent(),
        vec![
            normal_tpsl_child("normal-tpsl-tp", SpotOrderTriggerRole::TakeProfit),
            normal_tpsl_child("normal-tpsl-sl", SpotOrderTriggerRole::StopLoss),
        ],
    )?;

    assert_eq!(outcome.parent.order.group_relation, SpotOrderGroupRelation::NormalTpslParent);
    assert!(outcome.parent.freeze_ledger_entry.is_some());
    assert_eq!(outcome.children.len(), 2);
    for child in &mut outcome.children {
        assert_eq!(
            child.order.group_relation,
            SpotOrderGroupRelation::NormalTpslChild {
                parent_order_id: outcome.parent.order.order_id.clone(),
            }
        );
        assert!(child.order.is_trigger_pending());
        assert_eq!(child.order.status, SpotOrderStatus::Pending);
        assert!(child.order.active_reservation().is_none());
        assert!(child.freeze_ledger_entry.is_none());
        assert_eq!(child.order.fill(1), Err(SpotOrderV2MatchError::OrderNotMatchable));
    }
    assert_ne!(outcome.children[0].order.order_id, outcome.children[1].order.order_id);
    assert_eq!(
        outcome.children[0].order.order_type,
        SpotOrderType::Trigger {
            is_market: true,
            trigger_price: 130,
            tpsl: SpotOrderTriggerRole::TakeProfit,
        }
    );
    assert_eq!(
        outcome.children[1].order.order_type,
        SpotOrderType::Trigger {
            is_market: true,
            trigger_price: 90,
            tpsl: SpotOrderTriggerRole::StopLoss,
        }
    );
    Ok(())
}

/// 场景：Given 不满足 normalTpsl 不变量的父子输入；When 原子创建；Then 返回对应关系错误。
#[test]
fn normal_tpsl_rejects_each_invalid_parent_child_relation() {
    let mut trigger_parent = normal_tpsl_parent();
    trigger_parent.order_type =
        normal_tpsl_child("ignored", SpotOrderTriggerRole::TakeProfit).order_type;
    assert_relation_error(
        trigger_parent,
        vec![normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit)],
        SpotOrderGroupRelationError::ParentMustBeLimit,
    );

    let mut reduce_only_parent = normal_tpsl_parent();
    reduce_only_parent.reduce_only = true;
    assert_relation_error(
        reduce_only_parent,
        vec![normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit)],
        SpotOrderGroupRelationError::ParentMustNotBeReduceOnly,
    );
    assert_relation_error(
        normal_tpsl_parent(),
        Vec::new(),
        SpotOrderGroupRelationError::ChildrenRequired,
    );

    let mut limit_child = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    limit_child.order_type = SpotOrderType::Limit { tif: SpotOrderTif::Gtc };
    assert_relation_error(
        normal_tpsl_parent(),
        vec![limit_child],
        SpotOrderGroupRelationError::ChildMustBeTrigger,
    );

    let mut not_reduce_only = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    not_reduce_only.reduce_only = false;
    assert_relation_error(
        normal_tpsl_parent(),
        vec![not_reduce_only],
        SpotOrderGroupRelationError::ChildMustBeReduceOnly,
    );

    let mut same_side = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    same_side.side = SpotOrderSide::Buy;
    assert_relation_error(
        normal_tpsl_parent(),
        vec![same_side],
        SpotOrderGroupRelationError::ChildSideMustOpposeParent,
    );

    let mut other_account = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    other_account.account_id = "trader-2".to_owned();
    assert_relation_error(
        normal_tpsl_parent(),
        vec![other_account],
        SpotOrderGroupRelationError::ChildAccountMismatch,
    );

    let mut other_asset = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    other_asset.asset += 1;
    assert_relation_error(
        normal_tpsl_parent(),
        vec![other_asset],
        SpotOrderGroupRelationError::ChildAssetMismatch,
    );

    let mut other_symbol = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    other_symbol.symbol = "BTC/USDC".to_owned();
    assert_relation_error(
        normal_tpsl_parent(),
        vec![other_symbol],
        SpotOrderGroupRelationError::ChildSymbolMismatch,
    );

    let mut oversized = normal_tpsl_child("child", SpotOrderTriggerRole::TakeProfit);
    oversized.qty = 3;
    assert_relation_error(
        normal_tpsl_parent(),
        vec![oversized],
        SpotOrderGroupRelationError::ChildQuantityExceedsParent,
    );

    assert_relation_error(
        normal_tpsl_parent(),
        vec![
            normal_tpsl_child("duplicate", SpotOrderTriggerRole::TakeProfit),
            normal_tpsl_child("duplicate", SpotOrderTriggerRole::StopLoss),
        ],
        SpotOrderGroupRelationError::DuplicateOrderId,
    );
    assert_relation_error(
        normal_tpsl_parent(),
        vec![normal_tpsl_child("normal-tpsl-parent", SpotOrderTriggerRole::TakeProfit)],
        SpotOrderGroupRelationError::DuplicateOrderId,
    );
}
