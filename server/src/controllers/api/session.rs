use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::services::Services;

#[derive(Deserialize, Serialize)]
pub struct WriteArgs {
    path: String,
}
pub async fn save(services: State<Services>, Json(body): Json<WriteArgs>) -> Response {
    services
        .session
        .read()
        .save(
            body.path.as_str(),
            services.playlist.read().as_ref(),
            services.group.read().as_ref(),
            services.entry.read().as_ref(),
        )
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())
}

#[derive(Deserialize, Serialize)]
pub struct ReadArgs {
    path: String,
}
pub async fn load(services: State<Services>, Json(body): Json<ReadArgs>) -> Response {
    services
        .session
        .write()
        .load(
            body.path.as_str(),
            services.playlist.write().as_mut(),
            services.group.write().as_mut(),
            services.entry.write().as_mut(),
        )
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|err| (StatusCode::BAD_REQUEST, err.to_string()).into_response())
}
