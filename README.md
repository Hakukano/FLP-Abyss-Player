# FLP Abyss Player

This is a media player that allows you go through the entire folder of media recursively like looking into an abyss.

# Install

## Windows

- Download `install.ps1` from release page
- Run it

Use `uninstall.ps1` to uninstall that version.

## Install From crates.io

-`cargo install flp-abyss-player`
- Download the assets from release page

## Run From Source

- Clone this repo
- `cargo run --release -- ...`

# Usage

Run with `--help`

# Supported Media

## Images

- bmp
- gif
- jpeg/jpg
- png

## Video

- whatever vlc supports

# locale

Current supported locale:

- en\_us
- ja\_jp

# Config

## Example Config File

```
{
  "media_type": "Image",
  "root_path": "/home/example/config.json",
  "video_player": "Unset"
  "video_player_path": "/bin/vlc"
}
```

# View

## Keyboard Navigation

- ArrowRight: next media
- ArrowLeft: previous media
- R: random media
- Space: Pause/Resume media
- F: Fast forward 5s
- B: Rewind 5s

# TODO

- Implement image cache
- Implement more video player
- Improve ui: locale + interactive?
- Add more locales
