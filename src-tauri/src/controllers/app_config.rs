use serde_json::Value;

use super::{ApiResult, Response};
use crate::models::app_config::AppConfig;

pub fn index(app_config: &dyn AppConfig) -> ApiResult {
    Response::ok(app_config.to_json().map_err(|err| {
        error!("Serialization error: {}", err);
        Response::internal_server_error()
    })?)
}

pub fn update(args: Value, app_config: &mut dyn AppConfig) -> ApiResult {
    app_config
        .set_from_json(args)
        .map_err(|err| Response::bad_request(format!("Invalid arguments: {}", err)))?;
    Ok(Response::no_content())
}
