use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn, Meta, MetaNameValue, parse_macro_input};

#[proc_macro_attribute]
pub fn bdd_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match parse_macro_input::parse::<AttributeArgs>(attr) {
        Ok(args) => args,
        Err(e) => return e.into_compile_error().into(),
    };

    let metadata = match parse_bdd_metadata(&attr_args) {
        Ok(m) => m,
        Err(e) => return e.into_compile_error().into(),
    };

    let input: ItemFn = match syn::parse(item) {
        Ok(i) => i,
        Err(e) => return e.into_compile_error().into(),
    };

    let fn_name = input.sig.ident.clone();
    let fn_inputs = input.sig.inputs.clone();
    let fn_output = input.sig.output.clone();
    let block = input.block.clone();

    let feature = metadata.0.clone();
    let scenario = metadata.1.clone();
    let when = metadata.2.clone();
    let given_str = metadata.3.join(", ");
    let then_str = metadata.4.join(", ");
    let tags_str = metadata.5.join(", ");
    let priority = metadata.6;

    let expanded = quote! {
        #[test]
        fn #fn_name(#fn_inputs) #fn_output {
            println!("");
            println!("╔══════════════════════════════════════════════════════╗");
            println!("║  BDD Test: {}                          ║", #feature);
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  Scenario: {}                     ║", #scenario);
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  Given: {}", #given_str);
            println!("║  When:  {}", #when);
            println!("║  Then:  {}", #then_str);
            println!("╠══════════════════════════════════════════════════════╣");
            println!("║  Tags: {}                                    ║", #tags_str);
            println!("║  Priority: {}                                       ║", #priority);
            println!("╚══════════════════════════════════════════════════════╝");

            let test_start = std::time::Instant::now();

            #block

            let duration = test_start.elapsed();
            println!("");
            println!("✓ Test completed in {:?}", duration);
        }
    };

    expanded.into()
}

type BddMeta = (String, String, String, Vec<String>, Vec<String>, Vec<String>, u8);

fn parse_bdd_metadata(attr: &AttributeArgs) -> Result<BddMeta, syn::Error> {
    let mut feature = String::new();
    let mut scenario = String::new();
    let mut when = String::new();
    let mut given = Vec::new();
    let mut then = Vec::new();
    let mut tags = Vec::new();
    let mut priority = 3u8;

    for arg in attr {
        let ident = match arg {
            syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, .. })) => path.get_ident(),
            _ => continue,
        };

        if let Some(key) = ident {
            let key_str = key.to_string();
            match key_str.as_str() {
                "feature" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            feature = s.value();
                        }
                    }
                }
                "scenario" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            scenario = s.value();
                        }
                    }
                }
                "when" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            when = s.value();
                        }
                    }
                }
                "priority" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            priority = s.value().parse().unwrap_or(3);
                        }
                    }
                }
                "given" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            let val = s.value();
                            if val.starts_with('[') {
                                given = parse_string_array(&val);
                            } else {
                                given = vec![val];
                            }
                        }
                    }
                }
                "then" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            let val = s.value();
                            if val.starts_with('[') {
                                then = parse_string_array(&val);
                            } else {
                                then = vec![val];
                            }
                        }
                    }
                }
                "tags" => {
                    if let syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { lit, .. })) = arg {
                        if let syn::Lit::Str(s) = lit {
                            let val = s.value();
                            if val.starts_with('[') {
                                tags = parse_string_array(&val);
                            } else {
                                tags = vec![val];
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    Ok((feature, scenario, when, given, then, tags, priority))
}

fn parse_string_array(s: &str) -> Vec<String> {
    let s = s.trim();
    if !s.starts_with('[') || !s.ends_with(']') {
        return vec![s.to_string()];
    }
    let inner = &s[1..s.len() - 1];
    inner
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string_array() {
        let result = parse_string_array(r#"["item1", "item2"]"#);
        assert_eq!(result, vec!["item1", "item2"]);
    }

    #[test]
    fn test_parse_string_array_single() {
        let result = parse_string_array("single");
        assert_eq!(result, vec!["single"]);
    }
}
