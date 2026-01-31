use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

// mod test; // 移除 test 模块，在过程宏 crate 中不能直接使用自身定义的宏
/// Immutable 属性宏 - 将结构体标记为不可变
///
/// # 功能
/// - 自动生成 const getter 方法（返回字段的不可变引用）
/// - 自动生成 `pub const fn new` 构造函数
/// - 强制所有字段为私有（防止外部直接修改）
/// - 符合 Clean Architecture 中的值对象模式
///
/// # 编译时检查
/// - 检测到 `pub` 字段会报编译错误
/// - 确保所有字段只能通过 getter 访问
///
/// # 示例
/// ```ignore
/// #[immutable]
/// pub struct AccountId {
///     id: u64,          // ✅ 私有字段
///     name: String,     // ✅ 私有字段
/// }
///
/// // ❌ 编译错误
/// #[immutable]
/// pub struct BadExample {
///     pub id: u64,      // ❌ 错误：不可变结构体不能有 pub 字段
/// }
///
/// // 使用自动生成的构造函数和 getter
/// let account = AccountId::new(1, "test".into());
/// println!("ID: {:?}", account.id());
/// println!("Name: {}", account.name());
/// ```
#[proc_macro_attribute]
pub fn immutable(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // 检查是否为结构体
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("#[immutable] 只支持具名字段的结构体"),
        },
        _ => panic!("#[immutable] 只支持结构体"),
    };

    // 检查所有字段是否为私有
    for field in fields.iter() {
        if matches!(field.vis, syn::Visibility::Public(_)) {
            let field_name = field.ident.as_ref().unwrap();
            panic!(
                "#[immutable] 错误: 字段 '{}' 不能使用 'pub' 修饰符。\n\
                不可变结构体的所有字段必须是私有的，只能通过自动生成的 getter 方法访问。\n\
                请移除 'pub' 关键字: {} -> {}",
                field_name,
                quote!(pub #field_name),
                quote!(#field_name)
            );
        }
    }

    // 生成字段名和类型列表
    let field_info: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap();
            let field_type = &f.ty;
            (field_name, field_type)
        })
        .collect();

    // 生成 const getter 方法
    let getters = field_info.iter().map(|(field_name, field_type)| {
        quote! {
            #[inline]
            pub const fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        }
    });

    // 生成 `pub const fn new` 构造函数
    let field_names: Vec<_> = field_info.iter().map(|(name, _)| name).collect();
    let field_types: Vec<_> = field_info.iter().map(|(_, ty)| ty).collect();

    let constructor = quote! {
        /// 创建新的不可变实例
        ///
        /// 此构造函数由 `#[immutable]` 宏自动生成
        #[inline]
        pub const fn new(
            #(#field_names: #field_types),*
        ) -> Self {
            Self {
                #(#field_names),*
            }
        }
    };

    let expanded = quote! {
        #input

        impl #impl_generics #name #ty_generics #where_clause {
            #constructor
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}

