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
fn playlist() {
    let lhs = app_config_default().playlist();
    let rhs = None;
    assert_eq!(lhs, rhs);
}

#[test]
fn set_playlist() {
    let lhs = app_config_default()
        .tap_mut(|config| config.set_playlist(Some("/test".into())).unwrap())
        .playlist();
    let rhs = Some("/test".into());
    assert_eq!(lhs, rhs)
}

#[test]
fn to_json() {
    let lhs = app_config_default().to_json().unwrap();
    let rhs = json!({
        "locale": system_locale(),
        "playlist": null,
    });
    assert_eq!(lhs, rhs);
}

#[test]
fn set_from_json() {
    let lhs = app_config_default().tap_mut(|config| {
        config
            .set_from_json(json!({
                "locale": "xx_YY",
                "playlist": "/test",
            }))
            .unwrap();
    });
    let lhs = (lhs.locale(), lhs.playlist());
    let rhs = ("xx_YY".to_string(), Some(Path::new("/test").to_path_buf()));
    assert_eq!(lhs, rhs);
}
