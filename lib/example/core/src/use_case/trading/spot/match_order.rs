use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::Entity;
use thiserror::Error;

use crate::entity::{SpotOrder, SpotOrderMatchError, SpotOrderStatus, SpotTrade};

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
            SpotOrderMatchError::OrderNotMatchable => Self::OrderNotMatchable,
            SpotOrderMatchError::MakerIsTaker => Self::MakerIsTaker,
            SpotOrderMatchError::SameSideMaker => Self::SameSideMaker,
            SpotOrderMatchError::MakerMustBeLimit => Self::MakerMustBeLimit,
            SpotOrderMatchError::AssetMismatch => Self::AssetMismatch,
            SpotOrderMatchError::SymbolMismatch => Self::SymbolMismatch,
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
    /// 本次撮合中实际发生变化的 maker 订单 before/after，顺序与撮合顺序一致。
    pub updated_maker_orders: Vec<UpdatedEntityPair<SpotOrder>>,
    /// 本次撮合后的 taker 订单 before/after。
    pub updated_taker_order: UpdatedEntityPair<SpotOrder>,
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
            Vec::with_capacity(self.trades.len() + self.updated_maker_orders.len() + 1);
        for (trade, maker) in self.trades.iter().zip(&self.updated_maker_orders) {
            events.push(trade.track_create_event()?);
            events.push(maker.after.track_update_event_from(&maker.before)?);
        }
        events.push(
            self.updated_taker_order
                .after
                .track_update_event_from(&self.updated_taker_order.before)?,
        );
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
        taker.ensure_matchable()?;

        for maker in &state.maker_orders {
            maker.ensure_matchable()?;
            maker.ensure_compatible_maker_for(taker)?;
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
        let mut updated_maker_orders = Vec::new();
        let best_maker = state.maker_orders.first();

        if taker_order.would_be_rejected_as_alo(best_maker)? {
            taker_order.reject_as_bad_alo()?;
            return Ok(MatchSpotOrderChanges {
                trades,
                updated_maker_orders,
                updated_taker_order: UpdatedEntityPair {
                    before: taker_order_before,
                    after: taker_order,
                },
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

            let mut next_maker_order = maker_order;
            let previous_maker_order = next_maker_order.clone();
            next_maker_order.apply_fill(trade_qty)?;
            updated_maker_orders
                .push(UpdatedEntityPair { before: previous_maker_order, after: next_maker_order });

            taker_remaining = taker_remaining
                .checked_sub(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
            total_taker_fill = total_taker_fill
                .checked_add(trade_qty)
                .ok_or(MatchSpotOrderError::ArithmeticOverflow)?;
        }

        let finalization = taker_order.finalize_after_match(total_taker_fill)?;
        taker_order.apply_finalization(finalization)?;

        Ok(MatchSpotOrderChanges {
            trades,
            updated_maker_orders,
            updated_taker_order: UpdatedEntityPair {
                before: taker_order_before,
                after: taker_order,
            },
        })
    }
}

#[cfg(test)]
mod compute_replayable_events_happy_path;
