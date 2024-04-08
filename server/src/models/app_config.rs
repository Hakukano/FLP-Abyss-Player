use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub locale: String,
}
