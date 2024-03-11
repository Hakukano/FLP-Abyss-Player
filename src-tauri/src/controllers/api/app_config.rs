use serde_json::Value;

use crate::services::app_config::AppConfigService;

use super::{ApiResult, Response};

pub fn index(app_config_service: &dyn AppConfigService) -> ApiResult {
    Response::ok(app_config_service.all())
}

pub fn update(args: Value, app_config_service: &mut dyn AppConfigService) -> ApiResult {
    app_config_service
        .save(
            serde_json::from_value(args)
                .map_err(|err| Response::bad_request(format!("Invalid arguments: {}", err)))?,
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
        let rhs = Response::ok(mock_app_config.all());
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn update() {
        let mut mock_app_config_service = mock_app_config_default();
        super::update(
            json!({
                "locale": "xx_YY"
            }),
            mock_app_config_service.as_mut(),
        )
        .unwrap();
        let mock_app_config = mock_app_config_service.all();
        let lhs = mock_app_config.locale.as_str();
        let rhs = "xx_YY".to_string();
        assert_eq!(lhs, rhs);
    }
}
