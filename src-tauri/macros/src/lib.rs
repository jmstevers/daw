use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn tauri_anyhow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    quote! {
        #(#attrs)*
        #vis fn #name(#inputs) -> Result<(), tauri::ipc::InvokeError> {
            (|| -> anyhow::Result<()> {
                #body
            })().map_err(|e| tauri::ipc::InvokeError::from_anyhow(e))
        }
    }
    .into()
}
