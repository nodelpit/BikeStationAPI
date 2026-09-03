use tokio::net::TcpListener;

use bike_station_api::{app, state::AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new();

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app(state)).await?;

    Ok(())
}
