mod common;

use axum::http::StatusCode;
use axum_test::TestServer;

#[tokio::test]
async fn get_health_return_200() {
    let server = TestServer::new(common::test_app());

    let response = server.get("/health").await;

    response.assert_status(StatusCode::OK);
}
