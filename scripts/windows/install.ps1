param (
  [switch]$build = $false
)

$ScriptDirectory = Split-Path -Path $MyInvocation.MyCommand.Definition -Parent
try {
  . ("$ScriptDirectory\common.ps1")
}
catch {
  Write-Host "Error while loading supporting PowerShell Scripts"
}

# Install binary
rm -ErrorAction Ignore -Recurse -Force $bin_dir
mkdir -Force $bin_dir
if ($build) {
  & $cargo install --version $version --root $bin_dir $package
}
else {
  mkdir -Force "$bin_dir\bin"
  wget -Uri $bin_url -OutFile $bin_path
}

# Create app data
rm -ErrorAction Ignore -Recurse -Force $data_dir
mkdir -Force $data_dir
# Download assets
rm -ErrorAction Ignore -Force $assets_zip
wget -Uri $assets_url -OutFile $assets_zip
# Unzip assets
Expand-Archive -LiteralPath $assets_zip -DestinationPath $data_dir
rm -ErrorAction Ignore -Force $assets_zip

# Add shortcut to start menu
rm -ErrorAction Ignore -Force $start_menu_shortcut
if (!(Test-Path $start_menu_dir)) {
  mkdir -Force $start_menu_dir
}
$obj_shell = New-Object -ComObject ("WScript.Shell")
$obj_short_cut = $obj_shell.CreateShortcut($start_menu_shortcut)
$obj_short_cut.TargetPath = "$bin_path"
$obj_short_cut.Arguments = "--assets-path `"$assets_dir`""
$obj_short_cut.Save()

# Add context menu item
if (!(Test-Path $hkcr)) {
  New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
}
# Top
if (Test-Path $reg_top) {
  Remove-Item -Path $reg_top -Recurse
}
New-Item -Path $reg_shell -Name $reg_top_name
Set-ItemProperty -Path $reg_top -Name 'MUIVerb' -Value "$reg_top_name"
Set-ItemProperty -Path $reg_top -Name 'subcommands' -Value ""
if (Test-Path $reg_top_shell) {
  Remove-Item -Path $reg_top_shell -Recurse
}
New-Item -Path $reg_top -Name $reg_top_shell_name
# Server
if (Test-Path $reg_server) {
  Remove-Item -Path $reg_server -Recurse
}
New-Item -Path $reg_top_shell -Name $reg_server_name
New-Item -Path $reg_server -Name $reg_server_command_name
Set-ItemProperty -Path $reg_server_command -Name '(Default)' -Value "`"$bin_path`" --assets-path `"$assets_dir`" --media-type `"server`" --root-path `"%V`""
# Image
if (Test-Path $reg_image) {
  Remove-Item -Path $reg_image -Recurse
}
New-Item -Path $reg_top_shell -Name $reg_image_name
New-Item -Path $reg_image -Name $reg_image_command_name
Set-ItemProperty -Path $reg_image_command -Name '(Default)' -Value "`"$bin_path`" --assets-path `"$assets_dir`" --media-type `"image`" --root-path `"%V`""
# Video Native
if (Test-Path $reg_video_native) {
  Remove-Item -Path $reg_video_native -Recurse
}
New-Item -Path $reg_top_shell -Name $reg_video_native_name
New-Item -Path $reg_video_native -Name $reg_video_native_command_name
Set-ItemProperty -Path $reg_video_native_command -Name '(Default)' -Value "`"$bin_path`" --assets-path `"$assets_dir`" --media-type `"video`" --root-path `"%V`" --video-player `"native`""
# VLC
if (Test-Path $reg_vlc) {
  Remove-Item -Path $reg_vlc -Recurse
}
New-Item -Path $reg_top_shell -Name $reg_vlc_name
New-Item -Path $reg_vlc -Name $reg_vlc_command_name
Set-ItemProperty -Path $reg_vlc_command -Name '(Default)' -Value "`"$bin_path`" --assets-path `"$assets_dir`" --media-type `"video`" --root-path `"%V`" --video-player `"vlc`" --video-player-path `"$vlc_bin_path`""

# Associate playlist file
cmd /c assoc $playlist_extension=$playlist_filetype
cmd /c "ftype $playlist_filetype=`"$bin_path`" --assets-path `"$assets_dir`" --playlist-path `"%1`""
