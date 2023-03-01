use chrono::Utc;

use crate::config::{MediaType, VideoPlayer};

use super::{Body, Header, Version, APPL, FLP};

fn magic(mut buffer: Vec<u8>) -> Vec<u8> {
    buffer.append(&mut FLP.to_vec());
    buffer.append(&mut APPL.to_vec());
    buffer
}

fn version_part(mut buffer: Vec<u8>, data: u8) -> Vec<u8> {
    buffer.append(&mut data.to_le_bytes().to_vec());
    buffer
}

fn version(buffer: Vec<u8>) -> Vec<u8> {
    let version = Version::new();
    version_part(
        version_part(version_part(buffer, version.major), version.minor),
        version.patch,
    )
}

fn time(mut buffer: Vec<u8>) -> Vec<u8> {
    buffer.append(&mut Utc::now().timestamp().to_le_bytes().to_vec());
    buffer
}

fn media_type(mut buffer: Vec<u8>, data: MediaType) -> Vec<u8> {
    buffer.append(&mut Into::<u8>::into(data).to_le_bytes().to_vec());
    buffer
}

fn video_player(mut buffer: Vec<u8>, data: VideoPlayer) -> Vec<u8> {
    buffer.append(&mut Into::<u8>::into(data).to_le_bytes().to_vec());
    buffer
}

fn size(mut buffer: Vec<u8>, data: u64) -> Vec<u8> {
    buffer.append(&mut data.to_le_bytes().to_vec());
    buffer
}

fn string(mut buffer: Vec<u8>, data: &str) -> Vec<u8> {
    buffer.append(&mut data.as_bytes().to_vec());
    buffer
}

pub fn header(body: &Header) -> Vec<u8> {
    let buffer = media_type(time(version(magic(Vec::new()))), body.media_type);
    if body.media_type == MediaType::Video {
        let buffer = video_player(buffer, body.video_player);
        if let Some(video_player_path) = body.video_player_path.as_ref() {
            string(
                size(buffer, video_player_path.len() as u64),
                video_player_path,
            )
        } else {
            size(buffer, 0)
        }
    } else {
        buffer
    }
}

pub fn body(buffer: Vec<u8>, data: &Body) -> Vec<u8> {
    data.item_paths
        .iter()
        .fold(buffer, |acc, cur| string(acc, cur))
}
