use serde_json::json;
use std::path::Path;
use tap::Tap;

use super::*;

fn app_config_default() -> Box<dyn AppConfig> {
    Box::<json::AppConfig>::default()
}

#[test]
fn locale() {
    let lhs = app_config_default().locale();
    let rhs = system_locale();
    assert_eq!(lhs, rhs);
}

#[test]
fn set_locale() {
    let lhs = app_config_default()
        .tap_mut(|config| config.set_locale("xx_YY".to_string()))
        .locale();
    let rhs = "xx_YY";
    assert_eq!(lhs, rhs);
}

#[test]
fn root_path() {
    let lhs = app_config_default().root_path();
    let rhs = None;
    assert_eq!(lhs, rhs);
}

#[test]
fn set_root_path() {
    let lhs = app_config_default()
        .tap_mut(|config| config.set_root_path(Some("/test".into())).unwrap())
        .root_path();
    let rhs = Some("/test".into());
    assert_eq!(lhs, rhs)
}

#[test]
fn to_json() {
    let lhs = app_config_default().to_json().unwrap();
    let rhs = json!({
        "locale": system_locale(),
        "root_path": null,
    });
    assert_eq!(lhs, rhs);
}

#[test]
fn set_from_json() {
    let lhs = app_config_default().tap_mut(|config| {
        config
            .set_from_json(json!({
                "locale": "xx_YY",
                "root_path": "/test",
            }))
            .unwrap();
    });
    let lhs = (lhs.locale(), lhs.root_path());
    let rhs = ("xx_YY".to_string(), Some(Path::new("/test").to_path_buf()));
    assert_eq!(lhs, rhs);
}
