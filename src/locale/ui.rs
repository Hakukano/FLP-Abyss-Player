use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlaylistPath {
    pub label: String,
    pub unset: String,
    pub set: String,
}

#[derive(Deserialize)]
pub struct MediaType {
    pub label: String,
    pub unset: String,
    pub server: String,
    pub image: String,
    pub video: String,
}

#[derive(Deserialize)]
pub struct RootPath {
    pub label: String,
    pub unset: String,
    pub set: String,
}

#[derive(Deserialize)]
pub struct VideoPlayer {
    pub label: String,
    pub unset: String,
    #[cfg(feature = "native")]
    pub native: String,
    pub vlc: String,
}

#[derive(Deserialize)]
pub struct VideoPlayerPath {
    pub label: String,
    pub unset: String,
    pub set: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub title: String,
    pub alert: String,
    pub playlist_path: PlaylistPath,
    pub media_type: MediaType,
    pub root_path: RootPath,
    pub video_player: VideoPlayer,
    pub video_player_path: VideoPlayerPath,
    pub go: String,
}

#[derive(Deserialize)]
pub struct View {
    pub title: String,
    pub home: String,
}

#[derive(Deserialize)]
pub struct Locale {
    pub app_name: String,
    pub config: Config,
    pub view: View,
}
