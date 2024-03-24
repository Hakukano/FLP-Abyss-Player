use tap::prelude::*;

use crate::library::helper::fixtures_dir;

use super::*;

pub fn mock_default() -> Config {
    Config {
        id: "default".to_string(),

        locale: "en_US".to_string(),

        playlist_path: None,

        media_type: MediaType::Unset,
        root_path: fixtures_dir().to_str().map(|s| s.to_string()),
        video_player: VideoPlayer::Unset,
        video_player_path: None,
    }
}

pub fn mock_image() -> Config {
    mock_default().tap_mut(|config| config.media_type = MediaType::Image)
}

#[test]
fn can_play() {
    assert!(!mock_default().can_play());
    assert!(mock_image().can_play());
}

mod media_type {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(MediaType::default(), MediaType::Unset);
    }

    #[test]
    fn is_unset() {
        assert!(MediaType::Unset.is_unset());
        assert!(!MediaType::Image.is_unset());
    }

    #[test]
    fn supported_extensions() {
        assert!(MediaType::Unset.supported_extensions().is_empty());
        assert_eq!(
            MediaType::Image.supported_extensions(),
            &["bmp", "gif", "jpeg", "jpg", "png"]
        );
        assert_eq!(
            MediaType::Server.supported_extensions(),
            &["bmp", "gif", "jpeg", "jpg", "png", "avi", "mp4", "webm", "mp3", "wav",]
        );
        assert_eq!(
            MediaType::Video.supported_extensions(),
            &["avi", "mkv", "mov", "mp4", "webm"]
        );
    }

    #[test]
    fn find_all_paths() {
        let paths = MediaType::Image
            .find_all_paths(fixtures_dir().join("images"))
            .into_iter()
            .map(|p| p.to_str().unwrap().to_string())
            .collect::<Vec<_>>();
        assert_eq!(paths.len(), 9);
    }

    #[test]
    fn convert_u8() {
        assert_eq!(u8::from(MediaType::Unset), 0);
        assert_eq!(MediaType::from(0), MediaType::Unset);
        assert_eq!(u8::from(MediaType::Image), 1);
        assert_eq!(MediaType::from(1), MediaType::Image);
        assert_eq!(u8::from(MediaType::Video), 2);
        assert_eq!(MediaType::from(2), MediaType::Video);
        assert_eq!(u8::from(MediaType::Server), 255);
        assert_eq!(MediaType::from(255), MediaType::Server);
    }
}

mod video_player {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(VideoPlayer::default(), VideoPlayer::Unset);
    }

    #[test]
    fn is_unset() {
        assert!(VideoPlayer::Unset.is_unset());
        assert!(!VideoPlayer::Native.is_unset());
    }

    #[test]
    fn convert_u8() {
        assert_eq!(u8::from(VideoPlayer::Unset), 0);
        assert_eq!(VideoPlayer::from(0), VideoPlayer::Unset);
        assert_eq!(u8::from(VideoPlayer::Vlc), 1);
        assert_eq!(VideoPlayer::from(1), VideoPlayer::Vlc);
        assert_eq!(u8::from(VideoPlayer::Native), 255);
        assert_eq!(VideoPlayer::from(255), VideoPlayer::Native);
    }
}
