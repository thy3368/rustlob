use base_types::base_types::TraderId;
use base_types::{Price, Quantity, TradingPair};
use base_types::exchange::spot::spot_types::SpotTrade;
use base_types::exchange::spot::spot_types::{SpotOrder, TimeInForce};

use crate::core::use_case::execute_trading_batch::{
    ExecuteTradingBatchError, RestingSpotOrder, SpotOrderBook,
};
use crate::core::use_case::execute_trading_batch::spot::handler::SpotBatchHandler;
use crate::core::use_case::execute_trading_batch_handler::{
    BalanceDelta, ExecutedBatchBlock, ExecutedTrade, TradeExecutionLog,
};
use crate::core::SpotSide;

pub(in crate::core) fn split_spot_market(market: &str) -> Result<TradingPair, ExecuteTradingBatchError> {
    TradingPair::from_symbol_str(market)
        .or_else(|| TradingPair::from_symbol_str(&market.replace('-', "")))
        .ok_or_else(|| format!("invalid spot market: {market}"))
}

pub(in crate::core) fn match_spot_order(
    handler: &SpotBatchHandler,
    spot_order_book: &mut SpotOrderBook,
    order: &mut RestingSpotOrder,
    writes: &mut ExecutedBatchBlock,
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
            Price::from_raw(trade_price as i64),
            Quantity::from_raw(trade_quantity as i64),
            match order.side {
                SpotSide::Buy => base_types::OrderSide::Buy,
                SpotSide::Sell => base_types::OrderSide::Sell,
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

pub(in crate::core) fn build_spot_order(
    envelope: &crate::core::ExchangeCommandEnvelope,
    command: &crate::core::SpotPlaceOrderCmd,
    resting_order: &RestingSpotOrder,
) -> Result<SpotOrder, ExecuteTradingBatchError> {
    let trading_pair = split_spot_market(&command.market)?;
    let side = match command.side {
        SpotSide::Buy => base_types::OrderSide::Buy,
        SpotSide::Sell => base_types::OrderSide::Sell,
    };

    let filled_quantity = resting_order.original_quantity - resting_order.remaining_quantity;
    let mut spot_order = SpotOrder::create_order(
        resting_order.order_id,
        TraderId::new(envelope.trader_id.to_le_bytes()),
        trading_pair,
        side,
        Price::from_raw(command.price as i64),
        Quantity::from_raw(command.quantity as i64),
        TimeInForce::GTC,
        None,
        Quantity::default(),
    );
    spot_order.state.filled_base_qty = Quantity::from_raw(filled_quantity as i64);
    if resting_order.remaining_quantity == 0 {
        spot_order.state.status = base_types::exchange::spot::spot_types::OrderStatus::Filled;
    }

    Ok(spot_order)
}
