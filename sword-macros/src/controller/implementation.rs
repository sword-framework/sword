use super::middleware::MiddlewareArgs;
use crate::utils::*;

use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use syn::{Attribute, Ident, ImplItem, ItemImpl, parse_macro_input};

pub fn expand_controller_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let struct_self = &input.self_ty;
    let mut routes = vec![];

    for item in input.items.iter() {
        if let ImplItem::Fn(function) = item {
            let mut middlewares = Vec::new();
            let mut http_meta: Option<(&Attribute, Ident)> = None;

            for attr in &function.attrs {
                if attr.path().is_ident("middleware") {
                    match attr.parse_args::<MiddlewareArgs>() {
                        Ok(args) => middlewares.push(args),
                        Err(e) => {
                            emit_error!(attr, "Invalid middleware syntax: {}", e);
                            continue;
                        }
                    }
                } else if let Some(ident) = attr.path().get_ident() {
                    if HTTP_METHODS.contains(&ident.to_string().as_str()) {
                        http_meta = Some((attr, ident.clone()));
                    }
                }
            }

            if let Some((http_attr, http_ident)) = http_meta {
                let route_path = get_attr_http_route(http_attr);
                let method_name = &function.sig.ident;

                let mut handler = quote! {
                    ::sword::__private::#http_ident(#struct_self::#method_name)
                };

                for mw in middlewares.iter().rev() {
                    let mw_tokens = proc_macro2::TokenStream::from(mw);
                    handler = quote! {
                        #handler.layer(#mw_tokens)
                    };
                }

                let route = quote! {
                    .route(#route_path, #handler)
                };

                routes.push(route);
            }
        }
    }

    let expanded = quote! {
        #input

        impl ::sword::routing::RouterProvider for #struct_self {
            fn router(app_state: ::sword::application::SwordState) -> ::sword::routing::Router {

                let base_router = ::sword::routing::Router::new()
                    #(#routes)*
                    .with_state(app_state.clone());

                let pre_impl_router = #struct_self::pre_impl_router(app_state.clone());

                let a = pre_impl_router.merge(base_router);

                dbg!(&a);

                a
            }
        }
    };

    TokenStream::from(expanded)
}
