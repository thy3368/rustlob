// 定义 #[immutable] 属性宏
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_attribute]
pub fn immutable(_args: TokenStream, input: TokenStream) -> TokenStream {
let input = parse_macro_input!(input as DeriveInput);
let name = &input.ident;

    // 检查是否为结构体
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("只支持具名字段的结构体"),
        },
        _ => panic!("只支持结构体"),
    };
    
    // 生成字段名列表
    let field_names: Vec<_> = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    
    // 生成 getter 方法
    let getters = field_names.iter().map(|field_name| {
        let method_name = field_name;
        quote! {
            pub const fn #method_name(&self) -> &#field_name {
                &self.#field_name
            }
        }
    });
    
    let expanded = quote! {
        #input
        
        impl #name {
            #(#getters)*
            
            // 不允许可变方法
        }
    };
    
    expanded.into()
}

#[immutable]
pub struct User {
id: u64,
name: String,
email: String,
}

// 自动生成 getter，没有 setter
let user = User { id: 1, name: "Alice".into(), email: "alice@example.com".into() };
println!("ID: {}", user.id());
// user.set_id(2); // ❌ 编译错误：没有这个方法


