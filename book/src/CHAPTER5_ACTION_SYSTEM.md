# 第五章：交互进阶：Action 系统与快捷键

在传统的 UI 框架中，监听键盘和菜单事件通常是一件繁琐的事。GPUI 通过一套基于“命令”的 Action 系统，让应用的交互逻辑变得整洁且可扩展。

## 5.1 什么是 Action？

Action 是应用中“可执行指令”的抽象。它与具体的按钮或快捷键解耦。
例如：“退出应用”是一个 Action，“点击菜单项”或“按下 Cmd+Q”只是触发这个 Action 的方式。

## 5.2 定义与分发 Action

在 `src/lib.rs` 中，我们使用 `actions!` 宏定义它们：

```rust
// 1. 定义
actions!(hiposter, [OpenAbout, Quit, SendRequest]);

// 2. 绑定到逻辑
cx.on_action(|_: &Quit, cx: &mut App| {
    cx.quit();
});
```

## 5.3 快捷键绑定 (Key Bindings)

你可以在启动时全局绑定快捷键：

```rust
cx.bind_keys([
    KeyBinding::new("cmd-q", Quit, None),
    KeyBinding::new("cmd-enter", SendRequest, None),
]);
```

## 5.4 菜单系统 (Menus)

Action 还可以直接绑定到系统菜单项：

```rust
Menu {
    name: "Edit".into(),
    items: vec![
        MenuItem::action("Undo", gpui_component::input::Undo),
        MenuItem::separator(),
        MenuItem::action("Copy", gpui_component::input::Copy),
    ],
}
```

## 5.5 Action 系统的优势

*   **集中管理**: 所有的核心操作都在一个地方定义。
*   **解耦**: 开发者可以先写功能，最后再决定通过什么按键触发它。
*   **跨组件触发**: 只要在同一个窗口上下文中，任何组件都可以分发 Action，而不必显式地传递回调函数。

掌握了交互系统，你的应用就从“能看”变成了“好用”。接下来，让我们攻克最困难的部分：网络请求与异步任务。
