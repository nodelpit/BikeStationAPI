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

    assert_eq!(stations.len(), 4);

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

#[tokio::test]
async fn report_station_with_bikes() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.post("/stations/1/report").await;

    response.assert_status(StatusCode::OK);

    response.assert_json(&serde_json::json!({
        "id":1,"name":"station1","total_docks":20,"available_bikes":14,"free_docks":5
    }));
}

#[tokio::test]
async fn report_station_with_0_bike() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.post("/stations/4/report").await;

    response.assert_status(StatusCode::CONFLICT);
}

#[tokio::test]
async fn report_station_on_inexistant_id() {
    let state = AppState::new();

    let server = TestServer::new(app(state));

    let response = server.post("/stations/1111111/report").await;

    response.assert_status(StatusCode::NOT_FOUND);
}
