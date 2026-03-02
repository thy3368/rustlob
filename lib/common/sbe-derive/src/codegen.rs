//! Code generation for SBE encoders and decoders

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

use crate::attrs::{SbeContainerAttrs, SbeFieldAttrs};
use crate::types::{OffsetCalculator, TypeMapper};

/// Generate encoder implementation
pub fn generate_encoder(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let encoder_name = quote::format_ident!("{}Encoder", name);

    let container_attrs = SbeContainerAttrs::from_attributes(&input.attrs)?;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "SbeEncode only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "SbeEncode only supports structs",
            ))
        }
    };

    let mut offset_calc = OffsetCalculator::new();
    let mut field_methods = Vec::new();
    let mut var_data_fields = Vec::new();

    // Process fixed-size fields
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        let field_attrs = SbeFieldAttrs::from_attributes(&field.attrs)?;

        // Check if this is a variable-length field
        if TypeMapper::is_var_data(field_ty) {
            var_data_fields.push((field_name.clone(), field_ty.clone()));
            continue; // Skip offset calculation for var-data
        }

        let offset = offset_calc
            .next_offset(field_ty)
            .ok_or_else(|| syn::Error::new_spanned(field_ty, "Unsupported field type"))?;

        let field_size = TypeMapper::type_size(field_ty)
            .ok_or_else(|| syn::Error::new_spanned(field_ty, "Cannot determine field size"))?;
        let is_optional = TypeMapper::is_optional(field_ty);
        let is_constant = field_attrs.presence.as_deref() == Some("constant");

        // Skip constant fields - they don't get encoded
        if is_constant {
            continue;
        }

        let doc_comment = format!(
            "primitive field '{}'\n - encodedOffset: {}\n - encodedLength: {}\n - presence: {}",
            field_name,
            offset,
            field_size,
            if is_optional { "optional" } else { "required" }
        );

        let method = if is_optional {
            // Optional field handling
            let inner_ty = TypeMapper::inner_type(field_ty)
                .ok_or_else(|| syn::Error::new_spanned(field_ty, "Invalid Option type"))?;
            let write_method = TypeMapper::write_method(inner_ty)
                .ok_or_else(|| syn::Error::new_spanned(inner_ty, "Unsupported field type"))?;
            let write_method = quote::format_ident!("{}", write_method);
            let null_value = TypeMapper::null_value(inner_ty)
                .ok_or_else(|| syn::Error::new_spanned(inner_ty, "No null value for type"))?;
            let null_value: proc_macro2::TokenStream = null_value.parse().unwrap();

            quote! {
                #[doc = #doc_comment]
                #[inline]
                pub fn #field_name(&mut self, value: #field_ty) {
                    let offset = self.offset + #offset;
                    match value {
                        Some(v) => self.get_buf_mut().#write_method(offset, v),
                        None => self.get_buf_mut().#write_method(offset, #null_value),
                    }
                }
            }
        } else if matches!(field_ty, syn::Type::Array(_)) {
            // Fixed-length array handling
            quote! {
                #[doc = #doc_comment]
                #[inline]
                pub fn #field_name(&mut self, value: &#field_ty) {
                    let offset = self.offset + #offset;
                    self.get_buf_mut().put_slice_at(offset, value);
                }
            }
        } else {
            // Regular field handling
            let write_method = TypeMapper::write_method(field_ty)
                .ok_or_else(|| syn::Error::new_spanned(field_ty, "Unsupported field type"))?;
            let write_method = quote::format_ident!("{}", write_method);

            // Special handling for bool and char types
            let value_expr = if let syn::Type::Path(type_path) = field_ty {
                if let Some(segment) = type_path.path.segments.last() {
                    match segment.ident.to_string().as_str() {
                        "bool" => quote! { if value { 1u8 } else { 0u8 } },
                        "char" => quote! { value as u8 },
                        _ => quote! { value },
                    }
                } else {
                    quote! { value }
                }
            } else {
                quote! { value }
            };

            // Add range validation if specified
            let validation = if field_attrs.min_value.is_some() || field_attrs.max_value.is_some() {
                let mut checks = Vec::new();

                if let Some(min) = &field_attrs.min_value {
                    let min_val: proc_macro2::TokenStream = min.parse().unwrap();
                    checks.push(quote! {
                        assert!(value >= #min_val, "Value {} is below minimum {}", value, #min_val);
                    });
                }

                if let Some(max) = &field_attrs.max_value {
                    let max_val: proc_macro2::TokenStream = max.parse().unwrap();
                    checks.push(quote! {
                        assert!(value <= #max_val, "Value {} is above maximum {}", value, #max_val);
                    });
                }

                quote! { #(#checks)* }
            } else {
                quote! {}
            };

            quote! {
                #[doc = #doc_comment]
                #[inline]
                pub fn #field_name(&mut self, value: #field_ty) {
                    #validation
                    let offset = self.offset + #offset;
                    self.get_buf_mut().#write_method(offset, #value_expr);
                }
            }
        };

        field_methods.push(method);
    }

    // Generate variable-length data methods (appended after block)
    for (var_field_name, _var_field_ty) in &var_data_fields {
        let doc_comment = format!("variable-length data field '{}'", var_field_name);

        let method = quote! {
            #[doc = #doc_comment]
            #[inline]
            pub fn #var_field_name(&mut self, value: &[u8]) {
                let length = value.len() as u16;
                let offset = self.limit;

                // Write length prefix (2 bytes)
                self.get_buf_mut().put_u16_at(offset, length);

                // Write data
                self.get_buf_mut().put_slice_at(offset + 2, value);

                // Update limit
                self.limit = offset + 2 + value.len();
            }
        };

        field_methods.push(method);
    }

    let block_length = offset_calc.total_size() as u16;
    let template_id = container_attrs.template_id.unwrap_or(1);
    let schema_id = container_attrs.schema_id.unwrap_or(1);
    let version = container_attrs.version.unwrap_or(0);

    let output = quote! {
        pub use encoder::#encoder_name;

        pub mod encoder {
            use super::*;
            use sbe::{Writer, Encoder};

            pub const SBE_BLOCK_LENGTH: u16 = #block_length;
            pub const SBE_TEMPLATE_ID: u16 = #template_id;
            pub const SBE_SCHEMA_ID: u16 = #schema_id;
            pub const SBE_SCHEMA_VERSION: u16 = #version;

            #[derive(Debug, Default)]
            pub struct #encoder_name<'a> {
                buf: sbe::WriteBuf<'a>,
                initial_offset: usize,
                offset: usize,
                limit: usize,
            }

            impl<'a> sbe::Writer<'a> for #encoder_name<'a> {
                #[inline]
                fn get_buf_mut(&mut self) -> &mut sbe::WriteBuf<'a> {
                    &mut self.buf
                }
            }

            impl<'a> sbe::Encoder<'a> for #encoder_name<'a> {
                #[inline]
                fn get_limit(&self) -> usize {
                    self.limit
                }

                #[inline]
                fn set_limit(&mut self, limit: usize) {
                    self.limit = limit;
                }
            }

            impl<'a> #encoder_name<'a> {
                pub fn wrap(mut self, buf: sbe::WriteBuf<'a>, offset: usize) -> Self {
                    let limit = offset + SBE_BLOCK_LENGTH as usize;
                    self.buf = buf;
                    self.initial_offset = offset;
                    self.offset = offset;
                    self.limit = limit;
                    self
                }

                #[inline]
                pub fn encoded_length(&self) -> usize {
                    self.limit - self.offset
                }

                pub fn header(mut self, offset: usize) -> sbe::message_header_codec::MessageHeaderEncoder<Self> {
                    // Adjust encoder offset to point to message body (after header)
                    self.offset = offset + sbe::message_header_codec::ENCODED_LENGTH;
                    self.initial_offset = offset + sbe::message_header_codec::ENCODED_LENGTH;
                    self.limit = self.offset + SBE_BLOCK_LENGTH as usize;

                    let mut header = sbe::message_header_codec::MessageHeaderEncoder::default().wrap(self, offset);
                    header.block_length(SBE_BLOCK_LENGTH);
                    header.template_id(SBE_TEMPLATE_ID);
                    header.schema_id(SBE_SCHEMA_ID);
                    header.version(SBE_SCHEMA_VERSION);
                    header
                }

                #(#field_methods)*
            }
        }
    };

    Ok(output)
}

/// Generate decoder implementation
pub fn generate_decoder(input: &DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let decoder_name = quote::format_ident!("{}Decoder", name);

    let container_attrs = SbeContainerAttrs::from_attributes(&input.attrs)?;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "SbeDecode only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "SbeDecode only supports structs",
            ))
        }
    };

    let mut offset_calc = OffsetCalculator::new();
    let mut field_methods = Vec::new();
    let mut var_data_fields = Vec::new();

    // Process fixed-size fields
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        let field_attrs = SbeFieldAttrs::from_attributes(&field.attrs)?;

        // Check if this is a variable-length field
        if TypeMapper::is_var_data(field_ty) {
            var_data_fields.push((field_name.clone(), field_ty.clone()));
            continue; // Skip offset calculation for var-data
        }

        let offset = offset_calc
            .next_offset(field_ty)
            .ok_or_else(|| syn::Error::new_spanned(field_ty, "Unsupported field type"))?;

        let is_optional = TypeMapper::is_optional(field_ty);
        let is_constant = field_attrs.presence.as_deref() == Some("constant");
        let since_version = field_attrs.since_version;
        let _min_value = field_attrs.min_value.as_ref();
        let _max_value = field_attrs.max_value.as_ref();

        // Handle constant fields - return constant value
        if is_constant {
            if let Some(const_value) = &field_attrs.constant {
                let const_value: proc_macro2::TokenStream = const_value.parse().unwrap();
                let doc_comment = format!("constant field - always returns {}", const_value);
                let method = quote! {
                    #[doc = #doc_comment]
                    #[inline]
                    pub fn #field_name(&self) -> #field_ty {
                        #const_value
                    }
                };
                field_methods.push(method);
            }
            continue;
        }

        let version_info = if let Some(ver) = since_version {
            format!(" (since version {})", ver)
        } else {
            String::new()
        };

        let doc_comment = if is_optional {
            format!("primitive field - 'OPTIONAL'{}", version_info)
        } else {
            format!("primitive field - 'REQUIRED'{}", version_info)
        };

        let method = if is_optional {
            // Optional field handling
            let inner_ty = TypeMapper::inner_type(field_ty)
                .ok_or_else(|| syn::Error::new_spanned(field_ty, "Invalid Option type"))?;
            let read_method = TypeMapper::read_method(inner_ty)
                .ok_or_else(|| syn::Error::new_spanned(inner_ty, "Unsupported field type"))?;
            let read_method = quote::format_ident!("{}", read_method);
            let null_value = TypeMapper::null_value(inner_ty)
                .ok_or_else(|| syn::Error::new_spanned(inner_ty, "No null value for type"))?;
            let null_value: proc_macro2::TokenStream = null_value.parse().unwrap();

            quote! {
                #[doc = #doc_comment]
                #[inline]
                pub fn #field_name(&self) -> #field_ty {
                    let value = self.get_buf().#read_method(self.offset + #offset);
                    if value == #null_value {
                        None
                    } else {
                        Some(value)
                    }
                }
            }
        } else if matches!(field_ty, syn::Type::Array(_)) {
            // Fixed-length array handling
            if let syn::Type::Array(arr) = field_ty {
                let elem_ty = &arr.elem;
                let len = &arr.len;
                quote! {
                    #[doc = #doc_comment]
                    #[inline]
                    pub fn #field_name(&self) -> #field_ty {
                        let mut result = [<#elem_ty>::default(); #len];
                        let slice = self.get_buf().get_slice_at(self.offset + #offset, #len);
                        result.copy_from_slice(slice);
                        result
                    }
                }
            } else {
                return Err(syn::Error::new_spanned(field_ty, "Invalid array type"));
            }
        } else {
            // Regular field handling
            let read_method = TypeMapper::read_method(field_ty)
                .ok_or_else(|| syn::Error::new_spanned(field_ty, "Unsupported field type"))?;
            let read_method = quote::format_ident!("{}", read_method);

            // Special handling for bool and char types
            let value_expr = if let syn::Type::Path(type_path) = field_ty {
                if let Some(segment) = type_path.path.segments.last() {
                    match segment.ident.to_string().as_str() {
                        "bool" => quote! { self.get_buf().#read_method(self.offset + #offset) != 0 },
                        "char" => quote! { self.get_buf().#read_method(self.offset + #offset) as char },
                        _ => quote! { self.get_buf().#read_method(self.offset + #offset) },
                    }
                } else {
                    quote! { self.get_buf().#read_method(self.offset + #offset) }
                }
            } else {
                quote! { self.get_buf().#read_method(self.offset + #offset) }
            };

            let base_method = quote! {
                #[doc = #doc_comment]
                #[inline]
                pub fn #field_name(&self) -> #field_ty {
                    #value_expr
                }
            };

            // Wrap with version check if sinceVersion is specified
            if let Some(ver) = since_version {
                quote! {
                    #[doc = #doc_comment]
                    #[inline]
                    pub fn #field_name(&self) -> Option<#field_ty> {
                        if self.acting_version >= #ver {
                            Some(#value_expr)
                        } else {
                            None
                        }
                    }
                }
            } else {
                base_method
            }
        };

        field_methods.push(method);
    }

    // Generate variable-length data methods (read from after block)
    if !var_data_fields.is_empty() {
        for (var_field_name, _var_field_ty) in &var_data_fields {
            let doc_comment = format!("variable-length data field - 'REQUIRED'");

            let method = quote! {
                #[doc = #doc_comment]
                #[inline]
                pub fn #var_field_name(&self) -> Vec<u8> {
                    let offset = self.limit;

                    // Read length prefix (2 bytes)
                    let length = self.get_buf().get_u16_at(offset) as usize;

                    // Read data
                    let data = self.get_buf().get_slice_at(offset + 2, length);
                    data.to_vec()
                }
            };

            field_methods.push(method);
        }
    }

    let block_length = offset_calc.total_size() as u16;
    let template_id = container_attrs.template_id.unwrap_or(1);
    let schema_id = container_attrs.schema_id.unwrap_or(1);
    let version = container_attrs.version.unwrap_or(0);

    let output = quote! {
        pub use decoder::#decoder_name;

        pub mod decoder {
            use super::*;
            use sbe::{Reader, Decoder, ActingVersion};

            pub const SBE_BLOCK_LENGTH: u16 = #block_length;
            pub const SBE_TEMPLATE_ID: u16 = #template_id;
            pub const SBE_SCHEMA_ID: u16 = #schema_id;
            pub const SBE_SCHEMA_VERSION: u16 = #version;

            #[derive(Clone, Copy, Debug, Default)]
            pub struct #decoder_name<'a> {
                buf: sbe::ReadBuf<'a>,
                initial_offset: usize,
                offset: usize,
                limit: usize,
                pub acting_block_length: u16,
                pub acting_version: u16,
            }

            impl sbe::ActingVersion for #decoder_name<'_> {
                #[inline]
                fn acting_version(&self) -> u16 {
                    self.acting_version
                }
            }

            impl<'a> sbe::Reader<'a> for #decoder_name<'a> {
                #[inline]
                fn get_buf(&self) -> &sbe::ReadBuf<'a> {
                    &self.buf
                }
            }

            impl<'a> sbe::Decoder<'a> for #decoder_name<'a> {
                #[inline]
                fn get_limit(&self) -> usize {
                    self.limit
                }

                #[inline]
                fn set_limit(&mut self, limit: usize) {
                    self.limit = limit;
                }
            }

            impl<'a> #decoder_name<'a> {
                pub fn wrap(
                    mut self,
                    buf: sbe::ReadBuf<'a>,
                    offset: usize,
                    acting_block_length: u16,
                    acting_version: u16,
                ) -> Self {
                    let limit = offset + acting_block_length as usize;
                    self.buf = buf;
                    self.initial_offset = offset;
                    self.offset = offset;
                    self.limit = limit;
                    self.acting_block_length = acting_block_length;
                    self.acting_version = acting_version;
                    self
                }

                #[inline]
                pub fn encoded_length(&self) -> usize {
                    self.limit - self.offset
                }

                pub fn header(
                    self,
                    mut header: sbe::message_header_codec::MessageHeaderDecoder<sbe::ReadBuf<'a>>,
                    offset: usize,
                ) -> Self {
                    debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
                    let acting_block_length = header.block_length();
                    let acting_version = header.version();

                    self.wrap(
                        header.parent().unwrap(),
                        offset + sbe::message_header_codec::ENCODED_LENGTH,
                        acting_block_length,
                        acting_version,
                    )
                }

                #(#field_methods)*
            }
        }
    };

    Ok(output)
}
