# 第四章：状态管理：Context、Entity 与模式

在 GPUI 中，状态管理不再是“随处可见的全局变量”，而是一套严密的“所有权与通知”体系。

## 4.1 Context (cx)：全能的管家

`cx`（通常是 `ViewContext` 或 `WindowContext`）是你与 GPUI 交互的唯一桥梁。
*   **创建状态**: `cx.new(|cx| ...) -> Entity<T>`
*   **更新状态**: `entity.update(cx, |t, cx| ...)`
*   **发送通知**: `cx.notify()`
*   **全局数据**: `cx.global::<Theme>()`

## 4.2 Entity：有身份的状态

当一个结构体被包装在 `Entity<T>` 中时，它就拥有了“身份”：
1.  **稳定引用**: 即使 `Entity` 内部的数据变了，它的 Handle 依然有效。
2.  **订阅机制**: 你可以观察（Observe）一个 Entity 的变化。

## 4.3 组件间通信：Observe 与 Subscribe

这是 GPUI 状态流转的核心。

### Observe (观察者模式)
当父组件需要根据子组件的状态改变而重绘时：
```rust
// src/app.rs
let obs = cx.observe(&tab, |this, tab, cx| {
    // 当 tab 内部执行了 cx.notify() 时，这个闭包会触发
    cx.notify(); // 通知父组件也重绘
});
```

### Subscribe (订阅模式/事件中心)
当子组件需要主动告知父组件某个特定事件发生时（类似于 EventEmitter）：
```rust
// 1. 在子组件定义事件
impl EventEmitter<MyEvent> for ApiTab {}

// 2. 在子组件触发
cx.emit(MyEvent::RequestStarted);

// 3. 在父组件订阅
cx.subscribe(&tab, |this, tab, event, cx| {
    match event {
        MyEvent::RequestStarted => { ... }
    }
});
```

## 4.4 状态管理心法

1.  **向下传递 Handle**: 父组件将 `Entity<T>` 传给子组件，子组件通过 `.update()` 修改它。
2.  **向上传递 Event**: 子组件通过 `emit` 告知父组件重要动作。
3.  **单向数据流**: 尽量保证数据的修改源头是清晰的，利用 `WeakEntity` 避免循环引用。

掌握了状态管理，你就能构建出响应式的 UI。但当涉及到耗时的网络请求时，我们还需要异步并发，这正是第六章的内容。
