use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive macro for Diff trait
///
/// Automatically generates field-by-field comparison code
///
/// # Example
/// ```ignore
/// #[derive(Diff)]
/// struct Order {
///     id: u64,
///     symbol: String,
///     price: f64,
/// }
/// ```
#[proc_macro_derive(Diff)]
pub fn derive_diff(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let field_comparisons = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let comparisons = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_name_str = field_name.as_ref().unwrap().to_string();
                    quote! {
                        if self.#field_name != other.#field_name {
                            changes.push(::diff::FieldChange::new(
                                #field_name_str,
                                self.#field_name.to_string(),
                                other.#field_name.to_string(),
                            ));
                        }
                    }
                });
                quote! { #(#comparisons)* }
            }
            _ => panic!("Diff can only be derived for structs with named fields"),
        },
        _ => panic!("Diff can only be derived for structs"),
    };

    let expanded = quote! {
        impl ::diff::diff::diff_types::Diff for #name {
            fn diff(&self, other: &Self) -> Vec<::diff::FieldChange> {
                let mut changes = Vec::new();
                #field_comparisons
                changes
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for Replayable trait
///
/// Automatically generates field update code from ChangeLogEntry
///
/// # Example
/// ```ignore
/// #[derive(Replayable)]
/// struct Order {
///     id: u64,
///     symbol: String,
///     price: f64,
/// }
/// ```
#[proc_macro_derive(Replayable)]
pub fn derive_replayable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let field_updates = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                let updates = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_name_str = field_name.as_ref().unwrap().to_string();
                    let field_type = &f.ty;

                    quote! {
                        #field_name_str => {
                            self.#field_name = field.new_value.parse::<#field_type>()
                                .map_err(|e| format!("Failed to parse {}: {}", #field_name_str, e))?;
                        }
                    }
                });
                quote! { #(#updates)* }
            }
            _ => panic!("Replayable can only be derived for structs with named fields"),
        },
        _ => panic!("Replayable can only be derived for structs"),
    };

    let expanded = quote! {
        impl ::diff::diff::diff_types::Replayable for #name {
            fn replay(&mut self, entry: &::diff::ChangeLogEntry) -> Result<(), String> {
                if !self.can_replay(entry) {
                    return Err("Cannot replay: entity mismatch".to_string());
                }

                match &entry.change_type {
                    ::diff::ChangeType::Updated { changed_fields } => {
                        for field in changed_fields {
                            match field.field_name.as_str() {
                                #field_updates
                                _ => {}
                            }
                        }
                        Ok(())
                    }
                    ::diff::ChangeType::Deleted => {
                        Err("Cannot replay on deleted entity".to_string())
                    }
                    ::diff::ChangeType::Created => Ok(()),
                }
            }
        }
    };

    TokenStream::from(expanded)
}
