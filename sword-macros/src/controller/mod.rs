mod path_utils;

use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{
    Attribute, Ident, ImplItem, ItemImpl, ItemStruct, LitStr, Path, Token, parse_macro_input,
    punctuated::Punctuated,
};

use crate::controller::path_utils::HTTP_METHODS;

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
                    match attr.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated) {
                        Ok(paths) => middlewares = paths.into_iter().collect(),
                        Err(err) => {
                            emit_error!(
                                attr,
                                "Expected a comma-separated list of middleware types: {}",
                                err
                            );

                            return TokenStream::new();
                        }
                    }
                } else if let Some(ident) = attr.path().get_ident() {
                    if HTTP_METHODS.contains(&ident.to_string().as_str()) {
                        http_meta = Some((attr, ident.clone()));
                    }
                }
            }

            if let Some((http_attr, http_ident)) = http_meta {
                let route_path = path_utils::get_attr_http_route(http_attr);
                let method_name = &function.sig.ident;

                let mut handler = quote! {
                    ::sword::__private::#http_ident(#struct_self::#method_name)
                };

                for mw in middlewares.iter().rev() {
                    handler = quote! { #handler.layer(::sword::__private::from_fn_with_state(app_state.clone(), #mw::middleware_handle)) };
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
            fn router(app_state: ::sword::application::AppState) -> ::sword::routing::Router {
                let base_router = ::sword::routing::Router::new()
                    #(#routes)*;

                ::sword::routing::Router::new()
                    .nest(#struct_self::prefix(), base_router)
                    .with_state(app_state)
            }
        }
    };

    TokenStream::from(expanded)
}
