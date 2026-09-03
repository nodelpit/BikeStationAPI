use axum::http::StatusCode;
use axum_test::TestServer;
use bike_station_api::{app, state::AppState};

#[tokio::test]
async fn get_health_return_200() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.get("/health").await;

    response.assert_status(StatusCode::OK);
}
