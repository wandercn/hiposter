<#
.SYNOPSIS
    Builds the HiPoster application natively on Windows.
.DESCRIPTION
    This script compiles the project in release mode and packages the executable
    into a convenient output directory. The icon is automatically embedded via build.rs.
#>

$ErrorActionPreference = "Stop"

$AppName = "HiPoster"
$BinaryName = "hiposter-gpui.exe"
$Version = "0.1.0"

Write-Host "Building $AppName for Windows natively..." -ForegroundColor Cyan

# 1. Build the project using Cargo
Write-Host "Compiling project..." -ForegroundColor Yellow
cargo build --release

# 2. Prepare the output package directory
$BuildDir = "target\windows_release"
if (Test-Path -Path $BuildDir) {
    Remove-Item -Path $BuildDir -Recurse -Force
}
New-Item -Path $BuildDir -ItemType Directory | Out-Null

# 3. Copy the compiled executable to the output directory
$SourceExe = "target\release\$BinaryName"
if (Test-Path -Path $SourceExe) {
    Copy-Item -Path $SourceExe -Destination $BuildDir\
    Write-Host "Successfully built Windows binary at $BuildDir\$BinaryName" -ForegroundColor Green
    Write-Host "The application is ready to run or be packaged into a zip/installer." -ForegroundColor Green
} else {
    Write-Error "Failed to find the compiled executable at $SourceExe"
}
