# 第二章：应用入口与生命周期管理

GPUI 应用的生命周期由主线程的 Event Loop 驱动。理解 `lib.rs` 中的启动流程对于掌握窗口管理至关重要。

## 2.1 运行时初始化

GPUI 的异步操作依赖于 `tokio`。在 `main.rs` 调用 `lib::run()` 时，必须首先建立异步环境：

```rust
pub fn run() {
    let _runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime");
    let _guard = _runtime.enter(); // 确保后续 spawn 的任务都在此运行时内

    let app = gpui_platform::application().with_assets(AppAssets);
    // ...
}
```

## 2.2 窗口与根视图的绑定

在 `app.run` 回调中，我们使用 `cx.open_window`。这是应用从“进程”转变为“窗口程序”的瞬间。

```rust
cx.open_window(window_options, |window, cx| {
    // 1. 创建视图实例
    let view = cx.new(|cx| Hiposter::new(window, cx));
    
    // 2. 使用 Root 容器包装
    // Root 负责处理底层的绘制指令转发
    cx.new(|cx| gpui_component::Root::new(view, window, cx))
})
```

## 2.3 生命周期事件

*   **`cx.on_action`**: 监听系统级或自定义的全局动作（如 Quit, About）。
*   **`cx.set_menus`**: 在 macOS 上，这是设置顶部系统菜单的唯一途径。
*   **`cx.spawn`**: 开启一个脱离当前 UI 帧的任务，通常用于窗口初始化的资源加载。

## 2.4 工程细节：build.rs

为了让应用在 Windows 上不显示命令行黑框，并在各平台有图标，`build.rs` 是必不可少的。

```rust
// build.rs 片段
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("resources/icons/icon.ico");
        res.compile().unwrap();
    }
}
```

在这一章，我们把地基打好了。第三章我们将进入最有趣的环节：使用 Flexbox 画出精美的界面。
