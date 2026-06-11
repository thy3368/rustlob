use cmd_handler::command_use_case_def2::CommandUseCase2;

use super::test_support::sample_cmd;
use super::*;

const EXAMPLE_MARKET_AGGRESSIVE_PRICE: u64 = 101;

/// 当前立即单 command 的成功业务组合。
///
/// 这个枚举不是为了测试 Rust enum 本身，而是把 use case 目前承诺支持的 command
/// 业务面固定下来。写 proptest 时应优先复用这里的场景矩阵：如果新增市价单、卖单、
/// 新的 time-in-force 或其他成功业务形态，需要先扩展这里，再让 happy path 验证
/// `compute_replayable_events` 是否完整表达这些业务事实。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ImmediateCommandExample {
    /// 限价 GTC，不带客户端订单号。
    BuyLimitGtcWithoutCloid,
    /// 限价 GTC，带客户端订单号。
    BuyLimitGtcWithCloid,
    /// 限价 IOC，不带客户端订单号。
    BuyLimitIocWithoutCloid,
    /// 限价 IOC，带客户端订单号。
    BuyLimitIocWithCloid,
    /// 限价 ALO，不带客户端订单号。
    BuyLimitAloWithoutCloid,
    /// 限价 ALO，带客户端订单号。
    BuyLimitAloWithCloid,
    /// 市价意图，不带客户端订单号。
    ///
    /// Hyperliquid 普通市价单可由 IOC + aggressive limitPx 表达；
    /// 这里用 `aggressive_price` 记录买方愿意接受的最高成交价。
    BuyMarketWithoutCloid,
    /// 市价意图，带客户端订单号。
    ///
    /// 与 `MarketWithoutCloid` 相同，只是额外携带客户端订单号。
    BuyMarketWithCloid,
    /// 卖出限价 GTC，不带客户端订单号。
    SellLimitGtcWithoutCloid,
    /// 卖出限价 GTC，带客户端订单号。
    SellLimitGtcWithCloid,
    /// 卖出限价 IOC，不带客户端订单号。
    SellLimitIocWithoutCloid,
    /// 卖出限价 IOC，带客户端订单号。
    SellLimitIocWithCloid,
    /// 卖出限价 ALO，不带客户端订单号。
    SellLimitAloWithoutCloid,
    /// 卖出限价 ALO，带客户端订单号。
    SellLimitAloWithCloid,
    /// 卖出市价意图，不带客户端订单号。
    ///
    /// 对卖单而言，`aggressive_price` 表示卖方愿意接受的最低成交价。
    SellMarketWithoutCloid,
    /// 卖出市价意图，带客户端订单号。
    SellMarketWithCloid,
}

impl ImmediateCommandExample {
    pub(super) const ALL: [Self; 16] = [
        Self::BuyLimitGtcWithoutCloid,
        Self::BuyLimitGtcWithCloid,
        Self::BuyLimitIocWithoutCloid,
        Self::BuyLimitIocWithCloid,
        Self::BuyLimitAloWithoutCloid,
        Self::BuyLimitAloWithCloid,
        Self::BuyMarketWithoutCloid,
        Self::BuyMarketWithCloid,
        Self::SellLimitGtcWithoutCloid,
        Self::SellLimitGtcWithCloid,
        Self::SellLimitIocWithoutCloid,
        Self::SellLimitIocWithCloid,
        Self::SellLimitAloWithoutCloid,
        Self::SellLimitAloWithCloid,
        Self::SellMarketWithoutCloid,
        Self::SellMarketWithCloid,
    ];

    pub(super) const fn expected_time_in_force(self) -> PlaceOrderTimeInForce {
        match self {
            Self::BuyLimitGtcWithoutCloid
            | Self::BuyLimitGtcWithCloid
            | Self::SellLimitGtcWithoutCloid
            | Self::SellLimitGtcWithCloid => PlaceOrderTimeInForce::Gtc,
            Self::BuyLimitIocWithoutCloid
            | Self::BuyLimitIocWithCloid
            | Self::BuyMarketWithoutCloid
            | Self::BuyMarketWithCloid
            | Self::SellLimitIocWithoutCloid
            | Self::SellLimitIocWithCloid
            | Self::SellMarketWithoutCloid
            | Self::SellMarketWithCloid => PlaceOrderTimeInForce::Ioc,
            Self::BuyLimitAloWithoutCloid
            | Self::BuyLimitAloWithCloid
            | Self::SellLimitAloWithoutCloid
            | Self::SellLimitAloWithCloid => PlaceOrderTimeInForce::Alo,
        }
    }

    pub(super) const fn has_cloid(self) -> bool {
        matches!(
            self,
            Self::BuyLimitGtcWithCloid
                | Self::BuyLimitIocWithCloid
                | Self::BuyLimitAloWithCloid
                | Self::BuyMarketWithCloid
                | Self::SellLimitGtcWithCloid
                | Self::SellLimitIocWithCloid
                | Self::SellLimitAloWithCloid
                | Self::SellMarketWithCloid
        )
    }

    pub(super) const fn is_limit(self) -> bool {
        matches!(
            self,
            Self::BuyLimitGtcWithoutCloid
                | Self::BuyLimitGtcWithCloid
                | Self::BuyLimitIocWithoutCloid
                | Self::BuyLimitIocWithCloid
                | Self::BuyLimitAloWithoutCloid
                | Self::BuyLimitAloWithCloid
                | Self::SellLimitGtcWithoutCloid
                | Self::SellLimitGtcWithCloid
                | Self::SellLimitIocWithoutCloid
                | Self::SellLimitIocWithCloid
                | Self::SellLimitAloWithoutCloid
                | Self::SellLimitAloWithCloid
        )
    }

    pub(super) const fn is_market(self) -> bool {
        matches!(
            self,
            Self::BuyMarketWithoutCloid
                | Self::BuyMarketWithCloid
                | Self::SellMarketWithoutCloid
                | Self::SellMarketWithCloid
        )
    }

    pub(super) const fn expected_side(self) -> PlaceOrderSide {
        match self {
            Self::BuyLimitGtcWithoutCloid
            | Self::BuyLimitGtcWithCloid
            | Self::BuyLimitIocWithoutCloid
            | Self::BuyLimitIocWithCloid
            | Self::BuyLimitAloWithoutCloid
            | Self::BuyLimitAloWithCloid
            | Self::BuyMarketWithoutCloid
            | Self::BuyMarketWithCloid => PlaceOrderSide::Buy,
            Self::SellLimitGtcWithoutCloid
            | Self::SellLimitGtcWithCloid
            | Self::SellLimitIocWithoutCloid
            | Self::SellLimitIocWithCloid
            | Self::SellLimitAloWithoutCloid
            | Self::SellLimitAloWithCloid
            | Self::SellMarketWithoutCloid
            | Self::SellMarketWithCloid => PlaceOrderSide::Sell,
        }
    }

    pub(super) const fn expected_execution(self) -> &'static str {
        match self {
            Self::BuyLimitGtcWithoutCloid
            | Self::BuyLimitGtcWithCloid
            | Self::BuyLimitIocWithoutCloid
            | Self::BuyLimitIocWithCloid
            | Self::BuyLimitAloWithoutCloid
            | Self::BuyLimitAloWithCloid
            | Self::SellLimitGtcWithoutCloid
            | Self::SellLimitGtcWithCloid
            | Self::SellLimitIocWithoutCloid
            | Self::SellLimitIocWithCloid
            | Self::SellLimitAloWithoutCloid
            | Self::SellLimitAloWithCloid => "limit",
            Self::BuyMarketWithoutCloid
            | Self::BuyMarketWithCloid
            | Self::SellMarketWithoutCloid
            | Self::SellMarketWithCloid => "market",
        }
    }

    pub(super) fn command_from(self, base_cmd: PlaceImmediateOrderCmd) -> PlaceImmediateOrderCmd {
        let cloid = self.has_cloid().then(|| "0123456789abcdef0123456789abcdef".to_string());
        let execution = match self {
            Self::BuyLimitGtcWithoutCloid
            | Self::BuyLimitGtcWithCloid
            | Self::SellLimitGtcWithoutCloid
            | Self::SellLimitGtcWithCloid => PlaceImmediateOrderExecution::Limit {
                price: base_cmd.execution.reserve_price().unwrap_or_default(),
                time_in_force: PlaceOrderTimeInForce::Gtc,
            },
            Self::BuyLimitIocWithoutCloid
            | Self::BuyLimitIocWithCloid
            | Self::SellLimitIocWithoutCloid
            | Self::SellLimitIocWithCloid => PlaceImmediateOrderExecution::Limit {
                price: base_cmd.execution.reserve_price().unwrap_or_default(),
                time_in_force: PlaceOrderTimeInForce::Ioc,
            },
            Self::BuyLimitAloWithoutCloid
            | Self::BuyLimitAloWithCloid
            | Self::SellLimitAloWithoutCloid
            | Self::SellLimitAloWithCloid => PlaceImmediateOrderExecution::Limit {
                price: base_cmd.execution.reserve_price().unwrap_or_default(),
                time_in_force: PlaceOrderTimeInForce::Alo,
            },
            Self::BuyMarketWithoutCloid
            | Self::BuyMarketWithCloid
            | Self::SellMarketWithoutCloid
            | Self::SellMarketWithCloid => PlaceImmediateOrderExecution::Market {
                aggressive_price: EXAMPLE_MARKET_AGGRESSIVE_PRICE,
            },
        };

        PlaceImmediateOrderCmd {
            is_buy: self.expected_side() == PlaceOrderSide::Buy,
            execution,
            cloid,
            ..base_cmd
        }
    }
}

pub(super) fn supported_command_examples(
    base_cmd: PlaceImmediateOrderCmd,
) -> Vec<(ImmediateCommandExample, PlaceImmediateOrderCmd)> {
    // 覆盖充分性判断：
    // 参考 Hyperliquid exchange endpoint：
    // 1. 普通限价单使用 `t.limit.tif`，支持 Buy/Sell × Gtc/Ioc/Alo × cloid 有无，
    //    所以限价成功场景是 2 × 3 × 2 = 12 种 command 组合；
    // 2. 普通市价单是立即执行语义，API adapter 映射为 IOC + aggressive limitPx，
    //    不支持 Gtc/Alo；所以市价成功场景覆盖 Buy/Sell × IOC 语义 × cloid 有无 = 4 种。
    // 总成功矩阵是 16 种。这里的场景数就是对 command 业务支持面的检查。
    ImmediateCommandExample::ALL
        .into_iter()
        .map(|example| (example, example.command_from(base_cmd.clone())))
        .collect()
}

#[test]
fn supported_command_examples_cover_current_business_matrix() {
    let examples = supported_command_examples(sample_cmd());

    assert_eq!(examples.len(), 16);
    for scenario in ImmediateCommandExample::ALL {
        assert!(examples.iter().any(|(example, _)| *example == scenario));
    }
}

#[test]
fn hyperliquid_time_in_force_support_matrix_is_explicit() {
    let examples = supported_command_examples(sample_cmd());
    let limit_count = examples.iter().filter(|(example, _)| example.is_limit()).count();
    let market_count = examples.iter().filter(|(example, _)| example.is_market()).count();

    assert_eq!(limit_count, 12);
    assert_eq!(market_count, 4);

    for (example, cmd) in examples {
        match cmd.execution {
            PlaceImmediateOrderExecution::Limit { time_in_force, .. } => {
                assert!(example.is_limit());
                assert_eq!(time_in_force, example.expected_time_in_force());
            }
            PlaceImmediateOrderExecution::Market { .. } => {
                assert!(example.is_market());
                assert_eq!(example.expected_time_in_force(), PlaceOrderTimeInForce::Ioc);
            }
        }
    }
}

#[test]
fn market_command_examples_show_aggressive_price_explicitly() {
    let examples = supported_command_examples(sample_cmd());

    for (example, cmd) in examples.into_iter().filter(|(example, _)| example.is_market()) {
        match cmd.execution {
            PlaceImmediateOrderExecution::Market { aggressive_price } => {
                assert_eq!(
                    aggressive_price, EXAMPLE_MARKET_AGGRESSIVE_PRICE,
                    "market command example should show the aggressive price: {example:?}",
                );
            }
            PlaceImmediateOrderExecution::Limit { .. } => {
                panic!("market example should build market execution: {example:?}");
            }
        }
    }
}

#[test]
fn print_supported_command_examples() {
    for (example, cmd) in supported_command_examples(sample_cmd()) {
        match cmd.execution {
            PlaceImmediateOrderExecution::Limit { price, time_in_force } => {
                println!(
                    "{example:?}: asset={}, symbol={}, is_buy={}, execution=Limit, price={}, tif={:?}, size={}, reduce_only={}, cloid={:?}",
                    cmd.asset,
                    cmd.symbol,
                    cmd.is_buy,
                    price,
                    time_in_force,
                    cmd.size,
                    cmd.reduce_only,
                    cmd.cloid,
                );
            }
            PlaceImmediateOrderExecution::Market { aggressive_price } => {
                println!(
                    "{example:?}: asset={}, symbol={}, is_buy={}, execution=Market, aggressive_price={}, tif=Ioc, size={}, reduce_only={}, cloid={:?}",
                    cmd.asset,
                    cmd.symbol,
                    cmd.is_buy,
                    aggressive_price,
                    cmd.size,
                    cmd.reduce_only,
                    cmd.cloid,
                );
            }
        }
    }
}

#[test]
fn supported_command_examples_are_accepted_by_pre_check() {
    let use_case = PlaceImmediateOrderUseCase;

    for (example, cmd) in supported_command_examples(sample_cmd()) {
        assert_eq!(
            CommandUseCase2::pre_check_command(&use_case, &cmd),
            Ok(()),
            "immediate command example should be accepted: {example:?}",
        );
        assert_eq!(cmd.side(), example.expected_side());
        assert_eq!(cmd.execution.stored_time_in_force(), example.expected_time_in_force());
        assert_eq!(cmd.cloid.is_some(), example.has_cloid());
    }
}
