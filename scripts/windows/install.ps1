$ScriptDirectory = Split-Path -Path $MyInvocation.MyCommand.Definition -Parent
try {
    . ("$ScriptDirectory\common.ps1")
}
catch {
    Write-Host "Error while loading supporting PowerShell Scripts" 
}

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
$obj_short_cut.TargetPath = "$bin_path"
$obj_short_cut.Arguments = "--assets-path `"$assets_dir`""
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
Set-ItemProperty -Path $reg_image_command -Name '(Default)' -Value "`"$bin_path`" --assets-path `"$assets_dir`" --media-type `"image`" --root-path `"%V`""
# VLC
if (Test-Path $reg_vlc) {
  Remove-Item -Path $reg_vlc -Recurse
}
New-Item -Path $reg_shell -Name $reg_vlc_name
New-Item -Path $reg_vlc -Name $reg_vlc_command_name
Set-ItemProperty -Path $reg_vlc_command -Name '(Default)' -Value "`"$bin_path`" --assets-path `"$assets_dir`" --media-type `"video`" --root-path `"%V`" --video-player `"vlc`" --video-player-path `"$vlc_bin_path`""

# Associate playlist file
cmd /c assoc $playlist_extension=$playlist_filetype
cmd /c "ftype $playlist_filetype=`"$bin_path`" --assets-path `"$assets_dir`" --playlist-path `"%1`""
