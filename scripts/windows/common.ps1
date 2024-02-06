$ErrorActionPreference = "Stop"

$cargo = "cargo.exe"
if ((Get-Command "$cargo" -ErrorAction SilentlyContinue) -eq $null) { 
   Write-Host "Unable to find $cargo in your PATH"
}

$org = "FLProject"
$package = "flp-abyss-player"
$package_display_name = "FLP Abyss Player"
$version = "0.8.0"
$download_url = "https://github.com/Hakukano/FLP-Abyss-Player/releases/download/v$version"

$bin_file = "$package.exe"
$bin_url = "$download_url/$bin_file"
$bin_dir = "$env:PROGRAMFILES\$org\$package\$version"
$bin_path = "$bin_dir\bin\$bin_file"

$data_dir = "$env:LOCALAPPDATA\$org\$package\$version"

$assets_url = "$download_url/assets.zip"
$assets_zip = "$data_dir\assets.zip"
$assets_dir = "$data_dir\assets"

$start_menu_dir = "$env:USERPROFILE\Start Menu\Programs\$org"
$start_menu_shortcut = "$start_menu_dir\$package-$version.lnk"

$hkcr = "HKCR:"
$reg_shell = "$hkcr\Folder\shell"

$reg_top_name = "$package_display_name $version"
$reg_top = "$reg_shell\$reg_top_name"
$reg_top_shell_name = "shell"
$reg_top_shell = "$reg_top\$reg_top_shell_name"

$reg_server_name = "Start Server"
$reg_server = "$reg_top_shell\$reg_server_name"
$reg_server_command_name = "command"
$reg_server_command = "$reg_server\$reg_server_command_name"

$reg_image_name = "Play Images"
$reg_image = "$reg_top_shell\$reg_image_name"
$reg_image_command_name = "command"
$reg_image_command = "$reg_image\$reg_image_command_name"

$reg_video_native_name = "Play Videos With Native Player"
$reg_video_native = "$reg_top_shell\$reg_video_native_name"
$reg_video_native_command_name = "command"
$reg_video_native_command = "$reg_video_native\$reg_video_native_command_name"

$reg_vlc_name = "Play Videos With VLC"
$reg_vlc = "$reg_top_shell\$reg_vlc_name"
$reg_vlc_command_name = "command"
$reg_vlc_command = "$reg_vlc\$reg_vlc_command_name"
$vlc_bin_path = Get-Command vlc.exe -ErrorAction SilentlyContinue
if ($vlc_bin_path -eq $null) {
  $vlc_bin_path = ""
}

$playlist_filetype = "FLPAPPL"
$playlist_extension = ".fappl"
