use std::collections::{HashMap, HashSet};

use cmd_handler::{
    use_case_def::{CommandUseCase, CommandUseCaseExecutor, DomainEventPipeline, UseCaseReplyMapper},
    DomainEventSet,
};
use rust_decimal::Decimal;

use crate::use_case::command::{
    HyperliquidBuilderFee, HyperliquidOrderRequest, HyperliquidPlaceOrderCmd,
    HyperliquidPlaceOrdersCmd,
};

const MAX_BUILDER_FEE_TENTHS_OF_BPS: u16 = 1000;
const ALLOWED_GROUPINGS: [&str; 3] = ["na", "normalTpsl", "positionTpsl"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HyperliquidPlaceOrdersError {
    EmptyOrders,
    InvalidGrouping(String),
    EmptyBuilderAddress,
    BuilderFeeTooHigh { fee_tenths_of_bps: u16, max: u16 },
    NonPositivePrice { index: usize },
    NonPositiveSize { index: usize },
    NonPositiveTriggerPrice { index: usize },
    EmptyCloid { index: usize },
    DuplicateCloid(String),
    UnknownAsset { asset: u32 },
    AssetNotTradable { asset: u32 },
    InvalidPricePrecision {
        index: usize,
        asset: u32,
        allowed_dp: u32,
    },
    InvalidSizePrecision {
        index: usize,
        asset: u32,
        allowed_dp: u32,
    },
    ReduceOnlyWouldIncreaseExposure { index: usize, asset: u32 },
    GroupingNotAllowed(String),
    BuilderFeeExceedsPolicy { fee_tenths_of_bps: u16, max: u16 },
    LoadStateFailed(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidTradableAssetState {
    pub asset: u32,
    pub price_decimals: u32,
    pub size_decimals: u32,
    pub is_tradable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPositionState {
    pub asset: u32,
    pub signed_size: Decimal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPlaceOrdersStateSnapshot {
    pub tradable_assets: Vec<HyperliquidTradableAssetState>,
    pub positions: Vec<HyperliquidPositionState>,
    pub allowed_groupings: Vec<String>,
    pub max_builder_fee_tenths_of_bps: Option<u16>,
}

pub trait HyperliquidPlaceOrdersLoadPort: Send + Sync {
    fn load_place_orders_state(
        &self,
        cmd: &HyperliquidPlaceOrdersCmd,
    ) -> Result<HyperliquidPlaceOrdersStateSnapshot, HyperliquidPlaceOrdersError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPlaceOrdersState {
    tradable_assets: HashMap<u32, HyperliquidTradableAssetState>,
    signed_positions: HashMap<u32, Decimal>,
    allowed_groupings: HashSet<String>,
    max_builder_fee_tenths_of_bps: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedHyperliquidOrderIntent {
    pub index: usize,
    pub asset: u32,
    pub is_buy: bool,
    pub price: Decimal,
    pub size: Decimal,
    pub reduce_only: bool,
    pub order_type: HyperliquidOrderRequest,
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPlaceOrdersEvents {
    pub grouping: String,
    pub builder: Option<HyperliquidBuilderFee>,
    pub accepted_orders: Vec<AcceptedHyperliquidOrderIntent>,
}

impl DomainEventSet for HyperliquidPlaceOrdersEvents {
    fn domain_event_count(&self) -> usize {
        self.accepted_orders.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HyperliquidPlaceOrdersReply {
    pub accepted_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HyperliquidPlaceOrdersReplyMapper;

impl UseCaseReplyMapper<HyperliquidPlaceOrdersEvents> for HyperliquidPlaceOrdersReplyMapper {
    type Reply = HyperliquidPlaceOrdersReply;

    fn map(&self, events: HyperliquidPlaceOrdersEvents) -> Self::Reply {
        HyperliquidPlaceOrdersReply {
            accepted_count: events.accepted_orders.len(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HyperliquidPlaceOrdersUseCase;

impl CommandUseCase for HyperliquidPlaceOrdersUseCase {
    type Command = HyperliquidPlaceOrdersCmd;
    type GivenState = HyperliquidPlaceOrdersState;
    type Events = HyperliquidPlaceOrdersEvents;
    type Error = HyperliquidPlaceOrdersError;
    type LoadPort = dyn HyperliquidPlaceOrdersLoadPort;

    fn pre_check_command(&self, cmd: &Self::Command) -> Result<(), Self::Error> {
        if cmd.orders.is_empty() {
            return Err(HyperliquidPlaceOrdersError::EmptyOrders);
        }

        if !ALLOWED_GROUPINGS.contains(&cmd.grouping.as_str()) {
            return Err(HyperliquidPlaceOrdersError::InvalidGrouping(
                cmd.grouping.clone(),
            ));
        }

        if let Some(builder) = &cmd.builder {
            if builder.builder.trim().is_empty() {
                return Err(HyperliquidPlaceOrdersError::EmptyBuilderAddress);
            }
            if builder.fee_tenths_of_bps > MAX_BUILDER_FEE_TENTHS_OF_BPS {
                return Err(HyperliquidPlaceOrdersError::BuilderFeeTooHigh {
                    fee_tenths_of_bps: builder.fee_tenths_of_bps,
                    max: MAX_BUILDER_FEE_TENTHS_OF_BPS,
                });
            }
        }

        let mut cloids = HashSet::new();
        for (index, order) in cmd.orders.iter().enumerate() {
            validate_order_input(index, order)?;
            if let Some(cloid) = &order.cloid {
                if !cloids.insert(cloid.clone()) {
                    return Err(HyperliquidPlaceOrdersError::DuplicateCloid(cloid.clone()));
                }
            }
        }

        Ok(())
    }

    fn load_state(
        &self,
        cmd: &Self::Command,
        load_port: &Self::LoadPort,
    ) -> Result<Self::GivenState, Self::Error> {
        let snapshot = load_port.load_place_orders_state(cmd)?;
        Ok(HyperliquidPlaceOrdersState {
            tradable_assets: snapshot
                .tradable_assets
                .into_iter()
                .map(|asset| (asset.asset, asset))
                .collect(),
            signed_positions: snapshot
                .positions
                .into_iter()
                .map(|position| (position.asset, position.signed_size))
                .collect(),
            allowed_groupings: snapshot.allowed_groupings.into_iter().collect(),
            max_builder_fee_tenths_of_bps: snapshot.max_builder_fee_tenths_of_bps,
        })
    }

    fn validate_against_state(
        &self,
        cmd: &Self::Command,
        state: &Self::GivenState,
    ) -> Result<(), Self::Error> {
        if !state.allowed_groupings.is_empty() && !state.allowed_groupings.contains(&cmd.grouping) {
            return Err(HyperliquidPlaceOrdersError::GroupingNotAllowed(
                cmd.grouping.clone(),
            ));
        }

        if let (Some(builder), Some(max)) = (&cmd.builder, state.max_builder_fee_tenths_of_bps) {
            if builder.fee_tenths_of_bps > max {
                return Err(HyperliquidPlaceOrdersError::BuilderFeeExceedsPolicy {
                    fee_tenths_of_bps: builder.fee_tenths_of_bps,
                    max,
                });
            }
        }

        for (index, order) in cmd.orders.iter().enumerate() {
            let asset = state
                .tradable_assets
                .get(&order.asset)
                .ok_or(HyperliquidPlaceOrdersError::UnknownAsset { asset: order.asset })?;

            if !asset.is_tradable {
                return Err(HyperliquidPlaceOrdersError::AssetNotTradable { asset: order.asset });
            }

            if !scale_within_limit(order.price, asset.price_decimals) {
                return Err(HyperliquidPlaceOrdersError::InvalidPricePrecision {
                    index,
                    asset: order.asset,
                    allowed_dp: asset.price_decimals,
                });
            }

            if !scale_within_limit(order.size, asset.size_decimals) {
                return Err(HyperliquidPlaceOrdersError::InvalidSizePrecision {
                    index,
                    asset: order.asset,
                    allowed_dp: asset.size_decimals,
                });
            }

            if order.reduce_only {
                let signed_position = state
                    .signed_positions
                    .get(&order.asset)
                    .copied()
                    .unwrap_or(Decimal::ZERO);
                if reduce_only_increases_exposure(order, signed_position) {
                    return Err(
                        HyperliquidPlaceOrdersError::ReduceOnlyWouldIncreaseExposure {
                            index,
                            asset: order.asset,
                        },
                    );
                }
            }
        }

        Ok(())
    }

    fn then(
        &self,
        cmd: &Self::Command,
        _state: Self::GivenState,
    ) -> Result<Self::Events, Self::Error> {
        Ok(HyperliquidPlaceOrdersEvents {
            grouping: cmd.grouping.clone(),
            builder: cmd.builder.clone(),
            accepted_orders: cmd
                .orders
                .iter()
                .enumerate()
                .map(|(index, order)| AcceptedHyperliquidOrderIntent {
                    index,
                    asset: order.asset,
                    is_buy: order.is_buy,
                    price: order.price,
                    size: order.size,
                    reduce_only: order.reduce_only,
                    order_type: order.order_type.clone(),
                    cloid: order.cloid.clone(),
                })
                .collect(),
        })
    }
}

fn validate_order_input(
    index: usize,
    order: &HyperliquidPlaceOrderCmd,
) -> Result<(), HyperliquidPlaceOrdersError> {
    if order.price <= Decimal::ZERO {
        return Err(HyperliquidPlaceOrdersError::NonPositivePrice { index });
    }
    if order.size <= Decimal::ZERO {
        return Err(HyperliquidPlaceOrdersError::NonPositiveSize { index });
    }
    if let Some(cloid) = &order.cloid {
        if cloid.trim().is_empty() {
            return Err(HyperliquidPlaceOrdersError::EmptyCloid { index });
        }
    }
    if let HyperliquidOrderRequest::Trigger(trigger) = &order.order_type {
        if trigger.trigger_px <= Decimal::ZERO {
            return Err(HyperliquidPlaceOrdersError::NonPositiveTriggerPrice { index });
        }
    }
    Ok(())
}

fn scale_within_limit(value: Decimal, allowed_dp: u32) -> bool {
    value.normalize().scale() <= allowed_dp
}

fn reduce_only_increases_exposure(order: &HyperliquidPlaceOrderCmd, signed_position: Decimal) -> bool {
    if signed_position > Decimal::ZERO {
        order.is_buy
    } else if signed_position < Decimal::ZERO {
        !order.is_buy
    } else {
        true
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoopHyperliquidPlaceOrdersPipeline;

impl DomainEventPipeline<HyperliquidPlaceOrdersEvents, HyperliquidPlaceOrdersError>
    for NoopHyperliquidPlaceOrdersPipeline
{
    fn persist(&self, _events: &HyperliquidPlaceOrdersEvents) -> Result<(), HyperliquidPlaceOrdersError> {
        Ok(())
    }

    fn replay(&self, _events: &HyperliquidPlaceOrdersEvents) -> Result<(), HyperliquidPlaceOrdersError> {
        Ok(())
    }

    fn publish(&self, _events: &HyperliquidPlaceOrdersEvents) -> Result<(), HyperliquidPlaceOrdersError> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SpyHyperliquidPlaceOrdersPipeline {
    calls: std::sync::Mutex<Vec<&'static str>>,
}

impl SpyHyperliquidPlaceOrdersPipeline {
    pub fn calls(&self) -> Vec<&'static str> {
        self.calls.lock().unwrap().clone()
    }
}

impl DomainEventPipeline<HyperliquidPlaceOrdersEvents, HyperliquidPlaceOrdersError>
    for SpyHyperliquidPlaceOrdersPipeline
{
    fn persist(&self, _events: &HyperliquidPlaceOrdersEvents) -> Result<(), HyperliquidPlaceOrdersError> {
        self.calls.lock().unwrap().push("persist");
        Ok(())
    }

    fn replay(&self, _events: &HyperliquidPlaceOrdersEvents) -> Result<(), HyperliquidPlaceOrdersError> {
        self.calls.lock().unwrap().push("replay");
        Ok(())
    }

    fn publish(&self, _events: &HyperliquidPlaceOrdersEvents) -> Result<(), HyperliquidPlaceOrdersError> {
        self.calls.lock().unwrap().push("publish");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_case::command::{
        HyperliquidTif, HyperliquidTrigger, HyperliquidTriggerKind,
    };

    #[derive(Debug, Clone)]
    struct StubLoadPort {
        snapshot: HyperliquidPlaceOrdersStateSnapshot,
    }

    impl HyperliquidPlaceOrdersLoadPort for StubLoadPort {
        fn load_place_orders_state(
            &self,
            _cmd: &HyperliquidPlaceOrdersCmd,
        ) -> Result<HyperliquidPlaceOrdersStateSnapshot, HyperliquidPlaceOrdersError> {
            Ok(self.snapshot.clone())
        }
    }

    #[derive(Debug, Clone, Copy, Default)]
    struct FailingLoadPort;

    impl HyperliquidPlaceOrdersLoadPort for FailingLoadPort {
        fn load_place_orders_state(
            &self,
            _cmd: &HyperliquidPlaceOrdersCmd,
        ) -> Result<HyperliquidPlaceOrdersStateSnapshot, HyperliquidPlaceOrdersError> {
            Err(HyperliquidPlaceOrdersError::LoadStateFailed(
                "state unavailable".to_string(),
            ))
        }
    }

    #[test]
    fn reject_empty_batch() {
        let use_case = HyperliquidPlaceOrdersUseCase;
        let error = use_case
            .pre_check_command(&HyperliquidPlaceOrdersCmd {
                orders: vec![],
                grouping: "na".to_string(),
                builder: None,
            })
            .unwrap_err();
        assert_eq!(error, HyperliquidPlaceOrdersError::EmptyOrders);
    }

    #[test]
    fn reject_non_positive_price() {
        let use_case = HyperliquidPlaceOrdersUseCase;
        let mut cmd = valid_command();
        cmd.orders[0].price = Decimal::ZERO;

        let error = use_case.pre_check_command(&cmd).unwrap_err();
        assert_eq!(error, HyperliquidPlaceOrdersError::NonPositivePrice { index: 0 });
    }

    #[test]
    fn reject_duplicate_cloid() {
        let use_case = HyperliquidPlaceOrdersUseCase;
        let mut cmd = valid_command();
        cmd.orders.push(HyperliquidPlaceOrderCmd {
            cloid: Some("dup-1".to_string()),
            ..valid_order()
        });

        let error = use_case.pre_check_command(&cmd).unwrap_err();
        assert_eq!(error, HyperliquidPlaceOrdersError::DuplicateCloid("dup-1".to_string()));
    }

    #[test]
    fn reject_trigger_with_non_positive_trigger_price() {
        let use_case = HyperliquidPlaceOrdersUseCase;
        let mut cmd = valid_command();
        cmd.orders[0].order_type = HyperliquidOrderRequest::Trigger(HyperliquidTrigger {
            trigger_px: Decimal::ZERO,
            is_market: true,
            kind: HyperliquidTriggerKind::StopLoss,
        });

        let error = use_case.pre_check_command(&cmd).unwrap_err();
        assert_eq!(
            error,
            HyperliquidPlaceOrdersError::NonPositiveTriggerPrice { index: 0 }
        );
    }

    #[test]
    fn reject_unknown_asset_from_loaded_state() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = NoopHyperliquidPlaceOrdersPipeline;
        let load_port = StubLoadPort {
            snapshot: snapshot_with_assets(vec![tradable_asset(2, 2, 3)], vec![]),
        };

        let error = executor
            .execute(&use_case, valid_command(), &load_port, &pipeline)
            .unwrap_err();

        assert_eq!(error, HyperliquidPlaceOrdersError::UnknownAsset { asset: 1 });
    }

    #[test]
    fn reject_asset_with_invalid_price_precision() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = NoopHyperliquidPlaceOrdersPipeline;
        let mut cmd = valid_command();
        cmd.orders[0].price = Decimal::new(12345, 3);
        let load_port = StubLoadPort {
            snapshot: snapshot_with_assets(vec![tradable_asset(1, 2, 3)], vec![]),
        };

        let error = executor.execute(&use_case, cmd, &load_port, &pipeline).unwrap_err();

        assert_eq!(
            error,
            HyperliquidPlaceOrdersError::InvalidPricePrecision {
                index: 0,
                asset: 1,
                allowed_dp: 2,
            }
        );
    }

    #[test]
    fn reject_reduce_only_order_that_increases_exposure() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = NoopHyperliquidPlaceOrdersPipeline;
        let mut cmd = valid_command();
        cmd.orders[0].reduce_only = true;
        cmd.orders[0].is_buy = true;
        let load_port = StubLoadPort {
            snapshot: snapshot_with_assets(
                vec![tradable_asset(1, 2, 3)],
                vec![HyperliquidPositionState {
                    asset: 1,
                    signed_size: Decimal::new(5, 0),
                }],
            ),
        };

        let error = executor.execute(&use_case, cmd, &load_port, &pipeline).unwrap_err();

        assert_eq!(
            error,
            HyperliquidPlaceOrdersError::ReduceOnlyWouldIncreaseExposure {
                index: 0,
                asset: 1,
            }
        );
    }

    #[test]
    fn accept_valid_batch_and_produce_events() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = NoopHyperliquidPlaceOrdersPipeline;
        let cmd = valid_command();
        let load_port = StubLoadPort {
            snapshot: snapshot_with_assets(
                vec![tradable_asset(1, 2, 3)],
                vec![HyperliquidPositionState {
                    asset: 1,
                    signed_size: Decimal::new(5, 0),
                }],
            ),
        };

        let events = executor.execute(&use_case, cmd.clone(), &load_port, &pipeline).unwrap();

        assert_eq!(events.grouping, cmd.grouping);
        assert_eq!(events.builder, cmd.builder);
        assert_eq!(events.accepted_orders.len(), 1);
        assert_eq!(events.accepted_orders[0].index, 0);
        assert_eq!(events.accepted_orders[0].asset, 1);
        assert_eq!(events.accepted_orders[0].price, Decimal::new(1234, 1));
        assert_eq!(events.domain_event_count(), 1);
    }

    #[test]
    fn map_reply_outside_use_case_core() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = NoopHyperliquidPlaceOrdersPipeline;
        let mapper = HyperliquidPlaceOrdersReplyMapper;
        let load_port = StubLoadPort {
            snapshot: snapshot_with_assets(vec![tradable_asset(1, 2, 3)], vec![]),
        };

        let reply = executor
            .execute_and_map_reply(&use_case, valid_command(), &load_port, &pipeline, &mapper)
            .unwrap();

        assert_eq!(reply, HyperliquidPlaceOrdersReply { accepted_count: 1 });
    }

    #[test]
    fn execute_runs_pipeline_in_order() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = SpyHyperliquidPlaceOrdersPipeline::default();
        let load_port = StubLoadPort {
            snapshot: snapshot_with_assets(vec![tradable_asset(1, 2, 3)], vec![]),
        };

        executor
            .execute(&use_case, valid_command(), &load_port, &pipeline)
            .unwrap();

        assert_eq!(pipeline.calls(), vec!["persist", "replay", "publish"]);
    }

    #[test]
    fn propagate_load_state_error() {
        let executor = CommandUseCaseExecutor;
        let use_case = HyperliquidPlaceOrdersUseCase;
        let pipeline = NoopHyperliquidPlaceOrdersPipeline;

        let error = executor
            .execute(&use_case, valid_command(), &FailingLoadPort, &pipeline)
            .unwrap_err();

        assert_eq!(
            error,
            HyperliquidPlaceOrdersError::LoadStateFailed("state unavailable".to_string())
        );
    }

    fn valid_command() -> HyperliquidPlaceOrdersCmd {
        HyperliquidPlaceOrdersCmd {
            orders: vec![valid_order()],
            grouping: "na".to_string(),
            builder: Some(HyperliquidBuilderFee {
                builder: "0xabc".to_string(),
                fee_tenths_of_bps: 10,
            }),
        }
    }

    fn valid_order() -> HyperliquidPlaceOrderCmd {
        HyperliquidPlaceOrderCmd {
            asset: 1,
            is_buy: false,
            price: Decimal::new(1234, 1),
            size: Decimal::new(125, 2),
            reduce_only: false,
            order_type: HyperliquidOrderRequest::Limit {
                tif: HyperliquidTif::Gtc,
            },
            cloid: Some("dup-1".to_string()),
        }
    }

    fn tradable_asset(asset: u32, price_decimals: u32, size_decimals: u32) -> HyperliquidTradableAssetState {
        HyperliquidTradableAssetState {
            asset,
            price_decimals,
            size_decimals,
            is_tradable: true,
        }
    }

    fn snapshot_with_assets(
        tradable_assets: Vec<HyperliquidTradableAssetState>,
        positions: Vec<HyperliquidPositionState>,
    ) -> HyperliquidPlaceOrdersStateSnapshot {
        HyperliquidPlaceOrdersStateSnapshot {
            tradable_assets,
            positions,
            allowed_groupings: vec!["na".to_string(), "normalTpsl".to_string()],
            max_builder_fee_tenths_of_bps: Some(100),
        }
    }
}
