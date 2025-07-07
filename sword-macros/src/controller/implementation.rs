use super::middleware::MiddlewareArgs;
use crate::{controller::middleware::expand_middleware_args, utils::*};

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

                let routing_fn = match http_ident.to_string().as_str() {
                    "get" => quote! { axum_get_fn },
                    "post" => quote! { axum_post_fn },
                    "put" => quote! { axum_put_fn },
                    "patch" => quote! { axum_patch_fn },
                    "delete" => quote! { axum_delete_fn },
                    _ => {
                        emit_error!(http_attr, "Unsupported HTTP method: {}", http_ident);
                        continue;
                    }
                };

                let mut handler = quote! {
                    ::sword::__private::#routing_fn(#struct_self::#method_name)
                };

                for mw in middlewares.iter().rev() {
                    let mw_tokens = expand_middleware_args(mw);
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

                let router_with_global_mw = #struct_self::apply_global_middlewares(base_router, app_state);

                let prefix = #struct_self::prefix();
                if prefix == "/" {
                    router_with_global_mw
                } else {
                    ::sword::routing::Router::new()
                        .nest(prefix, router_with_global_mw)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
