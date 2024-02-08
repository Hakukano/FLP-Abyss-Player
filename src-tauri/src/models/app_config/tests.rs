use super::*;

pub fn mock_default() -> Box<dyn AppConfig> {
    instantiate()
}

#[test]
fn locale() {
    assert_eq!(mock_default().locale(), system_locale());
}
