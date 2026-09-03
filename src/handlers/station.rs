use crate::models::{Station, StationId, seed_stations};
use axum::{Json, extract::Path, http::StatusCode};

pub async fn list_stations() -> Json<Vec<Station>> {
    Json(seed_stations())
}

pub async fn get_station(Path(id): Path<StationId>) -> Result<Json<Station>, StatusCode> {
    match seed_stations().into_iter().find(|station| station.id == id) {
        Some(station) => Ok(Json(station)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn report_station() {}
