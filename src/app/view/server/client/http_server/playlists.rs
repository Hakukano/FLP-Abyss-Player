use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing, Extension, Json, Router,
};

use crate::app::view::server::service::playlist;

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

async fn list(
    state: Extension<super::HttpServer>,
    query: Query<playlist::list::Query>,
) -> impl IntoResponse {
    match state.playlist.list(query.0).await {
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
    }
}

pub fn route() -> Router {
    Router::new()
        .route("/", routing::get(list))
        .route("/:id", routing::get(read))
}
