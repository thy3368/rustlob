use std::collections::HashSet;

use common_entity::fsm_v2::{CommandMetadata, Fsm as FsmV2, SequencedCommand};
use common_entity::{
    CreatedEntityEvent, EntityError, EventPosition, PositionedEntity, ReplayableDomainEvent,
    ReplayableEntityChanges, Shard, TrackDiff, UpdatedEntityEvents,
};
use example_core_entity::entity::common::common_trade::TradeBase;
use example_core_entity::spot::{SpotTradeSettlementStatus, SpotTradeV3};
use example_core_entity::{OrderSide, OrderTif, SpotOrderStatus, SpotOrderV7, SpotOrderV7Command};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderInput {
    pub lsn: u64,
    pub logical_ts: u64,
    pub topic: u8,
    pub partition: u8,
    pub trade_id_start: u64,
    pub maker_fee_bps: u64,
    pub taker_fee_bps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchSpotOrderChanges {
    Resting,
    PartiallyFilled {
        taker: UpdatedEntityEvents<SpotOrderV7>,
        makers: Vec<UpdatedEntityEvents<SpotOrderV7>>,
        trades: Vec<CreatedEntityEvent<SpotTradeV3>>,
    },
    Filled {
        taker: UpdatedEntityEvents<SpotOrderV7>,
        makers: Vec<UpdatedEntityEvents<SpotOrderV7>>,
        trades: Vec<CreatedEntityEvent<SpotTradeV3>>,
    },
    CanceledAfterPartialFill {
        taker: UpdatedEntityEvents<SpotOrderV7>,
        makers: Vec<UpdatedEntityEvents<SpotOrderV7>>,
        trades: Vec<CreatedEntityEvent<SpotTradeV3>>,
    },
    Rejected {
        taker: UpdatedEntityEvents<SpotOrderV7>,
    },
}

impl ReplayableEntityChanges for MatchSpotOrderChanges {
    fn replayable_events(&self) -> Vec<&ReplayableDomainEvent> {
        let (taker, makers, trades) = match self {
            Self::Resting => return Vec::new(),
            Self::PartiallyFilled {
                taker,
                makers,
                trades,
            }
            | Self::Filled {
                taker,
                makers,
                trades,
                ..
            }
            | Self::CanceledAfterPartialFill {
                taker,
                makers,
                trades,
                ..
            } => (taker, makers.as_slice(), trades.as_slice()),
            Self::Rejected { taker, .. } => return taker.replayable_events(),
        };
        let mut events = Vec::with_capacity(1 + makers.len() + trades.len());
        events.extend(taker.replayable_events());
        for maker in makers {
            events.extend(maker.replayable_events());
        }
        for trade in trades {
            events.extend(trade.replayable_events());
        }
        events
    }
}

#[derive(Clone, Copy)]
struct PlannedFill {
    maker_index: usize,
    size: Decimal,
    quote_notional: Decimal,
    taker_fee: Decimal,
    maker_fee: Decimal,
    trade_id: u64,
}

/// 从只读订单快照生成一次现货撮合的实体变化与可重放事件。
pub fn match_spot_order(
    taker_before: &SpotOrderV7,
    makers: &[SpotOrderV7],
    input: MatchSpotOrderInput,
) -> Result<MatchSpotOrderChanges, EntityError> {
    validate_orders(taker_before, makers)?;
    let taker_facts = taker_before.order_base();
    let crosses = |maker: &SpotOrderV7| match taker_facts.side {
        OrderSide::Buy => taker_facts.limit_px >= maker.order_base().limit_px,
        OrderSide::Sell => taker_facts.limit_px <= maker.order_base().limit_px,
    };

    if taker_facts.effective_tif() == OrderTif::Alo && makers.iter().any(crosses) {
        let taker = transition_order(
            taker_before,
            SpotOrderV7Command::RejectedAsTaker,
            EventPosition::new(input.lsn, 0),
            input,
        )?;
        return Ok(MatchSpotOrderChanges::Rejected { taker });
    }

    let mut remaining = taker_facts.size;
    let mut taker_fee_total = Decimal::ZERO;
    let mut quote_notional_total = Decimal::ZERO;
    let mut fills = Vec::new();
    for (maker_index, maker) in makers.iter().enumerate() {
        if remaining == Decimal::ZERO {
            break;
        }
        if !crosses(maker) {
            continue;
        }
        let size = remaining.min(maker.order_base().size);
        let quote_notional = maker
            .order_base()
            .limit_px
            .checked_mul(size)
            .ok_or_else(|| match_error("notional overflow"))?;
        let taker_fee = match_fee(
            quote_notional,
            input.taker_fee_bps,
            taker_facts.symbol.quote.scale(),
        )?;
        let maker_fee = match_fee(
            quote_notional,
            input.maker_fee_bps,
            taker_facts.symbol.quote.scale(),
        )?;
        taker_fee_total = taker_fee_total
            .checked_add(taker_fee)
            .ok_or_else(|| match_error("fee overflow"))?;
        quote_notional_total = quote_notional_total
            .checked_add(quote_notional)
            .ok_or_else(|| match_error("notional total overflow"))?;
        let offset = u64::try_from(fills.len()).map_err(|_| match_error("trade id overflow"))?;
        let trade_id = input
            .trade_id_start
            .checked_add(offset)
            .ok_or_else(|| match_error("trade id overflow"))?;
        remaining = remaining
            .checked_sub(size)
            .ok_or_else(|| match_error("size underflow"))?;
        fills.push(PlannedFill {
            maker_index,
            size,
            quote_notional,
            taker_fee,
            maker_fee,
            trade_id,
        });
    }

    let filled_size = taker_facts
        .size
        .checked_sub(remaining)
        .ok_or_else(|| match_error("size underflow"))?;
    if filled_size == Decimal::ZERO && taker_facts.effective_tif() != OrderTif::Ioc {
        return Ok(MatchSpotOrderChanges::Resting);
    }

    let taker = transition_order(
        taker_before,
        SpotOrderV7Command::MatchedAsTaker {
            filled_size,
            quote_notional: quote_notional_total,
            quote_fee: taker_fee_total,
        },
        EventPosition::new(input.lsn, 0),
        input,
    )?;

    let mut maker_changes = Vec::with_capacity(fills.len());
    for (fill_index, fill) in fills.iter().enumerate() {
        let maker = makers
            .get(fill.maker_index)
            .ok_or_else(|| match_error("planned maker index is missing"))?;
        let command_index =
            u32::try_from(fill_index + 1).map_err(|_| match_error("command index overflow"))?;
        maker_changes.push(transition_order(
            maker,
            SpotOrderV7Command::MatchedAsMaker {
                filled_size: fill.size,
                quote_notional: fill.quote_notional,
                quote_fee: fill.maker_fee,
            },
            EventPosition::new(input.lsn, command_index),
            input,
        )?);
    }

    let trade_index_start = 1usize
        .checked_add(maker_changes.len())
        .ok_or_else(|| match_error("command index overflow"))?;
    let mut trades = Vec::with_capacity(fills.len());
    for (fill_index, fill) in fills.iter().enumerate() {
        let maker = makers
            .get(fill.maker_index)
            .ok_or_else(|| match_error("planned maker index is missing"))?;
        let command_index = u32::try_from(
            trade_index_start
                .checked_add(fill_index)
                .ok_or_else(|| match_error("command index overflow"))?,
        )
        .map_err(|_| match_error("command index overflow"))?;
        trades.push(create_trade(
            &taker,
            maker,
            *fill,
            EventPosition::new(input.lsn, command_index),
            input,
        )?);
    }

    match taker.after().status() {
        SpotOrderStatus::Filled => Ok(MatchSpotOrderChanges::Filled {
            taker,
            makers: maker_changes,
            trades,
        }),
        SpotOrderStatus::Canceled => Ok(MatchSpotOrderChanges::CanceledAfterPartialFill {
            taker,
            makers: maker_changes,
            trades,
        }),
        SpotOrderStatus::Rejected => Ok(MatchSpotOrderChanges::Rejected { taker }),
        SpotOrderStatus::PartiallyFilled => Ok(MatchSpotOrderChanges::PartiallyFilled {
            taker,
            makers: maker_changes,
            trades,
        }),
        _ => Err(match_error("unexpected taker state after matching")),
    }
}

fn validate_orders(taker: &SpotOrderV7, makers: &[SpotOrderV7]) -> Result<(), EntityError> {
    taker.validate_matchable_as_taker()?;
    let facts = taker.order_base();
    let mut ids = HashSet::with_capacity(makers.len() + 1);
    ids.insert(facts.order_id);
    for maker in makers {
        maker.validate_matchable_as_maker()?;
        let maker_facts = maker.order_base();
        if !ids.insert(maker_facts.order_id) {
            return Err(match_error("order ids must be unique"));
        }
        if maker_facts.symbol != facts.symbol || maker_facts.side == facts.side {
            return Err(match_error("maker market or side mismatch"));
        }
    }
    Ok(())
}

fn transition_order(
    before: &SpotOrderV7,
    command: SpotOrderV7Command,
    position: EventPosition,
    input: MatchSpotOrderInput,
) -> Result<UpdatedEntityEvents<SpotOrderV7>, EntityError> {
    let metadata = CommandMetadata::new_at(
        position,
        input.logical_ts,
        input.topic,
        input.partition,
        before.party_id(),
    );
    let (after, mut event) = FsmV2::transition(before, SequencedCommand::new(metadata, command))?;
    event.t = input.topic;
    event.p = input.partition;
    UpdatedEntityEvents::try_new(after, before.event_position(), vec![event])
        .map_err(|error| match_error(&format!("invalid order event chain: {error:?}")))
}

fn create_trade(
    taker: &UpdatedEntityEvents<SpotOrderV7>,
    maker: &SpotOrderV7,
    fill: PlannedFill,
    position: EventPosition,
    input: MatchSpotOrderInput,
) -> Result<CreatedEntityEvent<SpotTradeV3>, EntityError> {
    let taker_facts = taker.after().order_base();
    let trade = SpotTradeV3 {
        comon_trade: TradeBase {
            trade_id: fill.trade_id,
            symbol: taker_facts.symbol,
            taker_user: taker_facts.owner,
            maker_user: maker.order_base().owner,
            taker_order_id: taker_facts.order_id,
            maker_order_id: maker.order_base().order_id,
            taker_side: taker_facts.side,
            price: maker.order_base().limit_px,
            size: fill.size,
            taker_fee: fill.taker_fee,
            maker_fee: fill.maker_fee,
            created_at: input.logical_ts,
            updated_at: input.logical_ts,
            version: input.lsn,
        },
        settlement_status: SpotTradeSettlementStatus::Unsettled,
        position,
    };
    let mut event = trade.track_create_event_at(position, input.logical_ts)?;
    event.t = input.topic;
    event.p = input.partition;
    event.domain_event_type = 1;
    CreatedEntityEvent::try_new(trade, event)
        .map_err(|error| match_error(&format!("invalid trade event: {error:?}")))
}

fn match_fee(notional: Decimal, bps: u64, scale: u32) -> Result<Decimal, EntityError> {
    notional
        .checked_mul(Decimal::from(bps))
        .and_then(|value| value.checked_div(Decimal::from(10_000u64)))
        .map(|value| {
            value.round_dp_with_strategy(scale, rust_decimal::RoundingStrategy::ToPositiveInfinity)
        })
        .ok_or_else(|| match_error("fee overflow"))
}

fn match_error(message: &str) -> EntityError {
    EntityError::Custom(message.to_owned())
}

#[cfg(test)]
mod tests {
    use common_entity::{Entity, ReplayableEntityChanges};
    use example_core_entity::account::coin::Coin;
    use example_core_entity::account::symbol::Symbol;
    use example_core_entity::common::PriceType;
    use example_core_entity::entity::common::UserId;
    use example_core_entity::entity::common::common_order::OrderBase;

    use super::*;
    use crate::use_case::trading::spot::v3::pipeline::atom_spot_order_domain_service::{
        MatchSpotOrderV3Input, MatchSpotOrderV3Outcome, SpotOrderDomainService,
    };

    const LSN: u64 = 50;
    const LOGICAL_TS: u64 = 900;

    fn input() -> MatchSpotOrderInput {
        MatchSpotOrderInput {
            lsn: LSN,
            logical_ts: LOGICAL_TS,
            topic: 7,
            partition: 3,
            trade_id_start: 100,
            maker_fee_bps: 5,
            taker_fee_bps: 10,
        }
    }

    fn order(
        id: u64,
        side: OrderSide,
        price: Decimal,
        size: Decimal,
        tif: OrderTif,
    ) -> SpotOrderV7 {
        let principal = price.checked_mul(size).unwrap_or(Decimal::ZERO);
        SpotOrderV7::from_record_at(
            OrderBase {
                owner: UserId(id + 1_000),
                order_id: id,
                symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
                side,
                limit_px: price,
                size,
                orig_size: size,
                execution: PriceType::Limit,
                tif,
                status: SpotOrderStatus::Open,
                buy_quote_principal_frozen: if side == OrderSide::Buy {
                    principal
                } else {
                    Decimal::ZERO
                },
                buy_quote_fee_reserve_frozen: if side == OrderSide::Buy {
                    Decimal::from(10)
                } else {
                    Decimal::ZERO
                },
                lsn: 10,
                ..OrderBase::default()
            },
            EventPosition::new(10, id as u32),
        )
        .expect("测试订单必须有效")
    }

    #[test]
    fn gtc_without_crossing_liquidity_rests_without_events() {
        // Rule: 未穿价 GTC 必须保持静止，不能制造 update event 或 after 快照。
        // Given: 买价低于唯一卖方 maker。
        let taker = order(
            1,
            OrderSide::Buy,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        );
        let makers = vec![order(
            2,
            OrderSide::Sell,
            Decimal::from(100),
            Decimal::ONE,
            OrderTif::Gtc,
        )];

        // When: 执行独立撮合服务。
        let changes = match_spot_order(&taker, &makers, input()).expect("GTC 应进入挂单结果");

        // Then: 结果明确为 Resting 且事件集合为空，输入未变化。
        assert_eq!(changes, MatchSpotOrderChanges::Resting);
        assert!(changes.replayable_events().is_empty());
        assert_eq!(taker.order_base().size, Decimal::ONE);
        assert_eq!(makers[0].order_base().size, Decimal::ONE);
    }

    #[test]
    fn gtc_matches_makers_in_input_order_and_emits_contiguous_events() {
        // Rule: maker 输入顺序决定成交顺序，事件按 taker、makers、trades 连续定序。
        // Given: 两个 maker 依次提供 1 和 2 个 base，手续费需要向上取整。
        let taker = order(
            1,
            OrderSide::Buy,
            Decimal::from(110),
            Decimal::from(3),
            OrderTif::Gtc,
        );
        let makers = vec![
            order(
                2,
                OrderSide::Sell,
                Decimal::from(100),
                Decimal::ONE,
                OrderTif::Gtc,
            ),
            order(
                3,
                OrderSide::Sell,
                Decimal::from(101),
                Decimal::from(2),
                OrderTif::Gtc,
            ),
        ];

        // When: taker 完全吃完两笔 maker。
        let changes = match_spot_order(&taker, &makers, input()).expect("撮合应成功");

        // Then: 业务事实和成交价格采用 maker 价格。
        let MatchSpotOrderChanges::Filled {
            taker: taker_changes,
            makers,
            trades,
        } = &changes
        else {
            panic!("taker 应完全成交")
        };
        assert_eq!(taker_changes.after().status(), SpotOrderStatus::Filled);
        assert_eq!(makers.len(), 2);
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].after().comon_trade.maker_order_id, 2);
        assert_eq!(trades[1].after().comon_trade.maker_order_id, 3);
        assert_eq!(trades[0].after().comon_trade.trade_id, 100);
        assert_eq!(trades[1].after().comon_trade.trade_id, 101);
        assert_eq!(trades[0].after().comon_trade.price, Decimal::from(100));
        assert_eq!(trades[1].after().comon_trade.price, Decimal::from(101));
        // Then: 所有事件共享调用方元数据，位置从 0 到 4 连续且 update 保留加载位置。
        let events = changes.replayable_events();
        assert_eq!(events.len(), 5);
        assert_eq!(
            events
                .iter()
                .map(|event| event.position)
                .collect::<Vec<_>>(),
            (0..5)
                .map(|index| EventPosition::new(LSN, index))
                .collect::<Vec<_>>()
        );
        assert_eq!(events[0].previous_position, EventPosition::new(10, 1));
        assert_eq!(events[1].previous_position, EventPosition::new(10, 2));
        assert_eq!(events[2].previous_position, EventPosition::new(10, 3));
        for event in events {
            assert_eq!((event.logical_ts, event.t, event.p), (LOGICAL_TS, 7, 3));
        }
    }

    #[test]
    fn alo_crossing_liquidity_is_rejected_before_any_trade() {
        // Rule: ALO 一旦会吃单就拒绝，只更新 taker。
        // Given: ALO 买单穿过卖方 maker。
        let taker = order(
            1,
            OrderSide::Buy,
            Decimal::from(100),
            Decimal::ONE,
            OrderTif::Alo,
        );
        let makers = vec![order(
            2,
            OrderSide::Sell,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];

        // When: 执行撮合。
        let changes = match_spot_order(&taker, &makers, input()).expect("ALO 应业务拒绝");

        // Then: taker 进入 Rejected 且只有 index 0 事件。
        let MatchSpotOrderChanges::Rejected {
            taker: taker_changes,
        } = &changes
        else {
            panic!("ALO 应返回拒绝")
        };
        assert_eq!(taker_changes.after().status(), SpotOrderStatus::Rejected);
        assert_eq!(
            changes.replayable_events()[0].position,
            EventPosition::new(LSN, 0)
        );
        assert_eq!(makers[0].order_base().size, Decimal::ONE);
    }

    #[test]
    fn ioc_without_fill_is_rejected_and_partial_fill_is_canceled() {
        // Rule: IOC 不得挂单；零成交拒绝，部分成交后取消剩余量。
        // Given: 一个无流动性 IOC 和一个只有部分流动性的 IOC。
        let no_fill = order(
            1,
            OrderSide::Buy,
            Decimal::from(100),
            Decimal::from(2),
            OrderTif::Ioc,
        );
        let partial = no_fill.clone();
        let makers = vec![order(
            2,
            OrderSide::Sell,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];

        // When: 分别撮合。
        let rejected = match_spot_order(&no_fill, &[], input()).expect("空簿 IOC 应拒绝");
        let canceled = match_spot_order(&partial, &makers, input()).expect("IOC 应部分成交后取消");

        // Then: 两个终态明确，部分成交 IOC 仍创建一笔成交。
        let MatchSpotOrderChanges::Rejected { taker, .. } = rejected else {
            panic!("空簿 IOC 应返回拒绝")
        };
        assert_eq!(taker.after().status(), SpotOrderStatus::Rejected);
        let MatchSpotOrderChanges::CanceledAfterPartialFill { taker, trades, .. } = canceled else {
            panic!("部分成交 IOC 应取消")
        };
        assert_eq!(taker.after().status(), SpotOrderStatus::Canceled);
        assert_eq!(taker.after().order_base().size, Decimal::ONE);
        assert_eq!(trades.len(), 1);
    }

    #[test]
    fn terminal_release_precision_does_not_reject_matching() {
        // Rule: 撮合层只生成订单与成交事件，不承担解冻金额的币种精度校验。
        // Given: 卖方 ALO 的原始数量超过 BTC 支持的精度，且会穿过买方 maker。
        let taker = order(
            1,
            OrderSide::Sell,
            Decimal::new(1, 8),
            Decimal::new(1, 9),
            OrderTif::Alo,
        );
        let makers = vec![order(
            2,
            OrderSide::Buy,
            Decimal::new(1, 8),
            Decimal::new(1, 9),
            OrderTif::Gtc,
        )];

        // When: ALO 拒绝转换成功，即使后续资金层无法表示其解冻金额。
        let changes = match_spot_order(&taker, &makers, input()).expect("撮合层不应计算解冻金额");

        // Then: 撮合仍返回业务拒绝及对应订单事件。
        let MatchSpotOrderChanges::Rejected { taker } = changes else {
            panic!("ALO 应返回拒绝")
        };
        assert_eq!(taker.after().status(), SpotOrderStatus::Rejected);
    }

    #[test]
    fn gtc_with_insufficient_liquidity_remains_partially_filled() {
        // Rule: GTC 有成交但流动性不足时保留剩余数量继续挂单。
        // Given: taker 数量为 2，唯一 maker 只能成交 1。
        let taker = order(
            1,
            OrderSide::Buy,
            Decimal::from(100),
            Decimal::from(2),
            OrderTif::Gtc,
        );
        let makers = vec![order(
            2,
            OrderSide::Sell,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];

        // When: 执行撮合。
        let changes = match_spot_order(&taker, &makers, input()).expect("GTC 应部分成交");

        // Then: taker 保留一单位，maker 填满并创建一笔未结算成交。
        let MatchSpotOrderChanges::PartiallyFilled {
            taker,
            makers,
            trades,
        } = changes
        else {
            panic!("GTC 应返回部分成交")
        };
        assert_eq!(taker.after().status(), SpotOrderStatus::PartiallyFilled);
        assert_eq!(taker.after().order_base().size, Decimal::ONE);
        assert_eq!(makers[0].after().status(), SpotOrderStatus::Filled);
        assert_eq!(
            trades[0].after().settlement_status,
            SpotTradeSettlementStatus::Unsettled
        );
    }

    #[test]
    fn sell_taker_crosses_buy_maker_at_equal_limit_price() {
        // Rule: 卖方 taker 的限价等于 maker 买价时应成交。
        // Given: 一笔卖单和一笔同价同量买单。
        let taker = order(
            1,
            OrderSide::Sell,
            Decimal::from(100),
            Decimal::ONE,
            OrderTif::Gtc,
        );
        let makers = vec![order(
            2,
            OrderSide::Buy,
            Decimal::from(100),
            Decimal::ONE,
            OrderTif::Gtc,
        )];

        // When: 执行撮合。
        let changes = match_spot_order(&taker, &makers, input()).expect("同价卖单应成交");

        // Then: taker 完全成交且成交采用 maker 买价。
        let MatchSpotOrderChanges::Filled { trades, .. } = changes else {
            panic!("同价卖方 taker 应完全成交")
        };
        assert_eq!(trades[0].after().comon_trade.price, Decimal::from(100));
        assert_eq!(trades[0].after().comon_trade.taker_side, OrderSide::Sell);
    }

    #[test]
    fn invalid_or_overflowing_inputs_fail_without_mutating_orders() {
        // Rule: 规划阶段的身份与算术错误必须原子失败。
        // Given: 重复 ID、trade ID 溢出和名义金额溢出三种输入。
        let taker = order(
            1,
            OrderSide::Buy,
            Decimal::from(100),
            Decimal::from(2),
            OrderTif::Gtc,
        );
        let duplicate = vec![order(
            1,
            OrderSide::Sell,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];
        let maker = vec![order(
            2,
            OrderSide::Sell,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];
        let taker_before = taker.clone();
        let duplicate_before = duplicate.clone();

        // When/Then: 各错误均返回 Err，引用输入保持原样。
        assert!(match_spot_order(&taker, &duplicate, input()).is_err());
        let same_side = vec![order(
            2,
            OrderSide::Buy,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];
        assert!(match_spot_order(&taker, &same_side, input()).is_err());
        let mut overflow_id = input();
        overflow_id.trade_id_start = u64::MAX;
        let two_makers = vec![
            maker[0].clone(),
            order(
                3,
                OrderSide::Sell,
                Decimal::from(99),
                Decimal::ONE,
                OrderTif::Gtc,
            ),
        ];
        assert!(match_spot_order(&taker, &two_makers, overflow_id).is_err());

        let huge_taker = order(
            10,
            OrderSide::Buy,
            Decimal::MAX,
            Decimal::from(2),
            OrderTif::Gtc,
        );
        let huge_maker = vec![order(
            11,
            OrderSide::Sell,
            Decimal::MAX,
            Decimal::from(2),
            OrderTif::Gtc,
        )];
        assert!(match_spot_order(&huge_taker, &huge_maker, input()).is_err());
        assert_eq!(taker, taker_before);
        assert_eq!(duplicate, duplicate_before);
    }

    #[test]
    fn filled_result_matches_legacy_match_order_business_facts() {
        // Rule: 独立服务当前必须与保留的旧撮合算法产生等价业务事实。
        // Given: 单 maker 完全成交。
        let taker = order(
            1,
            OrderSide::Buy,
            Decimal::from(100),
            Decimal::ONE,
            OrderTif::Gtc,
        );
        let makers = vec![order(
            2,
            OrderSide::Sell,
            Decimal::from(99),
            Decimal::ONE,
            OrderTif::Gtc,
        )];
        let new_input = input();

        // When: 新旧实现处理相同订单和费用参数。
        let new = match_spot_order(&taker, &makers, new_input).expect("新服务应成功");
        let old = SpotOrderDomainService::match_order(
            taker,
            makers,
            MatchSpotOrderV3Input {
                lsn: new_input.lsn,
                trade_id_start: new_input.trade_id_start,
                maker_fee_bps: new_input.maker_fee_bps,
                taker_fee_bps: new_input.taker_fee_bps,
                timestamp: new_input.logical_ts,
            },
        )
        .expect("旧服务应成功");

        // Then: 忽略事件包装后，订单、成交和释放金额一致。
        let MatchSpotOrderChanges::Filled {
            taker: taker_changes,
            makers,
            trades,
        } = new
        else {
            panic!("新服务应完全成交")
        };
        let MatchSpotOrderV3Outcome::Filled {
            filled_taker_order_after,
            maker_orders_after,
            created_trades,
            ..
        } = old
        else {
            panic!("旧服务应完全成交")
        };
        assert_eq!(
            taker_changes.after().order_base(),
            filled_taker_order_after.order_base()
        );
        assert_eq!(
            makers[0].after().order_base(),
            maker_orders_after[0].order_base()
        );
        let new_trade = &trades[0].after().comon_trade;
        let old_trade = &created_trades[0].comon_trade;
        assert_eq!(new_trade.trade_id, old_trade.trade_id);
        assert_eq!(new_trade.symbol, old_trade.symbol);
        assert_eq!(new_trade.taker_user, old_trade.taker_user);
        assert_eq!(new_trade.maker_user, old_trade.maker_user);
        assert_eq!(new_trade.taker_order_id, old_trade.taker_order_id);
        assert_eq!(new_trade.maker_order_id, old_trade.maker_order_id);
        assert_eq!(new_trade.taker_side, old_trade.taker_side);
        assert_eq!(new_trade.price, old_trade.price);
        assert_eq!(new_trade.size, old_trade.size);
        assert_eq!(new_trade.taker_fee, old_trade.taker_fee);
        assert_eq!(new_trade.maker_fee, old_trade.maker_fee);
        assert_eq!(new_trade.created_at, old_trade.created_at);
        assert_eq!(new_trade.updated_at, old_trade.updated_at);
        assert_eq!(
            trades[0].after().settlement_status,
            SpotTradeSettlementStatus::Unsettled
        );
        assert_eq!(trades[0].after().position, EventPosition::new(LSN, 2));
        assert_eq!(
            trades[0].event().entity_type,
            <SpotTradeV3 as Entity>::entity_type()
        );
    }
}
