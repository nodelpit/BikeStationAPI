use crate::{
    models::{Station, seed_stations},
    state::AppState,
};
use tokio::time::{Duration, interval, sleep};

async fn simulated_source(latence: Duration) -> Vec<Station> {
    sleep(latence).await;

    seed_stations()
}

async fn refresh_once(state: &AppState, latence: Duration) {
    let new_stations = simulated_source(latence).await;

    let mut stations = state.inner.stations.write().unwrap();

    *stations = new_stations;
}

pub async fn background_task(state: &AppState) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        refresh_once(state, Duration::from_secs(2)).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_simulated_source() {
        let stations = simulated_source(Duration::from_millis(1)).await;

        assert_eq!(stations.len(), 4);
    }

    #[tokio::test]
    async fn test_refresh_once() {
        let state = AppState::new();

        {
            let mut stations = state.inner.stations.write().unwrap();
            stations.clear();
        }

        refresh_once(&state, Duration::from_millis(1)).await;

        let stations = state.inner.stations.read().unwrap();
        assert_eq!(stations.len(), 4);
    }
}
