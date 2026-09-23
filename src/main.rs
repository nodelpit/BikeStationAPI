use tokio::net::TcpListener;

use bike_station_api::{app, shutdown::shutdown_signal, sources::background_task, state::AppState};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new();
    let token = CancellationToken::new();

    let background_state = state.clone();
    let graceful_token = token.clone();

    let background_handle = tokio::spawn(async move {
        background_task(&background_state, graceful_token).await;
    });

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app(state))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    token.cancel();

    background_handle.await?;

    Ok(())
}
