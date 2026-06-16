use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    SpotOrder, SpotOrderFinalization, SpotOrderMatchError, SpotOrderStatus, SpotOrderStatusReason,
    SpotTrade,
};

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

impl From<SpotOrderMatchError> for MatchSpotOrderError {
    fn from(value: SpotOrderMatchError) -> Self {
        match value {
            SpotOrderMatchError::InconsistentExecutionState => Self::InconsistentOrderState,
            SpotOrderMatchError::SameSideMaker => Self::SameSideMaker,
            SpotOrderMatchError::MakerMustBeLimit => Self::MakerMustBeLimit,
            SpotOrderMatchError::NoTradesMatched => Self::NoTradesMatched,
            SpotOrderMatchError::ArithmeticOverflow => Self::ArithmeticOverflow,
        }
    }
}

/// 本次撮合的 typed output。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchSpotOrderChanges {
    /// 本次撮合新生成的成交事实。
    pub trades: Vec<SpotTrade>,
    /// 本次撮合后 taker 订单状态。
    pub taker_order_after: SpotOrder,
    /// 本次撮合中实际发生变化的 maker 订单 after 快照，顺序与撮合顺序一致。
    pub maker_orders_after: Vec<SpotOrder>,
    pub taker_order_before: SpotOrder,
    pub maker_orders_updated: Vec<UpdatedEntityPair<SpotOrder>>,
    trade_maker_updates: Vec<(SpotTrade, UpdatedEntityPair<SpotOrder>)>,
}

/// Use case that matches one spot taker order against pre-sorted maker orders.
///
/// 用例只负责订单撮合、成交事实创建和订单成交状态更新；账户清算由后续 use case 处理。
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchSpotOrderUseCase;

impl ReplayableChanges for MatchSpotOrderChanges {
    fn to_replayable_events(
        &self,
    ) -> Result<Vec<common_entity::EntityReplayableEvent>, EventProjectError> {
        let mut events =
            Vec::with_capacity(self.trades.len() + self.maker_orders_updated.len() + 1);
        for (trade, maker) in &self.trade_maker_updates {
            events.push(trade.track_create_event()?);
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        events.push(self.taker_order_after.track_update_event_from(&self.taker_order_before)?);
        Ok(events)
    }
}

impl CommandUseCase4 for MatchSpotOrderUseCase {
    type Command = MatchSpotOrderCmd;
    type GivenState = MatchSpotOrderState;
    type Error = MatchSpotOrderError;
    type Changes = MatchSpotOrderChanges;

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

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let mut taker_order = state.taker_order;
        let taker_order_before = taker_order.clone();
        let mut taker_remaining = taker_order.remaining_qty()?;
        let mut total_taker_fill = 0_u64;
        let mut trades = Vec::new();
        let mut maker_orders_after = Vec::new();
        let mut maker_orders_updated = Vec::new();
        let mut trade_maker_updates = Vec::new();
        let best_maker = state.maker_orders.first();

        if taker_order.would_be_rejected_as_alo(best_maker)? {
            let current_filled_qty = taker_order.filled_qty;
            apply_taker_order_update(
                &mut taker_order,
                current_filled_qty,
                SpotOrderStatus::Rejected,
                Some(SpotOrderStatusReason::BadAloPxRejected),
            )?;
            return Ok(MatchSpotOrderChanges {
                trades,
                taker_order_after: taker_order,
                maker_orders_after,
                taker_order_before,
                maker_orders_updated,
                trade_maker_updates,
            });
        }

        for (trade_index, maker_order) in state.maker_orders.into_iter().enumerate() {
            if taker_remaining == 0 {
                break;
            }

            if !taker_order.crosses_order(&maker_order)? {
                break;
            }

            let maker_price =
                maker_order.limit_price().ok_or(MatchSpotOrderError::MakerMustBeLimit)?;
            let maker_remaining = maker_order.remaining_qty()?;
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
            trades.push(trade);
            let trade_for_projection =
                trades.last().cloned().ok_or(MatchSpotOrderError::ArithmeticOverflow)?;

            let mut next_maker_order = maker_order;
            let previous_maker_order = next_maker_order.clone();
            let next_maker_filled = next_maker_order
                .filled_qty
                .checked_add(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            let next_maker_status = next_maker_order.matched_status_for(next_maker_filled);
            let next_maker_version = next_maker_order
                .version
                .checked_add(1)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            next_maker_order.filled_qty = next_maker_filled;
            next_maker_order.status = next_maker_status;
            next_maker_order.version = next_maker_version;
            let maker_update =
                UpdatedEntityPair { before: previous_maker_order, after: next_maker_order.clone() };
            maker_orders_after.push(next_maker_order);
            maker_orders_updated.push(maker_update.clone());
            trade_maker_updates.push((trade_for_projection, maker_update));

            taker_remaining = taker_remaining
                .checked_sub(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            total_taker_fill = total_taker_fill
                .checked_add(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
        }

        let finalization = taker_order.finalize_after_match(total_taker_fill)?;
        apply_taker_order_with_finalization(&mut taker_order, finalization)?;

        Ok(MatchSpotOrderChanges {
            trades,
            taker_order_after: taker_order,
            maker_orders_after,
            taker_order_before,
            maker_orders_updated,
            trade_maker_updates,
        })
    }
}

fn validate_matchable_order(order: &SpotOrder) -> Result<(), MatchSpotOrderError> {
    if !order.has_consistent_execution_state() {
        return Err(MatchSpotOrderError::InconsistentOrderState);
    }
    if !matches!(order.status, SpotOrderStatus::Open | SpotOrderStatus::PartiallyFilled) {
        return Err(MatchSpotOrderError::OrderNotMatchable);
    }
    if order.remaining_qty()? == 0 {
        return Err(MatchSpotOrderError::OrderNotMatchable);
    }
    Ok(())
}

fn apply_taker_order_update(
    taker_order: &mut SpotOrder,
    next_taker_filled: u64,
    next_taker_status: SpotOrderStatus,
    next_taker_status_reason: Option<SpotOrderStatusReason>,
) -> Result<(), MatchSpotOrderError> {
    let next_taker_version =
        taker_order.version.checked_add(1).ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
    taker_order.filled_qty = next_taker_filled;
    taker_order.status = next_taker_status;
    taker_order.status_reason = next_taker_status_reason;
    taker_order.version = next_taker_version;
    Ok(())
}

fn apply_taker_order_with_finalization(
    taker_order: &mut SpotOrder,
    finalization: SpotOrderFinalization,
) -> Result<(), MatchSpotOrderError> {
    apply_taker_order_update(
        taker_order,
        finalization.next_filled_qty,
        finalization.status,
        finalization.status_reason,
    )
}

#[cfg(test)]
mod compute_replayable_events_happy_path;
