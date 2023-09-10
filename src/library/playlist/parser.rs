use chrono::{DateTime, TimeZone, Utc};
use nom::{
    bytes::complete::{tag, take},
    combinator::map,
    error::{make_error, ErrorKind, ParseError},
    sequence::{pair, tuple},
    IResult,
};

use crate::config::{MediaType, VideoPlayer};

use super::{Body, Header, Version, APPL, FLP};

fn magic(data: &[u8]) -> IResult<&[u8], ()> {
    map(pair(tag(FLP), tag(APPL)), |_| ())(data)
}

fn version_part(data: &[u8]) -> IResult<&[u8], u8> {
    map(take(1usize), |a: &[u8]| {
        u8::from_le_bytes(a[0..1].try_into().unwrap())
    })(data)
}

fn version(data: &[u8]) -> IResult<&[u8], Version> {
    let (data, version) = map(
        tuple((version_part, version_part, version_part)),
        |(major, minor, patch)| Version {
            major,
            minor,
            patch,
        },
    )(data)?;
    if !version.is_supported() {
        return Err(nom::Err::Failure(make_error(data, ErrorKind::Fail)));
    }
    Ok((data, version))
}

fn time(data: &[u8]) -> IResult<&[u8], DateTime<Utc>> {
    map(take(8usize), |a: &[u8]| {
        Utc.timestamp_opt(i64::from_le_bytes(a[0..8].try_into().unwrap()), 0)
            .unwrap()
    })(data)
}

fn media_type(data: &[u8]) -> IResult<&[u8], MediaType> {
    map(take(1usize), |a: &[u8]| {
        u8::from_le_bytes(a[0..1].try_into().unwrap()).into()
    })(data)
}

fn video_player(data: &[u8]) -> IResult<&[u8], VideoPlayer> {
    map(take(1usize), |a: &[u8]| {
        u8::from_le_bytes(a[0..1].try_into().unwrap()).into()
    })(data)
}

fn size(data: &[u8]) -> IResult<&[u8], u64> {
    map(take(8usize), |a: &[u8]| {
        u64::from_le_bytes(a[0..8].try_into().unwrap())
    })(data)
}

fn string<'a, Error>(size: u64) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], String, Error>
where
    Error: ParseError<&'a [u8]>,
{
    map(take(size as usize), |a: &[u8]| {
        String::from_utf8(a.to_vec()).unwrap()
    })
}

pub fn header(data: &[u8]) -> IResult<&[u8], Header> {
    let (data, _) = magic(data)?;
    let (data, version) = version(data)?;
    let (data, time) = time(data)?;
    let (data, media_type) = media_type(data)?;
    if media_type == MediaType::Video {
        let (data, video_player) = video_player(data)?;
        let (data, video_path_path_size) = size(data)?;
        let (data, video_player_path) = if video_path_path_size == 0 {
            (data, None)
        } else {
            let (data, s) = string(video_path_path_size)(data)?;
            (data, Some(s))
        };
        Ok((
            data,
            Header {
                version,
                time,
                media_type,
                video_player,
                video_player_path,
            },
        ))
    } else {
        Ok((
            data,
            Header {
                version,
                time,
                media_type,
                video_player: VideoPlayer::Unset,
                video_player_path: None,
            },
        ))
    }
}

pub fn body(mut data: &[u8]) -> IResult<&[u8], Body> {
    let mut item_paths = Vec::new();
    while let Ok((d, item_path_size)) = size(data) {
        data = d;
        let (d, item_path) = string(item_path_size)(data)?;
        data = d;
        item_paths.push(item_path);
    }
    Ok((data, Body { item_paths }))
}
