use common_entity::MiStateMachineV2Unchecked;

use super::test_support::sample_cmd;
use super::*;

const EXAMPLE_TRIGGER_LIMIT_PRICE: u64 = 88;

/// 当前条件单 command 的成功业务组合。
///
/// 参考 Hyperliquid `t.trigger`：条件单仍然有买卖方向，触发角色包含止盈/止损，
/// 触发后可以是市价或限价，且可选 `cloid`。这里把这些业务面枚举出来，便于检查
/// command 设计是否覆盖下单 API 的核心场景。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConditionalCommandExample {
    /// 买入止损条件单，触发后按市价意图执行，不带客户端订单号。
    BuyStopLossMarketWithoutCloid,
    /// 买入止损条件单，触发后按市价意图执行，带客户端订单号。
    BuyStopLossMarketWithCloid,
    /// 买入止损条件单，触发后按限价执行，不带客户端订单号。
    BuyStopLossLimitWithoutCloid,
    /// 买入止损条件单，触发后按限价执行，带客户端订单号。
    BuyStopLossLimitWithCloid,
    /// 买入止盈条件单，触发后按市价意图执行，不带客户端订单号。
    BuyTakeProfitMarketWithoutCloid,
    /// 买入止盈条件单，触发后按市价意图执行，带客户端订单号。
    BuyTakeProfitMarketWithCloid,
    /// 买入止盈条件单，触发后按限价执行，不带客户端订单号。
    BuyTakeProfitLimitWithoutCloid,
    /// 买入止盈条件单，触发后按限价执行，带客户端订单号。
    BuyTakeProfitLimitWithCloid,
    /// 卖出止损条件单，触发后按市价意图执行，不带客户端订单号。
    SellStopLossMarketWithoutCloid,
    /// 卖出止损条件单，触发后按市价意图执行，带客户端订单号。
    SellStopLossMarketWithCloid,
    /// 卖出止损条件单，触发后按限价执行，不带客户端订单号。
    SellStopLossLimitWithoutCloid,
    /// 卖出止损条件单，触发后按限价执行，带客户端订单号。
    SellStopLossLimitWithCloid,
    /// 卖出止盈条件单，触发后按市价意图执行，不带客户端订单号。
    SellTakeProfitMarketWithoutCloid,
    /// 卖出止盈条件单，触发后按市价意图执行，带客户端订单号。
    SellTakeProfitMarketWithCloid,
    /// 卖出止盈条件单，触发后按限价执行，不带客户端订单号。
    SellTakeProfitLimitWithoutCloid,
    /// 卖出止盈条件单，触发后按限价执行，带客户端订单号。
    SellTakeProfitLimitWithCloid,
}

impl ConditionalCommandExample {
    const ALL: [Self; 16] = [
        Self::BuyStopLossMarketWithoutCloid,
        Self::BuyStopLossMarketWithCloid,
        Self::BuyStopLossLimitWithoutCloid,
        Self::BuyStopLossLimitWithCloid,
        Self::BuyTakeProfitMarketWithoutCloid,
        Self::BuyTakeProfitMarketWithCloid,
        Self::BuyTakeProfitLimitWithoutCloid,
        Self::BuyTakeProfitLimitWithCloid,
        Self::SellStopLossMarketWithoutCloid,
        Self::SellStopLossMarketWithCloid,
        Self::SellStopLossLimitWithoutCloid,
        Self::SellStopLossLimitWithCloid,
        Self::SellTakeProfitMarketWithoutCloid,
        Self::SellTakeProfitMarketWithCloid,
        Self::SellTakeProfitLimitWithoutCloid,
        Self::SellTakeProfitLimitWithCloid,
    ];

    const fn expected_side(self) -> PlaceOrderSide {
        match self {
            Self::BuyStopLossMarketWithoutCloid
            | Self::BuyStopLossMarketWithCloid
            | Self::BuyStopLossLimitWithoutCloid
            | Self::BuyStopLossLimitWithCloid
            | Self::BuyTakeProfitMarketWithoutCloid
            | Self::BuyTakeProfitMarketWithCloid
            | Self::BuyTakeProfitLimitWithoutCloid
            | Self::BuyTakeProfitLimitWithCloid => PlaceOrderSide::Buy,
            Self::SellStopLossMarketWithoutCloid
            | Self::SellStopLossMarketWithCloid
            | Self::SellStopLossLimitWithoutCloid
            | Self::SellStopLossLimitWithCloid
            | Self::SellTakeProfitMarketWithoutCloid
            | Self::SellTakeProfitMarketWithCloid
            | Self::SellTakeProfitLimitWithoutCloid
            | Self::SellTakeProfitLimitWithCloid => PlaceOrderSide::Sell,
        }
    }

    const fn expected_trigger_role(self) -> PlaceOrderTriggerRole {
        match self {
            Self::BuyStopLossMarketWithoutCloid
            | Self::BuyStopLossMarketWithCloid
            | Self::BuyStopLossLimitWithoutCloid
            | Self::BuyStopLossLimitWithCloid
            | Self::SellStopLossMarketWithoutCloid
            | Self::SellStopLossMarketWithCloid
            | Self::SellStopLossLimitWithoutCloid
            | Self::SellStopLossLimitWithCloid => PlaceOrderTriggerRole::StopLoss,
            Self::BuyTakeProfitMarketWithoutCloid
            | Self::BuyTakeProfitMarketWithCloid
            | Self::BuyTakeProfitLimitWithoutCloid
            | Self::BuyTakeProfitLimitWithCloid
            | Self::SellTakeProfitMarketWithoutCloid
            | Self::SellTakeProfitMarketWithCloid
            | Self::SellTakeProfitLimitWithoutCloid
            | Self::SellTakeProfitLimitWithCloid => PlaceOrderTriggerRole::TakeProfit,
        }
    }

    const fn is_limit(self) -> bool {
        matches!(
            self,
            Self::BuyStopLossLimitWithoutCloid
                | Self::BuyStopLossLimitWithCloid
                | Self::BuyTakeProfitLimitWithoutCloid
                | Self::BuyTakeProfitLimitWithCloid
                | Self::SellStopLossLimitWithoutCloid
                | Self::SellStopLossLimitWithCloid
                | Self::SellTakeProfitLimitWithoutCloid
                | Self::SellTakeProfitLimitWithCloid
        )
    }

    const fn has_cloid(self) -> bool {
        matches!(
            self,
            Self::BuyStopLossMarketWithCloid
                | Self::BuyStopLossLimitWithCloid
                | Self::BuyTakeProfitMarketWithCloid
                | Self::BuyTakeProfitLimitWithCloid
                | Self::SellStopLossMarketWithCloid
                | Self::SellStopLossLimitWithCloid
                | Self::SellTakeProfitMarketWithCloid
                | Self::SellTakeProfitLimitWithCloid
        )
    }

    fn command_from(self, base_cmd: PlaceConditionalOrderCmd) -> PlaceConditionalOrderCmd {
        let execution = if self.is_limit() {
            PlaceOrderExecution::Limit { price: EXAMPLE_TRIGGER_LIMIT_PRICE }
        } else {
            PlaceOrderExecution::Market { aggressive_price: 95 }
        };

        PlaceConditionalOrderCmd {
            side: self.expected_side(),
            trigger_role: self.expected_trigger_role(),
            execution,
            client_order_id: self
                .has_cloid()
                .then(|| "0123456789abcdef0123456789abcdef".to_string()),
            ..base_cmd
        }
    }
}

fn supported_command_examples(
    base_cmd: PlaceConditionalOrderCmd,
) -> Vec<(ConditionalCommandExample, PlaceConditionalOrderCmd)> {
    // 覆盖充分性判断：
    // 条件单支持 Buy/Sell × StopLoss/TakeProfit × Market/Limit × cloid 有无。
    // 当前矩阵共 16 种，是对 Hyperliquid trigger order 核心 command 场景的覆盖检查。
    ConditionalCommandExample::ALL
        .into_iter()
        .map(|example| (example, example.command_from(base_cmd.clone())))
        .collect()
}

#[test]
fn supported_command_examples_cover_current_business_matrix() {
    let examples = supported_command_examples(sample_cmd());

    assert_eq!(examples.len(), 16);
    for scenario in ConditionalCommandExample::ALL {
        assert!(examples.iter().any(|(example, _)| *example == scenario));
    }
}

#[test]
fn supported_command_examples_are_accepted_by_pre_check() {
    let use_case = PlaceConditionalOrderUseCase;

    for (example, cmd) in supported_command_examples(sample_cmd()) {
        assert_eq!(
            use_case.pre_check_command(&cmd),
            Ok(()),
            "conditional command example should be accepted: {example:?}",
        );
        assert_eq!(cmd.side, example.expected_side());
        assert_eq!(cmd.trigger_role, example.expected_trigger_role());
        assert_eq!(cmd.client_order_id.is_some(), example.has_cloid());
    }
}

#[test]
fn print_supported_command_examples() {
    for (example, cmd) in supported_command_examples(sample_cmd()) {
        println!(
            "{example:?}: asset={}, symbol={}, side={:?}, trigger_price={}, trigger_role={:?}, execution={:?}, qty={}, cloid={:?}",
            cmd.asset,
            cmd.symbol,
            cmd.side,
            cmd.trigger_price,
            cmd.trigger_role,
            cmd.execution,
            cmd.quantity,
            cmd.client_order_id,
        );
    }
}
