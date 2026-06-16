use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase3, IssuedByParty, UseCaseOutput};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    HyperliquidPerpOrder, HyperliquidPerpOrderSide, HyperliquidPerpOrderStatus,
    HyperliquidPerpTrade,
};

/// 撮合 Hyperliquid perp taker 订单时需要的已加载业务状态。
///
/// `maker_orders` 必须已经由 adapter 或订单簿按撮合优先级排好序；
/// core use case 只按 Vec 顺序从头到尾撮合。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderState {
    /// 本次作为主动吃单方的订单。
    pub taker_order: HyperliquidPerpOrder,
    /// 已按撮合优先级排好的被动挂单。
    pub maker_orders: Vec<HyperliquidPerpOrder>,
}

/// 执行一次 Hyperliquid perp 撮合批次的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderCmd {
    /// 发起撮合的业务账户，应等于 taker 订单账户。
    pub party_id: String,
    /// 本次撮合的 taker 订单 ID。
    pub taker_order_id: String,
    /// 一次撮合批次 ID，用于稳定生成多条 trade id。
    pub match_id: String,
}

impl IssuedByParty for MatchHyperliquidPerpOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// Hyperliquid perp 撮合可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchHyperliquidPerpOrderError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// taker order id 不能为空。
    #[error("taker_order_id must not be empty")]
    InvalidTakerOrderId,
    /// match id 不能为空。
    #[error("match_id must not be empty")]
    InvalidMatchId,
    /// 命令 taker id 和已加载 taker 订单不一致。
    #[error("taker order does not match command")]
    TakerOrderMismatch,
    /// taker 订单不属于命令账户。
    #[error("taker order does not belong to command party")]
    TakerOwnerMismatch,
    /// 订单生命周期状态不允许继续撮合。
    #[error("order is not matchable")]
    OrderNotMatchable,
    /// 订单状态和已成交数量不一致。
    #[error("order execution state is inconsistent")]
    InconsistentOrderState,
    /// maker 和 taker 必须方向相反。
    #[error("maker order has the same side as taker")]
    SameSideMaker,
    /// maker 必须是限价单，成交价取 maker 限价。
    #[error("maker order must be a limit order")]
    MakerMustBeLimit,
    /// maker 和 taker 必须交易同一 perp asset。
    #[error("maker order trades a different asset")]
    AssetMismatch,
    /// maker 和 taker 必须交易同一展示合约。
    #[error("maker order trades a different symbol")]
    SymbolMismatch,
    /// maker 订单不能和 taker 是同一张订单。
    #[error("maker order must not be the taker order")]
    MakerIsTaker,
    /// 按当前 maker 顺序没有任何可成交结果。
    #[error("no Hyperliquid perp trades were matched")]
    NoTradesMatched,
    /// 生成撮合结果时发生整数溢出。
    #[error("arithmetic overflow while deriving match result")]
    ArithmeticOverflow,
}

/// Use case that matches one Hyperliquid perp taker order against pre-sorted maker orders.
///
/// 用例只负责订单撮合、成交事实创建和订单成交状态更新；仓位、保证金、手续费、PnL、
/// 资金费和清算由后续 use case 处理。
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchHyperliquidPerpOrderUseCase;

/// 完成一轮撮合后的业务产出。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderOutput {
    /// 本批次新创建的成交事实。
    pub trades: Vec<HyperliquidPerpTrade>,
    /// 撮合后的 taker 订单快照。
    pub taker_order_after: HyperliquidPerpOrder,
    /// 按撮合发生顺序返回的 maker after 快照。
    pub maker_orders_after: Vec<HyperliquidPerpOrder>,
}

impl CommandUseCase3 for MatchHyperliquidPerpOrderUseCase {
    type Command = MatchHyperliquidPerpOrderCmd;
    type GivenState = MatchHyperliquidPerpOrderState;
    type Error = MatchHyperliquidPerpOrderError;
    type Output = MatchHyperliquidPerpOrderOutput;

    fn role(&self) -> &'static str {
        "MatchingEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchHyperliquidPerpOrderError::InvalidPartyId);
        }
        if cmd.taker_order_id.is_empty() {
            return Err(MatchHyperliquidPerpOrderError::InvalidTakerOrderId);
        }
        if cmd.match_id.is_empty() {
            return Err(MatchHyperliquidPerpOrderError::InvalidMatchId);
        }
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let taker = &state.taker_order;
        if taker.order_id != cmd.taker_order_id {
            return Err(MatchHyperliquidPerpOrderError::TakerOrderMismatch);
        }
        if !taker.belongs_to_account(&cmd.party_id) {
            return Err(MatchHyperliquidPerpOrderError::TakerOwnerMismatch);
        }
        validate_matchable_order(taker)?;

        for maker in &state.maker_orders {
            validate_matchable_order(maker)?;
            if maker.order_id == taker.order_id {
                return Err(MatchHyperliquidPerpOrderError::MakerIsTaker);
            }
            if maker.side == taker.side {
                return Err(MatchHyperliquidPerpOrderError::SameSideMaker);
            }
            if maker.limit_price().is_none() {
                return Err(MatchHyperliquidPerpOrderError::MakerMustBeLimit);
            }
            if !maker.trades_asset(taker.asset) {
                return Err(MatchHyperliquidPerpOrderError::AssetMismatch);
            }
            if !maker.trades_symbol(taker.symbol.as_str()) {
                return Err(MatchHyperliquidPerpOrderError::SymbolMismatch);
            }
        }

        Ok(())
    }

    fn compute_output_and_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<UseCaseOutput<Self::Output>, Self::Error> {
        let mut taker_order_after = state.taker_order.clone();
        let mut taker_remaining = remaining_qty(&taker_order_after)?;
        let mut total_taker_fill = 0_u64;
        let mut trades = Vec::new();
        let mut maker_orders_after = Vec::new();

        for (trade_index, maker_order) in state.maker_orders.iter().enumerate() {
            if taker_remaining == 0 {
                break;
            }

            let maker_price = maker_order
                .limit_price()
                .ok_or(MatchHyperliquidPerpOrderError::MakerMustBeLimit)?;
            if !can_cross(taker_order_after.side, taker_order_after.order_price(), maker_price) {
                break;
            }

            let maker_remaining = remaining_qty(maker_order)?;
            let trade_qty = taker_remaining.min(maker_remaining);
            if trade_qty == 0 {
                continue;
            }

            let trade = HyperliquidPerpTrade::new(
                format!("{}-{}", cmd.match_id, trade_index + 1),
                cmd.match_id.clone(),
                taker_order_after.asset,
                taker_order_after.symbol.clone(),
                taker_order_after.order_id.clone(),
                maker_order.order_id.clone(),
                taker_order_after.account_id.clone(),
                maker_order.account_id.clone(),
                taker_order_after.side,
                maker_price,
                trade_qty,
            );
            trades.push(trade);

            let mut next_maker_order = maker_order.clone();
            let next_maker_filled = next_maker_order
                .filled_qty
                .checked_add(trade_qty)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            let next_maker_status = matched_status(next_maker_order.qty, next_maker_filled);
            let next_maker_version = next_maker_order
                .version
                .checked_add(1)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            next_maker_order.filled_qty = next_maker_filled;
            next_maker_order.status = next_maker_status;
            next_maker_order.version = next_maker_version;
            maker_orders_after.push(next_maker_order);

            taker_remaining = taker_remaining
                .checked_sub(trade_qty)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            total_taker_fill = total_taker_fill
                .checked_add(trade_qty)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
        }

        if total_taker_fill == 0 {
            return Err(MatchHyperliquidPerpOrderError::NoTradesMatched);
        }

        let next_taker_filled = taker_order_after
            .filled_qty
            .checked_add(total_taker_fill)
            .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
        let next_taker_status = matched_status(taker_order_after.qty, next_taker_filled);
        let next_taker_version = taker_order_after
            .version
            .checked_add(1)
            .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
        taker_order_after.filled_qty = next_taker_filled;
        taker_order_after.status = next_taker_status;
        taker_order_after.version = next_taker_version;

        let mut events = Vec::new();
        for (trade, maker_after) in trades.iter().zip(&maker_orders_after) {
            events.push(
                trade
                    .track_create_event()
                    .map_err(|_| MatchHyperliquidPerpOrderError::ArithmeticOverflow)?,
            );
            let maker_before = state
                .maker_orders
                .iter()
                .find(|maker| maker.order_id == maker_after.order_id)
                .ok_or(MatchHyperliquidPerpOrderError::MakerIsTaker)?;
            events.push(
                maker_after
                    .track_update_event_from(maker_before)
                    .map_err(|_| MatchHyperliquidPerpOrderError::ArithmeticOverflow)?,
            );
        }
        events.push(
            taker_order_after
                .track_update_event_from(&state.taker_order)
                .map_err(|_| MatchHyperliquidPerpOrderError::ArithmeticOverflow)?,
        );

        Ok(UseCaseOutput {
            output: MatchHyperliquidPerpOrderOutput {
                trades,
                taker_order_after,
                maker_orders_after,
            },
            events,
        })
    }
}

fn validate_matchable_order(
    order: &HyperliquidPerpOrder,
) -> Result<(), MatchHyperliquidPerpOrderError> {
    if !order.has_consistent_execution_state() {
        return Err(MatchHyperliquidPerpOrderError::InconsistentOrderState);
    }
    if !order.is_matchable() {
        return Err(MatchHyperliquidPerpOrderError::OrderNotMatchable);
    }
    Ok(())
}

fn remaining_qty(order: &HyperliquidPerpOrder) -> Result<u64, MatchHyperliquidPerpOrderError> {
    order.remaining_qty().ok_or(MatchHyperliquidPerpOrderError::InconsistentOrderState)
}

fn can_cross(taker_side: HyperliquidPerpOrderSide, taker_price: u64, maker_price: u64) -> bool {
    match taker_side {
        HyperliquidPerpOrderSide::Buy => taker_price >= maker_price,
        HyperliquidPerpOrderSide::Sell => taker_price <= maker_price,
    }
}

fn matched_status(qty: u64, filled_qty: u64) -> HyperliquidPerpOrderStatus {
    if filled_qty == qty {
        HyperliquidPerpOrderStatus::Filled
    } else {
        HyperliquidPerpOrderStatus::PartiallyFilled
    }
}

#[cfg(test)]
mod tests {
    use cmd_handler::command_use_case_def2::CommandUseCase3;
    use proptest::prelude::*;

    use super::*;
    use crate::entity::{HyperliquidPerpOrderExecution, HyperliquidPerpOrderTimeInForce};
    use crate::use_case::support::field_as_u64;

    fn cmd() -> MatchHyperliquidPerpOrderCmd {
        MatchHyperliquidPerpOrderCmd {
            party_id: "buyer".to_string(),
            taker_order_id: "taker-1".to_string(),
            match_id: "match-1".to_string(),
        }
    }

    fn order(
        order_id: &str,
        account_id: &str,
        side: HyperliquidPerpOrderSide,
        price: u64,
        qty: u64,
    ) -> HyperliquidPerpOrder {
        HyperliquidPerpOrder::new(
            order_id.to_string(),
            Some(42),
            0,
            account_id.to_string(),
            "BTC-PERP".to_string(),
            side,
            HyperliquidPerpOrderExecution::Limit { price },
            HyperliquidPerpOrderTimeInForce::Gtc,
            qty,
            false,
            None,
        )
    }

    fn taker_buy(qty: u64, price: u64) -> HyperliquidPerpOrder {
        order("taker-1", "buyer", HyperliquidPerpOrderSide::Buy, price, qty)
    }

    fn maker_sell(order_id: &str, qty: u64, price: u64) -> HyperliquidPerpOrder {
        order(order_id, "seller", HyperliquidPerpOrderSide::Sell, price, qty)
    }

    fn event_field<'a>(event: &'a EntityReplayableEvent, field_name: &str) -> Option<&'a str> {
        event.field_changes.iter().find_map(|change| {
            if change.field_name_as_str().ok() != Some(field_name) {
                return None;
            }
            std::str::from_utf8(change.new_value_bytes()).ok()
        })
    }

    #[test]
    fn role_is_matching_engine() {
        assert_eq!(MatchHyperliquidPerpOrderUseCase.role(), "MatchingEngine");
    }

    #[test]
    fn pre_check_rejects_empty_command_fields() {
        let mut invalid_party = cmd();
        invalid_party.party_id.clear();
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.pre_check_command(&invalid_party),
            Err(MatchHyperliquidPerpOrderError::InvalidPartyId)
        );

        let mut invalid_taker = cmd();
        invalid_taker.taker_order_id.clear();
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.pre_check_command(&invalid_taker),
            Err(MatchHyperliquidPerpOrderError::InvalidTakerOrderId)
        );

        let mut invalid_match = cmd();
        invalid_match.match_id.clear();
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.pre_check_command(&invalid_match),
            Err(MatchHyperliquidPerpOrderError::InvalidMatchId)
        );
    }

    #[test]
    fn validate_rejects_taker_order_mismatch() {
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: Vec::new(),
        };
        let mut cmd = cmd();
        cmd.taker_order_id = "different".to_string();

        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd, &state),
            Err(MatchHyperliquidPerpOrderError::TakerOrderMismatch)
        );
    }

    #[test]
    fn validate_rejects_taker_owner_mismatch() {
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: Vec::new(),
        };
        let mut cmd = cmd();
        cmd.party_id = "other".to_string();

        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd, &state),
            Err(MatchHyperliquidPerpOrderError::TakerOwnerMismatch)
        );
    }

    #[test]
    fn validate_rejects_same_side_maker() {
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![order("maker-1", "seller", HyperliquidPerpOrderSide::Buy, 99, 1)],
        };

        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchHyperliquidPerpOrderError::SameSideMaker)
        );
    }

    #[test]
    fn validate_rejects_different_asset_and_symbol() {
        let mut maker = maker_sell("maker-1", 1, 99);
        maker.asset = 1;
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker],
        };
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchHyperliquidPerpOrderError::AssetMismatch)
        );

        let mut maker = maker_sell("maker-1", 1, 99);
        maker.symbol = "ETH-PERP".to_string();
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker],
        };
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchHyperliquidPerpOrderError::SymbolMismatch)
        );
    }

    #[test]
    fn validate_rejects_market_maker_unmatchable_order_and_maker_is_taker() {
        let mut maker = maker_sell("maker-1", 1, 99);
        maker.execution = HyperliquidPerpOrderExecution::Market { aggressive_price: 99 };
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker],
        };
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchHyperliquidPerpOrderError::MakerMustBeLimit)
        );

        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100)
                .with_execution_state(HyperliquidPerpOrderStatus::Canceled, 0),
            maker_orders: Vec::new(),
        };
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchHyperliquidPerpOrderError::OrderNotMatchable)
        );

        let mut maker = maker_sell("taker-1", 1, 99);
        maker.side = HyperliquidPerpOrderSide::Sell;
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker],
        };
        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchHyperliquidPerpOrderError::MakerIsTaker)
        );
    }

    #[test]
    fn compute_matches_multiple_makers_and_updates_orders()
    -> Result<(), MatchHyperliquidPerpOrderError> {
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker_sell("maker-1", 1, 99), maker_sell("maker-2", 2, 100)],
        };

        let result = MatchHyperliquidPerpOrderUseCase.compute_output_and_events(&cmd(), state)?;
        let events = result.events;

        assert_eq!(events.len(), 5);
        assert_eq!(result.output.trades.len(), 2);
        assert_eq!(result.output.taker_order_after.filled_qty, 3);
        assert!(events[0].is_created());
        assert_eq!(event_field(&events[0], "trade_id"), Some("match-1-1"));
        assert_eq!(event_field(&events[0], "maker_order_id"), Some("maker-1"));
        assert_eq!(field_as_u64(&events[0], "price"), Some(99));
        assert_eq!(field_as_u64(&events[0], "qty"), Some(1));
        assert!(events[1].is_updated());
        assert_eq!(event_field(&events[1], "filled_qty"), Some("1"));
        assert_eq!(event_field(&events[1], "status"), Some("filled"));
        assert!(events[2].is_created());
        assert_eq!(event_field(&events[2], "trade_id"), Some("match-1-2"));
        assert!(events[3].is_updated());
        assert_eq!(event_field(&events[3], "filled_qty"), Some("2"));
        assert_eq!(event_field(&events[3], "status"), Some("filled"));
        assert!(events[4].is_updated());
        assert_eq!(event_field(&events[4], "filled_qty"), Some("3"));
        assert_eq!(event_field(&events[4], "status"), Some("filled"));

        Ok(())
    }

    #[test]
    fn compute_stops_at_first_non_crossing_maker() -> Result<(), MatchHyperliquidPerpOrderError> {
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![
                maker_sell("maker-1", 1, 99),
                maker_sell("maker-2", 1, 101),
                maker_sell("maker-3", 1, 100),
            ],
        };

        let result = MatchHyperliquidPerpOrderUseCase.compute_output_and_events(&cmd(), state)?;
        let events = result.events;

        assert_eq!(events.len(), 3);
        assert_eq!(result.output.maker_orders_after.len(), 1);
        assert_eq!(event_field(&events[0], "maker_order_id"), Some("maker-1"));
        assert_eq!(event_field(&events[2], "filled_qty"), Some("1"));
        assert_eq!(event_field(&events[2], "status"), Some("partially_filled"));

        Ok(())
    }

    #[test]
    fn compute_rejects_when_no_trade_crosses() {
        let state = MatchHyperliquidPerpOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker_sell("maker-1", 1, 101)],
        };

        assert_eq!(
            MatchHyperliquidPerpOrderUseCase.compute_output_and_events(&cmd(), state),
            Err(MatchHyperliquidPerpOrderError::NoTradesMatched)
        );
    }

    proptest! {
        #[test]
        fn matched_events_preserve_quantities_and_identities(
            taker_qty in 1_u64..20,
            maker_qtys in proptest::collection::vec(1_u64..10, 1..8),
        ) {
            let makers: Vec<_> = maker_qtys
                .iter()
                .enumerate()
                .map(|(index, qty)| maker_sell(&format!("maker-{}", index + 1), *qty, 100))
                .collect();
            let state = MatchHyperliquidPerpOrderState {
                taker_order: taker_buy(taker_qty, 100),
                maker_orders: makers,
            };

            let result = MatchHyperliquidPerpOrderUseCase
                .compute_output_and_events(&cmd(), state)
                .expect("generated makers cross the taker price");
            let events = result.events;

            let trade_events: Vec<_> = events.iter().filter(|event| event.is_created()).collect();
            let trade_qty_sum: u64 = trade_events
                .iter()
                .filter_map(|event| field_as_u64(event, "qty"))
                .sum();

            prop_assert!(trade_qty_sum <= taker_qty);
            prop_assert_eq!(events.len(), trade_events.len() * 2 + 1);

            for (index, trade_event) in trade_events.iter().enumerate() {
                let trade_qty = field_as_u64(trade_event, "qty").unwrap_or(0);
                let expected_maker_order_id = format!("maker-{}", index + 1);
                prop_assert!(trade_qty <= maker_qtys[index]);
                prop_assert_eq!(event_field(trade_event, "match_id"), Some("match-1"));
                prop_assert_eq!(event_field(trade_event, "taker_order_id"), Some("taker-1"));
                prop_assert_eq!(
                    event_field(trade_event, "maker_order_id"),
                    Some(expected_maker_order_id.as_str())
                );
                prop_assert_eq!(event_field(trade_event, "taker_account_id"), Some("buyer"));
                prop_assert_eq!(event_field(trade_event, "maker_account_id"), Some("seller"));
                prop_assert_eq!(field_as_u64(trade_event, "asset"), Some(0));
                prop_assert_eq!(field_as_u64(trade_event, "price"), Some(100));
            }

            for maker_index in 0..trade_events.len() {
                let maker_update_index = maker_index * 2 + 1;
                let filled_qty = field_as_u64(&events[maker_update_index], "filled_qty")
                    .unwrap_or(0);
                prop_assert!(filled_qty <= maker_qtys[maker_index]);
            }

            let taker_update = events.last().expect("successful match updates taker");
            let taker_filled_qty = field_as_u64(taker_update, "filled_qty").unwrap_or(0);
            prop_assert!(taker_filled_qty <= taker_qty);
        }
    }
}
