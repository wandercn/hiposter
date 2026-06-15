# 第十章：跨平台构建与发布流水线

开发完成并不代表结束。将 Rust 源码转换为用户可以直接运行的 `.app` 或 `.exe`，是通往成功的最后一步。

## 10.1 构建脚本化：标准化的力量

不要手动输入复杂的编译命令！`HiPoster` 为每个平台提供了专门的构建脚本。
*   `scripts/build_macos.sh`: 处理 Universal Binary、图标注入和 Ad-hoc 签名。
*   `scripts/build_windows.ps1`: 调用 MSVC 环境并生成 Windows 资源。
*   `scripts/build_linux.sh`: 生成遵循 Debian 规范的 `.deb` 包。

## 10.2 自动化的版本管理

通过在脚本中自动提取 `Cargo.toml` 的版本号，可以避免因手动更新版本而导致的不一致。

```bash
VERSION=$(grep '^version =' Cargo.toml | cut -d '"' -f 2)
```

## 10.3 macOS 的特殊性：Universal Binary

为了同时支持 Intel 和 Apple Silicon 芯片，我们需要编译两个目标并使用 `lipo` 合并：
```bash
lipo -create \
    target/x86_64-apple-darwin/release/bin \
    target/aarch64-apple-darwin/release/bin \
    -output MyApp.app/Contents/MacOS/bin
```

## 10.4 Windows 的基石：MSVC

GPUI 在 Windows 上强依赖于 Microsoft Visual C++ 环境。
**注意**: 在 Windows 上打包时，务必在“Developer PowerShell for VS 2022”中运行构建脚本。

## 10.5 持续集成 (CI) 建议

建议利用 GitHub Actions 配置跨平台构建工作流。
*   **缓存**: 缓存 `target/` 目录以加速编译（Rust 的编译速度是出名的慢）。
*   **发布**: 当推送新的 Git Tag 时，自动触发脚本并上传构件至 GitHub Releases。

## 结语：你的 GPUI 之旅才刚刚开始

通过对 `HiPoster` 的全方位解剖，你已经掌握了 GPUI 开发最核心的知识体系。GPUI 是一个充满活力且不断进化的框架，它代表了桌面应用开发的未来——极致性能与极简开发的完美统一。

拿起 Rust 这把利剑，去创造属于你的高性能应用吧！
