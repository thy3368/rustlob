use cmd_handler::command_use_case_def2::{
    CommandUseCase4, EventProjectError, ReplayableChanges, UpdatedEntityPair,
};
use cmd_handler::EntityReplayableEvent;
use common_entity::Entity;
use example_core_entity::OrderSide;
use thiserror::Error;

use crate::entity::{HyperliquidPerpOrder, HyperliquidPerpOrderStatus, HyperliquidPerpTrade};
use crate::support::concat3;

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
    /// 本次撮合的 taker 订单 Hyperliquid perp `oid`。
    pub taker_order_id: u64,
    /// 一次撮合批次 ID，用于稳定生成多条 trade id。
    pub match_id: String,
    /// 本次成交批次的稳定执行时间，单位毫秒。
    pub executed_at_ms: u64,
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
    #[error("taker order does not match on_command")]
    TakerOrderMismatch,
    /// taker 订单不属于命令账户。
    #[error("taker order does not belong to on_command party")]
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

/// 完成一轮撮合后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchHyperliquidPerpOrderChanges {
    /// 本批次新创建的成交事实。
    pub created_trades: Vec<HyperliquidPerpTrade>,
    /// 按撮合发生顺序返回的 maker before/after。
    pub updated_maker_orders: Vec<UpdatedEntityPair<HyperliquidPerpOrder>>,
    /// 撮合后的 taker before/after。
    pub updated_taker_order: UpdatedEntityPair<HyperliquidPerpOrder>,
}

impl ReplayableChanges for MatchHyperliquidPerpOrderChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(0);
        for (trade, maker_order) in self.created_trades.iter().zip(&self.updated_maker_orders) {
            events.push(trade.track_create_event()?);
            events.push(maker_order.after.track_update_event_from(&maker_order.before)?);
        }
        events.push(
            self.updated_taker_order
                .after
                .track_update_event_from(&self.updated_taker_order.before)?,
        );
        Ok(events)
    }
}

impl CommandUseCase4 for MatchHyperliquidPerpOrderUseCase {
    type Command = MatchHyperliquidPerpOrderCmd;
    type GivenState = MatchHyperliquidPerpOrderState;
    type Error = MatchHyperliquidPerpOrderError;
    type Changes = MatchHyperliquidPerpOrderChanges;

    fn role(&self) -> &'static str {
        "MatchingEngine"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(MatchHyperliquidPerpOrderError::InvalidPartyId);
        }
        if cmd.taker_order_id == 0 {
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

    fn compute_changes(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Self::Changes, Self::Error> {
        let mut taker_order_after = state.taker_order.clone();
        let mut taker_remaining = remaining_qty(&taker_order_after)?;
        let mut total_taker_fill = 0_u64;
        let mut created_trades = Vec::with_capacity(0);
        let mut updated_maker_orders = Vec::with_capacity(0);

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
            let trade_suffix = trade_index
                .checked_add(1)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;

            let trade = HyperliquidPerpTrade::new(
                concat3(cmd.match_id.as_str(), "-", trade_suffix.to_string().as_str()),
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
                cmd.executed_at_ms,
            );
            created_trades.push(trade);

            let mut next_maker_order = maker_order.clone();
            let next_maker_filled = next_maker_order
                .filled_sz
                .checked_add(trade_qty)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            let next_maker_status = matched_status(next_maker_order.sz, next_maker_filled);
            let next_maker_version = next_maker_order
                .version
                .checked_add(1)
                .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
            next_maker_order.filled_sz = next_maker_filled;
            next_maker_order.status = next_maker_status;
            next_maker_order.version = next_maker_version;
            updated_maker_orders
                .push(UpdatedEntityPair { before: maker_order.clone(), after: next_maker_order });

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
            .filled_sz
            .checked_add(total_taker_fill)
            .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
        let next_taker_status = matched_status(taker_order_after.sz, next_taker_filled);
        let next_taker_version = taker_order_after
            .version
            .checked_add(1)
            .ok_or(MatchHyperliquidPerpOrderError::ArithmeticOverflow)?;
        taker_order_after.filled_sz = next_taker_filled;
        taker_order_after.status = next_taker_status;
        taker_order_after.version = next_taker_version;

        Ok(MatchHyperliquidPerpOrderChanges {
            created_trades,
            updated_maker_orders,
            updated_taker_order: UpdatedEntityPair {
                before: state.taker_order,
                after: taker_order_after,
            },
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

fn can_cross(taker_side: OrderSide, taker_price: u64, maker_price: u64) -> bool {
    match taker_side {
        OrderSide::Buy => taker_price >= maker_price,
        OrderSide::Sell => taker_price <= maker_price,
    }
}

fn matched_status(qty: u64, filled_qty: u64) -> HyperliquidPerpOrderStatus {
    if filled_qty == qty {
        HyperliquidPerpOrderStatus::Filled
    } else {
        HyperliquidPerpOrderStatus::PartiallyFilled
    }
}
