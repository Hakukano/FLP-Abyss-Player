use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub trait Differ {
    fn diff(&self, other: &Self) -> Option<Value>;
    fn apply_diff(&mut self, diff: Value);
}

impl<T> Differ for Option<T>
where
    T: Serialize + DeserializeOwned + Differ,
{
    fn diff(&self, other: &Self) -> Option<Value> {
        match (self.as_ref(), other.as_ref()) {
            (None, None) => None,
            (Some(s), Some(o)) => s.diff(o),
            _ => Some(serde_json::to_value(other).unwrap()),
        }
    }

    fn apply_diff(&mut self, diff: Value) {
        *self = serde_json::from_value(diff).unwrap();
    }
}

#[macro_export]
macro_rules! impl_differ_simple {
    ($typ:ty) => {
        impl Differ for $typ {
            fn diff(&self, other: &Self) -> Option<serde_json::Value> {
                if self == other {
                    None
                } else {
                    Some(serde_json::to_value(other).unwrap())
                }
            }

            fn apply_diff(&mut self, diff: serde_json::Value) {
                *self = serde_json::from_value(diff).unwrap();
            }
        }
    };
}

impl_differ_simple!(bool);
impl_differ_simple!(u8);
impl_differ_simple!(u32);
impl_differ_simple!(String);
impl_differ_simple!(DateTime<Utc>);
impl_differ_simple!(Vec<String>);
