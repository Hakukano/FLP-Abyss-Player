use serde_json::Value;

use crate::services::app_config::AppConfigService;

use super::{ApiResult, Response};

pub fn index(app_config: &dyn AppConfigService) -> ApiResult {
    Response::ok(app_config.all())
}

pub fn update(args: Value, app_config: &mut dyn AppConfigService) -> ApiResult {
    app_config
        .save(
            serde_json::from_value(args)
                .map_err(|err| Response::bad_request(format!("Invalid arguments: {}", err))),
        )
        .map_err(|err| {
            error!("Cannot update app config: {}", err);
            Response::internal_server_error()
        })?;
    Ok(Response::no_content())
}

#[cfg(test)]
mod tests {
    use crate::services::app_config::instantiate;

    use super::*;
    use serde_json::json;

    fn mock_app_config_default() -> Box<dyn AppConfigService> {
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
                "locale": "xx_YY"
            }),
            mock_app_config.as_mut(),
        )
        .unwrap();
        let lhs = mock_app_config.locale();
        let rhs = "xx_YY".to_string();
        assert_eq!(lhs, rhs);
    }
}
