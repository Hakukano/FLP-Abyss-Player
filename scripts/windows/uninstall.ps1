$ScriptDirectory = Split-Path -Path $MyInvocation.MyCommand.Definition -Parent
try {
    . ("$ScriptDirectory\common.ps1")
}
catch {
    Write-Host "Error while loading supporting PowerShell Scripts" 
}

# Disassociate playlist file
cmd /c ftype $playlist_filetype=
cmd /c assoc $playlist_extension=

# Remove context menu item
if (!(Test-Path $hkcr)) {
  New-PSDrive -PSProvider registry -Root HKEY_CLASSES_ROOT -Name HKCR
}
if (Test-Path $reg_top) {
  Remove-Item -Path $reg_top -Recurse
}

# Remove shortcut from start menu
rm -ErrorAction Ignore -Force $start_menu_shortcut

# Delete app data
rm -ErrorAction Ignore -Recurse -Force $data_dir

# Uninstall binary
rm -ErrorAction Ignore -Recurse -Force $bin_dir
