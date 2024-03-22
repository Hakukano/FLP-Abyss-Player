use derive_builder::Builder;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod playlist;

fn deserialize_list_query_filter<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?
        .split(',')
        .map(|s| s.trim().to_string())
        .collect())
}

fn serialize_list_query_filter<S>(filter: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(filter.join(",").as_str())
}

#[derive(Deserialize, Serialize, Builder)]
pub struct ListQuery {
    #[serde(deserialize_with = "deserialize_list_query_filter")]
    #[serde(serialize_with = "serialize_list_query_filter")]
    #[builder(default)]
    pub filter: Vec<String>,
    #[builder(default)]
    pub search: String,
    pub offset: u32,
    pub length: u32,
}

#[derive(Deserialize, Serialize, Builder)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
    pub count: u32,
}

impl<T> ListResponse<T>
where
    T: Clone,
{
    pub fn builder() -> ListResponseBuilder<T> {
        ListResponseBuilder::<T>::default()
    }
}
