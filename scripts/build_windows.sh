#!/bin/bash

# Build for Windows
set -e

APP_NAME="HiPoster"
VERSION="0.1.0"

echo "Building $APP_NAME for Windows..."

# This script is intended to be run on Windows (using Git Bash) 
# or in a cross-compilation environment.
# For native Windows build:
cargo build --release

echo "Successfully built target/release/hiposter-gpui.exe"
