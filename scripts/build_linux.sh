#!/bin/bash

# Build for Linux (Debian)
set -e

APP_NAME="hiposter"
VERSION="0.1.0"
MACHINE=$(uname -m)
if [ "$MACHINE" == "x86_64" ]; then
    ARCH="amd64"
else
    ARCH="arm64"
fi

echo "Building $APP_NAME for Linux..."

# Build the release binary
cargo build --release

# Create debian structure
BUILD_DIR="target/linux"
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/bin"
mkdir -p "$BUILD_DIR/usr/share/applications"
mkdir -p "$BUILD_DIR/usr/share/icons"

# Copy binary
cp "target/release/hiposter-gpui" "$BUILD_DIR/usr/bin/$APP_NAME"

# Copy icon
cp "resources/icons/logo.png" "$BUILD_DIR/usr/share/icons/$APP_NAME.png"

# Create control file
cat <<EOF > "$BUILD_DIR/DEBIAN/control"
Package: $APP_NAME
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Maintainer: wander <wander@rustpub.com>
Description: HiPoster GPUI version
 High performance API tester built with GPUI.
EOF

# Create desktop file
cat <<EOF > "$BUILD_DIR/usr/share/applications/$APP_NAME.desktop"
[Desktop Entry]
Name=HiPoster
Exec=/usr/bin/$APP_NAME
Icon=/usr/share/icons/$APP_NAME.png
Type=Application
Categories=Development;
EOF

echo "Created Linux build structure at $BUILD_DIR"
echo "To create a .deb package, run: dpkg -b $BUILD_DIR ${APP_NAME}_${VERSION}_${ARCH}.deb"
