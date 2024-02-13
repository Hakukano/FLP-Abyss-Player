use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
use std::path::PathBuf;

#[cfg(test)]
pub fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

pub fn system_time_to_utc(system_time: &SystemTime) -> Result<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp_millis(
        system_time.duration_since(UNIX_EPOCH)?.as_millis() as i64
    )
    .ok_or_else(|| anyhow!("Cannot convert milliseconds to Utc time"))
}
