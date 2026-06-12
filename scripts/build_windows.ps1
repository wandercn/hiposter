<#
.SYNOPSIS
    Builds the HiPoster application natively on Windows.
.DESCRIPTION
    This script compiles the project in release mode using the default MSVC toolchain
    required by the GPUI framework.
#>

$ErrorActionPreference = "Stop"

$AppName = "HiPoster"
$BinaryName = "hiposter-gpui.exe"

Write-Host "Building $AppName for Windows..." -ForegroundColor Cyan

# Check if MSVC is available (simple check via cl.exe)
if (!(Get-Command cl.exe -ErrorAction SilentlyContinue)) {
    Write-Host "WARNING: 'cl.exe' (MSVC Compiler) not found in PATH." -ForegroundColor Yellow
    Write-Host "The GPUI framework strictly requires Microsoft Visual C++ Build Tools." -ForegroundColor Red
    Write-Host ""
    Write-Host "Please ensure you have installed:" -ForegroundColor White
    Write-Host "1. Build Tools for Visual Studio 2022" -ForegroundColor White
    Write-Host "2. Workload: 'Desktop development with C++'" -ForegroundColor White
    Write-Host "3. Component: Windows 10/11 SDK" -ForegroundColor White
    Write-Host ""
    Write-Host "If installed, please run this script from the 'Developer PowerShell for VS 2022'." -ForegroundColor Cyan
}

# 1. Ensure Rust MSVC toolchain is installed
Write-Host "Checking Rust MSVC toolchain..." -ForegroundColor Cyan
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc

# 2. Build the project
Write-Host "Compiling project..." -ForegroundColor Yellow
cargo build --release

# 3. Prepare the output package directory
$BuildDir = "target\windows_release"
if (Test-Path -Path $BuildDir) {
    Remove-Item -Path $BuildDir -Recurse -Force
}
New-Item -Path $BuildDir -ItemType Directory | Out-Null

# 4. Copy the compiled executable to the output directory
$SourceExe = "target\release\$BinaryName"
if (Test-Path -Path $SourceExe) {
    Copy-Item -Path $SourceExe -Destination $BuildDir\
    Write-Host "Successfully built Windows binary at $BuildDir\$BinaryName" -ForegroundColor Green
} else {
    Write-Error "Failed to find the compiled executable at $SourceExe"
}
