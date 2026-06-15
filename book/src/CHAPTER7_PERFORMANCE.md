# 第七章：性能之巅：脏标记模式与渲染优化

高性能是 GPUI 的金字招牌。但如果不注意代码编写习惯，依然会导致掉帧。在 `HiPoster` 的开发中，我们通过一个真实的案例展示了如何进行渲染优化。

## 7.1 问题：Render 函数中的“计算陷阱”

GPUI 的 `render` 函数在视图需要重绘时会被高频调用（有时每秒 60 次）。

**错误示范（HiPoster 重构前）**:
```rust
fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    // ❌ 错误：每帧都在进行复杂的 JSON 解析和字符串美化
    let display_content = format_json(&serde_json::from_str(&self.response.body).unwrap());
    
    div().child(Label::new(display_content))
}
```

这种写法在遇到几兆大小的 JSON 响应时，会导致 UI 彻底卡死。

## 7.2 解决方案：脏标记 (Dirty Flag) 模式

我们将“繁重的计算”与“界面的绘制”解耦。

1.  **定义标记**: 在结构体中增加 `dirty_response: bool`。
2.  **设置时机**: 当数据发生变化（收到 Response）或显示模式切换（Pretty/Raw）时，将标记设为 `true`。
3.  **延迟计算**: 在 `render` 中检查标记。

```rust
// src/api_tab.rs 中的实现
impl Render for ApiTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // ✅ 只有脏了才重新计算，计算完后清空标记
        if self.dirty_response {
            self.update_response_display(window, cx); 
        }

        v_flex().child(...) // 纯布局逻辑，极快
    }
}
```

## 7.3 优化技巧：减少内存分配

在 Rust 应用中，字符串克隆（`.clone()`）是有代价的。特别是在循环渲染列表时：

*   **优化前**: `.child(Label::new(url.clone()))`
*   **优化后**: `.child(Label::new(url.as_str()))`

由于 GPUI 的组件大量支持引用，尽量利用这一特性来减少堆内存的频繁分配和释放。

## 7.4 善用 Context 的通知机制

`cx.notify()` 是刷新 UI 的唯一手动手段。
*   不要在不必要的时候调用它。
*   如果只是子组件内部状态变了，尽量只 notify 子组件，而不是顶层视图。

掌握了这些，你的 GPUI 应用将如丝般顺滑。下一章，我们将聊聊如何让这些优化后的数据安全地“落盘”。
