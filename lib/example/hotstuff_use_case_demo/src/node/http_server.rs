use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use use_case_executor::trading::spot::v3::build_spot_block_executor_v3::SpotBlockCommandV2;

use crate::node::demo_runtime::DemoResult;

pub const DEFAULT_HTTP_ADDR: &str = "127.0.0.1:39001";

#[derive(Clone)]
struct HttpCommandState {
    leader_request_queue: Arc<Mutex<Vec<SpotBlockCommandV2>>>,
}

async fn post_spot_block_command(
    State(state): State<HttpCommandState>,
    Json(command): Json<SpotBlockCommandV2>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    enqueue_spot_block_command(&state.leader_request_queue, command)?;
    Ok(StatusCode::ACCEPTED)
}

pub fn enqueue_spot_block_command(
    leader_request_queue: &Arc<Mutex<Vec<SpotBlockCommandV2>>>,
    command: SpotBlockCommandV2,
) -> Result<(), (StatusCode, &'static str)> {
    let mut queue = leader_request_queue.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "leader request queue poisoned",
        )
    })?;
    queue.push(command);
    Ok(())
}

pub fn start_http_server(
    addr: SocketAddr,
    leader_request_queue: Arc<Mutex<Vec<SpotBlockCommandV2>>>,
) -> DemoResult<std::thread::JoinHandle<()>> {
    let listener = TcpListener::bind(addr)?;
    listener.set_nonblocking(true)?;
    let local_addr = listener.local_addr()?;

    let handle = std::thread::spawn(move || {
        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime,
            Err(error) => {
                eprintln!("[hotstuff_use_case_demo] HTTP runtime 启动失败: {error}");
                return;
            }
        };

        runtime.block_on(async move {
            let listener = match tokio::net::TcpListener::from_std(listener) {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("[hotstuff_use_case_demo] HTTP listener 初始化失败: {error}");
                    return;
                }
            };
            let app = Router::new()
                .route("/spot-block/commands", post(post_spot_block_command))
                .with_state(HttpCommandState {
                    leader_request_queue,
                });

            println!("[hotstuff_use_case_demo] HTTP listener 已启动: http://{local_addr}");
            if let Err(error) = axum::serve(listener, app).await {
                eprintln!("[hotstuff_use_case_demo] HTTP server 退出: {error}");
            }
        });
    });

    Ok(handle)
}

#[cfg(test)]
mod tests {
    use example_core_use_case::entity::account::coin::Coin;
    use example_core_use_case::entity::account::symbol::Symbol;
    use example_core_use_case::{
        OrderGroupRelation, OrderSide, OrderTif, OrderType, PlaceSpotOrderPipelineV2Cmd,
    };
    use rust_decimal::Decimal;

    use super::*;

    fn place_match_command(order_id: u64, cloid: &str) -> SpotBlockCommandV2 {
        SpotBlockCommandV2::Place(PlaceSpotOrderPipelineV2Cmd {
            party_id: common_entity::stable_string_entity_id("buyer"),
            order_id,
            symbol: Symbol::new(1, Coin::BTC, Coin::USDT),
            side: OrderSide::Buy,
            price: Decimal::from(100),
            size: Decimal::ONE,
            order_type: OrderType::Limit { tif: OrderTif::Gtc },
            relation: OrderGroupRelation::Standalone,
            cloid: Some(cloid.to_string()),
            fee_bps: 10,
        })
    }

    #[test]
    fn enqueue_spot_block_command_pushes_place_match_to_leader_queue() {
        let queue = Arc::new(Mutex::new(Vec::new()));
        let command = place_match_command(1, "http-cloid-1");

        assert!(enqueue_spot_block_command(&queue, command.clone()).is_ok());

        let queued = queue
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        assert_eq!(queued.len(), 1);
        assert_eq!(queued[0], command);
    }
}
