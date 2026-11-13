use common_entity::{
    Entity, EntityError, EntityReplayableEvent, Fsm, LogEntry, ReplayableChanges,
    UpdatedEntityPair, UseCase,
};
use rust_decimal::Decimal;
use thiserror::Error;

use crate::entity::account::symbol::Symbol;
use crate::entity::common::{OrderBase, OrderTrigger, PriceType};
use crate::entity::{
    BalanceV2, BalanceV2Error, BalanceV2Event, OrderGroupRelation, OrderSide, OrderTif,
    PerpOrderV3, PerpPositionV3, SpotOrderStatus, UserId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceHyperliquidPerpOrderExecution {
    Limit { price: Decimal, time_in_force: OrderTif },
    Market { aggressive_price: Decimal },
}

impl PlaceHyperliquidPerpOrderExecution {
    fn price(self) -> Decimal {
        match self {
            Self::Limit { price, .. } => price,
            Self::Market { aggressive_price } => aggressive_price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderCmd {
    pub user_id: UserId,
    pub symbol: Symbol,
    pub side: OrderSide,
    pub size: Decimal,
    pub reduce_only: bool,
    pub execution: PlaceHyperliquidPerpOrderExecution,
    pub cloid: Option<String>,
    pub liquidation_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderState {
    pub trading_enabled: bool,
    pub next_order_sequence: u64,
    pub min_order_size: Decimal,
    pub market_symbol: Symbol,
    pub margin_balance: BalanceV2,
    pub position: PerpPositionV3,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceHyperliquidPerpOrderError {
    #[error("size must be greater than zero")]
    InvalidSize,
    #[error("price must be greater than zero")]
    InvalidPrice,
    #[error("trading is disabled")]
    TradingDisabled,
    #[error("size is below market minimum")]
    SizeBelowMin,
    #[error("loaded state does not match command identity")]
    StateMismatch,
    #[error("invalid reduce-only order")]
    InvalidReduceOnly,
    #[error("flip order must be split into close and open orders")]
    FlipOrderRequiresSplit,
    #[error("insufficient margin balance")]
    InsufficientMarginBalance,
    #[error("arithmetic overflow while deriving order")]
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceHyperliquidPerpOrderUseCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderChanges {
    pub created_order: PerpOrderV3,
    pub updated_margin_balance: Option<UpdatedEntityPair<BalanceV2>>,
}

impl ReplayableChanges for PlaceHyperliquidPerpOrderChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EntityError> {
        let mut events = vec![self.created_order.track_create_event()?];
        if let Some(balance) = &self.updated_margin_balance {
            events.push(balance.after.track_update_event_from(&balance.before)?);
        }
        Ok(events)
    }
}

impl UseCase for PlaceHyperliquidPerpOrderUseCase {
    type Command = LogEntry<PlaceHyperliquidPerpOrderCmd>;
    type StateGiven = PlaceHyperliquidPerpOrderState;
    type Error = PlaceHyperliquidPerpOrderError;
    type StateChanged = PlaceHyperliquidPerpOrderChanges;

    fn check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.size <= Decimal::ZERO {
            return Err(PlaceHyperliquidPerpOrderError::InvalidSize);
        }
        if cmd.execution.price() <= Decimal::ZERO {
            return Err(PlaceHyperliquidPerpOrderError::InvalidPrice);
        }
        Ok(())
    }

    fn validate_state_given(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<(), Self::Error> {
        if !state.trading_enabled && cmd.liquidation_id.is_none() {
            return Err(PlaceHyperliquidPerpOrderError::TradingDisabled);
        }
        if cmd.size < state.min_order_size {
            return Err(PlaceHyperliquidPerpOrderError::SizeBelowMin);
        }
        if state.market_symbol != cmd.symbol
            || state.position.user_id() != cmd.user_id
            || state.position.coin() != cmd.symbol.base
            || state.margin_balance.account_id() != cmd.user_id.0
            || state.margin_balance.coin_idx() != u64::from(cmd.symbol.quote.index())
        {
            return Err(PlaceHyperliquidPerpOrderError::StateMismatch);
        }
        let intent = derive_intent(cmd.side, cmd.size, state.position.signed_size())?;
        if cmd.reduce_only && !matches!(intent, PlaceIntent::Reduce | PlaceIntent::Close) {
            return Err(PlaceHyperliquidPerpOrderError::InvalidReduceOnly);
        }
        if matches!(intent, PlaceIntent::Open | PlaceIntent::Increase) && !cmd.reduce_only {
            let required =
                required_margin(cmd.size, cmd.execution.price(), state.position.leverage())?;
            if state.margin_balance.available() < required {
                return Err(PlaceHyperliquidPerpOrderError::InsufficientMarginBalance);
            }
        }
        Ok(())
    }

    fn compute_state_changed_only(
        &self,
        cmd: &Self::Command,
        state: &Self::StateGiven,
    ) -> Result<Self::StateChanged, Self::Error> {
        let intent = derive_intent(cmd.side, cmd.size, state.position.signed_size())?;
        let (execution, tif) = match cmd.execution {
            PlaceHyperliquidPerpOrderExecution::Limit { time_in_force, .. } => {
                (PriceType::Limit, time_in_force)
            }
            PlaceHyperliquidPerpOrderExecution::Market { .. } => (PriceType::Market, OrderTif::Ioc),
        };
        let created_order = PerpOrderV3 {
            comon_order: OrderBase {
                owner: cmd.user_id,
                order_id: state.next_order_sequence,
                symbol: cmd.symbol,
                side: cmd.side,
                limit_px: cmd.execution.price(),
                size: cmd.size,
                orig_size: cmd.size,
                execution,
                tif,
                trigger: OrderTrigger::None,
                trigger_px: None,
                group_relation: OrderGroupRelation::Standalone,
                is_position_tpsl: false,
                reduce_only: cmd.reduce_only,
                status: SpotOrderStatus::Pending,
                buy_quote_fee_reserve_limit: Decimal::ZERO,
                buy_quote_fee_reserve_consumed: Decimal::ZERO,
                cloid: cmd.cloid.clone(),
                created_at: cmd.logical_ts(),
                updated_at: cmd.logical_ts(),
                lsn: 0,
            },
            reduce_only: cmd.reduce_only,
            is_liquidation: cmd.liquidation_id.is_some(),
        };
        let updated_margin_balance =
            if matches!(intent, PlaceIntent::Open | PlaceIntent::Increase) && !cmd.reduce_only {
                let amount =
                    required_margin(cmd.size, cmd.execution.price(), state.position.leverage())?;
                let before = state.margin_balance.clone();
                let mut after = before.clone();
                after
                    .apply(BalanceV2Event::Freeze { amount, timestamp: cmd.logical_ts() })
                    .map_err(map_balance_error)?;
                Some(UpdatedEntityPair { before, after })
            } else {
                None
            };
        Ok(PlaceHyperliquidPerpOrderChanges { created_order, updated_margin_balance })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlaceIntent {
    Open,
    Increase,
    Reduce,
    Close,
}

fn derive_intent(
    side: OrderSide,
    size: Decimal,
    signed_position: Decimal,
) -> Result<PlaceIntent, PlaceHyperliquidPerpOrderError> {
    if signed_position.is_zero() {
        return Ok(PlaceIntent::Open);
    }
    let same_direction = matches!(side, OrderSide::Buy) == signed_position.is_sign_positive();
    if same_direction {
        return Ok(PlaceIntent::Increase);
    }
    match size.cmp(&signed_position.abs()) {
        std::cmp::Ordering::Less => Ok(PlaceIntent::Reduce),
        std::cmp::Ordering::Equal => Ok(PlaceIntent::Close),
        std::cmp::Ordering::Greater => Err(PlaceHyperliquidPerpOrderError::FlipOrderRequiresSplit),
    }
}

fn required_margin(
    size: Decimal,
    price: Decimal,
    leverage: Decimal,
) -> Result<Decimal, PlaceHyperliquidPerpOrderError> {
    if leverage <= Decimal::ZERO {
        return Err(PlaceHyperliquidPerpOrderError::ArithmeticOverflow);
    }
    size.checked_mul(price)
        .and_then(|value| value.checked_div(leverage))
        .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)
}

fn map_balance_error(error: BalanceV2Error) -> PlaceHyperliquidPerpOrderError {
    match error {
        BalanceV2Error::InsufficientAvailableBalance => {
            PlaceHyperliquidPerpOrderError::InsufficientMarginBalance
        }
        _ => PlaceHyperliquidPerpOrderError::ArithmeticOverflow,
    }
}
