use std::path::Path;

use anyhow::Result;
use async_trait::async_trait;
use derive_builder::{Builder, UninitializedFieldError};
use phf::phf_map;
use serde::{Deserialize, Serialize};

pub mod memory;

pub static MIME_TYPES: phf::Map<&'static str, &'static str> = phf_map! {
    "bmp" => "image/bmp",
    "gif" => "image/gif",
    "jpeg" => "image/jpeg",
    "jpg" => "image/jpeg",
    "png" => "image/png",
    "avi" => "video/x-msvideo",
    "mp4" => "video/mp4",
    "webm" => "video/webm",
    "mp3" => "audio/mpeg",
    "wav" => "audio/wav",
};

#[derive(Clone, Deserialize, Serialize, Builder)]
#[builder(build_fn(skip))]
pub struct Data {
    pub id: u32,
    pub path: String,
    pub mime_type: String,
}

impl Data {
    pub fn builder() -> DataBuilder {
        DataBuilder::default()
    }
}

impl DataBuilder {
    fn build(&self) -> Result<Data, DataBuilderError> {
        let path = Clone::clone(
            self.path
                .as_ref()
                .ok_or(DataBuilderError::from(UninitializedFieldError::new("path")))?,
        );
        let p = Path::new(path.as_str());
        let extension = p
            .extension()
            .ok_or(DataBuilderError::ValidationError(
                "File extension not found".to_string(),
            ))?
            .to_str()
            .ok_or(DataBuilderError::ValidationError(
                "Invalid file extension".to_string(),
            ))?;
        let mime_type = MIME_TYPES
            .get(extension)
            .ok_or(DataBuilderError::ValidationError(
                "Unknown file extension".to_string(),
            ))?
            .to_string();
        Ok(Data {
            id: Clone::clone(
                self.id
                    .as_ref()
                    .ok_or(DataBuilderError::UninitializedField("id"))?,
            ),
            path,
            mime_type,
        })
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
    use crate::view::player::server::service::{ListQuery, ListResponse};
    pub type Query = ListQuery;
    pub type Response = ListResponse<super::Data>;
}

#[async_trait]
pub trait Playlist: Send + Sync {
    async fn read(&self, query: read::Query) -> Result<Option<read::Response>>;
    async fn list(&self, query: list::Query) -> Result<list::Response>;
}
