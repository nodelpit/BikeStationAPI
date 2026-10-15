use std::time::Duration;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

use axum_governor::BoxedGovernorLayer;
use tower_http::{
    LatencyUnit,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::handlers::{
    health::health,
    station::{get_station, list_stations, report_station},
};

use crate::state::AppState;

const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

pub fn app(state: AppState, governor: BoxedGovernorLayer) -> Router {
    Router::new()
        .route("/stations", get(list_stations)) // liste des stations avec leur état (vélos dispo, bornes libres)
        .route("/stations/{id}", get(get_station)) // état d'une station
        .route("/stations/{id}/report", post(report_station)) // signaler un vélo défectueux (la seule écriture)
        .route("/health", get(health))
        .with_state(state) // sonde de vie, utile pour l'observabilité
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            HTTP_REQUEST_TIMEOUT,
        )) // timeout de 5s sur toutes les routes
        .layer(governor) // rate-limiting sur toutes les routes
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::DEBUG)
                        .latency_unit(LatencyUnit::Micros),
                ),
        ) // traçage de toutes les requetes http
}
