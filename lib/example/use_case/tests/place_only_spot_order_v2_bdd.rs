use common_entity::{
    Entity, EntityReplayableEvent, ExecutionContext, ReplayableChanges, StateMachineOwnedV2Diff,
    StateMachineV2Unchecked,
};
use example_core_entity::{
    ReservationStatus, SpotOrderGroupRelation, SpotOrderSide, SpotOrderStatus, SpotOrderTif,
    SpotOrderTriggerRole, SpotOrderType, SpotOrderV2,
};
use example_core_use_case::{
    PlaceOnlySpotOrderV2Changes, PlaceOnlySpotOrderV2Cmd, PlaceOnlySpotOrderV2Error,
    PlaceOnlySpotOrderV2OrderCmd, PlaceOnlySpotOrderV2OrderType, PlaceOnlySpotOrderV2UseCase,
};

const CREATED_AT: u64 = 1_717_171_717_000_000_000;
const NORMAL_TPSL_PARENT_ID: u64 = 100;
const NORMAL_TPSL_TP_ID: u64 = 101;
const NORMAL_TPSL_SL_ID: u64 = 102;

/// 构造普通限价订单 fixture；它可分别代表 GTC、IOC 和 ALO 三种现货订单形态。
fn limit_order(order_id: u64, tif: &str) -> PlaceOnlySpotOrderV2OrderCmd {
    PlaceOnlySpotOrderV2OrderCmd {
        party_id: "trader-1".to_owned(),
        asset: 10_001,
        order_id,
        symbol: "BTCUSDT".to_owned(),
        is_buy: true,
        price: "100".to_owned(),
        size: "2".to_owned(),
        order_type: PlaceOnlySpotOrderV2OrderType::Limit { tif: tif.to_owned() },
        reduce_only: false,
        cloid: Some(format!("cloid-{order_id}")),
        base_asset_id: "BTC".to_owned(),
        quote_asset_id: "USDT".to_owned(),
        maker_fee_bps: 1,
        taker_fee_bps: 5,
    }
}

/// 构造触发订单 fixture；`is_market = false` 表示触发后 GTC，`true` 表示触发后 IOC。
fn trigger_order(
    order_id: u64,
    is_market: bool,
    trigger_role: &str,
) -> PlaceOnlySpotOrderV2OrderCmd {
    PlaceOnlySpotOrderV2OrderCmd {
        party_id: "trader-1".to_owned(),
        asset: 10_001,
        order_id,
        symbol: "BTCUSDT".to_owned(),
        is_buy: false,
        price: "95".to_owned(),
        size: "1".to_owned(),
        order_type: PlaceOnlySpotOrderV2OrderType::Trigger {
            is_market,
            trigger_price: "90".to_owned(),
            trigger_role: trigger_role.to_owned(),
        },
        reduce_only: true,
        cloid: None,
        base_asset_id: "BTC".to_owned(),
        quote_asset_id: "USDT".to_owned(),
        maker_fee_bps: 1,
        taker_fee_bps: 5,
    }
}

/// 返回带稳定执行时间的业务上下文，避免测试依赖系统时钟。
fn execution_context() -> ExecutionContext {
    ExecutionContext { execution_time_ns: CREATED_AT }
}

/// 依次执行命令校验、给定状态校验和带固定时间的 Changes 推导。
fn compute_changes(
    command: &PlaceOnlySpotOrderV2Cmd,
) -> Result<PlaceOnlySpotOrderV2Changes, PlaceOnlySpotOrderV2Error> {
    let use_case = PlaceOnlySpotOrderV2UseCase;

    // 先验证不依赖状态的命令事实，再验证 normalTpsl 的父子业务约束。
    use_case.check_command(command)?;
    use_case.validate_state_given(command, &())?;

    // 通过固定上下文推导唯一业务真相 Changes，避免调用默认的系统时间。
    use_case.compute_state_diff_with_context(command, (), &execution_context())
}

/// 从单订单 Changes 中取出订单，避免测试绕过 Changes 直接构造实体。
fn created_single_order(
    changes: PlaceOnlySpotOrderV2Changes,
) -> Result<example_core_entity::SpotOrderV2, PlaceOnlySpotOrderV2Error> {
    match changes {
        PlaceOnlySpotOrderV2Changes::Single { created_order } => Ok(created_order),
        PlaceOnlySpotOrderV2Changes::NormalTpsl { .. } => {
            Err(PlaceOnlySpotOrderV2Error::BranchMismatch)
        }
    }
}

/// 断言订单处于 Pending，且 principal / fee 都没有产生有效冻结。
fn assert_pending_without_reservations(order: &example_core_entity::SpotOrderV2) {
    // PlaceOnly 只记录创建意图，不进入可撮合 Open 生命周期。
    assert_eq!(order.status, SpotOrderStatus::Pending);
    assert!(order.is_pending());
    assert!(order.active_reservation().is_none());
    assert!(order.active_fee_reservation().is_none());

    // 未触发条件单的两个 reservation 都是零金额、ClosedByRelease 的占位状态。
    assert_eq!(order.reservation.status, ReservationStatus::ClosedByRelease);
    assert_eq!(order.fee_reservation.status, ReservationStatus::ClosedByRelease);
    assert_eq!(order.reservation.original_amount, 0);
    assert_eq!(order.reservation.remaining_amount, 0);
    assert_eq!(order.fee_reservation.original_amount, 0);
    assert_eq!(order.fee_reservation.remaining_amount, 0);
}

/// 断言 replay projection 是创建事件，并且事件实体身份与 Changes 中的订单一致。
fn assert_created_event_for_order(
    event: &EntityReplayableEvent,
    order: &example_core_entity::SpotOrderV2,
) {
    // Changes 是权威业务真相，事件只应投影为同一订单的 create 事实。
    assert!(event.is_created());
    assert_eq!(event.entity_type, SpotOrderV2::entity_type());
    assert_eq!(
        event.entity_id,
        order.track_create_event().expect("订单创建事件应可投影").entity_id
    );
    assert_eq!(event.old_version, 0);
    assert_eq!(event.new_version, 1);
}

/// 场景：Given 五种 Hyperliquid 现货订单形态；When 通过 place-only use case 创建；
/// Then 五种订单都先进入 Pending，且创建阶段不产生有效冻结。
#[test]
fn given_hyperliquid_spot_order_shapes_when_placed_then_lifecycle_and_holds_match() {
    for (order_id, tif, expected_tif) in [
        (1, "gtc", SpotOrderTif::Gtc),
        (2, "ioc", SpotOrderTif::Ioc),
        (3, "alo", SpotOrderTif::Alo),
    ] {
        let command = PlaceOnlySpotOrderV2Cmd::Single(limit_order(order_id, tif));
        let order = created_single_order(compute_changes(&command).expect("限价订单应创建成功"))
            .expect("Single 分支应产生单订单");

        // 三种普通限价单都应保留命令指定的有效方式，并在创建阶段保持 Pending。
        assert_eq!(order.time_in_force(), expected_tif);
        assert_eq!(order.order_type, SpotOrderType::Limit { tif: expected_tif });
        assert_pending_without_reservations(&order);
        assert!(!order.can_enter_matching());

        // 固定执行上下文必须透传到订单生命周期时间，且创建时 updated_at 与 created_at 相同。
        assert_eq!(order.created_at, CREATED_AT);
        assert_eq!(order.updated_at, CREATED_AT);
        assert_eq!(order.version, 1);
    }

    for (order_id, is_market, trigger_role, expected_tif, expected_role) in [
        (4, false, "tp", SpotOrderTif::Gtc, SpotOrderTriggerRole::TakeProfit),
        (5, true, "sl", SpotOrderTif::Ioc, SpotOrderTriggerRole::StopLoss),
    ] {
        let command =
            PlaceOnlySpotOrderV2Cmd::Single(trigger_order(order_id, is_market, trigger_role));
        let order = created_single_order(compute_changes(&command).expect("触发订单应创建成功"))
            .expect("Single 分支应产生单订单");

        // is_market=false 的触发单映射为 GTC，is_market=true 的触发单映射为 IOC。
        assert_eq!(order.time_in_force(), expected_tif);
        assert_eq!(
            order.order_type,
            SpotOrderType::Trigger { is_market, trigger_price: 90, tpsl: expected_role }
        );
        assert_pending_without_reservations(&order);
        assert!(!order.can_enter_matching());

        // 触发单同样必须使用固定创建时间，但其冻结状态不因创建而提前激活。
        assert_eq!(order.created_at, CREATED_AT);
        assert_eq!(order.updated_at, CREATED_AT);
        assert_eq!(order.version, 1);
    }
}

/// 场景：Given 一个合法普通限价父单和多个 TP/SL 触发子单；When 原子创建；
/// Then Changes 先表达父单再表达子单，父子角色、方向和冻结语义完整。
#[test]
fn given_valid_normal_tpsl_when_placed_then_changes_keep_parent_child_truth() {
    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: limit_order(NORMAL_TPSL_PARENT_ID, "gtc"),
        children: vec![
            trigger_order(NORMAL_TPSL_TP_ID, false, "tp"),
            trigger_order(NORMAL_TPSL_SL_ID, true, "sl"),
        ],
    };
    let changes = compute_changes(&command).expect("合法 normalTpsl 应创建成功");

    let PlaceOnlySpotOrderV2Changes::NormalTpsl { created_parent_order, created_child_orders } =
        &changes
    else {
        panic!("normalTpsl 命令必须产生 NormalTpsl Changes");
    };

    // 父单是非 reduce-only 的普通限价单，但创建阶段仍保持 Pending 且不冻结。
    assert_eq!(created_parent_order.order_id, NORMAL_TPSL_PARENT_ID);
    assert_eq!(created_parent_order.group_relation, SpotOrderGroupRelation::NormalTpslParent);
    assert_eq!(created_parent_order.side, SpotOrderSide::Buy);
    assert!(!created_parent_order.reduce_only);
    assert_eq!(created_parent_order.order_type, SpotOrderType::Limit { tif: SpotOrderTif::Gtc });
    assert_pending_without_reservations(created_parent_order);
    assert!(!created_parent_order.can_enter_matching());

    // 子单数量、方向、账户、资产和交易对都来自合法父子约束；每个子单保持 Pending 且不冻结。
    assert_eq!(created_child_orders.len(), 2);
    for child in created_child_orders {
        assert_eq!(
            child.group_relation,
            SpotOrderGroupRelation::NormalTpslChild { parent_order_id: NORMAL_TPSL_PARENT_ID }
        );
        assert_eq!(child.side, SpotOrderSide::Sell);
        assert!(child.reduce_only);
        assert_eq!(child.account_id, created_parent_order.account_id);
        assert_eq!(child.asset, created_parent_order.asset);
        assert_eq!(child.symbol, created_parent_order.symbol);
        assert_pending_without_reservations(child);
        assert!(!child.can_enter_matching());
        assert_eq!(child.created_at, CREATED_AT);
        assert_eq!(child.updated_at, CREATED_AT);
    }

    let events = changes.to_replayable_events().expect("Changes 应可投影为 replay events");

    // replay 事件数量和实体类型必须与 Changes 中的父单加子单一一对应。
    assert_eq!(events.len(), 3);
    assert!(events.iter().all(EntityReplayableEvent::is_created));
    assert!(events.iter().all(|event| event.entity_type == SpotOrderV2::entity_type()));

    // 业务事件顺序必须是父单 -> TP 子单 -> SL 子单，不能只依赖事件数量。
    assert_created_event_for_order(&events[0], created_parent_order);
    assert_created_event_for_order(&events[1], &created_child_orders[0]);
    assert_created_event_for_order(&events[2], &created_child_orders[1]);
}

/// 场景：Given 单订单命令包含非法输入事实；When 执行 command pre-check；
/// Then 返回精确业务错误，且不会进入状态校验或实体创建。
#[test]
fn given_invalid_order_facts_when_command_is_checked_then_rejects_precisely() {
    let use_case = PlaceOnlySpotOrderV2UseCase;

    let mut invalid_price = limit_order(200, "gtc");
    invalid_price.price = "0".to_owned();
    // 价格必须是正整数字符串。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_price)),
        Err(PlaceOnlySpotOrderV2Error::InvalidPrice)
    );

    let mut invalid_size = limit_order(201, "gtc");
    invalid_size.size = "not-a-number".to_owned();
    // 数量必须是正整数字符串。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_size)),
        Err(PlaceOnlySpotOrderV2Error::InvalidSize)
    );

    let invalid_tif = limit_order(202, "day");
    // 现货限价单只允许 gtc、ioc、alo。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_tif)),
        Err(PlaceOnlySpotOrderV2Error::InvalidTimeInForce)
    );

    let mut invalid_trigger_price = trigger_order(203, false, "sl");
    invalid_trigger_price.order_type = PlaceOnlySpotOrderV2OrderType::Trigger {
        is_market: false,
        trigger_price: "0".to_owned(),
        trigger_role: "sl".to_owned(),
    };
    // 触发价格必须是正整数字符串。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_trigger_price)),
        Err(PlaceOnlySpotOrderV2Error::InvalidTriggerPrice)
    );

    let invalid_trigger_role = trigger_order(204, false, "break_even");
    // 触发角色只允许 TP 或 SL 的公开语义别名。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(invalid_trigger_role)),
        Err(PlaceOnlySpotOrderV2Error::InvalidTriggerRole)
    );

    let mut empty_party = limit_order(205, "gtc");
    empty_party.party_id.clear();
    // 命令主体不能为空。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::Single(empty_party)),
        Err(PlaceOnlySpotOrderV2Error::EmptyPartyId)
    );
}

/// 场景：Given normalTpsl 的父子命令不满足关系约束；When 执行 state validation；
/// Then 精确拒绝空子单、错误角色、方向、归属、数量和重复身份。
#[test]
fn given_invalid_normal_tpsl_relations_when_state_is_validated_then_rejects_business_conflicts() {
    let use_case = PlaceOnlySpotOrderV2UseCase;

    // 空子单列表无法形成 normalTpsl 父子事实，属于 command pre-check 错误。
    assert_eq!(
        use_case.check_command(&PlaceOnlySpotOrderV2Cmd::NormalTpsl {
            parent: limit_order(300, "gtc"),
            children: vec![],
        }),
        Err(PlaceOnlySpotOrderV2Error::ChildrenRequired)
    );

    let parent = limit_order(301, "gtc");
    let valid_child = trigger_order(302, false, "tp");

    // 父单必须是非 reduce-only 的普通限价单。
    let trigger_parent = trigger_order(303, false, "sl");
    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: trigger_parent,
        children: vec![valid_child.clone()],
    };
    assert_eq!(
        use_case.validate_state_given(&command, &()),
        Err(PlaceOnlySpotOrderV2Error::ParentMustBeLimit)
    );

    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: PlaceOnlySpotOrderV2OrderCmd { reduce_only: true, ..parent.clone() },
        children: vec![valid_child.clone()],
    };
    assert_eq!(
        use_case.validate_state_given(&command, &()),
        Err(PlaceOnlySpotOrderV2Error::ParentMustNotBeReduceOnly)
    );

    // 子单必须是 Trigger，且必须声明 reduce-only。
    let non_trigger_child = PlaceOnlySpotOrderV2OrderCmd {
        order_id: 304,
        is_buy: false,
        reduce_only: true,
        ..limit_order(304, "gtc")
    };
    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: parent.clone(),
        children: vec![non_trigger_child],
    };
    assert_eq!(
        use_case.validate_state_given(&command, &()),
        Err(PlaceOnlySpotOrderV2Error::ChildMustBeTrigger)
    );

    let non_reduce_only_child =
        PlaceOnlySpotOrderV2OrderCmd { reduce_only: false, ..valid_child.clone() };
    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: parent.clone(),
        children: vec![non_reduce_only_child],
    };
    assert_eq!(
        use_case.validate_state_given(&command, &()),
        Err(PlaceOnlySpotOrderV2Error::ChildMustBeReduceOnly)
    );

    // 子单必须与父单方向相反。
    let same_side_child = PlaceOnlySpotOrderV2OrderCmd { is_buy: true, ..valid_child.clone() };
    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: parent.clone(),
        children: vec![same_side_child],
    };
    assert_eq!(
        use_case.validate_state_given(&command, &()),
        Err(PlaceOnlySpotOrderV2Error::ChildSideMustOpposeParent)
    );

    // 子单必须属于同一账户、同一 asset 和同一 symbol。
    for (field_name, child, expected_error) in [
        (
            "account",
            PlaceOnlySpotOrderV2OrderCmd { party_id: "trader-2".to_owned(), ..valid_child.clone() },
            PlaceOnlySpotOrderV2Error::ChildAccountMismatch,
        ),
        (
            "asset",
            PlaceOnlySpotOrderV2OrderCmd { asset: 10_002, ..valid_child.clone() },
            PlaceOnlySpotOrderV2Error::ChildAssetMismatch,
        ),
        (
            "symbol",
            PlaceOnlySpotOrderV2OrderCmd { symbol: "ETHUSDT".to_owned(), ..valid_child.clone() },
            PlaceOnlySpotOrderV2Error::ChildSymbolMismatch,
        ),
    ] {
        let command =
            PlaceOnlySpotOrderV2Cmd::NormalTpsl { parent: parent.clone(), children: vec![child] };

        // 每个归属字段都必须被单独校验，避免用一个宽泛错误吞掉关系冲突。
        assert_eq!(
            use_case.validate_state_given(&command, &()),
            Err(expected_error),
            "child {field_name} mismatch should be rejected"
        );
    }

    // 子单数量不能超过父单数量。
    let oversized_child =
        PlaceOnlySpotOrderV2OrderCmd { size: "3".to_owned(), ..valid_child.clone() };
    let command = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: parent.clone(),
        children: vec![oversized_child],
    };
    assert_eq!(
        use_case.validate_state_given(&command, &()),
        Err(PlaceOnlySpotOrderV2Error::ChildQuantityExceedsParent)
    );

    // 父单和子单必须拥有全局唯一的订单 ID。
    let duplicate_parent_id = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent: parent.clone(),
        children: vec![PlaceOnlySpotOrderV2OrderCmd { order_id: 301, ..valid_child.clone() }],
    };
    assert_eq!(
        use_case.validate_state_given(&duplicate_parent_id, &()),
        Err(PlaceOnlySpotOrderV2Error::DuplicateOrderId)
    );

    let duplicate_child_id = PlaceOnlySpotOrderV2Cmd::NormalTpsl {
        parent,
        children: vec![valid_child.clone(), PlaceOnlySpotOrderV2OrderCmd { ..valid_child }],
    };
    assert_eq!(
        use_case.validate_state_given(&duplicate_child_id, &()),
        Err(PlaceOnlySpotOrderV2Error::DuplicateOrderId)
    );
}
