use cmd_handler::EntityReplayableEvent;
use cmd_handler::command_use_case_def2::{CommandUseCase2, IssuedByParty};
use common_entity::Entity;

use super::{
    PlaceOrderError, PlaceOrderExecution, PlaceOrderSide, PlaceOrderTriggerRole,
    check_common_command, checked_qty, limit_execution_price, validate_market_state,
};
use crate::MarketRules;
use crate::entity::{SpotConditionalOrder, SpotOrderExecution, SpotOrderTimeInForce};

/// 条件单创建需要的已加载业务状态。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceConditionalOrderState {
    /// 当前市场是否接受条件单创建。
    pub trading_enabled: bool,
    /// 用于生成稳定订单 ID 的下一个订单序号。
    pub next_order_sequence: u64,
    /// 下单账户 ID。创建条件单时不冻结资金，但账户归属仍是业务事实。
    pub account_id: String,
    /// 当前交易对规则快照。
    pub market_rules: MarketRules,
}

/// 创建条件现货订单的命令。
///
/// 条件单表达“先登记触发规则，触发后再进入执行流程”的业务动作。创建时不冻结资金；
/// 触发时再根据触发后的市价/限价执行方式做余额校验和冻结。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaceConditionalOrderCmd {
    /// 发起下单的交易账户 ID。
    pub party_id: String,
    /// Hyperliquid 资产编号；现货通常使用 `10000 + spot index`。
    pub asset: u32,
    /// 交易对，例如 `BTCUSDT`。
    pub symbol: String,
    /// 订单方向。当前示例用例只处理买单。
    pub side: PlaceOrderSide,
    /// 以 base asset 计价的下单数量，例如买入多少 BTC。
    pub quantity: u64,
    /// 满足该价格后进入触发执行流程。
    pub trigger_price: u64,
    /// 条件单角色，止盈或止损。
    pub trigger_role: PlaceOrderTriggerRole,
    /// 触发后的执行方式，市价意图或限价意图。
    pub execution: PlaceOrderExecution,
    /// 客户端自定义订单 ID，可由 adapter 映射为 Hyperliquid `cloid`。
    pub client_order_id: Option<String>,
}

impl PlaceConditionalOrderCmd {
    fn qty(&self) -> Result<u64, PlaceOrderError> {
        checked_qty(self.quantity)
    }

    fn validate_execution(&self) -> Result<(), PlaceOrderError> {
        let _ = limit_execution_price(self.execution)?;
        Ok(())
    }

    fn spot_execution(&self) -> SpotOrderExecution {
        match self.execution {
            PlaceOrderExecution::Market { aggressive_price } => {
                SpotOrderExecution::Market { aggressive_price }
            }
            PlaceOrderExecution::Limit { price } => SpotOrderExecution::Limit { price },
        }
    }

    fn triggered_time_in_force(&self) -> SpotOrderTimeInForce {
        match self.execution {
            PlaceOrderExecution::Market { .. } => SpotOrderTimeInForce::Ioc,
            PlaceOrderExecution::Limit { .. } => SpotOrderTimeInForce::Gtc,
        }
    }
}

impl IssuedByParty for PlaceConditionalOrderCmd {
    fn party_id(&self) -> Option<&str> {
        Some(self.party_id.as_str())
    }
}

/// Use case that creates a conditional spot order.
///
/// 条件单创建时不冻结资金。它只保存触发条件和触发后的执行意图；触发时再进入执行流程，
/// 根据当时账户余额、市场规则和成交保护规则决定是否冻结和成交。
#[derive(Debug, Clone, Copy, Default)]
pub struct PlaceConditionalOrderUseCase;

impl CommandUseCase2 for PlaceConditionalOrderUseCase {
    type Command = PlaceConditionalOrderCmd;
    type GivenState = PlaceConditionalOrderState;
    type Error = PlaceOrderError;

    fn role(&self) -> &'static str {
        "Trader"
    }

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        check_common_command(cmd.side, cmd.quantity)?;

        if cmd.trigger_price == 0 {
            return Err(PlaceOrderError::InvalidTriggerPrice);
        }

        cmd.validate_execution()?;

        Ok(())
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        let qty = cmd.qty()?;
        validate_market_state(
            cmd.party_id.as_str(),
            cmd.symbol.as_str(),
            qty,
            state.trading_enabled,
            state.account_id.as_str(),
            &state.market_rules,
        )
    }

    fn compute_replayable_events(
        &self,
        cmd: &Self::Command,
        state: Self::GivenState,
    ) -> Result<Vec<EntityReplayableEvent>, Self::Error> {
        let qty = cmd.qty()?;
        let order_id = format!("{}-{}-{}", cmd.party_id, cmd.symbol, state.next_order_sequence);

        let order = SpotConditionalOrder::new(
            order_id,
            cmd.asset,
            None,
            cmd.party_id.clone(),
            cmd.symbol.clone(),
            cmd.side,
            cmd.trigger_price,
            cmd.trigger_role,
            cmd.spot_execution(),
            cmd.triggered_time_in_force(),
            qty,
            cmd.client_order_id.clone(),
        );
        let order_event =
            order.track_create_event().map_err(|_| PlaceOrderError::ArithmeticOverflow)?;

        Ok(vec![order_event])
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
