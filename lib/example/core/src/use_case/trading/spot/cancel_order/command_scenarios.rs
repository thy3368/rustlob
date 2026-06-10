use cmd_handler::command_use_case_def2::CommandUseCase2;
use proptest::prelude::*;

use super::*;

/// 撤单命令本身的场景枚举。
///
/// 这里刻意只覆盖不需要加载订单状态的便宜校验：
/// - 发起方不能为空；
/// - Hyperliquid spot asset 必须大于等于 10000；
/// - OID 必须是正数；
/// - 多个字段同时非法时，错误顺序必须稳定。
#[derive(Debug, Clone)]
enum CancelSpotOrderCommandScenario {
    /// 合法的 Hyperliquid spot 撤单命令，对应 `{"a": asset, "o": oid}`。
    ValidSpotCancel { asset: u32, order_id: u64 },
    /// 业务发起方为空，adapter 没有把签名地址或会话身份映射成账户。
    EmptyPartyId { asset: u32, order_id: u64 },
    /// asset 落在 perp/非法区间，不是 Hyperliquid spot asset 编号。
    NonSpotAsset { asset: u32, order_id: u64 },
    /// OID 为 0，不能定位交易所订单。
    ZeroOrderId { asset: u32 },
    /// asset 和 OID 同时非法，用来固定错误优先级。
    NonSpotAssetAndZeroOrderId { asset: u32 },
}

impl CancelSpotOrderCommandScenario {
    /// 把业务场景转换成实际 use case command。
    fn cmd(&self) -> CancelSpotOrderCmd {
        match self {
            Self::ValidSpotCancel { asset, order_id } | Self::NonSpotAsset { asset, order_id } => {
                CancelSpotOrderCmd {
                    party_id: "trader-1".to_string(),
                    asset: *asset,
                    order_id: *order_id,
                }
            }
            Self::EmptyPartyId { asset, order_id } => {
                CancelSpotOrderCmd { party_id: String::new(), asset: *asset, order_id: *order_id }
            }
            Self::ZeroOrderId { asset } | Self::NonSpotAssetAndZeroOrderId { asset } => {
                CancelSpotOrderCmd { party_id: "trader-1".to_string(), asset: *asset, order_id: 0 }
            }
        }
    }

    /// 每个 command-only 场景期望的预检查结果。
    fn expected_pre_check(&self) -> Result<(), CancelSpotOrderError> {
        match self {
            Self::ValidSpotCancel { .. } => Ok(()),
            Self::EmptyPartyId { .. } => Err(CancelSpotOrderError::InvalidPartyId),
            Self::NonSpotAsset { .. } => Err(CancelSpotOrderError::InvalidSpotAsset),
            Self::ZeroOrderId { .. } => Err(CancelSpotOrderError::InvalidOrderId),
            Self::NonSpotAssetAndZeroOrderId { .. } => Err(CancelSpotOrderError::InvalidSpotAsset),
        }
    }
}

/// 枚举 command 层的有效和无效输入空间。
///
/// 状态相关规则，例如订单是否存在、订单归属、冻结余额是否足够，
/// 放在 `given_state_scenarios.rs`，避免 command 测试承担 adapter/load state 职责。
fn command_scenario_strategy() -> impl Strategy<Value = CancelSpotOrderCommandScenario> {
    prop_oneof![
        (10_000_u32..=20_000, 1_u64..=u64::MAX).prop_map(|(asset, order_id)| {
            CancelSpotOrderCommandScenario::ValidSpotCancel { asset, order_id }
        }),
        (10_000_u32..=20_000, 1_u64..=u64::MAX).prop_map(|(asset, order_id)| {
            CancelSpotOrderCommandScenario::EmptyPartyId { asset, order_id }
        }),
        (0_u32..10_000, 1_u64..=u64::MAX).prop_map(|(asset, order_id)| {
            CancelSpotOrderCommandScenario::NonSpotAsset { asset, order_id }
        }),
        (10_000_u32..=20_000)
            .prop_map(|asset| { CancelSpotOrderCommandScenario::ZeroOrderId { asset } }),
        (0_u32..10_000).prop_map(|asset| {
            CancelSpotOrderCommandScenario::NonSpotAssetAndZeroOrderId { asset }
        }),
    ]
}

proptest! {
    #[test]
    fn command_scenarios_are_rejected_before_state_load(
        scenario in command_scenario_strategy(),
    ) {
        let use_case = CancelSpotOrderUseCase;
        let result = use_case.pre_check_command(&scenario.cmd());

        prop_assert_eq!(result, scenario.expected_pre_check());
    }
}
