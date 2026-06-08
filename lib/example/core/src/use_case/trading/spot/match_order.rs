use cmd_handler::EntityReplayableEvent;
use cmd_handler::use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{SpotOrder, SpotOrderSide, SpotOrderStatus, SpotTrade};

/// 撮合现货 taker 订单时需要的已加载业务状态。
///
/// `maker_orders` 必须已经由 adapter 或订单簿按撮合优先级排好序；
/// core use case 只按 Vec 顺序从头到尾撮合。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderState {
    /// 本次作为主动吃单方的订单。
    pub taker_order: SpotOrder,
    /// 已按撮合优先级排好的被动挂单。
    pub maker_orders: Vec<SpotOrder>,
}

/// 执行一次现货撮合批次的命令。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderCmd {
    /// 发起撮合的业务账户，应等于 taker 订单账户。
    pub party_id: String,
    /// 本次撮合的 taker 订单 ID。
    pub taker_order_id: String,
    /// 一次撮合批次 ID，用于稳定生成多条 trade id。
    pub match_id: String,
}

impl IssuedByParty for MatchSpotOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 现货撮合可能产生的业务拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum MatchSpotOrderError {
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
    /// maker 和 taker 必须交易同一现货 asset。
    #[error("maker order trades a different asset")]
    AssetMismatch,
    /// maker 和 taker 必须交易同一展示交易对。
    #[error("maker order trades a different symbol")]
    SymbolMismatch,
    /// maker 订单不能和 taker 是同一张订单。
    #[error("maker order must not be the taker order")]
    MakerIsTaker,
    /// 按当前 maker 顺序没有任何可成交结果。
    #[error("no spot trades were matched")]
    NoTradesMatched,
    /// 生成撮合结果时发生整数溢出。
    #[error("arithmetic overflow while deriving match result")]
    ArithmeticOverflow,
}

/// Use case that matches one spot taker order against pre-sorted maker orders.
///
/// 用例只负责订单撮合、成交事实创建和订单成交状态更新；账户清算由后续 use case 处理。
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderUseCase;

impl CommandUseCase2 for MatchSpotOrderUseCase {
    type Command = MatchSpotOrderCmd;
    type GivenState = MatchSpotOrderState;
    type Error = MatchSpotOrderError;

    fn role(&self) -> &'static str {
        "MatchingEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchSpotOrderError::InvalidPartyId);
        }
        if cmd.taker_order_id.is_empty() {
            return Err(MatchSpotOrderError::InvalidTakerOrderId);
        }
        if cmd.match_id.is_empty() {
            return Err(MatchSpotOrderError::InvalidMatchId);
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
            return Err(MatchSpotOrderError::TakerOrderMismatch);
        }
        if !taker.belongs_to_account(&cmd.party_id) {
            return Err(MatchSpotOrderError::TakerOwnerMismatch);
        }
        validate_matchable_order(taker)?;

        for maker in &state.maker_orders {
            validate_matchable_order(maker)?;
            if maker.order_id == taker.order_id {
                return Err(MatchSpotOrderError::MakerIsTaker);
            }
            if maker.side == taker.side {
                return Err(MatchSpotOrderError::SameSideMaker);
            }
            if maker.limit_price().is_none() {
                return Err(MatchSpotOrderError::MakerMustBeLimit);
            }
            if !maker.trades_asset(taker.asset) {
                return Err(MatchSpotOrderError::AssetMismatch);
            }
            if !maker.trades_symbol(taker.symbol.as_str()) {
                return Err(MatchSpotOrderError::SymbolMismatch);
            }
        }

        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let mut taker_order = state.taker_order;
        let mut taker_remaining = remaining_qty(&taker_order)?;
        let mut total_taker_fill = 0_u64;
        let mut events = Vec::new();

        for (trade_index, maker_order) in state.maker_orders.into_iter().enumerate() {
            if taker_remaining == 0 {
                break;
            }

            let maker_price =
                maker_order.limit_price().ok_or(MatchSpotOrderError::MakerMustBeLimit)?;
            if !can_cross(taker_order.side, taker_order.order_price(), maker_price) {
                break;
            }

            let maker_remaining = remaining_qty(&maker_order)?;
            let trade_qty = taker_remaining.min(maker_remaining);
            if trade_qty == 0 {
                continue;
            }

            let trade = SpotTrade::new(
                format!("{}-{}", cmd.match_id, trade_index + 1),
                cmd.match_id.clone(),
                taker_order.asset,
                taker_order.symbol.clone(),
                taker_order.order_id.clone(),
                maker_order.order_id.clone(),
                taker_order.account_id.clone(),
                maker_order.account_id.clone(),
                taker_order.side,
                maker_price,
                trade_qty,
            );
            events.push(
                trade.track_create_event().map_err(|_| MatchSpotOrderError::ArithmeticOverflow)?,
            );

            let mut next_maker_order = maker_order;
            let next_maker_filled = next_maker_order
                .filled_qty
                .checked_add(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            let next_maker_status = matched_status(next_maker_order.qty, next_maker_filled);
            let next_maker_version = next_maker_order
                .version
                .checked_add(1)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            events.push(
                next_maker_order
                    .track_update_event(|order| {
                        order.filled_qty = next_maker_filled;
                        order.status = next_maker_status;
                        order.version = next_maker_version;
                    })
                    .map_err(|_| MatchSpotOrderError::ArithmeticOverflow)?,
            );

            taker_remaining = taker_remaining
                .checked_sub(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            total_taker_fill = total_taker_fill
                .checked_add(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
        }

        if total_taker_fill == 0 {
            return Err(MatchSpotOrderError::NoTradesMatched);
        }

        let next_taker_filled = taker_order
            .filled_qty
            .checked_add(total_taker_fill)
            .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
        let next_taker_status = matched_status(taker_order.qty, next_taker_filled);
        let next_taker_version =
            taker_order.version.checked_add(1).ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
        events.push(
            taker_order
                .track_update_event(|order| {
                    order.filled_qty = next_taker_filled;
                    order.status = next_taker_status;
                    order.version = next_taker_version;
                })
                .map_err(|_| MatchSpotOrderError::ArithmeticOverflow)?,
        );

        Ok(events)
    }
}

fn validate_matchable_order(order: &SpotOrder) -> Result<(), MatchSpotOrderError> {
    if !order.has_consistent_execution_state() {
        return Err(MatchSpotOrderError::InconsistentOrderState);
    }
    if !matches!(order.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled) {
        return Err(MatchSpotOrderError::OrderNotMatchable);
    }
    if remaining_qty(order)? == 0 {
        return Err(MatchSpotOrderError::OrderNotMatchable);
    }
    Ok(())
}

fn remaining_qty(order: &SpotOrder) -> Result<u64, MatchSpotOrderError> {
    order.qty.checked_sub(order.filled_qty).ok_or(MatchSpotOrderError::InconsistentOrderState)
}

fn can_cross(taker_side: SpotOrderSide, taker_price: u64, maker_price: u64) -> bool {
    match taker_side {
        SpotOrderSide::Buy => taker_price >= maker_price,
        SpotOrderSide::Sell => taker_price <= maker_price,
    }
}

fn matched_status(qty: u64, filled_qty: u64) -> SpotOrderStatus {
    if filled_qty == qty { SpotOrderStatus::Filled } else { SpotOrderStatus::PartiallyFilled }
}

#[cfg(test)]
mod tests {
    use cmd_handler::use_case_def2::CommandUseCase2;

    use super::*;
    use crate::entity::{SpotOrderExecution, SpotOrderTimeInForce};
    use crate::use_case::support::field_as_u64;

    fn cmd() -> MatchSpotOrderCmd {
        MatchSpotOrderCmd {
            party_id: "buyer".to_string(),
            taker_order_id: "taker-1".to_string(),
            match_id: "match-1".to_string(),
        }
    }

    fn order(
        order_id: &str,
        account_id: &str,
        side: SpotOrderSide,
        price: u64,
        qty: u64,
    ) -> SpotOrder {
        let (reserved_base, reserved_quote) = match side {
            SpotOrderSide::Buy => (0, qty * price),
            SpotOrderSide::Sell => (qty, 0),
        };
        SpotOrder::new(
            order_id.to_string(),
            10_001,
            Some(42),
            account_id.to_string(),
            "BTCUSDT".to_string(),
            side,
            SpotOrderExecution::Limit { price },
            SpotOrderTimeInForce::Gtc,
            qty,
            reserved_base,
            reserved_quote,
            None,
        )
    }

    fn taker_buy(qty: u64, price: u64) -> SpotOrder {
        order("taker-1", "buyer", SpotOrderSide::Buy, price, qty)
    }

    fn maker_sell(order_id: &str, qty: u64, price: u64) -> SpotOrder {
        order(order_id, "seller", SpotOrderSide::Sell, price, qty)
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
        assert_eq!(MatchSpotOrderUseCase.role(), "MatchingEngine");
    }

    #[test]
    fn pre_check_rejects_empty_match_id() {
        let mut cmd = cmd();
        cmd.match_id.clear();

        assert_eq!(
            MatchSpotOrderUseCase.pre_check_command(&cmd),
            Err(MatchSpotOrderError::InvalidMatchId)
        );
    }

    #[test]
    fn validate_rejects_taker_order_mismatch() {
        let state =
            MatchSpotOrderState { taker_order: taker_buy(3, 100), maker_orders: Vec::new() };
        let mut cmd = cmd();
        cmd.taker_order_id = "different".to_string();

        assert_eq!(
            MatchSpotOrderUseCase.validate_against_state(&cmd, &state),
            Err(MatchSpotOrderError::TakerOrderMismatch)
        );
    }

    #[test]
    fn validate_rejects_same_side_maker() {
        let state = MatchSpotOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![order("maker-1", "seller", SpotOrderSide::Buy, 99, 1)],
        };

        assert_eq!(
            MatchSpotOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchSpotOrderError::SameSideMaker)
        );
    }

    #[test]
    fn validate_rejects_market_maker() {
        let mut maker = maker_sell("maker-1", 1, 99);
        maker.execution = SpotOrderExecution::Market { aggressive_price: 99 };
        let state =
            MatchSpotOrderState { taker_order: taker_buy(3, 100), maker_orders: vec![maker] };

        assert_eq!(
            MatchSpotOrderUseCase.validate_against_state(&cmd(), &state),
            Err(MatchSpotOrderError::MakerMustBeLimit)
        );
    }

    #[test]
    fn compute_matches_multiple_makers_and_updates_orders() -> Result<(), MatchSpotOrderError> {
        let state = MatchSpotOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker_sell("maker-1", 1, 99), maker_sell("maker-2", 2, 100)],
        };

        let events = MatchSpotOrderUseCase.compute_replayable_events(&cmd(), state)?;

        assert_eq!(events.len(), 5);
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
    fn compute_stops_at_first_non_crossing_maker() -> Result<(), MatchSpotOrderError> {
        let state = MatchSpotOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![
                maker_sell("maker-1", 1, 99),
                maker_sell("maker-2", 1, 101),
                maker_sell("maker-3", 1, 100),
            ],
        };

        let events = MatchSpotOrderUseCase.compute_replayable_events(&cmd(), state)?;

        assert_eq!(events.len(), 3);
        assert_eq!(event_field(&events[0], "maker_order_id"), Some("maker-1"));
        assert_eq!(event_field(&events[2], "filled_qty"), Some("1"));
        assert_eq!(event_field(&events[2], "status"), Some("partially_filled"));

        Ok(())
    }

    #[test]
    fn compute_rejects_when_no_trade_crosses() {
        let state = MatchSpotOrderState {
            taker_order: taker_buy(3, 100),
            maker_orders: vec![maker_sell("maker-1", 1, 101)],
        };

        assert_eq!(
            MatchSpotOrderUseCase.compute_replayable_events(&cmd(), state),
            Err(MatchSpotOrderError::NoTradesMatched)
        );
    }
}
