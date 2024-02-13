use serde::Serialize;

use super::Meta;

#[derive(Clone, Debug, Serialize)]
pub struct Entry {
    pub meta: Meta,
    pub mime: String,
}

impl Entry {
    pub fn new(meta: Meta, mime: String) -> Self {
        Self { meta, mime }
    }

    pub fn matches_group(&self, group: impl AsRef<str>) -> bool {
        self.meta.path.starts_with(group.as_ref())
    }
}
