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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::app_config::{instantiate, AppConfig};
    use serde_json::json;
    use std::path::Path;

    fn mock_app_config_default() -> Box<dyn AppConfig> {
        instantiate()
    }

    #[test]
    fn index() {
        let mock_app_config = mock_app_config_default();
        let lhs = super::index(mock_app_config.as_ref());
        let rhs = Response::ok(mock_app_config.to_json().unwrap());
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn update() {
        let mut mock_app_config = mock_app_config_default();
        super::update(
            json!({
                "locale": "xx_YY",
                "playlist": "/test",
            }),
            mock_app_config.as_mut(),
        )
        .unwrap();
        let lhs = (mock_app_config.locale(), mock_app_config.playlist());
        let rhs = ("xx_YY".to_string(), Some(Path::new("/test").to_path_buf()));
        assert_eq!(lhs, rhs);
    }
}
