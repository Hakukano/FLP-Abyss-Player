#![allow(dead_code)]

use std::path::PathBuf;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

use super::init_tracing;

pub fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

pub struct TestResources {
    pub tracing_guard: WorkerGuard,
}

impl TestResources {
    pub fn new() -> Self {
        let tracing_guard = init_tracing(true, Level::TRACE);
        Self { tracing_guard }
    }
}
