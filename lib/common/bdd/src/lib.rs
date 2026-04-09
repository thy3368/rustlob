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
    let when_str = metadata.2.clone();
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
            println!("║  When:  {}", #when_str);
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
        match arg {
            syn::NestedMeta::Meta(Meta::List(list)) => {
                let key_str = list.path.get_ident().map(|i| i.to_string()).unwrap_or_default();
                if key_str == "given" {
                    given = list
                        .nested
                        .iter()
                        .filter_map(|n| {
                            if let syn::NestedMeta::Meta(Meta::Path(p)) = n {
                                p.get_ident().map(|i| i.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                } else if key_str == "then" {
                    then = list
                        .nested
                        .iter()
                        .filter_map(|n| {
                            if let syn::NestedMeta::Meta(Meta::Path(p)) = n {
                                p.get_ident().map(|i| i.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                } else if key_str == "tags" {
                    tags = list
                        .nested
                        .iter()
                        .filter_map(|n| {
                            if let syn::NestedMeta::Meta(Meta::Path(p)) = n {
                                p.get_ident().map(|i| i.to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                }
            }
            syn::NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) => {
                let key_str = path.get_ident().map(|i| i.to_string()).unwrap_or_default();
                if let syn::Lit::Str(s) = lit {
                    let val = s.value();
                    match key_str.as_str() {
                        "feature" => feature = val,
                        "scenario" => scenario = val,
                        "when" => when = val,
                        "priority" => priority = val.parse().unwrap_or(3),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok((feature, scenario, when, given, then, tags, priority))
}
