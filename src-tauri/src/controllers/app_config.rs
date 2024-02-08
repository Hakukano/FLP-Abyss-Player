use serde_json::Value;
use tauri::State;

use super::{ApiResult, Response};
use crate::models;

pub fn index(models: State<models::Models>) -> ApiResult {
    Response::ok(models.app_config.read().to_json().map_err(|err| {
        error!("Serialization error: {}", err);
        Response::internal_server_error()
    })?)
}

pub fn update(args: Value, models: State<models::Models>) -> ApiResult {
    models
        .app_config
        .write()
        .set_from_json(args)
        .map_err(|err| Response::bad_request(format!("Invalid arguments: {}", err)))?;
    Ok(Response::no_content())
}
