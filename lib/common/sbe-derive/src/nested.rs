//! Nested message support for SBE

use proc_macro2::TokenStream;
use quote::quote;

/// Check if a type is a nested message (user-defined struct)
#[allow(dead_code)]
pub fn is_nested_message(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let ident = segment.ident.to_string();
            // Not a primitive or standard type
            !matches!(
                ident.as_str(),
                "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64"
                    | "bool" | "char" | "Option" | "Vec" | "String"
            )
        } else {
            false
        }
    } else {
        false
    }
}

/// Generate nested message encoder call
#[allow(dead_code)]
pub fn generate_nested_encoder_call(
    field_name: &syn::Ident,
    field_ty: &syn::Type,
    offset: usize,
) -> TokenStream {
    let encoder_name = if let syn::Type::Path(type_path) = field_ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = &segment.ident;
            quote::format_ident!("{}Encoder", type_name)
        } else {
            return quote! {};
        }
    } else {
        return quote! {};
    };

    quote! {
        #[inline]
        pub fn #field_name(&mut self) -> #encoder_name {
            let offset = self.offset + #offset;
            #encoder_name::default().wrap(
                self.buf.clone(), // Note: This is simplified
                offset
            )
        }
    }
}

/// Generate nested message decoder call
pub fn generate_nested_decoder_call(
    field_name: &syn::Ident,
    field_ty: &syn::Type,
    offset: usize,
) -> TokenStream {
    let decoder_name = if let syn::Type::Path(type_path) = field_ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = &segment.ident;
            quote::format_ident!("{}Decoder", type_name)
        } else {
            return quote! {};
        }
    } else {
        return quote! {};
    };

    quote! {
        #[inline]
        pub fn #field_name(&self) -> #decoder_name {
            let offset = self.offset + #offset;
            #decoder_name::default().wrap(
                self.buf,
                offset,
                0, // block_length - would need to be calculated
                self.acting_version
            )
        }
    }
}
