use crate::{
    models::{Station, seed_stations},
    state::AppState,
};
use tokio::time::{Duration, interval, sleep, timeout};

async fn simulated_source(latence: Duration) -> Vec<Station> {
    sleep(latence).await;

    seed_stations()
}

async fn refresh_once(state: &AppState, latence: Duration, timeout_duration: Duration) {
    let result = timeout(timeout_duration, simulated_source(latence)).await;

    match result {
        Ok(new_station) => {
            let mut stations = state.inner.stations.write().unwrap();

            *stations = new_station
        }
        Err(_) => {
            eprintln!("request timeout - {:?} elapsed", timeout_duration);
        }
    }
}

pub async fn background_task(state: &AppState) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        interval.tick().await;
        refresh_once(state, Duration::from_secs(10), Duration::from_secs(2)).await;
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

        refresh_once(&state, Duration::from_millis(1), Duration::from_millis(10)).await;

        let stations = state.inner.stations.read().unwrap();
        assert_eq!(stations.len(), 4);
    }

    #[tokio::test]
    async fn test_refresh_timeout() {
        let state = AppState::new();

        {
            let mut stations = state.inner.stations.write().unwrap();
            stations.clear();
        }

        refresh_once(&state, Duration::from_millis(10), Duration::from_millis(1)).await;

        let stations = state.inner.stations.read().unwrap();
        assert_eq!(stations.len(), 0);
    }

    #[tokio::test]
    async fn test_refresh_after_timeout() {
        let state = AppState::new();

        {
            let mut stations = state.inner.stations.write().unwrap();
            stations.clear();
        }

        refresh_once(&state, Duration::from_millis(10), Duration::from_millis(1)).await;

        {
            let stations = state.inner.stations.read().unwrap();
            assert_eq!(stations.len(), 0);
        }

        refresh_once(&state, Duration::from_millis(1), Duration::from_millis(10)).await;

        let stations = state.inner.stations.read().unwrap();
        assert_eq!(stations.len(), 4);
    }
}
