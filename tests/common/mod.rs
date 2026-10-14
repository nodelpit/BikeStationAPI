use axum::Router;
use axum_governor::{BoxedGovernorLayer, Global, GovernorConfigBuilder, Quota, nz};
use bike_station_api::{app, state::AppState};

pub fn test_app() -> Router {
    let governor = BoxedGovernorLayer::from_config(
        GovernorConfigBuilder::default()
            .with_extractor(Global)
            .quota_default(Quota::requests_per_second(nz!(1000u32)))
            .finish()
            .unwrap(),
    );
    app(AppState::new(), governor)
}
