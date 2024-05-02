use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, PathArguments, ReturnType, Type};

#[proc_macro_attribute]
pub fn tauri_anyhow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    // Determine the Ok type of the original function's return type
    let ok_type = match output {
        ReturnType::Type(_, type_box) => match **type_box {
            Type::Path(ref type_path) => {
                // Assuming the return type is a Result, extract the Ok type
                let path_segments = &type_path.path.segments;
                if let Some(last_segment) = path_segments.last() {
                    if let PathArguments::AngleBracketed(ref args) = last_segment.arguments {
                        if let Some(first_arg) = args.args.first() {
                            quote! { #first_arg }
                        } else {
                            quote! { () } // Default to unit type if we can't extract the type
                        }
                    } else {
                        quote! { () } // Default to unit type if no generic arguments are found
                    }
                } else {
                    quote! { () } // Default to unit type if we can't extract the type
                }
            }
            _ => quote! { () }, // Default to unit type for unsupported types
        },
        ReturnType::Default => quote! { () }, // Default to unit type if no return type is specified
    };

    // Generate the new function, preserving the return type
    let expanded = quote! {
        #(#attrs)*
        #vis fn #name(#inputs) -> Result<#ok_type, String> {
            (|| -> anyhow::Result<#ok_type> {
                #body
            })().map_err(|e| format!("{:?}", e))
        }
    };

    expanded.into()
}
