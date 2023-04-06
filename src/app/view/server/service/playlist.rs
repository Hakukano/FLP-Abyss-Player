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
    pub struct Request {
        pub id: u32,
    }
    impl Request {
        pub fn builder() -> RequestBuilder {
            RequestBuilder::default()
        }
    }
    pub type Response = super::Data;
}

pub mod list {
    use crate::app::view::server::service::{ListRequest, ListResponse};
    pub type Request = ListRequest;
    pub type Response = ListResponse<super::Data>;
}

#[async_trait]
pub trait Playlist {
    async fn read(&self, req: read::Request) -> Result<read::Response>;
    async fn list(&self, req: list::Request) -> Result<list::Response>;
}
