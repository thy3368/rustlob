# SingleThread 派生宏

`#[derive(SingleThread)]` 是一个用于标记结构体为单线程类型的派生宏，提供运行时线程检查功能。

## 功能特性

### 1. 运行时线程检查

宏会生成方法来检查结构体实例是否在创建它的线程中被访问：

```rust
use cache_analyzer_derive::SingleThread;

#[derive(SingleThread)]
struct DatabaseConnection {
    url: String,
    is_connected: bool,
}

fn main() {
    let conn = DatabaseConnection {
        url: "localhost".to_string(),
        is_connected: false,
    };

    // 检查是否在同一线程
    assert!(conn.check_thread_bound().is_ok());

    // 安全地获取引用（带线程检查）
    let _ref = conn.thread_safe_get();
}
```

### 2. 线程绑定字段

使用 `#[thread_bound]` 属性标记必须在特定线程中访问的字段：

```rust
#[derive(SingleThread)]
struct GraphicsContext {
    #[thread_bound]
    gpu_handle: usize,  // GPU 句柄必须线程绑定
    #[thread_bound]
    window_handle: usize,  // 窗口句柄必须线程绑定
    width: u32,
    height: u32,
}

fn main() {
    let ctx = GraphicsContext {
        gpu_handle: 123,
        window_handle: 456,
        width: 800,
        height: 600,
    };

    // 自动生成的线程绑定字段访问器
    let _ref1 = ctx.get_gpu_handle();
    let _ref2 = ctx.get_window_handle();
}
```

### 3. 与 CacheAnalyzer 组合使用

可以同时使用 `CacheAnalyzer` 和 `SingleThread`：

```rust
#[repr(C)]
#[derive(CacheAnalyzer, SingleThread)]
struct OptimizedContext {
    #[hot]
    counter: u64,  // 热点字段
    #[thread_bound]
    handle: usize,  // 线程绑定字段
    data: [u8; 32],
}

fn main() {
    let ctx = OptimizedContext {
        counter: 100,
        handle: 999,
        data: [0; 32],
    };

    // 使用 CacheAnalyzer 的功能
    let report = OptimizedContext::detailed_cache_analysis();
    println!("缓存行数: {}", report.cache_lines_needed);

    // 使用 SingleThread 的功能
    assert!(!ctx.can_send_to_other_thread());
    assert!(ctx.check_thread_bound().is_ok());
}
```

## 生成的方法

### `can_send_to_other_thread() -> bool`

检查类型是否可以跨线程发送。对于 `SingleThread` 类型，总是返回 `false`。

```rust
#[derive(SingleThread)]
struct MyStruct { data: u32 }

let obj = MyStruct { data: 42 };
assert!(!obj.can_send_to_other_thread());
```

### `check_thread_bound() -> Result<(), String>`

运行时线程检查。验证当前访问是否在创建线程中。

```rust
#[derive(SingleThread)]
struct MyStruct { data: u32 }

let obj = MyStruct { data: 42 };

// 在创建线程中：Ok(())
assert!(obj.check_thread_bound().is_ok());

// 在不同线程中：Err(...)
// 注意：由于 Rust 的所有权系统，实际上很难在不同线程中访问
```

### `thread_safe_get(&self) -> &Self`

带线程检查的引用获取。如果在不同线程中调用，会 panic。

```rust
#[derive(SingleThread)]
struct MyStruct { data: u32 }

let obj = MyStruct { data: 42 };
let _ref = obj.thread_safe_get();  // 在同一线程：正常
```

### `thread_safe_get_mut(&mut self) -> &mut Self`

带线程检查的可变引用获取。

```rust
#[derive(SingleThread)]
struct MyStruct { data: u32 }

let mut obj = MyStruct { data: 42 };
let obj_mut = obj.thread_safe_get_mut();
obj_mut.data += 1;
assert_eq!(obj.data, 43);
```

### `get_{field_name}(&self) -> &Self`

对于每个标记为 `#[thread_bound]` 的字段，自动生成访问器方法。

```rust
#[derive(SingleThread)]
struct MyStruct {
    #[thread_bound]
    handle: usize,
}

let obj = MyStruct { handle: 123 };
let _ref = obj.get_handle();  // 带线程检查的访问
```

## 使用场景

### 1. 数据库连接

```rust
#[derive(SingleThread)]
struct DatabaseConnection {
    #[thread_bound]
    connection_handle: usize,
    url: String,
    is_connected: bool,
}

impl DatabaseConnection {
    fn query(&self, sql: &str) -> Result<Vec<Row>, Error> {
        // 自动线程检查
        let _ = self.thread_safe_get();

        // 执行查询
        // ...
    }
}
```

### 2. GUI 上下文

```rust
#[derive(SingleThread)]
struct GUIContext {
    #[thread_bound]
    window_handle: usize,
    #[thread_bound]
    device_context: usize,
    width: u32,
    height: u32,
}

impl GUIContext {
    fn render(&self) {
        // 确保在 UI 线程中
        let _ = self.get_window_handle();

        // 渲染逻辑
        // ...
    }
}
```

### 3. 网络连接

```rust
#[derive(SingleThread)]
struct TcpConnection {
    #[thread_bound]
    socket_fd: i32,
    buffer: Vec<u8>,
}

impl TcpConnection {
    fn send(&mut self, data: &[u8]) -> Result<usize, Error> {
        // 线程检查
        let _ = self.thread_safe_get_mut();

        // 发送数据
        // ...
    }
}
```

## 泛型支持

支持泛型结构体：

```rust
#[derive(SingleThread)]
struct Container<T> {
    value: T,
}

let obj = Container { value: 42u32 };
assert!(!obj.can_send_to_other_thread());

let obj2 = Container { value: "hello".to_string() };
assert!(obj2.check_thread_bound().is_ok());
```

## 注意事项

### 1. 编译时 vs 运行时检查

由于稳定版 Rust 不支持直接的 `!Send` 和 `!Sync` trait，此宏只提供**运行时检查**。

如果需要编译时保证不能跨线程，应该在结构体中添加 `PhantomData<*const ()>` 字段：

```rust
use std::marker::PhantomData;

#[derive(SingleThread)]
struct StrictSingleThread {
    data: u32,
    _not_send: PhantomData<*const ()>,  // 移除 Send 和 Sync
}
```

### 2. 线程ID追踪

宏使用全局原子计数器和 `thread_local!` 存储来追踪线程ID。这确保了每个线程都有唯一的ID。

### 3. 性能开销

- 线程ID检查是 O(1) 操作
- 使用 `thread_local!` 宏，访问开销极小
- 首次访问时会记录线程ID

### 4. 与其他派生宏兼容

`SingleThread` 可以与标准派生宏和其他自定义派生宏一起使用：

```rust
#[derive(Debug, Clone, SingleThread, CacheAnalyzer)]
struct MyStruct {
    data: u32,
}
```

## 实际示例

### 完整的数据库连接示例

```rust
use cache_analyzer_derive::SingleThread;

#[derive(SingleThread)]
struct Connection {
    #[thread_bound]
    handle: usize,
    database: String,
    is_connected: bool,
}

impl Connection {
    fn new(database: &str) -> Self {
        Self {
            handle: 12345,  // 模拟句柄
            database: database.to_string(),
            is_connected: false,
        }
    }

    fn connect(&mut self) -> Result<(), String> {
        // 线程检查
        let _ = self.thread_safe_get_mut();

        self.is_connected = true;
        Ok(())
    }

    fn query(&self, sql: &str) -> Result<Vec<String>, String> {
        // 自动线程检查
        let _ = self.get_handle();

        if !self.is_connected {
            return Err("未连接".to_string());
        }

        Ok(vec![format!("结果: {}", sql)])
    }
}

fn main() {
    let mut conn = Connection::new("mydb");

    // 连接数据库
    conn.connect().unwrap();

    // 执行查询
    let results = conn.query("SELECT * FROM users").unwrap();
    println!("{:?}", results);

    // 验证线程检查
    assert!(conn.check_thread_bound().is_ok());
}
```

## 错误处理

当在不同线程中访问时，错误消息包含详细信息：

```
结构体 `MyStruct` 被跨线程访问！
创建线程ID: 0, 当前线程ID: 1
此类型被标记为 SingleThread，只能在创建线程中使用。
```

## 最佳实践

1. **明确标记线程绑定字段**：使用 `#[thread_bound]` 清晰地表明哪些字段必须线程绑定

2. **在关键方法中检查**：在可能跨线程调用的方法开头使用 `thread_safe_get()`

3. **结合文档**：为 SingleThread 类型添加文档说明其线程要求

4. **考虑编译时保证**：对于严格的单线程要求，使用 `PhantomData<*const ()>`

5. **与 CacheAnalyzer 组合**：同时优化缓存和线程安全

## 参考

- [thread.md - 单线程结构体宏完整实现](./thread.md)
- [Rust 线程安全](https://doc.rust-lang.org/nomicon/send-and-sync.html)
- [PhantomData 文档](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
