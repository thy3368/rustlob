// HTTP 接口层
pub mod interfaces {

    pub mod spot {

        pub mod http {
            pub mod md_controller;
            pub mod trade_controller;
            pub mod trade_v2_controller;
            pub mod ud_controller;
        }

        pub mod websocket {
            pub mod md_sse_controller;
            pub mod spot_market_data_pusher;
            pub mod ud_sse_controller;
        }

        pub mod starter;
    }

    pub mod usds_m_future {

        pub mod http {
            pub mod md_controller;
            pub mod trade_controller;
            pub mod ud_controller;
        }

        pub mod websocket {

            pub mod md_sse_controller;
            pub mod ud_sse_controller;
        }
        pub mod starter;
    }


    pub mod coin_m_future {

        pub mod http {}

        pub mod websocket {}
    }


    pub mod option {

        pub mod http {}

        pub mod websocket {}
    }
}

use axum::response::IntoResponse;
use interfaces::{spot, usds_m_future};

#[tokio::main]
#[hotpath::main]
async fn main() {
    // 启动 Spot 模块
    if let Err(e) = spot::starter::start_spot_module().await {
        eprintln!("❌ Failed to start Spot module: {}", e);
    }

    // 启动 USDS-M Future 模块
    if let Err(e) = usds_m_future::starter::start_usds_m_future_module().await {
        eprintln!("❌ Failed to start USDS-M Future module: {}", e);
    }

    // 保持主线程运行
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
    println!("✅ Shutdown signal received. Exiting...");
}
