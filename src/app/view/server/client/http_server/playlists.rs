use std::{io::SeekFrom, ops::Deref};

use async_trait::async_trait;
use axum::{
    body::StreamBody,
    extract::{FromRequestParts, Path, Query},
    http::{header::RANGE, request::Parts, StatusCode},
    response::{AppendHeaders, IntoResponse},
    routing, Extension, Json, Router,
};
use http_range_header::ParsedRanges;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};
use tokio_util::io::ReaderStream;

use crate::app::view::server::service::playlist;

struct RangesHeader(ParsedRanges);

impl Deref for RangesHeader {
    type Target = ParsedRanges;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RangesHeader
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let range_header_str = parts
            .headers
            .get(RANGE)
            .ok_or((StatusCode::BAD_REQUEST, "`Range` header is missing"))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "`Authorization` header contains invalid characters",
                )
            })?;

        let ranges = http_range_header::parse_range_header(range_header_str).unwrap();

        Ok(Self(ranges))
    }
}

async fn list(
    state: Extension<super::HttpServer>,
    query: Query<playlist::list::Query>,
) -> impl IntoResponse {
    match state.playlist.list(query.0).await {
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
    }
}

async fn read(state: Extension<super::HttpServer>, Path(id): Path<u32>) -> impl IntoResponse {
    match state
        .playlist
        .read(playlist::read::Query::builder().id(id).build().unwrap())
        .await
    {
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, ()).into_response(),
        Ok(Some(res)) => (StatusCode::OK, Json(res)).into_response(),
    }
}

async fn stream(
    state: Extension<super::HttpServer>,
    ranges: Option<RangesHeader>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match state
        .playlist
        .read(playlist::read::Query::builder().id(id).build().unwrap())
        .await
    {
        Err(err) => Ok((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()),
        Ok(None) => Ok((StatusCode::NOT_FOUND, ()).into_response()),
        Ok(Some(res)) => {
            let mut file = File::open(res.path).await.map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            })?;
            let meta = file.metadata().await.map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            })?;
            if res.mime_type.starts_with("image/") {
                Ok((
                    AppendHeaders([
                        (CONTENT_TYPE, res.mime_type),
                        (CONTENT_LENGTH, meta.len().to_string()),
                    ]),
                    StreamBody::new(ReaderStream::new(file)),
                )
                    .into_response())
            } else if res.mime_type.starts_with("video/") || res.mime_type.starts_with("audio/") {
                let start = *ranges
                    .ok_or((StatusCode::BAD_REQUEST, "Range is needed").into_response())?
                    .validate(meta.len())
                    .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())?
                    .first()
                    .ok_or((StatusCode::BAD_REQUEST, "Range is needed").into_response())?
                    .clone()
                    .start();
                let end = (meta.len() - 1).min(start + 999_999);
                let content_length = end - start + 1;
                let content_range = format!("bytes {start}-{end}/{}", meta.len());
                file.seek(SeekFrom::Start(start)).await.map_err(|err| {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                })?;
                let file = file.take(content_length);
                Ok((
                    StatusCode::PARTIAL_CONTENT,
                    AppendHeaders([
                        (CONTENT_TYPE, res.mime_type),
                        (CONTENT_LENGTH, content_length.to_string()),
                        (CONTENT_RANGE, content_range),
                        (ACCEPT_RANGES, "bytes".to_string()),
                    ]),
                    StreamBody::new(ReaderStream::new(file)),
                )
                    .into_response())
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Unknown mime type").into_response())
            }
        }
    }
}

pub fn route() -> Router {
    Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(read))
        .route("/:id/stream", routing::get(stream))
}
