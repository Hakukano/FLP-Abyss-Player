use std::collections::HashMap;

use once_cell::sync::Lazy;
use parking_lot::RwLock;

pub mod config;
pub mod player;
pub mod playlist;

type Singleton<T> = Lazy<RwLock<HashMap<String, T>>>;
