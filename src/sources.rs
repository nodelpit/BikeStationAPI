use crate::{
    models::{Station, seed_stations},
    state::AppState,
};
use tokio::time::{Duration, interval, sleep, timeout};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, warn};

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
            warn!(?timeout_duration, "source timed out");
        }
    }
}

pub async fn background_task(state: &AppState, token: CancellationToken) {
    let mut interval = interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                debug!("Background refresh started");
                refresh_once(state, Duration::from_secs(1), Duration::from_secs(2)).await;
                debug!("Background refresh finished")
            }

            _ = token.cancelled() => {
                info!("Background task shutting down");
                break;
            }
        }
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

    #[tokio::test]
    async fn test_background_task_shutdown() {
        let state = AppState::new();
        let token = CancellationToken::new();

        let handle_token = token.clone();
        let handle_state = state.clone();

        let handle =
            tokio::spawn(async move { background_task(&handle_state, handle_token.clone()).await });

        token.cancel();

        let result = timeout(Duration::from_millis(500), handle).await;

        assert!(result.is_ok());
    }
}
