use axum::http::StatusCode;
use axum_test::TestServer;
use bike_station_api::app;

#[tokio::test]
async fn get_health_return_200() {
    let server = TestServer::new(app());

    let response = server.get("/health").await;

    response.assert_status(StatusCode::OK);
}
