use axum::http::StatusCode;
use axum_test::TestServer;
use bike_station_api::{app, models::Station, state::AppState};

#[tokio::test]
async fn get_stations_return_200_and_all_stations() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.get("/stations").await;

    response.assert_status(StatusCode::OK);

    let stations: Vec<Station> = response.json();

    assert_eq!(stations.len(), 3);

    assert!(stations.iter().all(|station| station.id.0 > 0));
    assert!(stations.iter().all(|station| !station.name.is_empty()));
    assert!(
        stations
            .iter()
            .all(|station| { station.available_bikes + station.free_docks <= station.total_docks })
    );
}

#[tokio::test]
async fn get_existing_stations_id_return_200_and_station() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.get("/stations/2").await;

    response.assert_status(StatusCode::OK);

    response.assert_json(&serde_json::json!({
        "id":2,"name":"station2","total_docks":20,"available_bikes":12,"free_docks":8
    }));
}

#[tokio::test]
async fn get_non_existing_stations_id_return_404() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.get("/stations/11111111").await;

    response.assert_status(StatusCode::NOT_FOUND);
}
