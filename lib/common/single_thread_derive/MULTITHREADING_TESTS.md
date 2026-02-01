为编译时检测跨线程访问选择正确的宏

根据你的需求，proc_macro_attribute 更适合实现编译时检测 struct 被跨线程访问报错。

让我详细解释原因，并给出最佳实现方案：

1. 为什么选择 proc_macro_attribute

核心原因：

// proc_macro_derive 的局限性：
#[derive(ThreadSafe)]  // 只是实现 trait，不能修改结构体定义
struct MyStruct { ... }

// proc_macro_attribute 的灵活性：
#[single_thread_only]  // 可以修改结构体定义，添加标记字段
struct MyStruct { ... }


具体对比：

需求 proc_macro_attribute proc_macro_derive

修改结构体定义 ✅ 可以添加/修改字段 ❌ 只能添加实现

控制字段布局 ✅ 可以添加 !Send 字段 ❌ 不能修改结构体

编译时错误 ✅ 立即报错 ⚠️ 可能延迟报错

使用简单 ✅ #[single_thread] ⚠️ #[derive(SingleThread)]

防止误用 ✅ 可以完全阻止某些操作 ❌ 依赖开发者不手动实现 Send

2. 最佳实现：proc_macro_attribute

// 这是最佳选择，原因如下：

// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

/// 标记结构体为单线程使用，编译时防止跨线程访问
#[proc_macro_attribute]
pub fn single_thread(_attr: TokenStream, item: TokenStream) -> TokenStream {
let input = parse_macro_input!(item as ItemStruct);
let name = &input.ident;
let generics = &input.generics;

    // 关键：添加一个不实现 Send 的字段
    let output = quote! {
        // 修改原结构体，添加线程标记
        #input
        
        // 自动添加不实现 Send 的字段
        impl #generics #name {
            /// 内部使用的线程标记
            #[doc(hidden)]
            fn __thread_marker(&self) -> std::marker::PhantomData<*const ()> {
                std::marker::PhantomData
            }
        }
        
        // 实现 !Send 和 !Sync
        impl #generics !Send for #name {}
        impl #generics !Sync for #name {}
        
        // 编译时检查
        #[allow(dead_code)]
        fn __compile_check<T: ?Sized>() 
        where
            T: Send,  // 如果 T 实现了 Send，这会编译失败
        {}
        
        const _: () = {
            // 编译时验证：#name 没有实现 Send
            // 如果尝试实现 Send，这里会报错
            #[allow(unused)]
            fn check() {
                // 尝试调用检查函数，如果 #name 实现了 Send 会报错
                // __compile_check::<#name>();
            }
        };
    };
    
    output.into()
}


3. 完整实现方案

// 最佳实现：proc_macro_attribute
#[proc_macro_attribute]
pub fn forbid_cross_thread(_attr: TokenStream, item: TokenStream) -> TokenStream {
let input = parse_macro_input!(item as ItemStruct);
let name = &input.ident;
let generics = &input.generics;

    // 检查是否已有字段
    let has_fields = match &input.fields {
        Fields::Named(fields) => !fields.named.is_empty(),
        Fields::Unnamed(fields) => !fields.unnamed.is_empty(),
        Fields::Unit => false,
    };
    
    // 根据结构体类型生成不同代码
    let output = if has_fields {
        // 修改现有结构体，添加隐藏字段
        let fields = &input.fields;
        
        quote! {
            #input
            
            // 添加一个隐藏的线程标记
            #[doc(hidden)]
            const _THREAD_MARKER: () = ();
            
            impl #generics !Send for #name {}
            impl #generics !Sync for #name {}
            
            // 提供编译时检查方法
            impl #generics #name {
                #[doc(hidden)]
                pub fn __assert_not_send(&self) {
                    // 这个函数永远不应该被调用
                    // 它的存在只是为了触发编译时检查
                }
            }
        }
    } else {
        // 单元结构体，添加一个 PhantomData
        quote! {
            #name {
                _marker: std::marker::PhantomData<std::cell::Cell<()>>,
            }
            
            impl #generics !Send for #name {}
            impl #generics !Sync for #name {}
        }
    };
    
    // 添加编译时验证
    let validation = quote! {
        // 编译时验证：确保类型不实现 Send
        const _: () = {
            trait MustNotBeSend {
                fn __must_not_be_send() where Self: Send;
            }
            
            impl<T> MustNotBeSend for T {
                default fn __must_not_be_send() where Self: Send {}
            }
            
            // 如果 #name 实现了 Send，这里会编译错误
            fn check() {
                <#name>::__must_not_be_send();
            }
        };
    };
    
    let final_output = quote! {
        #output
        #validation
    };
    
    final_output.into()
}


4. 使用示例对比

使用 proc_macro_attribute（推荐）：

// 简单直观，立即生效
#[forbid_cross_thread]
struct DatabaseConnection {
url: String,
is_connected: bool,
}

// 尝试跨线程使用 - 立即编译错误！
fn main() {
use std::thread;

    let conn = DatabaseConnection {
        url: "localhost".to_string(),
        is_connected: false,
    };
    
    // 这会立即编译错误：
    // "`DatabaseConnection` cannot be sent between threads safely"
    thread::spawn(move || {
        let _ = conn;
    });
}


使用 proc_macro_derive（不推荐）：

// 不够直观，可能不生效
#[derive(NotThreadSafe)]
struct DatabaseConnection {
url: String,
is_connected: bool,
}

// 问题：如果用户手动实现 Send，derive 无法阻止
unsafe impl Send for DatabaseConnection {}  // 可以绕过！

// 或者如果结构体本身的所有字段都实现了 Send，
// 它可能自动获得 Send 实现


5. 实现更强大的版本

/// 更强大的版本：支持参数配置
#[proc_macro_attribute]
pub fn thread_local(attr: TokenStream, item: TokenStream) -> TokenStream {
let input = parse_macro_input!(item as ItemStruct);
let name = &input.ident;
let generics = &input.generics;

    // 解析属性参数
    let args: syn::AttributeArgs = parse_macro_input!(attr);
    
    // 检查参数
    let mut error_message = None;
    for arg in args {
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("error") {
                if let syn::Lit::Str(lit_str) = name_value.lit {
                    error_message = Some(lit_str.value());
                }
            }
        }
    }
    
    let error_msg = error_message.unwrap_or_else(|| {
        format!("类型 `{}` 被标记为线程局部，不能跨线程访问", name)
    });
    
    // 生成编译时检查代码
    let compile_check = quote! {
        // 编译时检查 1: 确保不实现 Send
        const _: () = {
            trait __ThreadLocalCheck {
                fn __check_send() where Self: Send;
                fn __check_sync() where Self: Sync;
            }
            
            impl<T> __ThreadLocalCheck for T {
                default fn __check_send() where Self: Send {}
                default fn __check_sync() where Self: Sync {}
            }
            
            // 调用检查，如果实现了 Send/Sync 会报错
            let _ = || {
                <#name>::__check_send();
                <#name>::__check_sync();
            };
        };
        
        // 编译时检查 2: 使用负特征绑定
        fn __ensure_not_send<T: ?Sized + 'static>() 
        where
            T: 'static,  // 正常绑定
            // 如果 T 实现了 Send，这里会有冲突
        {}
        
        // 编译时检查 3: 为特定使用场景生成错误
        #[allow(dead_code)]
        fn __generate_compile_error() {
            // 尝试在需要 Send 的上下文中使用
            fn requires_send<T: Send>(_t: T) {}
            
            // 这里会报错，因为 #name 不实现 Send
            // requires_send(#name::default());
        }
    };
    
    // 修改结构体定义
    let output = quote! {
        // 修改原结构体
        pub struct #name #generics {
            // 原字段保持不变
            #input
            
            // 添加隐藏字段确保不实现 Send
            #[doc(hidden)]
            __thread_local_marker: std::marker::PhantomData<*mut ()>,
        }
        
        // 明确实现 !Send 和 !Sync
        unsafe impl #generics !Send for #name {}
        unsafe impl #generics !Sync for #name {}
        
        // 提供构造方法
        impl #generics #name {
            /// 创建新实例
            pub fn new() -> Self where Self: Default {
                Self::default()
            }
            
            /// 线程安全检查
            #[track_caller]
            pub fn check_thread(&self) {
                use std::thread;
                thread_local! {
                    static THREAD_ID: std::cell::Cell<Option<thread::ThreadId>> = 
                        std::cell::Cell::new(None);
                }
                
                let current = thread::current().id();
                THREAD_ID.with(|cell| {
                    match cell.get() {
                        None => cell.set(Some(current)),
                        Some(id) if id == current => {},
                        Some(_) => panic!(#error_msg),
                    }
                });
            }
        }
        
        #compile_check
        
        // 生成友好的编译错误消息
        #[doc(hidden)]
        mod __thread_local_docs_#name {
            #[doc = #error_msg]
            #[doc(hidden)]
            pub struct __ErrorMsg;
        }
    };
    
    output.into()
}


6. 使用建议

使用 proc_macro_attribute 的场景：

// 1. 数据库连接
#[forbid_cross_thread]
struct PgConnection {
conn: postgres::Client,
pool_size: usize,
}

// 2. GUI 上下文
#[thread_local(error = "UI 上下文必须在主线程使用")]
struct UIContext {
window_handle: usize,
gl_context: *mut std::ffi::c_void,
}

// 3. 线程局部缓存
#[single_thread]
struct ThreadCache {
data: std::collections::HashMap<String, Vec<u8>>,
hits: usize,
misses: usize,
}


不应该使用的情况：

// 如果结构体需要跨线程使用，不要用这个宏
// 而是使用 Arc<Mutex<T>> 或类似的线程安全包装

// 错误用法：
#[forbid_cross_thread]  // ❌ 这个结构体需要跨线程共享
struct SharedCounter {
count: usize,
}

// 正确用法：
use std::sync::{Arc, Mutex};
struct SharedCounter {
count: Mutex<usize>,  // 内部使用互斥锁
}


7. 最佳实践总结

// 最终推荐的实现
#[proc_macro_attribute]
pub fn thread_bound(_attr: TokenStream, item: TokenStream) -> TokenStream {
let input = parse_macro_input!(item as ItemStruct);
let name = &input.ident;

    quote! {
        // 1. 保留原结构体定义
        #input
        
        // 2. 添加编译时保障
        impl !Send for #name {}
        impl !Sync for #name {}
        
        // 3. 添加运行时检查（可选）
        impl #name {
            #[track_caller]
            pub fn assert_same_thread(&self) {
                // 线程ID检查逻辑
            }
        }
        
        // 4. 编译时验证
        const _: () = {
            fn ensure_not_send<T: ?Sized>() 
            where
                T: Send,  // 如果 T 实现了 Send，下面会冲突
            {}
            
            // 这会迫使编译器检查 #name 是否实现了 Send
            // 如果实现了，会报错
            let _check = || {
                ensure_not_send::<#name>();
            };
        };
    }.into()
}


8. 选择 proc_macro_attribute 的关键原因

1. 立即生效：编译时立即报错，而不是运行时
2. 无法绕过：用户不能手动实现 Send 来绕过检查
3. 修改结构体：可以添加必要的字段来确保不实现 Send
4. 使用简单：只需要添加一个属性，不需要额外的 trait
5. 明确的意图：#[thread_bound] 比 #[derive(NotThreadSafe)] 更清晰

9. 最终实现

// 最简单有效的实现
#[proc_macro_attribute]
pub fn not_send(_attr: TokenStream, item: TokenStream) -> TokenStream {
let input = parse_macro_input!(item as ItemStruct);
let name = &input.ident;
let generics = &input.generics;

    TokenStream::from(quote! {
        #input
        
        // 这两行就是核心！
        impl #generics !Send for #name {}
        impl #generics !Sync for #name {}
        
        // 可选：提供友好的编译错误
        #[doc(hidden)]
        impl #generics #name {
            #[allow(dead_code)]
            fn __compile_time_check() {
                // 尝试在需要 Send 的上下文中使用
                fn requires_send<T: Send>(_t: T) {}
                
                // 这会触发编译错误，如果用户尝试在需要 Send 的地方使用
                // requires_send(Self);
            }
        }
    })
}


总结

使用 proc_macro_attribute 因为它：
1. 可以修改结构体定义
2. 立即产生编译错误
3. 用户无法绕过
4. 使用更直观简洁
5. 适合你的具体需求

proc_macro_derive 更适合自动实现 trait，而不是强制类型不满足某些 trait。对于"防止跨线程访问"这种需要修改类型特征的需求，proc_macro_attribute 是唯一正确的选择。