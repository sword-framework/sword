use std::sync::OnceLock;

use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{Attribute, ImplItem, ItemImpl, LitStr, parse_macro_input};

static PATH_KIND_REGEX: OnceLock<Regex> = OnceLock::new();

fn path_kind_regex() -> &'static Regex {
    PATH_KIND_REGEX.get_or_init(|| {
        Regex::new(r"^\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\})*(?:\/(?:[^\/{}]+|\{[^*{}][^{}]*\}|\{\*[^{}]+\}))*$").unwrap()
    })
}

pub fn expand_router(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);
    let route = parse_macro_input!(attr as LitStr);

    let struct_self = &input.self_ty;
    let router_prefix_str = route.value();

    let mut routes = vec![];

    for item in input.items.iter() {
        if let ImplItem::Fn(function) = item {
            for attr in function.attrs.iter() {
                let Some(http_method) = attr.path().get_ident() else {
                    panic!("Handler in controller must have a http method proc macro")
                };

                let route_path = get_attr_http_route(attr);
                let method_name = &function.sig.ident;

                let route = match http_method.to_string().as_str() {
                    "get" => quote! {
                        .route(#route_path, ::axum::routing::get(#struct_self::#method_name))
                    },
                    "post" => quote! {
                        .route(#route_path, ::axum::routing::post(#struct_self::#method_name))
                    },
                    "put" => quote! {
                        .route(#route_path, ::axum::routing::put(#struct_self::#method_name))
                    },
                    "delete" => quote! {
                        .route(#route_path, ::axum::routing::delete(#struct_self::#method_name))
                    },
                    "patch" => quote! {
                        .route(#route_path, ::axum::routing::patch(#struct_self::#method_name))
                    },
                    _ => panic!("Unknown HTTP method: {}", http_method),
                };

                routes.push(route);
            }
        }
    }

    let base_router = quote! {
        ::axum::Router::new()
            #(#routes)*
    };

    let expanded = quote! {
        #input

        impl #struct_self {
            pub fn router() -> ::axum::Router {
                ::axum::Router::new()
                    .nest(#router_prefix_str, #base_router)
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_attr_http_route(attr: &Attribute) -> LitStr {
    let path: LitStr = attr.parse_args().expect(
        "Expected a string literal as path in HTTP method attribute, e.g., #[get(\"/path\")]",
    );

    let value = path.value();

    if !path_kind_regex().is_match(&value) {
        panic!(
            "Invalid route path: '{}'. Must start with '/' and not contain spaces.",
            value
        );
    }

    path
}
