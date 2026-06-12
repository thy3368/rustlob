use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use example_core::{Balance, SpotOrder, SpotOrderExecution, SpotOrderSide, SpotOrderTimeInForce};
use example_inbound_adapter::{
    DepositQuoteOutboundAccess, MatchSpotOrderEventRequest, PlaceOrderOutboundAccess,
    SettleSpotTradeEventRequest, WithdrawQuoteOutboundAccess, build_deposit_http_router,
    build_orders_http_router, build_withdraw_http_router, handle_spot_order_placed_event,
    handle_spot_trade_matched_event,
};
use example_outbound_adapter::{
    DepositQuoteOutboundError, InMemoryDepositQuoteOutbound, InMemoryMatchSpotOrderOutbound,
    InMemoryPlaceOrderOutbound, InMemorySettleSpotTradeOutbound, InMemorySpotPipelineBroker,
    InMemoryStore, InMemoryWithdrawQuoteOutbound, PlaceOrderOutboundError, SpotPipelineBroker,
    SpotPipelineMessage, StoreError, StoreSnapshot, WithdrawQuoteOutboundError,
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("http_demo failed: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = Arc::new(InMemoryDemoApp::new()?);
    let worker_app = app.clone();
    tokio::spawn(async move {
        loop {
            let _ = worker_app.drain_spot_pipeline();
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    });

    let bind_addr =
        std::env::var("HTTP_DEMO_ADDR").unwrap_or_else(|_| "127.0.0.1:3001".to_string());
    let listener = tokio::net::TcpListener::bind(bind_addr.as_str()).await?;
    let router = build_orders_http_router::<InMemoryDemoApp>()
        .merge(build_deposit_http_router::<InMemoryDemoApp>())
        .merge(build_withdraw_http_router::<InMemoryDemoApp>())
        .route("/snapshot", get(get_snapshot))
        .with_state(app);

    println!("http_demo listening on http://{bind_addr}");
    println!("POST /orders");
    println!("POST /deposits/quote");
    println!("POST /withdrawals/quote");
    println!("GET  /snapshot");

    axum::serve(listener, router).await?;
    Ok(())
}

async fn get_snapshot(
    State(app): State<std::sync::Arc<InMemoryDemoApp>>,
) -> Result<Json<Value>, SnapshotApiError> {
    Ok(Json(snapshot_json(app.snapshot().map_err(SnapshotApiError::from)?)))
}

#[derive(Debug)]
struct SnapshotApiError {
    message: String,
}

impl From<StoreError> for SnapshotApiError {
    fn from(error: StoreError) -> Self {
        Self { message: error.to_string() }
    }
}

impl IntoResponse for SnapshotApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": {
                    "code": "snapshot_unavailable",
                    "message": self.message
                }
            })),
        )
            .into_response()
    }
}

#[derive(Debug, Clone)]
struct InMemoryDemoApp {
    store: InMemoryStore,
    broker: InMemorySpotPipelineBroker,
    place_order_outbound: InMemoryPlaceOrderOutbound,
    deposit_quote_outbound: InMemoryDepositQuoteOutbound,
    withdraw_quote_outbound: InMemoryWithdrawQuoteOutbound,
    match_spot_order_outbound: InMemoryMatchSpotOrderOutbound,
    settle_spot_trade_outbound: InMemorySettleSpotTradeOutbound,
}

impl InMemoryDemoApp {
    fn new() -> Result<Self, StoreError> {
        let store = InMemoryStore::seed_balances(
            Balance::new("trader-1".to_string(), "BTC".to_string(), 0, 0, 2),
            Balance::new("trader-1".to_string(), "USDT".to_string(), 1_000, 0, 2),
            example_core::MarketRules { symbol: "BTCUSDT".to_string(), min_qty: 1 },
        )?;
        store.seed_balance(Balance::new("maker-1".to_string(), "BTC".to_string(), 0, 1, 1))?;
        store.seed_balance(Balance::new("maker-1".to_string(), "USDT".to_string(), 0, 0, 1))?;
        store.seed_order(SpotOrder::new(
            "maker-order-1".to_string(),
            10_001,
            None,
            "maker-1".to_string(),
            "BTCUSDT".to_string(),
            SpotOrderSide::Sell,
            SpotOrderExecution::Limit { price: 100 },
            SpotOrderTimeInForce::Gtc,
            1,
            1,
            0,
            None,
        ))?;
        let broker = InMemorySpotPipelineBroker::default();
        let place_order_outbound =
            InMemoryPlaceOrderOutbound::from_store_with_broker(store.clone(), broker.clone());
        let deposit_quote_outbound = InMemoryDepositQuoteOutbound::from_store(store.clone());
        let withdraw_quote_outbound = InMemoryWithdrawQuoteOutbound::from_store(store.clone());
        let match_spot_order_outbound =
            InMemoryMatchSpotOrderOutbound::new(store.clone(), broker.clone());
        let settle_spot_trade_outbound = InMemorySettleSpotTradeOutbound::new(store.clone());

        Ok(Self {
            store,
            broker,
            place_order_outbound,
            deposit_quote_outbound,
            withdraw_quote_outbound,
            match_spot_order_outbound,
            settle_spot_trade_outbound,
        })
    }

    fn drain_spot_pipeline(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let mut processed = 0;

        while let Some(message) = self.broker.pop()? {
            match message {
                SpotPipelineMessage::SpotOrderPlaced(message) => {
                    let _ = handle_spot_order_placed_event(
                        MatchSpotOrderEventRequest {
                            trace_id: message.trace_id,
                            command_id: message.command_id,
                            party_id: message.party_id,
                            taker_order_id: message.order_id,
                            match_id: message.match_id,
                        },
                        &self.match_spot_order_outbound,
                    )?;
                }
                SpotPipelineMessage::SpotTradeMatched(message) => {
                    let _ = handle_spot_trade_matched_event(
                        SettleSpotTradeEventRequest {
                            trace_id: message.trace_id,
                            command_id: message.command_id,
                            party_id: message.party_id,
                            settlement_batch_id: message.settlement_batch_id,
                            trade_ids: message.trade_ids,
                        },
                        &self.settle_spot_trade_outbound,
                    )?;
                }
            }
            processed += 1;
        }

        Ok(processed)
    }

    fn snapshot(&self) -> Result<StoreSnapshot, StoreError> {
        let broker_depth = self.broker.len().map_err(|_| StoreError::StoreUnavailable)?;
        self.store.snapshot_with_broker_depth(broker_depth)
    }
}

impl PlaceOrderOutboundAccess for InMemoryDemoApp {
    type OutboundError = PlaceOrderOutboundError;
    type Outbound = InMemoryPlaceOrderOutbound;

    fn place_order_outbound(&self) -> &Self::Outbound {
        &self.place_order_outbound
    }
}

impl DepositQuoteOutboundAccess for InMemoryDemoApp {
    type OutboundError = DepositQuoteOutboundError;
    type Outbound = InMemoryDepositQuoteOutbound;

    fn deposit_quote_outbound(&self) -> &Self::Outbound {
        &self.deposit_quote_outbound
    }
}

impl WithdrawQuoteOutboundAccess for InMemoryDemoApp {
    type OutboundError = WithdrawQuoteOutboundError;
    type Outbound = InMemoryWithdrawQuoteOutbound;

    fn withdraw_quote_outbound(&self) -> &Self::Outbound {
        &self.withdraw_quote_outbound
    }
}

fn snapshot_json(snapshot: StoreSnapshot) -> Value {
    let balances = snapshot
        .balances
        .into_iter()
        .map(|(balance_id, balance)| {
            (
                balance_id,
                json!({
                    "account_id": balance.account_id,
                    "asset_id": balance.asset_id,
                    "available": balance.available,
                    "frozen": balance.frozen,
                    "version": balance.version
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    let orders = snapshot
        .orders
        .into_iter()
        .map(|(order_id, order)| {
            (
                order_id,
                json!({
                    "order_id": order.order_id,
                    "account_id": order.account_id,
                    "symbol": order.symbol,
                    "qty": order.qty,
                    "price": order.order_price(),
                    "reserved_quote": order.reserved_quote
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    let trades = snapshot
        .trades
        .into_iter()
        .map(|(trade_id, trade)| {
            (
                trade_id,
                json!({
                    "match_id": trade.match_id,
                    "qty": trade.qty,
                    "price": trade.price
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    let settlements = snapshot
        .settlements
        .into_iter()
        .map(|(settlement_id, settlement)| {
            (
                settlement_id,
                json!({
                    "trade_id": settlement.trade_id,
                    "buyer_account_id": settlement.buyer_account_id,
                    "seller_account_id": settlement.seller_account_id
                }),
            )
        })
        .collect::<serde_json::Map<String, Value>>();

    json!({
        "balances": balances,
        "orders": orders,
        "trades": trades,
        "settlements": settlements,
        "persisted_event_count": snapshot.persisted_event_count,
        "published_event_count": snapshot.published_event_count,
        "broker_message_count": snapshot.broker_message_count,
        "next_order_sequence": snapshot.next_order_sequence
    })
}
