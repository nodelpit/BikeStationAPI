use crate::models::{Station, seed_stations};
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub stations: RwLock<Vec<Station>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                stations: (RwLock::new(seed_stations())),
            }),
        }
    }
}
