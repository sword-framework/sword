pub mod implementation;
pub mod middleware;

use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};

use crate::controller::middleware::MiddlewareArgs;

pub fn expand_controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as LitStr);

    let router_prefix_str = args.value();
    let struct_name = &input.ident;

    let mut route_middlewares = vec![];

    for attr in &input.attrs {
        if attr.path().is_ident("middleware") {
            match attr.parse_args::<MiddlewareArgs>() {
                Ok(args) => route_middlewares.push(proc_macro2::TokenStream::from(&args)),
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

            fn pre_impl_router(app_state: ::sword::application::SwordState) -> ::sword::routing::Router {
                let mut router = ::sword::routing::Router::new();

                #(
                    router = router.layer(#route_middlewares);
                )*

                let with_prefix = ::sword::routing::Router::new()
                    .nest(#struct_name::prefix(), router);

                with_prefix.with_state(app_state)
            }
        }
    };

    TokenStream::from(expanded)
}
