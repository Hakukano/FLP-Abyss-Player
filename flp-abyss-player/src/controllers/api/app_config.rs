use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;

use crate::{models::app_config::AppConfig, services::Services};

pub async fn index(services: State<Services>) -> Response {
    Json(services.app_config.read().all()).into_response()
}

pub async fn update(services: State<Services>, Json(body): Json<AppConfig>) -> Response {
    services
        .app_config
        .write()
        .save(body)
        .map(|_| StatusCode::NO_CONTENT.into_response())
        .unwrap_or_else(|err| {
            error!("Cannot update app config: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        })
}
