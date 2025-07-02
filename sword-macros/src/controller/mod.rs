pub mod implementation;
pub mod middleware;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

pub fn expand_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as LitStr);

    let router_prefix_str = args.value();
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn prefix() -> &'static str {
                #router_prefix_str
            }
        }
    };

    TokenStream::from(expanded)
}
