#!/bin/bash

# Build for macOS (Universal Binary with Ad-hoc Signing)
set -e

APP_NAME="HiPoster"
BINARY_NAME="hiposter-gpui"
BUNDLE_ID="com.obity.hiposter"
VERSION=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)

echo "Building Universal Binary for $APP_NAME version $VERSION..."

# Ensure targets are present
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Build for Intel
echo "Compiling for Intel (x86_64)..."
cargo build --release --target x86_64-apple-darwin

# Build for Apple Silicon
echo "Compiling for Apple Silicon (aarch64)..."
cargo build --release --target aarch64-apple-darwin

# Create the bundle structure
APP_DIR="target/release/$APP_NAME.app"
rm -rf "$APP_DIR"
mkdir -p "$APP_DIR/Contents/MacOS"
mkdir -p "$APP_DIR/Contents/Resources"

# Combine into Universal Binary using lipo
echo "Creating Universal Binary..."
lipo -create \
    "target/x86_64-apple-darwin/release/$BINARY_NAME" \
    "target/aarch64-apple-darwin/release/$BINARY_NAME" \
    -output "$APP_DIR/Contents/MacOS/$BINARY_NAME"

# Copy resources
cp "resources/Info.plist" "$APP_DIR/Contents/Info.plist"
cp "resources/icons/icon.icns" "$APP_DIR/Contents/Resources/icon.icns"

# Ad-hoc Code Signing
echo "Applying Ad-hoc Signing..."
codesign --force --deep --sign - "$APP_DIR"

echo "Successfully built Universal App at $APP_DIR"
lipo -info "$APP_DIR/Contents/MacOS/$BINARY_NAME"

# Package into .dmg
TEMP_DIR="target/release/dmg_temp"
DMG_NAME="target/release/$APP_NAME.dmg"

echo "Packaging Universal App into .dmg..."
rm -rf "$TEMP_DIR"
rm -f "$DMG_NAME"
mkdir -p "$TEMP_DIR"

cp -r "$APP_DIR" "$TEMP_DIR/"
ln -s /Applications "$TEMP_DIR/Applications"

hdiutil create -volname "$APP_NAME" -srcfolder "$TEMP_DIR" -ov -format UDZO "$DMG_NAME"

echo "Successfully created final $DMG_NAME"
