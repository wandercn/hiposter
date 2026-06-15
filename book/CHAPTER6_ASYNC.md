# 第六章：异步并发：在 UI 应用中处理网络请求

桌面应用最忌讳的就是“无响应”。网络请求作为不确定性最高的 IO 操作，是导致 UI 假死的头号元凶。在 `HiPoster` 中，我们利用 GPUI 的异步模型实现了流畅的请求体验。

## 6.1 UI 线程与后台线程

GPUI 的主线程（Main Thread）负责：绘制、输入事件、窗口管理。
**金律**: 永远不要在主线程执行阻塞操作（如同步 HTTP、密集计算）。

## 6.2 cx.spawn：开启异步世界

GPUI 深度集成了 `tokio`。通过 `cx.spawn`，你可以轻松开启一个后台异步任务。

```rust
// src/app.rs
cx.spawn(move |_this, mut cx| {
    async move {
        // 1. 执行异步 IO (不阻塞 UI)
        let result = http::execute_request(&req).await;
        
        // 2. 更新 UI (必须切回 UI 线程)
        tab.update(&mut cx, |tab, cx| {
            tab.loading = false;
            tab.set_response(result, cx);
        });
    }
}).detach(); // detach 让任务在后台独立运行
```

## 6.3 异步环境下的安全：WeakEntity

在异步回调发生时，原有的视图可能已经被用户关闭（销毁）了。直接操作已销毁的 Entity 会导致程序崩溃。

**对策**: 使用 `WeakEntity`。
```rust
cx.spawn(|this, mut cx| {
    async move {
        let data = fetch_data().await;
        // 尝试升级为强引用，如果视图已消失，则自动忽略
        if let Some(this) = this.upgrade() {
            this.update(&mut cx, |this, cx| { ... });
        }
    }
})
```

## 6.4 请求进度与取消

在 `HiPoster` 中，我们将 `loading` 状态存入 `ApiTab`。
*   请求开始：`loading = true` -> `cx.notify()` -> 界面显示加载动画。
*   请求结束：`loading = false` -> `cx.notify()` -> 加载动画消失。

## 6.5 异步心法

1.  **最小化临界区**: 只有真正修改 UI 数据时才进入 `entity.update`。
2.  **错误捕获**: 在异步块内部使用 `match result` 捕获异常，并转换为 UI 可读的错误消息反馈给用户。
3.  **防抖与节流**: 对于输入触发的搜索等请求，利用异步原语进行防抖，避免无效的网络开销。

网络请求搞定了，下一步我们要考虑如何优化大批量数据的渲染性能，请看第七章。
