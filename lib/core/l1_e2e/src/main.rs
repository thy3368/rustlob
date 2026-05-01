mod bootstrap;
mod http;
mod ingress_load_port;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let state = bootstrap::build_app_state()?;
    let app = http::router(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;

    tracing::info!(address = "127.0.0.1:3001", "l1_e2e server started");
    axum::serve(listener, app).await?;
    Ok(())
}
