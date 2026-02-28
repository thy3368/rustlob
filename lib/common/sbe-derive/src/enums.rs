//! Enum support for SBE derive macros

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Result};

/// Generate SBE enum encoding/decoding
pub fn generate_enum_impl(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data) => &data.variants,
        _ => return Err(syn::Error::new_spanned(input, "Expected enum")),
    };

    // Generate From<u8> and Into<u8> implementations
    let mut from_arms = Vec::new();
    let mut into_arms = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let discriminant = idx as u8;

        from_arms.push(quote! {
            #discriminant => #name::#variant_name,
        });

        into_arms.push(quote! {
            #name::#variant_name => #discriminant,
        });
    }

    let output = quote! {
        impl From<u8> for #name {
            fn from(value: u8) -> Self {
                match value {
                    #(#from_arms)*
                    _ => panic!("Invalid enum value: {}", value),
                }
            }
        }

        impl From<#name> for u8 {
            fn from(value: #name) -> u8 {
                match value {
                    #(#into_arms)*
                }
            }
        }

        impl #name {
            #[inline]
            pub fn from_u8(value: u8) -> Self {
                Self::from(value)
            }

            #[inline]
            pub fn to_u8(self) -> u8 {
                u8::from(self)
            }
        }
    };

    Ok(output)
}
