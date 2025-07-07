pub mod implementation;
pub mod middleware;

use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

use crate::controller::middleware::{MiddlewareArgs, expand_middleware_args};

pub fn expand_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as LitStr);

    let router_prefix_str = args.value();
    let struct_name = &input.ident;

    let mut route_middlewares = vec![];

    for attr in &input.attrs {
        if attr.path().is_ident("middleware") {
            match attr.parse_args::<MiddlewareArgs>() {
                Ok(args) => route_middlewares.push(expand_middleware_args(&args)),
                Err(e) => emit_error!("Failed to parse middleware arguments: {}", e),
            }
        }
    }

    let expanded = quote! {
        #input

        impl #struct_name {
            fn prefix() -> &'static str {
                #router_prefix_str
            }

            fn apply_global_middlewares(router: ::sword::routing::Router, app_state: ::sword::application::SwordState) -> ::sword::routing::Router {
                let mut result = router;

                #(
                    result = result.layer(#route_middlewares);
                )*

                result
            }
        }
    };

    TokenStream::from(expanded)
}
