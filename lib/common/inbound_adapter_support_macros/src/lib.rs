use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Error, FnArg, GenericArgument, Ident, ItemFn, LitInt, LitStr, Result, ReturnType, Token, Type,
    TypePath, parse_macro_input,
};

struct HttpEndpointArgs {
    name: LitStr,
    method: LitStr,
    path: LitStr,
    summary: LitStr,
    error_response_schema: LitStr,
    error_codes: Vec<(LitInt, LitStr)>,
}

impl Parse for HttpEndpointArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut name = None;
        let mut method = None;
        let mut path = None;
        let mut summary = None;
        let mut error_response_schema = None;
        let mut error_codes = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "name" => name = Some(input.parse()?),
                "method" => method = Some(input.parse()?),
                "path" => path = Some(input.parse()?),
                "summary" => summary = Some(input.parse()?),
                "error_response_schema" => error_response_schema = Some(input.parse()?),
                "error_codes" => {
                    let content;
                    syn::bracketed!(content in input);
                    let tuples =
                        Punctuated::<HttpErrorCodeTuple, Token![,]>::parse_terminated(&content)?;
                    error_codes =
                        Some(tuples.into_iter().map(|tuple| (tuple.0, tuple.1)).collect());
                }
                other => {
                    return Err(Error::new(
                        key.span(),
                        format!("unsupported attribute key `{other}`"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            name: name
                .ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "missing `name`"))?,
            method: method
                .ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "missing `method`"))?,
            path: path
                .ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "missing `path`"))?,
            summary: summary
                .ok_or_else(|| Error::new(proc_macro2::Span::call_site(), "missing `summary`"))?,
            error_response_schema: error_response_schema.ok_or_else(|| {
                Error::new(proc_macro2::Span::call_site(), "missing `error_response_schema`")
            })?,
            error_codes: error_codes.ok_or_else(|| {
                Error::new(proc_macro2::Span::call_site(), "missing `error_codes`")
            })?,
        })
    }
}

struct HttpErrorCodeTuple(LitInt, LitStr);

impl Parse for HttpErrorCodeTuple {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let content;
        syn::parenthesized!(content in input);
        let status_code: LitInt = content.parse()?;
        content.parse::<Token![,]>()?;
        let code: LitStr = content.parse()?;
        Ok(Self(status_code, code))
    }
}

#[proc_macro_attribute]
pub fn collect_http_endpoint(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as HttpEndpointArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    match expand_collect_http_endpoint(args, input_fn) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_collect_http_endpoint(
    args: HttpEndpointArgs,
    input_fn: ItemFn,
) -> Result<proc_macro2::TokenStream> {
    let fn_ident = input_fn.sig.ident.clone();
    let descriptor_fn_ident = format_ident!("{}_http_api_descriptor", fn_ident);

    let request_type = extract_request_type(&input_fn)?;
    let response_type = extract_response_type(&input_fn)?;

    let name = args.name;
    let method = args.method;
    let path = args.path;
    let summary = args.summary;
    let error_response_schema = args.error_response_schema;
    let error_code_exprs = args.error_codes.into_iter().map(|(status_code, code)| {
        quote! {
            ::inbound_adapter_support::http_error(#status_code, #code)
        }
    });

    Ok(quote! {
        #input_fn

        pub(crate) fn #descriptor_fn_ident() -> ::inbound_adapter_support::HttpApiDescriptor {
            ::inbound_adapter_support::HttpApiDescriptor {
                name: #name,
                method: #method,
                path: #path,
                summary: #summary,
                request_schema_name: stringify!(#request_type),
                success_response_schema_name: stringify!(#response_type),
                error_response_schema_name: #error_response_schema,
                error_codes: vec![#(#error_code_exprs),*],
            }
        }
    })
}

fn extract_request_type(input_fn: &ItemFn) -> Result<Type> {
    for arg in &input_fn.sig.inputs {
        let FnArg::Typed(arg) = arg else {
            continue;
        };

        if let Some(inner) = extract_json_inner_type(&arg.ty) {
            return Ok(inner.clone());
        }
    }

    Err(Error::new_spanned(&input_fn.sig.inputs, "expected one handler argument of type `Json<T>`"))
}

fn extract_response_type(input_fn: &ItemFn) -> Result<Type> {
    let ReturnType::Type(_, ty) = &input_fn.sig.output else {
        return Err(Error::new_spanned(
            &input_fn.sig.output,
            "expected `Result<Json<T>, E>` return type",
        ));
    };

    let Type::Path(result_path) = ty.as_ref() else {
        return Err(Error::new_spanned(ty, "expected `Result<Json<T>, E>` return type"));
    };

    let Some(result_segment) = result_path.path.segments.last() else {
        return Err(Error::new_spanned(result_path, "missing result path segment"));
    };

    if result_segment.ident != "Result" {
        return Err(Error::new_spanned(
            result_segment,
            "expected `Result<Json<T>, E>` return type",
        ));
    }

    let syn::PathArguments::AngleBracketed(args) = &result_segment.arguments else {
        return Err(Error::new_spanned(result_segment, "expected generic `Result<Json<T>, E>`"));
    };

    let Some(GenericArgument::Type(ok_type)) = args.args.first() else {
        return Err(Error::new_spanned(args, "expected `Result<Json<T>, E>` ok type"));
    };

    let Some(inner) = extract_json_inner_type(ok_type) else {
        return Err(Error::new_spanned(ok_type, "expected ok type `Json<T>`"));
    };

    Ok(inner.clone())
}

fn extract_json_inner_type(ty: &Type) -> Option<&Type> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };
    let segment = path.segments.last()?;
    if segment.ident != "Json" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        GenericArgument::Type(inner) => Some(inner),
        _ => None,
    }
}
