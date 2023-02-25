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
- Download the assets from release page
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

Not supported yet.

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
}
```

## Media Type

Choose the media type to player.

## Root Path

The player will looking for all media file with said type under this path recursively.

## Video Player

Video player to play the videos.

# View

## Keyboard Navigation

- ArrowRight: next media
- ArrowLeft: previous media
- R: random media

# TODO

- Implement image cache
- Implement video player
- Add more locales
