use axum::{
    Router,
    http::{Method, StatusCode},
    routing::get,
};
use axum_governor::extractor::PeerIp;
use axum_governor::{
    BoxedGovernorLayer, Global, GovernorConfigBuilder, GovernorLayer, Quota, nz,
    test_utils::request_with_peer,
};
use axum_test::TestServer;
use bike_station_api::{app, state::AppState};
use tokio::time::{Duration, sleep};
use tower::{Layer, ServiceExt};

#[tokio::test]
async fn test_rate_limit_allows_request() {
    let governor = BoxedGovernorLayer::from_config(
        GovernorConfigBuilder::default()
            .with_extractor(Global)
            .quota_default(Quota::requests_per_second(nz!(2u32)))
            .finish()
            .unwrap(),
    );

    let state = AppState::new();

    let server = TestServer::new(app(state, governor));

    let response = server.get("/health").await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_rate_limit_rejects_after_limit() {
    let governor = BoxedGovernorLayer::from_config(
        GovernorConfigBuilder::default()
            .with_extractor(Global)
            .quota_default(Quota::requests_per_second(nz!(1u32)))
            .finish()
            .unwrap(),
    );

    let state = AppState::new();

    let server = TestServer::new(app(state, governor));

    let response = server.get("/health").await;
    response.assert_status(StatusCode::OK);

    let response = server.get("/health").await;
    response.assert_status(StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn test_reset_bucket_after_limit() {
    let governor = BoxedGovernorLayer::from_config(
        GovernorConfigBuilder::default()
            .with_extractor(Global)
            .quota_default(Quota::requests_per_second(nz!(1u32)))
            .finish()
            .unwrap(),
    );

    let state = AppState::new();

    let server = TestServer::new(app(state, governor));

    let response = server.get("/health").await;
    response.assert_status(StatusCode::OK);

    let response = server.get("/health").await;
    response.assert_status(StatusCode::TOO_MANY_REQUESTS);

    sleep(Duration::from_millis(1100)).await;

    let response = server.get("/health").await;
    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn rate_limit_is_per_client() {
    let governor = GovernorConfigBuilder::default()
        .with_extractor(PeerIp::default())
        .expect_connect_info()
        .quota_default(Quota::requests_per_second(nz!(1u32)))
        .finish()
        .unwrap();

    let layer = GovernorLayer::new(governor);

    let router = Router::new().route("/health", get(|| async { StatusCode::OK }));

    let ip_a = "1.1.1.1:1111".parse().unwrap();
    let ip_b = "2.2.2.2:2222".parse().unwrap();

    let response = layer
        .layer(router.clone())
        .oneshot(request_with_peer(Method::GET, "/health", ip_a))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = layer
        .layer(router.clone())
        .oneshot(request_with_peer(Method::GET, "/health", ip_a))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);

    let response = layer
        .layer(router.clone())
        .oneshot(request_with_peer(Method::GET, "/health", ip_b))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
