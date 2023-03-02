$ErrorActionPreference = "Stop"

$cargo = "cargo.exe"
if ((Get-Command "$cargo" -ErrorAction SilentlyContinue) -eq $null) { 
   throw "Unable to find $cargo in your PATH"
}

$org = "FLProject"
$package = "flp-abyss-player"
$version = "0.4.0"

$bin_dir = "$env:PROGRAMFILES\$org\$package\$version"
$bin_path = "$bin_dir\bin\$package.exe"

$data_dir = "$env:LOCALAPPDATA\$org\$package\$version"

$assets_url = "https://github.com/Hakukano/FLP-Abyss-Player/releases/download/v$version/assets.zip"
$assets_zip = "$data_dir\assets.zip"
$assets_dir = "$data_dir\assets"

$start_menu_dir = "$env:USERPROFILE\Start Menu\Programs\$org"
$start_menu_shortcut = "$start_menu_dir\$package-$version.lnk"

$fonts = "NotoSansCJKjp-Regular.otf;Inter-Regular.ttf"
$locale = switch ((Get-WinSystemLocale).name) {
  "ja-JP" { "ja_jp"; break }
  "en-US" { "en_us"; break }
  default { "en_us"; break }
}

$hkcr = "HKCR:"
$reg_shell = "$hkcr\Folder\shell"

$reg_image_name = "$package-$version-image"
$reg_image = "$reg_shell\$reg_image_name"
$reg_image_command_name = "command"
$reg_image_command = "$reg_image\$reg_image_command_name"

$reg_vlc_name = "$package-$version-vlc"
$reg_vlc = "$reg_shell\$reg_vlc_name"
$reg_vlc_command_name = "command"
$reg_vlc_command = "$reg_vlc\$reg_vlc_command_name"
$vlc_bin_path = where.exe vlc

$playlist_filetype = "FLPAPPL"
$playlist_extension = ".fappl"
