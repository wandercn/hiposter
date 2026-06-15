# 第八章：数据持久化：标准化存储与迁移

一个合格的桌面应用，不应在关闭后“失忆”。`HiPoster` 通过标准化的持久化方案，确保了用户的历史请求和偏好设置能够跨设备、跨版本安全保存。

## 8.1 存储位置的“三板斧”

不要在项目源码目录下写文件！不同系统有其专属的应用数据目录：

*   **macOS**: `~/Library/Application Support/`
*   **Windows**: `%AppData%\Roaming\`
*   **Linux**: `~/.config/`

在 Rust 中，我们使用 `directories` crate 来自动获取这些路径，确保应用是“系统友好的”。

## 8.2 结构化序列化：Serde

Rust 的 `serde` 生态让持久化变得无比简单。
1.  **定义结构**: 为模型添加 `#[derive(Serialize, Deserialize)]`。
2.  **保存**: `serde_json::to_string()` -> `fs::write()`。
3.  **加载**: `fs::read_to_string()` -> `serde_json::from_str()`。

## 8.3 鲁棒性：自动目录创建与迁移

用户第一次打开应用时，配置目录并不存在。应用必须具备“自我初始化”的能力。

```rust
fn ensure_config_dir() -> Option<PathBuf> {
    let dir = config_dir()?;
    if !dir.exists() {
        fs::create_dir_all(&dir).ok()?;
        // 这里可以执行旧版本数据的迁移逻辑
    }
    Some(dir)
}
```

## 8.4 历史记录的容量控制

无限增长的历史记录会导致应用启动变慢。
**实践**: 在 `HiPoster` 中，我们对历史记录执行了 `truncate(50)` 策略，只保留最近的 50 条记录。

## 8.5 进阶建议

*   **二进制格式**: 如果数据量巨大，考虑使用 `bincode` 或 `MessagePack` 替代 JSON。
*   **数据库**: 对于数以万计的记录，建议引入 `SQLite`。
*   **原子写入**: 为了防止写入时断电导致文件损坏，建议采用“写临时文件 -> rename”的方案。

至此，我们的应用已经具备了完整的功能和出色的性能。最后一章，我们将讨论如何将它交付给最终用户。
