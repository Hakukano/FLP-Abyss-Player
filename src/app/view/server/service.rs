use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub mod playlist;

#[derive(Deserialize, Serialize, Builder)]
pub struct ListRequest {
    #[builder(default)]
    pub filter: Vec<String>,
    #[builder(default)]
    pub search: String,
    pub offset: u32,
    pub length: u32,
}

impl ListRequest {
    pub fn builder() -> ListRequestBuilder {
        ListRequestBuilder::default()
    }
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
