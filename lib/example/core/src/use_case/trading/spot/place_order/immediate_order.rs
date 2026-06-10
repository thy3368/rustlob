use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;

use super::{
    PlaceOrderError, PlaceOrderSide, PlaceOrderTimeInForce, check_common_command, checked_price,
    checked_qty, validate_market_state,
};
use crate::MarketRules;
use crate::entity::{Balance, SpotOrder, SpotOrderExecution};

/// 立即执行单需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceImmediateOrderState {
    /// 当前市场是否接受立即下单。
    pub trading_enabled: bool,
    /// 用于生成稳定订单 ID 的下一个订单序号。
    pub next_order_sequence: u64,
    /// 下单账户 ID。
    pub account_id: String,
    /// base 资产余额快照。
    pub base_balance: Balance,
    /// quote 资产余额快照。
    pub quote_balance: Balance,
    /// 当前交易对规则快照。
    pub market_rules: MarketRules,
}

/// 立即执行现货订单的执行意图。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceImmediateOrderExecution {
    /// 限价单，直接对齐 Hyperliquid `orderType.limit.tif`。
    Limit {
        /// 委托价格，对齐 Hyperliquid `limitPx`。
        price: u64,
        /// Hyperliquid `orderType.limit.tif`：`Gtc` / `Ioc` / `Alo`。
        time_in_force: PlaceOrderTimeInForce,
    },
    /// 市价意图。Hyperliquid 普通下单 API 没有独立 `market` 类型，adapter 可映射成
    /// IOC + 激进限价；这里的价格用于冻结余额上限和 adapter 生成 `limitPx`。
    Market {
        /// 市价买入愿意接受的最高成交价格，用于冻结 quote 余额上限。
        aggressive_price: u64,
    },
}

impl PlaceImmediateOrderExecution {
    fn reserve_price(self) -> Result<u64, PlaceOrderError> {
        match self {
            Self::Limit { price, .. } => checked_price(price),
            Self::Market { aggressive_price } => checked_price(aggressive_price),
        }
    }

    const fn spot_execution(self) -> SpotOrderExecution {
        match self {
            Self::Limit { price, .. } => SpotOrderExecution::Limit { price },
            Self::Market { aggressive_price } => SpotOrderExecution::Market { aggressive_price },
        }
    }

    const fn stored_time_in_force(self) -> PlaceOrderTimeInForce {
        match self {
            Self::Limit { time_in_force, .. } => time_in_force,
            Self::Market { .. } => PlaceOrderTimeInForce::Ioc,
        }
    }
}

/// 创建立即执行现货订单的命令。
///
/// 立即执行单表达“现在进入执行流程”的业务动作。字段基本对齐 Hyperliquid
/// 单笔下单 API：`asset` / `is_buy` / `limitPx` / `size` / `reduce_only`
/// / `order_type.limit.tif` / `cloid`。市价单在 core 中显式表达为 `Market`，
/// adapter 可映射为 Hyperliquid 的 IOC 激进限价。`party_id` 是 core 用例的业务发起方，
/// `symbol` 是本示例用于市场规则和订单快照的内部交易对标识。
///
/// # Examples
///
/// ```
/// use example_core::{PlaceImmediateOrderCmd, PlaceImmediateOrderExecution, PlaceOrderTimeInForce};
///
/// let cmd = PlaceImmediateOrderCmd {
///     party_id: "trader-1".to_string(),
///     asset: 10_001,
///     symbol: "BTCUSDT".to_string(),
///     is_buy: true,
///     size: 2,
///     reduce_only: false,
///     execution: PlaceImmediateOrderExecution::Limit {
///         price: 100,
///         time_in_force: PlaceOrderTimeInForce::Gtc,
///     },
///     cloid: None,
/// };
///
/// assert_eq!(cmd.symbol, "BTCUSDT");
/// assert_eq!(cmd.size, 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceImmediateOrderCmd {
    /// 发起下单的交易账户 ID，也是立即买单冻结 quote 余额的账户。
    pub party_id: String,
    /// Hyperliquid 资产编号；现货通常使用 `10000 + spot index`。
    pub asset: u32,
    /// 交易对，例如 `BTCUSDT`。
    pub symbol: String,
    /// 是否买单，对齐 Hyperliquid `isBuy`。
    pub is_buy: bool,
    /// 以 base asset 计价的下单数量，例如买入多少 BTC。
    pub size: u64,
    /// 是否只减仓。现货立即单不支持该语义，必须为 `false`。
    pub reduce_only: bool,
    /// 执行意图：限价单或市价意图。
    pub execution: PlaceImmediateOrderExecution,
    /// 客户端自定义订单号，对齐 Hyperliquid `cloid`。
    pub cloid: Option<String>,
}

impl PlaceImmediateOrderCmd {
    fn qty(&self) -> Result<u64, PlaceOrderError> {
        checked_qty(self.size)
    }

    fn reserve_price(&self) -> Result<u64, PlaceOrderError> {
        self.execution.reserve_price()
    }

    fn side(&self) -> PlaceOrderSide {
        if self.is_buy { PlaceOrderSide::Buy } else { PlaceOrderSide::Sell }
    }
}

impl IssuedByParty for PlaceImmediateOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// Use case that accepts an immediate spot order and derives order/account events.
///
/// The use case itself is deterministic for the same command and loaded state. It does not talk to
/// storage, publish events, or shape HTTP replies.
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceImmediateOrderUseCase;

impl CommandUseCase2 for PlaceImmediateOrderUseCase {
    type Command = PlaceImmediateOrderCmd;
    type GivenState = PlaceImmediateOrderState;
    type Error = PlaceOrderError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        check_common_command(cmd.side(), cmd.size)?;
        if cmd.reduce_only {
            return Err(PlaceOrderError::UnsupportedReduceOnly);
        }
        let _ = cmd.reserve_price()?;
        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let qty = cmd.qty()?;
        let reserve_price = cmd.reserve_price()?;

        validate_market_state(
            cmd.party_id.as_str(),
            cmd.symbol.as_str(),
            qty,
            state.trading_enabled,
            state.account_id.as_str(),
            &state.market_rules,
        )?;

        let reserved_quote = state
            .market_rules
            .required_quote(qty, reserve_price)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;

        match cmd.side() {
            PlaceOrderSide::Buy => {
                if !state.quote_balance.can_reserve(reserved_quote) {
                    return Err(PlaceOrderError::InsufficientQuoteBalance);
                }
            }
            PlaceOrderSide::Sell => {
                if !state.base_balance.can_reserve(qty) {
                    return Err(PlaceOrderError::InsufficientBaseBalance);
                }
            }
        }

        Ok(())
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let qty = cmd.qty()?;
        let reserve_price = cmd.reserve_price()?;
        let notional_quote = state
            .market_rules
            .required_quote(qty, reserve_price)
            .ok_or(PlaceOrderError::ArithmeticOverflow)?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);
        let side = cmd.side();
        let (reserved_base, reserved_quote) = match side {
            PlaceOrderSide::Buy => (0, notional_quote),
            PlaceOrderSide::Sell => (qty, 0),
        };
        let order = SpotOrder::new(
            order_id,
            cmd.asset,
            None,
            cmd.party_id.clone(),
            cmd.symbol.clone(),
            side,
            cmd.execution.spot_execution(),
            cmd.execution.stored_time_in_force(),
            qty,
            reserved_base,
            reserved_quote,
            cmd.cloid.clone(),
        );
        let order_event =
            order.track_create_event().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

        let tracked_balance_event = match side {
            PlaceOrderSide::Buy => {
                let mut next_balance = state.quote_balance.clone();
                let (next_available, next_frozen) = state
                    .quote_balance
                    .reserve_after(reserved_quote)
                    .ok_or(PlaceOrderError::ArithmeticOverflow)?;
                let next_version = state
                    .quote_balance
                    .version
                    .checked_add(1)
                    .ok_or(PlaceOrderError::ArithmeticOverflow)?;
                next_balance
                    .track_update_event(|balance| {
                        balance.apply_after(next_available, next_frozen, next_version);
                    })
                    .map_err(|_| PlaceOrderError::ArithmeticOverflow)?
            }
            PlaceOrderSide::Sell => {
                let mut next_balance = state.base_balance.clone();
                let (next_available, next_frozen) = state
                    .base_balance
                    .reserve_after(reserved_base)
                    .ok_or(PlaceOrderError::ArithmeticOverflow)?;
                let next_version = state
                    .base_balance
                    .version
                    .checked_add(1)
                    .ok_or(PlaceOrderError::ArithmeticOverflow)?;
                next_balance
                    .track_update_event(|balance| {
                        balance.apply_after(next_available, next_frozen, next_version);
                    })
                    .map_err(|_| PlaceOrderError::ArithmeticOverflow)?
            }
        };

        Ok(vec![order_event, tracked_balance_event])
    }
}

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod command_examples;

#[cfg(test)]
mod happy_path;

#[cfg(test)]
mod unhappy_path;
