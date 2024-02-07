use tauri::State;

use super::Response;
use crate::models;

pub fn index(models: State<models::Models>) -> Result<Response, Response> {
    Response::ok(models.app_config.read().to_json().map_err(|err| {
        error!("Serialization error: {}", err);
        Response::internal_server_error()
    })?)
}
