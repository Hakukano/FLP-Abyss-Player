use std::{io::SeekFrom, ops::Deref};

use axum::{
    body::Body,
    extract::{FromRequestParts, Path},
    response::{AppendHeaders, IntoResponse, Response},
    routing::get,
    Router,
};
use http::{
    header::{ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE, RANGE},
    request::Parts,
    StatusCode,
};
use http_range_header::ParsedRanges;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};
use tokio_util::io::ReaderStream;

use crate::models::entry::Entry;

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
                    "`Range` header contains invalid characters",
                )
            })?;

        let ranges = http_range_header::parse_range_header(range_header_str).unwrap();

        Ok(Self(ranges))
    }
}

async fn entry(ranges: Option<RangesHeader>, Path(id): Path<String>) -> Result<Response, Response> {
    let entry = Entry::find(&id);
    match entry {
        None => Ok((StatusCode::NOT_FOUND, ()).into_response()),
        Some(entry) => {
            let mut file = File::open(entry.meta.path.as_str()).await.map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            })?;
            let meta = file.metadata().await.map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            })?;
            if entry.mime.starts_with("image/") {
                Ok((
                    AppendHeaders([
                        (CONTENT_TYPE, entry.mime),
                        (CONTENT_LENGTH, meta.len().to_string()),
                    ]),
                    Body::from_stream(ReaderStream::new(file)),
                )
                    .into_response())
            } else if entry.mime.starts_with("video/") || entry.mime.starts_with("audio/") {
                let range = ranges
                    .ok_or((StatusCode::BAD_REQUEST, "Range is needed").into_response())?
                    .validate(meta.len())
                    .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())?
                    .first()
                    .ok_or((StatusCode::BAD_REQUEST, "Range is needed").into_response())?
                    .clone();
                let start = *range.start();
                let end = (*range.end()).min(meta.len() - 1).min(start + 999_999);
                let content_length = end - start + 1;
                let content_range = format!("bytes {start}-{end}/{}", meta.len());
                file.seek(SeekFrom::Start(start)).await.map_err(|err| {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                })?;
                let file = file.take(content_length);
                Ok((
                    StatusCode::PARTIAL_CONTENT,
                    AppendHeaders([
                        (CONTENT_TYPE, entry.mime),
                        (CONTENT_LENGTH, content_length.to_string()),
                        (CONTENT_RANGE, content_range),
                        (ACCEPT_RANGES, "bytes".to_string()),
                    ]),
                    Body::from_stream(ReaderStream::new(file)),
                )
                    .into_response())
            } else {
                Err((StatusCode::INTERNAL_SERVER_ERROR, "Unknown mime type").into_response())
            }
        }
    }
}

pub fn router() -> Router {
    Router::new().route("/entries/:id", get(entry))
}
