//! Zero-copy view code generation

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

use crate::attrs::SbeFieldAttrs;
use crate::types::{OffsetCalculator, TypeMapper};

/// Generate zero-copy view type
pub fn generate_view(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let view_name = quote::format_ident!("{}View", name);

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return Err(syn::Error::new_spanned(input, "SbeView only supports named fields")),
        },
        _ => return Err(syn::Error::new_spanned(input, "SbeView only supports structs")),
    };

    let mut offset_calc = OffsetCalculator::new();
    let mut field_methods = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        if TypeMapper::is_repeating_group(field_ty) {
            continue;
        }
        if TypeMapper::is_var_data(field_ty) {
            continue;
        }

        // Get offset for this field (advances the counter)
        let offset = offset_calc.next_offset(field_ty).unwrap_or_else(|| offset_calc.total_size());

        let accessor = generate_view_accessor(field_name, field_ty, offset);
        field_methods.push(accessor);

        // For custom types with explicit size, we need to skip additional bytes
        // because next_offset only skips the size returned by TypeMapper::type_size
        // which returns None for unknown types
        let explicit_size = SbeFieldAttrs::from_attributes(&field.attrs).ok().and_then(|a| a.size);
        if let Some(size) = explicit_size {
            let known_size = TypeMapper::type_size(field_ty).unwrap_or(0);
            if size > known_size {
                offset_calc.skip_composite(size - known_size);
            }
        }
    }

    let block_length = offset_calc.total_size();

    let output = quote! {
        #[derive(Debug, Clone, Copy)]
        pub struct #view_name<'a> { data: &'a [u8] }

        impl<'a> #view_name<'a> {
            #[inline]
            pub fn from_bytes(buffer: &'a [u8]) -> Option<Self> {
                if buffer.len() >= #block_length { Some(Self { data: buffer }) } else { None }
            }

            #[inline]
            pub unsafe fn from_bytes_unchecked(buffer: &'a [u8]) -> Self { Self { data: buffer } }

            #[inline]
            pub fn as_bytes(&self) -> &'a [u8] { self.data }

            pub const fn block_length() -> usize { #block_length }

            #(#field_methods)*
        }
    };

    Ok(output)
}

/// Generate view accessor for a field type
///
/// Uses ZeroCopyDecode trait if available, otherwise falls back to primitive handling
fn generate_view_accessor(
    field_name: &syn::Ident,
    field_ty: &syn::Type,
    offset: usize,
) -> TokenStream {
    let ty_str = quote!(#field_ty).to_string();

    // If type implements ZeroCopyDecode, use the trait method
    // Otherwise, use primitive handling
    match ty_str.as_str() {
        // Primitive types that have ZeroCopyDecode implementation
        "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "f32" | "f64" | "bool"
        | "char" => {
            quote! {
                #[inline]
                pub fn #field_name(&self) -> #field_ty {
                    <#field_ty as sbe::ZeroCopyDecode>::zero_copy_decode(self.data, #offset)
                }
            }
        }
        // Custom types - generate code that uses ZeroCopyDecode trait
        // The generated code will fail at compile time if the type doesn't implement ZeroCopyDecode
        _ => {
            quote! {
                #[inline]
                pub fn #field_name(&self) -> #field_ty {
                    <#field_ty as sbe::ZeroCopyDecode>::zero_copy_decode(self.data, #offset)
                }
            }
        }
    }
}
