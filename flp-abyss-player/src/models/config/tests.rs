use super::*;

mod media_type {
    use super::*;

    #[test]
    fn media_type_is_unset() {
        Config::set_media_type(MediaType::Unset);
        assert!(Config::media_type().is_unset());
    }
}
