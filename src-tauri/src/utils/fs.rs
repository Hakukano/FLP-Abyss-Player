use walkdir::WalkDir;

pub fn match_mime(mime: impl AsRef<str>, patterns: impl AsRef<[String]>) -> bool {
    patterns
        .as_ref()
        .iter()
        .any(|pattern| mime.as_ref().starts_with(pattern))
}

pub fn scan_medias(root_path: String, allowed_mimes: Vec<String>) -> Vec<String> {
    WalkDir::new(root_path)
        .into_iter()
        .filter_map(|err| err.ok())
        .filter_map(|entry| {
            mime_guess::from_path(entry.path())
                .into_iter()
                .find_map(|guess| {
                    let mime = guess.to_string();
                    if match_mime(mime.as_str(), allowed_mimes.as_slice()) {
                        entry.path().to_str().map(|s| s.to_string())
                    } else {
                        None
                    }
                })
        })
        .collect()
}
