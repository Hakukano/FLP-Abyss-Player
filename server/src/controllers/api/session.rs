use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::session::Session;

#[derive(Deserialize, Serialize)]
pub struct WriteArgs {
    path: String,
}
pub async fn save(Json(body): Json<WriteArgs>) -> Response {
    Session::save(body.path.as_str())
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())
}

#[derive(Deserialize, Serialize)]
pub struct ReadArgs {
    path: String,
}
pub async fn load(Json(body): Json<ReadArgs>) -> Response {
    Session::load(body.path.as_str())
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())
}
