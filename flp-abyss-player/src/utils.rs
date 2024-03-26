pub mod fs;
pub mod meta;
#[cfg(test)]
pub mod test;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::{
    net::TcpListener,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn init_tracing(
    stdout: bool,
    filter: tracing::Level,
) -> tracing_appender::non_blocking::WorkerGuard {
    // Decide which output should be used
    let (writer, guard) = if stdout {
        let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());
        (writer, guard)
    } else {
        // FIXME: Fix the log file path
        let file_appender = tracing_appender::rolling::daily("logs", "service.log");
        let (writer, guard) = tracing_appender::non_blocking(file_appender);
        (writer, guard)
    };

    // Initialize tracing instance
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_max_level(filter)
        .with_ansi(stdout)
        .with_target(false)
        .with_file(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    guard
}

pub fn system_time_to_utc(system_time: &SystemTime) -> Result<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp_millis(
        system_time.duration_since(UNIX_EPOCH)?.as_millis() as i64
    )
    .ok_or_else(|| anyhow!("Cannot convert milliseconds to Utc time"))
}

pub fn find_available_port() -> Option<u16> {
    (40000..40100).find(|port| port_is_available(*port))
}

pub fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}
