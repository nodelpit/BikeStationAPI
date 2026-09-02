use tokio::net::TcpListener;

use bike_station_api::app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app()).await?;

    Ok(())
}
