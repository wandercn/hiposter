# 第三章：声明式 UI 与 Flexbox 布局

GPUI 的布局系统是它最受开发者欢迎的部分。如果你有 Web 开发经验，你会感到宾至如归；如果你没有，你会发现它比传统的 Win32 或 Qt 布局要直观得多。

## 3.1 Flexbox：布局的乐高积木

在 GPUI 中，几乎所有的容器都是 Flex 容器。

*   **`v_flex()`**: 垂直排列子元素（相当于 `flex-direction: column`）。
*   **`h_flex()`**: 水平排列子元素（相当于 `flex-direction: row`）。

**实战案例：HiPoster 的 URL 栏**
```rust
h_flex() // 水平排列
    .px_4().py_2() // padding
    .gap_3() // 子元素间距
    .child(MethodButton::new()) // 请求方法按钮
    .child(UrlInput::new())     // URL 输入框 (flex-1)
    .child(SendButton::new())   // 发送按钮
```

## 3.2 链式样式定义 (Tailwind 风格)

GPUI 使用了高度一致的链式语法来定义样式，这不仅减少了代码量，还提供了编译时的类型检查。

*   **尺寸**: `.w_full()`, `.h_10()`, `.size(px(100.))`
*   **间距**: `.p_4()`, `.mt_2()`, `.gap_x_2()`
*   **视觉**: `.bg(color)`, `.border_1()`, `.rounded_md()`, `.shadow_lg()`
*   **对齐**: `.items_center()`, `.justify_between()`

## 3.3 IntoElement Trait

任何实现了 `IntoElement` 的对象都可以作为 `.child()` 传入。这意味着你可以轻松地拆分 UI：

```rust
fn render_sidebar(&self) -> impl IntoElement {
    v_flex()
        .w_64()
        .bg(colors.sidebar)
        .children(self.history.iter().map(|item| self.render_history_item(item)))
}
```

## 3.4 交互反馈：Hover 与 Active

GPUI 允许你直接在样式链中定义交互状态：

```rust
div()
    .bg(colors.base)
    .hover(|s| s.bg(colors.hover)) // 悬停变色
    .active(|s| s.bg(colors.active)) // 点击变色
    .cursor_pointer()
```

## 3.5 布局心法

在规划你的应用界面时，记住：**嵌套是常态**。
一个复杂的界面无非是多个 `v_flex` 嵌套 `h_flex`，再通过 `flex_1` 分配剩余空间的过程。在下一章，我们将看看如何给这些静态的“积木”注入灵魂——状态管理。
