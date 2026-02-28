//! XML Schema generation for SBE
//!
//! Generates SBE XML schema from Rust structs with #[sbe(...)] attributes.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result};

use crate::attrs::{SbeContainerAttrs, SbeFieldAttrs};
use crate::types::TypeMapper;

/// Generate XML schema from a struct
pub fn generate_xml_schema(input: &DeriveInput) -> Result<String> {
    let name = &input.ident;
    let container_attrs = SbeContainerAttrs::from_attributes(&input.attrs)?;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "XML generation only supports structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                input,
                "XML generation only supports structs",
            ))
        }
    };

    let template_id = container_attrs.template_id.unwrap_or(1);
    let schema_id = container_attrs.schema_id.unwrap_or(1);
    let version = container_attrs.version.unwrap_or(0);

    let mut xml = String::new();
    xml.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<sbe:messageSchema xmlns:sbe="http://fixprotocol.io/2016/sbe"
                   package="generated"
                   id="{}"
                   version="{}"
                   semanticVersion="0.1.0"
                   description="Generated SBE schema">

    <types>
        <composite name="messageHeader" description="Message header">
            <type name="blockLength" primitiveType="uint16"/>
            <type name="templateId" primitiveType="uint16"/>
            <type name="schemaId" primitiveType="uint16"/>
            <type name="version" primitiveType="uint16"/>
        </composite>
    </types>

    <sbe:message name="{}" id="{}" description="{}">
"#,
        schema_id, version, name, template_id, name
    ));

    // Generate field definitions
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;
        let field_attrs = SbeFieldAttrs::from_attributes(&field.attrs)?;

        let field_id = field_attrs.id.unwrap_or(0);
        let sbe_type = TypeMapper::rust_to_sbe_type(field_ty).unwrap_or("unknown");
        let presence = field_attrs.presence.as_deref().unwrap_or("required");

        xml.push_str(&format!(
            r#"        <field name="{}" id="{}" type="{}""#,
            field_name, field_id, sbe_type
        ));

        if presence != "required" {
            xml.push_str(&format!(r#" presence="{}""#, presence));
        }

        if let Some(since_version) = field_attrs.since_version {
            xml.push_str(&format!(r#" sinceVersion="{}""#, since_version));
        }

        if let Some(min_value) = &field_attrs.min_value {
            xml.push_str(&format!(r#" minValue="{}""#, min_value));
        }

        if let Some(max_value) = &field_attrs.max_value {
            xml.push_str(&format!(r#" maxValue="{}""#, max_value));
        }

        if let Some(null_value) = &field_attrs.null_value {
            xml.push_str(&format!(r#" nullValue="{}""#, null_value));
        }

        if let Some(constant) = &field_attrs.constant {
            xml.push_str(&format!(r#" valueRef="{}""#, constant));
        }

        if let Some(semantic_type) = &field_attrs.semantic_type {
            xml.push_str(&format!(r#" semanticType="{}""#, semantic_type));
        }

        xml.push_str("/>\n");
    }

    xml.push_str("    </sbe:message>\n");
    xml.push_str("</sbe:messageSchema>\n");

    Ok(xml)
}

/// Generate a proc macro that outputs XML schema at compile time
#[allow(dead_code)]
pub fn generate_xml_schema_macro(input: &DeriveInput) -> Result<TokenStream> {
    let xml = generate_xml_schema(input)?;

    Ok(quote! {
        // XML schema is generated at compile time
        // Use: const XML_SCHEMA: &str = include_str!(concat!(env!("OUT_DIR"), "/schema.xml"));
        #[doc = #xml]
        const _XML_SCHEMA_DOC: () = ();
    })
}
