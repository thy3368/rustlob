use std::marker::PhantomData;

use common_entity::{EntityError, EntityReplayableEvent, ReplayFieldChange};
use rust_decimal::Decimal;

use crate::account::coin::{CoinAmount, CoinAmountError};
use crate::entity::common::common_order::{
    CommonOrderExecution, OrderBase, SpotAmountConversionError,
};
use crate::entity::common::common_trade::TradeBase;
use crate::entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
use crate::entity::spot::spot_order_v2::OrderGroupRelation;
use crate::spot::spot_trade_v3::SpotTradeV3;

/// 已受理，等待进入执行流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pending;
/// 已进入执行流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Open;
/// 部分成交。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartiallyFilled;
/// 完全成交。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filled;
/// 已撤销。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Canceled;
/// 已拒绝。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rejected;

/// 撮合批次的时间、费用和稳定成交编号输入。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchSpotOrderV3Input {
    /// 本批第一笔成交的稳定 ID，后续成交按序递增。
    pub trade_id_start: u64,
    /// maker 费率，单位为万分比（bps）。
    pub maker_fee_bps: u64,
    /// taker 费率，单位为万分比（bps）。
    pub taker_fee_bps: u64,
    /// 成交及订单状态变更时间，单位为 Unix 纳秒。
    pub timestamp: u64,
}

/// 撮合后对 taker 的五种业务结论。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult {
    /// 没有成交，订单继续留在订单簿；本次没有其它业务副作用。
    Resting {
        //状态可能是多种
        taker_order_after: SpotOrderV3<Open>,
    },
    /// 发生成交但仍有剩余数量，订单继续可撮合。
    PartiallyFilled {
        taker_order_after: SpotOrderV3<PartiallyFilled>,
        maker_orders_after: Vec<SpotOrderV3>,
        created_trades: Vec<SpotTradeV3>,
    },
    /// 成交数量达到订单总数量，订单终止为 Filled。
    Filled {
        filled_taker_order_after: SpotOrderV3<Filled>,
        maker_orders_after: Vec<SpotOrderV3>,
        created_trades: Vec<SpotTradeV3>,
    },
    /// IOC 部分成交后，剩余数量被取消并释放冻结。
    CanceledAfterPartialFill {
        canceled_taker_order_after: SpotOrderV3<Canceled>,
        maker_orders_after: Vec<SpotOrderV3>,
        created_trades: Vec<SpotTradeV3>,
    },
    /// ALO crossing maker 或 IOC 无流动性导致拒单。
    Rejected { rejected_taker_order_after: SpotOrderV3<Rejected> },
}




/// 现货委托这一 Moment-Interval 业务事实及其生命周期类型。
///
/// `OrderBase` 保存稳定事实；私有包装保证工厂产物的类型与 Pending 状态一致。
/// 本模块暂只提供 Pending 创建入口，不提供状态转换或历史状态恢复入口。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpotOrderV3<S = Pending> {
    comon_order: OrderBase,
    stage: PhantomData<S>,
}

impl<S = Pending> SpotOrderV3<S> {
    /// 按 maker 优先级撮合 taker，并派生成交事实和订单 after-state。
    ///
    /// 本方法不修改输入实体。成交手续费按 quote 币精度向上取整；成交编号从
    /// `input.trade_id_start` 起逐笔递增。任何校验或 checked arithmetic 失败都会
    /// 返回错误，不会向调用方暴露部分撮合结果。
    pub fn match_with_input(
        &self,
        makers: Vec<SpotOrderV3>,
        input: MatchSpotOrderV3Input,
    ) -> Result<MatchResult, EntityError> {
        validate_taker(&self.comon_order)?;

        let mut taker_after = self.comon_order.clone();
        let tif = taker_after.effective_tif();
        let mut makers_after = makers;

        if tif == OrderTif::Alo {
            let crosses_best = if let Some(maker) = makers_after.first() {
                validate_maker(&taker_after, &maker.comon_order)?;
                crosses(&taker_after, &maker.comon_order)
            } else {
                false
            };
            if crosses_best {
                transition_status(&mut taker_after, SpotOrderStatus::Rejected, input.timestamp)?;
                return Ok(MatchResult::Rejected {
                    rejected_taker_order_after: with_stage(taker_after),
                });
            }
            rest_or_preserve_partial(taker_after, makers_after, Vec::new(), input.timestamp)
        } else {
            let mut trades = Vec::new();
            let mut next_trade_id = input.trade_id_start;

            for maker in &mut makers_after {
                validate_maker(&taker_after, &maker.comon_order)?;
                if !crosses(&taker_after, &maker.comon_order) {
                    break;
                }

                let taker_remaining = remaining(&taker_after)?;
                let maker_remaining = remaining(&maker.comon_order)?;
                let trade_size = taker_remaining.min(maker_remaining);
                if trade_size <= Decimal::ZERO {
                    return Err(entity_error("matched order has no remaining quantity"));
                }
                let price = maker.comon_order.limit_px;
                let notional = price
                    .checked_mul(trade_size)
                    .ok_or_else(|| entity_error("trade notional overflow"))?;
                let taker_fee = fee_amount(
                    notional,
                    input.taker_fee_bps,
                    taker_after.symbol.quote.scale(),
                )?;
                let maker_fee = fee_amount(
                    notional,
                    input.maker_fee_bps,
                    maker.comon_order.symbol.quote.scale(),
                )?;
                let next_taker = filled_order_after(&taker_after, trade_size, input.timestamp)?;
                let next_maker =
                    filled_order_after(&maker.comon_order, trade_size, input.timestamp)?;
                let next_trade_id = next_trade_id;
                let next_trade_id_after = next_trade_id
                    .checked_add(1)
                    .ok_or_else(|| entity_error("trade id overflow"))?;

                trades.push(SpotTradeV3 {
                    comon_trade: TradeBase {
                        trade_id: next_trade_id,
                        symbol: taker_after.symbol.clone(),
                        taker_user: taker_after.owner,
                        maker_user: maker.comon_order.owner,
                        taker_order_id: taker_after.order_id,
                        maker_order_id: maker.comon_order.order_id,
                        taker_side: taker_after.side,
                        price,
                        size: trade_size,
                        taker_fee,
                        maker_fee,
                        created_at: input.timestamp,
                        updated_at: input.timestamp,
                        version: 0,
                    },
                });
                taker_after = next_taker;
                maker.comon_order = next_maker;
                next_trade_id = next_trade_id_after;

                if remaining(&taker_after)? == Decimal::ZERO {
                    break;
                }
            }

            if remaining(&taker_after)? == Decimal::ZERO {
                return Ok(MatchResult::Filled {
                    filled_taker_order_after: with_stage(taker_after),
                    maker_orders_after: makers_after,
                    created_trades: trades,
                });
            }

            if tif == OrderTif::Ioc {
                if trades.is_empty() {
                    transition_status(&mut taker_after, SpotOrderStatus::Rejected, input.timestamp)?;
                    return Ok(MatchResult::Rejected {
                        rejected_taker_order_after: with_stage(taker_after),
                    });
                }
                transition_status(&mut taker_after, SpotOrderStatus::Canceled, input.timestamp)?;
                return Ok(MatchResult::CanceledAfterPartialFill {
                    canceled_taker_order_after: with_stage(taker_after),
                    maker_orders_after: makers_after,
                    created_trades: trades,
                });
            }

            if trades.is_empty() {
                rest_or_preserve_partial(taker_after, makers_after, trades, input.timestamp)
            } else {
                Ok(MatchResult::PartiallyFilled {
                    taker_order_after: with_stage(taker_after),
                    maker_orders_after: makers_after,
                    created_trades: trades,
                })
            }
        }
    }

    /// 兼容不带撮合上下文的调用方；新调用应使用 [`Self::match_with_input`]。
    ///
    /// 兼容路径采用零费率、订单更新时间和 taker 订单 ID 作为成交 ID 起点。
    pub fn matchOrder(&self, makers: Vec<SpotOrderV3>) -> Result<MatchResult, EntityError> {
        self.match_with_input(
            makers,
            MatchSpotOrderV3Input {
                trade_id_start: self.comon_order.order_id,
                maker_fee_bps: 0,
                taker_fee_bps: 0,
                timestamp: self.comon_order.updated_at,
            },
        )
    }
}

impl SpotOrderV3<Pending> {
    /// 将新建委托投影为完整字段创建事件，使用独立的 V3 实体类型 16。
    ///
    /// 字段值使用 OrderBase 的 JSON 表示，field_type 为 0；父子事件顺序由用例维护。
    /// 任一字段超过回放协议的 64 字节容量时返回错误，避免静默截断。
    pub fn track_create_event(&self) -> Result<EntityReplayableEvent, EntityError> {
        let record = &self.comon_order;
        let fields =
            serde_json::to_value(record).map_err(|error| EntityError::Custom(error.to_string()))?;
        let fields = fields
            .as_object()
            .ok_or_else(|| EntityError::Custom("order record must be an object".to_owned()))?;
        let mut event = EntityReplayableEvent::new(
            record.created_at,
            0,
            record.version,
            record.order_id,
            16,
            0,
        );
        for (name, value) in fields {
            let value = value.to_string();
            if value.len() > 64 {
                return Err(EntityError::FieldParseError {
                    field: name.clone(),
                    reason: "created field exceeds 64-byte replay capacity".to_owned(),
                });
            }
            event.field_changes.push(ReplayFieldChange::new(
                ReplayFieldChange::field_name_from_str(name),
                &[],
                value.as_bytes(),
                0,
            ));
        }
        Ok(event)
    }

    /// 仅装配已校验事实，规范化为独立 Pending 委托；其余字段原样保留。
    pub fn single(mut order: OrderBase) -> Self {
        order.status = SpotOrderStatus::Pending;
        order.group_relation = OrderGroupRelation::Standalone;
        Self { comon_order: order, stage: PhantomData }
    }

    /// 仅装配已校验的 normalTpsl 父子委托，保持子单顺序。
    ///
    /// 调用方须先校验子单非空及父子业务约束。本方法不重复 admission 校验；
    /// 即使传入空子单，也只返回 Pending 父单与空集合，不 panic。
    /// 除状态和关系外，所有订单事实保持不变。
    pub fn normal_tpsl(parent: OrderBase, children: Vec<OrderBase>) -> (Self, Vec<Self>) {
        let mut parent = Self::single(parent);
        parent.comon_order.group_relation = OrderGroupRelation::NormalTpslParent;
        let parent_order_id = parent.comon_order.order_id;
        let children = children
            .into_iter()
            .map(|child| {
                let mut child = Self::single(child);
                child.comon_order.group_relation =
                    OrderGroupRelation::NormalTpslChild { parent_order_id };
                child
            })
            .collect();
        (parent, children)
    }
}

impl<S> SpotOrderV3<S> {
    /// 委托的只读事实快照，供查询和持久化投影使用。
    pub fn order_base(&self) -> &OrderBase {
        &self.comon_order
    }

    /// 返回订单需要预冻结的单币种金额，不修改订单状态。
    ///
    /// 买单冻结 quote 币的原始数量限价金额及最大 taker 手续费；卖单冻结
    /// base 币原始数量，手续费从卖出所得中支付。
    pub fn freeze_requirement(&self) -> Result<CoinAmount, SpotAmountConversionError> {
        let order = &self.comon_order;
        let orig_size = order.orig_size;
        let fee = order.taker_max_fee;

        if orig_size <= Decimal::ZERO || fee < Decimal::ZERO {
            return Err(SpotAmountConversionError::InvalidAmount);
        }

        match order.side {
            OrderSide::Buy => {
                if order.limit_px <= Decimal::ZERO {
                    return Err(SpotAmountConversionError::InvalidAmount);
                }

                let amount = order
                    .limit_px
                    .checked_mul(orig_size)
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?
                    .checked_add(fee)
                    .ok_or(SpotAmountConversionError::ArithmeticOverflow)?;

                CoinAmount::new(order.symbol.quote, amount).map_err(map_coin_amount_error)
            }
            OrderSide::Sell => {
                CoinAmount::new(order.symbol.base, orig_size).map_err(map_coin_amount_error)
            }
        }
    }
}

fn map_coin_amount_error(_: CoinAmountError) -> SpotAmountConversionError {
    SpotAmountConversionError::InexactConversion
}

#[cfg(test)]
mod tests {
    use base_types::UserId;
    use rust_decimal::Decimal;

    use super::SpotOrderV3;
    use crate::account::coin::{Coin, CoinAmount};
    use crate::account::symbol::Symbol;
    use crate::entity::common::common_order::{
        CommonOrderExecution, CommonOrderTrigger, OrderBase, SpotAmountConversionError,
    };
    use crate::entity::spot::spot_order_primitives::{OrderSide, OrderTif, SpotOrderStatus};
    use crate::entity::spot::spot_order_v2::OrderGroupRelation;

    fn order(side: OrderSide, limit_px: Decimal, orig_size: Decimal, fee: Decimal) -> SpotOrderV3 {
        SpotOrderV3::single(OrderBase {
            owner: UserId(1),
            order_id: 1,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side,
            limit_px,
            size: Decimal::ZERO,
            orig_size,
            execution: CommonOrderExecution::Limit,
            tif: OrderTif::Gtc,
            trigger: CommonOrderTrigger::None,
            trigger_px: None,
            group_relation: OrderGroupRelation::Standalone,
            is_position_tpsl: false,
            reduce_only: false,
            status: SpotOrderStatus::Open,
            taker_max_fee: fee,
            cloid: None,
            created_at: 0,
            updated_at: 0,
            version: 0,
        })
    }

    #[test]
    fn factories_only_normalize_status_and_relationships() {
        let mut original =
            order(OrderSide::Buy, Decimal::ONE, Decimal::ONE, Decimal::ZERO).order_base().clone();
        original.status = SpotOrderStatus::Filled;
        original.group_relation = OrderGroupRelation::NormalTpslChild { parent_order_id: 99 };
        original.created_at = 123;
        original.updated_at = 456;
        original.version = 7;
        let single: SpotOrderV3<super::Pending> = SpotOrderV3::single(original.clone());
        let mut expected = original.clone();
        expected.status = SpotOrderStatus::Pending;
        expected.group_relation = OrderGroupRelation::Standalone;
        assert_eq!(single.order_base(), &expected);

        let mut first = original.clone();
        first.order_id = 2;
        let mut second = first.clone();
        second.order_id = 3;
        let (parent, children) =
            SpotOrderV3::normal_tpsl(original.clone(), vec![first.clone(), second.clone()]);
        expected.group_relation = OrderGroupRelation::NormalTpslParent;
        assert_eq!(parent.order_base(), &expected);
        for (child, mut expected) in children.iter().zip([first, second]) {
            expected.status = SpotOrderStatus::Pending;
            expected.group_relation = OrderGroupRelation::NormalTpslChild { parent_order_id: 1 };
            assert_eq!(child.order_base(), &expected);
        }
        let (parent, children) = SpotOrderV3::normal_tpsl(original, vec![]);
        assert_eq!(parent.order_base().status, SpotOrderStatus::Pending);
        assert!(children.is_empty());
    }

    #[test]
    fn lifecycle_types_share_read_only_queries() {
        fn query<S>(order: &SpotOrderV3<S>) {
            let _ = order.freeze_requirement();
        }
        let _ = query::<super::Pending>;
        let _ = query::<super::Open>;
        let _ = query::<super::PartiallyFilled>;
        let _ = query::<super::Filled>;
        let _ = query::<super::Canceled>;
        let _ = query::<super::Rejected>;
    }

    #[test]
    fn buy_freezes_original_quote_notional_plus_fee() {
        let order =
            order(OrderSide::Buy, Decimal::new(250, 2), Decimal::new(3, 0), Decimal::new(25, 2));

        assert_eq!(
            order.freeze_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(775, 2)).unwrap())
        );
    }

    #[test]
    fn sell_freezes_original_base_size_independent_of_quote_fee() {
        let order =
            order(OrderSide::Sell, Decimal::new(250, 2), Decimal::new(3, 0), Decimal::new(900, 0));

        assert_eq!(
            order.freeze_requirement(),
            Ok(CoinAmount::new(Coin::BTC, Decimal::new(3, 0)).unwrap())
        );
    }

    #[test]
    fn rejects_non_positive_size_negative_fee_and_non_positive_buy_price() {
        let invalid_size = order(OrderSide::Sell, Decimal::ONE, Decimal::ZERO, Decimal::ZERO);
        assert_eq!(
            invalid_size.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let negative_size =
            order(OrderSide::Buy, Decimal::ONE, Decimal::NEGATIVE_ONE, Decimal::ZERO);
        assert_eq!(
            negative_size.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        let negative_fee =
            order(OrderSide::Sell, Decimal::ONE, Decimal::ONE, Decimal::NEGATIVE_ONE);
        assert_eq!(
            negative_fee.freeze_requirement(),
            Err(SpotAmountConversionError::InvalidAmount)
        );

        for price in [Decimal::ZERO, Decimal::NEGATIVE_ONE] {
            let invalid_price = order(OrderSide::Buy, price, Decimal::ONE, Decimal::ZERO);
            assert_eq!(
                invalid_price.freeze_requirement(),
                Err(SpotAmountConversionError::InvalidAmount)
            );
        }
    }

    #[test]
    fn enforces_coin_precision_for_freeze_amount() {
        let exact = order(OrderSide::Buy, Decimal::new(123, 2), Decimal::ONE, Decimal::ZERO);
        assert_eq!(
            exact.freeze_requirement(),
            Ok(CoinAmount::new(Coin::USDT, Decimal::new(123, 2)).unwrap())
        );

        let excessive = order(OrderSide::Buy, Decimal::new(1, 7), Decimal::ONE, Decimal::ZERO);
        assert_eq!(
            excessive.freeze_requirement(),
            Err(SpotAmountConversionError::InexactConversion)
        );
    }

    #[test]
    fn reports_checked_decimal_multiplication_overflow() {
        let order = order(OrderSide::Buy, Decimal::MAX, Decimal::MAX, Decimal::ZERO);

        assert_eq!(order.freeze_requirement(), Err(SpotAmountConversionError::ArithmeticOverflow));
    }

    #[test]
    fn reports_checked_decimal_addition_overflow() {
        let order = order(OrderSide::Buy, Decimal::MAX, Decimal::ONE, Decimal::ONE);

        assert_eq!(order.freeze_requirement(), Err(SpotAmountConversionError::ArithmeticOverflow));
    }
}
