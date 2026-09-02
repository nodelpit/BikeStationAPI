use axum::{
    Router,
    routing::{get, post},
};

use crate::handlers::{
    health::health,
    station::{get_station, list_stations, report_station},
};

pub fn app() -> Router {
    Router::new()
        .route("/stations", get(list_stations)) // liste des stations avec leur état (vélos dispo, bornes libres)
        .route("/stations/{id}", get(get_station)) // état d'une station
        .route("/stations/{id}/report", post(report_station)) // signaler un vélo défectueux (la seule écriture)
        .route("/health", get(health)) // sonde de vie, utile pour l'observabilité       
}
