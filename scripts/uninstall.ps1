$ErrorActionPreference = "Stop"

$org = "FLProject"
$package = "flp-abyss-player"
$version = "0.4.0"

$bin_dir = "$env:PROGRAMFILES\$org\$package\$version"
$bin_path = "$bin_dir\bin\$package.exe"

$data_dir = "$env:LOCALAPPDATA\$org\$package\$version"

$start_menu_shortcut = "$env:USERPROFILE\Start Menu\Programs\$org\$package-$version.lnk"

$hkcr = "HKCR:"
$reg_shell = "$hkcr\Folder\shell"

$reg_image_name = "$package-$version-image"
$reg_image = "$reg_shell\$reg_image_name"

$reg_vlc_name = "$package-$version-vlc"
$reg_vlc = "$reg_shell\$reg_vlc_name"

# Remove context menu item
if (!(Test-Path $hkcr)) {
  New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
}
if (Test-Path $reg_vlc) {
  Remove-Item -Path $reg_vlc -Recurse
}
if (Test-Path $reg_image) {
  Remove-Item -Path $reg_image -Recurse
}

# Remove shortcut from start menu
rm -ErrorAction Ignore -Force $start_menu_shortcut

# Delete app data
rm -ErrorAction Ignore -Recurse -Force $data_dir

# Uninstall binary
rm -ErrorAction Ignore -Recurse -Force $bin_dir
