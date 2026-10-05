use axum::http::StatusCode;
use axum::{Router, routing::get};
use axum_test::TestServer;
use std::time::Duration;
use tokio::time::sleep;
use tower_http::timeout::TimeoutLayer;

#[tokio::test(start_paused = true)]
async fn request_timeout_return_408() {
    async fn slow_handler() {
        sleep(Duration::from_secs(7)).await;
    }

    let router =
        Router::new()
            .route("/slow", get(slow_handler))
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(3),
            ));

    let server = TestServer::new(router);

    let response = server.get("/slow").await;

    response.assert_status(StatusCode::REQUEST_TIMEOUT);
}
