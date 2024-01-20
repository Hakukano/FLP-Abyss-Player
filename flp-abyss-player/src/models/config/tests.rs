use super::*;

mod media_type {
    use crate::library::helper::fixtures_dir;

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

mod video_player {}
