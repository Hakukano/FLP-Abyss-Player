use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use tauri::{AppHandle, State};

use crate::models;

pub mod app_config;

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

#[derive(Serialize)]
pub struct Response {
    code: u16,
    body: Value,
}

impl Response {
    pub fn new<T>(code: u16, body: T) -> Result<Self, Self>
    where
        T: Serialize,
    {
        Ok(Self {
            code,
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
            code: 204,
            body: serde_json::to_value("No Content").unwrap(),
        }
    }

    pub fn bad_request(reason: impl AsRef<str>) -> Self {
        Self {
            code: 400,
            body: serde_json::to_value(reason.as_ref()).unwrap(),
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: 404,
            body: serde_json::to_value("Not Found").unwrap(),
        }
    }

    pub fn method_not_allowed() -> Self {
        Self {
            code: 405,
            body: serde_json::to_value("Method Not Allowed").unwrap(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            code: 500,
            body: serde_json::to_value("Internal Server Error").unwrap(),
        }
    }
}

pub type ApiResult = Result<Response, Response>;

#[tauri::command]
pub fn api(request: Request, _app_handle: AppHandle, models: State<models::Models>) -> ApiResult {
    match request.path.iter().map(AsRef::as_ref).collect::<Vec<_>>()[..] {
        ["app_config"] => match request.method {
            Method::Get => app_config::index(models),
            Method::Put => app_config::update(request.args, models),
            _ => Err(Response::method_not_allowed()),
        },
        _ => Err(Response::not_found()),
    }
}
