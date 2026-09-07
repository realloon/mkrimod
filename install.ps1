$ErrorActionPreference = 'Stop'

$repo = "realloon/create-rimworld-mod"
$bin = "mkrimod"
$dest = if ($env:CARGO_HOME) { "$env:CARGO_HOME\bin" } else { "$HOME\.local\bin" }

$url = "https://github.com/$repo/releases/latest/download/$bin-x86_64-pc-windows-msvc.zip"
$zipPath = "$env:TEMP\$bin-$([guid]::NewGuid().ToString('N')).zip"
$tempDir = "$env:TEMP\$bin-extract-$([guid]::NewGuid().ToString('N'))"

Write-Host "Downloading $bin..."
Invoke-WebRequest -Uri $url -OutFile $zipPath

New-Item -ItemType Directory -Force -Path $tempDir | Out-Null
Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

$binPath = (Get-ChildItem -Path $tempDir -Filter "$bin.exe" -Recurse | Select-Object -First 1).FullName
if (-not $binPath) {
    Remove-Item -Recurse -Force $tempDir, $zipPath
    throw "Failed to find $bin.exe in downloaded archive."
}

New-Item -ItemType Directory -Force -Path $dest | Out-Null
Copy-Item -Path $binPath -Destination "$dest\$bin.exe" -Force
Remove-Item -Recurse -Force $tempDir, $zipPath

if ($env:Path -notlike "*$dest*") {
    [Environment]::SetEnvironmentVariable("Path", "$env:Path;$dest", "User")
    $env:Path += ";$dest"
}

Write-Host "$bin installed successfully to $dest"
