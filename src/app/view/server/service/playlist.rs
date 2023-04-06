use anyhow::Result;
use async_trait::async_trait;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub mod memory;

#[derive(Clone, Deserialize, Serialize, Builder)]
pub struct Data {
    pub id: u32,
    pub path: String,
}

impl Data {
    pub fn builder() -> DataBuilder {
        DataBuilder::default()
    }
}

pub mod read {
    use derive_builder::Builder;
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize, Builder)]
    pub struct Query {
        pub id: u32,
    }
    impl Query {
        pub fn builder() -> QueryBuilder {
            QueryBuilder::default()
        }
    }
    pub type Response = super::Data;
}

pub mod list {
    use crate::app::view::server::service::{ListQuery, ListResponse};
    pub type Query = ListQuery;
    pub type Response = ListResponse<super::Data>;
}

#[async_trait]
pub trait Playlist: Send + Sync {
    async fn read(&self, query: read::Query) -> Result<Option<read::Response>>;
    async fn list(&self, query: list::Query) -> Result<list::Response>;
}
