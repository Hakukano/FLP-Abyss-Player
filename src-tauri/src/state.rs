use crate::models::app_config;
use parking_lot::RwLock;

use crate::models::app_config::AppConfig;

pub struct AppState {
    pub app_config: RwLock<Box<dyn AppConfig>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_config: RwLock::new(app_config::instantiate()),
        }
    }
}
