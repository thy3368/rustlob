use common_entity::{
    AggregateRole, Entity, FieldDiff, FinancialClassification, FourColorArchetype,
};

use crate::entity::option::cex::cex_option::*;

fn put_instrument() -> CexOptionInstrument {
    CexOptionInstrument::new(
        "BTC-20260828-100000-PUT".to_string(),
        "BTC".to_string(),
        "USDT".to_string(),
        "USDT".to_string(),
        1_787_865_600_000,
        100_000_000_000,
        CexOptionType::Put,
        CexOptionInstrumentStatus::Trading,
    )
}

fn call_instrument() -> CexOptionInstrument {
    CexOptionInstrument::new(
        "BTC-20260828-100000-CALL".to_string(),
        "BTC".to_string(),
        "USDT".to_string(),
        "USDT".to_string(),
        1_787_865_600_000,
        100_000_000_000,
        CexOptionType::Call,
        CexOptionInstrumentStatus::Trading,
    )
}

fn place_input(
    instrument: CexOptionInstrument,
    side: CexOptionOrderSide,
    intent: PlaceCexOptionOrderIntent,
) -> PlaceCexOptionOrderInput {
    PlaceCexOptionOrderInput {
        order_id: "option-order-1".to_string(),
        account_id: "account-1".to_string(),
        instrument_id: instrument.instrument_id.clone(),
        instrument,
        order_side: side,
        execution: CexOptionOrderExecution::Limit { price: 2_500_000 },
        time_in_force: CexOptionOrderTimeInForce::Gtc,
        quantity: 10,
        intent,
        premium_amount: Some(25_000_000),
        short_margin_amount: Some(100_000_000),
        fee_amount: None,
        client_order_id: Some("client-option-1".to_string()),
    }
}

#[test]
fn cex_option_instrument_keeps_strike_price_while_order_references_instrument_id() {
    // Given: 已挂牌的 PUT option 合约规格包含行权价与期权类型。
    let instrument = put_instrument();

    // When: 按该合约创建买方开多订单。
    let outcome = CexOptionOrder::place(place_input(
        instrument.clone(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::OpenLong,
    ))
    .unwrap();

    // Then: 订单只引用合约 ID，合约规格事实仍由 instrument 持有。
    assert_eq!(instrument.strike_price, 100_000_000_000);
    assert_eq!(outcome.order.instrument_id, "BTC-20260828-100000-PUT");
    assert!(!outcome
        .order
        .created_field_changes()
        .iter()
        .any(|change| change.field_name == "strike_price" || change.field_name == "option_type"));
}

#[test]
fn buy_put_open_long_derives_premium_hold_requirement() {
    // Given: 买入 PUT 并建立多头，需要为权利金预占用 quote 资产。
    // When: 创建买方开多 option 订单。
    let outcome = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::OpenLong,
    ))
    .unwrap();

    // Then: 订单不是 reduce-only，并派生权利金占用要求而非卖方保证金占用。
    assert_eq!(outcome.order.order_side, CexOptionOrderSide::Buy);
    assert!(!outcome.order.reduce_only);
    assert_eq!(
        outcome.premium_hold,
        Some(CexOptionPremiumHoldRequirement {
            account_id: "account-1".to_string(),
            order_id: "option-order-1".to_string(),
            instrument_id: "BTC-20260828-100000-PUT".to_string(),
            asset_id: "USDT".to_string(),
            premium_price: 2_500_000,
            quantity: 10,
            premium_amount: 25_000_000,
        })
    );
    assert_eq!(outcome.short_margin_hold, None);
}

#[test]
fn sell_put_open_short_derives_short_margin_hold_requirement() {
    // Given: 卖出 PUT 并建立空头，需要为卖方保证金预占用 settle 资产。
    // When: 创建卖方开空 option 订单。
    let outcome = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Sell,
        PlaceCexOptionOrderIntent::OpenShort,
    ))
    .unwrap();

    // Then: 订单不是 reduce-only，并派生卖方保证金占用要求而非权利金占用。
    assert_eq!(outcome.order.order_side, CexOptionOrderSide::Sell);
    assert!(!outcome.order.reduce_only);
    assert_eq!(
        outcome.short_margin_hold,
        Some(CexOptionShortMarginHoldRequirement {
            account_id: "account-1".to_string(),
            order_id: "option-order-1".to_string(),
            instrument_id: "BTC-20260828-100000-PUT".to_string(),
            asset_id: "USDT".to_string(),
            margin_amount: 100_000_000,
        })
    );
    assert_eq!(outcome.premium_hold, None);
}

#[test]
fn buy_call_and_sell_call_orders_can_be_created() {
    // Given: 已挂牌的 CALL option 合约允许买方加多和卖方加空意图。
    // When: 分别创建买入 CALL 与卖出 CALL 订单。
    let buy_call = CexOptionOrder::place(place_input(
        call_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::IncreaseLong,
    ))
    .unwrap();
    let sell_call = CexOptionOrder::place(place_input(
        call_instrument(),
        CexOptionOrderSide::Sell,
        PlaceCexOptionOrderIntent::IncreaseShort,
    ))
    .unwrap();

    // Then: 两个订单都引用 CALL 合约，并按仓位意图派生对应资金占用要求。
    assert_eq!(buy_call.order.instrument_id, "BTC-20260828-100000-CALL");
    assert_eq!(sell_call.order.instrument_id, "BTC-20260828-100000-CALL");
    assert!(buy_call.premium_hold.is_some());
    assert!(sell_call.short_margin_hold.is_some());
}

#[test]
fn place_rejects_zero_quantity() {
    // Given: 一笔买方开多 option 下单输入，但合约数量为零。
    let mut input =
        place_input(put_instrument(), CexOptionOrderSide::Buy, PlaceCexOptionOrderIntent::OpenLong);
    input.quantity = 0;

    // When / Then: 创建订单时拒绝无效数量。
    assert_eq!(CexOptionOrder::place(input), Err(CexOptionOrderBehaviorError::InvalidQuantity));
}

#[test]
fn place_rejects_zero_premium_price() {
    // Given: 一笔买方开多 option 下单输入，但权利金限价为零。
    let mut input =
        place_input(put_instrument(), CexOptionOrderSide::Buy, PlaceCexOptionOrderIntent::OpenLong);
    input.execution = CexOptionOrderExecution::Limit { price: 0 };

    // When / Then: 创建订单时拒绝无效权利金价格。
    assert_eq!(CexOptionOrder::place(input), Err(CexOptionOrderBehaviorError::InvalidPrice));
}

#[test]
fn remaining_quantity_and_matchable_follow_status_and_filled_quantity() {
    // Given: 一笔已创建且未成交的买方开多 option 订单。
    let order = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::OpenLong,
    ))
    .unwrap()
    .order;

    // When / Then: 未成交订单仍有全部剩余数量，并可撤销、可撮合且状态自洽。
    assert_eq!(order.remaining_quantity(), Some(10));
    assert!(order.is_open());
    assert!(order.is_cancelable());
    assert!(order.is_matchable());
    assert!(order.has_consistent_execution_state());

    // When / Then: 部分成交订单按已成交数量扣减剩余数量，仍可继续撮合。
    let partial = order.clone().with_execution_state(CexOptionOrderStatus::PartiallyFilled, 4);
    assert_eq!(partial.remaining_quantity(), Some(6));
    assert!(partial.is_matchable());
    assert!(partial.has_consistent_execution_state());

    // When / Then: 完全成交订单没有剩余数量，进入终态且不可撮合。
    let filled = order.clone().with_execution_state(CexOptionOrderStatus::Filled, 10);
    assert_eq!(filled.remaining_quantity(), Some(0));
    assert!(filled.is_terminal());
    assert!(!filled.is_matchable());
    assert!(filled.has_consistent_execution_state());

    // When / Then: Open 状态带已成交数量会被识别为执行状态不自洽。
    let inconsistent = order.with_execution_state(CexOptionOrderStatus::Open, 1);
    assert!(!inconsistent.has_consistent_execution_state());
}

#[test]
fn order_business_queries_return_owned_facts() {
    // Given: 买回平空的 option 订单应只减仓，不在订单实体内派生资金占用。
    // When: 创建 close-short 买方订单。
    let outcome = CexOptionOrder::place(place_input(
        put_instrument(),
        CexOptionOrderSide::Buy,
        PlaceCexOptionOrderIntent::CloseShort,
    ))
    .unwrap();

    // Then: 订单业务查询返回归属、合约、权利金价格与资金占用事实。
    assert!(outcome.order.reduce_only);
    assert!(outcome.order.belongs_to_account("account-1"));
    assert!(!outcome.order.belongs_to_account("account-2"));
    assert!(outcome.order.trades_instrument("BTC-20260828-100000-PUT"));
    assert!(!outcome.order.trades_instrument("BTC-20260828-100000-CALL"));
    assert_eq!(outcome.order.order_price(), 2_500_000);
    assert_eq!(outcome.order.limit_price(), Some(2_500_000));
    assert_eq!(outcome.premium_hold, None);
    assert_eq!(outcome.short_margin_hold, None);
}

#[test]
fn entity_metadata_matches_option_write_model_classification() {
    // Given: CEX option 合约规格与订单实体参与 option 写模型。
    // When / Then: entity 元数据表达合约描述对象与订单业务凭证聚合根分类。
    assert_eq!(CexOptionInstrument::four_color_archetype(), FourColorArchetype::Description);
    assert_eq!(CexOptionOrder::four_color_archetype(), FourColorArchetype::MomentInterval);
    assert_eq!(CexOptionOrder::aggregate_role(), AggregateRole::AggregateRoot);
    assert_eq!(
        CexOptionOrder::financial_classification(),
        FinancialClassification::BusinessVoucher
    );
}
