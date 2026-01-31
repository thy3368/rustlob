# SingleThread Derive Macro

`single_thread_derive` 是一个 Rust 过程宏 crate，提供 `#[derive(SingleThread)]` 用于标记结构体为单线程类型。

## 功能

- 运行时线程检查
- 线程绑定字段支持
- 自动生成线程安全访问方法

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
single_thread_derive = { path = "path/to/single_thread_derive" }
```

## 使用

### 基本使用

```rust
use single_thread_derive::SingleThread;

#[derive(SingleThread)]
struct DatabaseConnection {
    url: String,
}

fn main() {
    let conn = DatabaseConnection {
        url: "localhost".to_string(),
    };

    // 运行时线程检查
    assert!(conn.check_thread_bound().is_ok());
    assert!(!conn.can_send_to_other_thread());
}
```

### 线程绑定字段

使用 `#[thread_bound]` 属性标记必须在特定线程中访问的字段：

```rust
#[derive(SingleThread)]
struct GraphicsContext {
    #[thread_bound]
    gpu_handle: usize,
    width: u32,
    height: u32,
}

fn main() {
    let ctx = GraphicsContext {
        gpu_handle: 123,
        width: 800,
        height: 600,
    };

    // 自动生成的访问器方法（带线程检查）
    let _ref = ctx.get_gpu_handle();
}
```

## 生成的方法

### `can_send_to_other_thread() -> bool`

检查类型是否可以跨线程发送。对于 `SingleThread` 类型，总是返回 `false`。

### `check_thread_bound() -> Result<(), String>`

运行时线程检查。验证当前访问是否在创建线程中。

### `thread_safe_get(&self) -> &Self`

带线程检查的引用获取。如果在不同线程中调用，会 panic。

### `thread_safe_get_mut(&mut self) -> &mut Self`

带线程检查的可变引用获取。

### `get_{field_name}(&self) -> &Self`

对于每个标记为 `#[thread_bound]` 的字段，自动生成访问器方法。

## 线程追踪机制

- 使用全局原子计数器分配唯一线程ID
- 使用 `thread_local!` 存储每个线程的ID
- 首次访问时记录实例的创建线程ID
- 后续访问时验证是否在同一线程

## 注意事项

由于稳定版 Rust 不支持直接的 `!Send` 和 `!Sync`，此宏只提供**运行时检查**。

如果需要编译时保证，应该在结构体中添加 `PhantomData<*const ()>` 字段：

```rust
use std::marker::PhantomData;

#[derive(SingleThread)]
struct StrictSingleThread {
    data: u32,
    _not_send: PhantomData<*const ()>,
}
```

## 许可证

MIT License

## 参考

- [SINGLE_THREAD.md](../cache_analyzer_derive/SINGLE_THREAD.md) - 详细使用文档
- [thread.md](../cache_analyzer_derive/thread.md) - 实现参考
