$ErrorActionPreference = "Stop"

$cargo = "cargo.exe"
if ((Get-Command "$cargo" -ErrorAction SilentlyContinue) -eq $null) { 
   throw "Unable to find $cargo in your PATH"
}

$org = "FLProject"
$package = "flp-abyss-player"
$version = "0.3.0"

$bin_dir = "$env:PROGRAMFILES\$org\$package\$version"
$bin_path = "$bin_dir\bin\$package.exe"

$data_dir = "$env:LOCALAPPDATA\$org\$package\$version"

$assets_url = "https://github.com/Hakukano/FLP-Abyss-Player/releases/download/v$version/assets.zip"
$assets_zip = "$data_dir\assets.zip"
$assets_dir = "$data_dir\assets"

$start_menu_dir = "$env:USERPROFILE\Start Menu\Programs\$org"
$start_menu_shortcut = "$start_menu_dir\$package-$version.lnk"

$font_path = "$assets_dir\font"
$fonts = "NotoSansCJKjp-Regular.otf;Inter-Regular.ttf"
$locale_path = "$assets_dir\locale"
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

# Install binary
rm -ErrorAction Ignore -Recurse -Force $bin_dir
md -Force $bin_dir
& $cargo install --version $version --root $bin_dir $package

# Create app data
rm -ErrorAction Ignore -Recurse -Force $data_dir
md -Force $data_dir
# Download assets
rm -ErrorAction Ignore -Force $assets_zip
wget -Uri $assets_url -OutFile $assets_zip
# Unzip assets
Expand-Archive -LiteralPath $assets_zip -DestinationPath $data_dir
rm -ErrorAction Ignore -Force $assets_zip

# Add shortcut to start menu
rm -ErrorAction Ignore -Force $start_menu_shortcut
if (!(Test-Path $start_menu_dir)) {
  md -Force $start_menu_dir
}
$obj_shell = New-Object -ComObject ("WScript.Shell")
$obj_short_cut = $obj_shell.CreateShortcut($start_menu_shortcut)
$obj_short_cut.TargetPath = $bin_path
$obj_short_cut.Arguments = "--font-path `"$font_path`" --fonts `"$fonts`" --locale-path `"$locale_path`" --locale `"$locale`""
$obj_short_cut.Save()

# Add context menu item
if (!(Test-Path $hkcr)) {
  New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
}
# Image
if (Test-Path $reg_image) {
  Remove-Item -Path $reg_image -Recurse
}
New-Item -Path $reg_shell -Name $reg_image_name
New-Item -Path $reg_image -Name $reg_image_command_name
Set-ItemProperty -Path $reg_image_command -Name '(Default)' -Value "`"$bin_path`" --font-path `"$font_path`" --fonts `"$fonts`" --locale-path `"$locale_path`" --locale `"$locale`" --media-type `"image`" --root-path `"%V`""
# VLC
if (Test-Path $reg_vlc) {
  Remove-Item -Path $reg_vlc -Recurse
}
New-Item -Path $reg_shell -Name $reg_vlc_name
New-Item -Path $reg_vlc -Name $reg_vlc_command_name
Set-ItemProperty -Path $reg_vlc_command -Name '(Default)' -Value "`"$bin_path`" --font-path `"$font_path`" --fonts `"$fonts`" --locale-path `"$locale_path`" --locale `"$locale`" --media-type `"video`" --root-path `"%V`" --video-player `"vlc`" --video-player-path `"$vlc_bin_path`""
