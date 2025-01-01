use std::sync::{Arc, RwLock};

use crate::domain::hotel::Hotel;

pub type SharedState = Arc<RwLock<AppState>>;
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Vec<Hotel>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            db: vec![Hotel::lake_inn(), Hotel::falls_inn(), Hotel::forest_inn()],
        }
    }
}

impl AppState {
    pub fn list(&self) -> Vec<Hotel> {
        self.db.to_vec()
    }

    pub fn add(&mut self, hotel: Hotel) {
        self.db.push(hotel);
    }
}
