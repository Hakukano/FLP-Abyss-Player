use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::utils::fs::scan_medias;

#[derive(Deserialize, Serialize)]
pub struct IndexArgs {
    root_path: String,
    allowed_mimes: Vec<String>,
}
pub async fn index(Query(query): Query<IndexArgs>) -> Response {
    Json(scan_medias(query.root_path, query.allowed_mimes)).into_response()
}
