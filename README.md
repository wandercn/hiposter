# HiPoster

HiPoster is a high-performance, modern API testing tool built with Rust and the GPUI framework. It offers a fast, fluid, and native desktop experience designed to streamline your daily HTTP request debugging.

## Features

- **🚀 High Performance**: Built entirely in Rust and GPU-accelerated via Zed's GPUI framework.
- **💻 Cross-Platform**: Native builds available for macOS (Universal Binary for Intel & Apple Silicon), with scripts provided for Windows and Linux.
- **🎨 Beautiful Themes**: Comes with multiple built-in color schemes including GitHub Light, Solarized Light, One Light, Vitesse Light, and Catppuccin Latte.
- **🔄 Complete HTTP Support**: Full support for query parameters, custom headers, and multiple body types (`application/json`, `multipart/form-data`, `application/x-www-form-urlencoded`, `text/plain`, etc.).
- **🔒 Authentication**: Built-in support for Bearer Token and Basic Auth.
- **📚 History Tracking**: Automatically saves your request history for quick access and re-execution.
- **✨ Syntax Highlighting**: Auto-formats and syntax-highlights JSON requests and responses.

## Installation

You can build HiPoster from source.

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- On macOS, you will also need Xcode command-line tools.

### Building for macOS

To build a Universal Binary (supports both Intel and Apple Silicon Macs) packaged as a `.dmg`:

```bash
./scripts/build_macos.sh
```

The resulting `HiPoster.dmg` will be available in the `target/release/` directory.

### Building for Linux / Windows

Scripts are provided for compiling the app on Linux and Windows platforms:

```bash
# For Linux
./scripts/build_linux.sh

# For Windows
./scripts/build_windows.sh
```

## About

- **Version**: 0.1.0
- **Author**: wander
- **Source Code**: [https://github.com/wandercn/hiposter](https://github.com/wandercn/hiposter)

## License

This project is licensed under the MIT License.
