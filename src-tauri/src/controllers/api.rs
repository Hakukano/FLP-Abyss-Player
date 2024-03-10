use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use tap::Tap;
use tauri::{AppHandle, State};

use crate::models;

pub mod app_config;
pub mod playlist;

#[derive(Debug, Deserialize)]
enum Method {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Deserialize)]
pub struct Request {
    path: Vec<String>,
    method: Method,
    args: Value,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Response {
    status: u16,
    body: Value,
}

impl Response {
    pub fn new<T>(code: u16, body: T) -> Result<Self, Self>
    where
        T: Serialize,
    {
        Ok(Self {
            status: code,
            body: serde_json::to_value(body).map_err(|err| {
                error!("Serialization error: {}", err);
                Self::internal_server_error()
            })?,
        })
    }

    pub fn ok<T>(body: T) -> Result<Self, Self>
    where
        T: Serialize,
    {
        Self::new(200, body)
    }

    pub fn created<T>(body: T) -> Result<Self, Self>
    where
        T: Serialize,
    {
        Self::new(201, body)
    }

    pub fn no_content() -> Self {
        Self {
            status: 204,
            body: serde_json::to_value("No Content").unwrap(),
        }
    }

    pub fn bad_request(reason: impl AsRef<str>) -> Self {
        Self {
            status: 400,
            body: serde_json::to_value(reason.as_ref()).unwrap(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: 404,
            body: serde_json::to_value("Not Found").unwrap(),
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            status: 405,
            body: serde_json::to_value("Method Not Allowed").unwrap(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status: 500,
            body: serde_json::to_value("Internal Server Error").unwrap(),
        }
    }
}

pub type ApiResult = Result<Response, Response>;

#[tauri::command]
pub fn api(request: Request, _app_handle: AppHandle, models: State<models::Models>) -> ApiResult {
    let api_path = request.path.join("/");
    let method = request.method.to_string();
    let args = request.args.to_string();
    info!(
        command = "api",
        path = api_path,
        method = method,
        args = args,
        "Processing command"
    );
    match request.path.iter().map(AsRef::as_ref).collect::<Vec<_>>()[..] {
        ["app_config"] => match request.method {
            Method::Get => app_config::index(models.app_config.read().as_ref()),
            Method::Put => app_config::update(request.args, models.app_config.write().as_mut()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist"] => match request.method {
            Method::Get => playlist::index(models.playlist.read().as_ref()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "groups"] => match request.method {
            Method::Post => playlist::create_groups(request.args, models.playlist.write().as_mut()),
            Method::Delete => {
                playlist::delete_groups(request.args, models.playlist.write().as_mut())
            }
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "groups", "new"] => match request.method {
            Method::Get => playlist::new_groups(request.args, models.playlist.write().as_mut()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "groups", "sort"] => match request.method {
            Method::Put => playlist::sort_groups(request.args, models.playlist.write().as_mut()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "groups", "move"] => match request.method {
            Method::Put => playlist::move_group(request.args, models.playlist.write().as_mut()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "entries"] => match request.method {
            Method::Post => {
                playlist::create_entries(request.args, models.playlist.write().as_mut())
            }
            Method::Delete => {
                playlist::delete_entries(request.args, models.playlist.write().as_mut())
            }
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "entries", "new"] => match request.method {
            Method::Get => playlist::new_entries(request.args, models.playlist.read().as_ref()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "entries", "sort"] => match request.method {
            Method::Put => playlist::sort_entries(request.args, models.playlist.write().as_mut()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "entries", "move"] => match request.method {
            Method::Put => playlist::move_entry(request.args, models.playlist.write().as_mut()),
            _ => Err(Response::method_not_allowed()),
        },
        ["playlist", "search"] => match request.method {
            Method::Get => playlist::search(request.args, models.playlist.read().as_ref()),
            _ => Err(Response::method_not_allowed()),
        },
        _ => Err(Response::not_found()),
    }
    .tap(|result| {
        let status = result
            .as_ref()
            .map(|response| response.status)
            .unwrap_or_else(|err| err.status);
        let body = result
            .as_ref()
            .map(|response| response.body.to_string())
            .unwrap_or_else(|err| err.body.to_string());
        info!(
            command = "api",
            path = api_path,
            method = method,
            args = args,
            status = status,
            body = body,
            "Command completed"
        );
    })
}
