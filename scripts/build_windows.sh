#!/bin/bash

# Cross-compile for Windows from macOS/Linux
set -e

APP_NAME="HiPoster"
BINARY_NAME="hiposter-gpui.exe"
TARGET="x86_64-pc-windows-gnu"

echo "Building $APP_NAME for Windows ($TARGET)..."

# 1. Check if the Rust target is installed
if ! rustup target list | grep -q "${TARGET} (installed)"; then
    echo "Adding Rust target $TARGET..."
    rustup target add $TARGET
fi

# 2. Check if mingw-w64 is installed (required for cross-compiling to Windows)
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Error: mingw-w64 is not installed."
    echo "Please install it to cross-compile for Windows:"
    echo "  macOS (Homebrew): brew install mingw-w64"
    echo "  Ubuntu/Debian:    sudo apt install mingw-w64"
    exit 1
fi

# 3. Build the project
# We need to tell Cargo to use the mingw linker for the Windows target
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="x86_64-w64-mingw32-gcc"

echo "Compiling..."
cargo build --release --target $TARGET

# 4. Package the result
BUILD_DIR="target/windows"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

# Copy the binary
cp "target/$TARGET/release/$BINARY_NAME" "$BUILD_DIR/"

# Note: The icon is automatically embedded into the .exe via build.rs and the winres crate
# when compiling for the windows target.

echo "Successfully built Windows binary at $BUILD_DIR/$BINARY_NAME"
echo "You can transfer this .exe file to a Windows machine to run it."
