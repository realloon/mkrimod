$dir = "$HOME\.local\bin"
if (-not (Test-Path $dir)) { New-Item -ItemType Directory -Path $dir -Force | Out-Null }
$zip = "$env:TEMP\mkrimod-win-x64.zip"

Invoke-WebRequest "https://github.com/realloon/mkrimod/releases/latest/download/mkrimod-x86_64-pc-windows-msvc.zip" -OutFile $zip -UseBasicParsing

$tempDir = "$env:TEMP\mkrimod-extract-$([guid]::NewGuid().ToString('N'))"
Expand-Archive $zip -DestinationPath $tempDir -Force
$bin = (Get-ChildItem -Path $tempDir -Filter "mkrimod.exe" -Recurse | Select-Object -First 1).FullName
Copy-Item $bin -Destination "$dir\mkrimod.exe" -Force
Remove-Item -Recurse -Force $tempDir, $zip

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -split ';' -notcontains $dir) {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$dir", "User")
    $env:Path += ";$dir"
}

Write-Host "Installed mkrimod to $dir\mkrimod.exe"
