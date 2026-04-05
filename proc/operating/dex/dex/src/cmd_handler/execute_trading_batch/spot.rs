use base_types::exchange::spot::spot_types::{SpotOrder, SpotTrade, TimeInForce};
use base_types::{OrderId, OrderSide, Price, Quantity, TradingPair};

use crate::cmd_handler::{
    execute_trading_batch::context::ExecuteTradingBatchContext,
    execute_trading_batch_handler::{
        BalanceDelta, ExecuteTradingBatchError, ExecuteTradingBatchHandler, ExecutedOrder,
        ExecutedTrade, RestingSpotOrder, TradeExecutionLog,
    },
    ExchangeCommandEnvelope, SpotAmendOrderCmd, SpotCancelOrderCmd, SpotCommand,
    SpotPlaceOrderCmd, SpotSide,
};

pub fn handle_spot_command(
    handler: &ExecuteTradingBatchHandler,
    envelope: &ExchangeCommandEnvelope,
    command: &SpotCommand,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    match command {
        //todo 每个 子command  实现 自己的 ApplyCommandChanges，放在单独的文件
        SpotCommand::PlaceOrder(command) => handle_place_order(handler, envelope, command, ctx),
        SpotCommand::CancelOrder(command) => handle_cancel_order(command, ctx),
        SpotCommand::AmendOrder(command) => handle_amend_order(command, ctx),
    }
}

fn handle_place_order(
    handler: &ExecuteTradingBatchHandler,
    envelope: &ExchangeCommandEnvelope,
    command: &SpotPlaceOrderCmd,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    let trading_pair = split_spot_market(&command.market)?;
    let mut resting_order = RestingSpotOrder {
        order_id: envelope.command_id,
        trader_id: envelope.trader_id,
        market: command.market.clone(),
        side: command.side.clone(),
        price: command.price,
        original_quantity: command.quantity,
        remaining_quantity: command.quantity,
    };

    match_spot_order(handler, ctx.spot_order_book, &mut resting_order, ctx.writes, ctx.changelogs)?;

    let side = match command.side {
        SpotSide::Buy => OrderSide::Buy,
        SpotSide::Sell => OrderSide::Sell,
    };

    let filled_quantity = resting_order.original_quantity - resting_order.remaining_quantity;
    let mut spot_order = SpotOrder::create_order(
        resting_order.order_id,
        envelope.trader_id,
        trading_pair,
        side,
        Price::from_raw(command.price),
        Quantity::from_raw(command.quantity),
        TimeInForce::GTC,
        None,
        Quantity::default(),
    );
    spot_order.state.filled_base_qty = Quantity::from_raw(filled_quantity);
    if resting_order.remaining_quantity == 0 {
        spot_order.state.status = base_types::exchange::spot::spot_types::OrderStatus::Filled;
    }

    ctx.writes.summary.accepted_commands += 1;
    ctx.writes.summary.orders_created += 1;
    ctx.writes.orders.push(ExecutedOrder::SpotOrder(spot_order));

    if resting_order.remaining_quantity > 0 {
        ctx.spot_order_book
            .entry(command.market.clone())
            .or_default()
            .push(resting_order);
    }

    Ok(())
}

fn handle_cancel_order(
    _command: &SpotCancelOrderCmd,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}

fn handle_amend_order(
    _command: &SpotAmendOrderCmd,
    ctx: &mut ExecuteTradingBatchContext<'_>,
) -> Result<(), ExecuteTradingBatchError> {
    ctx.writes.summary.accepted_commands += 1;
    Ok(())
}

fn split_spot_market(market: &str) -> Result<TradingPair, ExecuteTradingBatchError> {
    TradingPair::from_symbol_str(market).ok_or_else(|| format!("invalid spot market: {market}"))
}

fn match_spot_order(
    handler: &ExecuteTradingBatchHandler,
    spot_order_book: &mut crate::cmd_handler::SpotOrderBook,
    order: &mut RestingSpotOrder,
    writes: &mut crate::cmd_handler::ExecutedBatchBlock,
    changelogs: &mut Vec<TradeExecutionLog>,
) -> Result<(), ExecuteTradingBatchError> {
    let Some(resting_orders) = spot_order_book.get_mut(&order.market) else {
        return Ok(());
    };

    let trading_pair = split_spot_market(&order.market)?;
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
            handler.next_order_id(),
            trading_pair,
            order.order_id,
            maker.order_id,
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
