use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;

use crate::models::app_config::AppConfig;

pub async fn index() -> Response {
    Json(AppConfig::all()).into_response()
}

pub async fn update(Json(body): Json<AppConfig>) -> Response {
    body.save()
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|err| {
            error!("Cannot update app config: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}
