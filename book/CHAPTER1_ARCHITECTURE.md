# 第一章：项目架构与模块规划

在开发任何复杂的 GPUI 应用之前，合理的模块规划是成功的基石。GPUI 并不是一个简单的库，它是一套关于如何组织代码、如何管理状态的“哲学”。

## 1.1 Entity-View-Model (EVM) 模式

`HiPoster` 严格遵循了 GPUI 倡导的 Entity-View-Model 模式：

*   **Model (数据模型)**: 存放在 `src/model.rs`。它是纯粹的 Rust 结构体，不依赖任何 UI 库。
    *   *示例*: `HttpRequest`, `HttpResponse`。
    *   *心法*: 保持模型的可序列化（Serializable），这是实现历史记录和持久化的前提。
*   **View (视图)**: 存放在 `src/api_tab.rs` 和 `src/app.rs`。视图是状态的容器，负责将 Model 转换为 UI 元素。
    *   *关键*: 实现 `Render` trait。
*   **Entity (实体)**: 在 GPUI 中，任何被 `cx.new()` 创建的对象都是 Entity。它是 GPUI 管理内存和通知重绘的最小单元。

## 1.2 HiPoster 的模块划分

我们的目录结构清晰地展示了功能边界：

```text
src/
├── model.rs      # 数据定义（地基）
├── http.rs       # 异步 IO 层（隔离网络复杂度）
├── theme.rs      # 样式配置（视觉中枢）
├── api_tab.rs    # 单个请求页面的业务视图（局部状态）
└── app.rs        # 根视图：管理 Tabs、侧边栏、全局动作（全局状态）
```

## 1.3 规划建议：如何开始你的应用？

1.  **逻辑先行**: 先写 `model.rs` 和 `logic.rs`（如有）。在不运行 UI 的情况下，通过单元测试跑通核心业务流。
2.  **由内而外**: 先开发最细粒度的 UI 组件（如一个输入框、一个标签），再组装成业务视图（如 `ApiTab`），最后放入全局布局（`Hiposter`）。
3.  **状态隔离**: 尽量让子视图（如 `ApiTab`）管理自己的内部状态，父视图只负责路由和跨组件通信。

下一章我们将深入 `lib.rs`，看看这个“机器”是如何被启动的。
