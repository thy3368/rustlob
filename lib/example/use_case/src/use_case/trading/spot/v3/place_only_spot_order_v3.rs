use std::collections::HashSet;

use base_types::UserId;
use common_entity::{
    EntityError, EntityReplayableEvent, LogEntry, ReplayableChanges, StateMachineV2,
    StateMachineV2Diff,
};
use example_core_entity::account::symbol::Symbol;
use example_core_entity::common::{CommonOrderExecution, CommonOrderTrigger, OrderBase};
use example_core_entity::spot::spot_order_v3::Pending;
use example_core_entity::{OrderGroupRelation, OrderSide, OrderTif, SpotOrderStatus, SpotOrderV3};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceOnlySpotOrderV3OrderType {
    None,
    Trigger { is_market: bool, trigger_price: String, trigger_role: CommonOrderTrigger },
}

/// 创建委托所需的完整事实；价格和数量按 Decimal 精确解析，不舍入。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceOnlySpotOrderV3OrderCmd {
    pub symbol: Symbol,
    pub owner: UserId,
    pub taker_max_fee: Decimal,
    pub order_id: u64,
    pub is_buy: bool,
    pub price: String,
    pub size: String,
    pub order_type: PlaceOnlySpotOrderV3OrderType,
    pub reduce_only: bool,
    pub tif: OrderTif,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceOnlySpotOrderV3Cmd {
    Single(PlaceOnlySpotOrderV3OrderCmd),
    NormalTpsl { parent: PlaceOnlySpotOrderV3OrderCmd, children: Vec<PlaceOnlySpotOrderV3OrderCmd> },
}

pub type PlaceOnlySpotOrderV3State = ();

/// 本次仅创建 Pending 事实；不进入 V2 撮合、撤单链路。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV3AfterChanges {
    Single {
        created_order: SpotOrderV3<Pending>,
    },
    NormalTpsl {
        created_parent_order: SpotOrderV3<Pending>,
        created_child_orders: Vec<SpotOrderV3<Pending>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceOnlySpotOrderV3Changes {
    Single {
        created_order: SpotOrderV3<Pending>,
    },
    NormalTpsl {
        created_parent_order: SpotOrderV3<Pending>,
        created_child_orders: Vec<SpotOrderV3<Pending>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceOnlySpotOrderV3Error {
    #[error("order oid must be greater than zero")]
    InvalidOrderId,
    #[error("price must be a positive decimal string")]
    InvalidPrice,
    #[error("size must be a positive decimal string")]
    InvalidSize,
    #[error("maximum taker fee must not be negative")]
    InvalidFee,
    #[error("trigger price must be a positive decimal string")]
    InvalidTriggerPrice,
    #[error("trigger role must be take profit or stop loss")]
    InvalidTriggerRole,
    #[error("normalTpsl must contain at least one child order")]
    ChildrenRequired,
    #[error("normalTpsl parent must be a limit order")]
    ParentMustBeLimit,
    #[error("normalTpsl parent must not be reduce-only")]
    ParentMustNotBeReduceOnly,
    #[error("normalTpsl child must be a trigger order")]
    ChildMustBeTrigger,
    #[error("normalTpsl child must be reduce-only")]
    ChildMustBeReduceOnly,
    #[error("normalTpsl child must have the opposite side from its parent")]
    ChildSideMustOpposeParent,
    #[error("normalTpsl child account differs from its parent")]
    ChildAccountMismatch,
    #[error("normalTpsl child assets differ from its parent")]
    ChildAssetMismatch,
    #[error("normalTpsl child symbol differs from its parent")]
    ChildSymbolMismatch,
    #[error("normalTpsl child quantity exceeds its parent quantity")]
    ChildQuantityExceedsParent,
    #[error("normalTpsl order ids must be unique")]
    DuplicateOrderId,
}

/// 无既有订单状态的创建入口。先校验命令与组关系，再装配实体。
///
/// 时间与版本分别来自命令 logical_ts 和 LSN，同一输入产生同一 after truth。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceOnlySpotOrderV3UseCase;

impl StateMachineV2 for PlaceOnlySpotOrderV3UseCase {
    type Command = LogEntry<PlaceOnlySpotOrderV3Cmd>;
    type StateGiven = PlaceOnlySpotOrderV3State;
    type Error = PlaceOnlySpotOrderV3Error;
    type StateChanged = PlaceOnlySpotOrderV3AfterChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        match cmd.payload() {
            PlaceOnlySpotOrderV3Cmd::Single(order) => check_order_command(order),
            PlaceOnlySpotOrderV3Cmd::NormalTpsl { parent, children } => {
                check_order_command(parent)?;
                if children.is_empty() {
                    return Err(PlaceOnlySpotOrderV3Error::ChildrenRequired);
                }
                for child in children {
                    check_order_command(child)?;
                }
                validate_normal_tpsl(parent, children)
            }
        }
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        _given_state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        match cmd.payload() {
            PlaceOnlySpotOrderV3Cmd::Single(order) => {
                Ok(PlaceOnlySpotOrderV3AfterChanges::Single {
                    created_order: SpotOrderV3::single(build_order(
                        order,
                        cmd.lsn(),
                        cmd.logical_ts(),
                    )?),
                })
            }
            PlaceOnlySpotOrderV3Cmd::NormalTpsl { parent, children } => {
                let parent = build_order(parent, cmd.lsn(), cmd.logical_ts())?;
                let children = children
                    .iter()
                    .map(|child| build_order(child, cmd.lsn(), cmd.logical_ts()))
                    .collect::<Result<Vec<_>, _>>()?;
                let (created_parent_order, created_child_orders) =
                    SpotOrderV3::normal_tpsl(parent, children);
                Ok(PlaceOnlySpotOrderV3AfterChanges::NormalTpsl {
                    created_parent_order,
                    created_child_orders,
                })
            }
        }
    }
}

impl StateMachineV2Diff for PlaceOnlySpotOrderV3UseCase {
    type StateDiff = PlaceOnlySpotOrderV3Changes;

    fn do_compute_state_diff(
        _: (),
        after: Self::StateChanged,
    ) -> Result<Self::StateDiff, Self::Error> {
        Ok(match after {
            PlaceOnlySpotOrderV3AfterChanges::Single { created_order } => {
                PlaceOnlySpotOrderV3Changes::Single { created_order }
            }
            PlaceOnlySpotOrderV3AfterChanges::NormalTpsl {
                created_parent_order,
                created_child_orders,
            } => PlaceOnlySpotOrderV3Changes::NormalTpsl {
                created_parent_order,
                created_child_orders,
            },
        })
    }
}

impl ReplayableChanges for PlaceOnlySpotOrderV3Changes {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        match self {
            Self::Single { created_order } => Ok(vec![created_order.track_create_event()?]),
            Self::NormalTpsl { created_parent_order, created_child_orders } => {
                let mut events = Vec::with_capacity(created_child_orders.len() + 1);
                events.push(created_parent_order.track_create_event()?);
                for child in created_child_orders {
                    events.push(child.track_create_event()?);
                }
                Ok(events)
            }
        }
    }
}

fn check_order_command(
    order: &PlaceOnlySpotOrderV3OrderCmd,
) -> Result<(), PlaceOnlySpotOrderV3Error> {
    if order.order_id == 0 {
        return Err(PlaceOnlySpotOrderV3Error::InvalidOrderId);
    }
    parse_positive_decimal(&order.price, PlaceOnlySpotOrderV3Error::InvalidPrice)?;
    parse_positive_decimal(&order.size, PlaceOnlySpotOrderV3Error::InvalidSize)?;
    if order.taker_max_fee < Decimal::ZERO {
        return Err(PlaceOnlySpotOrderV3Error::InvalidFee);
    }
    if let PlaceOnlySpotOrderV3OrderType::Trigger { trigger_price, trigger_role, .. } =
        &order.order_type
    {
        parse_positive_decimal(trigger_price, PlaceOnlySpotOrderV3Error::InvalidTriggerPrice)?;
        if *trigger_role == CommonOrderTrigger::None {
            return Err(PlaceOnlySpotOrderV3Error::InvalidTriggerRole);
        }
    }
    Ok(())
}

fn validate_normal_tpsl(
    parent: &PlaceOnlySpotOrderV3OrderCmd,
    children: &[PlaceOnlySpotOrderV3OrderCmd],
) -> Result<(), PlaceOnlySpotOrderV3Error> {
    if !matches!(parent.order_type, PlaceOnlySpotOrderV3OrderType::None) {
        return Err(PlaceOnlySpotOrderV3Error::ParentMustBeLimit);
    }
    if parent.reduce_only {
        return Err(PlaceOnlySpotOrderV3Error::ParentMustNotBeReduceOnly);
    }
    let parent_size = parse_positive_decimal(&parent.size, PlaceOnlySpotOrderV3Error::InvalidSize)?;
    let mut ids = HashSet::with_capacity(children.len() + 1);
    ids.insert(parent.order_id);
    for child in children {
        if !matches!(child.order_type, PlaceOnlySpotOrderV3OrderType::Trigger { .. }) {
            return Err(PlaceOnlySpotOrderV3Error::ChildMustBeTrigger);
        }
        if !child.reduce_only {
            return Err(PlaceOnlySpotOrderV3Error::ChildMustBeReduceOnly);
        }
        if child.is_buy == parent.is_buy {
            return Err(PlaceOnlySpotOrderV3Error::ChildSideMustOpposeParent);
        }
        if child.owner != parent.owner {
            return Err(PlaceOnlySpotOrderV3Error::ChildAccountMismatch);
        }
        if child.symbol.base != parent.symbol.base || child.symbol.quote != parent.symbol.quote {
            return Err(PlaceOnlySpotOrderV3Error::ChildAssetMismatch);
        }
        if child.symbol.id != parent.symbol.id {
            return Err(PlaceOnlySpotOrderV3Error::ChildSymbolMismatch);
        }
        if parse_positive_decimal(&child.size, PlaceOnlySpotOrderV3Error::InvalidSize)?
            > parent_size
        {
            return Err(PlaceOnlySpotOrderV3Error::ChildQuantityExceedsParent);
        }
        if !ids.insert(child.order_id) {
            return Err(PlaceOnlySpotOrderV3Error::DuplicateOrderId);
        }
    }
    Ok(())
}

fn build_order(
    order: &PlaceOnlySpotOrderV3OrderCmd,
    version: u64,
    created_at: u64,
) -> Result<OrderBase, PlaceOnlySpotOrderV3Error> {
    let size = parse_positive_decimal(&order.size, PlaceOnlySpotOrderV3Error::InvalidSize)?;
    let (execution, trigger, trigger_px, tif) = match &order.order_type {
        PlaceOnlySpotOrderV3OrderType::None => {
            (CommonOrderExecution::Limit, CommonOrderTrigger::None, None, order.tif)
        }
        PlaceOnlySpotOrderV3OrderType::Trigger { is_market, trigger_price, trigger_role } => (
            if *is_market { CommonOrderExecution::Market } else { CommonOrderExecution::Limit },
            *trigger_role,
            Some(parse_positive_decimal(
                trigger_price,
                PlaceOnlySpotOrderV3Error::InvalidTriggerPrice,
            )?),
            if *is_market { OrderTif::Ioc } else { order.tif },
        ),
    };
    Ok(OrderBase {
        owner: order.owner,
        order_id: order.order_id,
        symbol: order.symbol,
        side: if order.is_buy { OrderSide::Buy } else { OrderSide::Sell },
        limit_px: parse_positive_decimal(&order.price, PlaceOnlySpotOrderV3Error::InvalidPrice)?,
        size,
        orig_size: size,
        execution,
        tif,
        trigger,
        trigger_px,
        group_relation: OrderGroupRelation::Standalone,
        is_position_tpsl: false,
        reduce_only: order.reduce_only,
        status: SpotOrderStatus::Pending,
        taker_max_fee: order.taker_max_fee,
        cloid: order.cloid.clone(),
        created_at,
        updated_at: created_at,
        version,
    })
}

fn parse_positive_decimal(
    raw: &str,
    error: PlaceOnlySpotOrderV3Error,
) -> Result<Decimal, PlaceOnlySpotOrderV3Error> {
    match Decimal::from_str_exact(raw) {
        Ok(value) if value > Decimal::ZERO => Ok(value),
        _ => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use common_entity::LogEntrySequencer;
    use example_core_entity::account::coin::Coin;
    use rstest::rstest;

    use super::*;

    fn order(id: u64) -> PlaceOnlySpotOrderV3OrderCmd {
        PlaceOnlySpotOrderV3OrderCmd {
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            owner: UserId(7),
            taker_max_fee: Decimal::new(25, 2),
            order_id: id,
            is_buy: true,
            price: "100.25".into(),
            size: "1.5".into(),
            order_type: PlaceOnlySpotOrderV3OrderType::None,
            reduce_only: false,
            tif: OrderTif::Alo,
            cloid: Some("client-1".into()),
        }
    }

    fn child(id: u64) -> PlaceOnlySpotOrderV3OrderCmd {
        PlaceOnlySpotOrderV3OrderCmd {
            is_buy: false,
            reduce_only: true,
            order_type: PlaceOnlySpotOrderV3OrderType::Trigger {
                is_market: true,
                trigger_price: "110.5".into(),
                trigger_role: CommonOrderTrigger::TakeProfit,
            },
            ..order(id)
        }
    }

    fn log(payload: PlaceOnlySpotOrderV3Cmd) -> LogEntry<PlaceOnlySpotOrderV3Cmd> {
        LogEntrySequencer::from_metadata(42, 123, 1, 1, payload)
    }

    fn assert_replay(record: &OrderBase, event: &EntityReplayableEvent) {
        assert_eq!(
            (event.entity_id, event.entity_type, event.lsn, event.logical_ts),
            (record.order_id, 16, 42, 123)
        );
        assert_eq!(event.change_type, 0);
        let fields: serde_json::Map<String, serde_json::Value> = event
            .field_changes
            .iter()
            .map(|field| {
                assert!(field.old_value_bytes().is_empty());
                (
                    field.field_name_as_str().unwrap().to_owned(),
                    serde_json::from_slice(field.new_value_bytes()).unwrap(),
                )
            })
            .collect();
        let replayed: OrderBase =
            serde_json::from_value(serde_json::Value::Object(fields)).unwrap();
        assert_eq!(&replayed, record);
    }

    #[test]
    fn single_preserves_facts_and_projects_complete_create_event() {
        let cmd = log(PlaceOnlySpotOrderV3Cmd::Single(order(1)));
        let changes = PlaceOnlySpotOrderV3UseCase.compute_state_diff(&cmd, ()).unwrap();
        let PlaceOnlySpotOrderV3Changes::Single { created_order } = &changes else {
            panic!("single expected")
        };
        let record = created_order.order_base();
        assert_eq!(record.owner, UserId(7));
        assert_eq!(record.symbol, order(1).symbol);
        assert_eq!(record.limit_px, Decimal::new(10025, 2));
        assert_eq!((record.size, record.orig_size), (Decimal::new(15, 1), Decimal::new(15, 1)));
        assert_eq!(record.taker_max_fee, Decimal::new(25, 2));
        assert_eq!(record.tif, OrderTif::Alo);
        assert_eq!(record.status, SpotOrderStatus::Pending);
        assert_eq!(record.group_relation, OrderGroupRelation::Standalone);
        assert_eq!((record.created_at, record.updated_at, record.version), (123, 123, 42));
        assert!(record.has_consistent_execution_facts());
        assert_replay(record, &changes.to_replayable_events().unwrap()[0]);
    }

    #[test]
    fn normal_tpsl_preserves_order_and_projects_parent_then_children() {
        let tp = child(2);
        let mut sl = child(3);
        sl.order_type = PlaceOnlySpotOrderV3OrderType::Trigger {
            is_market: false,
            trigger_price: "90.5".into(),
            trigger_role: CommonOrderTrigger::StopLoss,
        };
        let cmd =
            log(PlaceOnlySpotOrderV3Cmd::NormalTpsl { parent: order(1), children: vec![tp, sl] });
        let changes = PlaceOnlySpotOrderV3UseCase.compute_state_diff(&cmd, ()).unwrap();
        let PlaceOnlySpotOrderV3Changes::NormalTpsl { created_parent_order, created_child_orders } =
            &changes
        else {
            panic!("group expected")
        };
        assert_eq!(
            created_parent_order.order_base().group_relation,
            OrderGroupRelation::NormalTpslParent
        );
        assert_eq!(created_child_orders.len(), 2);
        for (index, child) in created_child_orders.iter().enumerate() {
            let record = child.order_base();
            assert_eq!(record.order_id, index as u64 + 2);
            assert_eq!(
                record.group_relation,
                OrderGroupRelation::NormalTpslChild { parent_order_id: 1 }
            );
            assert_eq!(record.status, SpotOrderStatus::Pending);
            assert!(record.has_consistent_execution_facts());
        }
        assert_eq!(created_child_orders[0].order_base().execution, CommonOrderExecution::Market);
        assert_eq!(created_child_orders[0].order_base().tif, OrderTif::Ioc);
        assert_eq!(created_child_orders[1].order_base().execution, CommonOrderExecution::Limit);
        assert_eq!(created_child_orders[1].order_base().trigger, CommonOrderTrigger::StopLoss);
        let events = changes.to_replayable_events().unwrap();
        assert_eq!(events.len(), 3);
        for (record, event) in
            std::iter::once(created_parent_order).chain(created_child_orders).zip(&events)
        {
            assert_replay(record.order_base(), event);
        }
    }

    #[rstest]
    #[case("abc")]
    #[case("0")]
    #[case("-1")]
    #[case("0.00000000000000000000000000001")]
    fn rejects_invalid_decimal_fields(#[case] value: &str) {
        for field in ["price", "size", "trigger"] {
            let mut order = child(2);
            let expected = match field {
                "price" => {
                    order.price = value.into();
                    PlaceOnlySpotOrderV3Error::InvalidPrice
                }
                "size" => {
                    order.size = value.into();
                    PlaceOnlySpotOrderV3Error::InvalidSize
                }
                _ => {
                    order.order_type = PlaceOnlySpotOrderV3OrderType::Trigger {
                        is_market: true,
                        trigger_price: value.into(),
                        trigger_role: CommonOrderTrigger::TakeProfit,
                    };
                    PlaceOnlySpotOrderV3Error::InvalidTriggerPrice
                }
            };
            assert_eq!(
                PlaceOnlySpotOrderV3UseCase
                    .check_command(&log(PlaceOnlySpotOrderV3Cmd::Single(order))),
                Err(expected)
            );
        }
    }

    #[test]
    fn rejects_invalid_id_fee_and_trigger_role() {
        let mut invalid_fee = order(1);
        invalid_fee.taker_max_fee = Decimal::NEGATIVE_ONE;
        let mut invalid_role = child(2);
        invalid_role.order_type = PlaceOnlySpotOrderV3OrderType::Trigger {
            is_market: false,
            trigger_price: "1".into(),
            trigger_role: CommonOrderTrigger::None,
        };
        for (order, error) in [
            (order(0), PlaceOnlySpotOrderV3Error::InvalidOrderId),
            (invalid_fee, PlaceOnlySpotOrderV3Error::InvalidFee),
            (invalid_role, PlaceOnlySpotOrderV3Error::InvalidTriggerRole),
        ] {
            assert_eq!(
                PlaceOnlySpotOrderV3UseCase
                    .compute_state_changed(&log(PlaceOnlySpotOrderV3Cmd::Single(order)), &()),
                Err(error)
            );
        }
    }

    #[rstest]
    #[case("empty", PlaceOnlySpotOrderV3Error::ChildrenRequired)]
    #[case("parent_trigger", PlaceOnlySpotOrderV3Error::ParentMustBeLimit)]
    #[case("parent_reduce", PlaceOnlySpotOrderV3Error::ParentMustNotBeReduceOnly)]
    #[case("child_limit", PlaceOnlySpotOrderV3Error::ChildMustBeTrigger)]
    #[case("child_reduce", PlaceOnlySpotOrderV3Error::ChildMustBeReduceOnly)]
    #[case("side", PlaceOnlySpotOrderV3Error::ChildSideMustOpposeParent)]
    #[case("owner", PlaceOnlySpotOrderV3Error::ChildAccountMismatch)]
    #[case("asset", PlaceOnlySpotOrderV3Error::ChildAssetMismatch)]
    #[case("symbol", PlaceOnlySpotOrderV3Error::ChildSymbolMismatch)]
    #[case("size", PlaceOnlySpotOrderV3Error::ChildQuantityExceedsParent)]
    #[case("parent_id", PlaceOnlySpotOrderV3Error::DuplicateOrderId)]
    #[case("child_id", PlaceOnlySpotOrderV3Error::DuplicateOrderId)]
    fn rejects_invalid_groups(#[case] scenario: &str, #[case] expected: PlaceOnlySpotOrderV3Error) {
        let mut parent = order(1);
        let mut children = vec![child(2), child(3)];
        match scenario {
            "empty" => children.clear(),
            "parent_trigger" => parent.order_type = child(2).order_type,
            "parent_reduce" => parent.reduce_only = true,
            "child_limit" => children[0].order_type = PlaceOnlySpotOrderV3OrderType::None,
            "child_reduce" => children[0].reduce_only = false,
            "side" => children[0].is_buy = true,
            "owner" => children[0].owner = UserId(8),
            "asset" => children[0].symbol.base = Coin::ETH,
            "symbol" => children[0].symbol = Symbol::new(2, Coin::BTC, Coin::USDT),
            "size" => children[0].size = "1.51".into(),
            "parent_id" => children[0].order_id = 1,
            "child_id" => children[1].order_id = 2,
            _ => unreachable!(),
        }
        let cmd = log(PlaceOnlySpotOrderV3Cmd::NormalTpsl { parent, children });
        assert_eq!(PlaceOnlySpotOrderV3UseCase.compute_state_changed(&cmd, &()), Err(expected));
    }

    #[test]
    fn projection_rejects_oversized_fields_without_truncation() {
        let mut order = order(1);
        order.cloid = Some("x".repeat(65));
        let changes = PlaceOnlySpotOrderV3UseCase
            .compute_state_diff(&log(PlaceOnlySpotOrderV3Cmd::Single(order)), ())
            .unwrap();
        assert!(
            matches!(changes.to_replayable_events(), Err(EntityError::FieldParseError { field, .. }) if field == "cloid")
        );
    }
}
