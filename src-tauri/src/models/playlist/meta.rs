use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Copy, Deserialize)]
pub enum MetaCmpBy {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "path")]
    Path,
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "updated_at")]
    UpdatedAt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Meta {
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Meta {
    pub fn cmp_by(&self, other: &Meta, by: MetaCmpBy, ascend: bool) -> Ordering {
        match by {
            MetaCmpBy::Default => {
                if ascend {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            MetaCmpBy::Path => {
                if ascend {
                    self.path.cmp(&other.path)
                } else {
                    other.path.cmp(&self.path)
                }
            }
            MetaCmpBy::CreatedAt => {
                if ascend {
                    self.created_at.cmp(&other.created_at)
                } else {
                    other.created_at.cmp(&self.created_at)
                }
            }
            MetaCmpBy::UpdatedAt => {
                if ascend {
                    self.updated_at.cmp(&other.updated_at)
                } else {
                    other.updated_at.cmp(&self.updated_at)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta_1() -> Meta {
        Meta {
            path: "/1/path".to_string(),
            created_at: DateTime::<Utc>::from_timestamp_millis(2).unwrap(),
            updated_at: DateTime::<Utc>::from_timestamp_millis(3).unwrap(),
        }
    }

    fn meta_2() -> Meta {
        Meta {
            path: "/2/path".to_string(),
            created_at: DateTime::<Utc>::from_timestamp_millis(1).unwrap(),
            updated_at: DateTime::<Utc>::from_timestamp_millis(4).unwrap(),
        }
    }

    #[test]
    fn cmp_by() {
        let meta1 = meta_1();
        let meta2 = meta_2();
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::Default, true).is_lt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::Default, false).is_gt());
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::Path, true).is_lt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::Path, false).is_lt());
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::CreatedAt, true).is_gt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::CreatedAt, false).is_gt());
        assert!(meta1.cmp_by(&meta2, MetaCmpBy::UpdatedAt, true).is_lt());
        assert!(meta2.cmp_by(&meta1, MetaCmpBy::UpdatedAt, false).is_lt());
    }
}
