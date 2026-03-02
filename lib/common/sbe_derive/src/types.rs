//! Type mapping and offset calculation engine
//!
//! Maps Rust types to SBE types and calculates field offsets according to
//! FIX SBE 2.0 specification.

use syn::{Type, TypePath};

/// Maps Rust types to SBE primitive types and their byte sizes
pub struct TypeMapper;

/// Field type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FieldKind {
    Primitive,
    FixedArray,
    VarData,
    Optional,
    Constant,
    Decimal,
}

/// Decimal encoding configuration
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DecimalConfig {
    pub mantissa_type: String,
    pub exponent: i8,
}

impl TypeMapper {
    /// Get the byte size of a Rust type in SBE encoding
    /// Returns None for variable-length types (Vec<u8>)
    pub fn type_size(ty: &Type) -> Option<usize> {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                let ident = path.segments.last()?.ident.to_string();
                match ident.as_str() {
                    "u8" | "i8" | "bool" | "char" => Some(1),
                    "u16" | "i16" => Some(2),
                    "u32" | "i32" | "f32" => Some(4),
                    "u64" | "i64" | "f64" => Some(8),
                    "Vec" => None, // Variable-length, no fixed size
                    "Option" => {
                        // For Option<T>, size is same as T (null value encoded in-place)
                        if let syn::PathArguments::AngleBracketed(args) = &path.segments.last()?.arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                                return Self::type_size(inner_ty);
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            Type::Array(arr) => {
                // For arrays, multiply element size by length
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(len),
                    ..
                }) = &arr.len
                {
                    let elem_size = Self::type_size(&arr.elem)?;
                    let array_len: usize = len.base10_parse().ok()?;
                    Some(elem_size * array_len)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Map Rust type to SBE type name
    #[allow(dead_code)]
    pub fn rust_to_sbe_type(ty: &Type) -> Option<&'static str> {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                let ident = path.segments.last()?.ident.to_string();
                match ident.as_str() {
                    "u8" => Some("uint8"),
                    "u16" => Some("uint16"),
                    "u32" => Some("uint32"),
                    "u64" => Some("uint64"),
                    "i8" => Some("int8"),
                    "i16" => Some("int16"),
                    "i32" => Some("int32"),
                    "i64" => Some("int64"),
                    "f32" => Some("float"),
                    "f64" => Some("double"),
                    "bool" => Some("boolean"),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Get the null value for a given type (for optional fields)
    #[allow(dead_code)]
    pub fn null_value(ty: &Type) -> Option<String> {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                let ident = path.segments.last()?.ident.to_string();
                match ident.as_str() {
                    "u8" => Some("255".to_string()),
                    "u16" => Some("65535".to_string()),
                    "u32" => Some("4294967295".to_string()),
                    "u64" => Some("18446744073709551615".to_string()),
                    "i8" => Some("-128".to_string()),
                    "i16" => Some("-32768".to_string()),
                    "i32" => Some("-2147483648".to_string()),
                    "i64" => Some("-9223372036854775808".to_string()),
                    "f32" | "f64" => Some("f64::NAN".to_string()),
                    "bool" => Some("255".to_string()),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Get the WriteBuf method name for a type
    pub fn write_method(ty: &Type) -> Option<&'static str> {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                let ident = path.segments.last()?.ident.to_string();
                match ident.as_str() {
                    "u8" | "char" => Some("put_u8_at"),
                    "u16" => Some("put_u16_at"),
                    "u32" => Some("put_u32_at"),
                    "u64" => Some("put_u64_at"),
                    "i8" => Some("put_i8_at"),
                    "i16" => Some("put_i16_at"),
                    "i32" => Some("put_i32_at"),
                    "i64" => Some("put_i64_at"),
                    "f32" => Some("put_f32_at"),
                    "f64" => Some("put_f64_at"),
                    "bool" => Some("put_u8_at"), // bool encoded as u8
                    "Option" => {
                        // For Option<T>, use the write method of T
                        if let syn::PathArguments::AngleBracketed(args) = &path.segments.last()?.arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                                return Self::write_method(inner_ty);
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Get the ReadBuf method name for a type
    pub fn read_method(ty: &Type) -> Option<&'static str> {
        match ty {
            Type::Path(TypePath { path, .. }) => {
                let ident = path.segments.last()?.ident.to_string();
                match ident.as_str() {
                    "u8" | "char" => Some("get_u8_at"),
                    "u16" => Some("get_u16_at"),
                    "u32" => Some("get_u32_at"),
                    "u64" => Some("get_u64_at"),
                    "i8" => Some("get_i8_at"),
                    "i16" => Some("get_i16_at"),
                    "i32" => Some("get_i32_at"),
                    "i64" => Some("get_i64_at"),
                    "f32" => Some("get_f32_at"),
                    "f64" => Some("get_f64_at"),
                    "bool" => Some("get_u8_at"), // bool decoded from u8
                    "Option" => {
                        // For Option<T>, use the read method of T
                        if let syn::PathArguments::AngleBracketed(args) = &path.segments.last()?.arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                                return Self::read_method(inner_ty);
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Check if a type is Option<T>
    pub fn is_optional(ty: &Type) -> bool {
        if let Type::Path(TypePath { path, .. }) = ty {
            if let Some(segment) = path.segments.last() {
                return segment.ident == "Option";
            }
        }
        false
    }

    /// Extract inner type from Option<T>
    pub fn inner_type(ty: &Type) -> Option<&Type> {
        if let Type::Path(TypePath { path, .. }) = ty {
            if let Some(segment) = path.segments.last() {
                if segment.ident == "Option" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                            return Some(inner);
                        }
                    }
                }
            }
        }
        None
    }

    /// Check if a type is Vec<u8> (variable-length data)
    pub fn is_var_data(ty: &Type) -> bool {
        if let Type::Path(TypePath { path, .. }) = ty {
            if let Some(segment) = path.segments.last() {
                if segment.ident == "Vec" {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) = args.args.first() {
                            if let Some(inner_seg) = inner_path.path.segments.last() {
                                return inner_seg.ident == "u8";
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Check if a type is an enum (user-defined type that's not a primitive)
    #[allow(dead_code)]
    pub fn is_enum(ty: &Type) -> bool {
        if let Type::Path(TypePath { path, .. }) = ty {
            if let Some(segment) = path.segments.last() {
                let ident = segment.ident.to_string();
                // Not a primitive type and not Option/Vec
                !matches!(
                    ident.as_str(),
                    "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64"
                        | "bool" | "char" | "Option" | "Vec"
                )
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Classify field type
    #[allow(dead_code)]
    pub fn classify_field(ty: &Type) -> FieldKind {
        if Self::is_optional(ty) {
            FieldKind::Optional
        } else if Self::is_var_data(ty) {
            FieldKind::VarData
        } else if matches!(ty, Type::Array(_)) {
            FieldKind::FixedArray
        } else {
            FieldKind::Primitive
        }
    }
}

/// Calculate field offsets for a struct
pub struct OffsetCalculator {
    current_offset: usize,
}

impl OffsetCalculator {
    pub fn new() -> Self {
        Self { current_offset: 0 }
    }

    pub fn next_offset(&mut self, ty: &Type) -> Option<usize> {
        let offset = self.current_offset;
        let size = TypeMapper::type_size(ty)?;
        self.current_offset += size;
        Some(offset)
    }

    pub fn total_size(&self) -> usize {
        self.current_offset
    }
}

impl Default for OffsetCalculator {
    fn default() -> Self {
        Self::new()
    }
}
