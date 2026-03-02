//! SBE attribute parsing
//!
//! Parses `#[sbe(...)]` attributes according to FIX SBE 2.0 specification.

use syn::{Attribute, Lit, Result};

/// Container-level SBE attributes
#[derive(Debug, Default)]
pub struct SbeContainerAttrs {
    pub template_id: Option<u16>,
    pub schema_id: Option<u16>,
    pub version: Option<u16>,
    pub block_length: Option<u16>,
}

/// Field-level SBE attributes
#[derive(Debug, Default, Clone)]
pub struct SbeFieldAttrs {
    pub id: Option<usize>,
    pub field_type: Option<String>,
    pub primitive_type: Option<String>,
    pub presence: Option<String>,
    pub null_value: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub since_version: Option<u16>,
    pub deprecated_version: Option<u16>,
    pub constant: Option<String>,
    pub semantic_type: Option<String>,
    pub description: Option<String>,
    pub length: Option<usize>,
    pub character_encoding: Option<String>,
    pub mantissa_type: Option<String>,
    pub exponent: Option<i8>,
    pub length_field: Option<String>,
}

impl SbeContainerAttrs {
    pub fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("sbe") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("template_id") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.template_id = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("schema_id") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.schema_id = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("version") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.version = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("block_length") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.block_length = Some(lit_int.base10_parse()?);
                    }
                }
                Ok(())
            })?;
        }

        Ok(result)
    }
}

impl SbeFieldAttrs {
    pub fn from_attributes(attrs: &[Attribute]) -> Result<Self> {
        let mut result = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("sbe") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.id = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("field_type") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.field_type = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("primitive_type") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.primitive_type = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("presence") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.presence = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("null_value") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.null_value = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("min_value") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.min_value = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("max_value") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.max_value = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("since_version") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.since_version = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("deprecated_version") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.deprecated_version = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("constant") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.constant = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("semantic_type") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.semantic_type = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("description") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.description = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("length") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.length = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("character_encoding") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.character_encoding = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("mantissa_type") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.mantissa_type = Some(lit_str.value());
                    }
                } else if meta.path.is_ident("exponent") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Int(lit_int) = value {
                        result.exponent = Some(lit_int.base10_parse()?);
                    }
                } else if meta.path.is_ident("length_field") {
                    let value: Lit = meta.value()?.parse()?;
                    if let Lit::Str(lit_str) = value {
                        result.length_field = Some(lit_str.value());
                    }
                }
                Ok(())
            })?;
        }

        Ok(result)
    }
}
