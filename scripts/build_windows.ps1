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
    Write-Host "Warning: GCC (MinGW-w64) is not installed or not in PATH." -ForegroundColor Yellow
    Write-Host "Because you are not using Visual Studio Build Tools, MinGW-w64 is required." -ForegroundColor Yellow
    Write-Host "Attempting to install it automatically via Scoop..." -ForegroundColor Cyan

    # Temporarily allow executing remote scripts for Scoop installation
    Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser -Force

    # Check if scoop is installed
    if (!(Get-Command scoop -ErrorAction SilentlyContinue)) {
        Write-Host "Installing Scoop..." -ForegroundColor Yellow
        iwr -useb get.scoop.sh | iex
    } else {
        Write-Host "Scoop is already installed." -ForegroundColor Green
    }

    # Install gcc
    Write-Host "Installing GCC via Scoop..." -ForegroundColor Yellow
    scoop install gcc

    Write-Host "=========================================================" -ForegroundColor Green
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host "Please RESTART your PowerShell window to apply the new PATH," -ForegroundColor Yellow
    Write-Host "and then run this script again: .\scripts\build_windows.ps1" -ForegroundColor Yellow
    Write-Host "=========================================================" -ForegroundColor Green
    exit 1
}

# 2. Ensure Rust GNU toolchain is installed
$Toolchain = "stable-x86_64-pc-windows-gnu"
Write-Host "Checking Rust GNU toolchain ($Toolchain)..." -ForegroundColor Cyan
rustup toolchain install $Toolchain

# 3. Build the project using Cargo with the GNU toolchain
Write-Host "Compiling project with GNU toolchain..." -ForegroundColor Yellow
cargo +$Toolchain build --release

# 4. Prepare the output package directory
$BuildDir = "target\windows_release"
if (Test-Path -Path $BuildDir) {
    Remove-Item -Path $BuildDir -Recurse -Force
}
New-Item -Path $BuildDir -ItemType Directory | Out-Null

# 5. Copy the compiled executable to the output directory
$SourceExe = "target\release\$BinaryName"
if (Test-Path -Path $SourceExe) {
    Copy-Item -Path $SourceExe -Destination $BuildDir\
    Write-Host "Successfully built Windows binary at $BuildDir\$BinaryName" -ForegroundColor Green
} else {
    Write-Error "Failed to find the compiled executable at $SourceExe"
}
