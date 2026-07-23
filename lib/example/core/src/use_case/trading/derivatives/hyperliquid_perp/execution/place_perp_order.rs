use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{
    EventProjectError, IssuedByParty, ReplayableChanges, UpdatedEntityPair,
};
use common_entity::{Entity, MiStateMachineV2Unchecked};
use thiserror::Error;

use crate::MarketRules;
use crate::entity::{
    Balance, BalanceLedgerEntryV2, HyperliquidPerpOrder, HyperliquidPerpOrderExecution,
    HyperliquidPerpOrderSide, HyperliquidPerpOrderTimeInForce, HyperliquidPerpPosition,
    PlaceHyperliquidPerpOrderInput, PlaceHyperliquidPerpOrderIntent, required_position_margin,
};

/// Hyperliquid perp 下单可能返回的业务错误。
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PlaceHyperliquidPerpOrderError {
    /// 业务发起方不能为空。
    #[error("party_id must not be empty")]
    InvalidPartyId,
    /// 合约展示名不能为空。
    #[error("symbol must not be empty")]
    InvalidSymbol,
    /// 数量必须大于零。
    #[error("size must be greater than zero")]
    InvalidSize,
    /// 下单或保证金估算价格必须大于零。
    #[error("price must be greater than zero")]
    InvalidPrice,
    /// 市场当前不接受下单。
    #[error("trading is disabled")]
    TradingDisabled,
    /// command 账户和已加载账户状态不一致。
    #[error("account snapshot does not belong to command party")]
    AccountMismatch,
    /// 当前市场规则不支持该合约。
    #[error("symbol is not tradable in current market rules")]
    SymbolNotTradable,
    /// 下单数量低于市场最小数量。
    #[error("size is below market minimum")]
    SizeBelowMin,
    /// 保证金余额不是当前账户的 Cross 保证金币种余额。
    #[error("margin balance must be current account cross margin balance")]
    InvalidMarginBalance,
    /// 仓位账户、asset 或 symbol 与命令不一致。
    #[error("position does not match command account or market")]
    PositionMismatch,
    /// 仓位状态、数量、均价或保证金不一致。
    #[error("position state is inconsistent")]
    InconsistentPositionState,
    /// 仓位杠杆必须至少为 1。
    #[error("position leverage must be greater than or equal to one")]
    InvalidLeverage,
    /// 可用 Cross 保证金不足以冻结开仓保证金。
    #[error("insufficient cross margin balance")]
    InsufficientMarginBalance,
    /// reduce-only 订单不能增加或反向超过当前净仓位。
    #[error("invalid reduce-only order")]
    InvalidReduceOnly,
    /// 反手订单必须由上层拆成平仓订单和开仓订单。
    #[error("flip order must be split into close and open orders")]
    FlipOrderRequiresSplit,
    /// 推导订单或余额事件时发生算术溢出。
    #[error("arithmetic overflow while deriving business result")]
    ArithmeticOverflow,
}

/// 创建 Hyperliquid perp 订单的执行意图。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceHyperliquidPerpOrderExecution {
    /// Hyperliquid 限价订单。
    Limit {
        /// 委托价格。
        price: u64,
        /// Hyperliquid `tif`。
        time_in_force: HyperliquidPerpOrderTimeInForce,
    },
    /// 市价意图，adapter 可映射为 IOC + 激进限价。
    Market {
        /// 用于计算保证金冻结上限和撮合价格比较的激进价格。
        aggressive_price: u64,
    },
}

impl PlaceHyperliquidPerpOrderExecution {
    fn margin_price(self) -> Result<u64, PlaceHyperliquidPerpOrderError> {
        let price = match self {
            Self::Limit { price, .. } => price,
            Self::Market { aggressive_price } => aggressive_price,
        };
        if price == 0 {
            return Err(PlaceHyperliquidPerpOrderError::InvalidPrice);
        }
        Ok(price)
    }

    const fn stored_execution(self) -> HyperliquidPerpOrderExecution {
        match self {
            Self::Limit { price, .. } => HyperliquidPerpOrderExecution::Limit { price },
            Self::Market { aggressive_price } => {
                HyperliquidPerpOrderExecution::Market { aggressive_price }
            }
        }
    }

    const fn stored_time_in_force(self) -> HyperliquidPerpOrderTimeInForce {
        match self {
            Self::Limit { time_in_force, .. } => time_in_force,
            Self::Market { .. } => HyperliquidPerpOrderTimeInForce::Ioc,
        }
    }
}

/// 创建 Hyperliquid perp 订单的命令。
///
/// 字段对齐 Hyperliquid order API 第一版核心语义：`asset`、`isBuy`、`sz`、
/// `reduceOnly`、`limitPx` / aggressive price、`tif` 和 `cloid`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderCmd {
    /// 发起下单的交易账户 ID。
    pub party_id: String,
    /// Hyperliquid perp asset 编号。
    pub asset: u32,
    /// 合约展示名，例如 `BTC-PERP`。
    pub symbol: String,
    /// `true` 表示买入，`false` 表示卖出。
    pub is_buy: bool,
    /// 合约数量。
    pub size: u64,
    /// 是否只减仓。
    pub reduce_only: bool,
    /// 执行意图。
    pub execution: PlaceHyperliquidPerpOrderExecution,
    /// Hyperliquid `cloid`，客户端自定义订单 ID。
    pub cloid: Option<String>,
}

impl PlaceHyperliquidPerpOrderCmd {
    fn checked_size(&self) -> Result<u64, PlaceHyperliquidPerpOrderError> {
        if self.size == 0 {
            return Err(PlaceHyperliquidPerpOrderError::InvalidSize);
        }
        Ok(self.size)
    }

    const fn side(&self) -> HyperliquidPerpOrderSide {
        if self.is_buy { HyperliquidPerpOrderSide::Buy } else { HyperliquidPerpOrderSide::Sell }
    }
}

impl IssuedByParty for PlaceHyperliquidPerpOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// 创建 Hyperliquid perp 订单需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderState {
    /// 当前市场是否接受下单。
    pub trading_enabled: bool,
    /// 用于生成稳定订单 ID 的下一个订单序号。
    pub next_order_sequence: u64,
    /// 下单账户 ID。
    pub account_id: String,
    /// Cross 保证金余额快照。
    pub margin_balance: Balance,
    /// Cross 保证金币种，例如 `USDC`。
    pub margin_asset_id: String,
    /// 当前合约市场规则快照。
    pub market_rules: MarketRules,
    /// 当前账户在该 Hyperliquid perp asset 上的净仓位快照。
    pub position: HyperliquidPerpPosition,
}

/// 接受 Hyperliquid perp 订单，并在非 reduce-only 时冻结 Cross 保证金。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceHyperliquidPerpOrderUseCase;

/// 创建 Hyperliquid perp 订单后的业务 changes。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceHyperliquidPerpOrderChanges {
    /// 本次新创建的订单事实。
    pub created_order: HyperliquidPerpOrder,
    /// 本次新创建并已补齐 before/after 快照的余额冻结流水；无需冻结保证金时为空。
    pub created_balance_ledger_entry: Option<BalanceLedgerEntryV2>,
    /// 本次实际受影响的保证金余额 before/after；reduce-only 或无需新增保证金时为空。
    pub updated_margin_balances: Vec<UpdatedEntityPair<Balance>>,
}

impl ReplayableChanges for PlaceHyperliquidPerpOrderChanges {
    fn to_replayable_events(&self) -> Result<Vec<EntityReplayableEvent>, EventProjectError> {
        let mut events = Vec::with_capacity(
            1 + self.updated_margin_balances.len()
                + usize::from(self.created_balance_ledger_entry.is_some()),
        );
        events.push(self.created_order.track_create_event()?);
        if let Some(ledger_entry) = &self.created_balance_ledger_entry {
            events.push(ledger_entry.track_create_event()?);
        }
        for balance in &self.updated_margin_balances {
            events.push(balance.after.track_update_event_from(&balance.before)?);
        }
        Ok(events)
    }
}

impl MiStateMachineV2Unchecked for PlaceHyperliquidPerpOrderUseCase {
    type Command = PlaceHyperliquidPerpOrderCmd;
    type GivenState = PlaceHyperliquidPerpOrderState;
    type Error = PlaceHyperliquidPerpOrderError;
    type AfterChanges = PlaceHyperliquidPerpOrderChanges;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.party_id.is_empty() {
            return Err(PlaceHyperliquidPerpOrderError::InvalidPartyId);
        }
        if cmd.symbol.is_empty() {
            return Err(PlaceHyperliquidPerpOrderError::InvalidSymbol);
        }
        let _ = cmd.checked_size()?;
        let _ = cmd.execution.margin_price()?;
        Ok(())
    }

    fn validate_against_given_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let size = cmd.checked_size()?;
        let price = cmd.execution.margin_price()?;

        if !state.trading_enabled {
            return Err(PlaceHyperliquidPerpOrderError::TradingDisabled);
        }
        if state.account_id != cmd.party_id {
            return Err(PlaceHyperliquidPerpOrderError::AccountMismatch);
        }
        if !state.market_rules.supports_symbol(cmd.symbol.as_str()) {
            return Err(PlaceHyperliquidPerpOrderError::SymbolNotTradable);
        }
        if !state.market_rules.validate_qty(size) {
            return Err(PlaceHyperliquidPerpOrderError::SizeBelowMin);
        }
        if !state.margin_balance.belongs_to_account(state.account_id.as_str())
            || !state.margin_balance.is_asset(state.margin_asset_id.as_str())
        {
            return Err(PlaceHyperliquidPerpOrderError::InvalidMarginBalance);
        }
        if !state.position.belongs_to_account(state.account_id.as_str())
            || !state.position.trades_asset(cmd.asset)
            || !state.position.trades_symbol(cmd.symbol.as_str())
        {
            return Err(PlaceHyperliquidPerpOrderError::PositionMismatch);
        }
        if !state.position.has_consistent_state() {
            return Err(PlaceHyperliquidPerpOrderError::InconsistentPositionState);
        }
        if state.position.leverage_value == 0 {
            return Err(PlaceHyperliquidPerpOrderError::InvalidLeverage);
        }
        if cmd.reduce_only {
            validate_reduce_only(cmd, &state.position)?;
            return Ok(());
        }

        match derive_place_intent(cmd.side(), size, &state.position)? {
            DerivedPerpOrderIntent::Open => {
                let required_margin =
                    required_position_margin(size, price, state.position.leverage_value)
                        .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;
                if !state.margin_balance.can_reserve(required_margin) {
                    return Err(PlaceHyperliquidPerpOrderError::InsufficientMarginBalance);
                }
            }
            DerivedPerpOrderIntent::Close => {}
        }

        Ok(())
    }

    fn compute_after_changes_unchecked(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<Self::AfterChanges, Self::Error> {
        let size = cmd.checked_size()?;
        let price = cmd.execution.margin_price()?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);
        let intent = if cmd.reduce_only {
            PlaceHyperliquidPerpOrderIntent::Close
        } else {
            match derive_place_intent(cmd.side(), size, &state.position)? {
                DerivedPerpOrderIntent::Close => PlaceHyperliquidPerpOrderIntent::Close,
                DerivedPerpOrderIntent::Open => {
                    let required_margin =
                        required_position_margin(size, price, state.position.leverage_value)
                            .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;
                    PlaceHyperliquidPerpOrderIntent::Open {
                        margin_asset_id: state.margin_asset_id.clone(),
                        margin_balance_entity_id: state.margin_balance.entity_id(),
                        margin_amount: required_margin,
                    }
                }
            }
        };

        let place_outcome = HyperliquidPerpOrder::place(PlaceHyperliquidPerpOrderInput {
            order_id,
            asset: cmd.asset,
            account_id: state.account_id.clone(),
            symbol: cmd.symbol.clone(),
            side: cmd.side(),
            execution: cmd.execution.stored_execution(),
            time_in_force: cmd.execution.stored_time_in_force(),
            qty: size,
            client_order_id: cmd.cloid.clone(),
            liquidation_id: None,
            intent,
        })
        .map_err(|_| PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;

        let created_order = place_outcome.order;
        if created_order.reservation.is_none() {
            return Ok(PlaceHyperliquidPerpOrderChanges {
                created_order,
                created_balance_ledger_entry: None,
                updated_margin_balances: Vec::new(),
            });
        }
        let previous_balance = state.margin_balance.clone();
        let mut next_balance = previous_balance.clone();
        let mut freeze_ledger_entry = place_outcome
            .freeze_ledger_entry
            .ok_or(PlaceHyperliquidPerpOrderError::ArithmeticOverflow)?;
        freeze_ledger_entry.apply_to(&mut next_balance).map_err(map_margin_ledger_error)?;

        Ok(PlaceHyperliquidPerpOrderChanges {
            created_order,
            created_balance_ledger_entry: Some(freeze_ledger_entry),
            updated_margin_balances: vec![UpdatedEntityPair {
                before: previous_balance,
                after: next_balance,
            }],
        })
    }
}

fn map_margin_ledger_error(
    error: crate::entity::BalanceLedgerEntryV2Error,
) -> PlaceHyperliquidPerpOrderError {
    match error {
        crate::entity::BalanceLedgerEntryV2Error::InsufficientAvailableBalance => {
            PlaceHyperliquidPerpOrderError::InsufficientMarginBalance
        }
        crate::entity::BalanceLedgerEntryV2Error::InvalidAmount
        | crate::entity::BalanceLedgerEntryV2Error::InsufficientFrozenBalance
        | crate::entity::BalanceLedgerEntryV2Error::ArithmeticOverflow
        | crate::entity::BalanceLedgerEntryV2Error::BalanceIdentityMismatch
        | crate::entity::BalanceLedgerEntryV2Error::AlreadyApplied => {
            PlaceHyperliquidPerpOrderError::ArithmeticOverflow
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DerivedPerpOrderIntent {
    Open,
    Close,
}

fn derive_place_intent(
    order_side: HyperliquidPerpOrderSide,
    size: u64,
    position: &HyperliquidPerpPosition,
) -> Result<DerivedPerpOrderIntent, PlaceHyperliquidPerpOrderError> {
    if position.is_flat()
        || (order_side == HyperliquidPerpOrderSide::Buy && position.is_long())
        || (order_side == HyperliquidPerpOrderSide::Sell && position.is_short())
    {
        Ok(DerivedPerpOrderIntent::Open)
    } else {
        if (order_side == HyperliquidPerpOrderSide::Buy && position.is_short())
            || (order_side == HyperliquidPerpOrderSide::Sell && position.is_long())
        {
            if size > position.qty() {
                Err(PlaceHyperliquidPerpOrderError::FlipOrderRequiresSplit)
            } else {
                Ok(DerivedPerpOrderIntent::Close)
            }
        } else {
            Err(PlaceHyperliquidPerpOrderError::InconsistentPositionState)
        }
    }
}

fn validate_reduce_only(
    cmd: &PlaceHyperliquidPerpOrderCmd,
    position: &HyperliquidPerpPosition,
) -> Result<(), PlaceHyperliquidPerpOrderError> {
    let reduces_position = (cmd.side() == HyperliquidPerpOrderSide::Buy && position.is_short())
        || (cmd.side() == HyperliquidPerpOrderSide::Sell && position.is_long());
    if !reduces_position || cmd.size > position.qty() {
        return Err(PlaceHyperliquidPerpOrderError::InvalidReduceOnly);
    }
    Ok(())
}

#[cfg(test)]
mod compute_replayable_events_happy_path;
