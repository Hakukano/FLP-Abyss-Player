# FLP Abyss Player

This is a media player that allows you go through the entire folder of media recursively like looking into an abyss.

# Install

## Install From crates.io

-`cargo install flp-abyss-player`
- Download the assets from release page
- Setup environment variable to point to the assets

## Run From Source

- Clone this repo
- Download the assets from release page
- `cargo run --release`

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

# Env

```
FONT_PATH= # The path to font directory
FONTS= # The font files to be installed, separated by ; and the first one will be used as default
LOCALE_PATH= # The path to locale directory
LOCALE= # all lower cases. e.g. en_us
```

# Config

## Media Type

Choose the media type to player.

## Root Path

The player will looking for all media file with said type under this path recursively.

# View

## Keyboard Navigation

- ArrowRight: next media
- ArrowLeft: previous media
- R: random media

# TODO

- Implement image cache
- Implement video player
- Add more locales
