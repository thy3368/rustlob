//! SBE (Simple Binary Encoding) derive macros
//!
//! This crate provides `#[derive(SbeEncode, SbeDecode)]` macros for automatic
//! generation of SBE encoding/decoding code according to FIX SBE 2.0 specification.

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attrs;
mod codegen;
mod enums;
mod groups;
mod nested;
mod time_types;
mod types;
mod xml_schema;

use codegen::{generate_decoder, generate_encoder};
use enums::generate_enum_impl;

/// Derive macro for SBE encoding
///
/// # Example
/// ```ignore
/// #[derive(SbeEncode)]
/// #[sbe(template_id = 1, schema_id = 1, version = 0)]
/// struct Trade {
///     #[sbe(id = 0)]
///     trade_id: u64,
///     #[sbe(id = 1)]
///     symbol: u8,
///     #[sbe(id = 2)]
///     price: f64,
///     #[sbe(id = 3)]
///     quantity: i32,
/// }
/// ```
#[proc_macro_derive(SbeEncode, attributes(sbe))]
pub fn derive_sbe_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_encoder(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro for SBE decoding
///
/// # Example
/// ```ignore
/// #[derive(SbeDecode)]
/// #[sbe(template_id = 1, schema_id = 1, version = 0)]
/// struct Trade {
///     #[sbe(id = 0)]
///     trade_id: u64,
///     #[sbe(id = 1)]
///     symbol: u8,
///     #[sbe(id = 2)]
///     price: f64,
///     #[sbe(id = 3)]
///     quantity: i32,
/// }
/// ```
#[proc_macro_derive(SbeDecode, attributes(sbe))]
pub fn derive_sbe_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_decoder(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive macro for SBE enum support
///
/// Generates From<u8> and Into<u8> implementations for enums.
///
/// # Example
/// ```ignore
/// #[derive(SbeEnum)]
/// enum Side {
///     Buy,
///     Sell,
/// }
/// ```
#[proc_macro_derive(SbeEnum, attributes(sbe))]
pub fn derive_sbe_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match generate_enum_impl(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
