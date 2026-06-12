<#
.SYNOPSIS
    Builds the HiPoster application natively on Windows using the GNU toolchain.
.DESCRIPTION
    This script compiles the project in release mode using the x86_64-pc-windows-gnu target
    (avoiding the need for Visual Studio Build Tools) and packages the executable.
#>

$ErrorActionPreference = "Stop"

$AppName = "HiPoster"
$BinaryName = "hiposter-gpui.exe"
$Target = "x86_64-pc-windows-gnu"

Write-Host "Building $AppName for Windows using GNU Toolchain..." -ForegroundColor Cyan

# 1. Check for GCC (MinGW-w64)
try {
    $gcc_version = gcc --version
} catch {
    Write-Host "Error: GCC (MinGW-w64) is not installed or not in PATH." -ForegroundColor Red
    Write-Host "Because you are not using Visual Studio Build Tools, you MUST install MinGW-w64." -ForegroundColor Yellow
    Write-Host "Easiest way via Scoop in PowerShell:" -ForegroundColor White
    Write-Host "  1. iwr -useb get.scoop.sh | iex" -ForegroundColor White
    Write-Host "  2. scoop install gcc" -ForegroundColor White
    Write-Host "After installation, restart PowerShell and try again." -ForegroundColor Yellow
    exit 1
}

# 2. Ensure Rust GNU target is installed
Write-Host "Checking Rust GNU target..." -ForegroundColor Cyan
$installed_targets = rustup target list | Select-String "(installed)"
if (!($installed_targets -match $Target)) {
    Write-Host "Installing Rust target $Target..." -ForegroundColor Yellow
    rustup target add $Target
}

# 3. Build the project using Cargo with the GNU target
Write-Host "Compiling project..." -ForegroundColor Yellow
cargo build --release --target $Target

# 4. Prepare the output package directory
$BuildDir = "target\windows_release"
if (Test-Path -Path $BuildDir) {
    Remove-Item -Path $BuildDir -Recurse -Force
}
New-Item -Path $BuildDir -ItemType Directory | Out-Null

# 5. Copy the compiled executable to the output directory
$SourceExe = "target\$Target\release\$BinaryName"
if (Test-Path -Path $SourceExe) {
    Copy-Item -Path $SourceExe -Destination $BuildDir\
    Write-Host "Successfully built Windows binary at $BuildDir\$BinaryName" -ForegroundColor Green
} else {
    Write-Error "Failed to find the compiled executable at $SourceExe"
}
