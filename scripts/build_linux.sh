#!/bin/bash

# Build for Linux (Debian)
# This script compiles the binary and packages it into a .deb file.
# If running on macOS to build for Linux, it expects cross-compilation tools (like cargo-zigbuild).
set -e

APP_NAME="hiposter"
DISPLAY_NAME="HiPoster"
VERSION=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)

# Determine target architecture. Default is current machine, but can be overridden.
# E.g., `TARGET_ARCH=amd64 ./scripts/build_linux.sh`
if [ -z "$TARGET_ARCH" ]; then
    MACHINE=$(uname -m)
    if [ "$MACHINE" = "x86_64" ]; then
        TARGET_ARCH="amd64"
        RUST_TARGET="x86_64-unknown-linux-gnu"
    elif [ "$MACHINE" = "aarch64" ] || [ "$MACHINE" = "arm64" ]; then
        TARGET_ARCH="arm64"
        RUST_TARGET="aarch64-unknown-linux-gnu"
    else
        echo "Unsupported architecture: $MACHINE"
        exit 1
    fi
else
    if [ "$TARGET_ARCH" = "amd64" ]; then
        RUST_TARGET="x86_64-unknown-linux-gnu"
    elif [ "$TARGET_ARCH" = "arm64" ]; then
        RUST_TARGET="aarch64-unknown-linux-gnu"
    else
        echo "Unsupported TARGET_ARCH: $TARGET_ARCH"
        exit 1
    fi
fi

echo "Building $DISPLAY_NAME ($APP_NAME) version $VERSION for Linux ($TARGET_ARCH)..."

# Ensure target is installed
rustup target add "$RUST_TARGET"

# Check if we are cross-compiling from macOS to Linux
if [ "$(uname)" = "Darwin" ]; then
    echo "Detected macOS. Cross-compiling for Linux requires cargo-zigbuild."
    if ! command -v cargo-zigbuild &> /dev/null; then
        echo "Error: cargo-zigbuild not found."
        echo "Please install it via: brew install zig && cargo install cargo-zigbuild"
        exit 1
    fi
    echo "Compiling via cargo-zigbuild..."
    cargo zigbuild --release --target "$RUST_TARGET"
else
    # Native Linux build
    echo "Compiling via cargo..."
    cargo build --release --target "$RUST_TARGET"
fi

# Create debian structure
BUILD_DIR="target/linux_$TARGET_ARCH"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/bin"
mkdir -p "$BUILD_DIR/usr/share/applications"
mkdir -p "$BUILD_DIR/usr/share/icons"

# Copy binary
cp "target/$RUST_TARGET/release/hiposter-gpui" "$BUILD_DIR/usr/bin/$APP_NAME"

# Copy icon
cp "resources/icons/logo.png" "$BUILD_DIR/usr/share/icons/$APP_NAME.png"

# Create control file
cat <<EOF > "$BUILD_DIR/DEBIAN/control"
Package: $APP_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: $TARGET_ARCH
Maintainer: wander <wander@rustpub.com>
Description: $DISPLAY_NAME API Tester
 High performance API tester built with GPUI.
EOF

# Create desktop file
cat <<EOF > "$BUILD_DIR/usr/share/applications/$APP_NAME.desktop"
[Desktop Entry]
Name=$DISPLAY_NAME
Exec=/usr/bin/$APP_NAME
Icon=/usr/share/icons/$APP_NAME.png
Type=Application
Categories=Development;
EOF

# Fix permissions for Debian package requirements
echo "Setting correct permissions for dpkg..."
chmod 0755 "$BUILD_DIR/DEBIAN"
chmod 0644 "$BUILD_DIR/DEBIAN/control"
chmod 0755 "$BUILD_DIR/usr/bin/$APP_NAME"
chmod -R 0755 "$BUILD_DIR/usr"

DEB_FILE="target/release/${APP_NAME}_${VERSION}_${TARGET_ARCH}.deb"

# Only attempt dpkg-deb if the tool exists (usually on Linux or if installed on Mac via Homebrew)
if command -v dpkg-deb &> /dev/null; then
    echo "Creating Debian package..."
    dpkg-deb -b "$BUILD_DIR" "$DEB_FILE"
    echo "Successfully created Linux package: $DEB_FILE"
else
    echo "dpkg-deb tool not found. The application folder is ready at: $BUILD_DIR"
    echo "Please run this script on a Debian/Ubuntu system, or install dpkg on macOS (brew install dpkg) to generate the .deb file."
fi

