# FLP Abyss Player

This is a media player that allows you go through the entire folder of media recursively like looking into an abyss.

# Install

## Windows

- Download `scripts.zip` from release page
- Go to `windows`
- Run `Set-ExecutionPolicy -ExecutionPolicy ByPass -Scope Process`
- Run `install.ps1 [-native]`

Use `uninstall.ps1` to uninstall that version.

## Install From crates.io

- `cargo install flp-abyss-player`
- Download the assets from release page

## Run From Source

- Clone this repo
- `cargo run --release -- ...`

# Usage

Run with `--help`

# Features

## GStreamer

You need to install gstreamer package and build with `native` feature! It gives this player ability to play video/audio natively.

Follow [GStreamer Installation Guide](https://gstreamer.pages.freedesktop.org/gstreamer-rs/stable/latest/docs/gstreamer/#installation)

# Supported Media

## Images

- bmp
- gif
- jpeg/jpg
- png

## Video

- avi
- mkv
- mov
- mp4
- webm

# Locale

Current supported locale:

- en\_US
- ja\_JP

# Playlist

## Version Supporting

Breaking changes will only be made when bumping major versions.

## Extension

`fappl` - [f]LProject [a]byss [p]layer [p]lay[l]ist

## File Structure

### Header

- `46 4C 50`: FLP
- `41 50 50 4C`: APPL
- `XX`: version major
- `XX`: version minor
- `XX`: version patch
- `XX XX XX XX XX XX XX XX`: unix timestamp
- `XX`: media type
- <<< if media type is video
- `XX`: video player
- `XX XX XX XX XX XX XX XX`: video player path `$size`
- `(XX){$size}`: video player path
- \>>>

### Body

- REPEAT
- `XX XX XX XX XX XX XX XX`: item path `$size`
- `(XX){$size}`: item path
- TAEPER

# Config

## Example Config File

```
{
  "repeat": false,
  "auto": true,
  "auto_interval": 3,
  "loop": true,

  "playlist_path": null,

  "media_type": "Image",
  "root_path": "/home/example/config.json",
  "random": true,
  "video_player": "Unset",
  "video_player_path": "/bin/vlc"
}
```

# View

## Keyboard Navigation

### General
- J: next media
- K: previous media
- R: random media
- 1: toggle repeat current media
- 2: toggle auto play
- 3: toggle loop play
- 4: toggle random play

### Video
- Space: Pause/Resume media
- F: Fast forward 5s
- B: Rewind 5s

# TODO

- Implement image cache
- Implement more video player
- Improve ui: locale + interactive?
- Add more locales

# Assets Attribution

[Attribution List](attribution.md)
