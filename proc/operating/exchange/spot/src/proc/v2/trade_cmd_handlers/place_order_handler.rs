use base_types::exchange::spot::spot_types::{OrderType, SpotOrder};
use base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};
use base_types::{Timestamp, TradingPair};
use db_repo::{CmdRepo, MySqlDbRepo};
use diff::diff::diff_types::ChangeLog;
use diff::Entity;
use lob_repo::core::symbol_lob_repo::MultiSymbolLobRepo;

use crate::proc::behavior::spot_trade_behavior::{CommonError, SpotCmdErrorAny};
use crate::proc::behavior::v2::spot_trade_behavior_v2::{NewOrderAck, NewOrderCmd};
use crate::proc::v2::id_repo::order_next_id;
use crate::proc::v2::processor::kafka::event_publisher::EventPublisher;


pub(crate) fn validate_order_cmd(cmd: &NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
    if let Some(qty) = cmd.quantity() {
        if qty.is_zero() {
            return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                field: "quantity",
                reason: "must be greater than 0",
            }));
        }
    }

    match cmd.order_type() {
        OrderType::Limit
        | OrderType::StopLossLimit
        | OrderType::TakeProfitLimit
        | OrderType::LimitMaker => {
            if cmd.price().is_none() {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "price",
                    reason: "required for limit orders",
                }));
            }
        }
        _ => {}
    }

    match cmd.order_type() {
        OrderType::StopLoss
        | OrderType::StopLossLimit
        | OrderType::TakeProfit
        | OrderType::TakeProfitLimit => {
            if cmd.stop_price().is_none() {
                return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                    field: "stop_price",
                    reason: "required for conditional orders",
                }));
            }
        }
        _ => {}
    }

    Ok(())
}

pub(crate) fn is_symbol_supported(
    lob_repo: &dyn MultiSymbolLobRepo<Order = SpotOrder>,
    symbol: TradingPair,
) -> bool {
    lob_repo.contains_symbol(&symbol)
}

struct PlaceOrderState {
    symbol: TradingPair,
    order_id: u64,
    timestamp: Timestamp,
}

pub struct PlaceOrderHandler {
    order_repo: std::sync::Arc<MySqlDbRepo<SpotOrder>>,
    lob_repo: std::sync::Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
    event_publisher: std::sync::Arc<dyn EventPublisher>,
}

impl PlaceOrderHandler {
    pub fn new(
        order_repo: std::sync::Arc<MySqlDbRepo<SpotOrder>>,
        lob_repo: std::sync::Arc<dyn MultiSymbolLobRepo<Order = SpotOrder>>,
        event_publisher: std::sync::Arc<dyn EventPublisher>,
    ) -> Self {
        Self { order_repo, lob_repo, event_publisher }
    }
}

//todo 完善收单命令，不作余额冻结操作
impl CmdHandlerForUpdate<NewOrderCmd, PlaceOrderState, NewOrderAck, ChangeLog, SpotCmdErrorAny>
    for PlaceOrderHandler
{
    fn pre_check_command(&self, cmd: &NewOrderCmd) -> Result<(), SpotCmdErrorAny> {
        validate_order_cmd(cmd)
    }

    fn load_state_set_for_update(
        &self,
        cmd: &NewOrderCmd,
    ) -> Result<PlaceOrderState, SpotCmdErrorAny> {
        let symbol = *cmd.symbol();
        if !is_symbol_supported(self.lob_repo.as_ref(), symbol) {
            return Err(SpotCmdErrorAny::Common(CommonError::InvalidParameter {
                field: "symbol",
                reason: "trading pair not supported",
            }));
        }

        Ok(PlaceOrderState {
            symbol,
            order_id: order_next_id() as u64,
            timestamp: Timestamp::now_as_nanos(),
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &NewOrderCmd,
        _state_set: &PlaceOrderState,
    ) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &NewOrderCmd,
        state_set: PlaceOrderState,
    ) -> Result<ChangeSet<NewOrderAck, ChangeLog>, SpotCmdErrorAny> {
        let mut order = SpotOrder::from(cmd.clone());
        order.order_id = state_set.order_id;
        order.trading_pair = state_set.symbol;
        order.state.status = base_types::exchange::spot::spot_types::OrderStatus::New;
        order.state.last_updated = state_set.timestamp;

        let order_log = order.track_create().map_err(|e| {
            SpotCmdErrorAny::Common(CommonError::Internal {
                message: format!("Failed to track order creation: {}", e),
            })
        })?;

        let ack = NewOrderAck::new(
            state_set.symbol,
            state_set.order_id,
            -1,
            cmd.new_client_order_id().clone(),
            state_set.timestamp,
        );

        Ok(ChangeSet { writes: ack, changelogs: vec![order_log] })
    }

    fn persist_changelogs(&self, changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        for changelog in changelogs {
            self.order_repo.replay_event(changelog).map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to persist order: {}", e),
                })
            })?;
        }
        Ok(())
    }

    fn replay_changelogs_to_state(&self, _changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        Ok(())
    }

    fn publish_changelog(&self, changelogs: &[ChangeLog]) -> Result<(), SpotCmdErrorAny> {
        for changelog in changelogs {
            self.event_publisher.publish_order_log(changelog).map_err(|e| {
                SpotCmdErrorAny::Common(CommonError::Internal {
                    message: format!("Failed to publish order log: {}", e),
                })
            })?;
        }
        Ok(())
    }
}
