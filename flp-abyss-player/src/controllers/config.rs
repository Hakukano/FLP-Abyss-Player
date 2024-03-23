use crate::{models::config::Config, views::Packet};

pub fn get(id: &str) -> Packet {
    Config::find(id).map(|config| {
        Packet::new(
            vec!["configs".to_string(), id.to_string()],
            "got".to_string(),
            config,
        )
    })
}
