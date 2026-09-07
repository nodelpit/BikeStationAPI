use crate::{
    models::{Station, StationId},
    state::AppState,
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn list_stations(State(state): State<AppState>) -> Json<Vec<Station>> {
    let stations = state.inner.stations.read().unwrap().clone();
    Json(stations)
}

pub async fn get_station(
    State(state): State<AppState>,
    Path(id): Path<StationId>,
) -> Result<Json<Station>, StatusCode> {
    let station = state
        .inner
        .stations
        .read()
        .unwrap()
        .iter()
        .find(|station| station.id == id)
        .cloned();

    match station {
        Some(station) => Ok(Json(station)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn report_station(
    State(state): State<AppState>,
    Path(id): Path<StationId>,
) -> Result<Json<Station>, StatusCode> {
    let station = {
        let mut stations = state.inner.stations.write().unwrap();

        let station = stations
            .iter_mut()
            .find(|station| station.id == id)
            .ok_or(StatusCode::NOT_FOUND)?;

        if station.available_bikes == 0 {
            return Err(StatusCode::CONFLICT);
        }

        station.available_bikes -= 1;

        station.clone()
    };

    Ok(Json(station))
}
