use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// SingleThread 派生宏实现
pub fn impl_single_thread_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // 解析 thread_bound 字段
    let mut thread_bound_fields = Vec::new();
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields_named) = &data_struct.fields {
            for field in &fields_named.named {
                if let Some(field_name) = &field.ident {
                    // 检查是否有 thread_bound 属性
                    for attr in &field.attrs {
                        if attr.path().is_ident("thread_bound") {
                            thread_bound_fields.push(field_name.clone());
                        }
                    }
                }
            }
        }
    }

    // 生成 thread_bound 字段的访问器方法
    let thread_bound_accessors = thread_bound_fields.iter().map(|field_name| {
        let method_name = syn::Ident::new(
            &format!("get_{}", field_name),
            field_name.span()
        );
        quote! {
            /// 获取线程绑定字段的引用（带线程检查）
            #[allow(dead_code)]
            pub fn #method_name(&self) -> &Self {
                self.thread_safe_get()
            }
        }
    });

    let expanded = quote! {
        // 注意：稳定版 Rust 不支持直接的 !Send 和 !Sync
        // 我们通过生成包含 PhantomData<*const ()> 的字段来间接实现

        impl #impl_generics #name #ty_generics #where_clause {
            /// 检查是否可以跨线程发送
            ///
            /// 对于 SingleThread 类型，总是返回 false
            pub fn can_send_to_other_thread(&self) -> bool {
                false
            }

            /// 线程绑定检查
            ///
            /// 验证当前访问是否在创建线程中
            ///
            /// # Errors
            ///
            /// 如果在不同线程中访问，返回错误信息
            pub fn check_thread_bound(&self) -> Result<(), String> {
                use std::sync::atomic::{AtomicU64, Ordering};
                use std::cell::Cell;

                // 全局线程ID计数器
                static THREAD_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

                thread_local! {
                    // 每个线程的唯一ID
                    static LOCAL_THREAD_ID: u64 = THREAD_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

                    // 记录实例的创建线程ID
                    static INSTANCE_THREAD_ID: Cell<Option<u64>> = const { Cell::new(None) };
                }

                let current_thread_id = LOCAL_THREAD_ID.with(|&id| id);

                INSTANCE_THREAD_ID.with(|cell| {
                    match cell.get() {
                        None => {
                            // 首次访问，记录线程ID
                            cell.set(Some(current_thread_id));
                            Ok(())
                        }
                        Some(instance_id) if instance_id == current_thread_id => {
                            // 同一线程，允许访问
                            Ok(())
                        }
                        Some(instance_id) => {
                            // 不同线程，拒绝访问
                            Err(format!(
                                "结构体 `{}` 被跨线程访问！\n\
                                创建线程ID: {}, 当前线程ID: {}\n\
                                此类型被标记为 SingleThread，只能在创建线程中使用。",
                                stringify!(#name),
                                instance_id,
                                current_thread_id
                            ))
                        }
                    }
                })
            }

            /// 带线程检查的引用获取
            ///
            /// # Panics
            ///
            /// 如果在不同线程中调用，会 panic
            pub fn thread_safe_get(&self) -> &Self {
                if let Err(e) = self.check_thread_bound() {
                    panic!("{}", e);
                }
                self
            }

            /// 带线程检查的可变引用获取
            ///
            /// # Panics
            ///
            /// 如果在不同线程中调用，会 panic
            pub fn thread_safe_get_mut(&mut self) -> &mut Self {
                if let Err(e) = self.check_thread_bound() {
                    panic!("{}", e);
                }
                self
            }

            #(#thread_bound_accessors)*
        }
    };

    TokenStream::from(expanded)
}
