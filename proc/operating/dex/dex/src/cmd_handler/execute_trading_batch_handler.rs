use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex,
    },
};

use base_types::{OrderId, OrderSide, Price, Quantity, TraderId, TradingPair};
use base_types::exchange::prep::perp_types::PrepTrade;
use base_types::exchange::prep::prep_order::PrepOrder;
use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade, TimeInForce};
use base_types::handler::handler_update::{ChangeSet, CmdHandlerForUpdate};
use super::execute_trading_batch::context::ExecuteTradingBatchContext;
use super::trading_command::{
    ExchangeCommand, ExchangeCommandEnvelope, SpotSide, TradingCommand,
};

type ExecuteTradingBatchError = String;


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
pub enum ExecutedOrder {
    SpotOrder(SpotOrder),
    PrepOrder(PrepOrder),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutedTrade {
    SpotTrade(SpotTrade),
    PrepTrade(PrepTrade),
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
    pub orders: Vec<ExecutedOrder>,
    pub trades: Vec<ExecutedTrade>,
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

pub(crate) type SpotOrderBook = BTreeMap<String, Vec<RestingSpotOrder>>;

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

    fn split_spot_market(market: &str) -> Result<TradingPair, ExecuteTradingBatchError> {
        TradingPair::from_symbol_str(market)
            .ok_or_else(|| format!("invalid spot market: {market}"))
    }

    fn match_spot_order(
        spot_order_book: &mut SpotOrderBook,
        order: &mut RestingSpotOrder,
        writes: &mut ExecutedBatchBlock,
        changelogs: &mut Vec<TradeExecutionLog>,
    ) -> Result<(), ExecuteTradingBatchError> {
        let Some(resting_orders) = spot_order_book.get_mut(&order.market) else {
            return Ok(());
        };

        let trading_pair = Self::split_spot_market(&order.market)?;
        let opposite_side = match order.side {
            SpotSide::Buy => SpotSide::Sell,
            SpotSide::Sell => SpotSide::Buy,
        };

        let mut resting_index = 0;
        while order.remaining_quantity > 0 && resting_index < resting_orders.len() {
            if resting_orders[resting_index].side != opposite_side {
                resting_index += 1;
                continue;
            }

            let crosses = match order.side {
                SpotSide::Buy => order.price >= resting_orders[resting_index].price,
                SpotSide::Sell => order.price <= resting_orders[resting_index].price,
            };
            if !crosses {
                resting_index += 1;
                continue;
            }

            let maker = &mut resting_orders[resting_index];
            let trade_quantity = order.remaining_quantity.min(maker.remaining_quantity);
            let trade_price = maker.price;

            order.remaining_quantity -= trade_quantity;
            maker.remaining_quantity -= trade_quantity;

            writes.summary.trades_executed += 1;
            writes.trades.push(ExecutedTrade::SpotTrade(SpotTrade::new(
                self.next_order_id(),
                trading_pair,
                OrderId::from(order.order_id),
                OrderId::from(maker.order_id),
                base_types::Timestamp::now_as_nanos(),
                Price::from_raw(trade_price),
                Quantity::from_raw(trade_quantity),
                match order.side {
                    SpotSide::Buy => OrderSide::Buy,
                    SpotSide::Sell => OrderSide::Sell,
                },
                Quantity::default(),
                Quantity::default(),
                trading_pair.quote_asset(),
                0,
                0,
            )));
            changelogs.push(TradeExecutionLog::TradeExecuted {
                market: order.market.clone(),
                maker_order_id: maker.order_id,
                taker_order_id: order.order_id,
                price: trade_price,
                quantity: trade_quantity,
            });

            let quote_delta = (trade_price * trade_quantity) as i64;
            match order.side {
                SpotSide::Buy => {
                    writes.balance_deltas.extend([
                        BalanceDelta {
                            trader_id: order.trader_id,
                            asset: trading_pair.base_asset().as_str().to_string(),
                            delta: trade_quantity as i64,
                        },
                        BalanceDelta {
                            trader_id: order.trader_id,
                            asset: trading_pair.quote_asset().as_str().to_string(),
                            delta: -quote_delta,
                        },
                        BalanceDelta {
                            trader_id: maker.trader_id,
                            asset: trading_pair.base_asset().as_str().to_string(),
                            delta: -(trade_quantity as i64),
                        },
                        BalanceDelta {
                            trader_id: maker.trader_id,
                            asset: trading_pair.quote_asset().as_str().to_string(),
                            delta: quote_delta,
                        },
                    ]);
                }
                SpotSide::Sell => {
                    writes.balance_deltas.extend([
                        BalanceDelta {
                            trader_id: order.trader_id,
                            asset: trading_pair.base_asset().as_str().to_string(),
                            delta: -(trade_quantity as i64),
                        },
                        BalanceDelta {
                            trader_id: order.trader_id,
                            asset: trading_pair.quote_asset().as_str().to_string(),
                            delta: quote_delta,
                        },
                        BalanceDelta {
                            trader_id: maker.trader_id,
                            asset: trading_pair.base_asset().as_str().to_string(),
                            delta: trade_quantity as i64,
                        },
                        BalanceDelta {
                            trader_id: maker.trader_id,
                            asset: trading_pair.quote_asset().as_str().to_string(),
                            delta: -quote_delta,
                        },
                    ]);
                }
            }

            if maker.remaining_quantity == 0 {
                resting_orders.remove(resting_index);
            } else {
                resting_index += 1;
            }
        }

        Ok(())
    }
    fn handle_envelope(
        &self,
        envelope: &ExchangeCommandEnvelope,
        ctx: &mut ExecuteTradingBatchContext<'_>,
    ) -> Result<(), ExecuteTradingBatchError> {
        match &envelope.command {
            ExchangeCommand::TradingCommand(command) => {
                self.handle_trading_command(envelope, command, ctx)
            }
            ExchangeCommand::TreasuryCommand(command) => {
                super::execute_trading_batch::treasury::handle_treasury_command(
                    self,
                    envelope,
                    command,
                    ctx,
                )
            }
        }
    }

    fn handle_trading_command(
        &self,
        envelope: &ExchangeCommandEnvelope,
        command: &TradingCommand,
        ctx: &mut ExecuteTradingBatchContext<'_>,
    ) -> Result<(), ExecuteTradingBatchError> {
        match command {
            TradingCommand::Spot(command) => {
                super::execute_trading_batch::spot::handle_spot_command(self, envelope, command, ctx)
            }
            TradingCommand::Perp(command) => {
                super::execute_trading_batch::perp::handle_perp_command(self, envelope, command, ctx)
            }
            TradingCommand::Option(command) => {
                super::execute_trading_batch::option::handle_option_command(
                    self,
                    envelope,
                    command,
                    ctx,
                )
            }
        }
    }
}


//todo CmdHandlerForUpdate里对 ExchangeCommandEnvelope里面的子command 非常多，怎么做个每个子command的处理 提取到一个单独文件中方便维护？
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
            self.handle_envelope(
                envelope,
                &mut ExecuteTradingBatchContext {
                    writes: &mut writes,
                    changelogs: &mut changelogs,
                    spot_order_book: &mut spot_order_book,
                },
            )?;
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
