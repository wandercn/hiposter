#!/bin/bash

# Build for macOS
set -e

APP_NAME="HiPoster"
BINARY_NAME="hiposter-gpui"
BUNDLE_ID="com.obity.hiposter-gpui"
VERSION="0.1.0"

echo "Building $APP_NAME..."

# Build the release binary
cargo build --release

# Create the bundle structure
APP_DIR="target/release/$APP_NAME.app"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Copy the binary
cp "target/release/$BINARY_NAME" "$APP_DIR/Contents/MacOS/$BINARY_NAME"

# Copy resources
cp "resources/Info.plist" "$APP_DIR/Contents/Info.plist"
cp "resources/icons/icon.icns" "$APP_DIR/Contents/Resources/icon.icns"

echo "Successfully built $APP_DIR"
echo "You can find the app at $APP_DIR"
