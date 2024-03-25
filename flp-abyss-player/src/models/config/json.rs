use std::{fs::File, path::Path};

#[allow(dead_code)]
pub fn new(path: impl AsRef<Path>) -> super::Config {
    serde_json::from_reader(File::open(path).expect("Cannot open json config"))
        .expect("Cannot parse json config")
}
