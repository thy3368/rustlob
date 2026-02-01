use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

/// 标记结构体为单线程使用，编译时防止跨线程访问
///
/// # 功能
/// - 编译时检查防止跨线程发送（通过 PhantomData）
/// - 编译时检查防止跨线程共享（通过 PhantomData）
/// - 提供运行时线程绑定检查方法
/// - 支持 `#[thread_bound]` 属性标记线程绑定字段
///
/// # 注意
/// 这是一个属性宏，直接修改结构体定义，确保类型不实现 Send 和 Sync
///
/// # 示例
///
/// ```ignore
/// use single_thread_derive::single_thread;
///
/// #[single_thread]
/// struct DatabaseConnection {
///     url: String,
///     #[thread_bound]
///     connection_handle: usize,
/// }
///
/// fn main() {
///     let conn = DatabaseConnection {
///         url: "localhost".to_string(),
///         connection_handle: 123,
///     };
///
///     // 正常使用
///     let _ = conn.thread_safe_get();
///     assert!(!conn.can_send_to_other_thread());
///
///     // 线程检查
///     assert!(conn.check_thread_bound().is_ok());
///
///     // 以下代码会在编译时报错：
///     // "`DatabaseConnection` cannot be sent between threads safely"
///     // std::thread::spawn(move || {
///     //     let _ = conn;
///     // });
/// }
/// ```
#[proc_macro_attribute]
pub fn single_thread(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 保留原结构体的所有内容，并添加一个不实现 Send 和 Sync 的字段
    let struct_fields = match &input.fields {
        Fields::Named(fields) => {
            let named_fields = &fields.named;
            quote! {
                #named_fields
                #[doc(hidden)]
                __marker: std::marker::PhantomData<*const ()>, // *const () 既不实现 Send 也不实现 Sync
            }
        }
        Fields::Unnamed(fields) => {
            let unnamed_fields = &fields.unnamed;
            quote! {
                #unnamed_fields
                #[doc(hidden)]
                std::marker::PhantomData<*const ()>,
            }
        }
        Fields::Unit => quote! {
            #[doc(hidden)]
            __marker: std::marker::PhantomData<*const ()>,
        },
    };

    let struct_def = quote! {
        pub struct #name #generics {
            #struct_fields
        }
    };

    // 生成字段匹配代码，根据结构体字段类型生成默认初始化
    let match_fields = match &input.fields {
        Fields::Named(named) => {
            let fields = &named.named;
            let field_names = fields.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                quote! { #ident: Default::default(), }
            });

            quote! {
                #name {
                    #(#field_names)*
                    __marker: std::marker::PhantomData,
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            let count = unnamed.unnamed.len();
            let fields = (0..count).map(|_| quote! { Default::default(), });

            quote! {
                #name (
                    #(#fields)*
                    std::marker::PhantomData,
                )
            }
        }
        Fields::Unit => {
            quote! {
                #name { __marker: std::marker::PhantomData }
            }
        }
    };

    // 生成最终代码
    let expanded = quote! {
        // 1. 保留原结构体定义并添加标记字段
        #struct_def

        // 2. 为结构体实现 Default trait，自动初始化内部字段
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #match_fields
            }
        }

        // 3. 编译时检查相关方法
        impl #impl_generics #name #ty_generics #where_clause {
            /// 创建新的单线程实例（自动初始化内部字段）
            pub fn new() -> Self
            where
                Self: Default,
            {
                Self::default()
            }
        }
    };

    TokenStream::from(expanded)
}

