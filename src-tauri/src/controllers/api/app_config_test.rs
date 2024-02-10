use super::*;
use crate::models::app_config::AppConfig;
use serde_json::json;
use std::path::Path;

fn mock_app_config_default() -> Box<dyn AppConfig> {
    models::app_config::instantiate()
}

#[test]
fn index() {
    let mock_app_config = mock_app_config_default();
    let lhs = app_config::index(mock_app_config.as_ref());
    let rhs = Response::ok(mock_app_config.to_json().unwrap());
    assert_eq!(lhs, rhs);
}

#[test]
fn update() {
    let mut mock_app_config = mock_app_config_default();
    app_config::update(
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
