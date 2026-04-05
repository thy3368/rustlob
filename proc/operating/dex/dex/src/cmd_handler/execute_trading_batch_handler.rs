use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};

use super::trading_command::{
    ExchangeCommand, ExchangeCommandEnvelope, OptionCommand, PerpCommand, SpotCommand, SpotSide,
};

type ExecuteTradingBatchError = String;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TradeExecutionResult {
    pub market: String,
    pub maker_order_id: u64,
    pub taker_order_id: u64,
    pub price: u64,
    pub quantity: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BatchExecutionSummary {
    pub total_commands: usize,
    pub accepted_commands: usize,
    pub rejected_commands: usize,
    pub orders_created: usize,
    pub trades_executed: usize,
    pub balance_updates: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutedSpotOrder {
    pub order_id: u64,
    pub trader_id: u64,
    pub market: String,
    pub side: SpotSide,
    pub price: u64,
    pub original_quantity: u64,
    pub remaining_quantity: u64,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BalanceDelta {
    pub trader_id: u64,
    pub asset: String,
    pub delta: i64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExecutedBatchBlock {
    pub summary: BatchExecutionSummary,
    pub orders: Vec<ExecutedSpotOrder>,
    pub trades: Vec<TradeExecutionResult>,
    pub balance_deltas: Vec<BalanceDelta>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExecuteTradingBatchState {
    pub batch_size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TradeExecutionLog {
    TradeExecuted {
        market: String,
        maker_order_id: u64,
        taker_order_id: u64,
        price: u64,
        quantity: u64,
    },
    BatchExecuted { batch_size: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RestingSpotOrder {
    order_id: u64,
    trader_id: u64,
    market: String,
    side: SpotSide,
    price: u64,
    original_quantity: u64,
    remaining_quantity: u64,
}

type SpotOrderBook = BTreeMap<String, Vec<RestingSpotOrder>>;

#[derive(Debug, Default)]
pub struct ExecuteTradingBatchHandler {
    spot_order_book: Mutex<SpotOrderBook>,
    next_order_id: AtomicU64,
}

impl ExecuteTradingBatchHandler {
    pub fn new() -> Self {
        Self {
            spot_order_book: Mutex::new(BTreeMap::new()),
            next_order_id: AtomicU64::new(1),
        }
    }

    fn next_order_id(&self) -> u64 {
        self.next_order_id.fetch_add(1, Ordering::Relaxed)
    }
}

impl CmdHandlerForUpdate<
    Vec<ExchangeCommandEnvelope>,
    ExecuteTradingBatchState,
    ExecutedBatchBlock,
    TradeExecutionLog,
    ExecuteTradingBatchError,
> for ExecuteTradingBatchHandler
{
    fn pre_check_command(
        &self,
        _cmd: &Vec<ExchangeCommandEnvelope>,
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn load_state_set_for_update(
        &self,
        cmd: &Vec<ExchangeCommandEnvelope>,
    ) -> Result<ExecuteTradingBatchState, ExecuteTradingBatchError> {
        Ok(ExecuteTradingBatchState {
            batch_size: cmd.len(),
        })
    }

    fn validate_command_in_lock(
        &self,
        _cmd: &Vec<ExchangeCommandEnvelope>,
        _state_set: &ExecuteTradingBatchState,
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn apply_command_and_collect_changes(
        &self,
        cmd: &Vec<ExchangeCommandEnvelope>,
        state_set: ExecuteTradingBatchState,
    ) -> Result<ChangeSet<ExecutedBatchBlock, TradeExecutionLog>, ExecuteTradingBatchError> {
        let mut writes = ExecutedBatchBlock {
            summary: BatchExecutionSummary {
                total_commands: state_set.batch_size,
                ..BatchExecutionSummary::default()
            },
            ..ExecutedBatchBlock::default()
        };

        let mut changelogs = Vec::new();
        let mut spot_order_book = self.spot_order_book.lock().unwrap();

        for envelope in cmd {
            match &envelope.command {
                ExchangeCommand::TradingCommand(trading_command) => match trading_command {
                    super::trading_command::TradingCommand::Spot(SpotCommand::PlaceOrder(place_cmd)) => {
                        let order_id = self.next_order_id();
                        let order = RestingSpotOrder {
                            order_id,
                            trader_id: place_cmd.trader_id,
                            market: place_cmd.market.clone(),
                            side: place_cmd.side.clone(),
                            price: place_cmd.price,
                            original_quantity: place_cmd.quantity,
                            remaining_quantity: place_cmd.quantity,
                        };

                        writes.summary.accepted_commands += 1;
                        writes.summary.orders_created += 1;
                        writes.orders.push(ExecutedSpotOrder {
                            order_id,
                            trader_id: order.trader_id,
                            market: order.market.clone(),
                            side: order.side.clone(),
                            price: order.price,
                            original_quantity: order.original_quantity,
                            remaining_quantity: order.remaining_quantity,
                            status: OrderStatus::Open,
                        });

                        spot_order_book
                            .entry(order.market.clone())
                            .or_default()
                            .push(order);
                    }
                    super::trading_command::TradingCommand::Spot(
                        SpotCommand::CancelOrder(_) | SpotCommand::AmendOrder(_),
                    ) => {
                        writes.summary.accepted_commands += 1;
                    }
                    super::trading_command::TradingCommand::Perp(
                        PerpCommand::PlaceOrder(_)
                        | PerpCommand::CancelOrder(_)
                        | PerpCommand::AmendOrder(_)
                        | PerpCommand::SettleFunding(_)
                        | PerpCommand::LiquidatePosition(_),
                    ) => {
                        writes.summary.accepted_commands += 1;
                    }
                    super::trading_command::TradingCommand::Perp(PerpCommand::ExecuteTrade(cmd)) => {
                        writes.summary.accepted_commands += 1;
                        writes.summary.trades_executed += 1;
                        writes.trades.push(TradeExecutionResult {
                            market: cmd.market.clone(),
                            maker_order_id: cmd.maker_order_id,
                            taker_order_id: cmd.taker_order_id,
                            price: cmd.price,
                            quantity: cmd.quantity,
                        });
                        changelogs.push(TradeExecutionLog::TradeExecuted {
                            market: cmd.market.clone(),
                            maker_order_id: cmd.maker_order_id,
                            taker_order_id: cmd.taker_order_id,
                            price: cmd.price,
                            quantity: cmd.quantity,
                        });
                    }
                    super::trading_command::TradingCommand::Option(
                        OptionCommand::PlaceOrder(_)
                        | OptionCommand::CancelOrder(_)
                        | OptionCommand::AmendOrder(_),
                    ) => {
                        writes.summary.accepted_commands += 1;
                    }
                },
                ExchangeCommand::TreasuryCommand(_) => {
                    writes.summary.accepted_commands += 1;
                }
            }
        }

        writes.summary.balance_updates = writes.balance_deltas.len();

        changelogs.push(TradeExecutionLog::BatchExecuted {
            batch_size: writes.summary.total_commands,
        });

        Ok(ChangeSet { writes, changelogs })
    }

    fn persist_changelogs(
        &self,
        _changelogs: &[TradeExecutionLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn replay_changelogs_to_state(
        &self,
        _changelogs: &[TradeExecutionLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }

    fn publish_changelog(
        &self,
        _changelogs: &[TradeExecutionLog],
    ) -> Result<(), ExecuteTradingBatchError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd_handler::{
        ExchangeCommand, PerpAmendOrderCmd, PerpCommand, PerpPlaceOrderCmd, PerpSide,
        SpotCancelOrderCmd, SpotCommand, TradingCommand,
    };

    fn place_order_envelope(command_id: u64, trader_id: u64) -> ExchangeCommandEnvelope {
        ExchangeCommandEnvelope {
            command_id,
            trader_id,
            nonce: command_id,
            timestamp_ns: 1_000 + command_id,
            command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                    trader_id,
                    market: "BTC-PERP".into(),
                    side: PerpSide::Buy,
                    price: 100_000,
                    quantity: 2,
                    leverage: 1,
                    reduce_only: false,
                }),
            )),
        }
    }

    #[test]
    fn cmd_handle_counts_each_command_kind() {
        let handler = ExecuteTradingBatchHandler::new();

        let result = handler.cmd_handle(
            vec![
                place_order_envelope(1, 1),
                ExchangeCommandEnvelope {
                    command_id: 2,
                    trader_id: 1,
                    nonce: 2,
                    timestamp_ns: 1_002,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Spot(
                        SpotCommand::CancelOrder(SpotCancelOrderCmd {
                            trader_id: 1,
                            order_id: 42,
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 3,
                    trader_id: 1,
                    nonce: 3,
                    timestamp_ns: 1_003,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::AmendOrder(PerpAmendOrderCmd {
                            trader_id: 1,
                            order_id: 42,
                            new_price: Some(101_000),
                            new_quantity: Some(3),
                        }),
                    )),
                },
                ExchangeCommandEnvelope {
                    command_id: 4,
                    trader_id: 2,
                    nonce: 4,
                    timestamp_ns: 1_004,
                    command: ExchangeCommand::TradingCommand(TradingCommand::Perp(
                        PerpCommand::PlaceOrder(PerpPlaceOrderCmd {
                            trader_id: 2,
                            market: "ETH-PERP".into(),
                            side: PerpSide::Sell,
                            price: 3_000,
                            quantity: 5,
                            leverage: 1,
                            reduce_only: false,
                        }),
                    )),
                },
            ],
            |writes, _| writes.clone(),
        );

        assert!(result.is_ok());
        let writes = result.unwrap();
        assert_eq!(writes.summary.total_commands, 4);
        assert_eq!(writes.summary.accepted_commands, 4);
        assert_eq!(writes.summary.orders_created, 0);
        assert_eq!(writes.summary.trades_executed, 0);
    }

    #[test]
    fn cmd_handler_for_update_emits_batch_log() {
        let handler = ExecuteTradingBatchHandler::new();

        let result = handler.cmd_handle(vec![place_order_envelope(1, 1)], |writes, changelogs| {
            (writes.clone(), changelogs.to_vec())
        });

        assert!(result.is_ok());
        let (writes, changelogs) = result.unwrap();
        assert_eq!(writes.summary.total_commands, 1);
        assert_eq!(changelogs.len(), 1);
        assert_eq!(
            changelogs[0],
            TradeExecutionLog::BatchExecuted { batch_size: 1 }
        );
    }
}
