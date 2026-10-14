use std::net::SocketAddr;
use tokio::net::TcpListener;

use bike_station_api::{app, shutdown::shutdown_signal, sources::background_task, state::AppState};
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::EnvFilter;

use axum_governor::{BoxedGovernorLayer, GovernorConfigBuilder, Quota, extractor::PeerIp, nz};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    info!("Axum server started");

    let governor_cfg = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_second(nz!(50u32)))
        .finish()
        .unwrap();

    let boxed_governor = BoxedGovernorLayer::from_config(governor_cfg);

    let state = AppState::new();
    let token = CancellationToken::new();

    let background_state = state.clone();
    let graceful_token = token.clone();

    let background_handle = tokio::spawn(async move {
        background_task(&background_state, graceful_token).await;
    });

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(
        listener,
        app(state, boxed_governor).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    token.cancel();

    background_handle.await?;

    Ok(())
}
