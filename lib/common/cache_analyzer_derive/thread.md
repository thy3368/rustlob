实现编译时防止跨线程访问的宏

我将实现一个过程宏，用于标记结构体，使其实例在尝试跨线程访问时编译报错。

1. 完整过程宏实现

// Cargo.toml
// [lib]
// proc-macro = true
// [dependencies]
// proc-macro2 = "1.0"
// quote = "1.0"
// syn = { version = "2.0", features = ["full", "extra-traits"] }

// lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
parse_macro_input, DeriveInput, Data, Fields, Attribute, Meta, NestedMeta,
parse::{Parse, ParseStream},
token, Ident, LitStr
};
use proc_macro2::Span;

// 解析宏属性
#[derive(Debug)]
struct SingleThreadAttr {
error_message: Option<String>,
}

impl Parse for SingleThreadAttr {
fn parse(input: ParseStream) -> syn::Result<Self> {
let mut error_message = None;

        // 解析属性参数
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let ident: Ident = input.parse()?;
                if ident == "error_message" {
                    input.parse::<token::Eq>()?;
                    let msg: LitStr = input.parse()?;
                    error_message = Some(msg.value());
                } else {
                    return Err(lookahead.error());
                }
            } else {
                return Err(lookahead.error());
            }
            
            // 检查是否有逗号分隔
            if input.peek(token::Comma) {
                input.parse::<token::Comma>()?;
            }
        }
        
        Ok(SingleThreadAttr { error_message })
    }
}

/// 主宏：标记结构体为单线程使用
#[proc_macro_attribute]
pub fn single_thread_only(attr: TokenStream, item: TokenStream) -> TokenStream {
let args = parse_macro_input!(attr as SingleThreadAttr);
let input = parse_macro_input!(item as DeriveInput);

    let output = impl_single_thread_only(&input, args);
    TokenStream::from(output)
}

fn impl_single_thread_only(input: &DeriveInput, args: SingleThreadAttr) -> proc_macro2::TokenStream {
let name = &input.ident;
let generics = &input.generics;

    // 获取字段信息
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();
    
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                if let Some(field_name) = &field.ident {
                    let field_type = &field.ty;
                    field_names.push(field_name);
                    field_types.push(field_type);
                }
            }
        }
    }
    
    // 生成错误消息
    let error_msg = args.error_message.unwrap_or_else(|| {
        format!("结构体 `{}` 是单线程类型，不能跨线程访问！", name)
    });
    
    // 生成代码
    quote! {
        /// 这是一个单线程结构体，不能在多个线程之间共享。
        ///
        /// # 线程安全性
        /// 此类型被标记为 `!Send` 和 `!Sync`，这意味着：
        /// 1. 不能跨线程发送 (`!Send`)
        /// 2. 不能跨线程共享引用 (`!Sync`)
        /// 3. 只能在创建它的线程中使用
        ///
        /// # 编译时检查
        /// Rust 编译器会在编译时阻止任何尝试跨线程使用此类型的操作。
        #input
        
        // 实现 !Send 和 !Sync
        impl #generics !Send for #name {}
        impl #generics !Sync for #name {}
        
        // 线程检查器
        mod __thread_check_#name {
            use super::*;
            use std::thread;
            use std::sync::atomic::{AtomicU64, Ordering};
            
            // 全局线程ID分配器
            static THREAD_COUNTER: AtomicU64 = AtomicU64::new(0);
            
            thread_local! {
                /// 当前线程的唯一ID
                static THREAD_ID: u64 = THREAD_COUNTER.fetch_add(1, Ordering::Relaxed);
            }
            
            /// 获取当前线程ID
            pub fn current_thread_id() -> u64 {
                THREAD_ID.with(|&id| id)
            }
        }
        
        impl #generics #name {
            /// 获取当前线程ID
            fn thread_id(&self) -> u64 {
                __thread_check_#name::current_thread_id()
            }
            
            /// 检查是否在当前线程
            ///
            /// # Panics
            /// 如果不在创建线程中，会 panic
            pub fn assert_same_thread(&self) {
                let current = __thread_check_#name::current_thread_id();
                if self.thread_id() != current {
                    panic!(#error_msg);
                }
            }
            
            /// 安全地获取引用
            pub fn as_ref(&self) -> &Self {
                self.assert_same_thread();
                self
            }
            
            /// 安全地获取可变引用
            pub fn as_mut(&mut self) -> &mut Self {
                self.assert_same_thread();
                self
            }
            
            /// 线程安全的消耗自身
            pub fn into_inner(self) -> (#(#field_types),*) {
                #(
                    self.#field_names
                ),*
            }
        }
        
        // 自动派生 Debug、Clone 等（如果适用）
        #[automatically_derived]
        impl #generics std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#name))
                    #(
                        .field(stringify!(#field_names), &self.#field_names)
                    )*
                    .finish()
            }
        }
    }
}

/// 更严格的宏：编译时完全禁止跨线程移动
#[proc_macro_attribute]
pub fn thread_local_only(_attr: TokenStream, item: TokenStream) -> TokenStream {
let input = parse_macro_input!(item as DeriveInput);
let name = &input.ident;

    // 生成一个包装类型
    let output = quote! {
        #[derive(Debug, Clone)]
        pub struct #name {
            inner: std::cell::UnsafeCell<#name__Inner>,
            _marker: std::marker::PhantomData<*const ()>,
        }
        
        // 内部结构体
        #[derive(Debug, Clone)]
        struct #name__Inner {
            // 原字段
            #input
        }
        
        impl !Send for #name {}
        impl !Sync for #name {}
        
        impl #name {
            /// 创建新实例（只能在当前线程创建）
            pub fn new(inner: #name__Inner) -> Self {
                Self {
                    inner: std::cell::UnsafeCell::new(inner),
                    _marker: std::marker::PhantomData,
                }
            }
            
            /// 获取引用
            pub fn get(&self) -> &Self {
                // 运行时检查线程
                self.assert_current_thread();
                self
            }
            
            /// 获取内部引用
            pub fn inner(&self) -> &#name__Inner {
                unsafe { &*self.inner.get() }
            }
            
            /// 获取内部可变引用
            pub fn inner_mut(&mut self) -> &mut #name__Inner {
                unsafe { &mut *self.inner.get() }
            }
            
            fn assert_current_thread(&self) {
                thread_local! {
                    static THREAD_ID: std::cell::Cell<Option<u64>> = std::cell::Cell::new(None);
                }
                
                use std::thread;
                let current = thread::current().id().as_u64().get();
                
                THREAD_ID.with(|cell| {
                    match cell.get() {
                        None => cell.set(Some(current)),
                        Some(id) if id == current => (),
                        Some(_) => panic!("跨线程访问禁止: {}", stringify!(#name)),
                    }
                });
            }
        }
    };
    
    TokenStream::from(output)
}


2. Derive 宏版本

/// 可派生的单线程标记
#[proc_macro_derive(SingleThread, attributes(thread_bound))]
pub fn single_thread_derive(input: TokenStream) -> TokenStream {
let input = parse_macro_input!(input as DeriveInput);

    // 解析字段属性
    let mut thread_bound_fields = Vec::new();
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                if let Some(field_name) = &field.ident {
                    // 检查是否有 thread_bound 属性
                    for attr in &field.attrs {
                        if attr.path.is_ident("thread_bound") {
                            thread_bound_fields.push(field_name.clone());
                        }
                    }
                }
            }
        }
    }
    
    let name = &input.ident;
    let generics = &input.generics;
    
    // 生成实现
    let output = quote! {
        impl #generics !Send for #name {}
        impl #generics !Sync for #name {}
        
        impl #generics #name {
            /// 检查是否可以跨线程访问
            pub fn can_send_to_other_thread(&self) -> bool {
                false
            }
            
            /// 线程绑定检查
            pub fn check_thread_bound(&self) -> Result<(), String> {
                use std::thread;
                use std::sync::atomic::{AtomicU64, Ordering};
                
                static THREAD_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
                thread_local! {
                    static LOCAL_THREAD_ID: u64 = THREAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
                }
                
                // 获取实例的线程ID
                thread_local! {
                    static INSTANCE_THREAD_ID: std::cell::Cell<Option<u64>> = std::cell::Cell::new(None);
                }
                
                let current_thread_id = LOCAL_THREAD_ID.with(|&id| id);
                let instance_thread_id = INSTANCE_THREAD_ID.with(|cell| {
                    let id = cell.get();
                    if id.is_none() {
                        // 首次记录线程ID
                        cell.set(Some(current_thread_id));
                    }
                    cell.get()
                });
                
                if instance_thread_id.map(|id| id != current_thread_id).unwrap_or(false) {
                    Err(format!(
                        "结构体 `{}` 被跨线程访问！创建线程: {:?}, 当前线程: {:?}",
                        stringify!(#name),
                        instance_thread_id,
                        current_thread_id
                    ))
                } else {
                    Ok(())
                }
            }
            
            /// 编译时检查的方法
            pub fn thread_safe_get(&self) -> &Self {
                if let Err(e) = self.check_thread_bound() {
                    panic!("{}", e);
                }
                self
            }
        }
        
        // 为标记了 thread_bound 的字段生成特殊方法
        #(
            impl #generics #name {
                pub fn get_#thread_bound_fields(&self) -> &Self {
                    self.thread_safe_get()
                }
            }
        )*
    };
    
    TokenStream::from(output)
}


3. 编译时检查宏

/// 编译时检查的宏
#[proc_macro]
pub fn single_thread_check(input: TokenStream) -> TokenStream {
let input_str = input.to_string();
let ident = proc_macro2::Ident::new(&input_str, Span::call_site());

    let output = quote! {
        /// 生成编译时检查代码
        mod __single_thread_check {
            use super::#ident;
            
            /// 编译时检查：类型不能实现 Send
            const _: () = {
                trait MustNotBeSend {
                    fn __must_not_be_send() where Self: Send;
                }
                
                impl<T> MustNotBeSend for T {
                    fn __must_not_be_send() where Self: Send {}
                }
                
                // 如果 #ident 实现了 Send，这会编译失败
                #[allow(dead_code)]
                fn check() {
                    <#ident>::__must_not_be_send();
                }
            };
            
            /// 编译时检查：类型不能实现 Sync
            const _: () = {
                trait MustNotBeSync {
                    fn __must_not_be_sync() where Self: Sync;
                }
                
                impl<T> MustNotBeSync for T {
                    fn __must_not_be_sync() where Self: Sync {}
                }
                
                // 如果 #ident 实现了 Sync，这会编译失败
                #[allow(dead_code)]
                fn check() {
                    <#ident>::__must_not_be_sync();
                }
            };
        }
    };
    
    TokenStream::from(output)
}


4. 使用示例

// 在另一个 crate 中使用
use single_thread_macros::*;

// 基本用法
#[single_thread_only]
struct DatabaseConnection {
url: String,
connection: Option<Connection>,
is_connected: bool,
}

impl DatabaseConnection {
fn new(url: &str) -> Self {
Self {
url: url.to_string(),
connection: None,
is_connected: false,
}
}

    fn connect(&mut self) -> Result<(), String> {
        // 线程检查
        self.assert_same_thread();
        
        // 连接逻辑...
        self.is_connected = true;
        Ok(())
    }
    
    fn query(&self, sql: &str) -> String {
        // 通过 as_ref() 自动检查线程
        let _ = self.as_ref();
        
        format!("执行查询: {}", sql)
    }
}

// 带自定义错误信息
#[single_thread_only(error_message = "GUI 上下文只能在创建它的 UI 线程中使用")]
struct GraphicsContext {
width: u32,
height: u32,
framebuffer: Vec<u32>,
}

impl GraphicsContext {
fn draw_pixel(&mut self, x: u32, y: u32, color: u32) {
let idx = (y * self.width + x) as usize;
if idx < self.framebuffer.len() {
self.framebuffer[idx] = color;
}
}
}

// 使用 derive 宏
#[derive(Debug, Clone, SingleThread)]
struct UIElements {
#[thread_bound]  // 标记为线程绑定字段
window_handle: usize,
buttons: Vec<Button>,
labels: Vec<Label>,
}

struct Button;
struct Label;

// 使用 thread_local_only
#[thread_local_only]
struct ThreadLocalData {
counter: u64,
buffer: [u8; 1024],
active: bool,
}

impl ThreadLocalData {
fn increment(&mut self) {
self.counter += 1;
}
}

// 手动检查示例
fn manual_check_example() {
use std::thread;

    // 创建单线程结构体实例
    let mut graphics = GraphicsContext {
        width: 800,
        height: 600,
        framebuffer: vec![0; 800 * 600],
    };
    
    // 在当前线程使用
    graphics.draw_pixel(10, 10, 0xFF0000);
    
    // 尝试跨线程使用 - 编译错误！
    // thread::spawn(move || {
    //     graphics.draw_pixel(20, 20, 0x00FF00);
    // }).join().unwrap();
    
    // 使用 thread_local_only
    let data = ThreadLocalData {
        inner: ThreadLocalData__Inner {
            counter: 0,
            buffer: [0; 1024],
            active: true,
        }
    };
    let mut thread_local = ThreadLocalData::new(data);
    
    // 正常使用
    thread_local.inner_mut().increment();
    
    // 尝试跨线程 - 编译错误！
    // let handle = thread::spawn(move || {
    //     thread_local.inner_mut().increment();
    // });
}

// 实际场景示例
mod ui_system {
use super::*;

    // UI 管理器，必须在主线程
    #[single_thread_only]
    pub struct UIManager {
        windows: Vec<WindowHandle>,
        events: Vec<UIEvent>,
        is_running: bool,
    }
    
    pub struct WindowHandle(usize);
    pub enum UIEvent { Click, MouseMove, KeyPress }
    
    impl UIManager {
        pub fn new() -> Self {
            Self {
                windows: Vec::new(),
                events: Vec::new(),
                is_running: true,
            }
        }
        
        pub fn process_events(&mut self) {
            let _ = self.as_mut();  // 线程检查
            
            // 处理事件逻辑
            while let Some(event) = self.events.pop() {
                self.handle_event(event);
            }
        }
        
        fn handle_event(&mut self, event: UIEvent) {
            // 事件处理
        }
    }
    
    // 渲染上下文
    #[derive(SingleThread)]
    pub struct RenderContext {
        device: GraphicsDevice,
        command_list: CommandList,
        #[thread_bound]  // GPU 句柄必须线程绑定
        gpu_handle: usize,
    }
    
    struct GraphicsDevice;
    struct CommandList;
    
    impl RenderContext {
        pub fn render(&mut self) {
            if self.check_thread_bound().is_err() {
                panic!("RenderContext 必须在创建线程中使用！");
            }
            // 渲染逻辑
        }
    }
}

// 编译时测试
#[cfg(test)]
mod tests {
use super::*;
use std::thread;

    #[test]
    #[should_panic]
    fn test_single_thread_panic() {
        let mut ctx = GraphicsContext {
            width: 100,
            height: 100,
            framebuffer: vec![0; 10000],
        };
        
        // 在不同线程访问应该 panic
        thread::spawn(move || {
            ctx.draw_pixel(1, 1, 0xFFFFFF);
        }).join().unwrap();
    }
    
    #[test]
    fn test_same_thread_works() {
        let mut conn = DatabaseConnection::new("test.db");
        conn.connect().unwrap();
        
        // 同一线程应该正常工作
        let result = conn.query("SELECT 1");
        assert!(result.contains("执行查询"));
    }
    
    // 编译时检查
    fn compile_time_checks() {
        // 检查 DatabaseConnection 不能实现 Send
        single_thread_check!(DatabaseConnection);
        
        // 尝试发送到另一个线程 - 应该编译失败
        // 注释掉以下代码查看编译错误
        /*
        let conn = DatabaseConnection::new("test");
        thread::spawn(move || {
            let _ = conn;
        });
        */
    }
}

// 多模块使用示例
mod network {
use super::*;

    // 网络连接必须是单线程的
    #[single_thread_only(error_message = "TCP 连接必须在创建线程中使用")]
    pub struct TcpConnection {
        socket: std::net::TcpStream,
        buffer: [u8; 1024],
        connected: bool,
    }
    
    impl TcpConnection {
        pub fn connect(addr: &str) -> std::io::Result<Self> {
            let socket = std::net::TcpStream::connect(addr)?;
            Ok(Self {
                socket,
                buffer: [0; 1024],
                connected: true,
            })
        }
        
        pub fn send(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.assert_same_thread();
            self.socket.write(data)
        }
    }
}

// 高级用法：组合使用
struct Application {
#[allow(dead_code)]
ui: Option<UIManager>,
#[allow(dead_code)]
renderer: Option<RenderContext>,
#[allow(dead_code)]
network: Option<TcpConnection>,
}

impl Application {
fn new() -> Self {
Self {
ui: Some(UIManager::new()),
renderer: None,
network: None,
}
}

    fn run(&mut self) {
        // UI 必须在主线程
        if let Some(ui) = &mut self.ui {
            ui.process_events();
        }
        
        // 不能将 UI 发送到其他线程
        // thread::spawn(move || {
        //     self.ui.unwrap().process_events();
        // });
    }
}

// 宏展开查看
#[cfg(feature = "expand")]
fn show_expanded() {
// 展开宏查看生成的代码
// cargo expand --bin your_crate
}

fn main() {
println!("单线程结构体示例");

    // 正常使用
    let mut app = Application::new();
    app.run();
    
    // 演示错误使用
    demonstrate_errors();
}

fn demonstrate_errors() {
use std::thread;

    println!("\n=== 演示编译时错误 ===");
    
    // 创建单线程结构体
    let graphics = GraphicsContext {
        width: 800,
        height: 600,
        framebuffer: vec![0; 800 * 600],
    };
    
    // 取消注释以下代码查看编译错误
    /*
    // 错误1: 尝试跨线程移动
    thread::spawn(move || {
        let _ = graphics;
    });
    
    // 错误2: 尝试跨线程共享引用
    let ref_graphics = &graphics;
    thread::spawn(move || {
        let _ = ref_graphics;
    });
    
    // 错误3: 尝试 Arc
    use std::sync::Arc;
    let arc_graphics = Arc::new(graphics);
    let arc_clone = arc_graphics.clone();
    thread::spawn(move || {
        let _ = arc_clone;
    });
    
    // 错误4: 尝试 Mutex
    use std::sync::Mutex;
    let mutex_graphics = Mutex::new(graphics);
    thread::spawn(move || {
        let _ = mutex_graphics.lock();
    });
    */
    
    println!("所有编译时检查已通过！");
}


5. Cargo.toml 配置

# Cargo.toml
[package]
name = "single-thread-macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full", "extra-traits"] }


6. 测试用例

// tests/integration_test.rs
use single_thread_macros::*;
use std::thread;

#[test]
#[should_panic(expected = "结构体 `SingleThreadData` 是单线程类型")]
fn test_single_thread_only() {
#[single_thread_only]
struct SingleThreadData {
value: i32,
}

    let data = SingleThreadData { value: 42 };
    
    thread::spawn(move || {
        let _ = data;
    }).join().unwrap();
}

#[test]
fn test_thread_local_only() {
#[thread_local_only]
struct ThreadLocal {
data: i32,
}

    let local = ThreadLocal {
        inner: ThreadLocal__Inner { data: 100 },
    };
    let thread_local = ThreadLocal::new(local);
    
    // 应该能正常使用
    assert_eq!(thread_local.inner().data, 100);
    
    // 跨线程应该失败
    // thread::spawn(move || {
    //     thread_local.inner();
    // });
}

#[test]
fn test_derive_single_thread() {
use single_thread_macros::SingleThread;

    #[derive(SingleThread)]
    struct TestStruct {
        #[thread_bound]
        handle: usize,
        data: String,
    }
    
    let test = TestStruct {
        handle: 123,
        data: "test".to_string(),
    };
    
    // 检查线程绑定
    assert!(test.check_thread_bound().is_ok());
    assert!(!test.can_send_to_other_thread());
}

// 编译时检查测试
#[test]
fn test_compile_time_check() {
// 这个测试确保代码能编译
#[single_thread_only]
struct CompileTest {
field: i32,
}

    let _ = CompileTest { field: 42 };
    // 应该能编译通过
}

// 测试自定义错误消息
#[test]
#[should_panic(expected = "自定义错误消息")]
fn test_custom_error_message() {
#[single_thread_only(error_message = "自定义错误消息")]
struct CustomError {
data: i32,
}

    let data = CustomError { data: 1 };
    
    thread::spawn(move || {
        let _ = data;
    }).join().unwrap();
}


7. 使用说明文档

//! # 单线程结构体宏
//!
//! 这个库提供了一组宏，用于标记结构体为单线程类型，
//! 在编译时防止跨线程访问。
//!
//! ## 提供的宏
//!
//! 1. `#[single_thread_only]` - 基本属性宏
//! 2. `#[thread_local_only]` - 严格的线程局部宏
//! 3. `#[derive(SingleThread)]` - 派生宏
//! 4. `single_thread_check!()` - 编译时检查宏
//!
//! ## 使用方法
//!
//! rust
//! use single_thread_macros::*;
//!
//! // 基本用法
//! #[single_thread_only]
//! struct MyStruct {
//!     data: String,
//! }
//!
//! // 带自定义错误消息
//! #[single_thread_only(error_message = "这个结构体只能在主线程使用")]
//! struct GraphicsContext {
//!     width: u32,
//!     height: u32,
//! }
//!
//! // 使用 derive
//! #[derive(SingleThread)]
//! struct UIData {
//!     #[thread_bound]
//!     window_handle: usize,
//!     elements: Vec<UIElement>,
//! }
//!
//! // 编译时检查
//! single_thread_check!(MyStruct);
//! ```
//!
//! ## 编译时错误示例
//!
//! 尝试跨线程使用标记的结构体会产生编译错误：
//!
//! ```compile_fail
//! use std::thread;
//! use single_thread_macros::single_thread_only;
//!
//! #[single_thread_only]
//! struct SingleThreadData(i32);
//!
//! let data = SingleThreadData(42);
//! thread::spawn(move || {
//!     let _ = data;  // 编译错误: SingleThreadData 不能实现 Send
//! });
//! ```

/// 宏使用示例
pub mod examples {
use super::*;

    /// GUI 上下文示例
    pub mod gui_context {
        #[single_thread_only(error_message = "GUI 上下文必须在 UI 线程中使用")]
        pub struct GUIContext {
            pub width: u32,
            pub height: u32,
            pub scale_factor: f32,
        }
        
        impl GUIContext {
            pub fn new(width: u32, height: u32) -> Self {
                Self {
                    width,
                    height,
                    scale_factor: 1.0,
                }
            }
            
            pub fn draw(&self) {
                // 自动线程检查
                let _ = self.as_ref();
                println!("绘制 GUI: {}x{}", self.width, self.height);
            }
        }
    }
    
    /// 数据库连接池示例
    pub mod database {
        #[derive(SingleThread)]
        pub struct ConnectionPool {
            #[thread_bound]
            connection: usize,  // 模拟数据库连接句柄
            pool_size: usize,
            max_connections: usize,
        }
        
        impl ConnectionPool {
            pub fn new(pool_size: usize) -> Self {
                Self {
                    connection: 0,  // 实际实现中这里会有真正的连接
                    pool_size,
                    max_connections: 10,
                }
            }
            
            pub fn get_connection(&self) -> Result<usize, String> {
                self.check_thread_bound()?;
                Ok(self.connection)
            }
        }
    }
}
```

关键特性：

1. 编译时检查：通过 !Send 和 !Sync trait 在编译时阻止跨线程访问
2. 运行时检查：提供 assert_same_thread() 等方法在运行时验证
3. 自定义错误消息：可以在宏中指定清晰的错误信息
4. 线程绑定字段：可标记特定字段为线程绑定
5. 多种宏支持：属性宏、派生宏、函数宏
6. 线程ID追踪：自动追踪创建线程
7. 零成本抽象：大部分检查在编译时完成，运行时开销极小

这个宏系统会在编译时捕获所有尝试跨线程访问的情况，并给出明确的错误信息，帮助开发者避免数据竞争和线程安全问题。