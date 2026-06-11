use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{
    SpotOrder, SpotOrderFinalization, SpotOrderSide, SpotOrderStatus, SpotOrderStatusReason,
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
        // use case 只编排撮合流程与事件；价格交叉、ALO 拒绝、TIF 收尾都委托给 entity。
        let mut taker_order = state.taker_order;
        let mut taker_remaining = taker_order.remaining_qty()?;
        let mut total_taker_fill = 0_u64;
        let mut events = Vec::new();
        let best_maker = state.maker_orders.first();

        // ALO 只看当前最优 maker：如果会立即吃单，就直接产出一条 taker reject update event。
        if taker_order.would_be_rejected_as_alo(best_maker)? {
            let current_filled_qty = taker_order.filled_qty;
            return Ok(vec![update_taker_order(
                &mut taker_order,
                current_filled_qty,
                SpotOrderStatus::Rejected,
                Some(SpotOrderStatusReason::BadAloPxRejected),
            )?]);
        }

        for (trade_index, maker_order) in state.maker_orders.into_iter().enumerate() {
            if taker_remaining == 0 {
                break;
            }

            // maker 已按优先级排序；遇到首个不再交叉的价格就终止扫描，后续 maker 不再考虑。
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

            // 先记录成交事实，再分别推进 maker / taker 的成交数量与生命周期状态。
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
            let next_maker_status = next_maker_order.matched_status_for(next_maker_filled);
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

        // taker 收尾统一走 entity 决策：GTC / IOC / ALO 的最终状态在这里收口。
        let finalization = taker_order.finalize_after_match(total_taker_fill)?;
        events.push(update_taker_order_with_finalization(&mut taker_order, finalization)?);

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
    if order.remaining_qty()? == 0 {
        return Err(MatchSpotOrderError::OrderNotMatchable);
    }
    Ok(())
}

fn update_taker_order(
    taker_order: &mut SpotOrder,
    next_taker_filled: u64,
    next_taker_status: SpotOrderStatus,
    next_taker_status_reason: Option<SpotOrderStatusReason>,
) -> Result<EntityReplayableEvent, MatchSpotOrderError> {
    let next_taker_version =
        taker_order.version.checked_add(1).ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
    taker_order
        .track_update_event(|order| {
            order.filled_qty = next_taker_filled;
            order.status = next_taker_status;
            order.status_reason = next_taker_status_reason;
            order.version = next_taker_version;
        })
        .map_err(|_| MatchSpotOrderError::ArithmeticOverflow)
}

fn update_taker_order_with_finalization(
    taker_order: &mut SpotOrder,
    finalization: SpotOrderFinalization,
) -> Result<EntityReplayableEvent, MatchSpotOrderError> {
    update_taker_order(
        taker_order,
        finalization.next_filled_qty,
        finalization.status,
        finalization.status_reason,
    )
}

#[cfg(test)]
mod compute_replayable_events_happy_path;
